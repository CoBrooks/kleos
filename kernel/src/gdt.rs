use spin::Lazy;
use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::{CS, Segment, DS, ES, SS};
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector}; 
use x86_64::{structures::tss::TaskStateSegment, VirtAddr};

pub static GDT: Lazy<(GlobalDescriptorTable, Selectors)> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();
    let kernel_cs = gdt.add_entry(Descriptor::kernel_code_segment());
    let kernel_ds = gdt.add_entry(Descriptor::kernel_data_segment());
    let tss = gdt.add_entry(Descriptor::tss_segment(&TSS));

    (gdt, Selectors { kernel_cs, kernel_ds, tss })
});

pub(crate) static TSS: Lazy<TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();

    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
        // let stack_end = 
        stack_start + STACK_SIZE
    };

    tss
});

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

/// # Safety
///
/// This function may only be called once.
pub unsafe fn init() {
    let (gdt, selectors) = &*GDT;
    gdt.load();
    
    CS::set_reg(selectors.kernel_cs);
    DS::set_reg(selectors.kernel_ds);
    ES::set_reg(selectors.kernel_ds);
    SS::set_reg(selectors.kernel_ds);

    load_tss(selectors.tss);
}

pub struct Selectors {
    kernel_cs: SegmentSelector,
    kernel_ds: SegmentSelector,
    tss: SegmentSelector,
}
