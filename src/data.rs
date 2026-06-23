use std::collections::HashMap;

use crate::types::{VerbRoots, WordEntry};

// ---------------------------------------------------------------------------
// Embedded data files compiled into the binary at build time
// ---------------------------------------------------------------------------

/// Embedded word lexicon (`words.dat`).
pub const WORDS_DAT: &str = include_str!("../data/words.dat");
/// Embedded verb root list (`verbs.dat`).
pub const VERBS_DAT: &str = include_str!("../data/verbs.dat");
/// Embedded stopword list (`stopwords.dat`).
pub const STOPWORDS_DAT: &str = include_str!("../data/stopwords.dat");
/// Embedded abbreviation list (`abbreviations.dat`).
pub const ABBREVIATIONS_DAT: &str = include_str!("../data/abbreviations.dat");
/// Embedded informal word map (`iwords.dat`).
pub const INFORMAL_WORDS_DAT: &str = include_str!("../data/iwords.dat");
/// Embedded informal verb list (`iverbs.dat`).
pub const INFORMAL_VERBS_DAT: &str = include_str!("../data/iverbs.dat");

// ---------------------------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------------------------

/// Parses the lexicon file into a HashMap of word → WordEntry.
///
/// Each line has the format: `word\tfrequency\tPOS1,POS2,...`
pub fn parse_words(source: &str) -> HashMap<String, WordEntry> {
    let mut map = HashMap::new();
    for line in source.lines() {
        let parts: Vec<&str> = line.trim().splitn(3, '\t').collect();
        if parts.len() != 3 {
            continue;
        }
        let word = parts[0].to_string();
        let Ok(frequency) = parts[1].parse::<u64>() else {
            continue;
        };
        let pos_tags = parts[2].split(',').map(str::to_string).collect();
        map.insert(word, WordEntry { frequency, pos_tags });
    }
    map
}

/// Parses the verb file into a Vec of VerbRoots.
///
/// Each non-empty line has the format `past#present`.
pub fn parse_verbs(source: &str) -> Vec<VerbRoots> {
    source
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|line| {
            let line = line.trim();
            let mut parts = line.splitn(2, '#');
            let past = parts.next()?.to_string();
            let present = parts.next()?.to_string();
            Some(VerbRoots { past, present })
        })
        .collect()
}

/// Returns the list of stopwords as owned strings.
pub fn parse_stopwords(source: &str) -> Vec<String> {
    let mut words: Vec<String> = source
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    words.sort();
    words
}

/// Returns the list of abbreviations as owned strings.
pub fn parse_abbreviations(source: &str) -> Vec<String> {
    source
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

// ---------------------------------------------------------------------------
// Convenience accessors over embedded data
// ---------------------------------------------------------------------------

/// Returns the full word lexicon parsed from the embedded `words.dat`.
pub fn words_list() -> std::collections::HashMap<String, crate::types::WordEntry> {
    parse_words(WORDS_DAT)
}

/// Returns all verb root pairs from the embedded `verbs.dat`.
pub fn verbs_list() -> Vec<crate::types::VerbRoots> {
    parse_verbs(VERBS_DAT)
}

/// Returns the stopword list from the embedded `stopwords.dat`.
pub fn stopwords_list() -> Vec<String> {
    parse_stopwords(STOPWORDS_DAT)
}

/// Returns all past roots from the embedded `verbs.dat`.
pub fn past_roots() -> Vec<String> {
    parse_verbs(VERBS_DAT).into_iter().map(|v| v.past).collect()
}

/// Returns all present roots from the embedded `verbs.dat`.
pub fn present_roots() -> Vec<String> {
    parse_verbs(VERBS_DAT).into_iter().map(|v| v.present).collect()
}
