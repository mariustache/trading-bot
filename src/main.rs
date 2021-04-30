mod binance;
use binance::BinanceFeed;

use common::api_feed::ApiFeed;

use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let feed = BinanceFeed::new();
    let is_up = feed.system_status();
    let client = reqwest::Client::new();
    let res = client.get("https://testnet.binance.vision/api/v3/time")
        .send()
        .await?;
    println!("{:?}", res);
    Ok(())
}
