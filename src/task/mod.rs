pub mod allocater;
pub mod loader;
pub mod manager;
pub mod stack;
pub mod task;
pub mod switch;
use alloc::boxed::Box;

use riscv::register::sstatus::SPP;
use task::*;

use crate::{
    config::{APP_BASE_ADDRESS, APP_SIZE_LIMIT},
    stdlib::cell::STCell,
    task::manager::TaskManager, println,
};

use lazy_static::lazy_static;
lazy_static! {
    static ref TASK_MANAGER: STCell<TaskManager> = {
        STCell::new(TaskManager::new())
    };
}

pub fn get_current_task() -> *mut Task {
    TASK_MANAGER.borrow().current_task()
}

fn mark_current_task(state: TaskStatus) {
    TASK_MANAGER.borrow().mark_current_task(state);
}


// 初始化读取App
pub fn init() {
    let (nums, app_starts) = loader::get_apps();
    let task_manager = &mut TASK_MANAGER.borrow_mut();
    for i in 0..nums {
        let start = app_starts[i];
        let len = app_starts[i + 1] - start;
        let entry = APP_BASE_ADDRESS + i * APP_SIZE_LIMIT;
        loader::copy_mem(start, entry, len);
        task_manager.push_task(Box::new(Task::new(entry, 0, SPP::User)));
    }
}

#[inline]
pub fn raw_yield(state: TaskStatus) {
    mark_current_task(state);
    TASK_MANAGER.borrow().run_next();
}

pub fn exit_and_run_next() -> ! {
    raw_yield(TaskStatus::Exited);
    unreachable!()
}

pub fn run_first_app() -> ! {
    TASK_MANAGER.borrow().run_first_app()
}

pub fn block_and_run_next() {
    raw_yield(TaskStatus::Block);
}

pub fn to_yield() {
    raw_yield(TaskStatus::Ready)
}
