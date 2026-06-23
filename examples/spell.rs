//! Demonstrates the spell corrector.
//!
//! Run with:
//! ```
//! cargo run --example spell
//! ```

use rustam::SpellCorrector;

fn main() {
    let sc = SpellCorrector::new();

    // -----------------------------------------------------------------------
    // Known word check
    // -----------------------------------------------------------------------
    println!("=== Known word check ===");
    let words = ["کتاب", "مدرسه", "xyz_nonexistent", "ایران"];
    for w in &words {
        println!("  {:20} known={}", w, sc.known(w));
    }

    // -----------------------------------------------------------------------
    // Probability (relative corpus frequency)
    // -----------------------------------------------------------------------
    println!("\n=== Probability ===");
    let freq_words = ["و", "کتاب", "آتشفشان", "xyz"];
    for w in &freq_words {
        println!("  {:20} p={:.8}", w, sc.probability(w));
    }

    // -----------------------------------------------------------------------
    // Best correction
    // -----------------------------------------------------------------------
    println!("\n=== Correction ===");
    let misspelled = [
        "کتاب",        // already correct — should be unchanged
        "مدرسه",       // correct
        "کتاب‌ها",     // correct
    ];
    for w in &misspelled {
        println!("  {:20} → {}", w, sc.correction(w));
    }

    // -----------------------------------------------------------------------
    // All candidates (edit-distance 1 words that are in the lexicon)
    // -----------------------------------------------------------------------
    println!("\n=== Candidates for 'کتاب' ===");
    let cands = sc.candidates("کتاب");
    println!("  {:?}", cands);

    // -----------------------------------------------------------------------
    // Edit distance 1 set (all possible single-edit variants)
    // -----------------------------------------------------------------------
    println!("\n=== Edits-1 sample for 'در' ===");
    let mut edits: Vec<String> = sc.edits1("در").into_iter().collect();
    edits.sort();
    println!("  {} variants, first 10: {:?}", edits.len(), &edits[..edits.len().min(10)]);
}
