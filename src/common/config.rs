extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};
use crate::api_feed::{ApiRequest, SecurityType, EndpointParams};

use std::path::Path;
use std::fs;
use std::collections::HashMap;
use log::{info, debug};

type ApiRequests = HashMap<String, ApiRequest>;

fn extract(node: &Yaml) -> String {
    node.as_str().unwrap_or("Error").to_string()
}

pub struct ConfigLoader {
    pub filename: String,
    pub on_testnet: bool,
    pub requests: ApiRequests
}

impl ConfigLoader {
    pub fn load(&mut self) {
        let path = Path::new(&self.filename);
        assert_eq!(path.exists(), true);
        let path = path.to_str().unwrap();
        let contents = fs::read_to_string(path)
            .expect("Error reading contents from yaml file.");
        info!("Loading data from {}.", &self.filename);
        let contents = YamlLoader::load_from_str(&contents).unwrap();

        // Extract base url for endpoints.
        let metadata = &contents[0]["metadata"];
        let base_url = match self.on_testnet {
            true => extract(&metadata["test_endpoint"]),
            false => extract(&metadata["endpoint"])
        };

        // Create ApiRequest instances.
        let endpoints = &contents[0]["endpoints"].as_hash().unwrap();
        for (endpoint, value) in *endpoints {
            debug!("{:?}", endpoint);
            // Extract endpoint request type.
            let method = extract(&value["method"]);
            // Extract and append endpoint to base url.
            let url = format!("{}{}", &base_url, &extract(&value["endpoint"]));
            // Extract weight.
            let weight = extract(&value["weight"]);
            let weight = weight.parse::<u32>().unwrap();
            // Extract security type: NONE, SIGNED, etc.
            let security = match extract(&value["security"]).as_str() {
                "NONE" => SecurityType::NONE,
                "APIKEY" => SecurityType::APIKEY,
                "SIGNED" => SecurityType::SIGNED,
                _ => SecurityType::INVALID
    
            };
            // Extract endpoint parameters.
            let mut param_map = EndpointParams::new();
            match &value["parameters"].as_hash() {
                Some(parameters) => {
                    for (param, value) in *parameters {
                        let param_type = match extract(&value).as_str() {
                            "STRING" => "",
                            "INT" => "0",
                            _ => ""
                        };
                        param_map.push(
                            (extract(&param), String::from(param_type))
                        );
                    }
                },
                None =>  {
                    debug!("Found NONE on yaml: {:?}", &value["endpoint"]);
                }
            };            
            
            self.requests.insert(
                extract(&endpoint),
                ApiRequest {
                    method,
                    url,
                    weight,
                    security,
                    parameters: param_map,
                }
            );
        }
        
    }

    pub fn get_endpoint(&self, key: &str) -> ApiRequest {
        self.requests[key].clone()
    }
}