extern crate common;
use common::api_feed::{ApiFeed, ApiRequest};
use common::config::ConfigLoader;

const BINANCE_YAML: &str = "./data/binance.yaml";

pub struct BinanceFeed {
    loader: ConfigLoader
}

impl BinanceFeed {
    pub fn new() -> Box<dyn ApiFeed> {
        let mut loader = ConfigLoader {
            filename: String::from(BINANCE_YAML),
            config: vec![]
        };
        loader.load();
        Box::new(BinanceFeed{ 
            loader
        })
    }
}

impl ApiFeed for BinanceFeed {
    fn get_endpoint(&self, key: &str) -> ApiRequest {
        self.loader.get_endpoint(key)
    }

    fn system_status(&self) -> ApiRequest {
        self.get_endpoint("system_status")
    }

    fn coins_info(&self) -> ApiRequest {
        self.get_endpoint("coins_info")
    }
}