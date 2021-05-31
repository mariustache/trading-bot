use std::{env, str};
use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};

type HmacSha256 = Hmac<Sha256>;

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

pub fn sign(secret: &String, payload: &String) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC added secret key.");
    mac.update(payload.as_bytes());
    
    let bytes = mac.finalize().into_bytes();
    let mut res = String::from("");
    for _byte in &bytes {
        res.push_str(&format!("{:02x}", _byte));
    }

    res
}

#[test]
pub fn test_hmac() {
    let mut mac = HmacSha256::new_from_slice(b"my-secret-key")
        .expect("HMAC test secret key.");
    mac.update(b"My payload to be signed.");
    let code_bytes = mac.clone().finalize().into_bytes();
    mac.verify(&code_bytes).unwrap();
}

#[test]
pub fn test_hmac_sign() {
    let secret_key = String::from("my-secret-key");
    let payload = String::from("My payload to be signed.");
    let signed_payload = sign(&secret_key, &payload);
    assert_eq!(signed_payload, "d93ae508ce0da17d756899ca3bda429df810833b0a71cadcb9c4be594cb2a760");
}