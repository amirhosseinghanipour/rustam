//! Demonstrates access to the embedded lexicon and verb data.
//!
//! All data is compiled into the binary at build time via `include_str!`.
//!
//! Run with:
//! ```
//! cargo run --example data
//! ```

use rustam::{past_roots, present_roots, stopwords_list, verbs_list, words_list};

fn main() {
    // -----------------------------------------------------------------------
    // Word lexicon
    // -----------------------------------------------------------------------
    let words = words_list();
    println!("=== Word lexicon ===");
    println!("  Total entries: {}", words.len());

    // Show a few entries
    let samples = ["کتاب", "مدرسه", "آتشفشان", "ایران"];
    for w in &samples {
        if let Some(entry) = words.get(*w) {
            println!("  {:15} freq={:6}  pos={:?}", w, entry.frequency, entry.pos_tags);
        }
    }

    // -----------------------------------------------------------------------
    // Verb roots
    // -----------------------------------------------------------------------
    let verbs = verbs_list();
    println!("\n=== Verb roots ===");
    println!("  Total verb pairs: {}", verbs.len());
    println!("  First 5:");
    for v in verbs.iter().take(5) {
        println!("    past={:15} present={}", v.past, v.present);
    }

    // -----------------------------------------------------------------------
    // Past and present root sets (for quick membership tests)
    // -----------------------------------------------------------------------
    let pasts = past_roots();
    let presents = present_roots();
    println!("\n=== Root lists ===");
    println!("  Past roots:    {} entries", pasts.len());
    println!("  Present roots: {} entries", presents.len());
    println!("  'دید' is a past root:    {}", pasts.contains(&"دید".to_string()));
    println!("  'بین' is a present root: {}", presents.contains(&"بین".to_string()));

    // -----------------------------------------------------------------------
    // Stopwords
    // -----------------------------------------------------------------------
    let stops = stopwords_list();
    println!("\n=== Stopwords ===");
    println!("  Total: {}", stops.len());
    println!("  First 10: {:?}", &stops[..stops.len().min(10)]);
    println!("  'و' is a stopword: {}", stops.contains(&"و".to_string()));
    println!("  'کتاب' is a stopword: {}", stops.contains(&"کتاب".to_string()));
}
