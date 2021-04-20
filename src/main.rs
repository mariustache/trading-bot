use reqwest;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let binance_testnet = String::from("https://testnet.binance.vision");
    let endpoint = String::from("/api/v3/exchangeInfo");
    let payload = binance_testnet + &endpoint;
    println!("{:?}", payload);
    let mut res = reqwest::get(&payload)
        .await?
        .text()
        .await?;
    println!("{:?}", res);

    Ok(())
}
