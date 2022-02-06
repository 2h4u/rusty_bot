pub trait Parser<T> {
    fn parse(&self, conent_to_parse: &str) -> Vec<T>;
}
