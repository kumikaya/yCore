#![no_std]
#![no_main]
#![feature(panic_info_message)]
// 自定义测试框架
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)]
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

mod init;
mod lang_items;
mod sbi;
// mod test;
mod config;
mod debug;
mod mem;
mod stdlib;
mod syscall;
mod task;
mod timer;
mod trap;

use core::arch::global_asm;

use crate::stdlib::logging;

extern crate alloc;

global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    init::clear_bss();
    logging::init();
    mem::heap_alloc::init_heap();
    mem::heap_alloc::heap_test();
    #[cfg(test)]
    test_main();

    trap::init();
    task::allocater::init();
    task::init();
    trap::enable_timer_interrupt();
    // batch::init();
    // batch::run_next_app();
    task::run_first_app()
}
