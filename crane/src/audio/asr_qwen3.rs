//! [`Asr`] trait implementation for [`crane_core::models::qwen3_asr::Model`].

use anyhow::Result;
use crane_core::models::qwen3_asr::Model;

use super::asr::{Asr, TranscribeOptions, Transcript};
use super::language_qwen3::{LANGUAGES, language_name_to_code};

/// BCP-47 codes Qwen3-ASR claims to support; see [`LANGUAGES`].
fn supported_languages() -> Vec<String> {
    LANGUAGES.iter().map(ToString::to_string).collect()
}

impl Asr for Model {
    fn input_sample_rate(&self) -> u32 {
        self.sample_rate()
    }

    /// Delegates to [`Model::transcribe`], mapping the announced language
    /// name to its ISO code ("English" into "en"); unrecognized names pass
    /// through.
    fn transcribe(&mut self, audio: &[f32], opts: &TranscribeOptions) -> Result<Transcript> {
        let mut transcript = Self::transcribe(self, audio, opts)?;
        if let Some(name) = transcript.language.take() {
            transcript.language = Some(language_name_to_code(&name).to_string());
        }
        Ok(transcript)
    }

    fn supported_languages(&self) -> Vec<String> {
        supported_languages()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_languages_is_sorted_and_nonempty() {
        let languages = supported_languages();
        assert!(!languages.is_empty());
        let mut sorted = languages.clone();
        sorted.sort_unstable();
        assert_eq!(languages, sorted);
    }

    #[test]
    fn supported_languages_contains_expected_codes() {
        let languages = supported_languages();
        for code in ["en", "zh", "de"] {
            assert!(
                languages.contains(&code.to_string()),
                "missing language code {code}"
            );
        }
    }
}
