//! Demonstrates stemming and lemmatization.
//!
//! Run with:
//! ```
//! cargo run --example stem_lemmatize
//! ```

use rustam::{Lemmatizer, Stemmer};

fn main() {
    // -----------------------------------------------------------------------
    // Stemmer
    // -----------------------------------------------------------------------
    let stemmer = Stemmer::new();

    let words = [
        "کتاب‌ها",    // books → کتاب (removes ‌ها plural)
        "کتابی",      // a book → کتاب (removes ی indef.)
        "کتاب‌هایی",  // some books → کتاب
        "اندیشه‌اش",  // his thought → اندیشه
        "خانۀ",       // house (ۀ form) → خانه
        "رفته‌ام",    // I have gone → رفته
        "می‌خواهند",  // they want → می‌خواه
    ];

    println!("=== Stemmer ===");
    for w in &words {
        println!("  {:20} → {}", w, stemmer.stem(w));
    }

    // -----------------------------------------------------------------------
    // Lemmatizer — no POS hint
    // -----------------------------------------------------------------------
    let lem = Lemmatizer::new();

    let nouns_and_adj = [
        ("کتاب‌ها",     ""),     // books
        ("اجتماعی",    "ADJ"),  // social (passthrough for adj)
        ("مردم",       "PRON"), // people (passthrough for pronoun)
        ("آتشفشان",    ""),     // volcano (known word)
    ];

    println!("\n=== Lemmatizer (nouns / adjectives) ===");
    for (w, pos) in &nouns_and_adj {
        println!("  {:20} [{}]  → {}", w, pos, lem.lemmatize(w, pos));
    }

    // Verbs — returned as past#present
    let verbs = [
        ("می‌روم",           ""),  // I go   → رفت#رو
        ("می‌رفتم",          ""),  // I went → رفت#رو
        ("رفته‌ام",          ""),  // I have gone
        ("گفته_شده_است",    ""),  // (compound) has been said → گفت#گو
        ("خواهند_رفت",      ""),  // they will go
        ("می‌بینند",        ""),  // they see → دید#بین
    ];

    println!("\n=== Lemmatizer (verbs) ===");
    for (w, pos) in &verbs {
        println!("  {:25} → {}", w, lem.lemmatize(w, pos));
    }

    // -----------------------------------------------------------------------
    // Lemmatizer with joined verb parts disabled
    //   (use when the input is already pre-split, not joined with _)
    // -----------------------------------------------------------------------
    let lem_no_join = Lemmatizer::with_joined_parts(false);
    println!("\n=== Lemmatizer (joined_parts=false) ===");
    println!(
        "  رفته است  → {}",
        lem_no_join.lemmatize("رفته", "")
    );
}
