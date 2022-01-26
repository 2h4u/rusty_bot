use std::fmt::Display;

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

fn main() {
    CustomResponse::print_response("https://www.example.com/")
}
