#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]

extern crate alloc;

use bootloader_api::BootInfo;
use spin::{Once, Mutex, MutexGuard};
use x86_64::VirtAddr;

use crate::memory::BootInfoFrameAllocator;

pub mod allocator;
pub mod apic;
pub mod font;
pub mod framebuffer;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;

pub struct Locked<T> {
    inner: Mutex<T>
}

impl<T> Locked<T> {
    const fn new(inner: T) -> Self {
        Locked { inner: Mutex::new(inner) }
    }

    fn lock(&self) -> MutexGuard<T> {
        self.inner.lock()
    }
}

pub static PHYSICAL_MEM_OFFSET: Once<u64> = Once::new();

pub fn init(boot_info: &'static mut BootInfo) {
    PHYSICAL_MEM_OFFSET.call_once(|| *boot_info.physical_memory_offset.as_ref().unwrap());

    // Framebuffer Output
    let fb = boot_info.framebuffer.as_mut().unwrap();
    let fb_info = fb.info();
    framebuffer::init(fb.buffer_mut(), fb_info);
    println!("INIT: Framebuffer... [OK]");

    // Interrupts
    print!("INIT: Interrupts... ");
    interrupts::init();
    println!("[OK]");

    // APIC
    print!("INIT: APIC... ");
    apic::init();
    println!("[OK]");

    // Heap
    print!("INIT: Heap... ");
    let physical_mem_offset = VirtAddr::new(*PHYSICAL_MEM_OFFSET.get().unwrap());
    let mut mapper = unsafe { memory::init(physical_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_regions)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed");
    println!("[OK]");

    println!("Finished Initialization!");
}

pub fn hlt_loop() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)) }
    }
}
