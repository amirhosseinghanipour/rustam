//! Informal-to-formal Persian normalization.
//!
//! [`InformalNormalizer`] converts colloquial Persian text (e.g. chat messages)
//! to standard written Persian.  It maps informal word forms and verb
//! conjugations to their formal equivalents, returning all valid candidates
//! for each token so the caller can choose the best one.
//!
//! [`InformalLemmatizer`] extends [`Lemmatizer`] with an informal-verb
//! conjugation table and an informal-word map.

use std::collections::{HashMap, HashSet};

use crate::{
    conjugation::Conjugation,
    data::{parse_verbs, INFORMAL_VERBS_DAT, INFORMAL_WORDS_DAT, VERBS_DAT},
    lemmatizer::Lemmatizer,
    normalizer::Normalizer,
    sentence_tokenizer::SentenceTokenizer,
    types::WordEntry,
    word_tokenizer::WordTokenizer,
};

// ---------------------------------------------------------------------------
// Data parsing helpers
// ---------------------------------------------------------------------------

/// Parses `iwords.dat`: each line is `informal_word formal_word`.
fn parse_iwords(source: &str) -> HashMap<String, String> {
    source
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() { return None; }
            let mut parts = line.splitn(2, ' ');
            let informal = parts.next()?.to_string();
            let formal = parts.next()?.to_string();
            Some((informal, formal))
        })
        .collect()
}

/// Parses `iverbs.dat`: each line is `formal_past#formal_present informal_present flag`.
fn parse_iverbs(source: &str) -> Vec<InformalVerbEntry> {
    source
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() { return None; }
            let mut parts = line.splitn(3, ' ');
            let formal = parts.next()?.to_string();
            let informal_present = parts.next()?.to_string();
            let flag: bool = parts.next()?.trim() == "1";
            let mut roots = formal.splitn(2, '#');
            let past = roots.next()?.to_string();
            let present = roots.next()?.to_string();
            Some(InformalVerbEntry { formal, past, present: present.clone(), informal_present, flag })
        })
        .collect()
}

struct InformalVerbEntry {
    formal: String,    // "past#present"
    past: String,
    present: String,
    informal_present: String,
    flag: bool,        // true → bā-prefix verbs (برمی...)
}

// ---------------------------------------------------------------------------
// InformalLemmatizer
// ---------------------------------------------------------------------------

/// Extends [`Lemmatizer`] to handle informal (colloquial) Persian verb and word forms.
///
/// Adds all informal conjugation forms to the verb lookup table, and all
/// informal word surface forms to the word set.
pub struct InformalLemmatizer {
    inner: Lemmatizer,
    /// extra informal word set (surface forms only, no lemma — return as-is)
    extra_words: HashSet<String>,
}

impl InformalLemmatizer {
    /// Creates a new `InformalLemmatizer` from the embedded data files.
    pub fn new() -> Self {
        let mut inner = Lemmatizer::new();

        let iverbs = parse_iverbs(INFORMAL_VERBS_DAT);
        let iwords = parse_iwords(INFORMAL_WORDS_DAT);

        // Add informal word surfaces to words map (pointing to formal form).
        // Look up the formal entry first to avoid a double-borrow.
        let informal_inserts: Vec<(String, WordEntry)> = iwords
            .iter()
            .map(|(informal, formal)| {
                let entry = inner.words.get(formal.as_str()).cloned().unwrap_or(WordEntry {
                    frequency: 0,
                    pos_tags: vec![],
                });
                (informal.clone(), entry)
            })
            .collect();
        for (informal, entry) in informal_inserts {
            inner.words.entry(informal).or_insert(entry);
        }

        // For every ند-ending verb form, add a نه variant (informal spoken drop of د)
        let extra_verbs: Vec<(String, String)> = inner
            .verbs
            .iter()
            .filter(|(k, _)| k.ends_with('د'))
            .map(|(k, v)| {
                let n_form = format!("{}ن", &k[..k.len() - 'د'.len_utf8()]);
                (n_form, v.clone())
            })
            .collect();
        for (k, v) in extra_verbs {
            inner.verbs.entry(k).or_insert(v);
        }

        // Add informal conjugation forms
        for entry in &iverbs {
            for form in informal_conjugations(&entry.informal_present) {
                inner.verbs.entry(form).or_insert_with(|| entry.formal.clone());
            }
        }

        // Collect informal word surface set (for contains-check in all_in_lexicon)
        let mut extra_words: HashSet<String> = iwords.into_keys().collect();
        // Also add ً-stripped variants
        let stripped: Vec<String> = inner
            .words
            .keys()
            .filter(|w| w.ends_with('ً'))
            .map(|w| w[..w.len() - 'ً'.len_utf8()].to_string())
            .collect();
        extra_words.extend(stripped);

        Self { inner, extra_words }
    }

    /// Returns the lemma of `word` (informal or formal).
    pub fn lemmatize(&self, word: &str, pos: &str) -> String {
        self.inner.lemmatize(word, pos)
    }

    /// Returns `true` if `word` is in the lexicon (formal or informal surface).
    pub fn contains(&self, word: &str) -> bool {
        self.inner.words.contains_key(word)
            || self.inner.verbs.contains_key(word)
            || self.extra_words.contains(word)
    }
}

