use std::env;

pub fn get_env(key: &str) -> String {
    let value = match env::var(key) {
        Ok(val) => val,
        Err(e) => {
            println!("Cannot find env var {}: {}", key, e);
            String::from("")
        }
    };
    value
}

pub fn sign(secret: &str, payload: &str) -> String {
    String::from("")
}