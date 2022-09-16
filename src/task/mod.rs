pub mod allocater;
pub mod loader;
pub mod stack;
pub mod task;

use log::debug;
use task::*;

const MAX_TASK: usize = 16;

struct TaskManager {
    current: usize,
    tasks: [Task; MAX_TASK],
}

use lazy_static::*;

use crate::{
    config::{APP_BASE_ADDRESS, APP_SIZE_LIMIT},
    stdlib::cell::STCell, syscall::sys_yield,
};
lazy_static! {
    static ref TASK_MANAGER: STCell<TaskManager> = {
        STCell::new(TaskManager {
            current: 0,
            tasks: [Task::default(); MAX_TASK],
        })
    };
}

pub fn set_current_task_state(state: Status) {
    // info!("set current app state: {:?}", state);
    let mut manager = TASK_MANAGER.borrow_mut();
    let i = manager.current;
    manager.tasks[i].state = state;
}

pub fn set_current_task(uid: usize) {
    let mut manager = TASK_MANAGER.borrow_mut();
    manager.current = uid;
}

pub fn get_current_task() -> *mut Task {
    let manager = TASK_MANAGER.borrow();
    &manager.tasks[manager.current] as *const Task as *mut Task
}
pub fn get_next_task() -> Option<*mut Task> {
    let manager = TASK_MANAGER.borrow();
    let mut start = (manager.current + 1) % manager.tasks.len();
    for _ in 0..manager.tasks.len() {
        // info!("poll task_{}: {:?}", start, manager.tasks[start].state);
        match manager.tasks[start].state {
            Status::Init | Status::Ready => {
                debug!("go to app_{}", start);
                return Some(&manager.tasks[start] as *const Task as *mut Task);
            }
            _ => {
                start = (start + 1) % manager.tasks.len();
            }
        }
    }
    None
}

pub fn init() {
    let (nums, app_starts) = loader::get_apps();
    let tasks = &mut TASK_MANAGER.borrow_mut().tasks;
    for i in 0..nums {
        let start = app_starts[i];
        let len = app_starts[i + 1] - start;
        let entry = APP_BASE_ADDRESS + i * APP_SIZE_LIMIT;
        loader::copy_mem(start, entry, len);
        tasks[i + 1] = Task::new(entry, i + 1);
    }
}

pub fn to_next_app() -> ! {
    set_current_task_state(Status::Block);
    sys_yield();
    panic!()
}