impl Default for InformalLemmatizer {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// InformalNormalizer
// ---------------------------------------------------------------------------

/// Normalizes informal (colloquial) Persian text to standard written Persian.
///
/// The output is a `Vec<Vec<Vec<String>>>` — sentences of tokens of candidates —
/// because each informal word may have multiple valid formal readings.
///
/// # Examples
///
/// ```
/// use rustam::InformalNormalizer;
///
/// let n = InformalNormalizer::new();
/// let result = n.normalize("بابا یه شغل مناسب واسه بچه هام پیدا کرده");
/// // Each inner Vec<String> is the list of formal candidates for that token.
/// ```
pub struct InformalNormalizer {
    normalizer: Normalizer,
    sent_tokenizer: SentenceTokenizer,
    word_tokenizer: WordTokenizer,
    lemmatizer: Lemmatizer,
    ilemmatizer: InformalLemmatizer,
    iword_map: HashMap<String, String>,
    /// Maps informal past stem → canonical formal past stem
    past_verbs: HashMap<String, String>,
    /// Maps informal present stem → canonical formal present stem
    present_verbs: HashMap<String, String>,
    /// Maps every informal conjugated form → the matching formal conjugated form
    iverb_map: HashMap<String, String>,
}

impl InformalNormalizer {
    /// Creates an `InformalNormalizer` backed by the embedded data files.
    pub fn new() -> Self {
        let iverb_entries = parse_iverbs(INFORMAL_VERBS_DAT);
        let iword_map = parse_iwords(INFORMAL_WORDS_DAT);
        let default_verbs = parse_verbs(VERBS_DAT);

        let mut past_verbs: HashMap<String, String> = HashMap::new();
        let mut present_verbs: HashMap<String, String> = HashMap::new();
        let mut iverb_map: HashMap<String, String> = HashMap::new();

        // Populate from iverbs.dat
        for entry in &iverb_entries {
            present_verbs.insert(entry.informal_present.clone(), entry.present.clone());
            past_verbs.insert(entry.past.clone(), entry.past.clone());
        }
        // Populate from default verbs (formal present/past map to themselves)
        for vr in &default_verbs {
            present_verbs.insert(vr.present.clone(), vr.present.clone());
            past_verbs.insert(vr.past.clone(), vr.past.clone());
        }

        // Build informal conjugation → formal conjugation map
        let conj = Conjugation;
        for entry in &iverb_entries {
            let informal_forms = informal_conjugations(&entry.informal_present);
            let formal_root = format!("{}#{}", entry.past, entry.present);
            let formal_forms = conj.get_all(&formal_root);
            // flag == true → use forms starting at index 48 (imperfective)
            let (informal_range, formal_range) = if entry.flag {
                (0..informal_forms.len(), 48..formal_forms.len())
            } else {
                (8..informal_forms.len(), 56..formal_forms.len())
            };
            for (i, j) in informal_range.zip(formal_range) {
                if i < informal_forms.len() && j < formal_forms.len() {
                    let inf_form = &informal_forms[i];
                    let form_form = &formal_forms[j];
                    iverb_map.insert(inf_form.clone(), form_form.clone());
                    // Also insert ZWNJ-less and space variants
                    if inf_form.contains('\u{200C}') {
                        iverb_map.insert(
                            inf_form.replace('\u{200C}', ""),
                            form_form.clone(),
                        );
                        iverb_map.insert(
                            inf_form.replace('\u{200C}', " "),
                            form_form.clone(),
                        );
                    }
                    if inf_form.ends_with("ین") {
                        let alt = format!("{}د", &inf_form[..inf_form.len() - "ین".len()]);
                        iverb_map.insert(alt, form_form.clone());
                    }
                }
            }
        }

        Self {
            normalizer: Normalizer::new(),
            sent_tokenizer: SentenceTokenizer::new(),
            word_tokenizer: WordTokenizer::new(),
            lemmatizer: Lemmatizer::new(),
            ilemmatizer: InformalLemmatizer::new(),
            iword_map,
            past_verbs,
            present_verbs,
            iverb_map,
        }
    }

    // -----------------------------------------------------------------------
    // Accessors
    // -----------------------------------------------------------------------

    /// Returns a reference to the embedded `InformalLemmatizer`.
    pub fn lemmatizer(&self) -> &InformalLemmatizer {
        &self.ilemmatizer
    }

    /// Returns the map of informal past stem → canonical formal past stem.
    pub fn past_verb_map(&self) -> &HashMap<String, String> {
        &self.past_verbs
    }

    /// Returns the map of informal present stem → canonical formal present stem.
    pub fn present_verb_map(&self) -> &HashMap<String, String> {
        &self.present_verbs
    }

    // -----------------------------------------------------------------------
    // Public API
    // -----------------------------------------------------------------------

