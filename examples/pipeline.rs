//! End-to-end Persian NLP pipeline demo.
//!
//! Shows the full processing chain on a sample text, then demonstrates each
//! subsystem independently.
//!
//! Run with:
//! ```
//! cargo run --example pipeline
//! ```

use rustam::{
    Conjugation, InformalNormalizer, Lemmatizer, Normalizer, SentenceTokenizer, SpellCorrector,
    Stemmer, TokenSplitter, WordTokenizer,
};

fn header(s: &str) {
    println!("\n{}", "─".repeat(60));
    println!("  {s}");
    println!("{}", "─".repeat(60));
}

fn main() {
    let raw = "اِعلاممممم کَرد : « زمین لرزه ای به بُزرگیِ 6 دهم ریشتر ...»";
    println!("Input text:\n  {raw}");

    // -----------------------------------------------------------------------
    // Step 1 — Normalize
    // -----------------------------------------------------------------------
    header("1. Normalization");
    let norm = Normalizer::new();
    let normalized = norm.normalize(raw);
    println!("  {normalized}");

    // -----------------------------------------------------------------------
    // Step 2 — Sentence tokenize
    // -----------------------------------------------------------------------
    header("2. Sentence Tokenization");
    let text2 = "سلام. امروز هوا خوب است! آیا شما موافقید؟";
    let sent_tok = SentenceTokenizer::new();
    for (i, s) in sent_tok.tokenize(text2).iter().enumerate() {
        println!("  [{i}] {s}");
    }

    // -----------------------------------------------------------------------
    // Step 3 — Word tokenize
    // -----------------------------------------------------------------------
    header("3. Word Tokenization");
    let word_tok = WordTokenizer::new();
    let tokens = word_tok.tokenize(&normalized);
    println!("  {:?}", tokens);

    // Multi-part verb joining
    let verbs_text = "او رفته است و ما خواهیم رفت";
    println!("  verb joining: {:?}", word_tok.tokenize(verbs_text));

    // -----------------------------------------------------------------------
    // Step 4 — Stem
    // -----------------------------------------------------------------------
    header("4. Stemming");
    let stemmer = Stemmer::new();
    let stems: Vec<(&str, String)> = tokens
        .iter()
        .map(|t| (t.as_str(), stemmer.stem(t)))
        .collect();
    for (t, s) in &stems {
        if t != s {
            println!("  {:20} → {s}", t);
        }
    }
    if stems.iter().all(|(t, s)| t == s) {
        println!("  (all tokens already at their stem in this sample)");
    }

    // -----------------------------------------------------------------------
    // Step 5 — Lemmatize
    // -----------------------------------------------------------------------
    header("5. Lemmatization");
    let lem = Lemmatizer::new();
    let verb_tokens = ["می‌روم", "رفته‌ام", "گفته_شده_است", "می‌بینند"];
    for t in &verb_tokens {
        println!("  {:25} → {}", t, lem.lemmatize(t, ""));
    }

    // -----------------------------------------------------------------------
    // Step 6 — Conjugation
    // -----------------------------------------------------------------------
    header("6. Conjugation  (دیدن — past: دید, present: بین)");
    let c = Conjugation;
    let pronouns = ["من", "تو", "او", "ما", "شما", "آنها"];
    let past_forms = c.perfective_past("دید");
    let present_forms = c.imperfective_present("بین");
    let future_forms = c.perfective_future("دید");
    println!("  {:8} {:18} {:22} {:?}", "pronoun", "simple past", "imperfective present", "future");
    for i in 0..6 {
        println!(
            "  {:8} {:18} {:22} {}",
            pronouns[i], past_forms[i], present_forms[i], future_forms[i]
        );
    }
    println!("  Total forms via get_all: {}", c.get_all("دید#بین").len());

    // -----------------------------------------------------------------------
    // Step 7 — Informal normalization
    // -----------------------------------------------------------------------
    header("7. Informal Normalization");
    let informal = InformalNormalizer::new();
    let chat = "بابا یه شغل مناسب واسه بچه هام پیدا کرده";
    println!("  input: {chat}");
    for sentence in informal.normalize(chat) {
        let best: Vec<&str> = sentence
            .iter()
            .map(|cands| cands[0].as_str())
            .collect();
        println!("  best:  {}", best.join(" "));
    }

    // -----------------------------------------------------------------------
    // Step 8 — Token splitting
    // -----------------------------------------------------------------------
    header("8. Token Splitting (compound decomposition)");
    let splitter = TokenSplitter::new();
    let compounds = ["کتاب‌خانه", "صداوسیما", "دانشگاه"];
    for token in &compounds {
        let cands = splitter.split(token);
        println!("  {:20} → {:?}", token, cands.first().unwrap_or(&vec![token.to_string()]));
    }

    // -----------------------------------------------------------------------
    // Step 9 — Spell correction
    // -----------------------------------------------------------------------
    header("9. Spell Correction");
    let sc = SpellCorrector::new();
    let checks = ["کتاب", "مدرسه", "و"];
    for w in &checks {
        println!(
            "  {:15} known={:5}  correction={}  p={:.6}",
            w,
            sc.known(w),
            sc.correction(w),
            sc.probability(w)
        );
    }
}
