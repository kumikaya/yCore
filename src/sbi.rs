#![allow(unused)] // 此行在文件最开头
use core::arch::asm;
use log::info;
use sbi_rt::{self, SbiRet};

use crate::{println, rust_main, _start, config::HART_NUMBER, task::scheduler::get_hartid};
const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut result;
    unsafe {
        asm!(
            "li a6, 0",
            "ecall",
            inlateout("a0") arg0 => result,
            in("a1") arg1,
            in("a2") arg2,
            in("a7") which,
        );
    }
    result
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

#[inline]
pub fn set_timer(timer: usize) {
    sbi_rt::set_timer(timer as u64);
}

pub fn shutdown() -> ! {
    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::NoReason);
    panic!("It should shutdown!");
}


#[inline]
pub fn hart_start(hartid: usize, start_addr: usize, opaque: usize) {
    sbi_rt::hart_start(hartid, start_addr, opaque);
}

pub fn start_all_hart() {
    for id in 0..HART_NUMBER {
        if get_hartid() != id {
            sbi_rt::hart_start(id, _start as usize, 0);
        }
    }
}
