use std::collections::HashMap;

use crate::{
    conjugation::Conjugation,
    data::{parse_verbs, VERBS_DAT},
    stemmer::Stemmer,
    types::WordEntry,
    word_tokenizer::{WordTokenizer, WordTokenizerConfig, AFTER_VERBS, BEFORE_VERBS},
};

// ---------------------------------------------------------------------------
// Lemmatizer
// ---------------------------------------------------------------------------

/// Dictionary-based Persian lemmatizer.
///
/// `Lemmatizer` uses a full verb-conjugation table to map any conjugated form
/// back to its `past#present` lemma string, and the word lexicon to look up
/// nominal lemmas.
///
/// # Examples
///
/// ```
/// use rustam::Lemmatizer;
///
/// let lem = Lemmatizer::new();
/// assert_eq!(lem.lemmatize("کتاب‌ها", ""), "کتاب");
/// assert_eq!(lem.lemmatize("می‌روم",  ""), "رفت#رو");
/// assert_eq!(lem.lemmatize("گفته_شده_است", ""), "گفت#گو");
/// ```
pub struct Lemmatizer {
    /// The full word lexicon.
    pub words: HashMap<String, WordEntry>,
    /// Maps every conjugated verb form → `"past#present"` lemma string.
    pub verbs: HashMap<String, String>,
    stemmer: Stemmer,
}

impl Lemmatizer {
    /// Builds a lemmatizer using embedded default data.
    pub fn new() -> Self {
        Self::with_joined_parts(true)
    }

    /// Builds a lemmatizer, optionally adding joined multi-part verb forms.
    ///
    /// When `joined_verb_parts` is `true` the lemmatizer also recognises
    /// underscore-joined forms like `رفته_است`.
    pub fn with_joined_parts(joined_verb_parts: bool) -> Self {
        let tokenizer = WordTokenizer::with_config(WordTokenizerConfig {
            join_verb_parts: true,
            ..Default::default()
        });

        let words = tokenizer.words.clone();
        let conj = Conjugation;
        let verb_roots = parse_verbs(VERBS_DAT);

        // Build a flat list of `past#present` strings (same format as verbs.dat).
        let verb_entries: Vec<String> = verb_roots
            .iter()
            .map(|v| format!("{}#{}", v.past, v.present))
            .collect();

        let mut verbs: HashMap<String, String> = HashMap::new();

        // Special-case: است (copula) has no root pair in the verb file.
        verbs.insert("است".to_string(), "#است".to_string());

        // Iterate in reverse so earlier entries in verbs.dat overwrite later ones
        // for the same past root — earlier entries in verbs.dat take precedence,
        // so the verb list is iterated in reverse when building the conjugation table.
        for verb_entry in verb_entries.iter().rev() {
            for tense in conj.get_all(verb_entry) {
                verbs.insert(tense, verb_entry.clone());
            }
        }

        if joined_verb_parts {
            for verb_entry in verb_entries.iter().rev() {
                let bon = verb_entry.split('#').next().unwrap_or("");
                for av in AFTER_VERBS.iter() {
                    verbs.insert(format!("{bon}ه_{av}"), verb_entry.clone());
                    verbs.insert(format!("ن{bon}ه_{av}"), verb_entry.clone());
                }
                for bv in BEFORE_VERBS.iter() {
                    verbs.insert(format!("{bv}_{bon}"), verb_entry.clone());
                }
            }
        }

        Self {
            words,
            verbs,
            stemmer: Stemmer::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Public API
    // -----------------------------------------------------------------------

    /// Returns the lemma of `word`.
    ///
    /// `pos` is an optional part-of-speech hint (e.g. `"NOUN"`, `"VERB"`,
    /// `"ADJ"`, `"PRON"`).  Pass `""` when the POS is unknown.
    pub fn lemmatize(&self, word: &str, pos: &str) -> String {
        // 1. No POS hint and word is in lexicon → return as-is.
        if pos.is_empty() && self.words.contains_key(word) {
            return word.to_string();
        }

        // 2. Verb lookup (when POS is empty or VERB).
        if (pos.is_empty() || pos == "VERB") && self.verbs.contains_key(word) {
            return self.verbs[word].clone();
        }

        // 3. Adjective ending in ی → return as-is.
        if pos.starts_with("ADJ") && word.ends_with('ی') {
            return word.to_string();
        }

        // 4. Pronouns → return as-is.
        if pos == "PRON" {
            return word.to_string();
        }

        // 5. Word found in lexicon.
        if self.words.contains_key(word) {
            return word.to_string();
        }

        // 6. Try the stem and look it up in the lexicon.
        let stem = self.stemmer.stem(word);
        if !stem.is_empty() && self.words.contains_key(&stem) {
            return stem;
        }

        // 7. Return original word unchanged.
        word.to_string()
    }
}

impl Default for Lemmatizer {
    fn default() -> Self {
        Self::new()
    }
}
