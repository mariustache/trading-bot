use std::fmt;
use std::any::Any;

#[derive(Debug, Clone)]
pub enum SecurityType {
    NONE = 0,
    APIKEY,
    SIGNED,
    INVALID
}

pub type EndpointParams = Vec<(String, String)>;

#[derive(Clone)]
pub struct ApiRequest {
    pub method: String,
    pub url: String,
    pub weight: u32,
    pub security: SecurityType,
    pub parameters: EndpointParams,
}

impl ApiRequest {
    pub fn add_param(&mut self, name: String, value: String) {
        self.parameters.push((name, value));
    }

    pub fn get_param_payload(&self) -> String {
        let mut payload = String::from(&self.url);
        let mut param_str: Vec<String> = Vec::new();
        // Add parameters to payload.
        if !self.parameters.is_empty() {
            payload.push_str("?");
            for (param, value) in &self.parameters {
                param_str.push(format!("{}={}", param, value));
            }
            payload = payload + &param_str.join("&");
        }

        payload
    }
}

impl fmt::Debug for ApiRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = format!("Method: {}\n", self.method);
        output.push_str(format!("Endpoint: {}\n", self.url).as_str());
        output.push_str(format!("Weight: {}\n", self.weight).as_str());
        output.push_str(format!("Security: {:?}\n", self.security).as_str());
        output.push_str(format!("Parameters: {:?}\n", self.parameters).as_str());
        write!(f, "{}", output)
    }
}

pub trait ApiFeed {
    fn as_any(&self) -> &dyn Any;
    fn system_status(&self) -> String;
    fn coins_info(&self) -> String;
    fn depth(&self) -> String;
}