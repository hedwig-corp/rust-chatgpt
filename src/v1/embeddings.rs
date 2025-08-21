use crate::error::ChatGptError;
use crate::v1::{convert_from_value, trim_value, ChatGptRequest};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatGptResponseEmbeddings {
    value: Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatGptRequestEmbeddingsGenerations {
    model: String,
    input: Vec<String>,
    user: Option<String>,
}

impl ChatGptRequest for ChatGptRequestEmbeddingsGenerations {
    fn from_value(value: Value) -> Result<Self, ChatGptError>
    where
        Self: Sized,
    {
        convert_from_value(value)
    }

    fn to_value(&self) -> Value {
        trim_value(json!(self)).unwrap()
    }
}

impl ChatGptRequestEmbeddingsGenerations {
    pub fn new(model: &str, input: &str) -> Self {
        Self {
            model: model.to_string(),
            input: vec![input.to_string()],
            user: None,
        }
    }
}
