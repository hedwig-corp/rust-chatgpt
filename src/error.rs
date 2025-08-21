use serde_json::Value;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub enum ChatGptError {
    Connection(String),
    Status(u16, String),
    JsonParse(Value),
}

impl Display for ChatGptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatGptError::Connection(e) => write!(f, "{}", e),
            ChatGptError::JsonParse(e) => write!(f, "{}", e),
            ChatGptError::Status(status, value) => {
                write!(f, "status: {} ,message:{}", status, value)
            }
        }
    }
}

impl Debug for ChatGptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
