extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};
use std::path::Path;
use std::fs;

pub struct ConfigLoader {
    pub filename: String,
    pub config: Vec<Yaml>
}

impl ConfigLoader {
    pub fn load(&mut self) {
        let path = Path::new(&self.filename);
        assert_eq!(path.exists(), true);
        let path = path.to_str().unwrap_or("Error");
        let contents = fs::read_to_string(path)
            .expect("Error reading contents from yaml file.");
        let contents = YamlLoader::load_from_str(&contents).unwrap();
        self.config = contents;
    }

    pub fn get_endpoint(&self, key: &str) -> &str {
        let endpoints = &self.config[0]["endpoints"];
        let endpoint = &endpoints[key]["value"];

        endpoint.as_str().unwrap()
    }

    pub fn get_metadata(&self, key: &str) -> &str {
        let metadata = &self.config[0]["metadata"];
        let metadata = &metadata[key];

        metadata.as_str().unwrap()
    }
}