use crate::parsers::Parser;
use scraper::Html;
use scraper::Selector;

pub struct HtmlParser {}

impl Parser<String> for HtmlParser {
    fn parse(&self, html: &str) -> Vec<String> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(r#"div[class="content"]"#).unwrap();
        let selection = document.select(&selector);

        selection
            .map(|item| item.inner_html())
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let html = r#"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <title>Hello, world!</title>
    <h1 class="foo">Hello, <i>world!</i></h1>
    <div class="content">content1</div>
    <div class="content">content2</div>
    <div class="content">content3</div>
"#;

        let parser = HtmlParser {};
        let items = parser.parse(html);

        assert!(items.len() == 3);
        assert!(items.contains(&String::from("content1")));
        assert!(items.contains(&String::from("content2")));
        assert!(items.contains(&String::from("content3")));
    }
}
