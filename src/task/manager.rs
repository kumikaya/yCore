use core::task::Poll;

use super::task::{TaskControlBlock, TaskStatus};
use alloc::{collections::VecDeque, sync::Arc};
use spin::Mutex;

pub struct TaskManager {
    queue: Mutex<VecDeque<Arc<TaskControlBlock>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }
    #[inline]
    pub fn push(&self, task: Arc<TaskControlBlock>) {
        self.queue.lock().push_back(task);
    }
    #[inline]
    pub fn pop(&self) -> Option<Arc<TaskControlBlock>> {
        let mut queue = self.queue.lock();
        while let Some(task) = queue.pop_front() {
            // let mut inner = task.inner.borrow_mut();
            let mut state = task.state.lock();
            match &*state {
                TaskStatus::Ready => {
                    drop(state);
                    return Some(task);
                }
                TaskStatus::Blocked(tigger) => {
                    match tigger.poll() {
                        Poll::Ready(_) => {
                            *state = TaskStatus::Ready;
                        }
                        Poll::Pending => (),
                    }
                    drop(state);
                    queue.push_back(task);
                }
                _ => unreachable!(),
            }
        }
        None
    }

    #[inline]
    pub fn pop_spin(&self) -> Arc<TaskControlBlock> {
        loop {
            if let Some(task) = self.pop() {
                break task;
            }
        }
    }
}
