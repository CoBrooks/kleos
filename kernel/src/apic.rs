use pic8259::ChainedPics;
use x2apic::{lapic::{LocalApicBuilder, LocalApic}, ioapic::{IoApic, IrqMode, IrqFlags}};

use crate::{Lazy, PHYSICAL_MEM_OFFSET};

pub static mut LAPIC: Lazy<LocalApic> = Lazy::new(|| {
    let phys_addr = unsafe { x2apic::lapic::xapic_base() };
    let virt_addr = unsafe { phys_addr + *PHYSICAL_MEM_OFFSET.unwrap() };

    let lapic = LocalApicBuilder::new()
        .timer_vector(ApicInterruptIndex::Timer as usize)
        .error_vector(ApicInterruptIndex::Error as usize)
        .spurious_vector(ApicInterruptIndex::Spurious as usize)
        .set_xapic_base(virt_addr)
        .build()
        .expect("Failed to build LocalApic");

    lapic
});

const IOAPIC_IRQ_OFFSET: u8 = 0x20;

pub static mut IOAPIC: Lazy<IoApic> = Lazy::new(|| unsafe {
    let phys_addr = 0xFEC0_0000;
    let virt_addr = phys_addr + *PHYSICAL_MEM_OFFSET.unwrap();

    let mut ioapic = IoApic::new(virt_addr);
    ioapic.init(IOAPIC_IRQ_OFFSET);

    let apic_id = LAPIC.unwrap().id();

    let mut keyboard_entry = ioapic.table_entry(IrqVector::Keyboard as u8);
    keyboard_entry.set_mode(IrqMode::Fixed);
    keyboard_entry.set_flags(IrqFlags::MASKED);
    keyboard_entry.set_dest(apic_id as u8);
    ioapic.set_table_entry(IrqVector::Keyboard as u8, keyboard_entry);

    ioapic.enable_irq(IrqVector::Keyboard as u8);

    ioapic
});

pub fn init() {
    unsafe {
        disable_pic();
        LAPIC.unwrap().enable();
        IOAPIC.init();
        x86_64::instructions::interrupts::enable();
    }
}

unsafe fn disable_pic() {
    let mut pics = ChainedPics::new(32, 40);
    pics.initialize();
    pics.disable();
}

#[repr(usize)]
pub enum ApicInterruptIndex {
    Timer = 32,
    Keyboard = 33,
    Error = 60,
    Spurious = 61,
}

#[repr(u8)]
pub enum IrqVector {
    Keyboard = 1
}
