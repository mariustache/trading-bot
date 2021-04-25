extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/protos")
        .inputs(&["protos/binance/common.proto", "protos/binance/wallet.proto"])
        .include("protos/binance")
        .run()
        .expect("protoc failed.");
}