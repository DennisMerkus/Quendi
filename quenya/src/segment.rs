pub fn segment(sentence: &str) -> Vec<&str> {
    return Vec::new();
}

#[cfg(test)]
mod tests {
    use crate::segment::segment;

    #[test]
    fn it_segments_a_sentence() {
        let sentence = "にほんごをはなすことができます";

        let segmented = segment(sentence);

        assert_eq!(segmented, vec!["にほんご", "を", "はなす", "こと", "が", "できます"]);
    }
}
