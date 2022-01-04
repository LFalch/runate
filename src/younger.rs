use super::{RuneSet, rune_set, first};

pub fn rune_sets() -> (RuneSet, RuneSet) {
    let long = rune_set!{
        'ᚠ' => 'f';
        'ᚢ' => 'u', 'ú', 'w', 'v', 'y', 'ý', 'ø', 'ǿ', 'o', 'ó';
        'ᚦ' => 'þ', 'ð';
        'ᚬ' => 'ą', 'ǫ', 'ö', 'ǽ', 'á', 'å';
        'ᚱ' => 'r';
        'ᚴ' => 'k', 'g', 'c';
        'ᚼ' => 'h';
        'ᚾ' => 'n';
        'ᛁ' => 'i', 'í', 'e', 'é', 'j';
        'ᛅ' => 'a', 'ę', 'æ';
        'ᛋ' => 's';
        'ᛏ' => 't', 'd';
        'ᛒ' => 'b', 'p';
        'ᛘ' => 'm';
        'ᛚ' => 'l';
        'ᛦ' => 'ʀ', 'z', 'R';
    };
    let short = rune_set!{
        'ᚠ' => 'f';
        'ᚢ' => 'u', 'ú', 'w', 'v', 'y', 'ý', 'ø', 'ǿ', 'o', 'ó';
        'ᚦ' => 'þ', 'ð';
        'ᚭ' => 'ą', 'ǫ', 'ö', 'ǽ', 'á', 'å';
        'ᚱ' => 'r';
        'ᚴ' => 'k', 'g', 'c';
        'ᚽ' => 'h';
        'ᚿ' => 'n';
        'ᛁ' => 'i', 'í', 'e', 'é', 'j';
        'ᛆ' => 'a', 'ę', 'æ';
        'ᛌ' => 's';
        'ᛐ' => 't', 'd';
        'ᛓ' => 'b', 'p';
        'ᛙ' => 'm';
        'ᛚ' => 'l';
        'ᛧ' => 'ʀ', 'z', 'R';
    };

    (long, short)
}
