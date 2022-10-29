use alloc::boxed::Box;

use crate::{
    println,
    task::{
        block_and_run_next, get_current_task,
        task::{Task, TaskStatus, Tigger},
    },
    timer::{self, get_time_ms},
};

pub fn sys_get_time() -> isize {
    timer::get_time_ms() as isize
}

pub fn sys_sleep(ms: usize) -> isize {
    let expire_time = get_time_ms() + ms;
    let task = get_current_task();
    unsafe {
        (*task).trigger = Tigger::new(Box::new(move |task: *mut Task| {
            if get_time_ms() >= expire_time {
                (*task).set_state(TaskStatus::Ready);
            }
        }))
    }
    // println!("Sleep");
    block_and_run_next();
    // println!("Wake");
    0
}
