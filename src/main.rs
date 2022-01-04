use runate::{YoungerFutharkReport, MedievalRunesReport};
use std::env::args;

fn main() {
    let input = &args().skip(1).collect::<Vec<_>>().join(" ");
    let younger = YoungerFutharkReport::new(&input);
    let medieval = MedievalRunesReport::new(&input);

    println!("Younger futhark:\n");
    println!("Rune transliteration: {}", younger.transliteration);
    println!("Long-staved runes   : {}", younger.long_staved);
    println!("Short-staved runes  : {}", younger.short_staved);
    println!();
    println!("Medieval runes:\n");
    println!("Rune transliteration: {}", medieval.transliteration);
    println!("Attempt at runes    : {}", medieval.runes);
}
