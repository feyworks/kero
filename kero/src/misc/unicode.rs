use std::collections::HashSet;
use std::ops::{Add, AddAssign, RangeInclusive};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct UnicodeRange {
    name: &'static str,
    start: u32,
    end: u32,
}

impl UnicodeRange {
    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub fn count(&self) -> usize {
        (self.end - self.start) as usize
    }

    #[inline]
    pub fn combine(self, other: Self) -> UnicodeRanges {
        UnicodeRanges {
            ranges: [self, other].into_iter().collect(),
        }
    }

    pub const fn contains(&self, code: u32) -> bool {
        code >= self.start && code <= self.end
    }

    pub const fn codes(&self) -> RangeInclusive<u32> {
        self.start..=self.end
    }

    #[inline]
    pub fn contains_char(&self, chr: char) -> bool {
        self.chars().find(|c| c == &chr).is_some()
    }

    #[inline]
    pub fn chars(&self) -> impl Iterator<Item = char> {
        self.codes().filter_map(char::from_u32)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnicodeRanges {
    ranges: HashSet<UnicodeRange>,
}

impl UnicodeRanges {
    #[inline]
    pub fn count(&self) -> usize {
        self.ranges.iter().map(|r| r.count()).sum()
    }

    #[inline]
    pub fn add(&mut self, range: UnicodeRange) -> &mut Self {
        self.ranges.insert(range);
        self
    }

    #[inline]
    pub fn ranges(&self) -> impl Iterator<Item = UnicodeRange> + '_ {
        self.ranges.iter().copied()
    }

    #[inline]
    pub fn contains(&self, code: u32) -> bool {
        self.ranges.iter().any(|r| r.contains(code))
    }

    #[inline]
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.ranges.iter().map(|r| r.chars()).flatten()
    }

    #[inline]
    pub fn contains_char(&self, chr: char) -> bool {
        self.chars().find(|c| c == &chr).is_some()
    }
}

impl Add<UnicodeRange> for UnicodeRanges {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: UnicodeRange) -> Self::Output {
        self.ranges.insert(rhs);
        self
    }
}

impl AddAssign<UnicodeRange> for UnicodeRanges {
    #[inline]
    fn add_assign(&mut self, rhs: UnicodeRange) {
        self.ranges.insert(rhs);
    }
}

impl Add<UnicodeRange> for UnicodeRange {
    type Output = UnicodeRanges;

    #[inline]
    fn add(self, rhs: UnicodeRange) -> Self::Output {
        self.combine(rhs)
    }
}

