//! Language name ↔ ISO code mapping shared by the Qwen3 wrappers.
//!
//! Qwen3 models use full names on the wire (`codec_language_id` keys,
//! `language <Name>` announcements); Crane's audio API uses ISO codes.
//! Unknown values pass through unchanged.

/// `(name, code)` pairs for every Qwen3-TTS and Qwen3-ASR language, sorted
/// by name. `fil`/`yue` have no ISO 639-1 form; 639-3 is used.
const LANGUAGE_TABLE: &[(&str, &str)] = &[
    ("arabic", "ar"),
    ("cantonese", "yue"),
    ("chinese", "zh"),
    ("czech", "cs"),
    ("danish", "da"),
    ("dutch", "nl"),
    ("english", "en"),
    ("filipino", "fil"),
    ("finnish", "fi"),
    ("french", "fr"),
    ("german", "de"),
    ("greek", "el"),
    ("hindi", "hi"),
    ("hungarian", "hu"),
    ("indonesian", "id"),
    ("italian", "it"),
    ("japanese", "ja"),
    ("korean", "ko"),
    ("macedonian", "mk"),
    ("malay", "ms"),
    ("persian", "fa"),
    ("polish", "pl"),
    ("portuguese", "pt"),
    ("romanian", "ro"),
    ("russian", "ru"),
    ("spanish", "es"),
    ("swedish", "sv"),
    ("thai", "th"),
    ("turkish", "tr"),
    ("vietnamese", "vi"),
];

/// BCP-47 language codes Qwen3-ASR claims to support, per its model card.
/// Sorted alphabetically.
pub const LANGUAGES: &[&str] = &[
    "ar", "cs", "da", "de", "el", "en", "es", "fa", "fi", "fil", "fr", "hi", "hu", "id", "it",
    "ja", "ko", "mk", "ms", "nl", "pl", "pt", "ro", "ru", "sv", "th", "tr", "vi", "yue", "zh",
];

/// Maps a language name to its ISO code, case-insensitively. Unrecognized
/// names (future checkpoints, dialect overrides like `beijing_dialect`)
/// pass through unchanged.
pub(super) fn language_name_to_code(name: &str) -> &str {
    let lower = name.to_lowercase();
    LANGUAGE_TABLE
        .iter()
        .find(|entry| entry.0 == lower)
        .map_or(name, |entry| entry.1)
}

/// Maps an ISO code to the lowercase full name Qwen3 models use. Unknown
/// codes, "auto", and full names pass through unchanged.
pub(super) fn language_code_to_name(code: &str) -> String {
    LANGUAGE_TABLE
        .iter()
        .find(|entry| entry.1 == code)
        .map_or_else(|| code.to_string(), |entry| entry.0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_sorted_by_name_with_unique_codes() {
        let names: Vec<_> = LANGUAGE_TABLE.iter().map(|(name, _)| *name).collect();
        let mut sorted = names.clone();
        sorted.sort_unstable();
        assert_eq!(names, sorted);

        let mut codes: Vec<_> = LANGUAGE_TABLE.iter().map(|(_, code)| *code).collect();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(codes.len(), LANGUAGE_TABLE.len());
    }

    /// Codes must match `LANGUAGES`and the model card.
    #[test]
    fn table_codes_match_languages() {
        let mut codes: Vec<_> = LANGUAGE_TABLE.iter().map(|(_, code)| *code).collect();
        codes.sort_unstable();
        assert_eq!(codes, LANGUAGES);
    }

    #[test]
    fn name_to_code_known_names() {
        assert_eq!(language_name_to_code("chinese"), "zh");
        assert_eq!(language_name_to_code("english"), "en");
        assert_eq!(language_name_to_code("cantonese"), "yue");
        assert_eq!(language_name_to_code("filipino"), "fil");
        assert_eq!(language_name_to_code("macedonian"), "mk");
    }

    #[test]
    fn name_to_code_is_case_insensitive() {
        assert_eq!(language_name_to_code("English"), "en");
        assert_eq!(language_name_to_code("ENGLISH"), "en");
        assert_eq!(language_name_to_code("cHINese"), "zh");
    }

    #[test]
    fn name_to_code_passthrough() {
        // Dialect overrides and unknown names pass through.
        assert_eq!(language_name_to_code("beijing_dialect"), "beijing_dialect");
        assert_eq!(language_name_to_code("klingon"), "klingon");
    }

    #[test]
    fn code_to_name_known_codes() {
        assert_eq!(language_code_to_name("zh"), "chinese");
        assert_eq!(language_code_to_name("en"), "english");
        assert_eq!(language_code_to_name("de"), "german");
        assert_eq!(language_code_to_name("ja"), "japanese");
        assert_eq!(language_code_to_name("ko"), "korean");
        assert_eq!(language_code_to_name("fr"), "french");
        assert_eq!(language_code_to_name("ru"), "russian");
        assert_eq!(language_code_to_name("it"), "italian");
        assert_eq!(language_code_to_name("pt"), "portuguese");
        assert_eq!(language_code_to_name("es"), "spanish");
        assert_eq!(language_code_to_name("yue"), "cantonese");
    }

    #[test]
    fn code_to_name_passthrough() {
        // Full names pass through (backwards compatibility).
        assert_eq!(language_code_to_name("english"), "english");
        assert_eq!(language_code_to_name("chinese"), "chinese");
        assert_eq!(language_code_to_name("auto"), "auto");
        assert_eq!(language_code_to_name("xx"), "xx");
    }

    #[test]
    fn name_and_code_mapping_roundtrips() {
        for (name, code) in LANGUAGE_TABLE {
            assert_eq!(language_name_to_code(&capitalize(name)), *code);
            assert_eq!(language_code_to_name(code), *name);
        }
    }

    /// Capitalizes like Qwen3-ASR announcements (`"english"` into `"English"`).
    fn capitalize(name: &str) -> String {
        let mut chars = name.chars();
        match chars.next() {
            Some(first) => {
                first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
            },
            None => String::new(),
        }
    }
}
