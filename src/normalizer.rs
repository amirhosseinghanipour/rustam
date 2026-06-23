use std::collections::{HashMap, HashSet};

use fancy_regex::Regex;
use once_cell::sync::Lazy;

use crate::{
    constants::{
        AFFIX_SPACING_PATTERNS, DIACRITICS_PATTERN, EXTRA_SPACE_PATTERNS,
        MORE_THAN_TWO_REPEAT_PATTERN, NUMBERS_DST, NUMBERS_SRC, PERSIAN_STYLE_PATTERNS,
        PUNCTUATION_SPACING_PATTERNS, REPEATED_CHARS_PATTERN, SEPERATE_MI_PATTERN,
        SPECIAL_CHARS_PATTERN, SUFFIXES, TRANSLATION_DST, TRANSLATION_SRC, UNICODE_REPLACEMENTS,
    },
    translate::{make_trans, translate},
    word_tokenizer::{WordTokenizer, WordTokenizerConfig},
};

// ---------------------------------------------------------------------------
// Shared lazily-compiled translation tables
// ---------------------------------------------------------------------------

static CHAR_TRANS: Lazy<HashMap<char, char>> =
    Lazy::new(|| make_trans(TRANSLATION_SRC, TRANSLATION_DST));

static NUMBER_TRANS: Lazy<HashMap<char, char>> =
    Lazy::new(|| make_trans(NUMBERS_SRC, NUMBERS_DST));

// ---------------------------------------------------------------------------
// Helper: compile a list of (pattern_str, replacement) pairs into Regex pairs
// ---------------------------------------------------------------------------

fn compile_patterns(pairs: &[(&str, &str)]) -> Vec<(Regex, String)> {
    pairs
        .iter()
        .map(|(pat, repl)| {
            let re = Regex::new(pat)
                .unwrap_or_else(|e| panic!("failed to compile normalizer regex '{pat}': {e}"));
            (re, repl.to_string())
        })
        .collect()
}

fn apply_patterns(patterns: &[(Regex, String)], mut text: String) -> String {
    for (re, repl) in patterns {
        text = re.replace_all(&text, repl.as_str()).into_owned();
    }
    text
}

// ---------------------------------------------------------------------------
// Lazily compiled pattern groups
// ---------------------------------------------------------------------------

static EXTRA_SPACE_RE: Lazy<Vec<(Regex, String)>> =
    Lazy::new(|| compile_patterns(EXTRA_SPACE_PATTERNS));

static PUNCT_SPACING_RE: Lazy<Vec<(Regex, String)>> =
    Lazy::new(|| compile_patterns(PUNCTUATION_SPACING_PATTERNS));

static AFFIX_SPACING_RE: Lazy<Vec<(Regex, String)>> =
    Lazy::new(|| compile_patterns(AFFIX_SPACING_PATTERNS));

static PERSIAN_STYLE_RE: Lazy<Vec<(Regex, String)>> =
    Lazy::new(|| compile_patterns(PERSIAN_STYLE_PATTERNS));

static DIACRITICS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(DIACRITICS_PATTERN).expect("DIACRITICS_PATTERN"));

static SPECIAL_CHARS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(SPECIAL_CHARS_PATTERN).expect("SPECIAL_CHARS_PATTERN"));

static REPEATED_CHARS_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(REPEATED_CHARS_PATTERN).expect("REPEATED_CHARS_PATTERN"));

static MORE_THAN_TWO_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(MORE_THAN_TWO_REPEAT_PATTERN).expect("MORE_THAN_TWO_REPEAT_PATTERN"));

static SEPERATE_MI_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(SEPERATE_MI_PATTERN).expect("SEPERATE_MI_PATTERN"));

// ---------------------------------------------------------------------------
// NormalizerConfig
// ---------------------------------------------------------------------------

/// Flags that control which normalization steps run.
#[derive(Debug, Clone)]
pub struct NormalizerConfig {
    /// Fix spacing around punctuation and ZWNJ.
    pub correct_spacing: bool,
    /// Remove Arabic diacritics (harakat).
    pub remove_diacritics: bool,
    /// Remove non-Persian special characters.
    pub remove_specials_chars: bool,
    /// Collapse runs of repeated characters to at most two.
    pub decrease_repeated_chars: bool,
    /// Apply Persian-specific typographic rules.
    pub persian_style: bool,
    /// Transliterate Arabic-Indic digits to Persian.
    pub persian_numbers: bool,
    /// Replace common Unicode ligatures with canonical equivalents.
    pub unicodes_replacement: bool,
    /// Separate the verbal prefix می/نمی with a ZWNJ.
    pub seperate_mi: bool,
}

