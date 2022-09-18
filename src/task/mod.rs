pub mod allocater;
pub mod loader;
pub mod manager;
pub mod stack;
pub mod task;

use alloc::{boxed::Box, collections::{VecDeque, BinaryHeap}};

use task::*;

use crate::{
    config::{APP_BASE_ADDRESS, APP_SIZE_LIMIT},
    stdlib::cell::STCell,
    task::manager::TaskManager,
};

use lazy_static::lazy_static;
lazy_static! {
    static ref TASK_MANAGER: STCell<TaskManager> = {
        STCell::new(TaskManager {
            current: None,
            priority_queue: BinaryHeap::new(),
        })
    };
}

pub fn get_current() -> *mut Task {
    TASK_MANAGER.borrow_mut().get_current()
}

pub fn fetch_next_set_current(new_state: TaskStatus) -> *mut Task {
    TASK_MANAGER.borrow_mut().fetch_next_set_current(new_state)
}

pub fn init() {
    let (nums, app_starts) = loader::get_apps();
    let task_manager = &mut TASK_MANAGER.borrow_mut();
    for i in 0..nums {
        let start = app_starts[i];
        let len = app_starts[i + 1] - start;
        let entry = APP_BASE_ADDRESS + i * APP_SIZE_LIMIT;
        loader::copy_mem(start, entry, len);
        task_manager.push(Box::new(Task::new(entry, i + 1, i as i8)));
    }
}

use crate::syscall::switch::switch;


pub fn raw_yield(new_state: TaskStatus) {
    let current_task = get_current();
    let next_task = fetch_next_set_current(new_state); // .expect("No task!");
    switch(current_task, next_task);
}

pub fn exit_and_run_next() -> ! {
    raw_yield(TaskStatus::Exited);
    unreachable!()
}

pub fn run_first_app() -> ! {
    let task = fetch_next_set_current(TaskStatus::Ready); // .expect("No task!");
    unsafe {
        (*task).state = TaskStatus::Running;
    }
    static HOLE_STACK: [usize; 32] = [0; 32];
    let hole_task = Task {
        uid: 0,
        state: TaskStatus::Exited,
        sp: &HOLE_STACK as *const _ as usize,
        raw_priority: 0,
        priority: 0,
    };
    switch(&hole_task as *const _ as *mut Task, task);
    unreachable!()
}

pub fn block_and_run_next() {
    raw_yield(TaskStatus::Block)
}

pub fn to_yield() {
    raw_yield(TaskStatus::Ready)
}
