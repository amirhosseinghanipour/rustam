//! Demonstrates compound token splitting.
//!
//! [`TokenSplitter`] decomposes a compound token into component words using
//! the embedded lexicon.  It returns all valid split candidates.
//!
//! Run with:
//! ```
//! cargo run --example token_split
//! ```

use rustam::TokenSplitter;

fn main() {
    let splitter = TokenSplitter::new();

    println!("=== TokenSplitter ===");
    let tokens = [
        "صداوسیما",          // broadcast org — صدا + و + سیما
        "کتاب‌خانه",         // library — کتاب + خانه (ZWNJ-joined)
        "ایران‌گردی",        // travel around Iran
        "خانه",              // single word — should come back as itself
        "دانشگاه",           // university
        "دانشمند",           // scientist
    ];

    for token in &tokens {
        let candidates = splitter.split(token);
        match candidates.len() {
            0 => println!("  {:20} → (no candidates)", token),
            1 => println!("  {:20} → {:?}", token, candidates[0]),
            _ => {
                println!("  {:20} → {} candidates:", token, candidates.len());
                for (i, parts) in candidates.iter().enumerate() {
                    println!("              [{i}] {:?}", parts);
                }
            }
        }
    }
}
