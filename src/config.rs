use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const DEFAULT_SYSTEM_INSTRUCTION: &str = "You are an AI CLI assistant. Your answers should be short and concise. If the user asks for command, output just command without any comments.";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub system_instruction: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file: {0}")]
    Read(#[from] std::io::Error),
    #[error("failed to parse config: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("failed to write config: {0}")]
    Write(toml::ser::Error),
}

impl Config {
    pub fn parse(text: &str) -> Result<Self, ConfigError> {
        Ok(toml::from_str(text)?)
    }

    pub fn default_with(base_url: String, model: String, api_key: String) -> Self {
        Self {
            base_url,
            api_key,
            model,
            system_instruction: DEFAULT_SYSTEM_INSTRUCTION.to_string(),
        }
    }

    pub fn to_toml(&self) -> Result<String, ConfigError> {
        Ok(toml::to_string_pretty(self).map_err(ConfigError::Write)?)
    }
}

use std::path::PathBuf;

const ENV_API_KEY: &str = "LLM_CLI_API_KEY";
const ENV_BASE_URL: &str = "LLM_CLI_BASE_URL";
const ENV_MODEL: &str = "LLM_CLI_MODEL";

pub fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("llm-cli").join("config.toml"))
}

pub fn load_from_file(path: &PathBuf) -> Result<Config, ConfigError> {
    let text = std::fs::read_to_string(path)?;
    Config::parse(&text)
}

pub fn apply_env_overrides(mut cfg: Config) -> Config {
    if let Ok(v) = std::env::var(ENV_API_KEY) {
        if !v.is_empty() {
            cfg.api_key = v;
        }
    }
    if let Ok(v) = std::env::var(ENV_BASE_URL) {
        if !v.is_empty() {
            cfg.base_url = v;
        }
    }
    if let Ok(v) = std::env::var(ENV_MODEL) {
        if !v.is_empty() {
            cfg.model = v;
        }
    }
    cfg
}

use std::io::{self, BufRead, Write};

pub enum LoadOutcome {
    Loaded(Config),
    Created,
}

pub fn write_config(path: &PathBuf, cfg: &Config) -> Result<(), ConfigError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let text = cfg.to_toml()?;
    std::fs::write(path, text)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}

pub fn load() -> Result<LoadOutcome, ConfigError> {
    let Some(path) = config_path() else {
        return Err(ConfigError::Read(io::Error::new(
            io::ErrorKind::NotFound,
            "no config directory found on this platform",
        )));
    };
    if path.exists() {
        let cfg = load_from_file(&path)?;
        Ok(LoadOutcome::Loaded(apply_env_overrides(cfg)))
    } else {
        run_first_run_setup(&path)?;
        Ok(LoadOutcome::Created)
    }
}

fn run_first_run_setup(path: &PathBuf) -> Result<(), ConfigError> {
    println!("No config found at {}.", path.display());
    println!("Let's create one.\n");
    let base_url = prompt("Base URL (OpenAI-compatible, e.g. https://api.openai.com/v1): ")?;
    let model = prompt("Model id (e.g. gpt-4o-mini): ")?;
    let api_key = prompt("API key: ")?;
    let cfg = Config::default_with(
        base_url.trim().to_string(),
        model.trim().to_string(),
        api_key.trim().to_string(),
    );
    write_config(path, &cfg)?;
    println!("\nConfig written to {}.", path.display());
    println!(
        "The default system instruction has been set. Edit the file if needed, then re-run `llm`."
    );
    Ok(())
}

fn prompt(question: &str) -> Result<String, ConfigError> {
    print!("{}", question);
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;
    Ok(line.trim_end().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_config() {
        let toml = r#"
base_url = "https://api.openai.com/v1"
api_key = "sk-abc"
model = "gpt-4o-mini"
system_instruction = "be brief"
"#;
        let cfg = Config::parse(toml).unwrap();
        assert_eq!(cfg.base_url, "https://api.openai.com/v1");
        assert_eq!(cfg.api_key, "sk-abc");
        assert_eq!(cfg.model, "gpt-4o-mini");
        assert_eq!(cfg.system_instruction, "be brief");
    }

    #[test]
    fn default_instruction_matches_spec() {
        assert_eq!(
            DEFAULT_SYSTEM_INSTRUCTION,
            "You are an AI CLI assistant. Your answers should be short and concise. If the user asks for command, output just command without any comments."
        );
    }

    #[test]
    fn env_overrides_behavior() {
        let base = || Config {
            base_url: "b".into(),
            api_key: "k".into(),
            model: "m".into(),
            system_instruction: "s".into(),
        };

        std::env::set_var("LLM_CLI_API_KEY", "NEWKEY");
        std::env::set_var("LLM_CLI_BASE_URL", "https://example/v1");
        std::env::set_var("LLM_CLI_MODEL", "newmodel");
        let out = apply_env_overrides(base());
        assert_eq!(out.api_key, "NEWKEY");
        assert_eq!(out.base_url, "https://example/v1");
        assert_eq!(out.model, "newmodel");

        std::env::set_var("LLM_CLI_API_KEY", "");
        let out = apply_env_overrides(base());
        assert_eq!(out.api_key, "k");

        std::env::remove_var("LLM_CLI_API_KEY");
        std::env::remove_var("LLM_CLI_BASE_URL");
        std::env::remove_var("LLM_CLI_MODEL");
        let out = apply_env_overrides(base());
        assert_eq!(out.api_key, "k");
        assert_eq!(out.base_url, "b");
        assert_eq!(out.model, "m");
    }

    #[test]
    fn loads_config_from_file() {
        let path = std::env::temp_dir().join(format!("llm-cli-cfg-{}.toml", std::process::id()));
        std::fs::write(
            &path,
            "base_url=\"u\"\napi_key=\"k\"\nmodel=\"m\"\nsystem_instruction=\"s\"\n",
        )
        .unwrap();
        let cfg = load_from_file(&path).unwrap();
        assert_eq!(cfg.model, "m");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn default_with_sets_default_instruction() {
        let cfg = Config::default_with("u".into(), "m".into(), "k".into());
        assert_eq!(cfg.system_instruction, DEFAULT_SYSTEM_INSTRUCTION);
    }

    #[test]
    fn to_toml_roundtrips() {
        let cfg = Config::default_with("https://e/v1".into(), "gpt".into(), "sk".into());
        let text = cfg.to_toml().unwrap();
        let back = Config::parse(&text).unwrap();
        assert_eq!(cfg, back);
    }

    #[test]
    fn write_config_creates_file_with_permissions() {
        let dir = std::env::temp_dir().join(format!("llm-cli-w-{}", std::process::id()));
        let path = dir.join("nested").join("config.toml");
        let cfg = Config::default_with("u".into(), "m".into(), "k".into());
        write_config(&path, &cfg).unwrap();
        assert!(path.exists());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
            assert_eq!(mode, 0o600);
        }
        let back = load_from_file(&path).unwrap();
        assert_eq!(back, cfg);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
