#![no_std]
#![no_main]
#![feature(panic_info_message)]
// 自定义测试框架
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

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

mod lang_items;
mod init;
mod sbi;
// mod test;
mod debug;
mod stdlib;
mod syscall;
mod trap;
mod timer;
mod config;
mod task;

use core::arch::global_asm;

use crate::{stdlib::logging, task::to_next_app};


global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    init::clear_bss();
    logging::init();

    #[cfg(test)]
    test_main();
    
    
    trap::init();
    task::allocater::init();
    task::init();
    // batch::init();
    // batch::run_next_app();
    to_next_app();
}

