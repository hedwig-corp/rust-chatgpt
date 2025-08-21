pub(crate) mod error;
pub(crate) mod http;
mod v1;

use crate::error::ChatGptError;
use crate::http::HttpClient;
use reqwest::multipart::Form;
use serde_json::Value;

pub use crate::v1::audio::*;
pub use crate::v1::chat::*;
pub use crate::v1::completions::*;
pub use crate::v1::edits::*;
pub use crate::v1::embeddings::*;
pub use crate::v1::images::*;
pub use crate::v1::models::*;
pub use crate::v1::{ChatGptRequest, ChatGptResponse};

pub struct ChatGpt {
    oepenai_api_key: String,
    org_id: String,
    base_url: String,
}

impl ChatGpt {
    pub fn new(oepenai_api_key: &str) -> Self {
        Self {
            oepenai_api_key: oepenai_api_key.to_string(),
            org_id: "".to_string(),
            base_url: "https://api.openai.com".to_string(),
        }
    }
    pub fn new_org(oepenai_api_key: String, org_id: String) -> Self {
        Self {
            oepenai_api_key,
            org_id,
            base_url: "https://api.openai.com".to_string(),
        }
    }

    pub fn new_with_base_url(oepenai_api_key: &str, base_url: &str) -> Self {
        Self {
            oepenai_api_key: oepenai_api_key.to_string(),
            org_id: "".to_string(),
            base_url: base_url.to_string(),
        }
    }

    pub fn new_org_with_base_url(
        oepenai_api_key: String,
        org_id: String,
        base_url: String,
    ) -> Self {
        Self {
            oepenai_api_key,
            org_id,
            base_url,
        }
    }

    pub async fn models_list(&self) -> Result<ChatGptResponseModelList, ChatGptError> {
        let url = format!("{}/v1/models", self.base_url);
        let value = HttpClient::get(
            self.oepenai_api_key.as_str(),
            self.org_id.as_str(),
            &url,
            &Value::default(),
        )
        .await?;
        Ok(ChatGptResponseModelList { value })
    }
    pub async fn models_retrieve(
        &self,
        model: &str,
    ) -> Result<ChatGptResponseModelRetrieve, ChatGptError> {
        let url = format!("{}/v1/models/{}", self.base_url, model);
        let value = HttpClient::get(
            self.oepenai_api_key.as_str(),
            self.org_id.as_str(),
            &url,
            &Value::default(),
        )
        .await?;
        Ok(ChatGptResponseModelRetrieve { value })
    }
    pub async fn completions_create(
        &self,
        request: &ChatGptRequestCompletionsCreate,
    ) -> Result<ChatGptResponseModelRetrieve, ChatGptError> {
        let url = format!("{}/v1/completions", self.base_url);

        let value = HttpClient::post(
            self.oepenai_api_key.as_str(),
            self.org_id.as_str(),
            &url,
            &request.to_value(),
        )
        .await?;
        Ok(ChatGptResponseModelRetrieve { value })
    }
    pub async fn chat_completions(
        &self,
        request: &ChatGptRequestChatCompletions,
    ) -> Result<ChatGptResponseChatCompletions, ChatGptError> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let data = request.to_value();
        let value = HttpClient::post(
            self.oepenai_api_key.as_str(),
            self.org_id.as_str(),
            &url,
            &data,
        )
        .await?;
        Ok(ChatGptResponseChatCompletions { value })
    }
    pub async fn edits(&self, request: &ChatGptRequestEdits) -> Result<Value, ChatGptError> {
        let url = format!("{}/v1/edits", self.base_url);
        let data = request.to_value();
        HttpClient::post(
            self.oepenai_api_key.as_str(),
            self.org_id.as_str(),
            &url,
            &data,
        )
        .await
    }
    pub async fn images_generations(
        &self,
        request: &ChatGptRequestImagesGenerations,
    ) -> Result<ChatGptResponseImagesGenerations, ChatGptError> {
        let url = format!("{}/v1/images/generations", self.base_url);
        let data = request.to_value();
        let value = HttpClient::post(
            self.oepenai_api_key.as_str(),
            self.org_id.as_str(),
            &url,
            &data,
        )
        .await?;
        Ok(ChatGptResponseImagesGenerations { value })
    }
    pub async fn images_edits(
        &self,
        request: &ChatGptRequestImagesEdits,
    ) -> Result<ChatGptResponseImagesEdits, ChatGptError> {
        let url = format!("{}/v1/images/edits", self.base_url);
        let value = HttpClient::post_data(
            self.oepenai_api_key.as_str(),
            self.org_id.as_str(),
            &url,
            Form::from(request.clone()),
        )
        .await?;
        Ok(ChatGptResponseImagesEdits { value })
    }
    pub async fn images_variations(
        &self,
        request: &ChatGptRequestImagesVariation,
    ) -> Result<ChatGptResponseImagesVariation, ChatGptError> {
        let url = format!("{}/v1/images/variations", self.base_url);
        let value = HttpClient::post_data(
            self.oepenai_api_key.as_str(),
            self.org_id.as_str(),
            &url,
            Form::from(request.clone()),
        )
        .await?;
        Ok(ChatGptResponseImagesVariation { value })
    }
    pub async fn embeddings(
        &self,
        request: &ChatGptRequestEmbeddingsGenerations,
    ) -> Result<Value, ChatGptError> {
        let url = format!("{}/v1/embeddings", self.base_url);
        let data = request.to_value();
        HttpClient::post(
            self.oepenai_api_key.as_str(),
            self.org_id.as_str(),
            &url,
            &data,
        )
        .await
    }
    pub async fn audio_transcriptions(
        &self,
        request: &ChatGptRequestAudioTranscriptions,
    ) -> Result<ChatGptResponseAudioTranscriptions, ChatGptError> {
        let url = format!("{}/v1/audio/transcriptions", self.base_url);
        let value = HttpClient::post_data(
            self.oepenai_api_key.as_str(),
            self.org_id.as_str(),
            &url,
            Form::from(request.clone()),
        )
        .await?;
        Ok(ChatGptResponseAudioTranscriptions { value })
    }
    pub async fn audio_translations(
        &self,
        request: &ChatGptRequestAudioTranslations,
    ) -> Result<ChatGptResponseAudioTranslations, ChatGptError> {
        let url = format!("{}/v1/audio/translations", self.base_url);
        let value = HttpClient::post_data(
            self.oepenai_api_key.as_str(),
            self.org_id.as_str(),
            &url,
            Form::from(request.clone()),
        )
        .await?;
        Ok(ChatGptResponseAudioTranslations { value })
    }
}
