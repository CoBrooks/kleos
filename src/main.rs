use std::{process::{Command, Output}, fs::File, time::Instant};

use time;

#[allow(clippy::upper_case_acronyms, unused)]
enum BootType {
    UEFI,
    BIOS
}

const BOOT_TYPE: BootType = BootType::BIOS;

struct Debugger {
    cmd: Command
}

impl Debugger {
    fn wrap(cmd: Command) -> Self {
        Debugger { cmd }
    }

    fn run(&mut self) -> Output {
        let child = self.cmd.spawn().unwrap();
        child.wait_with_output().unwrap()
    }
}

impl Drop for Debugger {
    fn drop(&mut self) {
        use std::io::Write;

        let output = self.run();

        let mut file = File::options()
            .append(true)
            .create(true)
            .write(true)
            .open("kleos.log")
            .unwrap();

        let now = time::OffsetDateTime::now_local().unwrap();
        let format = time::format_description::parse("[month]/[day]/[year] [hour]:[minute]").unwrap();

        writeln!(file, "{:-^48}", format!(" {} ", now.format(&format).unwrap())).unwrap();
        writeln!(file, "{}\n", String::from_utf8(output.stdout).unwrap()).unwrap();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    let headless = std::env::var("NO_DISPLAY").unwrap_or("false".to_string());

    let mut cmd = Command::new("qemu-system-x86_64");
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

    let debugger = Debugger::wrap(cmd);

    Ok(())
}

