#![no_std]
#![feature(
    abi_x86_interrupt,
    alloc_error_handler,
    const_mut_refs,
    let_chains,
    slice_as_chunks
)]

extern crate alloc;

use bootloader_api::BootInfo;
use spin::{Once, Mutex, MutexGuard};
use x86_64::VirtAddr;

use crate::memory::BootInfoFrameAllocator;

pub mod allocator;
pub mod apic;
pub mod color;
pub mod font;
pub mod framebuffer;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod tracing;

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

    let green = color::ANSI_ESCAPES[color::ColorName::Green as usize];
    let clear = color::ANSI_ESCAPES[color::ColorName::Foreground as usize];

    // Interrupts
    print!("INIT: Interrupts.... ");
    interrupts::init();
    println!("[{green}OK{clear}]");

    // APIC
    print!("INIT: APIC.......... ");
    apic::init();
    println!("[{green}OK{clear}]");

    // Heap
    print!("INIT: Heap.......... ");
    let physical_mem_offset = VirtAddr::new(*PHYSICAL_MEM_OFFSET.get().unwrap());
    let mut mapper = unsafe { memory::init(physical_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_regions)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap initialization failed");
    println!("[{green}OK{clear}]");
    
    // Framebuffer Output
    let fb = boot_info.framebuffer.as_mut().unwrap();
    let fb_info = fb.info();
    framebuffer::init(fb.buffer_mut(), fb_info);
    println!("INIT: Framebuffer... [{green}OK{clear}]");

    // Tracing
    print!("INIT: Tracing....... ");
    tracing::init_tracing();
    println!("[{green}OK{clear}]");

    println!("Finished Initialization!\n");
}

pub fn hlt_loop() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)) }
    }
}
