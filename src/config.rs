/// 内核堆大小
pub const KERNEL_HEAP_SIZE: usize = 0x40_0000;
/// 内核初始化栈大小
pub const KERNEL_INIT_STACK_SIZE: usize = 0x3000;
/// 硬件线程数
pub const HART_NUMBER: usize = 2;
/// 应用内核栈大小
pub const KERNEL_STACK_SIZE: usize = 0x3000;
pub const GUARD_PAGE_SIZE: usize = 4 * PAGE_SIZE;
/// 用户栈大小
pub const USER_STACK_SIZE: usize = 0x4000;
/// 跳板地址
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;
pub const MEMORY_END: usize = 0x8800_0000;

/// 平台时钟频率
pub const CLOCK_FREQ: usize = 12500000;
/// 自定义时钟中断频率
pub const TICK_FREQ: usize = 100;

pub const PID_START: usize = 1000;


pub const PAGE_WIDTH: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_WIDTH;
pub const SV39_PAGE_LEVEL: usize = 3;
pub const SV39_PAGE_INDEX_WIDTH: usize = 9;

pub fn kernel_stack_position(pid: isize) -> (usize, usize) {
    let bottom = TRAMPOLINE - (pid as usize - PID_START) * (KERNEL_STACK_SIZE + GUARD_PAGE_SIZE);
    let top = bottom - KERNEL_STACK_SIZE;
    // println!("ksp: {:X}-{:X}", top, bottom);
    (top, bottom)
}