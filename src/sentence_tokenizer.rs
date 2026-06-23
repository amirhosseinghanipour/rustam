use once_cell::sync::Lazy;

use fancy_regex::Regex;

/// Compiled sentence-boundary pattern (Persian + Latin terminators).
static SENTENCE_BOUNDARY: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([!.?⸮؟]+)[ \n]+").expect("valid sentence boundary regex"));

/// Splits Persian (and mixed) text into individual sentences.
///
/// Sentence boundaries are detected at sequences of `.`, `!`, `?`, `؟`, or `⸮`
/// followed by whitespace or a newline.
///
/// # Examples
///
/// ```
/// use rustam::SentenceTokenizer;
///
/// let tokenizer = SentenceTokenizer::new();
/// let sentences = tokenizer.tokenize("جدا کردن ساده است. تقریبا البته!");
/// assert_eq!(sentences, vec!["جدا کردن ساده است.", "تقریبا البته!"]);
/// ```
pub struct SentenceTokenizer;

impl SentenceTokenizer {
    /// Creates a new sentence tokenizer.
    pub fn new() -> Self {
        Self
    }

    /// Splits `text` into a `Vec` of sentence strings.
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let replaced = SENTENCE_BOUNDARY.replace_all(text, "$1\n\n");
        replaced
            .split("\n\n")
            .map(|s| s.replace('\n', " ").trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

impl Default for SentenceTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function: splits `text` into sentences.
pub fn sent_tokenize(text: &str) -> Vec<String> {
    SentenceTokenizer::new().tokenize(text)
}
