use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}

#[derive(Debug)]
pub struct Conversation {
    messages: Vec<Message>,
}

impl Conversation {
    pub fn new(system_instruction: &str) -> Self {
        Self {
            messages: vec![Message::system(system_instruction)],
        }
    }
    pub fn add_user(&mut self, content: impl Into<String>) {
        self.messages.push(Message::user(content));
    }
    pub fn add_assistant(&mut self, content: impl Into<String>) {
        self.messages.push(Message::assistant(content));
    }
    pub fn pop_user(&mut self) {
        if self.messages.last().is_some_and(|m| m.role == Role::User) {
            self.messages.pop();
        }
    }
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }
}

pub fn build_payload(messages: &[Message], model: &str) -> serde_json::Value {
    serde_json::json!({ "model": model, "messages": messages })
}

pub fn extract_content(value: &serde_json::Value) -> Option<String> {
    value
        .get("choices")?
        .get(0)?
        .get("message")?
        .get("content")?
        .as_str()
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversation_starts_with_system_and_appends() {
        let mut c = Conversation::new("sys");
        assert_eq!(c.messages().len(), 1);
        assert_eq!(c.messages()[0].role, Role::System);
        c.add_user("hi");
        c.add_assistant("hello");
        assert_eq!(c.messages().len(), 3);
        assert_eq!(c.messages()[1].role, Role::User);
        assert_eq!(c.messages()[2].role, Role::Assistant);
    }

    #[test]
    fn pop_user_removes_only_trailing_user() {
        let mut c = Conversation::new("sys");
        c.add_user("hi");
        c.pop_user();
        assert_eq!(c.messages().len(), 1);
    }

    #[test]
    fn role_serializes_lowercase() {
        let m = Message::assistant("x");
        assert_eq!(
            serde_json::to_value(&m).unwrap(),
            serde_json::json!({"role":"assistant","content":"x"})
        );
    }

    #[test]
    fn build_payload_shape() {
        let mut c = Conversation::new("sys");
        c.add_user("hi");
        let p = build_payload(c.messages(), "gpt");
        assert_eq!(p["model"], "gpt");
        assert_eq!(p["messages"][0]["role"], "system");
        assert_eq!(p["messages"][1]["content"], "hi");
    }

    #[test]
    fn extract_content_from_response() {
        let v =
            serde_json::json!({"choices":[{"message":{"role":"assistant","content":"ls -la"}}]});
        assert_eq!(extract_content(&v).unwrap(), "ls -la");
        assert!(extract_content(&serde_json::json!({})).is_none());
    }
}
