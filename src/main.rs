extern crate dotenv;
use dotenv::dotenv;

use env_logger;
use log::{info};

mod binance;
use binance::BinanceFeed;

mod client;
use client::HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let feed = BinanceFeed::new();
    let client = HttpClient::new();
    info!("{:?}", feed.system_status());
    info!("{:?}", feed.depth(&String::from("BTCUSDT")));
    let res = client.send(feed.system_status()).await?;
    info!("{:?}", res.text().await?);
    let binance_feed: &BinanceFeed = 
        match feed.as_any().downcast_ref::<BinanceFeed>() {
            Some(b) => b,
            None => panic!("Cannot downcast to BinanceFeed.")
    };
    
    Ok(())
}
