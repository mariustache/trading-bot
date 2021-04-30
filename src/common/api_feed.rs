use std::fmt;

#[derive(Debug)]
pub enum SecurityType {
    NONE = 0,
    APIKEY,
    SIGNED,
    NOT_VALID
}

pub struct ApiRequest {
    pub method: String,
    pub endpoint: String,
    pub weight: u32,
    pub security: SecurityType
}

impl fmt::Debug for ApiRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = format!("Method: {}\n", self.method);
        output.push_str(format!("Endpoint: {}\n", self.endpoint).as_str());
        output.push_str(format!("Weight: {}\n", self.weight).as_str());
        output.push_str(format!("Security: {:?}\n", self.security).as_str());
        write!(f, "{}", output)
    }
}

pub trait ApiFeed {
    fn get_endpoint(&self, key: &str) -> ApiRequest;
    fn system_status(&self) -> ApiRequest;
    fn coins_info(&self) -> ApiRequest;
}