#![no_std]
#![no_main]
#![feature(step_trait)]
#![feature(fn_align, naked_functions, asm_const)]
#![feature(panic_info_message, alloc_error_handler)]
mod config;
mod debug;
mod init;
mod lang_items;
mod mem;
mod sbi;
mod stdlib;
mod syscall;
mod task;
mod timer;
mod trap;

use crate::{config::KERNEL_INIT_STACK_SIZE, stdlib::logging};
use core::arch::{asm, global_asm};
extern crate alloc;

#[cfg(feature = "qemu")]
#[path = "boards/qemu.rs"]
mod board;

global_asm!(include_str!("link_app.S"));

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    #[link_section = ".stack"]
    static mut KERNEL_STACK: [u8; KERNEL_INIT_STACK_SIZE] = [0; KERNEL_INIT_STACK_SIZE];
    asm! {"
        la  sp, {stack}
        li  t0, {stack_size}
        add sp, sp, t0
        csrr a0, sstatus
        call {main}",
        stack = sym KERNEL_STACK,
        stack_size = const KERNEL_INIT_STACK_SIZE,
        main = sym rust_main,
        options(noreturn)
    }
}

#[no_mangle]
pub fn rust_main() -> ! {
    // 初始化bss段
    init::clear_bss();
    // init::init_stack_guard();
    // 初始化日志系统
    logging::init();
    mem::heap_allocator::init_heap();

    #[cfg(feature = "debug_test")]
    {
        mem::heap_allocator::heap_test();
        mem::frame_allocator::frame_allocator_test();
        mem::memory_set::identical_map_test();
        mem::memory_set::framed_map_test();
    }

    #[cfg(test)]
    test_main();

    // 中断初始化
    trap::init();
    task::allocater::init();
    task::init();
    // init::stack_cover_test();
    trap::enable_timer_interrupt();
    task::run_first_app()
}
