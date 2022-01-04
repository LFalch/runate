use std::{collections::HashMap};

mod younger;
mod medieval;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct YoungerFutharkReport {
    pub transliteration: String,
    pub short_staved: String,
    pub long_staved: String,
}

impl YoungerFutharkReport {
    pub fn new(s: &str) -> Self {
        let (long, short) = younger::rune_sets();

        let input = s
            .chars()
            .map(|c| {
                if let Some(&c) = long.rune_to_trans.get(&c).or(short.rune_to_trans.get(&c)) {
                    c
                } else {
                    c
                }
            });

        let short_staved = input
            .clone()
            .map(|c| *short.trans_to_rune.get(&c).unwrap_or(&c))
            .collect();

        let long_staved: String = input
            .map(|c| *long.trans_to_rune.get(&c).unwrap_or(&c))
            .collect();

        let transliteration = long_staved
            .chars()
            .map(|c| *long.rune_to_trans.get(&c).unwrap_or(&c))
            .collect();

        YoungerFutharkReport { transliteration, short_staved, long_staved }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MedievalRunesReport {
    pub transliteration: String,
    pub runes: String,
}

impl MedievalRunesReport {
    pub fn new(s: &str) -> Self {
        let set = medieval::rune_set();

        let input = s
            .chars()
            .map(|c| {
                if let Some(&c) = set.rune_to_trans.get(&c) {
                    c
                } else {
                    c
                }
            });

        let runes: String = input
            .map(|c| *set.trans_to_rune.get(&c).unwrap_or(&c))
            .collect();

        let transliteration = runes
            .chars()
            .map(|c| *set.rune_to_trans.get(&c).unwrap_or(&c))
            .collect();

        MedievalRunesReport { transliteration, runes }
    }
}

#[macro_export]
macro_rules! first {
    ($e:expr $(, $tail:expr)*) => {
        $e
    };
}

#[macro_export]
macro_rules! rune_set {
    ($(
        $($rc:expr),+ => $($tc:expr),+;
    )*) => {{
        use std::collections::HashMap;
        let mut rune_to_trans = HashMap::new();
        let mut trans_to_rune = HashMap::new();

        {
            $(
                let std_t = first!($($tc),*);
                $( rune_to_trans.insert($rc,  std_t); )*
                let std_r = first!($($rc),*);
                $( trans_to_rune.insert($tc,  std_r); )*
            )*
        }

        RuneSet {
            rune_to_trans,
            trans_to_rune
        }
    }};
}

pub struct RuneSet {
    rune_to_trans: HashMap<char, char>,
    trans_to_rune: HashMap<char, char>,
}
