use crate::{
    println,
    task::{raw_yield, task::TaskStatus, to_yield, exit_and_run_next},
};

pub fn sys_exit(code: i32) -> ! {
    println!("[Kernel] App exit with code {}", code);
    exit_and_run_next()
}

pub fn sys_get_pid() -> isize {
    todo!()
}

pub fn sys_yield() -> isize {
    to_yield();
    0
}
