use std::any::Any;
extern crate common;
use common::api_feed::{ApiFeed, ApiRequest};
use common::config::ConfigLoader;
use common::utils::get_env;

const BINANCE_YAML: &str = "./data/binance.yaml";

pub struct BinanceFeed {
    loader: ConfigLoader,
    secret_key: String,
    public_key: String
}

impl BinanceFeed {
    pub fn new() -> Box<dyn ApiFeed> {
        let on_testnet = get_env("ON_TESTNET") == "true";
        let mut loader = ConfigLoader {
            filename: String::from(BINANCE_YAML),
            config: vec![],
            on_testnet
        };
        loader.load();
        Box::new(BinanceFeed{ 
            loader,
            secret_key: match on_testnet {
                true => get_env("BINANCE_TESTNET_API_SECRET"),
                false => get_env("BINANCE_API_SECRET")
            },
            public_key: match on_testnet {
                true => get_env("BINANCE_TESTNET_API_PUBLIC"),
                false => get_env("BINANCE_API_PUBLIC")
            },
            
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
    
    fn system_status(&self) -> ApiRequest {
        self.loader.get_endpoint("system_status")
    }

    fn coins_info(&self) -> ApiRequest {
        self.loader.get_endpoint("coins_info")
    }
}