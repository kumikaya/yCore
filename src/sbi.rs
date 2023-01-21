#![allow(unused)] // 此行在文件最开头
use core::arch::asm;
use log::{error, info};
use sbi_rt::{self, SbiRet};

use crate::{_start, config::NUM_HARTS, println, rust_main};
const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

fn sbi_call(which: usize, args: [usize; 3]) -> usize {
    let mut result;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") args[0] => result,
            in("a1") args[1],
            in("a2") args[2],
            in("a7") which,
        );
    }
    result
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, [c, 0, 0]);
}

pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, [0, 0, 0])
}

#[inline]
pub fn set_timer(timer: usize) {
    sbi_rt::set_timer(timer as u64);
}

pub fn shutdown() -> ! {
    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::NoReason);
    panic!("It should shutdown!");
}

pub fn start_all_hart() {
    for id in 0..NUM_HARTS {
        if get_hartid() != id {
            sbi_rt::hart_start(id, _start as usize, 0);
        }
    }
}

pub fn halt() {
    unsafe { riscv::asm::wfi() }
}

#[inline]
pub fn get_hartid() -> usize {
    let hartid: usize;
    unsafe {
        asm! {r"
            mv {x}, tp",
            x = out(reg) hartid
        }
    };
    hartid
}
