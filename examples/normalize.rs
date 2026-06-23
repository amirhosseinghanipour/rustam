//! Demonstrates every normalization step available in [`Normalizer`].
//!
//! Run with:
//! ```
//! cargo run --example normalize
//! ```

use rustam::{Normalizer, NormalizerConfig};

fn main() {
    let n = Normalizer::new();

    // -----------------------------------------------------------------------
    // Full pipeline (all steps enabled)
    // -----------------------------------------------------------------------
    let raw = "اِعلاممممم کَرد : « زمین لرزه ای به بُزرگیِ 6 دهم ریشتر ...»";
    println!("Input:      {raw}");
    println!("Normalized: {}\n", n.normalize(raw));

    // -----------------------------------------------------------------------
    // Individual steps
    // -----------------------------------------------------------------------

    // 1. Remove Arabic diacritics (tashkeel)
    let diacritics = "حَذفِ اِعراب";
    println!("Diacritics in:  {diacritics}");
    println!("Diacritics out: {}\n", n.remove_diacritics(diacritics));

    // 2. Convert digits to Persian (ASCII → ۰–۹, Arabic-Indic → ۰–۹)
    let digits = "score: 98.6 و ٤٢ امتیاز";
    println!("Digits in:  {digits}");
    println!("Digits out: {}\n", n.persian_number(digits));

    // 3. Persian typographic style (guillemets, ellipsis, decimal point)
    let style_in = "\"نرمال‌سازی\" و 6.5 درصد و ...";
    println!("Style in:  {style_in}");
    println!("Style out: {}\n", n.persian_style(style_in));

    // 4. Remove decorative Unicode special characters
    let special = "پیامبر اکرم \u{FDFA} بزرگ است";
    println!("Specials in:  {special}");
    println!("Specials out: {}\n", n.remove_specials_chars(&special));

    // 5. Reduce excessive character repetition (3+ → ≤2)
    let repeated = "خیلیییییی زیباست";
    println!("Repeated in:  {repeated}");
    println!("Repeated out: {}\n", n.decrease_repeated_chars(repeated));

    // 6. Separate می/نمی prefix with ZWNJ
    let mi_in = "میروم و نمیدانم";
    println!("می in:  {mi_in}");
    println!("می out: {}\n", n.seperate_mi(mi_in));

    // 7. Replace Unicode ligatures
    let ligature = "\u{FDF2} الله";  // U+FDF2 ARABIC LIGATURE ALLAH
    println!("Ligature in:  {ligature}");
    println!("Ligature out: {}\n", n.unicodes_replacement(ligature));

    // 8. Fix spacing around punctuation
    let spacing = "کتاب ،که خوب بود،  را خواندم  .";
    println!("Spacing in:  {spacing}");
    println!("Spacing out: {}\n", n.correct_spacing(spacing));

    // -----------------------------------------------------------------------
    // Custom config: only diacritics removal and Persian numbers
    // -----------------------------------------------------------------------
    let minimal = Normalizer::with_config(NormalizerConfig {
        correct_spacing: false,
        remove_diacritics: true,
        remove_specials_chars: false,
        decrease_repeated_chars: false,
        persian_style: false,
        persian_numbers: true,
        unicodes_replacement: false,
        seperate_mi: false,
    });
    println!("Minimal config input:  {raw}");
    println!("Minimal config output: {}", minimal.normalize(raw));
}
