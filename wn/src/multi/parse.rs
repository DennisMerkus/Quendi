use regex::Regex;

use crate::pos::{Part, parse_pos};
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, PartialEq)]
struct Lemma {
    language: String,
    offset: String,
    pos: Part,
    lemma: String,
}

#[derive(Debug, PartialEq)]
struct Definition {
    language: String,
    offset: String,
    pos: Part,
    sid: i8,
    definition: String,
}

#[derive(Debug, PartialEq)]
struct Example {
    language: String,
    offset: String,
    pos: Part,
    example: String,
}

#[derive(Debug, PartialEq)]
enum Entry {
    Definition(Definition),
    Example(Example),
    Lemma(Lemma),
}

fn parse_multilingual_wordnet_line(line: &str) -> Entry {
    let regex = Regex::new(r"^(?P<offset>\d{8})-(?P<pos>[nvars])\s(?P<language>\w{3}):(?P<type>lemma|def|exe)\s((?P<sid>\d{1})\s)?(?P<content>.+)\s*$").unwrap();

    let captures = regex.captures(line).unwrap();

    let language = captures.name("language").unwrap().as_str().to_string();
    let offset = captures.name("offset").unwrap().as_str().to_string();
    let pos = parse_pos(&captures["pos"]);
    let content = captures.name("content").unwrap().as_str().to_string();

    let line_type = &captures["type"];

    if line_type == "lemma" {
        Entry::Lemma(Lemma {
            language: language,
            offset,
            pos,
            lemma: content,
        })
    } else if line_type == "def" {
        Entry::Definition(Definition {
            language,
            offset,
            pos,
            sid: captures.name("sid").unwrap().as_str().parse::<i8>().unwrap(),
            definition: content,
        })
    } else if line_type == "exe" {
        Entry::Example(Example {
            language,
            offset,
            pos,
            example: content,
        })
    } else {
        panic!()
    }
}

fn parse_multilingual_wordnet_file(file: File) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();

    for line in BufReader::new(file).lines() {
        if let Ok(test) = line {
            if test.starts_with('#') {
                continue;
            }

            entries.push(parse_multilingual_wordnet_line(test.as_str()))
        }
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_extracts_indonesian_lemma() {
        let line = "00018158-v	ind:lemma	membubung\n";

        let lemma = parse_multilingual_wordnet_line(line);

        assert_eq!(lemma, Entry::Lemma(Lemma {
            lemma: String::from("membubung"),
            language: String::from("ind"),
            offset: String::from("00018158"),
            pos: Part::Verb,
        }))
    }

    #[test]
    fn it_extract_indonesian_definition() {
        let line = "00006024-n	ind:def	0	organisme yang tergantung pada zat organik kompleks untuk gizi\n";

        let definition = parse_multilingual_wordnet_line(line);

        assert_eq!(definition, Entry::Definition(Definition {
            sid: 0,
            definition: String::from("organisme yang tergantung pada zat organik kompleks untuk gizi"),
            language: String::from("ind"),
            pos: Part::Noun,
            offset: String::from("00006024"),
        }))
    }

    #[test]
    fn it_extracts_japanese_lemmas() {
        let line = "00006610-r	jpn:lemma	著しく\n";

        let lemma = parse_multilingual_wordnet_line(line);

        assert_eq!(lemma, Entry::Lemma(Lemma {
            language: String::from("jpn"),
            offset: String::from("00006610"),
            pos: Part::Adverb,
            lemma: String::from("著しく"),
        }))
    }

    #[test]
    fn it_extracts_japanese_definitions() {
        let line = "10363445-n	jpn:def	0	気に留める人\n";

        let definition = parse_multilingual_wordnet_line(line);

        assert_eq!(definition, Entry::Definition(Definition {
            sid: 0,
            definition: String::from("気に留める人"),
            language: String::from("jpn"),
            pos: Part::Noun,
            offset: String::from("10363445"),
        }))
    }

    #[test]
    fn it_extract_japanese_examples() {
        let line = "01785341-a	jpn:exe	2	ロシアの最後の時間は、容赦ない確実性とともに訪れたようであった\n";

        let example = parse_multilingual_wordnet_line(line);

        assert_eq!(example, Entry::Example(Example {
            example: String::from("ロシアの最後の時間は、容赦ない確実性とともに訪れたようであった"),
            pos: Part::Adjective,
            offset: String::from("01785341"),
            language: String::from("jpn"),
        }))
    }

    #[test]
    fn it_reads_indonesian_wordnet_file() {
        let path = Path::new("C:\\Users\\Dennis\\Dev\\Quenya\\dict\\msa\\wn-data-ind-test.tab");

        let entries = parse_multilingual_wordnet_file(File::open(&path).unwrap());

        assert_eq!(entries.len(), 3);
        assert_eq!(*entries.get(0).unwrap(), Entry::Lemma(Lemma {
            offset: String::from("00001740"),
            lemma: String::from("berdaya"),
            pos: Part::Adjective,
            language: String::from("ind"),
        }))
    }
}