    /// Normalizes `text`, returning all formal-candidate lists per token.
    ///
    /// Structure: `result[sentence][token]` = `Vec<formal_candidates>`.
    pub fn normalize(&self, text: &str) -> Vec<Vec<Vec<String>>> {
        let text = self.normalizer.normalize(text);
        self.sent_tokenizer
            .tokenize(&text)
            .into_iter()
            .map(|sentence| {
                self.word_tokenizer
                    .tokenize(&sentence)
                    .into_iter()
                    .map(|word| self.normalized_word(&word))
                    .collect()
            })
            .collect()
    }

    /// Returns all formal candidates for a single informal `word`.
    pub fn normalized_word(&self, word: &str) -> Vec<String> {
        // Direct verb form lookup
        if let Some(formal) = self.iverb_map.get(word) {
            return vec![formal.clone()];
        }
        // Direct word map lookup
        if let Some(formal) = self.iword_map.get(word) {
            return vec![formal.clone(), word.to_string()];
        }
        // The word is already formal
        if self.lemmatizer.words.contains_key(word) {
            return vec![word.to_string()];
        }
        // Try suffix analysis
        let analyzed = self.analyze_word(word);
        if !analyzed.is_empty() {
            return analyzed;
        }
        vec![word.to_string()]
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn analyze_word(&self, word: &str) -> Vec<String> {
        const SUFFIXES: &[&str] = &[
            "هاست", "هایی", "هایم", "ترین", "ایی", "انی", "شان", "شون", "است",
            "تان", "تون", "مان", "مون", "هام", "هاش", "های", "طور", "ها", "تر",
            "ئی", "یی", "یم", "ام", "ای", "ان", "هم", "رو", "یت", "ه", "ی",
            "ش", "و", "ا", "ت", "م",
        ];

        let mut results: Vec<String> = Vec::new();

        for suffix in SUFFIXES {
            if word.ends_with(suffix) && word.len() > suffix.len() {
                let stem = &word[..word.len() - suffix.len()];
                let formal_stem = self
                    .iword_map
                    .get(stem)
                    .map(|s| s.as_str())
                    .or_else(|| self.lemmatizer.words.contains_key(stem).then_some(stem));

                if let Some(fs) = formal_stem {
                    let normalized = apply_suffix(fs, suffix);
                    for form in normalized {
                        if !results.contains(&form) {
                            results.push(form);
                        }
                    }
                }
            }
        }

        results
    }
}

impl Default for InformalNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Informal conjugation generator
// ---------------------------------------------------------------------------

/// Generates the informal spoken conjugation forms for `verb` (present stem).
///
/// Produces 36 forms (6 × present_simple + negated + imperfective +
/// negated-imperfective + subjunctive + negated-subjunctive).
pub fn informal_conjugations(verb: &str) -> Vec<String> {
    let ends = ["م", "ی", "", "یم", "ین", "ن"];
    let mut present_simples: Vec<String> = ends.iter().map(|e| format!("{verb}{e}")).collect();
    // 3rd person singular: +ه unless verb ends in ا, then +د
    if verb.ends_with('ا') {
        present_simples[2] = format!("{verb}د");
    } else {
        present_simples[2] = format!("{verb}ه");
    }

    let negated: Vec<String> = present_simples.iter().map(|f| format!("ن{f}")).collect();
    let imperfective: Vec<String> =
        present_simples.iter().map(|f| format!("می\u{200C}{f}")).collect();
    let neg_imperfective: Vec<String> =
        present_simples.iter().map(|f| format!("نمی\u{200C}{f}")).collect();
    let subjunctive: Vec<String> = present_simples
        .iter()
        .map(|f| {
            if f.starts_with('ب') {
                f.clone()
            } else {
                format!("ب{f}")
            }
        })
        .collect();
    let neg_subjunctive: Vec<String> = present_simples.iter().map(|f| format!("ن{f}")).collect();

    [
        present_simples,
        negated,
        imperfective,
        neg_imperfective,
        subjunctive,
        neg_subjunctive,
    ]
    .concat()
}

// ---------------------------------------------------------------------------
// Suffix attachment helper
// ---------------------------------------------------------------------------

const ADHESIVE: &str = "بپتثجچحخسشصضعغفقکگلمنهی";

fn is_adhesive(ch: char) -> bool {
    ADHESIVE.contains(ch)
}

/// Attaches a morphological suffix to a stem, inserting ZWNJ where needed.
fn apply_suffix(stem: &str, suffix: &str) -> Vec<String> {
    let last = stem.chars().last().unwrap_or(' ');
    match suffix {
        "شون" => vec![format!("{stem}شان")],
        "تون" => vec![format!("{stem}تان")],
        "مون" => vec![format!("{stem}مان")],
        "هام" => {
            let sep = if is_adhesive(last) { "\u{200C}" } else { "" };
            vec![format!("{stem}{sep}هایم")]
        }
        "ها" | "ا" => {
            let sep = if is_adhesive(last) { "\u{200C}" } else { "" };
            vec![format!("{stem}{sep}ها")]
        }
        "رو" => vec![format!("{stem} را")],
        "و" => vec![format!("{stem} را"), format!("{stem} و")],
        "ه" => vec![
            format!("{stem}ه"),
            format!("{stem}ه است"),
            format!("{stem} است"),
        ],
        _ => vec![format!("{stem}{suffix}")],
    }
}
