extern crate protoc_rust;
use std::env;
use std::path::Path;

fn main() {
    let out_dir_env = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_env);
    protoc_rust::Codegen::new()
        .out_dir("src/protos")
        .inputs(&["src/protos/wallet.proto"])
        .include("src/protos")
        .run()
        .expect("protoc failed.");
}