use std::collections::HashMap;

pub struct Lexeme<'a> {
    lemma: &'a str,
    categories: Vec<&'a str>,
}

#[derive(Default)]
pub struct Lexicon<'a> {
    pub lexemes: HashMap<&'a str, &'a Lexeme<'a>>
}

impl Lexicon<'_> {
    fn insert_lexeme(&self) {}
}

#[cfg(test)]
mod tests {}
