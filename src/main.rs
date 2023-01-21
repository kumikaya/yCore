#![no_std]
#![no_main]
#![feature(derive_const)]
#![feature(const_trait_impl, step_trait)]
#![feature(alloc_error_handler)]
#![feature(fn_align, naked_functions)]
#![feature(maybe_uninit_uninit_array)]
#![feature(format_args_nl)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
mod console;
mod config;
mod drivers;
mod fs;
mod lang_items;
mod mm;
mod sbi;
mod syscall;
mod task;
#[cfg(feature = "debug")]
pub mod tests;
mod timer;
mod tools;
mod trap;

use sbi::get_hartid;

use crate::{
    config::{KERNEL_INIT_STACK_SIZE, NUM_HARTS},
    mm::memory_set,
    tools::logging,
};
use core::{
    arch::naked_asm,
    hint, slice,
    sync::atomic::{AtomicBool, Ordering},
};
extern crate alloc;

#[macro_use]
extern crate anyhow;

#[cfg(feature = "qemu")]
#[path = "boards/qemu.rs"]
mod board;

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    naked_asm! {"
        mv tp, a0
        call {locate_stack}
        call {main}",
        locate_stack = sym locate_stack,
        main = sym rust_main,
        options()
    }
}

pub const STACK_SIZE: usize = NUM_HARTS * KERNEL_INIT_STACK_SIZE;
#[link_section = ".bss.stack"]
pub static mut KERNEL_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

/// 为每个硬件线程分配初始化栈
#[naked]
unsafe extern "C" fn locate_stack() -> ! {
    naked_asm! {"
        # a0 == hartid
        # pc == 0x80200000
        # sp == 0x800xxxxx
        la sp, {stack_top}
        li t0, {kernel_stack_size}
        mv t1, a0
        addi t1, t1, 1
    1:  add sp, sp, t0
        addi t1, t1, -1
        bnez t1, 1b
        ret",
        stack_top = sym KERNEL_STACK,
        kernel_stack_size = const KERNEL_INIT_STACK_SIZE,
        options()
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

pub fn rust_main(hartid: usize, _device_tree_addr: usize) -> ! {
    assert_eq!(hartid, get_hartid());
    // 初始化bss段
    clear_bss();
    logging::init();
    mm::init();
    task::add_initproc();
    fs::inode::list_apps();
    // 启动所有硬件线程
    // sbi::start_all_hart();
    // 中断初始化
    trap::init();
    memory_set::init_kernel_space();
    #[cfg(test)]
    test_main();
    task::entrap_task()
}

fn others_main(hartid: usize) -> ! {
    assert_eq!(hartid, get_hartid());
    // 中断初始化
    trap::init();
    memory_set::init_kernel_space();
    task::entrap_task()
}
