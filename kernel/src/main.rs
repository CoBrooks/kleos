#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate tracing;

use core::panic::PanicInfo;

use bootloader_api::{entry_point, config::Mapping, BootloaderConfig, BootInfo};

use kernel::println;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{info}");

    kernel::hlt_loop()
}

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::FixedAddress(0x0000_f000_0000_0000));
    config
};
entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

#[instrument]
fn test_tracing(a: u64, b: bool) {
    info!("Testing tracing!");
}

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel::init(boot_info);

    debug!("Hello, World!");
    info!("Hello, Again!");
    warn!("Hello, Again!");
    error!("Hello, Again!");
    trace!("Hello, Again!");

    test_tracing(42, false);

    debug!("It did not crash!");
    kernel::hlt_loop()
}
