use rustam::{
    Conjugation, Lemmatizer, Normalizer, Stemmer, WordTokenizer,
    SentenceTokenizer,
};

// ---------------------------------------------------------------------------
// Sentence tokenizer
// ---------------------------------------------------------------------------

#[test]
fn sentence_tokenizer_splits_on_period() {
    let tok = SentenceTokenizer::new();
    let result = tok.tokenize("جدا کردن ساده است. تقریبا البته!");
    assert_eq!(result, vec!["جدا کردن ساده است.", "تقریبا البته!"]);
}

#[test]
fn sentence_tokenizer_persian_question_mark() {
    let tok = SentenceTokenizer::new();
    let result = tok.tokenize("چطوری؟ خوبم.");
    assert_eq!(result, vec!["چطوری؟", "خوبم."]);
}

#[test]
fn sentence_tokenizer_empty_string() {
    let tok = SentenceTokenizer::new();
    assert!(tok.tokenize("").is_empty());
}

// ---------------------------------------------------------------------------
// Stemmer
// ---------------------------------------------------------------------------

#[test]
fn stemmer_strips_ha_plural() {
    let s = Stemmer::new();
    assert_eq!(s.stem("کتاب‌ها"), "کتاب");
}

#[test]
fn stemmer_strips_i_suffix() {
    let s = Stemmer::new();
    assert_eq!(s.stem("کتابی"), "کتاب");
}

#[test]
fn stemmer_strips_hayi_suffix() {
    let s = Stemmer::new();
    assert_eq!(s.stem("کتاب‌هایی"), "کتاب");
}

#[test]
fn stemmer_strips_clitic() {
    let s = Stemmer::new();
    assert_eq!(s.stem("اندیشه‌اش"), "اندیشه");
}

#[test]
fn stemmer_normalizes_heh_hamza() {
    let s = Stemmer::new();
    assert_eq!(s.stem("خانۀ"), "خانه");
}

#[test]
fn stemmer_passthrough_short_stem() {
    // "دم" has a single-char suffix ی but removing it leaves < 3 chars — should not strip.
    let s = Stemmer::new();
    // "دم" is only 2 chars; don't strip.
    assert_eq!(s.stem("دم"), "دم");
}

// ---------------------------------------------------------------------------
// Word tokenizer
// ---------------------------------------------------------------------------

#[test]
fn word_tokenizer_basic_split() {
    let tok = WordTokenizer::new();
    let result = tok.tokenize("این جمله (خیلی) پیچیده نیست!!!");
    assert_eq!(
        result,
        vec!["این", "جمله", "(", "خیلی", ")", "پیچیده", "نیست", "!!!"]
    );
}

#[test]
fn word_tokenizer_joins_past_perfect() {
    let tok = WordTokenizer::new();
    let result = tok.tokenize("رفته است");
    assert_eq!(result, vec!["رفته_است"]);
}

#[test]
fn word_tokenizer_joins_compound_verb() {
    let tok = WordTokenizer::new();
    let result = tok.tokenize("گفته شده است");
    assert_eq!(result, vec!["گفته_شده_است"]);
}

#[test]
fn word_tokenizer_joins_future_verb() {
    let tok = WordTokenizer::new();
    let result = tok.tokenize("خواهد رفت");
    assert_eq!(result, vec!["خواهد_رفت"]);
}

#[test]
fn word_tokenizer_no_verb_joining() {
    use rustam::WordTokenizerConfig;
    let tok = WordTokenizer::with_config(WordTokenizerConfig {
        join_verb_parts: false,
        ..Default::default()
    });
    let result = tok.tokenize("سلام.");
    assert!(result.contains(&"سلام".to_string()));
    assert!(result.contains(&".".to_string()));
}

// ---------------------------------------------------------------------------
// Conjugation
// ---------------------------------------------------------------------------

#[test]
fn conjugation_perfective_past() {
    let c = Conjugation;
    assert_eq!(
        c.perfective_past("دید"),
        vec!["دیدم", "دیدی", "دید", "دیدیم", "دیدید", "دیدند"]
    );
}

#[test]
fn conjugation_imperfective_present() {
    let c = Conjugation;
    assert_eq!(
        c.imperfective_present("بین"),
        vec!["می‌بینم", "می‌بینی", "می‌بیند", "می‌بینیم", "می‌بینید", "می‌بینند"]
    );
}

#[test]
fn conjugation_perfective_future() {
    let c = Conjugation;
    assert_eq!(
        c.perfective_future("دید"),
        vec!["خواهم دید", "خواهی دید", "خواهد دید", "خواهیم دید", "خواهید دید", "خواهند دید"]
    );
}

