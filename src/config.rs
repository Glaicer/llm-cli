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
}
