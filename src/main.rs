#[macro_use]
extern crate dotenv_codegen;

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
    println!("{}", dotenv!("BINANCE_TESTNET_API_SECRET"));
    println!("{:?}", res.text().await?);
    let binance_feed: &BinanceFeed = 
        match feed.as_any().downcast_ref::<BinanceFeed>() {
            Some(b) => b,
            None => panic!("Cannot downcast to BinanceFeed.")
    };
    println!("{:?}", binance_feed.on_testnet());
    Ok(())
}
