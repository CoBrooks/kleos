use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Automatically set by cargo
    let out_dir = std::env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .unwrap();

    // See https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies
    let kernel: PathBuf = std::env::var_os("CARGO_BIN_FILE_KERNEL")
        .map(PathBuf::from)
        .unwrap();

    // Create UEFI disk image
    let uefi_path = out_dir.join("uefi.img");
    bootloader::UefiBoot::new(&kernel).create_disk_image(&uefi_path)?;

    // Create BIOS disk image
    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel).create_disk_image(&bios_path)?;

    // Pass disk image path as env variables to the `main.rs`
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());

    Ok(())
}
