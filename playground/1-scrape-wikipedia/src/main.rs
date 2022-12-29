use reqwest;
use std::error::Error;
use std::time::Duration;
use std::fs;
use html_parser::Dom;
use json;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let html = client
        .get("https://en.wikipedia.org/wiki/Lionel_Messi")
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;

    let json = Dom::parse(&html)?.to_json_pretty()?;
    let parsed = json::parse(&json).unwrap();
    // println!("Parsed {}", parsed);
    println!("Element {}", parsed["children"][0]);
    // fs::write("./txt.json", json).expect("Unable to write file");
    Ok(())
}