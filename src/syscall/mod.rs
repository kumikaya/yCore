mod fs;
mod process;
mod sys;
pub mod switch;
use self::{fs::*, process::*, sys::*};

pub use process::sys_yield;

const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_TIME: usize = 169;
const SYSCALL_GET_PID: usize = 172;

const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;

pub fn syscall(syscall_id: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    match syscall_id {
        SYSCALL_READ    => sys_read(arg0, arg1 as *const u8, arg2),
        SYSCALL_WRITE   => sys_write(arg0, arg1 as *const u8, arg2),
        SYSCALL_EXIT    => sys_exit(arg0 as i32),
        SYSCALL_YIELD   => sys_yield(),
        SYSCALL_TIME    => sys_get_time(),
        SYSCALL_GET_PID => sys_get_pid(),
        _ => todo!(),
    }
}