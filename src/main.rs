struct ResponseNew {
    text: String,
    version: reqwest::Version,
    statusCode: reqwest::StatusCode,
}

fn print_response() -> Result<(), reqwest::Error> {
    let res = reqwest::blocking::get("https://www.example.com/")?;
    let version: reqwest::Version = res.version();
    // let header = res.headers();
    let statusCode = res.status();
    let text = match res.text() {
        Ok(x) => x,
        _ => panic!(),
    };
    let custom_response = ResponseNew {
        text: text,
        version: version,
        statusCode: statusCode,
    };
    Ok(())
}

fn main() {
    print_response();
}
