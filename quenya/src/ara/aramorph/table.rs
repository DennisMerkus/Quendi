use std::collections::HashSet;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Tables {
    pub AB: HashSet<String>,
    pub AC: HashSet<String>,
    pub BC: HashSet<String>,
}

pub fn load_table(path: &String) -> HashSet<String> {
    let mut table = HashSet::new();

    let path = Path::new(path);

    let mut file = match File::open(&path) {
        Err(error) => panic!("Could not open {}: {}", path.display(), error),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            table.insert(line.trim().to_string());
        }
    }

    table
}
