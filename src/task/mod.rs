pub mod loader;
pub mod manager;
// pub mod stack;
pub mod task;
pub mod switch;
use core::slice;

use alloc::boxed::Box;
use log::info;
use task::*;

use crate::{
    stdlib::cell::STCell,
    task::manager::TaskManager, mem::{address::{PhysAddr, VirtAddr}, memory_set::MemorySet}, trap::context::TrapContext,
};

use lazy_static::lazy_static;
lazy_static! {
    static ref TASK_MANAGER: STCell<TaskManager> = {
        STCell::new(TaskManager::new())
    };
}

pub fn current_task_trap_cx() -> *mut TrapContext {
    unsafe {
        TASK_MANAGER.borrow().current_task_trap_cx()
    }
}

pub fn get_current_task() -> *mut Task {
    TASK_MANAGER.borrow().current_task()
}

pub fn user_space() -> &'static MemorySet {
    unsafe {
        &(*get_current_task()).memory_set
    }
}

pub fn get_task(uid: usize) -> Option<*mut Task> {
    TASK_MANAGER.borrow().get_task(uid)
}

pub fn user_addr_translate(va: VirtAddr) -> Option<PhysAddr> {
    user_space().va_translate(va)
}

pub fn user_token() -> usize {
    user_space().token()
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
        let elf = xmas_elf::ElfFile::new(
            unsafe {
                slice::from_raw_parts(start as *const u8, len)
            }
        ).unwrap();
        task_manager.push_task(Box::new(Task::from_elf(elf)));
    }
}

#[inline]
pub fn raw_yield(state: TaskStatus) {
    mark_current_task(state);
    TASK_MANAGER.borrow().switch_next();
}

pub fn exit_and_run_next() -> ! {
    info!("exit app.");
    mark_current_task(TaskStatus::Exited);
    TASK_MANAGER.borrow().go_next_app()
}

pub fn run_first_app() -> ! {
    TASK_MANAGER.borrow().go_next_app()
}

pub fn block_and_run_next() {
    raw_yield(TaskStatus::Block);
}

pub fn to_yield() {
    raw_yield(TaskStatus::Ready)
}