#[test]
fn conjugation_negative_perfective_past() {
    let c = Conjugation;
    assert_eq!(
        c.negative_perfective_past("دید"),
        vec!["ندیدم", "ندیدی", "ندید", "ندیدیم", "ندیدید", "ندیدند"]
    );
}

#[test]
fn conjugation_present_perfect() {
    let c = Conjugation;
    let forms = c.present_perfect("دید");
    assert_eq!(forms[0], "دیده‌ام");
    assert_eq!(forms[2], "دیده است");
}

#[test]
fn conjugation_get_all_non_empty() {
    let c = Conjugation;
    let forms = c.get_all("دید#بین");
    assert!(!forms.is_empty());
    assert!(forms.contains(&"دیدم".to_string()));
    assert!(forms.contains(&"می‌بینم".to_string()));
    assert!(forms.contains(&"خواهم دید".to_string()));
}

#[test]
fn conjugation_imperfective_past() {
    let c = Conjugation;
    let forms = c.imperfective_past("دید");
    // [0] = می‌دیدم, [2] = می‌دید
    assert_eq!(forms[0], "می‌دیدم");
    assert_eq!(forms[2], "می‌دید");
}

#[test]
fn conjugation_passive_perfective_past() {
    let c = Conjugation;
    let forms = c.passive_perfective_past("دید");
    // forms[0] should be "دیده شدم"
    assert!(forms[0].contains("دیده"));
}

#[test]
fn conjugation_subjunctive_perfective_present() {
    let c = Conjugation;
    let forms = c.subjunctive_perfective_present("بین");
    // ببینم is first person singular subjunctive of بین
    assert_eq!(forms[0], "ببینم");
}

// ---------------------------------------------------------------------------
// Lemmatizer
// ---------------------------------------------------------------------------

#[test]
fn lemmatizer_lemmatizes_plural_noun() {
    let lem = Lemmatizer::new();
    assert_eq!(lem.lemmatize("کتاب‌ها", ""), "کتاب");
}

#[test]
fn lemmatizer_passthrough_known_word() {
    let lem = Lemmatizer::new();
    assert_eq!(lem.lemmatize("آتشفشان", ""), "آتشفشان");
}

#[test]
fn lemmatizer_verb_present() {
    let lem = Lemmatizer::new();
    // می‌روم → رفت#رو
    assert_eq!(lem.lemmatize("می‌روم", ""), "رفت#رو");
}

#[test]
fn lemmatizer_joined_verb() {
    let lem = Lemmatizer::new();
    assert_eq!(lem.lemmatize("گفته_شده_است", ""), "گفت#گو");
}

#[test]
fn lemmatizer_adj_passthrough() {
    let lem = Lemmatizer::new();
    assert_eq!(lem.lemmatize("اجتماعی", "ADJ"), "اجتماعی");
}

#[test]
fn lemmatizer_pron_passthrough() {
    let lem = Lemmatizer::new();
    assert_eq!(lem.lemmatize("مردم", "PRON"), "مردم");
}

// ---------------------------------------------------------------------------
// Normalizer
// ---------------------------------------------------------------------------

#[test]
fn normalizer_removes_diacritics() {
    let n = Normalizer::new();
    assert_eq!(n.remove_diacritics("حَذفِ اِعراب"), "حذف اعراب");
}

#[test]
fn normalizer_persian_number() {
    let n = Normalizer::new();
    assert_eq!(n.persian_number("5 درصد"), "۵ درصد");
}

#[test]
fn normalizer_persian_style_quotes() {
    let n = Normalizer::new();
    assert_eq!(n.persian_style("\"نرمال‌سازی\""), "«نرمال‌سازی»");
}

#[test]
fn normalizer_persian_style_ellipsis() {
    let n = Normalizer::new();
    assert_eq!(n.persian_style("و ..."), "و …");
}

#[test]
fn normalizer_remove_specials_chars() {
    let n = Normalizer::new();
    // ﷺ is U+FDFA which is in SPECIAL_CHARS_PATTERN
    let result = n.remove_specials_chars("پیامبر اکرم \u{FDFA}");
    assert!(!result.contains('\u{FDFA}'));
    assert!(result.contains("پیامبر اکرم"));
}

#[test]
fn normalizer_empty_string() {
    let n = Normalizer::new();
    assert_eq!(n.normalize(""), "");
}

// ---------------------------------------------------------------------------
// InformalNormalizer
// ---------------------------------------------------------------------------

use rustam::InformalNormalizer;

#[test]
fn informal_normalizer_direct_word_map() {
    // "واسه" is informal for "برای"
    let n = InformalNormalizer::new();
    let candidates = n.normalized_word("واسه");
    assert!(candidates.contains(&"برای".to_string()), "candidates: {candidates:?}");
}

