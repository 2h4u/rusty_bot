use crate::parsers::Parser;
use crate::parsers::ParserResult;
use scraper::Html;
use scraper::Selector;

pub struct ExampleParser {}

impl Parser<ExampleParserResult> for ExampleParser {
    fn parse(&self, html: &str) -> Vec<ExampleParserResult> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(r#"div[class="content"]"#).unwrap();
        let selection = document.select(&selector);

        selection
            .map(|item| ExampleParserResult {
                text: item.inner_html(),
            })
            .collect::<Vec<ExampleParserResult>>()
    }
}

pub struct ExampleParserResult {
    text: String,
}

impl ExampleParserResult {
    fn get_text(&self) -> &String {
        &self.text
    }
}

impl ParserResult for ExampleParserResult {}

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

        let parser = ExampleParser {};
        let items = parser
            .parse(html)
            .iter()
            .map(|parser_result| String::from(parser_result.get_text()))
            .collect::<Vec<String>>();

        assert!(items.len() == 3);
        assert!(items.contains(&String::from("content1")));
        assert!(items.contains(&String::from("content2")));
        assert!(items.contains(&String::from("content3")));
    }
}
