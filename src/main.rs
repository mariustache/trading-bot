use reqwest;

mod protos;
use protos::common::{RequestHeader};
use protos::market_data::{ServerTimeRequest};

pub trait RequestPrinter {
    fn print(&self) -> String;
}

impl RequestPrinter for RequestHeader {
    fn print(&self) -> String {
        String::from(self.get_endpoint())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut header = RequestHeader::new();
    header.set_field_type("GET".to_string());
    header.set_endpoint("/api/v3/time".to_string());
    println!("{}", header.print());
    let mut request = ServerTimeRequest::new();
    request.set_header(header);

    let client = reqwest::Client::new();
    let res = client.get("https://testnet.binance.vision/api/v3/time")
        .send()
        .await?;
    println!("{:?}", res);
    Ok(())
}
