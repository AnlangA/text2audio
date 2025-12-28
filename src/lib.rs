pub mod ai_splitter;
pub mod audio_merger;
pub mod client;
pub mod config;
pub mod error;

pub use ai_splitter::AiSplitter;
pub use audio_merger::AudioMerger;
pub use client::{Client, Model, TtsConfig};
pub use config::Voice;
pub use error::{Error, Result};

use futures::stream::{self, StreamExt};
use std::time::Duration;

/// Main entry point for text-to-audio conversion
///
/// # Examples
///
/// ```
/// use text2audio::Text2Audio;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let api_key = std::env::var("ZHIPU_API_KEY")?;
/// let converter = Text2Audio::new(&api_key);
/// converter.convert("你好，世界！", "output.wav").await?;
/// # Ok(())
/// # }
/// ```
pub struct Text2Audio {
    api_key: String,
    model: Model,
    voice: Voice,
    speed: f32,
    volume: f32,
    max_segment_length: usize,
    enable_parallel: bool,
    max_parallel: usize,
    max_retries: u32,
    retry_delay: Duration,
    enable_thinking: bool,
    coding_plan: bool,
}

impl Text2Audio {
    /// Create a new Text2Audio converter with default settings
    ///
    /// # Arguments
    ///
    /// * `api_key` - Zhipu AI API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            model: Model::default(),
            voice: Voice::default(),
            speed: 1.0,
            volume: 1.0,
            max_segment_length: 500,
            enable_parallel: false,
            max_parallel: 3,
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
            enable_thinking: false,
            coding_plan: false,
        }
    }

    /// Create a builder for Text2Audio configuration
    ///
    /// # Arguments
    ///
    /// * `api_key` - Zhipu AI API key
    pub fn builder(api_key: impl Into<String>) -> Builder {
        Builder::new(api_key)
    }

    /// Set the AI model for text splitting
    ///
    /// # Arguments
    ///
    /// * `model` - AI model to use for splitting
    ///
    /// # Examples
    ///
    /// ```
    /// use text2audio::{Text2Audio, Model};
    ///
    /// let converter = Text2Audio::new("api_key")
    ///     .with_model(Model::GLM4_7);
    /// ```
    pub fn with_model(mut self, model: Model) -> Self {
        self.model = model;
        self
    }

    /// Set the voice type for TTS
    ///
    /// # Arguments
    ///
    /// * `voice` - Voice selection from available voices
    ///
    /// # Examples
    ///
    /// ```
    /// use text2audio::{Text2Audio, Voice};
    ///
    /// let converter = Text2Audio::new("api_key")
    ///     .with_voice(Voice::Xiaochen);
    /// ```
    pub fn with_voice(mut self, voice: Voice) -> Self {
        self.voice = voice;
        self
    }

    /// Set the speech speed
    ///
    /// # Arguments
    ///
    /// * `speed` - Speech speed between 0.5 (slow) and 2.0 (fast)
    ///
    /// # Examples
    ///
    /// ```
    /// use text2audio::Text2Audio;
    ///
    /// let converter = Text2Audio::new("api_key")
    ///     .with_speed(1.5);
    /// ```
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed.clamp(0.5, 2.0);
        self
    }

    /// Set the speech volume
    ///
    /// # Arguments
    ///
    /// * `volume` - Speech volume between 0.0 (silent) and 10.0 (loud)
    ///
    /// # Examples
    ///
    /// ```
    /// use text2audio::Text2Audio;
    ///
    /// let converter = Text2Audio::new("api_key")
    ///     .with_volume(3.0);
    /// ```
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 10.0);
        self
    }

    /// Set the maximum segment length
    ///
    /// # Arguments
    ///
    /// * `max_length` - Maximum length per segment (100-1024 characters)
    ///
    /// # Examples
    ///
    /// ```
    /// use text2audio::Text2Audio;
    ///
    /// let converter = Text2Audio::new("api_key")
    ///     .with_max_segment_length(800);
    /// ```
    pub fn with_max_segment_length(mut self, max_length: usize) -> Self {
        self.max_segment_length = max_length.clamp(100, 1024);
        self
    }

    /// Enable parallel processing of audio segments
    ///
    /// # Arguments
    ///
    /// * `max_parallel` - Maximum number of parallel requests (1-10)
    ///
    /// # Examples
    ///
    /// ```
    /// use text2audio::Text2Audio;
    ///
    /// let converter = Text2Audio::new("api_key")
    ///     .with_parallel(5);
    /// ```
    pub fn with_parallel(mut self, max_parallel: usize) -> Self {
        self.enable_parallel = true;
        self.max_parallel = max_parallel.clamp(1, 10);
        self
    }

    /// Enable thinking mode for AI splitting
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable thinking
    ///
    /// # Examples
    ///
    /// ```
    /// use text2audio::Text2Audio;
    ///
    /// let converter = Text2Audio::new("api_key")
    ///     .with_thinking(true);
    /// ```
    pub fn with_thinking(mut self, enable: bool) -> Self {
        self.enable_thinking = enable;
        self
    }

    /// Enable coding plan endpoint
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to enable coding plan
    ///
    /// # Examples
    ///
    /// ```
    /// use text2audio::Text2Audio;
    ///
    /// let converter = Text2Audio::new("api_key")
    ///     .with_coding_plan(true);
    /// ```
    pub fn with_coding_plan(mut self, enable: bool) -> Self {
        self.coding_plan = enable;
        self
    }

    /// Set retry configuration for API calls
    ///
    /// # Arguments
    ///
    /// * `max_retries` - Maximum number of retry attempts on failure
    /// * `retry_delay` - Initial delay between retries (exponential backoff is applied)
    ///
    /// # Examples
    ///
    /// ```
    /// use text2audio::Text2Audio;
    /// use std::time::Duration;
    ///
    /// let converter = Text2Audio::new("api_key")
    ///     .with_retry_config(5, Duration::from_millis(200));
    /// ```
    pub fn with_retry_config(mut self, max_retries: u32, retry_delay: Duration) -> Self {
        self.max_retries = max_retries;
        self.retry_delay = retry_delay;
        self
    }

    /// Convert text to audio file
    ///
    /// Automatically determines whether to use segmented or direct mode
    /// based on text length. AI splitting is used when needed.
    ///
    /// # Arguments
    ///
    /// * `text` - Input text to convert
    /// * `output_path` - Output WAV file path
    ///
    /// # Errors
    ///
    /// Returns error if text processing, API calls, or audio processing fail.
    pub async fn convert(&self, text: &str, output_path: &str) -> Result<()> {
        let text = text.trim();
        if text.is_empty() {
            return Err(Error::EmptyInput);
        }

        let char_count = text.chars().count();

        if char_count <= self.max_segment_length {
            self.convert_direct(text, output_path).await
        } else {
            self.convert_segmented(text, output_path).await
        }
    }

    async fn convert_direct(&self, text: &str, output_path: &str) -> Result<()> {
        let audio_bytes = self.text_to_audio_with_retry(text).await?;
        AudioMerger::save_single(&audio_bytes, output_path).await
    }

    async fn convert_segmented(&self, text: &str, output_path: &str) -> Result<()> {
        let splitter = AiSplitter::new(self.api_key.clone(), self.model, self.max_segment_length)
            .with_thinking(self.enable_thinking)
            .with_coding_plan(self.coding_plan);

        let segments = splitter.split(text).await?;

        if segments.is_empty() {
            return Err(Error::EmptyInput);
        }

        let audio_segments = if self.enable_parallel {
            self.collect_audio_parallel(&segments).await?
        } else {
            self.collect_audio_sequential(&segments).await?
        };

        AudioMerger::merge(audio_segments, output_path).await
    }

    async fn text_to_audio_with_retry(&self, text: &str) -> Result<Vec<u8>> {
        let mut last_error = None;

        for attempt in 0..self.max_retries {
            match self.try_convert(text).await {
                Ok(audio) => return Ok(audio),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.max_retries - 1 {
                        let delay = self.retry_delay * 2_u32.pow(attempt);
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| Error::TtsApi("Unknown error".to_string())))
    }

    async fn try_convert(&self, text: &str) -> Result<Vec<u8>> {
        let tts_config = TtsConfig {
            voice: self.voice.as_tts_voice(),
            speed: self.speed,
            volume: self.volume,
        };

        let client = Client::new(self.api_key.clone());
        client
            .text_to_audio(text, &tts_config)
            .await
            .map_err(|e| Error::TtsApi(format!("TTS request failed: {}", e)))
    }

    async fn collect_audio_sequential(&self, segments: &[String]) -> Result<Vec<Vec<u8>>> {
        let mut audio_segments = Vec::new();

        for segment in segments {
            let audio_bytes = self.text_to_audio_with_retry(segment).await?;
            audio_segments.push(audio_bytes);
        }

        Ok(audio_segments)
    }

    async fn collect_audio_parallel(&self, segments: &[String]) -> Result<Vec<Vec<u8>>> {
        let api_key = self.api_key.clone();
        let speed = self.speed;
        let volume = self.volume;
        let voice = self.voice.as_tts_voice();
        let max_retries = self.max_retries;
        let retry_delay = self.retry_delay;
        let max_parallel = self.max_parallel;

        let results = stream::iter(segments)
            .map(move |segment| {
                let api_key = api_key.clone();
                let segment = segment.clone();
                let voice = voice.clone();

                async move {
                    let tts_config = TtsConfig {
                        voice: voice.clone(),
                        speed,
                        volume,
                    };

                    let mut last_error: Option<Error> = None;
                    for attempt in 0..max_retries {
                        let client = Client::new(api_key.clone());
                        match client.text_to_audio(&segment, &tts_config).await {
                            Ok(bytes) => return Ok::<Vec<u8>, Error>(bytes),
                            Err(e) => {
                                last_error =
                                    Some(Error::TtsApi(format!("Retry {}: {}", attempt, e)));
                                if attempt < max_retries - 1 {
                                    tokio::time::sleep(retry_delay * 2_u32.pow(attempt)).await;
                                }
                            }
                        }
                    }
                    if let Some(e) = last_error {
                        Err(e)
                    } else {
                        Err(Error::TtsApi("All retry attempts failed".to_string()))
                    }
                }
            })
            .buffer_unordered(max_parallel)
            .collect::<Vec<_>>()
            .await;

        let mut audio_segments = Vec::new();
        for result in results {
            audio_segments.push(result?);
        }

        Ok(audio_segments)
    }
}

