pub struct Token<'a> {
    raw: &'a str,
    lemma: &'static str,
}

pub struct AnnotatedText<'a> {
    tokens: &'a [Token<'a>],
}
