use crate::{task::task::{TaskContex, Task}, stdlib::tools::align_size};
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

extern "C" {
    pub fn __switch(current_ptr: usize, next: usize);
}

#[inline(always)]
pub fn switch(current: *mut Task, next: *mut Task) {
    unsafe {
        __switch(&(*current).sp as *const _ as usize, (*next).sp);
    }
}