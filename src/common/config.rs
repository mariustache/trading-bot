extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};

use crate::api_feed::{ApiRequest, SecurityType};

use std::path::Path;
use std::fs;

const ON_TESTNET: bool = true;

pub struct ConfigLoader {
    pub filename: String,
    pub config: Vec<Yaml>
}

impl ConfigLoader {
    pub fn load(&mut self) {
        let path = Path::new(&self.filename);
        assert_eq!(path.exists(), true);
        let path = path.to_str().unwrap();
        let contents = fs::read_to_string(path)
            .expect("Error reading contents from yaml file.");
        let contents = YamlLoader::load_from_str(&contents).unwrap();
        self.config = contents;
    }

    pub fn get_endpoint(&self, key: &str) -> ApiRequest {
        let endpoints = &self.config[0]["endpoints"];
        let element = &endpoints[key];
        
        let extract = |node: &Yaml| -> String {
            node.as_str().unwrap_or("Error").to_string()
        };

        let method = extract(&element["method"]);
        
        let mut endpoint = match ON_TESTNET {
            true => self.get_metadata("test_endpoint"),
            false => self.get_metadata("endpoint")
        };
        endpoint.push_str(&extract(&element["endpoint"]));

        let weight = extract(&element["weight"]);
        let weight = weight.parse::<u32>().unwrap();

        let security = match extract(&element["security"]).as_str() {
            "NONE" => SecurityType::NONE,
            "APIKEY" => SecurityType::APIKEY,
            "SIGNED" => SecurityType::SIGNED,
            _ => SecurityType::NOT_VALID

        };

        ApiRequest {
            method,
            endpoint,
            weight,
            security

        }
    }

    pub fn get_metadata(&self, key: &str) -> String {
        let metadata = &self.config[0]["metadata"];
        let metadata = &metadata[key];

        format!("{}", metadata.as_str().unwrap())
    }
}