impl Default for Text2Audio {
    fn default() -> Self {
        Self::new("")
    }
}

/// Builder for Text2Audio configuration
///
/// Provides a fluent interface for configuring text-to-audio conversion.
pub struct Builder {
    converter: Text2Audio,
}

impl Builder {
    fn new(api_key: impl Into<String>) -> Self {
        Self {
            converter: Text2Audio::new(api_key),
        }
    }

    /// Set the AI model for text splitting
    pub fn model(mut self, model: Model) -> Self {
        self.converter = self.converter.with_model(model);
        self
    }

    /// Set the voice type for TTS
    pub fn voice(mut self, voice: Voice) -> Self {
        self.converter = self.converter.with_voice(voice);
        self
    }

    /// Set the speech speed
    pub fn speed(mut self, speed: f32) -> Self {
        self.converter = self.converter.with_speed(speed);
        self
    }

    /// Set the speech volume
    pub fn volume(mut self, volume: f32) -> Self {
        self.converter = self.converter.with_volume(volume);
        self
    }

    /// Set the maximum segment length
    pub fn max_segment_length(mut self, max_length: usize) -> Self {
        self.converter = self.converter.with_max_segment_length(max_length);
        self
    }

    /// Enable parallel processing
    pub fn parallel(mut self, max_parallel: usize) -> Self {
        self.converter = self.converter.with_parallel(max_parallel);
        self
    }

