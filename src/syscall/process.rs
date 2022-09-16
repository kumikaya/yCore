
use crate::{
    println,
    task::{
        get_current_task, get_next_task,
        task::{self, TaskContex}, to_next_app, set_current_task,
    },
    trap::__restore,
};

use super::switch::__switch;

pub fn sys_exit(code: i32) -> ! {
    println!("[Kernel] App exit with code {}", code);
    to_next_app()
    // run_next_app()
}

pub fn sys_get_pid() -> isize {
    todo!()
}

pub fn sys_yield() -> isize {
    unsafe {
        let current_task = get_current_task();
        let next_task = get_next_task().expect("No task!");
        let current = &(*current_task).cx as *const TaskContex as *mut TaskContex;
        set_current_task((*next_task).uid);
        let (next, state) = (&((*next_task).cx) as *const TaskContex, (*next_task).state);
        match state {
            task::Status::Init => {
                // let tc = (*next).sp as *const TrapContext;
                // info!("next sp: {:#?}", *tc);
                __restore((*next).sp);
                (*next_task).state = task::Status::Running;
            }
            task::Status::Ready => {
                // (*current_task).state = task::Status::Ready;
                __switch(current, next);
            }
            _ => unreachable!(),
        }
        // do something

        0
    }
}
