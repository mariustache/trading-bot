mod protos;
use protos::wallet::Request;

fn main() {
    let mut request = Request::new();
    request.set_field_type("GET".to_string());
    request.set_endpoint("/sapi/v1/system/status".to_string());
}
