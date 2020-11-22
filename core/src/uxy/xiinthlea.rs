fn  word_from_srx(text: &str) -> &str {
    text
}

pub fn from_srx(text: &str) -> String {
    let words: Vec<&str> = text.split(' ').collect();
    
    let words: Vec<&str> = words.iter().map(|w| word_from_srx(w)).collect();

    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_converts_low_pitch() {
        let srx = ".ā";

        let result = from_srx(srx);

        assert_eq!(result, "ā1");
    }

    #[test]
    #[ignore]
    fn it_converts_rising_pitch() {
        let srx = "Su.n’a";

        let result = from_srx(srx);

        assert_eq!(result, "9su04na5");
    }

    #[test]
    #[ignore]
    fn it_converts_high_pitch() {
        let srx = "yu”";

        let result = from_srx(srx);

        assert_eq!(result, "yu3");
    }

    #[test]
    #[ignore]
    fn it_converts_multiple_syllables() {
        let srx = "p.i’t’an";

        let result = from_srx(srx);

        assert_eq!(result, "4pi5tan2");
    }

    #[test]
    #[ignore]
    fn it_converts_multiple_words() {
        let srx = "tya e Yii’ua";

        let result = from_srx(srx);

        assert_eq!(result, "tya0 7e0 9yii2ua0");
    }

    #[test]
    #[ignore]
    fn it_converts_more_syllables() {
        let srx = "tha’oa.se”";

        let result = from_srx(srx);

        assert_eq!(result, "tha2oa04se3");
    }

    #[test]
    #[ignore]
    fn it_converts_questions() {
        let srx = "o thlēng xii’xy.oa?";

        let result = from_srx(srx);

        assert_eq!(result, "o0 thlēng0 xii2xyoa1?");
    }
}