impl Default for NormalizerConfig {
    fn default() -> Self {
        Self {
            correct_spacing: true,
            remove_diacritics: true,
            remove_specials_chars: true,
            decrease_repeated_chars: true,
            persian_style: true,
            persian_numbers: true,
            unicodes_replacement: true,
            seperate_mi: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Normalizer
// ---------------------------------------------------------------------------

/// Normalizes Persian text: fixes spacing, removes diacritics, converts digits,
/// reduces repeated characters, and more.
///
/// All steps are individually toggleable via [`NormalizerConfig`].
///
/// # Examples
///
/// ```
/// use rustam::Normalizer;
///
/// let n = Normalizer::new();
/// assert_eq!(
///     n.normalize("اِعلاممممم کَرد : « زمین لرزه ای به بُزرگیِ 6 دهم ریشتر ...»"),
///     "اعلام کرد: «زمین‌لرزه‌ای به بزرگی ۶ دهم ریشتر …»"
/// );
/// ```
pub struct Normalizer {
    config: NormalizerConfig,
    /// Word lexicon used by `correct_spacing` and `decrease_repeated_chars`.
    words: Option<HashMap<String, crate::types::WordEntry>>,
    /// Known verb forms used by `seperate_mi`.
    verbs: Option<HashSet<String>>,
    /// Suffix set for token spacing.
    suffixes: HashSet<&'static str>,
}

impl Normalizer {
    /// Creates a normalizer with all steps enabled (default configuration).
    pub fn new() -> Self {
        Self::with_config(NormalizerConfig::default())
    }

    /// Creates a normalizer from the given configuration.
    pub fn with_config(config: NormalizerConfig) -> Self {
        let words = if config.correct_spacing || config.decrease_repeated_chars {
            let tok = WordTokenizer::with_config(WordTokenizerConfig {
                join_verb_parts: false,
                ..Default::default()
            });
            Some(tok.words)
        } else {
            None
        };

        let verbs = if config.seperate_mi {
            use crate::lemmatizer::Lemmatizer;
            let lem = Lemmatizer::with_joined_parts(false);
            Some(lem.verbs.keys().cloned().collect())
        } else {
            None
        };

        let suffixes = SUFFIXES.iter().copied().collect();

        Self {
            config,
            words,
            verbs,
            suffixes,
        }
    }

    // -----------------------------------------------------------------------
    // Full pipeline
    // -----------------------------------------------------------------------

    /// Runs all enabled normalization steps on `text`.
    pub fn normalize(&self, text: &str) -> String {
        let mut text = translate(text, &CHAR_TRANS);

        if self.config.persian_style {
            text = self.persian_style(&text);
        }
        if self.config.persian_numbers {
            text = self.persian_number(&text);
        }
        if self.config.remove_diacritics {
            text = self.remove_diacritics(&text);
        }
        if self.config.correct_spacing {
            text = self.correct_spacing(&text);
        }
        if self.config.unicodes_replacement {
            text = self.unicodes_replacement(&text);
        }
        if self.config.remove_specials_chars {
            text = self.remove_specials_chars(&text);
        }
        if self.config.decrease_repeated_chars {
            text = self.decrease_repeated_chars(&text);
        }
        if self.config.seperate_mi {
            text = self.seperate_mi(&text);
        }

        text
    }

    // -----------------------------------------------------------------------
    // Individual steps
    // -----------------------------------------------------------------------

    /// Fixes spacing around punctuation, suffixes, and prefixes.
    ///
    /// Tokenizes each line with a simple word tokenizer, calls
    /// [`token_spacing`](Self::token_spacing) to merge compound tokens, then
    /// applies affix and punctuation spacing patterns.
    pub fn correct_spacing(&self, text: &str) -> String {
        let text = apply_patterns(&EXTRA_SPACE_RE, text.to_string());

        let tok = WordTokenizer::with_config(WordTokenizerConfig {
            join_verb_parts: false,
            ..Default::default()
        });

        let lines: Vec<String> = text
            .split('\n')
            .map(|line| {
                if line.trim().is_empty() {
                    return line.to_string();
                }
                let tokens = tok.tokenize(line);
                let spaced = self.token_spacing(tokens);
                spaced.join(" ")
            })
            .collect();

        let text = lines.join("\n");
        let text = apply_patterns(&AFFIX_SPACING_RE, text);
        apply_patterns(&PUNCT_SPACING_RE, text)
    }

    /// Removes Arabic diacritical marks (tashkeel / harakat).
    pub fn remove_diacritics(&self, text: &str) -> String {
        DIACRITICS_RE.replace_all(text, "").into_owned()
    }

    /// Removes decorative/unusual Unicode code points.
    pub fn remove_specials_chars(&self, text: &str) -> String {
        SPECIAL_CHARS_RE.replace_all(text, "").into_owned()
    }

    /// Reduces runs of 3+ identical Persian letters to at most 2 (or 1 if the
    /// single-repeat form exists in the lexicon and the double-repeat does not).
    pub fn decrease_repeated_chars(&self, text: &str) -> String {
        let matches: Vec<_> = REPEATED_CHARS_RE
            .find_iter(text)
            .filter_map(|m| m.ok())
            .collect();

        if matches.is_empty() {
            return text.to_string();
        }

        let mut result = text.to_string();
        // Process matches in reverse so byte offsets stay valid.
        for m in matches.into_iter().rev() {
            let word = m.as_str().to_string();
            if let Some(words) = &self.words {
                if !words.contains_key(&word) {
                    let no_repeat = MORE_THAN_TWO_RE.replace_all(&word, "$1").into_owned();
                    let two_repeat = MORE_THAN_TWO_RE.replace_all(&word, "$1$1").into_owned();

                    let no_in = words.contains_key(&no_repeat);
                    let two_in = words.contains_key(&two_repeat);

                    let replacement = if no_in != two_in {
                        if no_in { no_repeat } else { two_repeat }
                    } else {
                        two_repeat
                    };

                    result.replace_range(m.start()..m.end(), &replacement);
                }
            }
        }

        result
    }

    /// Converts English/ASCII digits and Arabic-Indic digits to Persian digits.
    pub fn persian_number(&self, text: &str) -> String {
        translate(text, &NUMBER_TRANS)
    }

    /// Applies Persian typographic conventions (guillemets, Persian decimal
    /// point, ellipsis).
    pub fn persian_style(&self, text: &str) -> String {
        apply_patterns(&PERSIAN_STYLE_RE, text.to_string())
    }

    /// Replaces Unicode ligatures with their spelled-out Persian equivalents.
    pub fn unicodes_replacement(&self, text: &str) -> String {
        let mut text = text.to_string();
        for (old, new) in UNICODE_REPLACEMENTS {
            text = text.replace(old, new);
        }
        text
    }

    /// Adds a ZWNJ between the می/نمی prefix and the verb stem when the
    /// resulting form appears in the verb lexicon.
    pub fn seperate_mi(&self, text: &str) -> String {
        let verbs = match &self.verbs {
            Some(v) => v,
            None => return text.to_string(),
        };

        SEPERATE_MI_RE
            .replace_all(text, |caps: &fancy_regex::Captures| {
                let m = caps.get(0).map_or("", |m| m.as_str());
                let candidate = if let Some(stripped) = m.strip_prefix("نمی") {
                    format!("نمی\u{200C}{stripped}") // نمی is 3 chars = 6 bytes
                } else {
                    format!("می\u{200C}{}", &m[4..]) // می is 2 chars = 4 bytes
                };
                if verbs.contains(&candidate) {
                    candidate
                } else {
                    m.to_string()
                }
            })
            .into_owned()
    }

    // -----------------------------------------------------------------------
    // Token spacing (used inside correct_spacing)
    // -----------------------------------------------------------------------

    /// Merges token pairs that form a lexical compound via ZWNJ.
    ///
    /// For example, `["کتاب", "ها"]` → `["کتاب‌ها"]`.
    pub fn token_spacing(&self, tokens: Vec<String>) -> Vec<String> {
        let words = match &self.words {
            Some(w) => w,
            None => return tokens,
        };
        let verbs = self.verbs.as_ref();

        let mut result: Vec<String> = Vec::with_capacity(tokens.len());

        for (t, token) in tokens.iter().enumerate() {
            let mut joined = false;

            if let Some(last) = result.last() {
                let pair = format!("{last}\u{200C}{token}");

                let pair_is_known = verbs.is_some_and(|vs| vs.contains(&pair))
                    || words
                        .get(&pair)
                        .is_some_and(|e| e.frequency > 0);

                if pair_is_known {
                    joined = true;

                    // Don't join if the *next* token starts a known verb sequence.
                    if let Some(next) = tokens.get(t + 1) {
                        if verbs.is_some_and(|vs| vs.contains(&format!("{token}_{next}"))) {
                            joined = false;
                        }
                    }
                } else if self.suffixes.contains(token.as_str()) && words.contains_key(last.as_str()) {
                    joined = true;
                }

                if joined {
                    *result.last_mut().expect("non-empty") = pair;
                    continue;
                }
            }

            result.push(token.clone());
        }

        result
    }
}

impl Default for Normalizer {
    fn default() -> Self {
        Self::new()
    }
}
