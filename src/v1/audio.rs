use crate::error::ChatGptError;
use crate::v1::{convert_form, convert_from_value, trim_value, ChatGptRequest};
use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatGptResponseAudioTranscriptions {
    pub(crate) value: Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatGptRequestAudioTranscriptions {
    file: String,
    model: String,
    prompt: Option<String>,
    response_format: Option<String>,
    temperature: Option<f32>,
    language: Option<String>,
}

impl From<ChatGptRequestAudioTranscriptions> for Form {
    fn from(val: ChatGptRequestAudioTranscriptions) -> Self {
        convert_form(val.to_value(), vec!["file".to_string()])
    }
}

impl ChatGptRequest for ChatGptRequestAudioTranscriptions {
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

impl ChatGptRequestAudioTranscriptions {
    pub fn new(model: &str, file: &str) -> Self {
        Self {
            model: model.to_string(),
            prompt: None,
            response_format: None,
            temperature: None,
            file: file.to_string(),
            language: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatGptResponseAudioTranslations {
    pub(crate) value: Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatGptRequestAudioTranslations {
    file: String,
    model: String,
    prompt: Option<String>,
    response_format: Option<String>,
    temperature: Option<f32>,
}

impl From<ChatGptRequestAudioTranslations> for Form {
    fn from(val: ChatGptRequestAudioTranslations) -> Self {
        convert_form(val.to_value(), vec!["file".to_string()])
    }
}

impl ChatGptRequest for ChatGptRequestAudioTranslations {
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

impl ChatGptRequestAudioTranslations {
    pub fn new(model: &str, file: &str) -> Self {
        Self {
            model: model.to_string(),
            prompt: None,
            response_format: None,
            temperature: None,
            file: file.to_string(),
        }
    }
}
