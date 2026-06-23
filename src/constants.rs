/// Source characters for Unicode normalization (maps variant forms to standard Persian).
pub const TRANSLATION_SRC: &str = "ؠٸؽؾؿكيٮٯٷٸٹٺٻټٽٿڀځٵٶٷٸٹٺٻټٽٿڀځڂڅڇڈډڊڋڌڍڎڏڐڑڒړڔڕږڗڙښڛڜڝڞڟڠڡڢڣڤڥڦڧڨڪګڬڭڮڰڱڲڳڴڵڶڷڸڹںڻڼڽھڿہۂۃۄۅۆۇۈۉۊۋۏۍێېۑےۓەۮۯۺۻۼۿݐݑݒݓݔݕݖݗݘݙݚݛݜݝݞݟݠݡݢݣݤݥݦݧݨݩݪݫݬݭݮݯݰݱݲݳݴݵݶݷݸݹݺݻݼݽݾݿࢠࢡࢢࢣࢤࢥࢦࢧࢨࢩࢪࢫࢮࢯࢰࢱࢬࢲࢳࢴࢶࢷࢸࢹࢺࢻࢼࢽﭐﭑﭒﭓﭔﭕﭖﭗﭘﭙﭚﭛﭜﭝﭞﭟﭠﭡﭢﭣﭤﭥﭦﭧﭨﭩﭮﭯﭰﭱﭲﭳﭴﭵﭶﭷﭸﭹﭺﭻﭼﭽﭾﭿﮀﮁﮂﮃﮄﮅﮆﮇﮈﮉﮊﮋﮌﮍﮎﮏﮐﮑﮒﮓﮔﮕﮖﮗﮘﮙﮚﮛﮜﮝﮞﮟﮠﮡﮢﮣﮤﮥﮦﮧﮨﮩﮪﮫﮬﮭﮮﮯﮰﮱﺀﺁﺃﺄﺅﺆﺇﺈﺉﺊﺋﺌﺍﺎﺏﺐﺑﺒﺕﺖﺗﺘﺙﺚﺛﺜﺝﺞﺟﺠﺡﺢﺣﺤﺥﺦﺧﺨﺩﺪﺫﺬﺭﺮﺯﺰﺱﺲﺳﺴﺵﺶﺷﺸﺹﺺﺻﺼﺽﺾﺿﻀﻁﻂﻃﻄﻅﻆﻇﻈﻉﻊﻋﻌﻍﻎﻏﻐﻑﻒﻓﻔﻕﻖﻗﻘﻙﻚﻛﻜﻝﻞﻟﻠﻡﻢﻣﻤﻥﻦﻧﻨﻩﻪﻫﻬﻭﻮﻯﻰﻱﻲﻳﻴىكيﺂﯽﯼ\u{201C}\u{201D}";

/// Destination characters for the Unicode normalization table.
pub const TRANSLATION_DST: &str = "یككیییكیبقویتتبتتتبحاوویتتبتتتبحححچدددددددددررررررررسسسصصطعففففففققکككككگگگگگللللنننننهچهههوووووووووییییییهدرشضغهبببببببححددرسعععففکككممنننلررسححسرحاایییووییحسسکببجطفقلمییرودصگویزعکبپتریفقنااببببپپپپببببتتتتتتتتتتتتففففححححححححچچچچچچچچددددددددژژررككككگگگگگگگگگگگگننننننههههههههههییییءاااووااییییااببببتتتتثثثثججججححححخخخخددذذررززسسسسششششصصصصضضضضططططظظظظععععغغغغففففققققككككللللممممننننههههوویییییییكیایی\"\"";

/// Source characters for digit conversion (ASCII/Arabic → Persian).
pub const NUMBERS_SRC: &str = "0123456789%٠١٢٣٤٥٦٧٨٩";

/// Persian equivalents used to replace NUMBERS_SRC characters.
pub const NUMBERS_DST: &str = "۰۱۲۳۴۵۶۷۸۹٪۰۱۲۳۴۵۶۷۸۹";

