// use core::mem::size_of;

use crate::{mem::address::{VirtAddr, PhysAddr}, trap::context::TrapContext, stdlib::tools::aligned_size};

pub const KERNEL_HEAP_SIZE: usize = 0x40_0000;
pub const KERNEL_INIT_STACK_SIZE: usize = 0x4000;
pub const KERNEL_STACK_SIZE: usize = 0x2000;
pub const GUARD_PAGE_SIZE: usize = 1 * PAGE_SIZE;

pub const USER_STACK_SIZE: usize = 0x4000;

pub const TRAMPOLINE: VirtAddr = VirtAddr(usize::MAX - PAGE_SIZE + 1);
pub const TRAP_CONTEXT: VirtAddr = TRAMPOLINE - VirtAddr(PAGE_SIZE);
pub const KERNEL_STACK_BOTTOM: VirtAddr = TRAMPOLINE - VirtAddr(PAGE_SIZE);
// pub const TRAP_CONTEXT: VirtAddr = VirtAddr(KERNEL_STACK_BOTTOM.0 - aligned_size::<TrapContext>(16));
pub const KERNEL_STACK_TOP: VirtAddr = KERNEL_STACK_BOTTOM - VirtAddr(KERNEL_STACK_SIZE);
pub const MEMORY_END: usize = 0x8800_0000;

pub const CLOCK_FREQ: usize = 12500000;
pub const TICK_FREQ: usize = 100;

pub const PAGE_WIDTH: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_WIDTH;
pub const SV39_PAGE_LEVEL: usize = 3;
pub const SV39_PAGE_INDEX_WIDTH: usize = 9;

pub fn kernel_stack_position(uid: usize) -> (usize, usize) {
    let bottom = TRAMPOLINE.0 - (uid - 1000) * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let top = bottom - KERNEL_STACK_SIZE;
    (top, bottom)
}

pub fn config_align_check() {
    assert!(KERNEL_STACK_BOTTOM.is_page_align());
    assert!(TRAMPOLINE.is_page_align());
}