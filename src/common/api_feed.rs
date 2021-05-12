use std::fmt;
use std::any::Any;
use std::collections::HashMap;

#[derive(Debug)]
pub enum SecurityType {
    NONE = 0,
    APIKEY,
    SIGNED,
    INVALID
}

#[derive(Debug)]
pub enum ParameterType {
    STRING = 0,
    INT,
    INVALID
}

pub type EndpointParamsMap = HashMap<String, ParameterType>;

pub struct ApiRequest {
    pub method: String,
    pub url: String,
    pub weight: u32,
    pub security: SecurityType,
    pub parameters: EndpointParamsMap
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
    fn system_status(&self) -> &ApiRequest;
    fn coins_info(&self) -> &ApiRequest;
}