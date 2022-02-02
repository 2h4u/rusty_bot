use std::fmt::Display;

use scraper::Html;
use scraper::Selector;

struct CustomResponse {
    text: String,
    version: reqwest::Version,
    headers: reqwest::header::HeaderMap,
    status_code: reqwest::StatusCode,
}

trait Shorten {
    fn shorten(&self) -> Self;
}

impl Display for CustomResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        let short_string = self.text.shorten();
        // Same as:
        // let short_string = &self.text[..50];
        write!(f, "{:?} - {} - {:?} - {}", self.version, self.status_code, self.headers, short_string)
    }
}

impl Shorten for String {
    fn shorten(&self) -> String {
        self[..50].to_string()
    }
}

fn get_response(url: &str) -> Result<CustomResponse, reqwest::Error> {
    let res = reqwest::blocking::get(url)?;
    let version: reqwest::Version = res.version();
    let headers = res.headers().clone();
    let status_code = res.status();
    let text = res.text()?;
    let my_response = CustomResponse {
        text: text,
        version: version,
        headers: headers,
        status_code: status_code,
    };
    Ok(my_response)
}

impl CustomResponse {
    fn print_response(url: &str) {
        match get_response(url) {
            Ok(x) => println!("{}", x),
            _ => panic!(),
        }
    }
}

fn parse(url: &str) {
    let res = reqwest::blocking::get(url);
    let html = res.unwrap().text().unwrap();
    let document = Html::parse_document(html.as_str());
    let selector = Selector::parse(r#"div[id="skip-to-resultlist"]"#).unwrap();
    let resultlist = document.select(&selector).next().unwrap();
    let selector2 = Selector::parse(r#"div[class="Box-sc-wfmb7k-0 fVCnsK"]"#).unwrap();
    let element = resultlist.select(&selector2).next().unwrap();
    let selector3 = Selector::parse(r#"h3[class="Text-sc-10o2fdq-0 kQBDHL"]"#).unwrap();
    let element_text = element.select(&selector3).next().unwrap();
    /*
    let test_value = element.children().next().unwrap().value();
    let test_unwrap = test_value.as_element().unwrap();
    println!("{:#?}", test_unwrap);
    println!("{:#?}", test_unwrap.name());
    println!("{:#?}", test_unwrap.classes);
    println!("{:#?}", test_unwrap.attrs);
    println!("{:#?}", test_unwrap.id);
    println!("{:#?}", test_unwrap.name);

     */

   // let element_value = element.value();

    println!("{:#?}", element_text.first_child().unwrap().value().as_text().unwrap());
    println!("#######################################################################################");

    println!("{:#?}", element.value());
    println!("#######################################################################################");
    println!("{:#?}", element.first_child().unwrap().first_child().unwrap().first_child().unwrap().value());
    println!("#######################################################################################");
    println!("{:#?}", element.html());
    println!("#######################################################################################");
}

fn main() {
    //CustomResponse::print_response("https://www.example.com/")

    parse("https://www.example.com/");
}
