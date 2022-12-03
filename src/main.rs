#[allow(clippy::upper_case_acronyms, unused)]
enum BootType {
    UEFI,
    BIOS
}

const BOOT_TYPE: BootType = BootType::BIOS;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    let headless = std::env::var("NO_DISPLAY").unwrap_or("false".to_string());

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd
        // Freeze QEMU instead of rebooting
        .args([ "-action", "reboot=shutdown,shutdown=pause" ])
        // Send serial output to stdout
        .args([ "-serial", "stdio" ])
        // Display options
        .args([ "-display", "gtk,gl=on,full-screen=on" ])
        // Increase memory available to QEMU
        .args([ "-m", "4G" ]);

    if headless == "true" {
        cmd.args([ "-display", "none" ]);
    }

    match BOOT_TYPE {
        BootType::UEFI => {
            cmd.arg("-bios")
                .arg(ovmf_prebuilt::ovmf_pure_efi());
            cmd.arg("-drive")
                .arg(format!("format=raw,file={uefi_path}"));
        },
        BootType::BIOS => {
            cmd.arg("-drive")
                .arg(format!("format=raw,file={bios_path}"));
        },
    }

    let child = cmd.spawn()?;
    let out = child.wait_with_output()?;

    eprintln!("{}", String::from_utf8(out.stdout)?);

    Ok(())
}