    /// Enable thinking mode for AI splitting
    pub fn thinking(mut self, enable: bool) -> Self {
        self.converter = self.converter.with_thinking(enable);
        self
    }

    /// Enable coding plan endpoint
    pub fn coding_plan(mut self, enable: bool) -> Self {
        self.converter = self.converter.with_coding_plan(enable);
        self
    }

    /// Set retry configuration
    pub fn retry_config(mut self, max_retries: u32, delay: Duration) -> Self {
        self.converter = self.converter.with_retry_config(max_retries, delay);
        self
    }

    /// Build the Text2Audio converter
    pub fn build(self) -> Text2Audio {
        self.converter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let converter = Text2Audio::new("test_key");
        assert_eq!(converter.model, Model::default());
        assert_eq!(converter.voice, Voice::default());
        assert_eq!(converter.speed, 1.0);
        assert_eq!(converter.volume, 1.0);
        assert_eq!(converter.max_segment_length, 500);
    }

    #[test]
    fn test_with_model() {
        let converter = Text2Audio::new("test_key").with_model(Model::GLM4_7);
        assert_eq!(converter.model, Model::GLM4_7);
    }

    #[test]
    fn test_with_voice() {
        let converter = Text2Audio::new("test_key").with_voice(Voice::Xiaochen);
        assert_eq!(converter.voice, Voice::Xiaochen);
    }

