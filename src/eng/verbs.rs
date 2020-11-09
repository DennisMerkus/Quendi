// Determining whether a word is a form of a verb

// trait Conjugator {
//     fn is_irregular_verb(word: &str) -> bool;
//     fn is_verb_form(word: &str) -> bool;
// }

// 1. Is it a form of an irregular verb?
// 2. Does it have the pattern of a verb form and does the base match with a lemma?


fn is_present_participle(word: &str) -> Result<&str, ()> {
    if word.ends_with("ing") {
        return Ok(&word[..word.len() - 3]);
    }

    Err(())
}

fn is_past_simple(word: &str) -> Result<&str, ()> {
    if word.ends_with("ed") {
        return Ok(&word[..word.len() - 2]);
    }

    Err(())
}

fn is_present_simple(word: &str) -> Result<&str, ()> {
    if word.ends_with("s") {
        return Ok(&word[..word.len() - 1]);
    }

    Err(())
}

fn make_present_simple(word: &str) -> &str {
    word + 's'
}

fn make_past_simple(word: &str) -> &str {
    word + "ed"
}

fn make_present_participle(word: &str) -> &str {
    word + "ing"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_allows_working() {
        let result = is_present_participle("working");

        assert_eq!(result, Ok("work"));
    }

    #[test]
    fn it_should_not_allow_works() {
        let result = is_present_participle("works");

        assert_eq!(result, Err(()));
    }

    #[test]
    fn it_should_allow_past_simple_of_work() {
        let result = is_past_simple("worked");

        assert_eq!(result, Ok("work"));
    }

    #[test]
    fn it_should_allow_past_simple_of_close() {
        let result = is_past_simple("closed");

        assert_eq!(result, Ok("close"));
    }

    #[test]
    fn it_should_allow_present_simple_of_work() {
        let result = is_present_simple("works");

        assert_eq!(result, Ok("work"));
    }
}

// Conjugating a verb

// @Laura does action :Close on @Window in timeframe now. Focus @Laura.
// In general, when something does an action on something, it's Subject + Verb + Direct Object.
// Person and Number match between the Subject and the Verb.
/*
event = {
    "actor": "@Laura",
    "action": "@Window:Close"
}

mode = {
    "focus": "@Laura" // Active voice
}
*/
