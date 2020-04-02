use crate::traits::ToInt;
use whatlang::Script;

impl ToInt for Script {
    fn to_int(&self) -> i32 {
        match self {
            Script::Latin => 0,
            Script::Cyrillic => 1,
            Script::Arabic => 2,
            Script::Devanagari => 3,
            Script::Hiragana => 4,
            Script::Katakana => 5,
            Script::Ethiopic => 6,
            Script::Hebrew => 7,
            Script::Bengali => 8,
            Script::Georgian => 9,
            Script::Mandarin => 10,
            Script::Hangul => 11,
            Script::Greek => 12,
            Script::Kannada => 13,
            Script::Tamil => 14,
            Script::Thai => 15,
            Script::Gujarati => 16,
            Script::Gurmukhi => 17,
            Script::Telugu => 18,
            Script::Malayalam => 19,
            Script::Oriya => 20,
            Script::Myanmar => 21,
            Script::Sinhala => 22,
            Script::Khmer => 23,
        }
    }
}

fn script_from_int(i: i32) -> Script {
    match i {
        0 => Script::Latin,
        1 => Script::Cyrillic,
        2 => Script::Arabic,
        3 => Script::Devanagari,
        4 => Script::Hiragana,
        5 => Script::Katakana,
        6 => Script::Ethiopic,
        7 => Script::Hebrew,
        8 => Script::Bengali,
        9 => Script::Georgian,
        10 => Script::Mandarin,
        11 => Script::Hangul,
        12 => Script::Greek,
        13 => Script::Kannada,
        14 => Script::Tamil,
        15 => Script::Thai,
        16 => Script::Gujarati,
        17 => Script::Gurmukhi,
        18 => Script::Telugu,
        19 => Script::Malayalam,
        20 => Script::Oriya,
        21 => Script::Myanmar,
        22 => Script::Sinhala,
        23 => Script::Khmer,
        _ => panic!("Invalid Int Passed"),
    }
}
