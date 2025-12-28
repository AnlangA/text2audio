use crate::client::{Client, Model};
use crate::error::Result;

/// Default delimiter for AI-split text segments
const SEGMENT_DELIMITER: &str = "|||";

/// AI-powered text splitter using GLM models
///
/// Uses AI to semantically split long text while maintaining coherence.
pub struct AiSplitter {
    client: Client,
    max_length: usize,
}

impl AiSplitter {
    /// Create a new AI splitter with the specified model
    ///
    /// # Examples
    ///
    /// ```
    /// use text2audio::ai_splitter::AiSplitter;
    /// use text2audio::client::Model;
    ///
    /// let splitter = AiSplitter::new("api_key", Model::GLM4_5Flash, 1000);
    /// ```
    pub fn new(api_key: impl Into<String>, model: Model, max_length: usize) -> Self {
        let client = Client::new(api_key).with_model(model);
        Self { client, max_length }
    }

    /// Enable or disable thinking mode for better semantic understanding
    ///
    /// When enabled, the AI model will think before generating the split,
    /// which can improve semantic coherence but increases processing time.
    pub fn with_thinking(mut self, enable: bool) -> Self {
        self.client = self.client.with_thinking(enable);
        self
    }

    /// Enable or disable coding plan endpoint
    ///
    /// When enabled, uses the coding plan API endpoint for code/structured text analysis.
    pub fn with_coding_plan(mut self, enable: bool) -> Self {
        self.client = self.client.with_coding_plan(enable);
        self
    }

    /// Split text using AI to ensure semantic coherence
    ///
    /// # Process
    ///
    /// 1. If text is short enough, return as-is
    /// 2. Send to AI model with splitting instructions
    /// 3. Parse AI response using delimiter
    pub async fn split(&self, text: &str) -> Result<Vec<String>> {
        let char_count = text.chars().count();

        if char_count == 0 {
            return Ok(vec![]);
        }

        if char_count <= self.max_length {
            return Ok(vec![text.to_string()]);
        }

        let prompt = self.build_prompt(text);
        let raw_response = self.client.chat_completion(&prompt).await?;
        self.parse_segments(&raw_response)
    }

    fn build_prompt(&self, text: &str) -> String {
        format!(
            "请将以下文本分割成多个段落，每个段落的字符数不超过 {} 字符。\
            分割时要保持语义完整性，优先按照句子的自然边界（如句号、问号、感叹号）进行分割。\
            分割后，请按顺序输出每个段落，每个段落用特殊标记 ||| 分隔。\
            不要添加任何解释性文字，只输出分割后的段落。\n\n待分割的文本：\n{}",
            self.max_length, text
        )
    }

    fn parse_segments(&self, raw_response: &str) -> Result<Vec<String>> {
        let segments: Vec<String> = raw_response
            .split(SEGMENT_DELIMITER)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if segments.is_empty() {
            return Ok(vec![raw_response.to_string()]);
        }

        Ok(segments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_splitter_new() {
        let splitter = AiSplitter::new("api_key", Model::GLM4_7, 1000);
        assert_eq!(splitter.max_length, 1000);
    }

    #[test]
    fn test_ai_splitter_with_thinking() {
        let _splitter = AiSplitter::new("api_key", Model::GLM4_7, 1000).with_thinking(true);
    }

    #[test]
    fn test_ai_splitter_with_coding_plan() {
        let _splitter = AiSplitter::new("api_key", Model::GLM4_7, 1000).with_coding_plan(true);
    }

    #[test]
    fn test_build_prompt() {
        let splitter = AiSplitter::new("api_key", Model::GLM4_7, 100);
        let text = "Hello world!";
        let prompt = splitter.build_prompt(text);
        assert!(prompt.contains("100"));
        assert!(prompt.contains(text));
        assert!(prompt.contains("|||"));
    }

    #[test]
    fn test_parse_segments() {
        let splitter = AiSplitter::new("api_key", Model::GLM4_7, 100);
        let raw = "First segment|||Second segment|||Third segment";
        let segments = splitter.parse_segments(raw).unwrap();
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0], "First segment");
        assert_eq!(segments[1], "Second segment");
        assert_eq!(segments[2], "Third segment");
    }

    #[test]
    fn test_parse_segments_empty() {
        let splitter = AiSplitter::new("api_key", Model::GLM4_7, 100);
        let raw = "No delimiters here";
        let segments = splitter.parse_segments(raw).unwrap();
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0], "No delimiters here");
    }
}