// ---------------------------------------------------------------------------
// Persian letter set (used inside character classes in regexes)
// ---------------------------------------------------------------------------

/// All standard Persian/Arabic letters, used to build character class patterns.
pub const PERSIAN_LETTERS: &str = "آابپتثجچحخدذرزژسشصضطظعغفقکگلمنوهی";

// ---------------------------------------------------------------------------
// Suffix set — sorted longest-first in the Stemmer at runtime
// ---------------------------------------------------------------------------

/// Inflectional and derivational suffixes stripped by the rule-based stemmer.
pub const SUFFIXES: &[&str] = &[
    "هایمان", "هایتان", "هایشان",
    "هایی", "های",
    "ترین", "تری", "تر",
    "گری", "گر",
    "مان", "تان", "شان",
    "ها", "ای", "ان", "ین",
    "ام", "ات", "اش",
    "یم", "ید", "ند",
    "م", "ت", "ش", "ی",
    // extra endings handled by Stemmer
    "\u{0654}", // ٔ (hamza above)
    "\u{200C}ا",  // ZWNJ + alef
    "\u{200C}",   // bare ZWNJ
];

// ---------------------------------------------------------------------------
// Unicode replacements (ligatures → words)
// ---------------------------------------------------------------------------

/// Pairs of (ligature, replacement text) for common Arabic/Persian Unicode ligatures.
pub const UNICODE_REPLACEMENTS: &[(&str, &str)] = &[
    ("\u{FDFD}", "بسم الله الرحمن الرحیم"), // ﷽
    ("\u{FDFC}", "ریال"),                    // ﷼
    ("\u{FDF0}", "صلی"),                     // ﷰ
    ("\u{FDF9}", "صلی"),                     // ﷹ
    ("\u{FDF2}", "الله"),                    // ﷲ
    ("\u{FDF3}", "اکبر"),                    // ﷳ
    ("\u{FDF4}", "محمد"),                    // ﷴ
    ("\u{FDF5}", "صلعم"),                    // ﷵ
    ("\u{FDF6}", "رسول"),                    // ﷶ
    ("\u{FDF7}", "علیه"),                    // ﷷ
    ("\u{FDF8}", "وسلم"),                    // ﷸ
    // lam-alef ligatures
    ("\u{FEF5}", "لا"), ("\u{FEF6}", "لا"), ("\u{FEF7}", "لا"),
    ("\u{FEF8}", "لا"), ("\u{FEF9}", "لا"), ("\u{FEFA}", "لا"),
    ("\u{FEFB}", "لا"), ("\u{FEFC}", "لا"),
];

// ---------------------------------------------------------------------------
// Regex pattern strings
// ---------------------------------------------------------------------------

// Punctuation character classes used in several patterns below.
// PUNC_AFTER: chars that appear after content (close-side punctuation)
const _PUNC_AFTER: &str = r"\.:!،؛؟»\]\)\}";
// PUNC_BEFORE: chars that appear before content (open-side punctuation)
const _PUNC_BEFORE: &str = r"«\[\(\{";

/// `(pattern, replacement)` pairs for removing redundant whitespace and ZWNJ.
pub const EXTRA_SPACE_PATTERNS: &[(&str, &str)] = &[
    (r"^ +| +$", ""),
    (r" {2,}", " "),
    (r"\n{3,}", "\n\n"),
    ("\u{200C}{2,}", "\u{200C}"),
    ("\u{200C}{1,} ", " "),
    (" \u{200C}{1,}", " "),
    (r"\b\u{200C}*\B", ""),
    (r"\B\u{200C}*\b", ""),
    (r"[ـ\r]", ""),
];

