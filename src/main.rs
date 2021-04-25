mod protos;
use protos::common::{RequestHeader};

fn main() {
    let mut request = RequestHeader::new();
    request.set_field_type("GET".to_string());
    request.set_endpoint("/sapi/v1/system/status".to_string());
    println!("{:?}", request);
}