    #[test]
    fn test_with_speed() {
        let converter = Text2Audio::new("test_key").with_speed(1.2);
        assert_eq!(converter.speed, 1.2);
    }

    #[test]
    fn test_speed_clamp() {
        let converter = Text2Audio::new("test_key").with_speed(3.0);
        assert_eq!(converter.speed, 2.0);

        let converter = Text2Audio::new("test_key").with_speed(0.2);
        assert_eq!(converter.speed, 0.5);
    }

    #[test]
    fn test_with_volume() {
        let converter = Text2Audio::new("test_key").with_volume(2.5);
        assert_eq!(converter.volume, 2.5);
    }

    #[test]
    fn test_volume_clamp() {
        let converter = Text2Audio::new("test_key").with_volume(15.0);
        assert_eq!(converter.volume, 10.0);

        let converter = Text2Audio::new("test_key").with_volume(-1.0);
        assert_eq!(converter.volume, 0.0);
    }

    #[test]
    fn test_with_max_segment_length() {
        let converter = Text2Audio::new("test_key").with_max_segment_length(800);
        assert_eq!(converter.max_segment_length, 800);
    }

    #[test]
    fn test_max_segment_length_clamp() {
        let converter = Text2Audio::new("test_key").with_max_segment_length(50);
        assert_eq!(converter.max_segment_length, 100);

        let converter = Text2Audio::new("test_key").with_max_segment_length(2000);
        assert_eq!(converter.max_segment_length, 1024);
    }

    #[test]
    fn test_with_parallel() {
        let converter = Text2Audio::new("test_key").with_parallel(5);
        assert!(converter.enable_parallel);
        assert_eq!(converter.max_parallel, 5);
    }

    #[test]
    fn test_parallel_clamp() {
        let converter = Text2Audio::new("test_key").with_parallel(20);
        assert_eq!(converter.max_parallel, 10);

        let converter = Text2Audio::new("test_key").with_parallel(0);
        assert_eq!(converter.max_parallel, 1);
    }

    #[test]
    fn test_with_thinking() {
        let converter = Text2Audio::new("test_key").with_thinking(true);
        assert!(converter.enable_thinking);
    }

    #[test]
    fn test_with_coding_plan() {
        let converter = Text2Audio::new("test_key").with_coding_plan(true);
        assert!(converter.coding_plan);
    }

    #[test]
    fn test_builder() {
        let converter = Text2Audio::builder("api_key")
            .model(Model::GLM4_7)
            .voice(Voice::Tongtong)
            .speed(1.5)
            .volume(3.0)
            .max_segment_length(300)
            .parallel(4)
            .thinking(true)
            .coding_plan(false)
            .build();

        assert_eq!(converter.model, Model::GLM4_7);
        assert_eq!(converter.voice, Voice::Tongtong);
        assert_eq!(converter.speed, 1.5);
        assert_eq!(converter.volume, 3.0);
        assert_eq!(converter.max_segment_length, 300);
        assert!(converter.enable_parallel);
        assert_eq!(converter.max_parallel, 4);
        assert!(converter.enable_thinking);
        assert!(!converter.coding_plan);
    }

    #[test]
    fn test_default() {
        let converter = Text2Audio::default();
        assert_eq!(converter.api_key, "");
    }
}
