#![no_std]
#![no_main]
#![feature(box_syntax)]
#![feature(const_trait_impl, step_trait)]
#![feature(panic_info_message, alloc_error_handler)]
#![feature(fn_align, naked_functions, asm_const, default_free_fn)]

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
    config::{HART_NUMBER, KERNEL_INIT_STACK_SIZE},
    tools::logging, mm::memory_set,
};
use core::{
    arch::asm,
    slice,
    sync::atomic::{AtomicBool, Ordering},
};
extern crate alloc;

#[cfg(feature = "qemu")]
#[path = "boards/qemu.rs"]
mod board;

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    asm! {"
        call {locate_stack}
        call {main}",
        locate_stack = sym locate_stack,
        main = sym rust_main,
        options(noreturn)
    }
}

pub const STACK_SIZE: usize = HART_NUMBER * KERNEL_INIT_STACK_SIZE;
#[link_section = ".bss.stack"]
pub static mut KERNEL_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

/// 为每个硬件线程分配初始化栈
#[naked]
unsafe extern "C" fn locate_stack(hartid: usize) -> ! {
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

pub fn rust_main(hartid: usize, _dtb_pa: usize) -> ! {
    static GENESIS: AtomicBool = AtomicBool::new(true);
    if GENESIS.swap(false, Ordering::AcqRel) {
        // 初始化bss段
        clear_bss();
        logging::init();
        // drivers::print_dtb(_dtb_pa);
        mm::init();
        task::add_initproc();
        fs::inode::list_apps();
        // fs::inode::inode_test();
        // 启动所有硬件线程
        sbi::start_all_hart();
    }
    // 中断初始化
    trap::init();
    memory_set::init_kernel_space();
    task::entrap_task(hartid)
}
