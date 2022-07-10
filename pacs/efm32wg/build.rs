use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());

        let device_file = if env::var_os("CARGO_FEATURE_EFM32WG230").is_some() {
            "src/efm32wg230/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG232").is_some() {
            "src/efm32wg232/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG280").is_some() {
            "src/efm32wg280/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG290").is_some() {
            "src/efm32wg290/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG295").is_some() {
            "src/efm32wg295/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG330").is_some() {
            "src/efm32wg330/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG332").is_some() {
            "src/efm32wg332/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG360").is_some() {
            "src/efm32wg360/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG380").is_some() {
            "src/efm32wg380/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG390").is_some() {
            "src/efm32wg390/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG395").is_some() {
            "src/efm32wg395/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG840").is_some() {
            "src/efm32wg840/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG842").is_some() {
            "src/efm32wg842/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG880").is_some() {
            "src/efm32wg880/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG890").is_some() {
            "src/efm32wg890/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG895").is_some() {
            "src/efm32wg895/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG900").is_some() {
            "src/efm32wg900/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG940").is_some() {
            "src/efm32wg940/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG942").is_some() {
            "src/efm32wg942/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG980").is_some() {
            "src/efm32wg980/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG990").is_some() {
            "src/efm32wg990/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32WG995").is_some() {
            "src/efm32wg995/device.x"
        } else { panic!("No device features selected"); };

        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }

    println!("cargo:rerun-if-changed=build.rs");
}

