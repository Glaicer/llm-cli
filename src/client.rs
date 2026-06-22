use crate::chat::{build_payload, extract_content, Message};
use crate::config::Config;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("provider returned status {0}: {1}")]
    Status(u16, String),
    #[error("could not read content from response: {0}")]
    Parse(String),
}

pub trait ChatClient {
    fn chat(&self, messages: &[Message]) -> Result<String, ClientError>;
}

pub struct ReqwestClient {
    client: reqwest::blocking::Client,
    base_url: String,
    api_key: String,
    model: String,
}

impl ReqwestClient {
    pub fn new(cfg: &Config) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap_or_else(|_| reqwest::blocking::Client::new());
        Self {
            client,
            base_url: cfg.base_url.trim_end_matches('/').to_string(),
            api_key: cfg.api_key.clone(),
            model: cfg.model.clone(),
        }
    }
}

impl ChatClient for ReqwestClient {
    fn chat(&self, messages: &[Message]) -> Result<String, ClientError> {
        let payload = build_payload(messages, &self.model);
        let resp = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().unwrap_or_default();
            return Err(ClientError::Status(status.as_u16(), body));
        }
        let value: serde_json::Value = resp.json()?;
        extract_content(&value)
            .ok_or_else(|| ClientError::Parse("missing choices[0].message.content".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_error_displays_code_and_body() {
        let e = ClientError::Status(404, "not found".into());
        let s = e.to_string();
        assert!(s.contains("404"), "got: {s}");
        assert!(s.contains("not found"));
    }
}
