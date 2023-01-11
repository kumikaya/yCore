#![no_std]
#![no_main]
#![feature(box_syntax)]
#![feature(const_trait_impl, step_trait)]
#![feature(fn_align, naked_functions, asm_const)]
#![feature(panic_info_message, alloc_error_handler)]
#![feature(default_free_fn)]
mod config;
mod drivers;
mod fs;
mod kernel;
mod lang_items;
mod mem;
mod sbi;
mod syscall;
mod task;
mod timer;
mod tools;
mod trap;

use riscv::register;

use crate::{config::KERNEL_INIT_STACK_SIZE, tools::logging};
use core::{
    arch::{asm, global_asm},
    slice, sync::atomic::{AtomicBool, Ordering},
};
extern crate alloc;

#[cfg(feature = "qemu")]
#[path = "boards/qemu.rs"]
mod board;


global_asm!(include_str!("link_app.S"));

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    #[link_section = ".bss.stack"]
    static mut KERNEL_STACK: [u8; KERNEL_INIT_STACK_SIZE] = [0; KERNEL_INIT_STACK_SIZE];
    asm! {"
        la  sp, {stack} + {stack_size}
        call {main}",
        stack = sym KERNEL_STACK,
        stack_size = const KERNEL_INIT_STACK_SIZE,
        main = sym rust_main,
        options(noreturn)
    }
}

fn clear_bss() {
    extern "C" {
        fn stack_bottom();
        fn ebss();
    }
    let start = stack_bottom as usize;
    let len = ebss as usize - start;
    unsafe {
        slice::from_raw_parts_mut(start as *mut u8, len).fill(0);
    }
}

pub fn rust_main(hartid: usize, _dtb_pa: usize) -> ! {
    // 初始化bss段
    clear_bss();
    logging::init();
    mem::init();
    // 中断初始化
    trap::init();
    // task::init();
    // trap::enable_timer_interrupt();

    config::config_align_check();
    #[cfg(feature = "debug_test")]
    {
        mem::heap_allocator::heap_test();
        mem::frame_allocator::frame_allocator_test();
        mem::memory_set::identical_map_test();
        mem::memory_set::framed_map_test();
    }

    // #[cfg(test)]
    // test_main();
    kernel::init();
    task::app_info::list_apps();
    kernel::run_first_app()
}
