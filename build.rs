fn main() {
    // Only pass linker arguments if we are not building for the host (assuming host is not embedded target)
    let target = std::env::var("TARGET").unwrap();
    if !target.contains("x86_64") && !target.contains("aarch64") && !target.contains("windows") && !target.contains("macos") && !target.contains("linux") {
        #[cfg(feature = "defmt")]
        println!("cargo:rustc-link-arg=-Tdefmt.x");

        println!("cargo:rustc-link-arg=--nmagic");
        println!("cargo:rustc-link-arg=-Tlink.x");
    }

    println!("cargo:rustc-check-cfg=cfg(otg_hs)"); // enable cfg otg_hs
    println!("cargo:rustc-check-cfg=cfg(otg_fs)"); // enable cfg otg_hs
    println!("cargo:rustc-check-cfg=cfg(stm32u575)");
    println!("cargo:rustc-check-cfg=cfg(stm32u5a5)");
    println!("cargo:rustc-check-cfg=cfg(stm32u595)");
    println!("cargo:rustc-check-cfg=cfg(sdmmc)");
    println!("cargo:rustc-check-cfg=cfg(dcmi)");
    // list chips has sdmmc
    let sdmmc_chips = ["STM32U5A5ZJ", "STM32U5A5ZI", "STM32U5A5QJ"];
    let dcmi_chips = ["STM32U5A5ZJ", "STM32U5A5ZI", "STM32U5A5QJ"];
    // let otg_hs_chips = ["STM32U5A5ZJ", "STM32U5A5ZI", "STM32U5A5QJ"];
    // let otg_fs_chips = ["STM32U575ZG", "STM32U575ZI", "STM32U575ZG", "STM32U575CI"];

    // enable cfg sdmmc
    // println!("cargo:rustc-cfg=sdmmc");  // for stm32metapac
    for (key, _value) in std::env::vars() {
        // Check if the current environment variable is a feature that starts with "STM32U5"
        if key.starts_with("CARGO_FEATURE_STM32U575") {
            println!("cargo:rustc-cfg=stm32u575");
            break;
        } else if key.starts_with("CARGO_FEATURE_STM32U5A5") {
            println!("cargo:rustc-cfg=stm32u5a5");
            break;
        } else if key.starts_with("CARGO_FEATURE_STM32U595") {
            println!("cargo:rustc-cfg=stm32u595");
            break;
        }
    }
    // for otg_hs
    for (key, _value) in std::env::vars() {
        // if otg_fs_chips.iter().any(|&chip| key.starts_with(&format!("CARGO_FEATURE_{}", chip))) {
        //     println!("cargo:rustc-cfg=otg_fs");
        // }
        //
        // if otg_hs_chips.iter().any(|&chip| key.starts_with(&format!("CARGO_FEATURE_{}", chip))) {
        //     println!("cargo:rustc-cfg=otg_hs");
        // }
        if sdmmc_chips
            .iter()
            .any(|&chip| key.starts_with(&format!("CARGO_FEATURE_{}", chip)))
        {
            println!("cargo:rustc-cfg=sdmmc");
        }
        if dcmi_chips
            .iter()
            .any(|&chip| key.starts_with(&format!("CARGO_FEATURE_{}", chip)))
        {
            println!("cargo:rustc-cfg=dcmi");
        }
    }
}
