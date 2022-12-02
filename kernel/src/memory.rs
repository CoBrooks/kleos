use bootloader_api::info::{MemoryRegions, MemoryRegionKind};
use x86_64::structures::paging::{
    FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};

/// Intialize a new OffsetPageTable
///
/// # Safety
///
/// The caller must guarantee that the complete physical memory
/// is mapped to virtual memory at the passed `physical_offset`.
/// Additionally, this function must only be called *once* to
/// avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn init(physical_offset: VirtAddr) -> OffsetPageTable<'static> {
    let lvl4_table = active_lvl4_table(physical_offset);
    OffsetPageTable::new(lvl4_table, physical_offset)
}

/// Returns a mutable reference to the active level 4 table.
///
/// # Safety
///
/// The caller must guarantee that the complete physical memory
/// is mapped to virtual memory at the passed `physical_offset`.
/// Additionally, this function must only be called *once* to
/// avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_lvl4_table(physical_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (lvl4_table_frame, _flags) = Cr3::read();

    let phys = lvl4_table_frame.start_address();
    let virt = physical_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe { mapper.map_to(page, frame, flags, frame_allocator) };
    map_to_result.expect("map_to failed").flush();
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}

/// A FrameAllocator that returns usable
/// frames from the bootloader's memory map.
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that the passed memory map
    /// is valid. The primary requirement is that all frames
    /// marked `USABLE` are really unused.
    pub unsafe fn init(memory_map: &'static MemoryRegions) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames inside of the
    /// memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        self.memory_map
            .iter()
            // Get usable regions from memory map
            .filter(|r| r.kind == MemoryRegionKind::Usable)
            // Map each region to its address range
            .map(|r| r.start..r.end)
            // Transform into iterator of frame start addresses
            .flat_map(|r| r.step_by(4096))
            // Create `PhysFrame` types from the start addrs
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
