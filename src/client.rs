use crate::error::{Error, Result};
use serde::Serialize;
use zai_rs::client::HttpClient;
use zai_rs::model::chat_base_response::ChatCompletionResponse;
use zai_rs::model::text_to_audio::{
    data::TextToAudioRequest, model::GlmTts, request::TtsAudioFormat, Voice,
};
use zai_rs::model::traits::{Bounded, Chat, ModelName, ThinkEnable};
use zai_rs::model::{
    ChatCompletion, GLM4_5_air, GLM4_5_flash, TextMessage, ThinkingType, GLM4_5, GLM4_6, GLM4_7,
};

/// AI model for text splitting
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Model {
    /// GLM-4.7 - Latest flagship model
    GLM4_7,
    /// GLM-4.6 - Advanced reasoning model
    GLM4_6,
    /// GLM-4.5 - Advanced reasoning model
    GLM4_5,
    /// GLM-4.5-Flash - Optimized for speed (default)
    #[default]
    GLM4_5Flash,
    /// GLM-4.5-Air - Lightweight and cost-effective
    GLM4_5Air,
}

impl Model {
    pub fn as_str(&self) -> &str {
        match self {
            Model::GLM4_7 => "glm-4.7",
            Model::GLM4_6 => "glm-4.6",
            Model::GLM4_5 => "glm-4.5",
            Model::GLM4_5Flash => "glm-4.5-flash",
            Model::GLM4_5Air => "glm-4.5-air",
        }
    }
}

/// TTS configuration
pub struct TtsConfig {
    pub voice: Voice,
    pub speed: f32,
    pub volume: f32,
}

impl Default for TtsConfig {
    fn default() -> Self {
        Self {
            voice: Voice::Tongtong,
            speed: 1.0,
            volume: 1.0,
        }
    }
}

/// Zhipu AI API client wrapper
///
/// Provides a unified interface for chat completion and text-to-speech APIs.
/// Supports model selection, thinking mode, and coding plan endpoint.
pub struct Client {
    api_key: String,
    model: Model,
    thinking: bool,
    coding_plan: bool,
}

