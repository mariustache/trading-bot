use std::fmt;
use std::any::Any;
use log::{error};

use crate::utils::sign;

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug)]
pub struct HttpPayload {
    pub payload: String,
    pub api_key: String
}

impl ApiRequest {
    pub fn set_param(&mut self, name: &String, value: &String) {
        for index in 0..self.parameters.len() {
            if &self.parameters[index].0 == name {
                self.parameters[index].1 = value.clone();
                break;
            } else {
                error!("Parameter {} is not defined in api.", name);
            }
        }
    }

    // Create parameters payload without base url or signature.
    pub fn get_param_payload(&self) -> String {
        let mut payload = String::from("");
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

    // Create full payload.
    pub fn get_payload(&self) -> String {
        let mut payload = String::from(&self.url);
        payload.push_str(&self.get_param_payload());

        payload
    }

    // Create full signed payload.
    pub fn get_signed_payload(&self, secret_key: &String) -> String {
        assert_eq!(self.security, SecurityType::SIGNED);
        let mut payload = String::from(&self.url);
        let params_payload = self.get_param_payload();
        payload.push_str(&params_payload);

        // Sign payload and add signature to payload.
        let signature = sign(secret_key, &params_payload);
        let signature = format!("&signature={}", signature);
        payload.push_str(&signature);

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
    fn system_status(&self) -> HttpPayload;
    fn ping(&self) -> HttpPayload;
    fn coins_info(&self) -> HttpPayload;
    fn depth(&self, symbol: &str) -> HttpPayload;

    fn get_payload(&self, request: &ApiRequest, secret: &String, public: &String) -> HttpPayload {
        let mut payload = String::from("");
        let mut api_key = public.to_string();

        match request.security {
            SecurityType::NONE => {
                payload = request.get_payload();
                api_key = String::from("");
            },
            SecurityType::APIKEY => {
                payload = request.get_payload();
            },
            SecurityType::SIGNED => {
                payload = request.get_signed_payload(secret);
            },
            SecurityType::INVALID => {
                error!("Security type is invalid.");
            }
        }

        HttpPayload {
            payload,
            api_key
        }
    }
}