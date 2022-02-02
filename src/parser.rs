use scraper::Html;
use scraper::Selector;

pub fn parse(html: &str) -> Vec<String> {

    
    // let document = Html::parse_document(html.as_str());
    // let selector = Selector::parse(r#"div[id="skip-to-resultlist"]"#).unwrap();

    vec![String::from("test123")]
}  

#[cfg(test)]
mod tests {
    use std::ops::RangeBounds;
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

        let items = parse(html);

        assert!(items.len() == 3);
        assert!(items.contains(&String::from("content1")));
        assert!(items.contains(&String::from("content2")));
        assert!(items.contains(&String::from("content3")));
    }
}