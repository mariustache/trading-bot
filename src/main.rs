mod binance;
use binance::BinanceFeed;

mod client;
use client::HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let feed = BinanceFeed::new();
    println!("{:?}", feed.system_status());
    println!("{:?}", feed.coins_info());
    let client = HttpClient::new();
    let res = client.send(feed.system_status()).await?;
    println!("{:?}", res);
    Ok(())
}
