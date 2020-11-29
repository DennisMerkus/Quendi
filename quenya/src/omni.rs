use crate::lexicon::Lexicon;
use std::collections::HashMap;

struct Omniglot {
    lexicon: HashMap<String, Lexicon>
}

impl Omniglot {
    fn new() -> Omniglot {
        let mut omni = Omniglot {
            lexicon: HashMap::new(),
        };
        
        omni.lexicon.insert(String::from("nld"), crate::nld::load_lexicon());
        
        omni
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wn::pos::Part;

    #[test]
    fn it_should_create() {
        let omni = Omniglot::new();
        
        assert_eq!(omni.lexicon.get("nld").is_some(), true);
        
        assert_eq!(omni.lexicon.get("nld").unwrap().find_lexeme_by_lemma("zoeken", Part::Verb).is_ok(), true);
        assert_eq!(omni.lexicon.get("nld").unwrap().find_lexeme_by_lemma("3513dsags", Part::Verb).is_ok(), false);
    }
}
