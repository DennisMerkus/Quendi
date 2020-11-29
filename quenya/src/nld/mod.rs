use crate::lexicon::{Lexicon, Lexeme};
use std::path::Path;
use wn::multi::parse::{parse_multilingual_wordnet_file, Entry, Lemma};
use std::fs::File;

pub fn load_lexicon() -> Lexicon {
    let path = Path::new("C:\\Users\\Dennis\\Dev\\Quenya\\dict\\nld\\wn-data-nld.tab");

    let entries = parse_multilingual_wordnet_file(File::open(&path).unwrap());

    let mut lexicon = Lexicon::new();

    for entry in entries {
        match entry {
            Entry::Lemma(lemma) => {
                let lexeme = Lexeme {
                    lemma: lemma.lemma,
                    pos: lemma.pos,
                };

                lexicon.add_lexeme(lexeme);
            }
            _ => {}
        }
    }

    lexicon
}
