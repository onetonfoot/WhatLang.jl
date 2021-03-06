use crate::traits::ToInt;
use whatlang::Lang;

impl ToInt for Lang {
    fn to_int(&self) -> i32 {
        *self as i32
    }
}

pub fn lang_from_int(i: &i32) -> Lang {
    match i {
        0 => Lang::Epo,
        1 => Lang::Eng,
        2 => Lang::Rus,
        3 => Lang::Cmn,
        4 => Lang::Spa,
        5 => Lang::Por,
        6 => Lang::Ita,
        7 => Lang::Ben,
        8 => Lang::Fra,
        9 => Lang::Deu,
        10 => Lang::Ukr,
        11 => Lang::Kat,
        12 => Lang::Arb,
        13 => Lang::Hin,
        14 => Lang::Jpn,
        15 => Lang::Heb,
        16 => Lang::Ydd,
        17 => Lang::Pol,
        18 => Lang::Amh,
        19 => Lang::Tir,
        20 => Lang::Jav,
        21 => Lang::Kor,
        22 => Lang::Nob,
        23 => Lang::Nno,
        24 => Lang::Dan,
        25 => Lang::Swe,
        26 => Lang::Fin,
        27 => Lang::Tur,
        28 => Lang::Nld,
        29 => Lang::Hun,
        30 => Lang::Ces,
        31 => Lang::Ell,
        32 => Lang::Bul,
        33 => Lang::Bel,
        34 => Lang::Mar,
        35 => Lang::Kan,
        36 => Lang::Ron,
        37 => Lang::Slv,
        38 => Lang::Hrv,
        39 => Lang::Srp,
        40 => Lang::Mkd,
        41 => Lang::Lit,
        42 => Lang::Lav,
        43 => Lang::Est,
        44 => Lang::Tam,
        45 => Lang::Vie,
        46 => Lang::Urd,
        47 => Lang::Tha,
        48 => Lang::Guj,
        49 => Lang::Uzb,
        50 => Lang::Pan,
        51 => Lang::Azj,
        52 => Lang::Ind,
        53 => Lang::Tel,
        54 => Lang::Pes,
        55 => Lang::Mal,
        56 => Lang::Hau,
        57 => Lang::Ori,
        58 => Lang::Mya,
        59 => Lang::Bho,
        60 => Lang::Tgl,
        61 => Lang::Yor,
        62 => Lang::Mai,
        63 => Lang::Orm,
        64 => Lang::Ibo,
        65 => Lang::Ceb,
        66 => Lang::Kur,
        67 => Lang::Mlg,
        68 => Lang::Skr,
        69 => Lang::Nep,
        70 => Lang::Sin,
        71 => Lang::Khm,
        72 => Lang::Tuk,
        73 => Lang::Som,
        74 => Lang::Nya,
        75 => Lang::Aka,
        76 => Lang::Zul,
        77 => Lang::Kin,
        78 => Lang::Hat,
        79 => Lang::Ilo,
        80 => Lang::Run,
        81 => Lang::Sna,
        82 => Lang::Uig,
        83 => Lang::Afr,
        _ => panic!("Unknown int passed for langauge"),
    }
}
