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
}
