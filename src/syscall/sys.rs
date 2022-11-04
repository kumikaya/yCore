use crate::{
    task::{block_and_run_next, current_task, task::Tigger},
    timer,
};

pub fn sys_get_time() -> isize {
    timer::get_time_ms() as isize
}

pub fn sys_sleep(ms: usize) -> isize {
    unsafe {
        (*current_task()).trigger = Tigger::timer(ms);
    }
    block_and_run_next();
    0
}
