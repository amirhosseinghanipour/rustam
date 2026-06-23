//! Demonstrates informal Persian normalization and lemmatization.
//!
//! Converts colloquial / chat-style Persian to standard written Persian,
//! returning multiple formal candidates per token.
//!
//! Run with:
//! ```
//! cargo run --example informal
//! ```

use rustam::{InformalLemmatizer, InformalNormalizer};

fn main() {
    // -----------------------------------------------------------------------
    // InformalNormalizer — word-level
    // -----------------------------------------------------------------------
    let n = InformalNormalizer::new();

    println!("=== InformalNormalizer — single words ===");
    let informal_words = [
        "واسه",    // informal for برای (for)
        "اینجوری", // informal for اینطوری (like this)
        "میگم",    // informal for می‌گویم (I say)
        "بچه‌هام", // informal for بچه‌هایم (my children)
        "ریختن",   // already formal — should pass through
        "خونه",    // informal for خانه (home)
    ];
    for w in &informal_words {
        let candidates = n.normalized_word(w);
        println!("  {:15} → {:?}", w, candidates);
    }

    // -----------------------------------------------------------------------
    // InformalNormalizer — full sentences
    // -----------------------------------------------------------------------
    println!("\n=== InformalNormalizer — full sentence ===");
    let text = "بابا یه شغل مناسب واسه بچه هام پیدا کرده";
    println!("Input: {text}");
    let result = n.normalize(text);
    for (si, sentence) in result.iter().enumerate() {
        println!("  Sentence [{si}]:");
        for (ti, candidates) in sentence.iter().enumerate() {
            if candidates.len() == 1 {
                println!("    [{ti}] {}", candidates[0]);
            } else {
                println!("    [{ti}] {:?}  (multiple candidates)", candidates);
            }
        }
    }

    // -----------------------------------------------------------------------
    // InformalNormalizer — accessor methods
    // -----------------------------------------------------------------------
    println!("\n=== InformalNormalizer — accessor info ===");
    println!("  Past verb map size:    {}", n.past_verb_map().len());
    println!("  Present verb map size: {}", n.present_verb_map().len());

    // -----------------------------------------------------------------------
    // InformalLemmatizer
    // -----------------------------------------------------------------------
    println!("\n=== InformalLemmatizer ===");
    let lem = InformalLemmatizer::new();

    let checks = [
        ("واسه",   ""),      // informal word
        ("کتاب",   ""),      // formal word — should be in lexicon
        ("میگم",   "V"),     // informal verb form
        ("xyz_nonexistent", ""),
    ];
    for (w, pos) in &checks {
        println!(
            "  {:20} in_lexicon={:5}  lemma={:?}",
            w,
            lem.contains(w),
            lem.lemmatize(w, pos)
        );
    }
}
