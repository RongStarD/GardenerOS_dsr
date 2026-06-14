mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::*;
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::{remap_test, KERNEL_SPACE, MapPermission, MemorySet};
pub use page_table::{translated_byte_buffer, PageTable, PageTableEntry, PTEFlags};

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}
