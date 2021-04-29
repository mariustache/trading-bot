extern crate yaml_rust;

use yaml_rust::YamlLoader;
use std::path::Path;

pub struct ConfigLoader {
    pub filename: String
}

impl ConfigLoader {
    pub fn load(&self) -> bool {
        let path = Path::new(&self.filename);
        let path = path.to_str().unwrap_or("Error");
        let config = YamlLoader::load_from_str(path).unwrap();
        println!("{:?}", config[0]);

        true
    }
}