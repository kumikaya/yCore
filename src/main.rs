#![no_std]
#![no_main]
#![feature(derive_const)]
#![feature(atomic_mut_ptr)]
#![feature(const_trait_impl, step_trait)]
#![feature(panic_info_message, alloc_error_handler)]
#![feature(fn_align, naked_functions, asm_const, default_free_fn)]
#![feature(allow_internal_unstable)]
#![feature(maybe_uninit_uninit_array)]
#![feature(const_maybe_uninit_uninit_array)]
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
mod timer;
mod tools;
mod trap;

use crate::{
    config::{NUM_HARTS, KERNEL_INIT_STACK_SIZE},
    mm::memory_set,
    tools::logging,
};
use core::{
    arch::asm,
    slice,
    sync::atomic::{AtomicBool, Ordering}, hint,
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
    asm! {"
        mv tp, a0
        call {locate_stack}
        call {main}",
        locate_stack = sym locate_stack,
        main = sym rust_main,
        options(noreturn)
    }
}

pub const STACK_SIZE: usize = NUM_HARTS * KERNEL_INIT_STACK_SIZE;
#[link_section = ".bss.stack"]
pub static mut KERNEL_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

/// 为每个硬件线程分配初始化栈
#[naked]
unsafe extern "C" fn locate_stack() -> ! {
    asm! {"
        la sp, {stack_top}
        li t0, {per_stack_size}
        mv t1, a0
        addi t1, t1, 1
    1:  add sp, sp, t0
        addi t1, t1, -1
        bnez t1, 1b
        ret",
        stack_top = sym KERNEL_STACK,
        per_stack_size = const KERNEL_INIT_STACK_SIZE,
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

pub fn rust_main(_hartid: usize, _device_tree_addr: usize) -> ! {
    static GENESIS: AtomicBool = AtomicBool::new(true);
    static DONE: AtomicBool = AtomicBool::new(false);
    if GENESIS.swap(false, Ordering::AcqRel) {
        // 初始化bss段
        clear_bss();
        logging::init();
        // 启动所有硬件线程
        sbi::start_all_hart();
        mm::init();
        task::add_initproc();
        fs::inode::list_apps();
        DONE.store(true, Ordering::Release);
    }
    while !DONE.load(Ordering::Acquire) {
        hint::spin_loop();
    }
    trap::init();
    // 中断初始化
    memory_set::init_kernel_space();
    task::entrap_task()
}
