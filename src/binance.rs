use std::any::Any;
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
            config: vec![],
            on_testnet: dotenv!("ON_TESTNET") == "true"
        };
        loader.load();
        Box::new(BinanceFeed{ 
            loader
        })
    }

    pub fn on_testnet(&self) -> bool {
        self.loader.on_testnet
    }
}

impl ApiFeed for BinanceFeed {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn secret_key(&self) -> String {
        let key = match self.on_testnet() {
            true => dotenv!("BINANCE_TESTNET_API_SECRET"),
            false => dotenv!("BINANCE_API_SECRET")
        };
        key.to_string()
    }

    fn public_key(&self) -> String {
        let key = match self.loader.on_testnet {
            true => dotenv!("BINANCE_TESTNET_API_PUBLIC"),
            false => dotenv!("BINANCE_API_PUBLIC")
        };
        key.to_string()
    }
    
    fn system_status(&self) -> ApiRequest {
        self.loader.get_endpoint("system_status")
    }

    fn coins_info(&self) -> ApiRequest {
        self.loader.get_endpoint("coins_info")
    }
}