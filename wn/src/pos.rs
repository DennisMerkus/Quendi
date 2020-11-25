#[derive(Debug, PartialEq)]
pub enum Part {
    Adjective,
    Adverb,
    Noun,
    Verb,
}

pub fn parse_pos(pos: &str) -> Part {
    match pos {
        "a" => Part::Adjective,
        "n" => Part::Noun,
        "r" => Part::Adverb,
        "v" => Part::Verb,
        _ => panic!()
    }
}