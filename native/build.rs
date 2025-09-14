use std::process::Command;
use std::path::PathBuf;
use std::fs;

fn main() {

    //println!("cargo:rerun-if-changed=../native/src/api.rs");

    let dart_out = PathBuf::from("../lib/bridge/generated");

    if let Some(parent) = dart_out.parent() {
        fs::create_dir_all(parent).expect("failed to create parent directories for dart output");
    }

    // println!("cargo:warning=dart output path: {:?}", dart_out);

    
    let status = Command::new("flutter_rust_bridge_codegen")
        .arg("generate")
        .arg("--rust-root")              
        .arg(".")                       
        .arg("--rust-input")              
        .arg("crate::api")                     
        .arg("--dart-output")
        .arg(dart_out.to_str().unwrap())
        // .arg("--c-output").arg("../../ios/Runner/bridge_generated.h") 
        .status()
        .expect("failed to run flutter_rust_bridge_codegen");

    // if !status.success() {
    //     panic!("flutter_rust_bridge_codegen failed");
    // }
}
