use reqwest::{blocking::ClientBuilder, header::HeaderMap};

const BASE_URL: &str = "https://adventofcode.com/2022/day/DAY_ID/input";

fn main() {
    let ciao = BASE_URL.replace("DAY_ID", 1.to_string().as_str());

    let mut headers = HeaderMap::new();
    headers.insert("Cookie", "session=".parse().unwrap());

    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();
    let input = client.get(ciao).send().unwrap().text().unwrap();

    println!("{input}");
}