impl Client {
    /// Create a new client with default settings
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            model: Model::default(),
            thinking: false,
            coding_plan: false,
        }
    }

    /// Set the AI model for chat completion
    pub fn with_model(mut self, model: Model) -> Self {
        self.model = model;
        self
    }

    /// Enable or disable thinking mode for chat completion
    pub fn with_thinking(mut self, enable: bool) -> Self {
        self.thinking = enable;
        self
    }

    /// Enable or disable coding plan endpoint
    pub fn with_coding_plan(mut self, enable: bool) -> Self {
        self.coding_plan = enable;
        self
    }

    /// Perform chat completion
    pub async fn chat_completion(&self, prompt: &str) -> Result<String> {
        let response: ChatCompletionResponse = match self.model {
            Model::GLM4_7 => {
                if self.thinking {
                    self.call_chat_with_thinking(GLM4_7 {}, prompt).await?
                } else {
                    self.call_chat(GLM4_7 {}, prompt).await?
                }
            }
            Model::GLM4_6 => {
                if self.thinking {
                    self.call_chat_with_thinking(GLM4_6 {}, prompt).await?
                } else {
                    self.call_chat(GLM4_6 {}, prompt).await?
                }
            }
            Model::GLM4_5 => {
                if self.thinking {
                    self.call_chat_with_thinking(GLM4_5 {}, prompt).await?
                } else {
                    self.call_chat(GLM4_5 {}, prompt).await?
                }
            }
            Model::GLM4_5Flash => self.call_chat(GLM4_5_flash {}, prompt).await?,
            Model::GLM4_5Air => self.call_chat(GLM4_5_air {}, prompt).await?,
        };

        let content = response
            .choices
            .and_then(|choices: Vec<_>| choices.into_iter().next())
            .and_then(|choice| choice.message.content)
            .and_then(|content| match content {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
            .ok_or_else(|| Error::AiApi("Invalid AI response format".to_string()))?;

        Ok(content)
    }

    /// Perform text-to-audio conversion
    pub async fn text_to_audio(&self, text: &str, config: &TtsConfig) -> Result<Vec<u8>> {
        let request = TextToAudioRequest::new(GlmTts {}, self.api_key.clone())
            .with_input(text)
            .with_voice(config.voice.clone())
            .with_speed(config.speed)
            .with_volume(config.volume)
            .with_response_format(TtsAudioFormat::Wav);

        let response = request
            .post()
            .await
            .map_err(|e| Error::TtsApi(format!("TTS request failed: {}", e)))?;

        let audio_bytes = response
            .bytes()
            .await
            .map_err(|e| Error::TtsApi(format!("Failed to read audio data: {}", e)))?;

        if audio_bytes.is_empty() {
            return Err(Error::TtsApi("Received empty audio data".to_string()));
        }

        Ok(audio_bytes.to_vec())
    }

    async fn call_chat<M>(&self, model: M, prompt: &str) -> Result<ChatCompletionResponse>
    where
        M: ModelName + Chat + Serialize + Send + Sync + 'static,
        (M, TextMessage): Bounded,
    {
        let system_message = TextMessage::system(
            "作为全球顶级的语言学家，你取得了全球所有语种博士学位，
            并且每种语言都拥有100年的使用经验。根据提供的文本，按照语义学进行分段。",
        );
        let mut request = ChatCompletion::new(model, system_message, self.api_key.clone())
            .add_messages(TextMessage::user(prompt));

        if self.coding_plan {
            request = request.with_coding_plan();
        }

        request
            .send()
            .await
            .map_err(|e| Error::AiApi(format!("Chat completion failed: {}", e)))
    }

    async fn call_chat_with_thinking<M>(
        &self,
        model: M,
        prompt: &str,
    ) -> Result<ChatCompletionResponse>
    where
        M: ModelName + Chat + ThinkEnable + Serialize + Send + Sync + 'static,
        (M, TextMessage): Bounded,
    {
        let system_message = TextMessage::system(
            "作为全球顶级的语言学家，你取得了全球所有语种博士学位，
            并且每种语言都拥有100年的使用经验。根据提供的文本，按照语义学进行分段。",
        );
        let mut request = ChatCompletion::new(model, system_message, self.api_key.clone())
            .add_messages(TextMessage::system(prompt));

        if self.coding_plan {
            request = request.with_coding_plan();
        }

        request = request.with_thinking(ThinkingType::Enabled);

        request
            .send()
            .await
            .map_err(|e| Error::AiApi(format!("Chat completion failed: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_as_str() {
        assert_eq!(Model::GLM4_7.as_str(), "glm-4.7");
        assert_eq!(Model::GLM4_6.as_str(), "glm-4.6");
        assert_eq!(Model::GLM4_5.as_str(), "glm-4.5");
        assert_eq!(Model::GLM4_5Flash.as_str(), "glm-4.5-flash");
        assert_eq!(Model::GLM4_5Air.as_str(), "glm-4.5-air");
    }

    #[test]
    fn test_model_default() {
        assert_eq!(Model::default(), Model::GLM4_5Flash);
    }

    #[test]
    fn test_tts_config_default() {
        let config = TtsConfig::default();
        assert!(matches!(config.voice, Voice::Tongtong));
        assert_eq!(config.speed, 1.0);
        assert_eq!(config.volume, 1.0);
    }

    #[test]
    fn test_client_new() {
        let client = Client::new("test_key");
        assert_eq!(client.api_key, "test_key");
        assert_eq!(client.model, Model::default());
        assert!(!client.thinking);
        assert!(!client.coding_plan);
    }

    #[test]
    fn test_client_with_model() {
        let client = Client::new("test_key").with_model(Model::GLM4_7);
        assert_eq!(client.model, Model::GLM4_7);
    }

    #[test]
    fn test_client_with_thinking() {
        let client = Client::new("test_key").with_thinking(true);
        assert!(client.thinking);
    }

    #[test]
    fn test_client_with_coding_plan() {
        let client = Client::new("test_key").with_coding_plan(true);
        assert!(client.coding_plan);
    }

    #[test]
    fn test_client_chaining() {
        let client = Client::new("test_key")
            .with_model(Model::GLM4_6)
            .with_thinking(true)
            .with_coding_plan(true);

        assert_eq!(client.model, Model::GLM4_6);
        assert!(client.thinking);
        assert!(client.coding_plan);
    }
}
