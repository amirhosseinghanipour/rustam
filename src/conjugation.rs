/// Generates all conjugated forms of Persian verbs.
///
/// Each verb is described by two roots:
/// - `ri`  — بن ماضی (past root, e.g. `"دید"`)
/// - `rii` — بن مضارع (present root, e.g. `"بین"`)
///
/// # Examples
///
/// ```
/// use rustam::Conjugation;
///
/// let c = Conjugation;
/// assert_eq!(c.perfective_past("دید"), vec!["دیدم", "دیدی", "دید", "دیدیم", "دیدید", "دیدند"]);
/// assert_eq!(c.imperfective_present("بین"), vec!["می‌بینم", "می‌بینی", "می‌بیند", "می‌بینیم", "می‌بینید", "می‌بینند"]);
/// ```
pub struct Conjugation;

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn past_suffixes(ri: &str) -> [String; 6] {
    ["م", "ی", "", "یم", "ید", "ند"].map(|s| format!("{ri}{s}"))
}

fn present_suffixes(rii: &str) -> [String; 6] {
    ["م", "ی", "د", "یم", "ید", "ند"].map(|s| format!("{rii}{s}"))
}

fn neg<const N: usize>(forms: [String; N]) -> [String; N] {
    forms.map(|f| format!("ن{f}"))
}

fn mi<const N: usize>(forms: [String; N]) -> [String; N] {
    forms.map(|f| format!("می‌{f}"))
}

fn nami<const N: usize>(forms: [String; N]) -> [String; N] {
    forms.map(|f| format!("نمی‌{f}"))
}

fn passive_suffix(ri: &str, after: &str) -> String {
    format!("{ri}ه {after}")
}

fn zip_join(a: [String; 6], b: [String; 6]) -> [String; 6] {
    std::array::from_fn(|i| format!("{} {}", a[i], b[i]))
}

impl Conjugation {
    // -----------------------------------------------------------------------
    // گذشتهٔ مطلق  (simple past)
    // -----------------------------------------------------------------------

    /// Returns simple past (گذشتهٔ مطلق) forms for the given past root.
    pub fn perfective_past(&self, ri: &str) -> Vec<String> {
        past_suffixes(ri).into()
    }

    /// Returns negative simple past forms for the given past root.
    pub fn negative_perfective_past(&self, ri: &str) -> Vec<String> {
        neg(past_suffixes(ri)).into()
    }

    /// Returns passive simple past forms for the given past root.
    pub fn passive_perfective_past(&self, ri: &str) -> Vec<String> {
        past_suffixes("شد")
            .map(|s| passive_suffix(ri, &s))
            .into()
    }

    /// Returns negative passive simple past forms for the given past root.
    pub fn negative_passive_perfective_past(&self, ri: &str) -> Vec<String> {
        neg(past_suffixes("شد"))
            .map(|s| passive_suffix(ri, &s))
            .into()
    }

    // -----------------------------------------------------------------------
    // گذشتهٔ پایا  (past imperfective / habitual past)
    // -----------------------------------------------------------------------

    /// Returns habitual past (گذشتهٔ پایا) forms with می‌ prefix.
    pub fn imperfective_past(&self, ri: &str) -> Vec<String> {
        mi(past_suffixes(ri)).into()
    }

    /// Returns negative habitual past forms with نمی‌ prefix.
    pub fn negative_imperfective_past(&self, ri: &str) -> Vec<String> {
        nami(past_suffixes(ri)).into()
    }

    /// Returns passive habitual past forms for the given past root.
    pub fn passive_imperfective_past(&self, ri: &str) -> Vec<String> {
        mi(past_suffixes("شد"))
            .map(|s| passive_suffix(ri, &s))
            .into()
    }

    /// Returns negative passive habitual past forms for the given past root.
    pub fn negative_passive_imperfective_past(&self, ri: &str) -> Vec<String> {
        nami(past_suffixes("شد"))
            .map(|s| passive_suffix(ri, &s))
            .into()
    }

