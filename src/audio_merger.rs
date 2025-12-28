use crate::error::Result;
use hound::{WavReader, WavSpec, WavWriter};
use std::io::Cursor;

/// Audio merger for combining multiple audio segments into a single WAV file
///
/// Uses the hound library to read and write WAV files with proper format handling.
pub struct AudioMerger;

impl AudioMerger {
    /// Merge multiple audio byte segments into a single WAV file
    ///
    /// All audio segments must have the same sample rate and format.
    ///
    /// # Arguments
    ///
    /// * `audio_segments` - Vector of audio data bytes
    /// * `output_path` - Path to save the merged WAV file
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - No audio segments provided
    /// - Audio segments have incompatible formats
    /// - File I/O fails
    pub async fn merge(audio_segments: Vec<Vec<u8>>, output_path: &str) -> Result<()> {
        if audio_segments.is_empty() {
            return Err(crate::error::Error::Audio(
                "No audio segments to merge".to_string(),
            ));
        }

        // Get spec from first segment
        let first_spec = Self::extract_wav_spec(&audio_segments[0])?;

        // Create output writer with first segment's spec
        let spec = first_spec;
        let mut writer = WavWriter::create(output_path, spec)?;

        // Write each audio segment
        for (idx, segment) in audio_segments.iter().enumerate() {
            Self::write_segment(&mut writer, segment, idx)?;
        }

        writer.finalize()?;
        Ok(())
    }

    /// Convert a single audio segment to WAV file
    ///
    /// # Arguments
    ///
    /// * `audio_bytes` - Raw audio data in WAV format
    /// * `output_path` - Path to save the WAV file
    pub async fn save_single(audio_bytes: &[u8], output_path: &str) -> Result<()> {
        if audio_bytes.is_empty() {
            return Err(crate::error::Error::Audio("Empty audio data".to_string()));
        }

        let cursor = Cursor::new(audio_bytes);
        let mut reader = WavReader::new(cursor)
            .map_err(|e| crate::error::Error::Audio(format!("Invalid WAV format: {}", e)))?;

        let spec = reader.spec();
        let mut writer = WavWriter::create(output_path, spec)?;

        for sample in reader.samples::<i16>() {
            writer.write_sample(sample?)?;
        }

        writer.finalize()?;
        Ok(())
    }

    /// Extract WAV specification from audio bytes
    fn extract_wav_spec(audio_bytes: &[u8]) -> Result<WavSpec> {
        let cursor = Cursor::new(audio_bytes);
        let reader = WavReader::new(cursor)
            .map_err(|e| crate::error::Error::Audio(format!("Invalid WAV format: {}", e)))?;

        Ok(reader.spec())
    }

    /// Write a single audio segment to the WAV writer
    fn write_segment(
        writer: &mut WavWriter<std::io::BufWriter<std::fs::File>>,
        segment: &[u8],
        idx: usize,
    ) -> Result<()> {
        let cursor = Cursor::new(segment);
        let mut reader = WavReader::new(cursor).map_err(|e| {
            crate::error::Error::Audio(format!("Segment {} invalid WAV: {}", idx, e))
        })?;

        for sample in reader.samples::<i16>() {
            writer.write_sample(sample?)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Actual audio tests require real WAV data
    // These are placeholder tests for structure

    #[test]
    fn test_empty_segments() {
        let result = std::thread::spawn(|| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(AudioMerger::merge(vec![], "output.wav"))
        })
        .join()
        .unwrap();

        assert!(result.is_err());
    }

    #[test]
    fn test_empty_single() {
        let result = std::thread::spawn(|| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(AudioMerger::save_single(&[], "output.wav"))
        })
        .join()
        .unwrap();

        assert!(result.is_err());
    }
}
