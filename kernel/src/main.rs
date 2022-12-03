#![no_std]
#![no_main]

extern crate alloc;

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

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel::init(boot_info);

    println!("Hello, World!");
    println!("Hello, Again!");

    println!("It did not crash!");
    kernel::hlt_loop()
}
