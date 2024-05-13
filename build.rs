fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");   // required for defmt


    println!("cargo:rustc-check-cfg=cfg(otg_hs)");  // enable cfg otg_hs
    println!("cargo:rustc-check-cfg=cfg(otg_fs)");  // enable cfg otg_hs
    println!("cargo:rustc-check-cfg=cfg(stm32u575)");  
    println!("cargo:rustc-check-cfg=cfg(stm32u5a5)");  
    println!("cargo:rustc-check-cfg=cfg(stm32u595)");  
    println!("cargo:rustc-check-cfg=cfg(sdmmc)");
    println!("cargo:rustc-check-cfg=cfg(dcmi)");

    // enable cfg sdmmc
    // println!("cargo:rustc-cfg=sdmmc");  // for stm32metapac
    for (key, _value) in std::env::vars() {
        // Check if the current environment variable is a feature that starts with "STM32U5"
        if key.starts_with("CARGO_FEATURE_STM32U575") {
            // If found, print the cargo directive to set the `stm32u5` cfg flag
            println!("cargo:rustc-cfg=stm32u575");
            break; // Exit the loop once the first matching feature is found
        }
        if key.starts_with("CARGO_FEATURE_STM32U5A5") {
            println!("cargo:rustc-cfg=stm32u5a5");
            break; // Exit the loop once the first matching feature is found
        }
    }
    // for otg_hs
    for (key, _value) in std::env::vars() {
        // list of chips that have otg_hs
        let otg_hs_chips = ["STM32U5A5ZJ", "STM32U5A5ZI", "STM32U5A5QJ"];
        if otg_hs_chips.iter().any(|&chip| key.starts_with(&format!("CARGO_FEATURE_{}", chip))) {
            println!("cargo:rustc-cfg=otg_hs");
            break; 
        }
    }
    // for sdmmc
    for (key, _value) in std::env::vars() {
        // list of chips that have otg_hs
        let sdmmc_chips = ["STM32U5A5ZJ", "STM32U5A5ZI", "STM32U5A5QJ"];
        if sdmmc_chips.iter().any(|&chip| key.starts_with(&format!("CARGO_FEATURE_{}", chip))) {
            println!("cargo:rustc-cfg=sdmmc");
            break; 
        }
    }
    // dmci 
    for (key, _value) in std::env::vars() {
        // list of chips that have otg_hs
        let dcmi_chips = ["STM32U5A5ZJ", "STM32U5A5ZI", "STM32U5A5QJ"];
        if dcmi_chips.iter().any(|&chip| key.starts_with(&format!("CARGO_FEATURE_{}", chip))) {
            println!("cargo:rustc-cfg=dcmi");
            break; 
        }
    }

}
