use std::collections::{HashMap, HashSet};

use fancy_regex::Regex;
use once_cell::sync::Lazy;

use crate::{
    constants::{
        EMAIL_PATTERN, HASHTAG_PATTERN, ID_PATTERN, LINK_PATTERN, NUMBER_FLOAT_PATTERN,
        NUMBER_INT_PATTERN, WORD_SPLIT_PATTERN,
    },
    data::{parse_verbs, parse_words, VERBS_DAT, WORDS_DAT},
    types::WordEntry,
};

// ---------------------------------------------------------------------------
// Compiled regex patterns (shared across all tokenizer instances)
// ---------------------------------------------------------------------------

static SPLIT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(WORD_SPLIT_PATTERN).expect("WORD_SPLIT_PATTERN"));
static EMAIL_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(EMAIL_PATTERN).expect("EMAIL_PATTERN"));
static LINK_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(LINK_PATTERN).expect("LINK_PATTERN"));
static ID_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(ID_PATTERN).expect("ID_PATTERN"));
static HASHTAG_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(HASHTAG_PATTERN).expect("HASHTAG_PATTERN"));
static NUMBER_INT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(NUMBER_INT_PATTERN).expect("NUMBER_INT_PATTERN"));
static NUMBER_FLOAT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(NUMBER_FLOAT_PATTERN).expect("NUMBER_FLOAT_PATTERN"));

/// Modal verbs that appear *before* the main verb (future auxiliaries).
pub static BEFORE_VERBS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "خواهم", "خواهی", "خواهد", "خواهیم", "خواهید", "خواهند",
        "نخواهم", "نخواهی", "نخواهد", "نخواهیم", "نخواهید", "نخواهند",
    ]
    .into()
});

/// Auxiliary verbs / copulas that appear *after* the participle/root.
pub static AFTER_VERBS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "ام", "ای", "است", "ایم", "اید", "اند",
        "بودم", "بودی", "بود", "بودیم", "بودید", "بودند",
        "باشم", "باشی", "باشد", "باشیم", "باشید", "باشند",
        "شده_ام", "شده_ای", "شده_است", "شده_ایم", "شده_اید", "شده_اند",
        "شده_بودم", "شده_بودی", "شده_بود", "شده_بودیم", "شده_بودید", "شده_بودند",
        "شده_باشم", "شده_باشی", "شده_باشد", "شده_باشیم", "شده_باشید", "شده_باشند",
        "نشده_ام", "نشده_ای", "نشده_است", "نشده_ایم", "نشده_اید", "نشده_اند",
        "نشده_بودم", "نشده_بودی", "نشده_بود", "نشده_بودیم", "نشده_بودید", "نشده_بودند",
        "نشده_باشم", "نشده_باشی", "نشده_باشد", "نشده_باشیم", "نشده_باشید", "نشده_باشند",
        "شوم", "شوی", "شود", "شویم", "شوید", "شوند",
        "شدم", "شدی", "شد", "شدیم", "شدید", "شدند",
        "نشوم", "نشوی", "نشود", "نشویم", "نشوید", "نشوند",
        "نشدم", "نشدی", "نشد", "نشدیم", "نشدید", "نشدند",
        "می‌شوم", "می‌شوی", "می‌شود", "می‌شویم", "می‌شوید", "می‌شوند",
        "می‌شدم", "می‌شدی", "می‌شد", "می‌شدیم", "می‌شدید", "می‌شدند",
        "نمی‌شوم", "نمی‌شوی", "نمی‌شود", "نمی‌شویم", "نمی‌شوید", "نمی‌شوند",
        "نمی‌شدم", "نمی‌شدی", "نمی‌شد", "نمی‌شدیم", "نمی‌شدید", "نمی‌شدند",
        "خواهم_شد", "خواهی_شد", "خواهد_شد", "خواهیم_شد", "خواهید_شد", "خواهند_شد",
        "نخواهم_شد", "نخواهی_شد", "نخواهد_شد", "نخواهیم_شد", "نخواهید_شد", "نخواهند_شد",
    ]
    .into()
});

// ---------------------------------------------------------------------------
// WordTokenizer configuration
// ---------------------------------------------------------------------------

/// Configuration for the word tokenizer.
#[derive(Debug, Clone)]
pub struct WordTokenizerConfig {
    /// Join multi-part verbs with underscores (e.g. `رفته_است`).
    pub join_verb_parts: bool,
    /// Replace `@mention` tokens with `ID`.
    pub replace_ids: bool,
    /// Replace URL tokens with `LINK`.
    pub replace_links: bool,
    /// Replace e-mail tokens with `EMAIL`.
    pub replace_emails: bool,
    /// Replace numbers: floats → `NUMF`, integers → `NUM<digit-count>`.
    pub replace_numbers: bool,
    /// Replace `#hashtag` tokens with `TAG <text>`.
    pub replace_hashtags: bool,
}

impl Default for WordTokenizerConfig {
    fn default() -> Self {
        Self {
            join_verb_parts: true,
            replace_ids: false,
            replace_links: false,
            replace_emails: false,
            replace_numbers: false,
            replace_hashtags: false,
        }
    }
}

