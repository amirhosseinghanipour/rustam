use once_cell::sync::Lazy;

use crate::constants::SUFFIXES;

/// Suffixes sorted longest-first, used for greedy stripping.
static SORTED_SUFFIXES: Lazy<Vec<&'static str>> = Lazy::new(|| {
    let mut v: Vec<&str> = SUFFIXES.to_vec();
    v.sort_by_key(|b| std::cmp::Reverse(b.len()));
    v
});

/// Rule-based Persian stemmer.
///
/// Strips one inflectional suffix from the word by trying the longest match
/// first.  Unlike [`Lemmatizer`](crate::Lemmatizer), the `Stemmer` has no
/// vocabulary knowledge and may return an incorrect stem for ambiguous words.
///
/// # Examples
///
/// ```
/// use rustam::Stemmer;
///
/// let stemmer = Stemmer::new();
/// assert_eq!(stemmer.stem("کتاب‌ها"), "کتاب");
/// assert_eq!(stemmer.stem("کتابی"),   "کتاب");
/// assert_eq!(stemmer.stem("خانۀ"),    "خانه");
/// ```
pub struct Stemmer;

impl Stemmer {
    /// Creates a new stemmer.
    pub fn new() -> Self {
        Self
    }

    /// Returns the stem of `word` after removing the first matching suffix.
    pub fn stem(&self, word: &str) -> String {
        let mut result = word.to_string();

        for &suffix in SORTED_SUFFIXES.iter() {
            if result.ends_with(suffix) {
                let stem_len = result.len() - suffix.len();
                // single-char suffix requires a minimum stem length of 3 chars
                if suffix.chars().count() == 1 && result[..stem_len].chars().count() < 3 {
                    continue;
                }
                result.truncate(stem_len);
                break;
            }
        }

        // ۀ (U+06C0 heh with hamza above) → ه (U+0647 heh)
        if result.ends_with('\u{06C0}') {
            result.pop();
            result.push('\u{0647}');
        }

        // bare trailing ZWNJ
        if result.ends_with('\u{200C}') {
            result.pop();
        }

        result
    }
}

impl Default for Stemmer {
    fn default() -> Self {
        Self::new()
    }
}
