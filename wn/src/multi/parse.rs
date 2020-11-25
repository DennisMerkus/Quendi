use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};

use crate::pos::{Part, parse_pos};

#[derive(Debug, PartialEq)]
struct Lemma<'a> {
    language: &'a str,
    offset: &'a str,
    pos: Part,
    lemma: &'a str,
}

#[derive(Debug, PartialEq)]
struct Definition<'a> {
    language: &'a str,
    offset: &'a str,
    pos: Part,
    sid: i8,
    definition: &'a str,
}

#[derive(Debug, PartialEq)]
struct Example<'a> {
    language: &'a str,
    offset: &'a str,
    pos: Part,
    example: &'a str,
}

#[derive(Debug, PartialEq)]
enum Entry<'a> {
    Definition(Definition<'a>),
    Example(Example<'a>),
    Lemma(Lemma<'a>),
}

fn parse_multilingual_wordnet_line(line: &str) -> Entry {
    let regex = Regex::new(r"^(?P<offset>\d{8})-(?P<pos>[nvars])\s(?P<language>\w{3}):(?P<type>lemma|def|exe)\s((?P<sid>\d{1})\s)?(?P<content>.+)\s*$").unwrap();

    let captures = regex.captures(line).unwrap();

    let language = captures.name("language").unwrap().as_str();
    let offset = captures.name("offset").unwrap().as_str();
    let pos = parse_pos(&captures["pos"]);
    let content = captures.name("content").unwrap().as_str();

    let line_type = &captures["type"];

    if line_type == "lemma" {
        Entry::Lemma(Lemma {
            language,
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

// fn parse_multilingual_wordnet_file(file: File) -> Vec<Entry<'static>> {
//     let mut entries: Vec<Entry> = Vec::new();
// 
//     for line in BufReader::new(file).lines() {
//         if let Ok(line) = line {
//             entries.push(parse_multilingual_wordnet_line(line.as_str()))
//         }
//     }
// 
//     entries
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_extracts_indonesian_lemma() {
        let line = "00018158-v	ind:lemma	membubung\n";

        let lemma = parse_multilingual_wordnet_line(line);

        assert_eq!(lemma, Entry::Lemma(Lemma {
            lemma: "membubung",
            language: "ind",
            offset: "00018158",
            pos: Part::Verb,
        }))
    }

    #[test]
    fn it_extract_indonesian_definition() {
        let line = "00006024-n	ind:def	0	organisme yang tergantung pada zat organik kompleks untuk gizi\n";

        let definition = parse_multilingual_wordnet_line(line);

        assert_eq!(definition, Entry::Definition(Definition {
            sid: 0,
            definition: "organisme yang tergantung pada zat organik kompleks untuk gizi",
            language: "ind",
            pos: Part::Noun,
            offset: "00006024",
        }))
    }

    // #[test]
    // fn it_reads_indonesian_wordnet_file() {
    //     let path = Path::new("dict/msa/wn-data-ind.tab");
    // 
    //     let _entries = parse_multilingual_wordnet_file(File::open(path).unwrap());
    // }
}