pub const BASIC_LATIN: UnicodeRange = UnicodeRange {
    name: "Basic Latin",
    start: 0x0,
    end: 0x7F,
};
pub const LATIN_1_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Latin-1 Supplement",
    start: 0x80,
    end: 0xFF,
};
pub const LATIN_EXTENDED_A: UnicodeRange = UnicodeRange {
    name: "Latin Extended-A",
    start: 0x100,
    end: 0x17F,
};
pub const LATIN_EXTENDED_B: UnicodeRange = UnicodeRange {
    name: "Latin Extended-B",
    start: 0x180,
    end: 0x24F,
};
pub const IPA_EXTENSIONS: UnicodeRange = UnicodeRange {
    name: "IPA Extensions",
    start: 0x250,
    end: 0x2AF,
};
pub const SPACING_MODIFIER_LETTERS: UnicodeRange = UnicodeRange {
    name: "Spacing Modifier Letters",
    start: 0x2B0,
    end: 0x2FF,
};
pub const COMBINING_DIACRITICAL_MARKS: UnicodeRange = UnicodeRange {
    name: "Combining Diacritical Marks",
    start: 0x300,
    end: 0x36F,
};
pub const GREEK_AND_COPTIC: UnicodeRange = UnicodeRange {
    name: "Greek and Coptic",
    start: 0x370,
    end: 0x3FF,
};
pub const CYRILLIC: UnicodeRange = UnicodeRange {
    name: "Cyrillic",
    start: 0x400,
    end: 0x4FF,
};
pub const CYRILLIC_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Cyrillic Supplement",
    start: 0x500,
    end: 0x52F,
};
pub const ARMENIAN: UnicodeRange = UnicodeRange {
    name: "Armenian",
    start: 0x530,
    end: 0x58F,
};
pub const HEBREW: UnicodeRange = UnicodeRange {
    name: "Hebrew",
    start: 0x590,
    end: 0x5FF,
};
pub const ARABIC: UnicodeRange = UnicodeRange {
    name: "Arabic",
    start: 0x600,
    end: 0x6FF,
};
pub const SYRIAC: UnicodeRange = UnicodeRange {
    name: "Syriac",
    start: 0x700,
    end: 0x74F,
};
pub const ARABIC_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Arabic Supplement",
    start: 0x750,
    end: 0x77F,
};
pub const THAANA: UnicodeRange = UnicodeRange {
    name: "Thaana",
    start: 0x780,
    end: 0x7BF,
};
pub const N_KO: UnicodeRange = UnicodeRange {
    name: "NKo",
    start: 0x7C0,
    end: 0x7FF,
};
pub const SAMARITAN: UnicodeRange = UnicodeRange {
    name: "Samaritan",
    start: 0x800,
    end: 0x83F,
};
pub const MANDAIC: UnicodeRange = UnicodeRange {
    name: "Mandaic",
    start: 0x840,
    end: 0x85F,
};
pub const SYRIAC_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Syriac Supplement",
    start: 0x860,
    end: 0x86F,
};
pub const ARABIC_EXTENDED_A: UnicodeRange = UnicodeRange {
    name: "Arabic Extended-A",
    start: 0x8A0,
    end: 0x8FF,
};
pub const DEVANAGARI: UnicodeRange = UnicodeRange {
    name: "Devanagari",
    start: 0x900,
    end: 0x97F,
};
pub const BENGALI: UnicodeRange = UnicodeRange {
    name: "Bengali",
    start: 0x980,
    end: 0x9FF,
};
pub const GURMUKHI: UnicodeRange = UnicodeRange {
    name: "Gurmukhi",
    start: 0xA00,
    end: 0xA7F,
};
pub const GUJARATI: UnicodeRange = UnicodeRange {
    name: "Gujarati",
    start: 0xA80,
    end: 0xAFF,
};
pub const ORIYA: UnicodeRange = UnicodeRange {
    name: "Oriya",
    start: 0xB00,
    end: 0xB7F,
};
pub const TAMIL: UnicodeRange = UnicodeRange {
    name: "Tamil",
    start: 0xB80,
    end: 0xBFF,
};
pub const TELUGU: UnicodeRange = UnicodeRange {
    name: "Telugu",
    start: 0xC00,
    end: 0xC7F,
};
pub const KANNADA: UnicodeRange = UnicodeRange {
    name: "Kannada",
    start: 0xC80,
    end: 0xCFF,
};
pub const MALAYALAM: UnicodeRange = UnicodeRange {
    name: "Malayalam",
    start: 0xD00,
    end: 0xD7F,
};
pub const SINHALA: UnicodeRange = UnicodeRange {
    name: "Sinhala",
    start: 0xD80,
    end: 0xDFF,
};
pub const THAI: UnicodeRange = UnicodeRange {
    name: "Thai",
    start: 0xE00,
    end: 0xE7F,
};
pub const LAO: UnicodeRange = UnicodeRange {
    name: "Lao",
    start: 0xE80,
    end: 0xEFF,
};
pub const TIBETAN: UnicodeRange = UnicodeRange {
    name: "Tibetan",
    start: 0xF00,
    end: 0xFFF,
};
pub const MYANMAR: UnicodeRange = UnicodeRange {
    name: "Myanmar",
    start: 0x1000,
    end: 0x109F,
};
pub const GEORGIAN: UnicodeRange = UnicodeRange {
    name: "Georgian",
    start: 0x10A0,
    end: 0x10FF,
};
pub const HANGUL_JAMO: UnicodeRange = UnicodeRange {
    name: "Hangul Jamo",
    start: 0x1100,
    end: 0x11FF,
};
pub const ETHIOPIC: UnicodeRange = UnicodeRange {
    name: "Ethiopic",
    start: 0x1200,
    end: 0x137F,
};
pub const ETHIOPIC_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Ethiopic Supplement",
    start: 0x1380,
    end: 0x139F,
};
pub const CHEROKEE: UnicodeRange = UnicodeRange {
    name: "Cherokee",
    start: 0x13A0,
    end: 0x13FF,
};
pub const UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS: UnicodeRange = UnicodeRange {
    name: "Unified Canadian Aboriginal Syllabics",
    start: 0x1400,
    end: 0x167F,
};
pub const OGHAM: UnicodeRange = UnicodeRange {
    name: "Ogham",
    start: 0x1680,
    end: 0x169F,
};
pub const RUNIC: UnicodeRange = UnicodeRange {
    name: "Runic",
    start: 0x16A0,
    end: 0x16FF,
};
pub const TAGALOG: UnicodeRange = UnicodeRange {
    name: "Tagalog",
    start: 0x1700,
    end: 0x171F,
};
pub const HANUNOO: UnicodeRange = UnicodeRange {
    name: "Hanunoo",
    start: 0x1720,
    end: 0x173F,
};
pub const BUHID: UnicodeRange = UnicodeRange {
    name: "Buhid",
    start: 0x1740,
    end: 0x175F,
};
pub const TAGBANWA: UnicodeRange = UnicodeRange {
    name: "Tagbanwa",
    start: 0x1760,
    end: 0x177F,
};
pub const KHMER: UnicodeRange = UnicodeRange {
    name: "Khmer",
    start: 0x1780,
    end: 0x17FF,
};
pub const MONGOLIAN: UnicodeRange = UnicodeRange {
    name: "Mongolian",
    start: 0x1800,
    end: 0x18AF,
};
pub const UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED: UnicodeRange = UnicodeRange {
    name: "Unified Canadian Aboriginal Syllabics Extended",
    start: 0x18B0,
    end: 0x18FF,
};
pub const LIMBU: UnicodeRange = UnicodeRange {
    name: "Limbu",
    start: 0x1900,
    end: 0x194F,
};
pub const TAI_LE: UnicodeRange = UnicodeRange {
    name: "Tai Le",
    start: 0x1950,
    end: 0x197F,
};
pub const NEW_TAI_LUE: UnicodeRange = UnicodeRange {
    name: "New Tai Lue",
    start: 0x1980,
    end: 0x19DF,
};
pub const KHMER_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Khmer Symbols",
    start: 0x19E0,
    end: 0x19FF,
};
pub const BUGINESE: UnicodeRange = UnicodeRange {
    name: "Buginese",
    start: 0x1A00,
    end: 0x1A1F,
};
pub const TAI_THAM: UnicodeRange = UnicodeRange {
    name: "Tai Tham",
    start: 0x1A20,
    end: 0x1AAF,
};
pub const COMBINING_DIACRITICAL_MARKS_EXTENDED: UnicodeRange = UnicodeRange {
    name: "Combining Diacritical Marks Extended",
    start: 0x1AB0,
    end: 0x1AFF,
};
pub const BALINESE: UnicodeRange = UnicodeRange {
    name: "Balinese",
    start: 0x1B00,
    end: 0x1B7F,
};
pub const SUNDANESE: UnicodeRange = UnicodeRange {
    name: "Sundanese",
    start: 0x1B80,
    end: 0x1BBF,
};
pub const BATAK: UnicodeRange = UnicodeRange {
    name: "Batak",
    start: 0x1BC0,
    end: 0x1BFF,
};
pub const LEPCHA: UnicodeRange = UnicodeRange {
    name: "Lepcha",
    start: 0x1C00,
    end: 0x1C4F,
};
pub const OL_CHIKI: UnicodeRange = UnicodeRange {
    name: "Ol Chiki",
    start: 0x1C50,
    end: 0x1C7F,
};
pub const CYRILLIC_EXTENDED_C: UnicodeRange = UnicodeRange {
    name: "Cyrillic Extended-C",
    start: 0x1C80,
    end: 0x1C8F,
};
pub const GEORGIAN_EXTENDED: UnicodeRange = UnicodeRange {
    name: "Georgian Extended",
    start: 0x1C90,
    end: 0x1CBF,
};
pub const SUNDANESE_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Sundanese Supplement",
    start: 0x1CC0,
    end: 0x1CCF,
};
pub const VEDIC_EXTENSIONS: UnicodeRange = UnicodeRange {
    name: "Vedic Extensions",
    start: 0x1CD0,
    end: 0x1CFF,
};
pub const PHONETIC_EXTENSIONS: UnicodeRange = UnicodeRange {
    name: "Phonetic Extensions",
    start: 0x1D00,
    end: 0x1D7F,
};
pub const PHONETIC_EXTENSIONS_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Phonetic Extensions Supplement",
    start: 0x1D80,
    end: 0x1DBF,
};
pub const COMBINING_DIACRITICAL_MARKS_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Combining Diacritical Marks Supplement",
    start: 0x1DC0,
    end: 0x1DFF,
};
pub const LATIN_EXTENDED_ADDITIONAL: UnicodeRange = UnicodeRange {
    name: "Latin Extended Additional",
    start: 0x1E00,
    end: 0x1EFF,
};
pub const GREEK_EXTENDED: UnicodeRange = UnicodeRange {
    name: "Greek Extended",
    start: 0x1F00,
    end: 0x1FFF,
};
pub const GENERAL_PUNCTUATION: UnicodeRange = UnicodeRange {
    name: "General Punctuation",
    start: 0x2000,
    end: 0x206F,
};
pub const SUPERSCRIPTS_AND_SUBSCRIPTS: UnicodeRange = UnicodeRange {
    name: "Superscripts and Subscripts",
    start: 0x2070,
    end: 0x209F,
};
pub const CURRENCY_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Currency Symbols",
    start: 0x20A0,
    end: 0x20CF,
};
pub const COMBINING_DIACRITICAL_MARKS_FOR_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Combining Diacritical Marks for Symbols",
    start: 0x20D0,
    end: 0x20FF,
};
pub const LETTERLIKE_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Letterlike Symbols",
    start: 0x2100,
    end: 0x214F,
};
pub const NUMBER_FORMS: UnicodeRange = UnicodeRange {
    name: "Number Forms",
    start: 0x2150,
    end: 0x218F,
};
pub const ARROWS: UnicodeRange = UnicodeRange {
    name: "Arrows",
    start: 0x2190,
    end: 0x21FF,
};
pub const MATHEMATICAL_OPERATORS: UnicodeRange = UnicodeRange {
    name: "Mathematical Operators",
    start: 0x2200,
    end: 0x22FF,
};
pub const MISCELLANEOUS_TECHNICAL: UnicodeRange = UnicodeRange {
    name: "Miscellaneous Technical",
    start: 0x2300,
    end: 0x23FF,
};
pub const CONTROL_PICTURES: UnicodeRange = UnicodeRange {
    name: "Control Pictures",
    start: 0x2400,
    end: 0x243F,
};
pub const OPTICAL_CHARACTER_RECOGNITION: UnicodeRange = UnicodeRange {
    name: "Optical Character Recognition",
    start: 0x2440,
    end: 0x245F,
};
pub const ENCLOSED_ALPHANUMERICS: UnicodeRange = UnicodeRange {
    name: "Enclosed Alphanumerics",
    start: 0x2460,
    end: 0x24FF,
};
pub const BOX_DRAWING: UnicodeRange = UnicodeRange {
    name: "Box Drawing",
    start: 0x2500,
    end: 0x257F,
};
pub const BLOCK_ELEMENTS: UnicodeRange = UnicodeRange {
    name: "Block Elements",
    start: 0x2580,
    end: 0x259F,
};
pub const GEOMETRIC_SHAPES: UnicodeRange = UnicodeRange {
    name: "Geometric Shapes",
    start: 0x25A0,
    end: 0x25FF,
};
pub const MISCELLANEOUS_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Miscellaneous Symbols",
    start: 0x2600,
    end: 0x26FF,
};
pub const DINGBATS: UnicodeRange = UnicodeRange {
    name: "Dingbats",
    start: 0x2700,
    end: 0x27BF,
};
pub const MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A: UnicodeRange = UnicodeRange {
    name: "Miscellaneous Mathematical Symbols-A",
    start: 0x27C0,
    end: 0x27EF,
};
pub const SUPPLEMENTAL_ARROWS_A: UnicodeRange = UnicodeRange {
    name: "Supplemental Arrows-A",
    start: 0x27F0,
    end: 0x27FF,
};
pub const BRAILLE_PATTERNS: UnicodeRange = UnicodeRange {
    name: "Braille Patterns",
    start: 0x2800,
    end: 0x28FF,
};
pub const SUPPLEMENTAL_ARROWS_B: UnicodeRange = UnicodeRange {
    name: "Supplemental Arrows-B",
    start: 0x2900,
    end: 0x297F,
};
pub const MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B: UnicodeRange = UnicodeRange {
    name: "Miscellaneous Mathematical Symbols-B",
    start: 0x2980,
    end: 0x29FF,
};
pub const SUPPLEMENTAL_MATHEMATICAL_OPERATORS: UnicodeRange = UnicodeRange {
    name: "Supplemental Mathematical Operators",
    start: 0x2A00,
    end: 0x2AFF,
};
pub const MISCELLANEOUS_SYMBOLS_AND_ARROWS: UnicodeRange = UnicodeRange {
    name: "Miscellaneous Symbols and Arrows",
    start: 0x2B00,
    end: 0x2BFF,
};
pub const GLAGOLITIC: UnicodeRange = UnicodeRange {
    name: "Glagolitic",
    start: 0x2C00,
    end: 0x2C5F,
};
pub const LATIN_EXTENDED_C: UnicodeRange = UnicodeRange {
    name: "Latin Extended-C",
    start: 0x2C60,
    end: 0x2C7F,
};
pub const COPTIC: UnicodeRange = UnicodeRange {
    name: "Coptic",
    start: 0x2C80,
    end: 0x2CFF,
};
pub const GEORGIAN_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Georgian Supplement",
    start: 0x2D00,
    end: 0x2D2F,
};
pub const TIFINAGH: UnicodeRange = UnicodeRange {
    name: "Tifinagh",
    start: 0x2D30,
    end: 0x2D7F,
};
pub const ETHIOPIC_EXTENDED: UnicodeRange = UnicodeRange {
    name: "Ethiopic Extended",
    start: 0x2D80,
    end: 0x2DDF,
};
pub const CYRILLIC_EXTENDED_A: UnicodeRange = UnicodeRange {
    name: "Cyrillic Extended-A",
    start: 0x2DE0,
    end: 0x2DFF,
};
pub const SUPPLEMENTAL_PUNCTUATION: UnicodeRange = UnicodeRange {
    name: "Supplemental Punctuation",
    start: 0x2E00,
    end: 0x2E7F,
};
pub const CJK_RADICALS_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "CJK Radicals Supplement",
    start: 0x2E80,
    end: 0x2EFF,
};
pub const KANGXI_RADICALS: UnicodeRange = UnicodeRange {
    name: "Kangxi Radicals",
    start: 0x2F00,
    end: 0x2FDF,
};
pub const IDEOGRAPHIC_DESCRIPTION_CHARACTERS: UnicodeRange = UnicodeRange {
    name: "Ideographic Description Characters",
    start: 0x2FF0,
    end: 0x2FFF,
};
pub const CJK_SYMBOLS_AND_PUNCTUATION: UnicodeRange = UnicodeRange {
    name: "CJK Symbols and Punctuation",
    start: 0x3000,
    end: 0x303F,
};
pub const HIRAGANA: UnicodeRange = UnicodeRange {
    name: "Hiragana",
    start: 0x3040,
    end: 0x309F,
};
pub const KATAKANA: UnicodeRange = UnicodeRange {
    name: "Katakana",
    start: 0x30A0,
    end: 0x30FF,
};
pub const BOPOMOFO: UnicodeRange = UnicodeRange {
    name: "Bopomofo",
    start: 0x3100,
    end: 0x312F,
};
pub const HANGUL_COMPATIBILITY_JAMO: UnicodeRange = UnicodeRange {
    name: "Hangul Compatibility Jamo",
    start: 0x3130,
    end: 0x318F,
};
pub const KANBUN: UnicodeRange = UnicodeRange {
    name: "Kanbun",
    start: 0x3190,
    end: 0x319F,
};
pub const BOPOMOFO_EXTENDED: UnicodeRange = UnicodeRange {
    name: "Bopomofo Extended",
    start: 0x31A0,
    end: 0x31BF,
};
pub const CJK_STROKES: UnicodeRange = UnicodeRange {
    name: "CJK Strokes",
    start: 0x31C0,
    end: 0x31EF,
};
pub const KATAKANA_PHONETIC_EXTENSIONS: UnicodeRange = UnicodeRange {
    name: "Katakana Phonetic Extensions",
    start: 0x31F0,
    end: 0x31FF,
};
pub const ENCLOSED_CJK_LETTERS_AND_MONTHS: UnicodeRange = UnicodeRange {
    name: "Enclosed CJK Letters and Months",
    start: 0x3200,
    end: 0x32FF,
};
pub const CJK_COMPATIBILITY: UnicodeRange = UnicodeRange {
    name: "CJK Compatibility",
    start: 0x3300,
    end: 0x33FF,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A: UnicodeRange = UnicodeRange {
    name: "CJK Unified Ideographs Extension A",
    start: 0x3400,
    end: 0x4DBF,
};
pub const YIJING_HEXAGRAM_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Yijing Hexagram Symbols",
    start: 0x4DC0,
    end: 0x4DFF,
};
pub const CJK_UNIFIED_IDEOGRAPHS: UnicodeRange = UnicodeRange {
    name: "CJK Unified Ideographs",
    start: 0x4E00,
    end: 0x9FFF,
};
pub const YI_SYLLABLES: UnicodeRange = UnicodeRange {
    name: "Yi Syllables",
    start: 0xA000,
    end: 0xA48F,
};
pub const YI_RADICALS: UnicodeRange = UnicodeRange {
    name: "Yi Radicals",
    start: 0xA490,
    end: 0xA4CF,
};
pub const LISU: UnicodeRange = UnicodeRange {
    name: "Lisu",
    start: 0xA4D0,
    end: 0xA4FF,
};
pub const VAI: UnicodeRange = UnicodeRange {
    name: "Vai",
    start: 0xA500,
    end: 0xA63F,
};
pub const CYRILLIC_EXTENDED_B: UnicodeRange = UnicodeRange {
    name: "Cyrillic Extended-B",
    start: 0xA640,
    end: 0xA69F,
};
pub const BAMUM: UnicodeRange = UnicodeRange {
    name: "Bamum",
    start: 0xA6A0,
    end: 0xA6FF,
};
pub const MODIFIER_TONE_LETTERS: UnicodeRange = UnicodeRange {
    name: "Modifier Tone Letters",
    start: 0xA700,
    end: 0xA71F,
};
pub const LATIN_EXTENDED_D: UnicodeRange = UnicodeRange {
    name: "Latin Extended-D",
    start: 0xA720,
    end: 0xA7FF,
};
pub const SYLOTI_NAGRI: UnicodeRange = UnicodeRange {
    name: "Syloti Nagri",
    start: 0xA800,
    end: 0xA82F,
};
pub const COMMON_INDIC_NUMBER_FORMS: UnicodeRange = UnicodeRange {
    name: "Common Indic Number Forms",
    start: 0xA830,
    end: 0xA83F,
};
pub const PHAGS_PA: UnicodeRange = UnicodeRange {
    name: "Phags-pa",
    start: 0xA840,
    end: 0xA87F,
};
pub const SAURASHTRA: UnicodeRange = UnicodeRange {
    name: "Saurashtra",
    start: 0xA880,
    end: 0xA8DF,
};
pub const DEVANAGARI_EXTENDED: UnicodeRange = UnicodeRange {
    name: "Devanagari Extended",
    start: 0xA8E0,
    end: 0xA8FF,
};
pub const KAYAH_LI: UnicodeRange = UnicodeRange {
    name: "Kayah Li",
    start: 0xA900,
    end: 0xA92F,
};
pub const REJANG: UnicodeRange = UnicodeRange {
    name: "Rejang",
    start: 0xA930,
    end: 0xA95F,
};
pub const HANGUL_JAMO_EXTENDED_A: UnicodeRange = UnicodeRange {
    name: "Hangul Jamo Extended-A",
    start: 0xA960,
    end: 0xA97F,
};
pub const JAVANESE: UnicodeRange = UnicodeRange {
    name: "Javanese",
    start: 0xA980,
    end: 0xA9DF,
};
pub const MYANMAR_EXTENDED_B: UnicodeRange = UnicodeRange {
    name: "Myanmar Extended-B",
    start: 0xA9E0,
    end: 0xA9FF,
};
pub const CHAM: UnicodeRange = UnicodeRange {
    name: "Cham",
    start: 0xAA00,
    end: 0xAA5F,
};
pub const MYANMAR_EXTENDED_A: UnicodeRange = UnicodeRange {
    name: "Myanmar Extended-A",
    start: 0xAA60,
    end: 0xAA7F,
};
pub const TAI_VIET: UnicodeRange = UnicodeRange {
    name: "Tai Viet",
    start: 0xAA80,
    end: 0xAADF,
};
pub const MEETEI_MAYEK_EXTENSIONS: UnicodeRange = UnicodeRange {
    name: "Meetei Mayek Extensions",
    start: 0xAAE0,
    end: 0xAAFF,
};
pub const ETHIOPIC_EXTENDED_A: UnicodeRange = UnicodeRange {
    name: "Ethiopic Extended-A",
    start: 0xAB00,
    end: 0xAB2F,
};
pub const LATIN_EXTENDED_E: UnicodeRange = UnicodeRange {
    name: "Latin Extended-E",
    start: 0xAB30,
    end: 0xAB6F,
};
pub const CHEROKEE_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Cherokee Supplement",
    start: 0xAB70,
    end: 0xABBF,
};
pub const MEETEI_MAYEK: UnicodeRange = UnicodeRange {
    name: "Meetei Mayek",
    start: 0xABC0,
    end: 0xABFF,
};
pub const HANGUL_SYLLABLES: UnicodeRange = UnicodeRange {
    name: "Hangul Syllables",
    start: 0xAC00,
    end: 0xD7AF,
};
pub const HANGUL_JAMO_EXTENDED_B: UnicodeRange = UnicodeRange {
    name: "Hangul Jamo Extended-B",
    start: 0xD7B0,
    end: 0xD7FF,
};
pub const HIGH_SURROGATES: UnicodeRange = UnicodeRange {
    name: "High Surrogates",
    start: 0xD800,
    end: 0xDB7F,
};
pub const HIGH_PRIVATE_USE_SURROGATES: UnicodeRange = UnicodeRange {
    name: "High Private Use Surrogates",
    start: 0xDB80,
    end: 0xDBFF,
};
pub const LOW_SURROGATES: UnicodeRange = UnicodeRange {
    name: "Low Surrogates",
    start: 0xDC00,
    end: 0xDFFF,
};
pub const PRIVATE_USE_AREA: UnicodeRange = UnicodeRange {
    name: "Private Use Area",
    start: 0xE000,
    end: 0xF8FF,
};
pub const CJK_COMPATIBILITY_IDEOGRAPHS: UnicodeRange = UnicodeRange {
    name: "CJK Compatibility Ideographs",
    start: 0xF900,
    end: 0xFAFF,
};
pub const ALPHABETIC_PRESENTATION_FORMS: UnicodeRange = UnicodeRange {
    name: "Alphabetic Presentation Forms",
    start: 0xFB00,
    end: 0xFB4F,
};
pub const ARABIC_PRESENTATION_FORMS_A: UnicodeRange = UnicodeRange {
    name: "Arabic Presentation Forms-A",
    start: 0xFB50,
    end: 0xFDFF,
};
pub const VARIATION_SELECTORS: UnicodeRange = UnicodeRange {
    name: "Variation Selectors",
    start: 0xFE00,
    end: 0xFE0F,
};
pub const VERTICAL_FORMS: UnicodeRange = UnicodeRange {
    name: "Vertical Forms",
    start: 0xFE10,
    end: 0xFE1F,
};
pub const COMBINING_HALF_MARKS: UnicodeRange = UnicodeRange {
    name: "Combining Half Marks",
    start: 0xFE20,
    end: 0xFE2F,
};
pub const CJK_COMPATIBILITY_FORMS: UnicodeRange = UnicodeRange {
    name: "CJK Compatibility Forms",
    start: 0xFE30,
    end: 0xFE4F,
};
pub const SMALL_FORM_VARIANTS: UnicodeRange = UnicodeRange {
    name: "Small Form Variants",
    start: 0xFE50,
    end: 0xFE6F,
};
pub const ARABIC_PRESENTATION_FORMS_B: UnicodeRange = UnicodeRange {
    name: "Arabic Presentation Forms-B",
    start: 0xFE70,
    end: 0xFEFF,
};
pub const HALFWIDTH_AND_FULLWIDTH_FORMS: UnicodeRange = UnicodeRange {
    name: "Halfwidth and Fullwidth Forms",
    start: 0xFF00,
    end: 0xFFEF,
};
pub const SPECIALS: UnicodeRange = UnicodeRange {
    name: "Specials",
    start: 0xFFF0,
    end: 0xFFFF,
};
pub const LINEAR_B_SYLLABARY: UnicodeRange = UnicodeRange {
    name: "Linear B Syllabary",
    start: 0x10000,
    end: 0x1007F,
};
pub const LINEAR_B_IDEOGRAMS: UnicodeRange = UnicodeRange {
    name: "Linear B Ideograms",
    start: 0x10080,
    end: 0x100FF,
};
pub const AEGEAN_NUMBERS: UnicodeRange = UnicodeRange {
    name: "Aegean Numbers",
    start: 0x10100,
    end: 0x1013F,
};
pub const ANCIENT_GREEK_NUMBERS: UnicodeRange = UnicodeRange {
    name: "Ancient Greek Numbers",
    start: 0x10140,
    end: 0x1018F,
};
pub const ANCIENT_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Ancient Symbols",
    start: 0x10190,
    end: 0x101CF,
};
pub const PHAISTOS_DISC: UnicodeRange = UnicodeRange {
    name: "Phaistos Disc",
    start: 0x101D0,
    end: 0x101FF,
};
pub const LYCIAN: UnicodeRange = UnicodeRange {
    name: "Lycian",
    start: 0x10280,
    end: 0x1029F,
};
pub const CARIAN: UnicodeRange = UnicodeRange {
    name: "Carian",
    start: 0x102A0,
    end: 0x102DF,
};
pub const COPTIC_EPACT_NUMBERS: UnicodeRange = UnicodeRange {
    name: "Coptic Epact Numbers",
    start: 0x102E0,
    end: 0x102FF,
};
pub const OLD_ITALIC: UnicodeRange = UnicodeRange {
    name: "Old Italic",
    start: 0x10300,
    end: 0x1032F,
};
pub const GOTHIC: UnicodeRange = UnicodeRange {
    name: "Gothic",
    start: 0x10330,
    end: 0x1034F,
};
pub const OLD_PERMIC: UnicodeRange = UnicodeRange {
    name: "Old Permic",
    start: 0x10350,
    end: 0x1037F,
};
pub const UGARITIC: UnicodeRange = UnicodeRange {
    name: "Ugaritic",
    start: 0x10380,
    end: 0x1039F,
};
pub const OLD_PERSIAN: UnicodeRange = UnicodeRange {
    name: "Old Persian",
    start: 0x103A0,
    end: 0x103DF,
};
pub const DESERET: UnicodeRange = UnicodeRange {
    name: "Deseret",
    start: 0x10400,
    end: 0x1044F,
};
pub const SHAVIAN: UnicodeRange = UnicodeRange {
    name: "Shavian",
    start: 0x10450,
    end: 0x1047F,
};
pub const OSMANYA: UnicodeRange = UnicodeRange {
    name: "Osmanya",
    start: 0x10480,
    end: 0x104AF,
};
pub const OSAGE: UnicodeRange = UnicodeRange {
    name: "Osage",
    start: 0x104B0,
    end: 0x104FF,
};
pub const ELBASAN: UnicodeRange = UnicodeRange {
    name: "Elbasan",
    start: 0x10500,
    end: 0x1052F,
};
pub const CAUCASIAN_ALBANIAN: UnicodeRange = UnicodeRange {
    name: "Caucasian Albanian",
    start: 0x10530,
    end: 0x1056F,
};
pub const LINEAR_A: UnicodeRange = UnicodeRange {
    name: "Linear A",
    start: 0x10600,
    end: 0x1077F,
};
pub const CYPRIOT_SYLLABARY: UnicodeRange = UnicodeRange {
    name: "Cypriot Syllabary",
    start: 0x10800,
    end: 0x1083F,
};
pub const IMPERIAL_ARAMAIC: UnicodeRange = UnicodeRange {
    name: "Imperial Aramaic",
    start: 0x10840,
    end: 0x1085F,
};
pub const PALMYRENE: UnicodeRange = UnicodeRange {
    name: "Palmyrene",
    start: 0x10860,
    end: 0x1087F,
};
pub const NABATAEAN: UnicodeRange = UnicodeRange {
    name: "Nabataean",
    start: 0x10880,
    end: 0x108AF,
};
pub const HATRAN: UnicodeRange = UnicodeRange {
    name: "Hatran",
    start: 0x108E0,
    end: 0x108FF,
};
pub const PHOENICIAN: UnicodeRange = UnicodeRange {
    name: "Phoenician",
    start: 0x10900,
    end: 0x1091F,
};
pub const LYDIAN: UnicodeRange = UnicodeRange {
    name: "Lydian",
    start: 0x10920,
    end: 0x1093F,
};
pub const MEROITIC_HIEROGLYPHS: UnicodeRange = UnicodeRange {
    name: "Meroitic Hieroglyphs",
    start: 0x10980,
    end: 0x1099F,
};
pub const MEROITIC_CURSIVE: UnicodeRange = UnicodeRange {
    name: "Meroitic Cursive",
    start: 0x109A0,
    end: 0x109FF,
};
pub const KHAROSHTHI: UnicodeRange = UnicodeRange {
    name: "Kharoshthi",
    start: 0x10A00,
    end: 0x10A5F,
};
pub const OLD_SOUTH_ARABIAN: UnicodeRange = UnicodeRange {
    name: "Old South Arabian",
    start: 0x10A60,
    end: 0x10A7F,
};
pub const OLD_NORTH_ARABIAN: UnicodeRange = UnicodeRange {
    name: "Old North Arabian",
    start: 0x10A80,
    end: 0x10A9F,
};
pub const MANICHAEAN: UnicodeRange = UnicodeRange {
    name: "Manichaean",
    start: 0x10AC0,
    end: 0x10AFF,
};
pub const AVESTAN: UnicodeRange = UnicodeRange {
    name: "Avestan",
    start: 0x10B00,
    end: 0x10B3F,
};
pub const INSCRIPTIONAL_PARTHIAN: UnicodeRange = UnicodeRange {
    name: "Inscriptional Parthian",
    start: 0x10B40,
    end: 0x10B5F,
};
pub const INSCRIPTIONAL_PAHLAVI: UnicodeRange = UnicodeRange {
    name: "Inscriptional Pahlavi",
    start: 0x10B60,
    end: 0x10B7F,
};
pub const PSALTER_PAHLAVI: UnicodeRange = UnicodeRange {
    name: "Psalter Pahlavi",
    start: 0x10B80,
    end: 0x10BAF,
};
pub const OLD_TURKIC: UnicodeRange = UnicodeRange {
    name: "Old Turkic",
    start: 0x10C00,
    end: 0x10C4F,
};
pub const OLD_HUNGARIAN: UnicodeRange = UnicodeRange {
    name: "Old Hungarian",
    start: 0x10C80,
    end: 0x10CFF,
};
pub const HANIFI_ROHINGYA: UnicodeRange = UnicodeRange {
    name: "Hanifi Rohingya",
    start: 0x10D00,
    end: 0x10D3F,
};
pub const RUMI_NUMERAL_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Rumi Numeral Symbols",
    start: 0x10E60,
    end: 0x10E7F,
};
pub const YEZIDI: UnicodeRange = UnicodeRange {
    name: "Yezidi",
    start: 0x10E80,
    end: 0x10EBF,
};
pub const OLD_SOGDIAN: UnicodeRange = UnicodeRange {
    name: "Old Sogdian",
    start: 0x10F00,
    end: 0x10F2F,
};
pub const SOGDIAN: UnicodeRange = UnicodeRange {
    name: "Sogdian",
    start: 0x10F30,
    end: 0x10F6F,
};
pub const CHORASMIAN: UnicodeRange = UnicodeRange {
    name: "Chorasmian",
    start: 0x10FB0,
    end: 0x10FDF,
};
pub const ELYMAIC: UnicodeRange = UnicodeRange {
    name: "Elymaic",
    start: 0x10FE0,
    end: 0x10FFF,
};
pub const BRAHMI: UnicodeRange = UnicodeRange {
    name: "Brahmi",
    start: 0x11000,
    end: 0x1107F,
};
pub const KAITHI: UnicodeRange = UnicodeRange {
    name: "Kaithi",
    start: 0x11080,
    end: 0x110CF,
};
pub const SORA_SOMPENG: UnicodeRange = UnicodeRange {
    name: "Sora Sompeng",
    start: 0x110D0,
    end: 0x110FF,
};
pub const CHAKMA: UnicodeRange = UnicodeRange {
    name: "Chakma",
    start: 0x11100,
    end: 0x1114F,
};
pub const MAHAJANI: UnicodeRange = UnicodeRange {
    name: "Mahajani",
    start: 0x11150,
    end: 0x1117F,
};
pub const SHARADA: UnicodeRange = UnicodeRange {
    name: "Sharada",
    start: 0x11180,
    end: 0x111DF,
};
pub const SINHALA_ARCHAIC_NUMBERS: UnicodeRange = UnicodeRange {
    name: "Sinhala Archaic Numbers",
    start: 0x111E0,
    end: 0x111FF,
};
pub const KHOJKI: UnicodeRange = UnicodeRange {
    name: "Khojki",
    start: 0x11200,
    end: 0x1124F,
};
pub const MULTANI: UnicodeRange = UnicodeRange {
    name: "Multani",
    start: 0x11280,
    end: 0x112AF,
};
pub const KHUDAWADI: UnicodeRange = UnicodeRange {
    name: "Khudawadi",
    start: 0x112B0,
    end: 0x112FF,
};
pub const GRANTHA: UnicodeRange = UnicodeRange {
    name: "Grantha",
    start: 0x11300,
    end: 0x1137F,
};
pub const NEWA: UnicodeRange = UnicodeRange {
    name: "Newa",
    start: 0x11400,
    end: 0x1147F,
};
pub const TIRHUTA: UnicodeRange = UnicodeRange {
    name: "Tirhuta",
    start: 0x11480,
    end: 0x114DF,
};
pub const SIDDHAM: UnicodeRange = UnicodeRange {
    name: "Siddham",
    start: 0x11580,
    end: 0x115FF,
};
pub const MODI: UnicodeRange = UnicodeRange {
    name: "Modi",
    start: 0x11600,
    end: 0x1165F,
};
pub const MONGOLIAN_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Mongolian Supplement",
    start: 0x11660,
    end: 0x1167F,
};
pub const TAKRI: UnicodeRange = UnicodeRange {
    name: "Takri",
    start: 0x11680,
    end: 0x116CF,
};
pub const AHOM: UnicodeRange = UnicodeRange {
    name: "Ahom",
    start: 0x11700,
    end: 0x1173F,
};
pub const DOGRA: UnicodeRange = UnicodeRange {
    name: "Dogra",
    start: 0x11800,
    end: 0x1184F,
};
pub const WARANG_CITI: UnicodeRange = UnicodeRange {
    name: "Warang Citi",
    start: 0x118A0,
    end: 0x118FF,
};
pub const DIVES_AKURU: UnicodeRange = UnicodeRange {
    name: "Dives Akuru",
    start: 0x11900,
    end: 0x1195F,
};
pub const NANDINAGARI: UnicodeRange = UnicodeRange {
    name: "Nandinagari",
    start: 0x119A0,
    end: 0x119FF,
};
pub const ZANABAZAR_SQUARE: UnicodeRange = UnicodeRange {
    name: "Zanabazar Square",
    start: 0x11A00,
    end: 0x11A4F,
};
pub const SOYOMBO: UnicodeRange = UnicodeRange {
    name: "Soyombo",
    start: 0x11A50,
    end: 0x11AAF,
};
pub const PAU_CIN_HAU: UnicodeRange = UnicodeRange {
    name: "Pau Cin Hau",
    start: 0x11AC0,
    end: 0x11AFF,
};
pub const BHAIKSUKI: UnicodeRange = UnicodeRange {
    name: "Bhaiksuki",
    start: 0x11C00,
    end: 0x11C6F,
};
pub const MARCHEN: UnicodeRange = UnicodeRange {
    name: "Marchen",
    start: 0x11C70,
    end: 0x11CBF,
};
pub const MASARAM_GONDI: UnicodeRange = UnicodeRange {
    name: "Masaram Gondi",
    start: 0x11D00,
    end: 0x11D5F,
};
pub const GUNJALA_GONDI: UnicodeRange = UnicodeRange {
    name: "Gunjala Gondi",
    start: 0x11D60,
    end: 0x11DAF,
};
pub const MAKASAR: UnicodeRange = UnicodeRange {
    name: "Makasar",
    start: 0x11EE0,
    end: 0x11EFF,
};
pub const LISU_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Lisu Supplement",
    start: 0x11FB0,
    end: 0x11FBF,
};
pub const TAMIL_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Tamil Supplement",
    start: 0x11FC0,
    end: 0x11FFF,
};
pub const CUNEIFORM: UnicodeRange = UnicodeRange {
    name: "Cuneiform",
    start: 0x12000,
    end: 0x123FF,
};
pub const CUNEIFORM_NUMBERS_AND_PUNCTUATION: UnicodeRange = UnicodeRange {
    name: "Cuneiform Numbers and Punctuation",
    start: 0x12400,
    end: 0x1247F,
};
pub const EARLY_DYNASTIC_CUNEIFORM: UnicodeRange = UnicodeRange {
    name: "Early Dynastic Cuneiform",
    start: 0x12480,
    end: 0x1254F,
};
pub const EGYPTIAN_HIEROGLYPHS: UnicodeRange = UnicodeRange {
    name: "Egyptian Hieroglyphs",
    start: 0x13000,
    end: 0x1342F,
};
pub const EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS: UnicodeRange = UnicodeRange {
    name: "Egyptian Hieroglyph Format Controls",
    start: 0x13430,
    end: 0x1343F,
};
pub const ANATOLIAN_HIEROGLYPHS: UnicodeRange = UnicodeRange {
    name: "Anatolian Hieroglyphs",
    start: 0x14400,
    end: 0x1467F,
};
pub const BAMUM_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Bamum Supplement",
    start: 0x16800,
    end: 0x16A3F,
};
pub const MRO: UnicodeRange = UnicodeRange {
    name: "Mro",
    start: 0x16A40,
    end: 0x16A6F,
};
pub const BASSA_VAH: UnicodeRange = UnicodeRange {
    name: "Bassa Vah",
    start: 0x16AD0,
    end: 0x16AFF,
};
pub const PAHAWH_HMONG: UnicodeRange = UnicodeRange {
    name: "Pahawh Hmong",
    start: 0x16B00,
    end: 0x16B8F,
};
pub const MEDEFAIDRIN: UnicodeRange = UnicodeRange {
    name: "Medefaidrin",
    start: 0x16E40,
    end: 0x16E9F,
};
pub const MIAO: UnicodeRange = UnicodeRange {
    name: "Miao",
    start: 0x16F00,
    end: 0x16F9F,
};
pub const IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION: UnicodeRange = UnicodeRange {
    name: "Ideographic Symbols and Punctuation",
    start: 0x16FE0,
    end: 0x16FFF,
};
pub const TANGUT: UnicodeRange = UnicodeRange {
    name: "Tangut",
    start: 0x17000,
    end: 0x187FF,
};
pub const TANGUT_COMPONENTS: UnicodeRange = UnicodeRange {
    name: "Tangut Components",
    start: 0x18800,
    end: 0x18AFF,
};
pub const KHITAN_SMALL_SCRIPT: UnicodeRange = UnicodeRange {
    name: "Khitan Small Script",
    start: 0x18B00,
    end: 0x18CFF,
};
pub const TANGUT_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Tangut Supplement",
    start: 0x18D00,
    end: 0x18D8F,
};
pub const KANA_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Kana Supplement",
    start: 0x1B000,
    end: 0x1B0FF,
};
pub const KANA_EXTENDED_A: UnicodeRange = UnicodeRange {
    name: "Kana Extended-A",
    start: 0x1B100,
    end: 0x1B12F,
};
pub const SMALL_KANA_EXTENSION: UnicodeRange = UnicodeRange {
    name: "Small Kana Extension",
    start: 0x1B130,
    end: 0x1B16F,
};
pub const NUSHU: UnicodeRange = UnicodeRange {
    name: "Nushu",
    start: 0x1B170,
    end: 0x1B2FF,
};
pub const DUPLOYAN: UnicodeRange = UnicodeRange {
    name: "Duployan",
    start: 0x1BC00,
    end: 0x1BC9F,
};
pub const SHORTHAND_FORMAT_CONTROLS: UnicodeRange = UnicodeRange {
    name: "Shorthand Format Controls",
    start: 0x1BCA0,
    end: 0x1BCAF,
};
pub const BYZANTINE_MUSICAL_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Byzantine Musical Symbols",
    start: 0x1D000,
    end: 0x1D0FF,
};
pub const MUSICAL_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Musical Symbols",
    start: 0x1D100,
    end: 0x1D1FF,
};
pub const ANCIENT_GREEK_MUSICAL_NOTATION: UnicodeRange = UnicodeRange {
    name: "Ancient Greek Musical Notation",
    start: 0x1D200,
    end: 0x1D24F,
};
pub const MAYAN_NUMERALS: UnicodeRange = UnicodeRange {
    name: "Mayan Numerals",
    start: 0x1D2E0,
    end: 0x1D2FF,
};
pub const TAI_XUAN_JING_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Tai Xuan Jing Symbols",
    start: 0x1D300,
    end: 0x1D35F,
};
pub const COUNTING_ROD_NUMERALS: UnicodeRange = UnicodeRange {
    name: "Counting Rod Numerals",
    start: 0x1D360,
    end: 0x1D37F,
};
pub const MATHEMATICAL_ALPHANUMERIC_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Mathematical Alphanumeric Symbols",
    start: 0x1D400,
    end: 0x1D7FF,
};
pub const SUTTON_SIGN_WRITING: UnicodeRange = UnicodeRange {
    name: "Sutton SignWriting",
    start: 0x1D800,
    end: 0x1DAAF,
};
pub const GLAGOLITIC_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Glagolitic Supplement",
    start: 0x1E000,
    end: 0x1E02F,
};
pub const NYIAKENG_PUACHUE_HMONG: UnicodeRange = UnicodeRange {
    name: "Nyiakeng Puachue Hmong",
    start: 0x1E100,
    end: 0x1E14F,
};
pub const WANCHO: UnicodeRange = UnicodeRange {
    name: "Wancho",
    start: 0x1E2C0,
    end: 0x1E2FF,
};
pub const MENDE_KIKAKUI: UnicodeRange = UnicodeRange {
    name: "Mende Kikakui",
    start: 0x1E800,
    end: 0x1E8DF,
};
pub const ADLAM: UnicodeRange = UnicodeRange {
    name: "Adlam",
    start: 0x1E900,
    end: 0x1E95F,
};
pub const INDIC_SIYAQ_NUMBERS: UnicodeRange = UnicodeRange {
    name: "Indic Siyaq Numbers",
    start: 0x1EC70,
    end: 0x1ECBF,
};
pub const OTTOMAN_SIYAQ_NUMBERS: UnicodeRange = UnicodeRange {
    name: "Ottoman Siyaq Numbers",
    start: 0x1ED00,
    end: 0x1ED4F,
};
pub const ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Arabic Mathematical Alphabetic Symbols",
    start: 0x1EE00,
    end: 0x1EEFF,
};
pub const MAHJONG_TILES: UnicodeRange = UnicodeRange {
    name: "Mahjong Tiles",
    start: 0x1F000,
    end: 0x1F02F,
};
pub const DOMINO_TILES: UnicodeRange = UnicodeRange {
    name: "Domino Tiles",
    start: 0x1F030,
    end: 0x1F09F,
};
pub const PLAYING_CARDS: UnicodeRange = UnicodeRange {
    name: "Playing Cards",
    start: 0x1F0A0,
    end: 0x1F0FF,
};
pub const ENCLOSED_ALPHANUMERIC_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Enclosed Alphanumeric Supplement",
    start: 0x1F100,
    end: 0x1F1FF,
};
pub const ENCLOSED_IDEOGRAPHIC_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Enclosed Ideographic Supplement",
    start: 0x1F200,
    end: 0x1F2FF,
};
pub const MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS: UnicodeRange = UnicodeRange {
    name: "Miscellaneous Symbols and Pictographs",
    start: 0x1F300,
    end: 0x1F5FF,
};
pub const EMOTICONS: UnicodeRange = UnicodeRange {
    name: "Emoticons",
    start: 0x1F600,
    end: 0x1F64F,
};
pub const ORNAMENTAL_DINGBATS: UnicodeRange = UnicodeRange {
    name: "Ornamental Dingbats",
    start: 0x1F650,
    end: 0x1F67F,
};
pub const TRANSPORT_AND_MAP_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Transport and Map Symbols",
    start: 0x1F680,
    end: 0x1F6FF,
};
pub const ALCHEMICAL_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Alchemical Symbols",
    start: 0x1F700,
    end: 0x1F77F,
};
pub const GEOMETRIC_SHAPES_EXTENDED: UnicodeRange = UnicodeRange {
    name: "Geometric Shapes Extended",
    start: 0x1F780,
    end: 0x1F7FF,
};
pub const SUPPLEMENTAL_ARROWS_C: UnicodeRange = UnicodeRange {
    name: "Supplemental Arrows-C",
    start: 0x1F800,
    end: 0x1F8FF,
};
pub const SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS: UnicodeRange = UnicodeRange {
    name: "Supplemental Symbols and Pictographs",
    start: 0x1F900,
    end: 0x1F9FF,
};
pub const CHESS_SYMBOLS: UnicodeRange = UnicodeRange {
    name: "Chess Symbols",
    start: 0x1FA00,
    end: 0x1FA6F,
};
pub const SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A: UnicodeRange = UnicodeRange {
    name: "Symbols and Pictographs Extended-A",
    start: 0x1FA70,
    end: 0x1FAFF,
};
pub const SYMBOLS_FOR_LEGACY_COMPUTING: UnicodeRange = UnicodeRange {
    name: "Symbols for Legacy Computing",
    start: 0x1FB00,
    end: 0x1FBFF,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B: UnicodeRange = UnicodeRange {
    name: "CJK Unified Ideographs Extension B",
    start: 0x20000,
    end: 0x2A6DF,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C: UnicodeRange = UnicodeRange {
    name: "CJK Unified Ideographs Extension C",
    start: 0x2A700,
    end: 0x2B73F,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D: UnicodeRange = UnicodeRange {
    name: "CJK Unified Ideographs Extension D",
    start: 0x2B740,
    end: 0x2B81F,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E: UnicodeRange = UnicodeRange {
    name: "CJK Unified Ideographs Extension E",
    start: 0x2B820,
    end: 0x2CEAF,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F: UnicodeRange = UnicodeRange {
    name: "CJK Unified Ideographs Extension F",
    start: 0x2CEB0,
    end: 0x2EBEF,
};
pub const CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "CJK Compatibility Ideographs Supplement",
    start: 0x2F800,
    end: 0x2FA1F,
};
pub const CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G: UnicodeRange = UnicodeRange {
    name: "CJK Unified Ideographs Extension G",
    start: 0x30000,
    end: 0x3134F,
};
pub const TAGS: UnicodeRange = UnicodeRange {
    name: "Tags",
    start: 0xE0000,
    end: 0xE007F,
};
pub const VARIATION_SELECTORS_SUPPLEMENT: UnicodeRange = UnicodeRange {
    name: "Variation Selectors Supplement",
    start: 0xE0100,
    end: 0xE01EF,
};
pub const SUPPLEMENTARY_PRIVATE_USE_AREA_A: UnicodeRange = UnicodeRange {
    name: "Supplementary Private Use Area-A",
    start: 0xF0000,
    end: 0xFFFFF,
};
pub const SUPPLEMENTARY_PRIVATE_USE_AREA_B: UnicodeRange = UnicodeRange {
    name: "Supplementary Private Use Area-B",
    start: 0x100000,
    end: 0x10FFFF,
};

pub const ALL: [UnicodeRange; 308] = [
    BASIC_LATIN,
    LATIN_1_SUPPLEMENT,
    LATIN_EXTENDED_A,
    LATIN_EXTENDED_B,
    IPA_EXTENSIONS,
    SPACING_MODIFIER_LETTERS,
    COMBINING_DIACRITICAL_MARKS,
    GREEK_AND_COPTIC,
    CYRILLIC,
    CYRILLIC_SUPPLEMENT,
    ARMENIAN,
    HEBREW,
    ARABIC,
    SYRIAC,
    ARABIC_SUPPLEMENT,
    THAANA,
    N_KO,
    SAMARITAN,
    MANDAIC,
    SYRIAC_SUPPLEMENT,
    ARABIC_EXTENDED_A,
    DEVANAGARI,
    BENGALI,
    GURMUKHI,
    GUJARATI,
    ORIYA,
    TAMIL,
    TELUGU,
    KANNADA,
    MALAYALAM,
    SINHALA,
    THAI,
    LAO,
    TIBETAN,
    MYANMAR,
    GEORGIAN,
    HANGUL_JAMO,
    ETHIOPIC,
    ETHIOPIC_SUPPLEMENT,
    CHEROKEE,
    UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS,
    OGHAM,
    RUNIC,
    TAGALOG,
    HANUNOO,
    BUHID,
    TAGBANWA,
    KHMER,
    MONGOLIAN,
    UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED,
    LIMBU,
    TAI_LE,
    NEW_TAI_LUE,
    KHMER_SYMBOLS,
    BUGINESE,
    TAI_THAM,
    COMBINING_DIACRITICAL_MARKS_EXTENDED,
    BALINESE,
    SUNDANESE,
    BATAK,
    LEPCHA,
    OL_CHIKI,
    CYRILLIC_EXTENDED_C,
    GEORGIAN_EXTENDED,
    SUNDANESE_SUPPLEMENT,
    VEDIC_EXTENSIONS,
    PHONETIC_EXTENSIONS,
    PHONETIC_EXTENSIONS_SUPPLEMENT,
    COMBINING_DIACRITICAL_MARKS_SUPPLEMENT,
    LATIN_EXTENDED_ADDITIONAL,
    GREEK_EXTENDED,
    GENERAL_PUNCTUATION,
    SUPERSCRIPTS_AND_SUBSCRIPTS,
    CURRENCY_SYMBOLS,
    COMBINING_DIACRITICAL_MARKS_FOR_SYMBOLS,
    LETTERLIKE_SYMBOLS,
    NUMBER_FORMS,
    ARROWS,
    MATHEMATICAL_OPERATORS,
    MISCELLANEOUS_TECHNICAL,
    CONTROL_PICTURES,
    OPTICAL_CHARACTER_RECOGNITION,
    ENCLOSED_ALPHANUMERICS,
    BOX_DRAWING,
    BLOCK_ELEMENTS,
    GEOMETRIC_SHAPES,
    MISCELLANEOUS_SYMBOLS,
    DINGBATS,
    MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A,
    SUPPLEMENTAL_ARROWS_A,
    BRAILLE_PATTERNS,
    SUPPLEMENTAL_ARROWS_B,
    MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B,
    SUPPLEMENTAL_MATHEMATICAL_OPERATORS,
    MISCELLANEOUS_SYMBOLS_AND_ARROWS,
    GLAGOLITIC,
    LATIN_EXTENDED_C,
    COPTIC,
    GEORGIAN_SUPPLEMENT,
    TIFINAGH,
    ETHIOPIC_EXTENDED,
    CYRILLIC_EXTENDED_A,
    SUPPLEMENTAL_PUNCTUATION,
    CJK_RADICALS_SUPPLEMENT,
    KANGXI_RADICALS,
    IDEOGRAPHIC_DESCRIPTION_CHARACTERS,
    CJK_SYMBOLS_AND_PUNCTUATION,
    HIRAGANA,
    KATAKANA,
    BOPOMOFO,
    HANGUL_COMPATIBILITY_JAMO,
    KANBUN,
    BOPOMOFO_EXTENDED,
    CJK_STROKES,
    KATAKANA_PHONETIC_EXTENSIONS,
    ENCLOSED_CJK_LETTERS_AND_MONTHS,
    CJK_COMPATIBILITY,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A,
    YIJING_HEXAGRAM_SYMBOLS,
    CJK_UNIFIED_IDEOGRAPHS,
    YI_SYLLABLES,
    YI_RADICALS,
    LISU,
    VAI,
    CYRILLIC_EXTENDED_B,
    BAMUM,
    MODIFIER_TONE_LETTERS,
    LATIN_EXTENDED_D,
    SYLOTI_NAGRI,
    COMMON_INDIC_NUMBER_FORMS,
    PHAGS_PA,
    SAURASHTRA,
    DEVANAGARI_EXTENDED,
    KAYAH_LI,
    REJANG,
    HANGUL_JAMO_EXTENDED_A,
    JAVANESE,
    MYANMAR_EXTENDED_B,
    CHAM,
    MYANMAR_EXTENDED_A,
    TAI_VIET,
    MEETEI_MAYEK_EXTENSIONS,
    ETHIOPIC_EXTENDED_A,
    LATIN_EXTENDED_E,
    CHEROKEE_SUPPLEMENT,
    MEETEI_MAYEK,
    HANGUL_SYLLABLES,
    HANGUL_JAMO_EXTENDED_B,
    HIGH_SURROGATES,
    HIGH_PRIVATE_USE_SURROGATES,
    LOW_SURROGATES,
    PRIVATE_USE_AREA,
    CJK_COMPATIBILITY_IDEOGRAPHS,
    ALPHABETIC_PRESENTATION_FORMS,
    ARABIC_PRESENTATION_FORMS_A,
    VARIATION_SELECTORS,
    VERTICAL_FORMS,
    COMBINING_HALF_MARKS,
    CJK_COMPATIBILITY_FORMS,
    SMALL_FORM_VARIANTS,
    ARABIC_PRESENTATION_FORMS_B,
    HALFWIDTH_AND_FULLWIDTH_FORMS,
    SPECIALS,
    LINEAR_B_SYLLABARY,
    LINEAR_B_IDEOGRAMS,
    AEGEAN_NUMBERS,
    ANCIENT_GREEK_NUMBERS,
    ANCIENT_SYMBOLS,
    PHAISTOS_DISC,
    LYCIAN,
    CARIAN,
    COPTIC_EPACT_NUMBERS,
    OLD_ITALIC,
    GOTHIC,
    OLD_PERMIC,
    UGARITIC,
    OLD_PERSIAN,
    DESERET,
    SHAVIAN,
    OSMANYA,
    OSAGE,
    ELBASAN,
    CAUCASIAN_ALBANIAN,
    LINEAR_A,
    CYPRIOT_SYLLABARY,
    IMPERIAL_ARAMAIC,
    PALMYRENE,
    NABATAEAN,
    HATRAN,
    PHOENICIAN,
    LYDIAN,
    MEROITIC_HIEROGLYPHS,
    MEROITIC_CURSIVE,
    KHAROSHTHI,
    OLD_SOUTH_ARABIAN,
    OLD_NORTH_ARABIAN,
    MANICHAEAN,
    AVESTAN,
    INSCRIPTIONAL_PARTHIAN,
    INSCRIPTIONAL_PAHLAVI,
    PSALTER_PAHLAVI,
    OLD_TURKIC,
    OLD_HUNGARIAN,
    HANIFI_ROHINGYA,
    RUMI_NUMERAL_SYMBOLS,
    YEZIDI,
    OLD_SOGDIAN,
    SOGDIAN,
    CHORASMIAN,
    ELYMAIC,
    BRAHMI,
    KAITHI,
    SORA_SOMPENG,
    CHAKMA,
    MAHAJANI,
    SHARADA,
    SINHALA_ARCHAIC_NUMBERS,
    KHOJKI,
    MULTANI,
    KHUDAWADI,
    GRANTHA,
    NEWA,
    TIRHUTA,
    SIDDHAM,
    MODI,
    MONGOLIAN_SUPPLEMENT,
    TAKRI,
    AHOM,
    DOGRA,
    WARANG_CITI,
    DIVES_AKURU,
    NANDINAGARI,
    ZANABAZAR_SQUARE,
    SOYOMBO,
    PAU_CIN_HAU,
    BHAIKSUKI,
    MARCHEN,
    MASARAM_GONDI,
    GUNJALA_GONDI,
    MAKASAR,
    LISU_SUPPLEMENT,
    TAMIL_SUPPLEMENT,
    CUNEIFORM,
    CUNEIFORM_NUMBERS_AND_PUNCTUATION,
    EARLY_DYNASTIC_CUNEIFORM,
    EGYPTIAN_HIEROGLYPHS,
    EGYPTIAN_HIEROGLYPH_FORMAT_CONTROLS,
    ANATOLIAN_HIEROGLYPHS,
    BAMUM_SUPPLEMENT,
    MRO,
    BASSA_VAH,
    PAHAWH_HMONG,
    MEDEFAIDRIN,
    MIAO,
    IDEOGRAPHIC_SYMBOLS_AND_PUNCTUATION,
    TANGUT,
    TANGUT_COMPONENTS,
    KHITAN_SMALL_SCRIPT,
    TANGUT_SUPPLEMENT,
    KANA_SUPPLEMENT,
    KANA_EXTENDED_A,
    SMALL_KANA_EXTENSION,
    NUSHU,
    DUPLOYAN,
    SHORTHAND_FORMAT_CONTROLS,
    BYZANTINE_MUSICAL_SYMBOLS,
    MUSICAL_SYMBOLS,
    ANCIENT_GREEK_MUSICAL_NOTATION,
    MAYAN_NUMERALS,
    TAI_XUAN_JING_SYMBOLS,
    COUNTING_ROD_NUMERALS,
    MATHEMATICAL_ALPHANUMERIC_SYMBOLS,
    SUTTON_SIGN_WRITING,
    GLAGOLITIC_SUPPLEMENT,
    NYIAKENG_PUACHUE_HMONG,
    WANCHO,
    MENDE_KIKAKUI,
    ADLAM,
    INDIC_SIYAQ_NUMBERS,
    OTTOMAN_SIYAQ_NUMBERS,
    ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS,
    MAHJONG_TILES,
    DOMINO_TILES,
    PLAYING_CARDS,
    ENCLOSED_ALPHANUMERIC_SUPPLEMENT,
    ENCLOSED_IDEOGRAPHIC_SUPPLEMENT,
    MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS,
    EMOTICONS,
    ORNAMENTAL_DINGBATS,
    TRANSPORT_AND_MAP_SYMBOLS,
    ALCHEMICAL_SYMBOLS,
    GEOMETRIC_SHAPES_EXTENDED,
    SUPPLEMENTAL_ARROWS_C,
    SUPPLEMENTAL_SYMBOLS_AND_PICTOGRAPHS,
    CHESS_SYMBOLS,
    SYMBOLS_AND_PICTOGRAPHS_EXTENDED_A,
    SYMBOLS_FOR_LEGACY_COMPUTING,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_E,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_F,
    CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT,
    CJK_UNIFIED_IDEOGRAPHS_EXTENSION_G,
    TAGS,
    VARIATION_SELECTORS_SUPPLEMENT,
    SUPPLEMENTARY_PRIVATE_USE_AREA_A,
    SUPPLEMENTARY_PRIVATE_USE_AREA_B,
];
