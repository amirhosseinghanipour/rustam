#![allow(missing_docs)]
//! Reader for the Quran corpus with Buckwalter transliteration.

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::PathBuf,
};

fn buckwalter_to_arabic(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\'' => 'ء',
            '|' => 'آ',
            '>' => 'أ',
            '&' => 'ؤ',
            '<' => 'إ',
            '}' => 'ئ',
            'A' => 'ا',
            'b' => 'ب',
            'p' => 'ة',
            't' => 'ت',
            'v' => 'ث',
            'j' => 'ج',
            'H' => 'ح',
            'x' => 'خ',
            'd' => 'د',
            '*' => 'ذ',
            'r' => 'ر',
            'z' => 'ز',
            's' => 'س',
            '$' => 'ش',
            'S' => 'ص',
            'D' => 'ض',
            'T' => 'ط',
            'Z' => 'ظ',
            'E' => 'ع',
            'g' => 'غ',
            '_' => 'ـ',
            'f' => 'ف',
            'q' => 'ق',
            'k' => 'ك',
            'l' => 'ل',
            'm' => 'م',
            'n' => 'ن',
            'h' => 'ه',
            'w' => 'و',
            'Y' => 'ى',
            'y' => 'ي',
            '~' => 'ّ',
            'o' => 'ْ',
            'a' => 'َ',
            'i' => 'ِ',
            'u' => 'ُ',
            'F' => 'ً',
            'K' => 'ٍ',
            'N' => 'ٌ',
            '`' => 'ٰ',
            '{' => 'ٱ',
            'P' => 'پ',
            'J' => 'چ',
            'V' => 'ژ',
            'G' => 'گ',
            other => other,
        })
        .collect()
}

fn parse_location(loc_str: &str) -> (u32, u32, u32, u32) {
    // Strip parens: "(1:2:3:4)" → "1:2:3:4"
    let inner = loc_str.trim_start_matches('(').trim_end_matches(')');
    let parts: Vec<&str> = inner.split(':').collect();
    let get = |i: usize| parts.get(i).and_then(|s| s.parse().ok()).unwrap_or(0);
    (get(0), get(1), get(2), get(3))
}

/// A morphological part from the Quran corpus.
pub struct QuranPart {
    pub loc: (u32, u32, u32, u32),
    pub text: String,
    pub tag: String,
    pub lem: Option<String>,
    pub root: Option<String>,
}

/// Reads the Quran corpus with Buckwalter transliteration.
pub struct QuranReader {
    path: PathBuf,
}

impl QuranReader {
    /// Opens the Quran corpus file.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Returns an iterator over all morphological parts.
    pub fn parts(&self) -> Box<dyn Iterator<Item = QuranPart> + '_> {
        let file = match File::open(&self.path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("QuranReader: cannot open {:?}: {}", self.path, e);
                return Box::new(iter::empty());
            }
        };

        let mut parts: Vec<QuranPart> = Vec::new();

        for line in BufReader::new(file).lines().filter_map(|l| l.ok()) {
            if !line.starts_with('(') {
                continue;
            }
            let cols: Vec<&str> = line.splitn(4, '\t').collect();
            if cols.len() < 4 {
                continue;
            }
            let loc_str = cols[0];
            let word_bw = cols[1];
            let tag = cols[2].to_string();
            let features_str = cols[3];

            let loc = parse_location(loc_str);
            let text = buckwalter_to_arabic(word_bw);

            let mut lem: Option<String> = None;
            let mut root: Option<String> = None;

            for feat in features_str.split('|') {
                if let Some(val) = feat.strip_prefix("LEM:") {
                    lem = Some(buckwalter_to_arabic(val.trim()));
                } else if let Some(val) = feat.strip_prefix("ROOT:") {
                    root = Some(buckwalter_to_arabic(val.trim()));
                }
            }

            parts.push(QuranPart { loc, text, tag, lem, root });
        }

        Box::new(parts.into_iter())
    }

    /// Returns an iterator over words, grouped by (chapter, verse, word_index).
    /// Yields (word_text, tag, lem, root, location_str, parts_vec).
    pub fn words(
        &self,
    ) -> Box<dyn Iterator<Item = (String, String, Option<String>, Option<String>, String, Vec<QuranPart>)> + '_>
    {
        let file = match File::open(&self.path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("QuranReader: cannot open {:?}: {}", self.path, e);
                return Box::new(iter::empty());
            }
        };

        let mut all_parts: Vec<QuranPart> = Vec::new();

        for line in BufReader::new(file).lines().filter_map(|l| l.ok()) {
            if !line.starts_with('(') {
                continue;
            }
            let cols: Vec<&str> = line.splitn(4, '\t').collect();
            if cols.len() < 4 {
                continue;
            }
            let loc_str = cols[0];
            let word_bw = cols[1];
            let tag = cols[2].to_string();
            let features_str = cols[3];

            let loc = parse_location(loc_str);
            let text = buckwalter_to_arabic(word_bw);

            let mut lem: Option<String> = None;
            let mut root: Option<String> = None;

            for feat in features_str.split('|') {
                if let Some(val) = feat.strip_prefix("LEM:") {
                    lem = Some(buckwalter_to_arabic(val.trim()));
                } else if let Some(val) = feat.strip_prefix("ROOT:") {
                    root = Some(buckwalter_to_arabic(val.trim()));
                }
            }

            all_parts.push(QuranPart { loc, text, tag, lem, root });
        }

        // Group by (ch, verse, word_idx) = (loc.0, loc.1, loc.2)
        let mut grouped: Vec<(String, String, Option<String>, Option<String>, String, Vec<QuranPart>)> =
            Vec::new();

        let mut i = 0;
        while i < all_parts.len() {
            let key = (all_parts[i].loc.0, all_parts[i].loc.1, all_parts[i].loc.2);
            let mut group: Vec<QuranPart> = Vec::new();
            while i < all_parts.len()
                && (all_parts[i].loc.0, all_parts[i].loc.1, all_parts[i].loc.2) == key
            {
                // We can't just move from the vec easily, so we reconstruct
                let p = &all_parts[i];
                group.push(QuranPart {
                    loc: p.loc,
                    text: p.text.clone(),
                    tag: p.tag.clone(),
                    lem: p.lem.clone(),
                    root: p.root.clone(),
                });
                i += 1;
            }
            let word_text: String = group.iter().map(|p| p.text.clone()).collect();
            let first_tag = group.first().map(|p| p.tag.clone()).unwrap_or_default();
            let first_lem = group.first().and_then(|p| p.lem.clone());
            let first_root = group.first().and_then(|p| p.root.clone());
            let loc_str = format!("{}:{}", key.0, key.1);
            grouped.push((word_text, first_tag, first_lem, first_root, loc_str, group));
        }

        Box::new(grouped.into_iter())
    }
}
