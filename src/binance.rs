extern crate common;

use common::api_feed::{ApiFeed, WalletInfo};
use common::config::ConfigLoader;

use std::collections::HashMap;

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
        Box::new(BinanceFeed{ loader })
    }
}

impl ApiFeed for BinanceFeed {
    fn system_status(&self) -> bool {
        let endpoint = self.loader.get_endpoint("system_status");
        println!("{:?}", endpoint);
        true
    }

    fn wallet_info(&self) -> WalletInfo {
        WalletInfo {
            coins: HashMap::new()
        }
    }
}