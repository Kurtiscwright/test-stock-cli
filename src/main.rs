use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
//use std::env;
use std::io;


mod keys;
pub use self::keys::api_key;

#[derive(Serialize, Deserialize, Debug)]
struct CompanyQuote {
    c: f64,
    h: f64,
    l: f64,
    o: f64,
    pc: f64,
    t: i128,
}

impl CompanyQuote {
    async fn get(symbol: &String, key: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, key
        );

        let url = Url::parse(&url)?;
        let res = reqwest::get(url).await?.json::<CompanyQuote>().await?;

        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let key = api_key::sandbox_key();
    let mut company = String::new();
    io::stdin().read_line(&mut company).expect("Please input a valid company symbol");
    let mut symbol: String = "AAPL".to_string();

    if company.len() < 2 {
        println!("Since you didn't specify a company symbol, it is defaulted to AAPL.");
        
    } else {
        symbol = company.clone().to_uppercase();
    }

    let res = CompanyQuote::get(&symbol, &key).await?;
    println!("{}'s current stock price is {}", &symbol[..symbol.len()-1], res.c);

    Ok(())
}