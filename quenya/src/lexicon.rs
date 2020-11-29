use std::collections::HashMap;
use wn::pos::{Part, as_char};


pub struct Lexeme {
    pub lemma: String,
    pub pos: Part,
}

#[derive(Default)]
pub struct Lexicon {
    lexemes: HashMap<String, Lexeme>
}

fn lexeme_key(lemma: &str, pos: Part) -> String {
    format!("{}:{}", lemma, as_char(pos))
}

impl Lexicon {
    pub fn new() -> Lexicon {
        Lexicon {
            lexemes: HashMap::new(),
        }
    }

    pub fn add_lexeme(&mut self, lexeme: Lexeme) {
        let key = lexeme_key(lexeme.lemma.as_str(), lexeme.pos);

        self.lexemes.insert(key, lexeme);
    }

    pub fn find_lexeme_by_lemma(&self, lemma: &str, pos: Part) -> Result<&Lexeme, ()> {
        let key = lexeme_key(lemma, pos);

        match self.lexemes.get(&key) {
            Some(lexeme) => Ok(lexeme),
            None => Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_err_when_lemma_is_not_found() {
        let lexicon = Lexicon::new();

        assert_eq!(lexicon.find_lexeme_by_lemma("search", Part::Verb).is_err(), true);
    }

    #[test]
    fn it_returns_lexemes_that_were_added() {
        let mut lexicon = Lexicon::new();

        let lexeme = Lexeme {
            lemma: String::from("search"),
            pos: Part::Verb,
        };

        lexicon.add_lexeme(lexeme);

        assert_eq!(lexicon.find_lexeme_by_lemma("search", Part::Verb).unwrap().lemma, String::from("search"));
    }
}
