extern crate protoc_rust;
use std::fs;
use std::path::Path;

const PROTOS_OUT_DIR: &str = "src/protos";
const PROTOS_INCLUDES: [&'static str; 1] = [
    "protos/binance",
];
const PROTOS_INPUTS: [&'static str; 3] = [
    "protos/binance/common.proto",
    "protos/binance/market_data.proto",
    "protos/binance/wallet.proto",
];

fn main() {
    // Create output folder for generated code.
    if Path::new(PROTOS_OUT_DIR).is_dir() == false {
        let res = fs::create_dir(PROTOS_OUT_DIR);
        match res {
            Ok(v) => println!("Created folder {:?}", v),
            Err(e) => panic!("{:?}", e)
        }
    }
    
    let mut generator = protoc_rust::Codegen::new();
    generator.out_dir(PROTOS_OUT_DIR);
    generator.inputs(&PROTOS_INPUTS);

    for _include in &PROTOS_INCLUDES {
        generator.include(_include);
    }
    
    generator.run().expect("protoc failed.");
}