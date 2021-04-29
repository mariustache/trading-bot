extern crate common;

use common::api_feed::{ApiFeed, WalletInfo};
use common::config::ConfigLoader;

use std::collections::HashMap;

const BINANCE_YAML: &str = "./data/binance.yaml";

pub struct BinanceFeed {
    loader: ConfigLoader
}

impl BinanceFeed {
    pub fn new() -> BinanceFeed {
        BinanceFeed {
            loader: ConfigLoader {
                filename: String::from("")
            }
        }
    }

    pub fn init(mut self) {
        self.loader = ConfigLoader{ filename: String::from(BINANCE_YAML) };
        self.loader.load();
    }
}

impl ApiFeed for BinanceFeed {
    fn system_status() -> bool {
        true
    }

    fn wallet_info() -> WalletInfo {
        WalletInfo {
            coins: HashMap::new()
        }
    }
}