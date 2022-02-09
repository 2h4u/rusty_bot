pub trait Parser<T: ParserResult> {
    fn parse(&self, conent_to_parse: &str) -> Vec<T>;
}

pub trait ParserResult {}
