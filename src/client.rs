use reqwest;
use common::api_feed::{ApiRequest, SecurityType};

pub struct HttpClient {
    client: reqwest::Client
}

impl HttpClient {
    pub fn new() -> HttpClient {
        HttpClient {
            client: reqwest::Client::new()
        }
    }

    pub async fn send(&self, req: ApiRequest) 
    -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let res = self.client.get(req.endpoint)
            .send()
            .await?;
        
        Ok(res)
    }
}