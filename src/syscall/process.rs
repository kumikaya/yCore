use log::info;

use crate::task::{to_yield, exit_and_run_next};

pub fn sys_exit(code: i32) -> ! {
    info!("App exit with code {}", code);
    exit_and_run_next()
}

pub fn sys_get_pid() -> isize {
    todo!()
}

pub fn sys_yield() -> isize {
    to_yield();
    0
}
