use std::env;
fn main() {
    let target = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    // if target.contains("arm") {
        println!("cargo:rustc-link-arg-bins=--nmagic");
        println!("cargo:rustc-link-arg-bins=-Tlink.x");
        println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
        // enable cfg sdmmc
        print!("cargo:rustc-cfg=sdmmc");
    // }
}
