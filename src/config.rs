/// Voice selection for TTS
///
/// Maps directly to zai-rs Voice enum.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Voice {
    #[default]
    Tongtong,
    Chuichui,
    Xiaochen,
    Jam,
    Kazi,
    Douji,
    Luodo,
}

impl Voice {
    /// Convert to zai-rs Voice
    pub fn as_tts_voice(&self) -> zai_rs::model::text_to_audio::request::Voice {
        match self {
            Voice::Tongtong => zai_rs::model::text_to_audio::request::Voice::Tongtong,
            Voice::Chuichui => zai_rs::model::text_to_audio::request::Voice::Chuichui,
            Voice::Xiaochen => zai_rs::model::text_to_audio::request::Voice::Xiaochen,
            Voice::Jam => zai_rs::model::text_to_audio::request::Voice::Jam,
            Voice::Kazi => zai_rs::model::text_to_audio::request::Voice::Kazi,
            Voice::Douji => zai_rs::model::text_to_audio::request::Voice::Douji,
            Voice::Luodo => zai_rs::model::text_to_audio::request::Voice::Luodo,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Voice::Tongtong => "Tongtong",
            Voice::Chuichui => "Chuichui",
            Voice::Xiaochen => "Xiaochen",
            Voice::Jam => "Jam",
            Voice::Kazi => "Kazi",
            Voice::Douji => "Douji",
            Voice::Luodo => "Luodo",
        }
    }
}

impl std::fmt::Display for Voice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_default() {
        assert_eq!(Voice::default(), Voice::Tongtong);
    }

    #[test]
    fn test_voice_as_tts_voice() {
        assert!(matches!(
            Voice::Tongtong.as_tts_voice(),
            zai_rs::model::text_to_audio::request::Voice::Tongtong
        ));
        assert!(matches!(
            Voice::Xiaochen.as_tts_voice(),
            zai_rs::model::text_to_audio::request::Voice::Xiaochen
        ));
        assert!(matches!(
            Voice::Jam.as_tts_voice(),
            zai_rs::model::text_to_audio::request::Voice::Jam
        ));
        assert!(matches!(
            Voice::Kazi.as_tts_voice(),
            zai_rs::model::text_to_audio::request::Voice::Kazi
        ));
        assert!(matches!(
            Voice::Douji.as_tts_voice(),
            zai_rs::model::text_to_audio::request::Voice::Douji
        ));
        assert!(matches!(
            Voice::Luodo.as_tts_voice(),
            zai_rs::model::text_to_audio::request::Voice::Luodo
        ));
    }

    #[test]
    fn test_voice_as_str() {
        assert_eq!(Voice::Tongtong.as_str(), "Tongtong");
        assert_eq!(Voice::Chuichui.as_str(), "Chuichui");
    }

    #[test]
    fn test_voice_display() {
        assert_eq!(format!("{}", Voice::Jam), "Jam");
    }
}
