use crate::pos::{Part, parse_pos};
use regex::Regex;

#[derive(Debug, PartialEq)]
struct WordNetIdentifier<'a> {
    word: &'a str,
    lex_id: i8,
}

#[derive(Debug, PartialEq)]
struct WordNetData<'a> {
    synset_offset: &'a str,

    ss_type: Part,

    words: Vec<WordNetIdentifier<'a>>,

    gloss: &'a str,
}

fn parse_words(words_string: &str) -> Vec<WordNetIdentifier> {
    let word_regex = Regex::new(r"((?P<word>[\S]+)\s(?P<lex_id>[0-9A-Fa-f]))+").unwrap();

    let mut words: Vec<WordNetIdentifier> = Vec::new();
    
    for capture in word_regex.captures_iter(words_string) {
        words.push(WordNetIdentifier {
            word: capture.name("word").unwrap().as_str(),
            lex_id: i8::from_str_radix(capture.name("lex_id").unwrap().as_str(), 16).unwrap(),
        })
    }

    words
}

fn parse_wordnet_data(line: &str) -> WordNetData {
    let data_regex = Regex::new(r"^(?P<synset_offset>\d{8})\s(?P<lex_filenum>\d{2})\s(?P<ss_type>[nasrv])\s(?P<w_cnt>[0-9A-Fa-f]{2})\s(?P<words>([\S]+\s[0-9A-Fa-f]\s)+)(000|(?P<p_cnt>\d{3})\s(?P<pointers>([\S]+\s\d{8}\s[nasrv]\s[0-9A-Fa-f]{4}\s?)+))(\s(?P<f_cnt>\d{2})(?P<frames>(\s\+\s(?P<f_num>\d{2})\s(?P<w_num>[0-9A-Fa-f]{2}))+))?\s\|\s(?P<gloss>.*?)\s*$").unwrap();

    let captures = data_regex.captures(line).unwrap();

    let synset_offset = captures.name("synset_offset").unwrap().as_str();
    let ss_type = captures.name("ss_type").unwrap().as_str();
    let words = captures.name("words").unwrap().as_str();
    let gloss = captures.name("gloss").unwrap().as_str();

    WordNetData {
        synset_offset,
        ss_type: parse_pos(ss_type),
        words: parse_words(words),
        gloss,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extracts_noun_data() {
        let line = "11563715 20 n 02 Jungermanniaceae 0 family_Jungermanniaceae 0 002 @ 11558116 n 0000 #m 11563371 n 0000 | comprising the leafy members of the order Jungermanniales  ";

        let data = parse_wordnet_data(line);

        assert_eq!(data.synset_offset, "11563715");
        assert_eq!(data.ss_type, Part::Noun);
        assert_eq!(data.words.get(0).unwrap(), &WordNetIdentifier { word: "Jungermanniaceae", lex_id: 0 });
        assert_eq!(data.words.get(1).unwrap(), &WordNetIdentifier { word: "family_Jungermanniaceae", lex_id: 0 });
        assert_eq!(data.gloss, "comprising the leafy members of the order Jungermanniales");
    }
}