// ---------------------------------------------------------------------------
// WordTokenizer
// ---------------------------------------------------------------------------

/// Tokenizes Persian (and mixed) text into words.
///
/// Multi-part verbs such as `رفته شده است` are joined as `رفته_شده_است` by
/// default.  Optional replacements for links, IDs, numbers, etc. can be
/// enabled through [`WordTokenizerConfig`].
///
/// # Examples
///
/// ```
/// use rustam::WordTokenizer;
///
/// let tok = WordTokenizer::new();
/// let tokens = tok.tokenize("این جمله (خیلی) پیچیده نیست!!!");
/// assert_eq!(tokens, vec!["این", "جمله", "(", "خیلی", ")", "پیچیده", "نیست", "!!!"]);
/// ```
pub struct WordTokenizer {
    config: WordTokenizerConfig,
    /// Lexicon: word → (frequency, pos_tags).
    pub words: HashMap<String, WordEntry>,
    #[allow(dead_code)]
    verbs: Vec<String>,
    /// Past participle forms that can appear before an after-verb.
    verbe: HashSet<String>,
}

impl WordTokenizer {
    /// Creates a tokenizer using the embedded default lexicon and verb list.
    pub fn new() -> Self {
        Self::with_config(WordTokenizerConfig::default())
    }

    /// Creates a tokenizer with custom configuration.
    pub fn with_config(config: WordTokenizerConfig) -> Self {
        let words = parse_words(WORDS_DAT);

        let verb_roots = parse_verbs(VERBS_DAT);
        let mut verbs: Vec<String> = verb_roots
            .iter()
            .map(|v| format!("{}#{}", v.past, v.present))
            .collect();
        // Reverse so longer/more-specific verbs are checked first when joining.
        verbs.reverse();

        let mut verbe = HashSet::new();
        for v in &verb_roots {
            verbe.insert(format!("{}ه", v.past));
            verbe.insert(format!("ن{}ه", v.past));
        }

        Self {
            config,
            words,
            verbs,
            verbe,
        }
    }

    /// Creates a tokenizer with verb-joining disabled.
    pub fn without_verb_joining() -> Self {
        Self::with_config(WordTokenizerConfig {
            join_verb_parts: false,
            ..Default::default()
        })
    }

    // -----------------------------------------------------------------------
    // Public API
    // -----------------------------------------------------------------------

    /// Tokenizes `text` into a `Vec` of word strings.
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let mut text = text.replace(['\n', '\t'], " ");

        if self.config.replace_emails {
            text = EMAIL_RE.replace_all(&text, " EMAIL ").into_owned();
        }
        if self.config.replace_links {
            text = LINK_RE.replace_all(&text, " LINK ").into_owned();
        }
        if self.config.replace_ids {
            text = ID_RE.replace_all(&text, " ID ").into_owned();
        }
        if self.config.replace_hashtags {
            text = HASHTAG_RE
                .replace_all(&text, |caps: &fancy_regex::Captures| {
                    let tag = caps.get(1).map_or("", |m| m.as_str());
                    format!("TAG {}", tag.replace('_', " "))
                })
                .into_owned();
        }
        if self.config.replace_numbers {
            text = NUMBER_FLOAT_RE.replace_all(&text, " NUMF ").into_owned();
            text = NUMBER_INT_RE
                .replace_all(&text, |caps: &fancy_regex::Captures| {
                    let n = caps.get(1).map_or("", |m| m.as_str());
                    format!(" NUM{} ", n.chars().count())
                })
                .into_owned();
        }

        // Insert spaces around punctuation/brackets so we can split on spaces.
        let text = SPLIT_RE.replace_all(&text, " $1 ").into_owned();
        let mut tokens: Vec<String> = text
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();

        if self.config.join_verb_parts {
            tokens = self.join_verb_parts(tokens);
        }

        tokens
    }

    /// Joins consecutive tokens that form a multi-part Persian verb.
    ///
    /// Iterates over tokens right-to-left so that multi-part sequences like
    /// `گفته شده است` are collapsed to `گفته_شده_است`.
    pub fn join_verb_parts(&self, tokens: Vec<String>) -> Vec<String> {
        if tokens.len() <= 1 {
            return tokens;
        }

        // We build the result list in reverse then flip it.
        let mut result: Vec<String> = vec!["".to_string()];

        for token in tokens.into_iter().rev() {
            let last = result.last().expect("non-empty");
            if BEFORE_VERBS.contains(token.as_str())
                || (AFTER_VERBS.contains(last.as_str()) && self.verbe.contains(token.as_str()))
            {
                let joined = format!("{token}_{last}");
                *result.last_mut().expect("non-empty") = joined;
            } else {
                result.push(token);
            }
        }

        // Sentinel "" was pushed first; after reversing it ends up last.
        result.reverse();
        result.pop();
        result
    }
}

impl Default for WordTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function: tokenizes `text` into words using default settings.
pub fn word_tokenize(text: &str) -> Vec<String> {
    WordTokenizer::new().tokenize(text)
}
