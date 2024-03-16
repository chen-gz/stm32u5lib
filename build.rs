fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
    // enable cfg sdmmc
    println!("cargo:rustc-cfg=sdmmc");
    for (key, _value) in std::env::vars() {
        // Check if the current environment variable is a feature that starts with "STM32U5"
        if key.starts_with("CARGO_FEATURE_STM32U575") {
            // If found, print the cargo directive to set the `stm32u5` cfg flag
            println!("cargo:rustc-cfg=stm32u575");
            break; // Exit the loop once the first matching feature is found
        }
        if key.starts_with("CARGO_FEATURE_STM32U5A5") {
            // If found, print the cargo directive to set the `stm32u5` cfg flag
            println!("cargo:rustc-cfg=stm32u5a5");
            break; // Exit the loop once the first matching feature is found
        }
    }
}
