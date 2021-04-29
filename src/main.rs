extern crate common;
use common::config::ConfigLoader;

mod binance;
use binance::BinanceFeed;

use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let feed = BinanceFeed::new();
    feed.init();
    let client = reqwest::Client::new();
    let res = client.get("https://testnet.binance.vision/api/v3/time")
        .send()
        .await?;
    println!("{:?}", res);
    Ok(())
}