#[test]
fn informal_normalizer_preserves_known_formal_word() {
    let n = InformalNormalizer::new();
    let candidates = n.normalized_word("کتاب");
    assert_eq!(candidates, vec!["کتاب"]);
}

#[test]
fn informal_normalizer_normalize_returns_structure() {
    let n = InformalNormalizer::new();
    let result = n.normalize("سلام خوبی");
    // Result is Vec<Vec<Vec<String>>>: sentences > tokens > candidates
    assert!(!result.is_empty());
    assert!(!result[0].is_empty());
}

// ---------------------------------------------------------------------------
// SpellCorrector
// ---------------------------------------------------------------------------

use rustam::SpellCorrector;

#[test]
fn spell_corrector_known_word_unchanged() {
    let sc = SpellCorrector::new();
    assert_eq!(sc.correction("کتاب"), "کتاب");
}

#[test]
fn spell_corrector_known_returns_true() {
    let sc = SpellCorrector::new();
    assert!(sc.known("کتاب"));
    assert!(!sc.known("xyz_nonexistent"));
}

#[test]
fn spell_corrector_probability_positive_for_common_word() {
    let sc = SpellCorrector::new();
    assert!(sc.probability("کتاب") > 0.0);
}

#[test]
fn spell_corrector_probability_zero_for_unknown() {
    let sc = SpellCorrector::new();
    assert_eq!(sc.probability("xyzxyz_nonexistent"), 0.0);
}

// ---------------------------------------------------------------------------
// TokenSplitter
// ---------------------------------------------------------------------------

use rustam::TokenSplitter;

#[test]
fn token_splitter_splits_compound_word() {
    let splitter = TokenSplitter::new();
    // "صداوسیماجمهوری" is a known compound — "صداوسیما" + "جمهوری"
    let result = splitter.split("صداوسیماجمهوری");
    assert!(!result.is_empty(), "expected at least one split candidate");
    assert!(
        result.iter().any(|parts| parts == &vec!["صداوسیما".to_string(), "جمهوری".to_string()]),
        "expected split into [صداوسیما, جمهوری], got: {result:?}"
    );
}

#[test]
fn token_splitter_zwnj_split() {
    let splitter = TokenSplitter::new();
    // A ZWNJ-joined compound that both parts exist in the lexicon
    // "کتاب‌خانه" → ["کتاب", "خانه"]
    let result = splitter.split("کتاب\u{200C}خانه");
    assert!(
        result.iter().any(|parts| {
            parts.len() == 2
                && parts[0] == "کتاب"
                && parts[1] == "خانه"
        }),
        "expected ZWNJ split into [کتاب, خانه], got: {result:?}"
    );
}

#[test]
fn token_splitter_single_lexicon_word_returns_itself() {
    let splitter = TokenSplitter::new();
    // A simple dictionary word should be returned as itself (whole-token candidate)
    let result = splitter.split("کتاب");
    assert!(
        result.iter().any(|parts| parts == &vec!["کتاب".to_string()]),
        "expected [کتاب] as a candidate, got: {result:?}"
    );
}

// ---------------------------------------------------------------------------
// InformalLemmatizer
// ---------------------------------------------------------------------------

use rustam::InformalLemmatizer;

#[test]
fn informal_lemmatizer_contains_informal_word() {
    let lem = InformalLemmatizer::new();
    // "واسه" is informal for "برای" — should be in the informal word set
    assert!(lem.contains("واسه"));
}

#[test]
fn informal_lemmatizer_contains_formal_word() {
    let lem = InformalLemmatizer::new();
    // "کتاب" is a common formal word
    assert!(lem.contains("کتاب"));
}

// ---------------------------------------------------------------------------
// Data accessors
// ---------------------------------------------------------------------------

use rustam::{words_list, verbs_list, stopwords_list, past_roots, present_roots};

#[test]
fn data_words_list_non_empty() {
    assert!(!words_list().is_empty());
}

#[test]
fn data_verbs_list_non_empty() {
    let verbs = verbs_list();
    assert!(!verbs.is_empty());
    // At least some entries should have non-empty past and present roots
    assert!(verbs.iter().any(|v| !v.past.is_empty() && !v.present.is_empty()));
}

#[test]
fn data_stopwords_non_empty() {
    let sw = stopwords_list();
    assert!(!sw.is_empty());
    assert!(sw.contains(&"و".to_string()));
}

#[test]
fn data_past_roots_non_empty() {
    assert!(!past_roots().is_empty());
}

#[test]
fn data_present_roots_non_empty() {
    assert!(!present_roots().is_empty());
}
