use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap};
use regex::Regex;

pub struct Entry {
    pub entry: String,
    pub vocabulary: String,
    pub category: String,
    pub gloss: String,
    pub pos: String,
    pub lemma: Option<String>,
}

pub struct Dicts {
    pub prefixes: HashMap<String, Vec<Entry>>,
    pub stems: HashMap<String, Vec<Entry>>,
    pub suffixes: HashMap<String, Vec<Entry>>,
}

fn load_dictionary(path: &str) -> HashMap<String, Vec<Entry>> {
    let path = Path::new(path);

    let mut file = match File::open(&path) {
        Err(error) => panic!("Could not open {}: {}", path.display(), error),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let mut entries: HashMap<String, Vec<Entry>> = HashMap::new();

    let mut currentLemmaId: Option<String> = None;
    let mut seenLemmaIds: HashSet<String> = HashSet::new();

    let posRegex = Regex::new(r"<pos>(.+)</pos>").unwrap();

    for line in reader.lines() {
        if let Ok(line) = line {
            println!("{}", line);

            if is_lemma_definition(&line) {
                let lemma = String::from(&line.as_str()[3..]);

                if seenLemmaIds.contains(&lemma) {
                    panic!("lemmaID {} is not unique", lemma);
                }

                seenLemmaIds.insert(String::from(&lemma));
                currentLemmaId = Some(lemma);
            } else if is_comment(&line) {
                // Ignore comments
            } else {
                // Check if line has 4 fields (3 tabs)
                let tabs: u32 = line.chars().map(|x| if x == '\t' { 1 } else { 0 }).sum();

                if tabs != 3 {
                    panic!("Entry does not have 4 fields\n{}", line);
                }

                let mut fields = line.splitn(4, '\t');
                let entry = fields.next().unwrap();
                let vocabulary = fields.next().unwrap();
                let category = fields.next().unwrap();
                let glossPOS = fields.next().unwrap();
                let mut pos = String::new();

                if posRegex.is_match(glossPOS) {
                    pos = String::from(posRegex.captures_iter(glossPOS).next().unwrap().get(1).unwrap().as_str());
                } else {
                    // Deduce POS
                    if category.starts_with("Pref-0") || category.starts_with("Suff-0") {
                        pos = String::new();
                    } else if category.starts_with('F') {
                        pos = vocabulary.to_owned() + "/FUNC_WORD";
                    } else if category.starts_with("IV") {
                        pos = vocabulary.to_owned() + "/VERB_IMPERFECT";
                    } else if category.starts_with("PV") {
                        pos = vocabulary.to_owned() + "/VERB_PERFECT";
                    } else if category.starts_with("CV") {
                        pos = vocabulary.to_owned() + "/VERB_IMPERATIVE";
                    } else if category.starts_with('N') && vocabulary.starts_with(|c| c >= 'A' && c <= 'Z') {
                        pos = vocabulary.to_owned() + "/NOUN_PROP";
                    } else if category.starts_with('N') && vocabulary.ends_with("iy~") {
                        pos = vocabulary.to_owned() + "/NOUN"
                    } else if category.starts_with('N') {
                        pos = vocabulary.to_owned() + "/NOUN"
                    } else {
                        panic!("No POS can be deduced for {}", line);
                    }
                }

                let key = String::from(entry);
                let entry = Entry {
                    entry: String::from(entry),
                    category: String::from(category),
                    gloss: clean_gloss(String::from(glossPOS)),
                    pos: String::from(pos),
                    vocabulary: String::from(vocabulary),
                    lemma: currentLemmaId.to_owned(),
                };

                match entries.get(&key) {
                    Some(existing) => {
                        existing.push(entry);
                    },
                    None => {
                        entries.insert(key, vec!(entry));
                    }
                }
            }
        }
    }

    entries
}

fn is_lemma_definition(line: &String) -> bool {
    line.starts_with(";; ")
}

fn is_comment(line: &String) -> bool {
    line.starts_with(';')
}

fn clean_gloss(gloss: String) -> String {
    // TODO: Implement.
    gloss
}

fn remove_kashida(text: &String) {
    str::replace(text, '\u{0640}', "");
}

fn remove_fatHatAn(text: &String) -> String {
    text.chars().filter(|c| match c {
        '\u{064B}'..='\u{0652}' => false,
        _ => true
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_recognizes_comment() {
        let line = String::from("; This is a comment.");

        assert_eq!(is_comment(&line), true);
    }

    #[test]
    fn it_recognizes_non_comment() {
        let line = String::from("y	ya	IVPref-hw-ya	he/it <pos>ya/IV3MS+</pos>");

        assert_eq!(is_comment(&line), false);
    }
}