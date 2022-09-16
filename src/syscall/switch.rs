use crate::task::task::TaskContex;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

extern "C" {
    pub fn __switch(current: *mut TaskContex, next: *const TaskContex);
}
