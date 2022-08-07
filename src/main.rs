use json;
use reqwest;
use std::{env::args, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();

    let word = &args[1];

    let body = reqwest::get("https://api.dictionaryapi.dev/api/v2/entries/en/".to_owned() + word)
        .await?
        .text()
        .await?;

    let jswon = json::parse(&body).unwrap();

    for value in jswon[0]["meanings"].members() {
        println!(
            "{}",
            value["partOfSpeech"]
                .to_owned()
                .take_string()
                .unwrap()
                .to_uppercase()
        );
        println!("###########################################################");
        for definition in value["definitions"].members() {
            println!("{}", definition.pretty(2));
        }
        println!("###########################################################");
    }

    Ok(())
}
