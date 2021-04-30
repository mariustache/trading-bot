mod binance;
use binance::BinanceFeed;

use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let feed = BinanceFeed::new();
    println!("{:?}", feed.system_status());
    let client = reqwest::Client::new();
    let res = client.get("https://testnet.binance.vision/api/v3/time")
        .send()
        .await?;
    println!("{:?}", res);
    Ok(())
}
