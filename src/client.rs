use reqwest;

pub struct HttpClient {
    client: reqwest::Client
}

impl HttpClient {
    pub fn new() -> HttpClient {
        HttpClient {
            client: reqwest::Client::new()
        }
    }

    pub async fn send(&self, req: String) 
    -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        let res = self.client.get(req)
            .send()
            .await?;
        
        Ok(res)
    }
}