use std::any::Any;
use std::collections::HashMap;

use log::{info};

extern crate common;
use common::api_feed::{ApiFeed, SecurityType};
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
        info!("Creating Binance feed. On testnet: {}", &on_testnet);
        let mut loader = ConfigLoader {
            filename: String::from(BINANCE_YAML),
            on_testnet,
            requests: HashMap::new()
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

    pub fn print_requests(&self) {
        for request in &self.loader.requests {
            println!("{:?}", request);
        }
    }
}

impl ApiFeed for BinanceFeed {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn system_status(&self) -> String {
        let request = self.loader.get_endpoint("system_status");
        
        request.get_payload()
    }

    fn coins_info(&self) -> String {
        //self.loader.get_endpoint("coins_info")
        String::from("")
    }

    fn depth(&self, symbol: &String) -> String {
        let mut request = self.loader.get_endpoint("depth");
        request.set_param(&String::from("symbol"), symbol);
        
        if request.security == SecurityType::SIGNED {
            request.get_signed_payload(&self.secret_key)
        } else {
            request.get_payload()
        }
    }
}