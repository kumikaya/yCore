// use core::mem::size_of;

pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;
pub const KERNEL_INIT_STACK_SIZE: usize = 0x4000;
pub const KERNEL_STACK_SIZE: usize = 0x1000;
pub const GUARD_PAGE_NUM: usize = 1;
// pub const KERNEL_STACK_SIZE_PER_HART: usize = KERNEL_STACK_SIZE / size_of::<usize>();

pub const APP_BASE_ADDRESS: usize = 0x8100_0000;
pub const APP_SIZE_LIMIT: usize = 0x2_0000;
pub const APP_STACK_SIZE: usize = 0x2000;

pub const KERNEL_MEMORY_END: usize = 0x8100_0000;
pub const MEMORY_START: usize = 0x8000_0000;
pub const MEMORY_END: usize = 0x8800_0000;

pub const CLOCK_FREQ: usize = 12500000;
pub const TICK_FREQ: usize = 100;

pub const PAGE_WIDTH: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_WIDTH;
pub const SV39_PAGE_LEVEL: usize = 3;
pub const SV39_PAGE_INDEX_WIDTH: usize = 9;