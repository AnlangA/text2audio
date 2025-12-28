use thiserror::Error;

/// Error types for text2audio library
#[derive(Error, Debug)]
pub enum Error {
    /// AI API call failed
    #[error("AI API error: {0}")]
    AiApi(String),

    /// TTS API call failed
    #[error("TTS API error: {0}")]
    TtsApi(String),

    /// Audio processing failed
    #[error("Audio processing error: {0}")]
    Audio(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Hound audio library error
    #[error("Audio library error: {0}")]
    Hound(#[from] hound::Error),

    /// HTTP error
    #[error("HTTP error: {0}")]
    Http(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    Config(String),

    /// Empty input text
    #[error("Input text is empty")]
    EmptyInput,
}

pub type Result<T> = std::result::Result<T, Error>;