/// `(pattern, replacement)` pairs for normalizing spacing around punctuation.
pub const PUNCTUATION_SPACING_PATTERNS: &[(&str, &str)] = &[
    (r#"" ([^\n"]+) ""#, r#""$1""#),
    (r" ([\.:!،؛؟»\]\)\}])", "$1"),
    (r"([«\[\(\{]) ", "$1"),
    (r"([\.:])([^ \.:!،؛؟»\]\)\}\d۰-۹])", "$1 $2"),
    (r"([!،؛؟»\]\)\}])([^ \.:!،؛؟»\]\)\}])", "$1 $2"),
    (r"([^ «\[\(\{])([«\[\(\{])", "$1 $2"),
    (r"(\d)([آابپتثجچحخدذرزژسشصضطظعغفقکگلمنوهی])", "$1 $2"),
    (r"([آابپتثجچحخدذرزژسشصضطظعغفقکگلمنوهی])(\d)", "$1 $2"),
];

/// `(pattern, replacement)` pairs for attaching prefixes/suffixes with ZWNJ.
pub const AFFIX_SPACING_PATTERNS: &[(&str, &str)] = &[
    // fix trailing ی that should be attached via ZWNJ
    (r"([^ ]ه) ی ", "$1\u{200C}ی "),
    // attach می/نمی prefix
    (r"(^| )(ن?می) ", "$1$2\u{200C}"),
    // attach comparative/superlative and plural suffixes
    (
        r"(?<=[^\n\d \.:!،؛؟»\]\)\}«\[\(\{]{2}) (تر(ین?)?|گری?|های?)(?=[ \n\.:!،؛؟»\]\)\}«\[\(\{]|$)",
        "\u{200C}$1",
    ),
    // attach clitic pronouns to words ending in ه
    (
        r"([^ ]ه) (ا(م|یم|ش|ند|ی|ید|ت))(?=[ \n\.:!،؛؟»\]\)\}]|$)",
        "$1\u{200C}$2",
    ),
    // ه + ها  →  ه‌ها
    (r"(ه)(ها)", "$1\u{200C}$2"),
];

/// `(pattern, replacement)` pairs for Persian typographic style.
pub const PERSIAN_STYLE_PATTERNS: &[(&str, &str)] = &[
    (r#""([^\n"]+)""#, "«$1»"),
    (r"([\d+])\.([\d+])", "$1٫$2"),
    (r" ?\.\.\.", " …"),
];

/// Regex pattern that matches Arabic diacritical marks to be removed.
pub const DIACRITICS_PATTERN: &str =
    r"[\u{064B}\u{064C}\u{064D}\u{064E}\u{064F}\u{0650}\u{0651}\u{0652}]";

/// Regex pattern matching rare/decorative Unicode code points to be stripped.
pub const SPECIAL_CHARS_PATTERN: &str = concat!(
    r"[\u{0605}\u{0653}\u{0654}\u{0655}\u{0656}\u{0657}\u{0658}",
    r"\u{0659}\u{065A}\u{065B}\u{065C}\u{065D}\u{065E}\u{065F}",
    r"\u{0670}\u{0610}\u{0611}\u{0612}\u{0613}\u{0614}\u{0615}",
    r"\u{0616}\u{0618}\u{0619}\u{061A}\u{061E}\u{06D4}\u{06D6}",
    r"\u{06D7}\u{06D8}\u{06D9}\u{06DA}\u{06DB}\u{06DC}\u{06DD}",
    r"\u{06DE}\u{06DF}\u{06E0}\u{06E1}\u{06E2}\u{06E3}\u{06E4}",
    r"\u{06E5}\u{06E6}\u{06E7}\u{06E8}\u{06E9}\u{06EA}\u{06EB}",
    r"\u{06EC}\u{06ED}\u{06FD}\u{06FE}\u{08AD}",
    r"\u{08D4}\u{08D5}\u{08D6}\u{08D7}\u{08D8}\u{08D9}\u{08DA}",
    r"\u{08DB}\u{08DC}\u{08DD}\u{08DE}\u{08DF}\u{08E0}\u{08E1}",
    r"\u{08E2}\u{08E3}\u{08E4}\u{08E5}\u{08E6}\u{08E7}\u{08E8}",
    r"\u{08E9}\u{08EA}\u{08EB}\u{08EC}\u{08ED}\u{08EE}\u{08EF}",
    r"\u{08F0}\u{08F1}\u{08F2}\u{08F3}\u{08F4}\u{08F5}\u{08F6}",
    r"\u{08F7}\u{08F8}\u{08F9}\u{08FA}\u{08FB}\u{08FC}\u{08FD}",
    r"\u{08FE}\u{08FF}",
    r"\u{FBB2}\u{FBB3}\u{FBB4}\u{FBB5}\u{FBB6}\u{FBB7}\u{FBB8}",
    r"\u{FBB9}\u{FBBA}\u{FBBB}\u{FBBC}\u{FBBD}\u{FBBE}\u{FBBF}",
    r"\u{FBC0}\u{FBC1}",
    r"\u{FC5E}\u{FC5F}\u{FC60}\u{FC61}\u{FC62}\u{FC63}",
    r"\u{FCF2}\u{FCF3}\u{FCF4}",
    r"\u{FD3E}\u{FD3F}",
    r"\u{FE70}\u{FE71}\u{FE72}\u{FE76}\u{FE77}\u{FE78}\u{FE79}",
    r"\u{FE7A}\u{FE7B}\u{FE7C}\u{FE7D}\u{FE7E}\u{FE7F}",
    r"\u{FDFA}\u{FDFB}]",
);

// ---------------------------------------------------------------------------
// Word tokenizer patterns
// ---------------------------------------------------------------------------

/// Main tokenization split pattern — splits on punctuation, brackets, digits.
pub const WORD_SPLIT_PATTERN: &str =
    r"([؟!?]+|[\d.:]+|[:.،؛»\])}\u{22}«\[({/\\])";

/// Pattern for Social-network @mentions.
pub const ID_PATTERN: &str = r"(?<![\w._])(@[\w_]+)";

/// Pattern for URLs/links.
pub const LINK_PATTERN: &str =
    r"((https?|ftp)://)?(?<!@)(([a-zA-Z0-9\-]+\.)+[a-zA-Z]{2,})[-\w@:%_.+/~#?=&]*";

/// Pattern for e-mail addresses.
pub const EMAIL_PATTERN: &str =
    r"[a-zA-Z0-9._+\-]+@([a-zA-Z0-9\-]+\.)+[A-Za-z]{2,}";

/// Pattern for integer numbers (both ASCII and Persian digits).
pub const NUMBER_INT_PATTERN: &str =
    r"\b(?<![\d۰-۹][.٫٬,])([\d۰-۹]+)(?![.٫٬,][\d۰-۹])\b";

/// Pattern for floating-point numbers.
pub const NUMBER_FLOAT_PATTERN: &str =
    r"\b(?<!\.)([\d۰-۹,٬]+[.٫٬][\d۰-۹]+)\b(?!\.)";

/// Pattern for hashtags.
pub const HASHTAG_PATTERN: &str = r"#(\S+)";

/// Emoji range pattern (covers major Unicode emoji blocks).
pub const EMOJI_PATTERN: &str = concat!(
    r"[\U0001F600-\U0001F64F",
    r"\U0001F300-\U0001F5FF",
    r"\U0001F4CC\U0001F4CD]",
);

// ---------------------------------------------------------------------------
// Normalizer — repeated character patterns
// ---------------------------------------------------------------------------

/// Matches any Persian letter repeated 3+ times in a row.
pub const MORE_THAN_TWO_REPEAT_PATTERN: &str =
    r"([آابپتثجچحخدذرزژسشصضطظعغفقکگلمنوهی])\1{2,}";

/// Matches entire words that contain a Persian letter repeated 3+ times.
pub const REPEATED_CHARS_PATTERN: &str = concat!(
    r"[آابپتثجچحخدذرزژسشصضطظعغفقکگلمنوهی]*",
    r"([آابپتثجچحخدذرزژسشصضطظعغفقکگلمنوهی])\1{2,}",
    r"[آابپتثجچحخدذرزژسشصضطظعغفقکگلمنوهی]*",
);

/// Pattern used in `seperate_mi` — matches می/نمی attached to a verb stem.
pub const SEPERATE_MI_PATTERN: &str =
    r"\bن?می[آابپتثجچحخدذرزژسشصضطظعغفقکگلمنوهی]+";
