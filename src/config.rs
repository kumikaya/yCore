
pub const USER_STACK_SIZE: usize = 4096;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x20000;
pub const KERNEL_MEMORY_END: usize = 0x80400000;
pub const MEMORY_START: usize = 0x80000000;
pub const MEMORY_END: usize = 0x88000000;
pub const CLOCK_FREQ: usize = 12500000;
pub const PAGE_SIZE: usize = 4 * 1024;