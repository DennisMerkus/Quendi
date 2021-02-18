use std::collections::{HashSet, HashMap};
use crate::ara::aramorph::dict::{Entry, Dicts};
use crate::ara::aramorph::table::Tables;

mod buckwalter;

mod dict;
mod table;

struct Solution {
    vocalization: String,
    lemma: String,
    pos: String,
    gloss: String,
}

pub fn analyze(word: String, dict: Dicts, table: Tables) -> Vec<Solution> {
    let mut solutions: Vec<Solution> = Vec::new();

    for segmentation in segment_word(word) {
        if dict.prefixes.contains_key(&segmentation.prefix) && dict.stems.contains_key(&segmentation.stem) && dict.suffixes.contains_key(&segmentation.suffix) {
            for prefix in dict.prefixes.get(&segmentation.prefix).unwrap() {
                for stem in dict.stems.get(&segmentation.stem).unwrap() {
                    if table.AB.contains(format!("{} {}", prefix.category, stem.category).as_str()) {
                        for suffix in dict.suffixes.get(&segmentation.suffix).unwrap() {
                            if table.AC.contains(format!("{} {}", prefix.category, suffix.category).as_str()) && table.BC.contains(format!("{}, {}", stem.category, suffix.category).as_str()) {
                                solutions.push(Solution {
                                    vocalization: format!("{}{}{}", prefix.vocabulary, stem.vocabulary, suffix.vocabulary),
                                    pos: format!("{}{}{}", prefix.pos, stem.pos, suffix.pos),
                                    gloss: format!("{}{}{}", prefix.gloss, stem.gloss, suffix.gloss),
                                    lemma: match &stem.lemma {
                                        Some(lemma) => String::from(lemma),
                                        None => panic!("Stem {} did not have associated lemmaID", stem.entry),
                                    },
                                })
                            }
                        }
                    }
                }
            }
        }
    }

    solutions
}

struct SegmentedWord {
    prefix: String,
    stem: String,
    suffix: String,
}

fn segment_word(word: String) -> Vec<SegmentedWord> {
    let mut prefix_length = 0;
    let mut suffix_length = 0;

    let mut segmentations: Vec<SegmentedWord> = Vec::new();

    while prefix_length <= 4 {
        let prefix = &word[..prefix_length];
        let mut stem_length = prefix.len() - word.len();

        suffix_length = 0;

        while stem_length >= 1 && suffix_length <= 6 {
            let stem = &word[prefix_length..(prefix_length + stem_length)];
            let suffix = &word[(prefix_length + stem_length)..];

            segmentations.push(SegmentedWord {
                prefix: String::from(prefix),
                stem: String::from(stem),
                suffix: String::from(suffix),
            });

            stem_length -= 1;
            suffix_length += 1;
        }
    }

    segmentations
}
