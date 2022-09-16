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
mod test;
mod debug;
mod stdlib;
mod syscall;
mod trap;
mod batch;

use core::arch::global_asm;


use crate::stdlib::logging;


global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    logging::init();

    #[cfg(test)]
    test_main();
    
    init::clear_bss();
    trap::init();
    batch::init();
    batch::run_next_app();
    // panic!("Should not reach here.");
}

