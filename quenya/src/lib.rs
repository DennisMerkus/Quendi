mod lexicon;
mod eng;
pub mod jpn;
pub mod nld;
pub mod uxy;
pub mod text;

mod omni;

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
