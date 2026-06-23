//! Demonstrates sentence and word tokenization.
//!
//! Run with:
//! ```
//! cargo run --example tokenize
//! ```

use rustam::{SentenceTokenizer, WordTokenizer, WordTokenizerConfig};

fn main() {
    // -----------------------------------------------------------------------
    // Sentence tokenizer
    // -----------------------------------------------------------------------
    let text = "سلام. حال شما چطور است؟ امیدوارم خوب باشید!";
    let sent_tok = SentenceTokenizer::new();
    let sentences = sent_tok.tokenize(text);
    println!("=== Sentence Tokenizer ===");
    println!("Input:     {text}");
    for (i, s) in sentences.iter().enumerate() {
        println!("  [{i}] {s}");
    }

    // -----------------------------------------------------------------------
    // Word tokenizer — default (joins multi-part verbs with _)
    // -----------------------------------------------------------------------
    let sentence = "او رفته است و ما خواهیم رفت";
    let word_tok = WordTokenizer::new();
    println!("\n=== Word Tokenizer (verb joining ON) ===");
    println!("Input:  {sentence}");
    println!("Tokens: {:?}", word_tok.tokenize(sentence));

    // -----------------------------------------------------------------------
    // Word tokenizer — verb joining disabled
    // -----------------------------------------------------------------------
    let no_join = WordTokenizer::without_verb_joining();
    println!("\n=== Word Tokenizer (verb joining OFF) ===");
    println!("Input:  {sentence}");
    println!("Tokens: {:?}", no_join.tokenize(sentence));

    // -----------------------------------------------------------------------
    // Word tokenizer — custom config
    // -----------------------------------------------------------------------
    let custom = WordTokenizer::with_config(WordTokenizerConfig {
        join_verb_parts: true,
        replace_emails: true,
        replace_links: true,
        replace_ids: false,
        replace_numbers: false,
        replace_hashtags: false,
    });
    let mixed = "ایمیل من test@example.com است و وبسایت https://example.com را ببینید";
    println!("\n=== Word Tokenizer (email + URL masking) ===");
    println!("Input:  {mixed}");
    println!("Tokens: {:?}", custom.tokenize(mixed));

    // -----------------------------------------------------------------------
    // Punctuation and parentheses
    // -----------------------------------------------------------------------
    let punct = "این جمله (خیلی) پیچیده نیست!!!";
    println!("\n=== Punctuation handling ===");
    println!("Input:  {punct}");
    println!("Tokens: {:?}", word_tok.tokenize(punct));
}
