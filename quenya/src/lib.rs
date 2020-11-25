mod lexicon;
mod eng;
mod jpn;
pub mod uxy;
pub mod text;

pub fn annotate(text: &str) -> &str {
    text
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