    // -----------------------------------------------------------------------
    // گذشتهٔ استمراری  (past progressive)
    // -----------------------------------------------------------------------

    /// Returns past progressive (گذشتهٔ استمراری) forms using داشت auxiliary.
    pub fn past_progressive(&self, ri: &str) -> Vec<String> {
        zip_join(past_suffixes("داشت"), mi(past_suffixes(ri))).into()
    }

    /// Returns passive past progressive forms for the given past root.
    pub fn passive_past_progressive(&self, ri: &str) -> Vec<String> {
        zip_join(
            past_suffixes("داشت"),
            mi(past_suffixes("شد")).map(|s| passive_suffix(ri, &s)),
        )
        .into()
    }

    // -----------------------------------------------------------------------
    // حال کامل  (present perfect)
    // -----------------------------------------------------------------------

    /// Returns present perfect (حال کامل) forms for the given past root.
    pub fn present_perfect(&self, ri: &str) -> Vec<String> {
        ["ه‌ام", "ه‌ای", "ه است", "ه", "ه‌ایم", "ه‌اید", "ه‌اند"]
            .map(|s| format!("{ri}{s}"))
            .into()
    }

    /// Returns negative present perfect forms for the given past root.
    pub fn negative_present_perfect(&self, ri: &str) -> Vec<String> {
        self.present_perfect(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns subjunctive present perfect forms for the given past root.
    pub fn subjunctive_present_perfect(&self, ri: &str) -> Vec<String> {
        present_suffixes("باش")
            .map(|s| passive_suffix(ri, &s))
            .into()
    }

    /// Returns negative subjunctive present perfect forms for the given past root.
    pub fn negative_subjunctive_present_perfect(&self, ri: &str) -> Vec<String> {
        self.subjunctive_present_perfect(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns grammatical (imperative-style) present perfect forms for the given past root.
    pub fn grammatical_present_perfect(&self, ri: &str) -> Vec<String> {
        present_suffixes("باش")
            .map(|s| {
                let after = if s == "باشی" { "باش".to_string() } else { s };
                passive_suffix(ri, &after)
            })
            .into()
    }

    /// Returns negative grammatical present perfect forms for the given past root.
    pub fn negative_grammatical_present_perfect(&self, ri: &str) -> Vec<String> {
        self.grammatical_present_perfect(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns passive present perfect forms for the given past root.
    pub fn passive_present_perfect(&self, ri: &str) -> Vec<String> {
        self.present_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive present perfect forms for the given past root.
    pub fn negative_passive_present_perfect(&self, ri: &str) -> Vec<String> {
        self.negative_present_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns passive subjunctive present perfect forms for the given past root.
    pub fn passive_subjunctive_present_perfect(&self, ri: &str) -> Vec<String> {
        self.subjunctive_present_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive subjunctive present perfect forms for the given past root.
    pub fn negative_passive_subjunctive_present_perfect(&self, ri: &str) -> Vec<String> {
        self.negative_subjunctive_present_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns passive grammatical present perfect forms for the given past root.
    pub fn passive_grammatical_present_perfect(&self, ri: &str) -> Vec<String> {
        present_suffixes("باش")
            .map(|s| {
                let after = if s == "باشی" { "باش".to_string() } else { s };
                format!("{ri}ه شده {after}")
            })
            .into()
    }

    /// Returns negative passive grammatical present perfect forms for the given past root.
    pub fn negative_passive_grammatical_present_perfect(&self, ri: &str) -> Vec<String> {
        present_suffixes("باش")
            .map(|s| {
                let after = if s == "باشی" { "باش".to_string() } else { s };
                format!("{ri}ه نشده {after}")
            })
            .into()
    }

    // -----------------------------------------------------------------------
    // حال کامل پایا  (present perfect imperfective)
    // -----------------------------------------------------------------------

    /// Returns imperfective present perfect (حال کامل پایا) forms with می‌ prefix.
    pub fn imperfective_present_perfect(&self, ri: &str) -> Vec<String> {
        self.present_perfect(ri)
            .into_iter()
            .map(|f| format!("می‌{f}"))
            .collect()
    }

    /// Returns negative imperfective present perfect forms with نمی‌ prefix.
    pub fn negative_imperfective_present_perfect(&self, ri: &str) -> Vec<String> {
        self.imperfective_present_perfect(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns subjunctive imperfective present perfect forms for the given past root.
    pub fn subjunctive_imperfective_present_perfect(&self, ri: &str) -> Vec<String> {
        self.subjunctive_present_perfect(ri)
            .into_iter()
            .map(|f| format!("می‌{f}"))
            .collect()
    }

    /// Returns negative subjunctive imperfective present perfect forms for the given past root.
    pub fn negative_subjunctive_imperfective_present_perfect(&self, ri: &str) -> Vec<String> {
        self.subjunctive_imperfective_present_perfect(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns passive imperfective present perfect forms for the given past root.
    pub fn passive_imperfective_present_perfect(&self, ri: &str) -> Vec<String> {
        self.imperfective_present_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive imperfective present perfect forms for the given past root.
    pub fn negative_passive_imperfective_present_perfect(&self, ri: &str) -> Vec<String> {
        self.negative_imperfective_present_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns passive subjunctive imperfective present perfect forms for the given past root.
    pub fn passive_subjunctive_imperfective_present_perfect(&self, ri: &str) -> Vec<String> {
        self.subjunctive_imperfective_present_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive subjunctive imperfective present perfect forms for the given past root.
    pub fn negative_passive_subjunctive_imperfective_present_perfect(
        &self,
        ri: &str,
    ) -> Vec<String> {
        self.negative_subjunctive_imperfective_present_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    // -----------------------------------------------------------------------
    // حال کامل استمراری  (present perfect progressive)
    // -----------------------------------------------------------------------

    /// Returns present perfect progressive (حال کامل استمراری) forms using داشت auxiliary.
    pub fn present_perfect_progressive(&self, ri: &str) -> Vec<String> {
        self.present_perfect("داشت")
            .into_iter()
            .zip(self.imperfective_present_perfect(ri))
            .map(|(a, b)| format!("{a} {b}"))
            .collect()
    }

    /// Returns passive present perfect progressive forms for the given past root.
    pub fn passive_present_perfect_progressive(&self, ri: &str) -> Vec<String> {
        self.present_perfect("داشت")
            .into_iter()
            .zip(self.passive_imperfective_present_perfect(ri))
            .map(|(a, b)| format!("{a} {b}"))
            .collect()
    }

    // -----------------------------------------------------------------------
    // گذشتهٔ پیشین  (past perfect / pluperfect)
    // -----------------------------------------------------------------------

    /// Returns pluperfect (گذشتهٔ پیشین) forms using بود auxiliary.
    pub fn past_precedent(&self, ri: &str) -> Vec<String> {
        past_suffixes("بود")
            .map(|s| passive_suffix(ri, &s))
            .into()
    }

    /// Returns negative pluperfect forms for the given past root.
    pub fn negative_past_precedent(&self, ri: &str) -> Vec<String> {
        self.past_precedent(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns passive pluperfect forms for the given past root.
    pub fn passive_past_precedent(&self, ri: &str) -> Vec<String> {
        self.past_precedent("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive pluperfect forms for the given past root.
    pub fn negative_passive_past_precedent(&self, ri: &str) -> Vec<String> {
        self.negative_past_precedent("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns imperfective pluperfect forms with می‌ prefix.
    pub fn imperfective_past_precedent(&self, ri: &str) -> Vec<String> {
        self.past_precedent(ri)
            .into_iter()
            .map(|f| format!("می‌{f}"))
            .collect()
    }

    /// Returns negative imperfective pluperfect forms with نمی‌ prefix.
    pub fn negative_imperfective_past_precedent(&self, ri: &str) -> Vec<String> {
        self.imperfective_past_precedent(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns passive imperfective pluperfect forms for the given past root.
    pub fn passive_imperfective_past_precedent(&self, ri: &str) -> Vec<String> {
        self.imperfective_past_precedent("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive imperfective pluperfect forms for the given past root.
    pub fn negative_passive_imperfective_past_precedent(&self, ri: &str) -> Vec<String> {
        self.negative_imperfective_past_precedent("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns past precedent progressive forms using داشت auxiliary.
    pub fn past_precedent_progressive(&self, ri: &str) -> Vec<String> {
        past_suffixes("داشت")
            .into_iter()
            .zip(self.imperfective_past_precedent(ri))
            .map(|(a, b)| format!("{a} {b}"))
            .collect()
    }

    /// Returns passive past precedent progressive forms for the given past root.
    pub fn passive_past_precedent_progressive(&self, ri: &str) -> Vec<String> {
        past_suffixes("داشت")
            .into_iter()
            .zip(self.passive_imperfective_past_precedent(ri))
            .map(|(a, b)| format!("{a} {b}"))
            .collect()
    }

    // -----------------------------------------------------------------------
    // گذشتهٔ پیشین کامل  (past perfect perfective)
    // -----------------------------------------------------------------------

    /// Returns past precedent perfect (گذشتهٔ پیشین کامل) forms for the given past root.
    pub fn past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.present_perfect("بود")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative past precedent perfect forms for the given past root.
    pub fn negative_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.past_precedent_perfect(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns subjunctive past precedent perfect forms for the given past root.
    pub fn subjunctive_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.subjunctive_present_perfect("بود")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative subjunctive past precedent perfect forms for the given past root.
    pub fn negative_subjunctive_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.subjunctive_past_precedent_perfect(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns grammatical past precedent perfect forms for the given past root.
    pub fn grammatical_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        present_suffixes("باش")
            .map(|s| {
                let after = if s == "باشی" { "باش".to_string() } else { s };
                format!("{ri}ه بوده {after}")
            })
            .into()
    }

    /// Returns negative grammatical past precedent perfect forms for the given past root.
    pub fn negative_grammatical_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.grammatical_past_precedent_perfect(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns passive past precedent perfect forms for the given past root.
    pub fn passive_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.past_precedent_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive past precedent perfect forms for the given past root.
    pub fn negative_passive_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.negative_past_precedent_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns passive subjunctive past precedent perfect forms for the given past root.
    pub fn passive_subjunctive_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.subjunctive_past_precedent_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive subjunctive past precedent perfect forms for the given past root.
    pub fn negative_passive_subjunctive_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        // Note: Python prepends ن after the passive prefix, not before
        self.subjunctive_past_precedent_perfect("شد")
            .into_iter()
            .map(|s| format!("{ri}ه ن{s}"))
            .collect()
    }

    /// Returns passive grammatical past precedent perfect forms for the given past root.
    pub fn passive_grammatical_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.grammatical_past_precedent_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive grammatical past precedent perfect forms for the given past root.
    pub fn negative_passive_grammatical_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.negative_grammatical_past_precedent_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    // -----------------------------------------------------------------------
    // گذشتهٔ پیشین کامل پایا  (past perfect imperfective)
    // -----------------------------------------------------------------------

    /// Returns imperfective past precedent perfect (گذشتهٔ پیشین کامل پایا) forms with می‌ prefix.
    pub fn imperfective_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.past_precedent_perfect(ri)
            .into_iter()
            .map(|f| format!("می‌{f}"))
            .collect()
    }

    /// Returns negative imperfective past precedent perfect forms with نمی‌ prefix.
    pub fn negative_imperfective_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.imperfective_past_precedent_perfect(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns subjunctive imperfective past precedent perfect forms for the given past root.
    pub fn subjunctive_imperfective_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.subjunctive_past_precedent_perfect(ri)
            .into_iter()
            .map(|f| format!("می‌{f}"))
            .collect()
    }

    /// Returns negative subjunctive imperfective past precedent perfect forms for the given past root.
    pub fn negative_subjunctive_imperfective_past_precedent_perfect(
        &self,
        ri: &str,
    ) -> Vec<String> {
        self.subjunctive_imperfective_past_precedent_perfect(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns passive imperfective past precedent perfect forms for the given past root.
    pub fn passive_imperfective_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.imperfective_past_precedent_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive imperfective past precedent perfect forms for the given past root.
    pub fn negative_passive_imperfective_past_precedent_perfect(&self, ri: &str) -> Vec<String> {
        self.negative_imperfective_past_precedent_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns passive subjunctive imperfective past precedent perfect forms for the given past root.
    pub fn passive_subjunctive_imperfective_past_precedent_perfect(
        &self,
        ri: &str,
    ) -> Vec<String> {
        self.subjunctive_imperfective_past_precedent_perfect("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive subjunctive imperfective past precedent perfect forms for the given past root.
    pub fn negative_passive_subjunctive_imperfective_past_precedent_perfect(
        &self,
        ri: &str,
    ) -> Vec<String> {
        self.subjunctive_imperfective_past_precedent_perfect("شد")
            .into_iter()
            .map(|s| format!("{ri}ه ن{s}"))
            .collect()
    }

    /// Returns past precedent perfect progressive forms using داشت auxiliary.
    pub fn past_precedent_perfect_progressive(&self, ri: &str) -> Vec<String> {
        self.present_perfect("داشت")
            .into_iter()
            .zip(self.imperfective_past_precedent_perfect(ri))
            .map(|(a, b)| format!("{a} {b}"))
            .collect()
    }

    /// Returns passive past precedent perfect progressive forms for the given past root.
    pub fn passive_past_precedent_perfect_progressive(&self, ri: &str) -> Vec<String> {
        self.present_perfect("داشت")
            .into_iter()
            .zip(self.passive_imperfective_past_precedent_perfect(ri))
            .map(|(a, b)| format!("{a} {b}"))
            .collect()
    }

    // -----------------------------------------------------------------------
    // حال مطلق  (simple present / subjunctive)
    // -----------------------------------------------------------------------

    /// Returns simple present (حال مطلق) forms for the given present root.
    pub fn perfective_present(&self, rii: &str) -> Vec<String> {
        present_suffixes(rii).into()
    }

    /// Returns negative simple present forms for the given present root.
    pub fn negative_perfective_present(&self, rii: &str) -> Vec<String> {
        neg(present_suffixes(rii)).into()
    }

    /// Returns subjunctive present forms with ب prefix for the given present root.
    pub fn subjunctive_perfective_present(&self, rii: &str) -> Vec<String> {
        present_suffixes(rii)
            .map(|f| format!("ب{f}"))
            .into()
    }

    /// Returns negative subjunctive present forms for the given present root.
    pub fn negative_subjunctive_perfective_present(&self, rii: &str) -> Vec<String> {
        neg(present_suffixes(rii)).into()
    }

    /// Returns imperative (grammatical) present forms for the given present root.
    pub fn grammatical_perfective_present(&self, rii: &str) -> Vec<String> {
        self.subjunctive_perfective_present(rii)
            .into_iter()
            .map(|f| {
                // ببینی → ببین  (imperative 2nd sg strips ی)
                if f.ends_with("ی") {
                    let stem: String = f.chars().take(f.chars().count() - 1).collect();
                    stem
                } else {
                    f
                }
            })
            .collect()
    }

    /// Returns negative imperative (grammatical) present forms for the given present root.
    pub fn negative_grammatical_perfective_present(&self, rii: &str) -> Vec<String> {
        present_suffixes(rii)
            .map(|f| {
                if f.ends_with("ی") {
                    let stem: String = f.chars().take(f.chars().count() - 1).collect();
                    format!("ن{stem}")
                } else {
                    format!("ن{f}")
                }
            })
            .into()
    }

    /// Returns passive simple present forms for the given past root.
    pub fn passive_perfective_present(&self, ri: &str) -> Vec<String> {
        present_suffixes("شو")
            .map(|s| passive_suffix(ri, &s))
            .into()
    }

    /// Returns negative passive simple present forms for the given past root.
    pub fn negative_passive_perfective_present(&self, ri: &str) -> Vec<String> {
        neg(present_suffixes("شو"))
            .map(|s| passive_suffix(ri, &s))
            .into()
    }

    /// Returns passive subjunctive present forms for the given past root.
    pub fn passive_subjunctive_perfective_present(&self, ri: &str) -> Vec<String> {
        self.subjunctive_perfective_present("شو")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive subjunctive present forms for the given past root.
    pub fn negative_passive_subjunctive_perfective_present(&self, ri: &str) -> Vec<String> {
        self.negative_subjunctive_perfective_present("شو")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns passive grammatical (imperative) present forms for the given past root.
    pub fn passive_grammatical_perfective_present(&self, ri: &str) -> Vec<String> {
        self.grammatical_perfective_present("شو")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive grammatical present forms for the given past root.
    pub fn negative_passive_grammatical_perfective_present(&self, ri: &str) -> Vec<String> {
        present_suffixes("شو")
            .map(|s| {
                let after = if s.ends_with("ی") {
                    let stem: String = s.chars().take(s.chars().count() - 1).collect();
                    format!("ن{stem}")
                } else {
                    format!("ن{s}")
                };
                passive_suffix(ri, &after)
            })
            .into()
    }

    // -----------------------------------------------------------------------
    // حال پایا  (present imperfective / present habitual)
    // -----------------------------------------------------------------------

    /// Returns present habitual (حال پایا) forms with می‌ prefix for the given present root.
    pub fn imperfective_present(&self, rii: &str) -> Vec<String> {
        present_suffixes(rii)
            .map(|f| format!("می‌{f}"))
            .into()
    }

    /// Returns negative present habitual forms with نمی‌ prefix for the given present root.
    pub fn negative_imperfective_present(&self, rii: &str) -> Vec<String> {
        self.imperfective_present(rii)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns passive present habitual forms for the given past root.
    pub fn passive_imperfective_present(&self, ri: &str) -> Vec<String> {
        self.imperfective_present("شو")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive present habitual forms for the given past root.
    pub fn negative_passive_imperfective_present(&self, ri: &str) -> Vec<String> {
        self.negative_imperfective_present("شو")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    // -----------------------------------------------------------------------
    // حال استمراری  (present progressive)
    // -----------------------------------------------------------------------

    /// Returns present progressive (حال استمراری) forms using دار auxiliary.
    pub fn present_progressive(&self, rii: &str) -> Vec<String> {
        present_suffixes("دار")
            .into_iter()
            .zip(self.imperfective_present(rii))
            .map(|(a, b)| format!("{a} {b}"))
            .collect()
    }

    /// Returns passive present progressive forms for the given past root.
    pub fn passive_present_progressive(&self, ri: &str) -> Vec<String> {
        present_suffixes("دار")
            .into_iter()
            .zip(self.passive_imperfective_present(ri))
            .map(|(a, b)| format!("{a} {b}"))
            .collect()
    }

    // -----------------------------------------------------------------------
    // آیندهٔ مطلق  (simple future)
    // -----------------------------------------------------------------------

    /// Returns simple future (آیندهٔ مطلق) forms using خواه auxiliary.
    pub fn perfective_future(&self, ri: &str) -> Vec<String> {
        present_suffixes("خواه")
            .map(|s| format!("{s} {ri}"))
            .into()
    }

    /// Returns negative simple future forms for the given past root.
    pub fn negative_perfective_future(&self, ri: &str) -> Vec<String> {
        self.perfective_future(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns passive simple future forms for the given past root.
    pub fn passive_perfective_future(&self, ri: &str) -> Vec<String> {
        self.perfective_future("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive simple future forms for the given past root.
    pub fn negative_passive_perfective_future(&self, ri: &str) -> Vec<String> {
        self.negative_perfective_future("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    // -----------------------------------------------------------------------
    // آیندهٔ پایا  (future imperfective)
    // -----------------------------------------------------------------------

    /// Returns future imperfective (آیندهٔ پایا) forms with می‌ prefix.
    pub fn imperfective_future(&self, ri: &str) -> Vec<String> {
        self.perfective_future(ri)
            .into_iter()
            .map(|f| format!("می‌{f}"))
            .collect()
    }

    /// Returns negative future imperfective forms with نمی‌ prefix.
    pub fn negative_imperfective_future(&self, ri: &str) -> Vec<String> {
        self.imperfective_future(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns passive future imperfective forms for the given past root.
    pub fn passive_imperfective_future(&self, ri: &str) -> Vec<String> {
        self.imperfective_future("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive future imperfective forms for the given past root.
    pub fn negative_passive_imperfective_future(&self, ri: &str) -> Vec<String> {
        self.negative_imperfective_future("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    // -----------------------------------------------------------------------
    // آیندهٔ پیشین  (future perfect)
    // -----------------------------------------------------------------------

    /// Returns future perfect (آیندهٔ پیشین) forms for the given past root.
    pub fn future_precedent(&self, ri: &str) -> Vec<String> {
        self.perfective_future("بود")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative future perfect forms for the given past root.
    pub fn negative_future_precedent(&self, ri: &str) -> Vec<String> {
        self.future_precedent(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns passive future perfect forms for the given past root.
    pub fn passive_future_precedent(&self, ri: &str) -> Vec<String> {
        self.future_precedent("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive future perfect forms for the given past root.
    pub fn negative_passive_future_precedent(&self, ri: &str) -> Vec<String> {
        self.negative_future_precedent("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    // -----------------------------------------------------------------------
    // آیندهٔ پیشین پایا  (future perfect imperfective)
    // -----------------------------------------------------------------------

    /// Returns future perfect imperfective (آیندهٔ پیشین پایا) forms with می‌ prefix.
    pub fn future_precedent_imperfective(&self, ri: &str) -> Vec<String> {
        self.future_precedent(ri)
            .into_iter()
            .map(|f| format!("می‌{f}"))
            .collect()
    }

    /// Returns negative future perfect imperfective forms with نمی‌ prefix.
    pub fn negative_future_precedent_imperfective(&self, ri: &str) -> Vec<String> {
        self.future_precedent_imperfective(ri)
            .into_iter()
            .map(|f| format!("ن{f}"))
            .collect()
    }

    /// Returns passive future perfect imperfective forms for the given past root.
    pub fn passive_future_precedent_imperfective(&self, ri: &str) -> Vec<String> {
        self.future_precedent_imperfective("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    /// Returns negative passive future perfect imperfective forms for the given past root.
    pub fn negative_passive_future_precedent_imperfective(&self, ri: &str) -> Vec<String> {
        self.negative_future_precedent_imperfective("شد")
            .into_iter()
            .map(|s| passive_suffix(ri, &s))
            .collect()
    }

    // -----------------------------------------------------------------------
    // get_all — full conjugation table for a `past#present` verb entry
    // -----------------------------------------------------------------------

    /// Generates every conjugated form for a `"past#present"` verb string.
    ///
    /// # Panics
    ///
    /// Panics if `verb` does not contain `#`.  Use `Lemmatizer` for safe access.
    pub fn get_all(&self, verb: &str) -> Vec<String> {
        let (ri, rii) = verb.split_once('#').expect("verb must be 'past#present'");
        let mut all: Vec<String> = Vec::new();

        // infinitive
        all.push(format!("{ri}ن"));

        macro_rules! add {
            ($($method:ident($arg:expr)),+ $(,)?) => {
                $(all.extend(self.$method($arg));)+
            };
        }

        add!(
            perfective_past(ri),
            negative_perfective_past(ri),
            passive_perfective_past(ri),
            negative_passive_perfective_past(ri),
            imperfective_past(ri),
            negative_imperfective_past(ri),
            passive_imperfective_past(ri),
            negative_passive_imperfective_past(ri),
            past_progressive(ri),
            passive_past_progressive(ri),
            present_perfect(ri),
            negative_present_perfect(ri),
            subjunctive_present_perfect(ri),
            negative_subjunctive_present_perfect(ri),
            grammatical_present_perfect(ri),
            negative_grammatical_present_perfect(ri),
            passive_present_perfect(ri),
            negative_passive_present_perfect(ri),
            passive_subjunctive_present_perfect(ri),
            negative_passive_subjunctive_present_perfect(ri),
            passive_grammatical_present_perfect(ri),
            negative_passive_grammatical_present_perfect(ri),
            imperfective_present_perfect(ri),
            negative_imperfective_present_perfect(ri),
            subjunctive_imperfective_present_perfect(ri),
            negative_subjunctive_imperfective_present_perfect(ri),
            passive_imperfective_present_perfect(ri),
            negative_passive_imperfective_present_perfect(ri),
            passive_subjunctive_imperfective_present_perfect(ri),
            negative_passive_subjunctive_imperfective_present_perfect(ri),
            present_perfect_progressive(ri),
            passive_present_perfect_progressive(ri),
            past_precedent(ri),
            negative_past_precedent(ri),
            passive_past_precedent(ri),
            negative_passive_past_precedent(ri),
            imperfective_past_precedent(ri),
            negative_imperfective_past_precedent(ri),
            passive_imperfective_past_precedent(ri),
            negative_passive_imperfective_past_precedent(ri),
            past_precedent_progressive(ri),
            passive_past_precedent_progressive(ri),
            past_precedent_perfect(ri),
            negative_past_precedent_perfect(ri),
            subjunctive_past_precedent_perfect(ri),
            negative_subjunctive_past_precedent_perfect(ri),
            grammatical_past_precedent_perfect(ri),
            negative_grammatical_past_precedent_perfect(ri),
            passive_past_precedent_perfect(ri),
            negative_passive_past_precedent_perfect(ri),
            passive_subjunctive_past_precedent_perfect(ri),
            negative_passive_subjunctive_past_precedent_perfect(ri),
            passive_grammatical_past_precedent_perfect(ri),
            negative_passive_grammatical_past_precedent_perfect(ri),
            imperfective_past_precedent_perfect(ri),
            negative_imperfective_past_precedent_perfect(ri),
            subjunctive_imperfective_past_precedent_perfect(ri),
            negative_subjunctive_imperfective_past_precedent_perfect(ri),
            passive_imperfective_past_precedent_perfect(ri),
            negative_passive_imperfective_past_precedent_perfect(ri),
            passive_subjunctive_imperfective_past_precedent_perfect(ri),
            negative_passive_subjunctive_imperfective_past_precedent_perfect(ri),
            past_precedent_perfect_progressive(ri),
            passive_past_precedent_perfect_progressive(ri),
            perfective_present(rii),
            negative_perfective_present(rii),
            subjunctive_perfective_present(rii),
            negative_subjunctive_perfective_present(rii),
            grammatical_perfective_present(rii),
            negative_grammatical_perfective_present(rii),
            passive_perfective_present(ri),
            negative_passive_perfective_present(ri),
            passive_subjunctive_perfective_present(ri),
            negative_passive_subjunctive_perfective_present(ri),
            passive_grammatical_perfective_present(ri),
            negative_passive_grammatical_perfective_present(ri),
            imperfective_present(rii),
            negative_imperfective_present(rii),
            passive_imperfective_present(ri),
            negative_passive_imperfective_present(ri),
            present_progressive(rii),
            passive_present_progressive(ri),
            perfective_future(ri),
            negative_perfective_future(ri),
            passive_perfective_future(ri),
            negative_passive_perfective_future(ri),
            imperfective_future(ri),
            negative_imperfective_future(ri),
            passive_imperfective_future(ri),
            negative_passive_imperfective_future(ri),
            future_precedent(ri),
            negative_future_precedent(ri),
            passive_future_precedent(ri),
            negative_passive_future_precedent(ri),
            future_precedent_imperfective(ri),
            negative_future_precedent_imperfective(ri),
            passive_future_precedent_imperfective(ri),
            negative_passive_future_precedent_imperfective(ri),
        );

        all
    }
}
