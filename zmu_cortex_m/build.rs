use std::env;

fn main() {
    let armv7em = env::var("CARGO_FEATURE_ARMV7EM").unwrap_or("0".to_string());
    let armv7m = env::var("CARGO_FEATURE_ARMV7M").unwrap_or("0".to_string());
    let armv6m = env::var("CARGO_FEATURE_ARMV6M").unwrap_or("0".to_string());

    if armv6m == "1" {
        println!("cargo:rustc-cfg=armv6m")
    } else if armv7m == "1" {
        println!("cargo:rustc-cfg=armv7m")
    } else if armv7em == "1" {
        println!("cargo:rustc-cfg=armv7em")
    } else {
        println!("cargo:rustc-cfg=armv7em")
    }

    println!("cargo:rerun-if-changed=build.rs");
}
