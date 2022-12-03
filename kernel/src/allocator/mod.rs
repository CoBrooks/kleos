use alloc::alloc::Layout;

use x86_64::{structures::paging::{Mapper, Size4KiB, FrameAllocator, Page, mapper::MapToError, PageTableFlags}, VirtAddr};

use crate::Locked;

pub mod fixed_size_block;
use fixed_size_block::FixedSizeBlockAllocator;

pub const HEAP_START: usize = 0x4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {layout:#?}");
}

pub(crate) fn init_heap<M, F>(mapper: &mut M, frame_allocator: &mut F) -> Result<(), MapToError<Size4KiB>>
where 
    M: Mapper<Size4KiB>,
    F: FrameAllocator<Size4KiB>
{
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);

        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        }
    }

    unsafe { ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE); }

    Ok(())
}

