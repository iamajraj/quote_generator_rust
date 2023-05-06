use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{io::Write, thread};

#[derive(Serialize, Deserialize)]
struct Quote {
    q: String,
    a: String,
}

#[derive(Serialize, Deserialize)]
struct Quotes(Vec<Quote>);

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let res = client
        .get("https://zenquotes.io/api/random")
        .send()
        .await
        .unwrap();

    let quote: Quotes = res.json().await.unwrap();

    println!("Quote by: {}\n", quote.0[0].a);
    for character in quote.0[0].q.as_bytes() {
        print!("{}", char::from(character.to_owned()));
        thread::sleep(Duration::from_millis(30));
        std::io::stdout().flush().unwrap();
    }
}
