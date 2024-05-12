fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    // println!("cargo:rustc-link-arg-bins=-Tlink.x"); // this is done in cortex-m-rt package
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");   // required for defmt
    // enable cfg sdmmc
    println!("cargo:rustc-cfg=sdmmc");  // for stm32metapac
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
    // for otg_hs
    for (key, _value) in std::env::vars() {
        // list of chips that have otg_hs
        let otg_hs_chips = ["STM32U5A5ZJ", "STM32U5A5ZI"];
        if otg_hs_chips.iter().any(|&chip| key.starts_with(&format!("CARGO_FEATURE_{}", chip))) {
            println!("cargo:rustc-cfg=otg_hs");
            break; 
        }
    }
    // for sdmmc
    for (key, _value) in std::env::vars() {
        // list of chips that have otg_hs
        let sdmmc_chips = ["STM32U5A5ZJ", "STM32U5A5ZI"];
        if sdmmc_chips.iter().any(|&chip| key.starts_with(&format!("CARGO_FEATURE_{}", chip))) {
            println!("cargo:rustc-cfg=sdmmc");
            break; 
        }
    }

}
