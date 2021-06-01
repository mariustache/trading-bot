use reqwest;
use common::api_feed::HttpPayload;

pub struct HttpClient {
    client: reqwest::Client
}

impl HttpClient {
    pub fn new() -> HttpClient {
        HttpClient {
            client: reqwest::Client::new()
        }
    }

    pub async fn get(&self, req: HttpPayload) 
    -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let mut client = self.client.get(req.payload);
        if req.api_key != "" {
            client = client.header("X-MBX-APIKEY", req.api_key);
        }
        
        let res = client.send().await?;
        
        Ok(res)
    }

    pub async fn post(&self, req: HttpPayload)
    -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let mut client = self.client.post(req.payload);
        if req.api_key != "" {
            client = client.header("X-MBX-APIKEY", req.api_key);
        }
        
        let res = client.send().await?;
        
        Ok(res)
    }
}