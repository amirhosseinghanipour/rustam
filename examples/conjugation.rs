//! Demonstrates the full conjugation table for a Persian verb.
//!
//! Uses دیدن (to see) whose past root is دید and present root is بین.
//!
//! Run with:
//! ```
//! cargo run --example conjugation
//! ```

use rustam::Conjugation;

fn print_forms(label: &str, forms: &[String]) {
    let pronouns = ["من", "تو", "او", "ما", "شما", "آنها"];
    println!("  {label}:");
    for (pron, form) in pronouns.iter().zip(forms.iter()) {
        println!("    {pron:5} → {form}");
    }
}

fn main() {
    let c = Conjugation;
    let past = "دید";      // past root of دیدن (to see)
    let present = "بین";   // present root of دیدن

    println!("=== Conjugation of دیدن  (past: {past}, present: {present}) ===\n");

    // --- Past tenses ---
    println!("── Past ─────────────────────────────────────");
    print_forms("Perfective past (simple past)", &c.perfective_past(past));
    print_forms("Negative perfective past", &c.negative_perfective_past(past));
    print_forms("Imperfective past (habitual)", &c.imperfective_past(past));
    print_forms("Negative imperfective past", &c.negative_imperfective_past(past));
    print_forms("Past progressive", &c.past_progressive(past));

    println!("\n── Present Perfect ───────────────────────────");
    print_forms("Present perfect", &c.present_perfect(past));
    print_forms("Negative present perfect", &c.negative_present_perfect(past));
    print_forms("Subjunctive present perfect", &c.subjunctive_present_perfect(past));
    print_forms("Imperfective present perfect", &c.imperfective_present_perfect(past));

    println!("\n── Present ───────────────────────────────────");
    print_forms("Perfective present (simple present)", &c.perfective_present(present));
    print_forms("Negative perfective present", &c.negative_perfective_present(present));
    print_forms("Subjunctive present", &c.subjunctive_perfective_present(present));
    print_forms("Imperfective present (continuous)", &c.imperfective_present(present));
    print_forms("Negative imperfective present", &c.negative_imperfective_present(present));
    print_forms("Present progressive", &c.present_progressive(present));

    println!("\n── Future ────────────────────────────────────");
    print_forms("Perfective future", &c.perfective_future(past));
    print_forms("Negative perfective future", &c.negative_perfective_future(past));
    print_forms("Imperfective future", &c.imperfective_future(past));

    println!("\n── Passive (sample) ──────────────────────────");
    print_forms("Passive perfective past", &c.passive_perfective_past(past));
    print_forms("Passive present perfect", &c.passive_present_perfect(past));
    print_forms("Passive perfective present", &c.passive_perfective_present(past));

    println!("\n── get_all ───────────────────────────────────");
    let all = c.get_all(&format!("{past}#{present}"));
    println!("  Total forms for {past}#{present}: {}", all.len());
    println!("  First 6: {:?}", &all[..6]);
}
