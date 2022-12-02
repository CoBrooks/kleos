#![no_std]
#![feature(abi_x86_interrupt)]

use bootloader_api::BootInfo;

pub mod apic;
pub mod font;
pub mod framebuffer;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;

pub enum Lazy<T: 'static> {
    Initialized(T),
    Uninitialized(fn() -> T),
    Empty
}

impl<T> Lazy<T> {
    pub const fn new(init: fn() -> T) -> Self {
        Lazy::Uninitialized(init)
    }

    /// # Safety
    ///
    /// Caller must only call this once, or else risk 
    /// swapping the value in place.
    pub unsafe fn unsafe_init(&mut self, t: T) {
        *self = Lazy::Initialized(t);
    }

    pub fn init(&mut self) {
        if let Lazy::Uninitialized(initfn) = self {
            *self = Lazy::Initialized(initfn());
        } else {
            panic!("Lazy has already been initialized")
        }
    }

    pub fn unwrap(&mut self) -> &mut T {
        match self {
            Lazy::Initialized(t) => t,
            Lazy::Uninitialized(_) => {
                self.init();
                self.unwrap()
            }
            Lazy::Empty => panic!("Cannot unwrap Lazy::Empty. Lazy must be initialized or contain an initializer.")
        }
    }
}

pub static mut PHYSICAL_MEM_OFFSET: Lazy<u64> = Lazy::Empty;

pub fn init(boot_info: &'static mut BootInfo) {
    unsafe { PHYSICAL_MEM_OFFSET.unsafe_init(*boot_info.physical_memory_offset.as_ref().unwrap()); }

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

    println!("Finished Initialization!");
}

pub fn hlt_loop() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt", options(nomem, nostack)) }
    }
}
