use core::mem::swap;

use crate::{print, println, stdlib::cell::STCell, task::switch::__switch, config::TRAP_CONTEXT, mem::page_table::PageTableEntry, trap::context::TrapContext};

use super::task::{Task, TaskContex, TaskStatus};
use alloc::{boxed::Box, collections::BinaryHeap, vec::Vec};
use log::info;

pub type TaskPin = Box<Task>;

const LOWEST_PRIORITY: i8 = -4;
const HIGHEST_PRIORITY: i8 = 6;

pub struct TaskManager {
    inner: STCell<TaskManagerInner>,
}

struct TaskManagerInner {
    current: Option<TaskPin>,
    priority_queue: BinaryHeap<TaskPin>,
    _exited: Vec<TaskPin>,
}

impl TaskManagerInner {
    /// This has O(n) time complexity.
    fn init_heap(&mut self) {
        let mut uninit_heap = BinaryHeap::with_capacity(self.priority_queue.len());
        swap(&mut self.priority_queue, &mut uninit_heap);
        let mut uninit_heap = uninit_heap.into_vec();
        uninit_heap
            .iter_mut()
            .for_each(|task| task.reset_priority());
        self.priority_queue = BinaryHeap::from(uninit_heap);
    }

    fn push_task(&mut self, task: TaskPin) {
        self.priority_queue.push(task);
    }

    fn peek_task(&self) -> Option<&TaskPin> {
        self.priority_queue.peek()
    }

    fn pop_task(&mut self) -> Option<TaskPin> {
        self.priority_queue.pop()
    }

    fn mark_current_task(&mut self, state: TaskStatus) {
        self.current.as_mut().unwrap().set_state(state);
    }

    fn select_next(&mut self) -> *mut Task {
        // 将当前任务放入优先级队列
        if let Some(task) = self.current.take() {
            self.push_task(task);
            let top_task = unsafe { self.peek_task().unwrap_unchecked() };
            // 当最高的优先级低于一定值时重置所有任务的优先级
            if top_task.priority <= LOWEST_PRIORITY {
                self.init_heap()
            }
        }

        
        // 按实时优先级寻找Ready的任务
        while let Some(mut task) = self.pop_task() {
            match task.state {
                TaskStatus::Exited => (),
                TaskStatus::Ready => {
                    task.down();
                    task.state = TaskStatus::Running;
                    self.current = Some(task);
                    break;
                }
                TaskStatus::Block => {
                    task.pull();
                    task.down();
                    self.push_task(task);
                }
                _ => {
                    task.down();
                    self.push_task(task);
                }
            }
        }
        if let Some(task) = self.current.as_mut() {
            task.as_mut()
        } else {
            panic!("All applications completed!");
        }
    }
    fn current_task(&mut self) -> *mut Task {
        self.current.as_mut().unwrap().as_mut() as *mut Task
    }
    pub fn get_task(&mut self, uid: usize) -> Option<*mut Task> {
        if let Some(task) = &self.current {
            if task.uid == uid {
                return Some(task.as_ref() as *const _ as *mut Task);
            }
        }
        for task in self.priority_queue.iter() {
            if task.uid == uid {
                return Some(task.as_ref() as *const _ as *mut Task);
            }
        }
        None
    }
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            inner: STCell::new(TaskManagerInner {
                current: None,
                priority_queue: BinaryHeap::new(),
                _exited: Vec::new(),
            }),
        }
    }

    pub fn current_task(&self) -> *mut Task {
        self.inner.borrow_mut().current_task()
    }

    pub fn mark_current_task(&self, state: TaskStatus) {
        self.inner.borrow_mut().mark_current_task(state);
    }

    pub fn push_task(&self, task: TaskPin) {
        self.inner.borrow_mut().push_task(task);
    }

    pub fn switch_next(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task();
        let next = inner.select_next();
        drop(inner);
        unsafe {
            __switch(
                &(*current).cx as *const _ as *mut TaskContex,
                &(*next).cx as *const _ as *mut TaskContex,
            );
        }
    }

    pub fn get_task(&self, uid: usize) -> Option<*mut Task> {
        if unsafe { (*self.current_task()).uid } == uid {
            return Some(self.current_task());
        }
        for task in self.inner.borrow().priority_queue.iter() {
            if task.uid == uid {
                return Some(task.as_ref() as *const _ as *mut Task);
            }
        }
        None
    }

    pub unsafe fn current_task_trap_cx(&self) -> *mut TrapContext {
        (*self.current_task()).trap_cx.0 as *mut TrapContext
    }

    pub fn go_next_app(&self) -> ! {
        let mut inner = self.inner.borrow_mut();
        let next = inner.select_next();
        drop(inner);
        unsafe {
            __switch(
                &TaskContex::default() as *const _ as *mut TaskContex,
                &(*next).cx as *const _ as *mut TaskContex,
            );
        }
        unreachable!()
    }
}
