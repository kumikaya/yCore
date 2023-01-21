pub mod address;
pub mod frame_allocator;
pub mod heap_allocator;
pub mod memory_set;
pub mod page_table;

pub fn init() {
    heap_allocator::init_heap();
    // frame_allocator::init_frame_allocator();
    // memory_set::init_kernel_space();
}
