use crate::{
    println, stdlib::cell::STCell, task::switch::__switch, trap::context::TrapContext,
};

use super::task::{Task, TaskContex, TaskStatus};
use alloc::{
    boxed::Box,
    collections::VecDeque,
};
use log::info;


pub struct TaskManager {
    inner: STCell<TaskManagerInner>,
}

struct TaskManagerInner {
    deque: VecDeque<Box<Task>>,
}

impl TaskManagerInner {
    #[inline]
    fn push_task(&mut self, task: Task) {
        self.deque.push_back(Box::new(task));
    }

    fn mark_current_task(&mut self, state: TaskStatus) {
        self.deque[0].set_state(state);
    }

    fn select_next(&mut self) -> *mut Task {
        let current_task = self.deque.pop_front().unwrap();
        self.deque.push_back(current_task);
        while let Some(mut task) = self.deque.pop_front() {
            match task.state {
                TaskStatus::Exited => (),
                TaskStatus::Ready => {
                    task.state = TaskStatus::Running;
                    self.deque.push_front(task);
                    break;
                }
                TaskStatus::Block => {
                    task.poll();
                    self.deque.push_back(task);
                }
                _ => {
                    self.deque.push_back(task);
                }
            }
        }
        self.deque
            .get_mut(0)
            .unwrap_or_else(|| {
                panic!("All applications completed!");
            })
            .as_mut() as *mut Task
    }
    fn current_task(&mut self) -> *mut Task {
        self.deque[0].as_mut() as *mut Task
    }
    pub fn get_task(&self, uid: usize) -> Option<*const Task> {
        for task in self.deque.iter() {
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
                deque: VecDeque::new(),
            }),
        }
    }

    pub fn current_task(&self) -> *mut Task {
        self.inner.borrow_mut().current_task()
    }

    pub fn mark_current_task(&self, state: TaskStatus) {
        self.inner.borrow_mut().mark_current_task(state);
    }

    #[inline]
    pub fn push_task(&self, task: Task) {
        self.inner.borrow_mut().push_task(task);
    }

    pub fn switch_next(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task();
        let next = inner.select_next();
        drop(inner);
        unsafe {
            __switch(
                &(*current).task_cx as *const _ as *mut TaskContex,
                &(*next).task_cx as *const _ as *mut TaskContex,
            );
        }
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
                &mut TaskContex::default() as *mut TaskContex,
                &(*next).task_cx as *const _ as *mut TaskContex,
            );
        }
        unreachable!()
    }

    pub fn get_task(&self, uid: usize) -> Option<*const Task> {
        self.inner.borrow().get_task(uid)
    }
}
