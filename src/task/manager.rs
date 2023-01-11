use core::task::Poll;

use super::task::{TaskControlBlock, TaskStatus};
use alloc::{collections::VecDeque, sync::Arc};

pub struct TaskManager {
    queue: VecDeque<Arc<TaskControlBlock>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
    pub fn push(&mut self, task: Arc<TaskControlBlock>) {
        self.queue.push_back(task);
    }
    pub fn pop(&mut self) -> Option<Arc<TaskControlBlock>> {
        while let Some(task) = self.queue.pop_front() {
            let mut inner = task.inner.borrow_mut();
            match inner.get_state() {
                TaskStatus::Ready => {
                    drop(inner);
                    return Some(task);
                }
                TaskStatus::Blocked(tigger) => {
                    match tigger.poll() {
                        Poll::Ready(_) => {
                            inner.set_state(TaskStatus::Ready);
                        }
                        Poll::Pending => (),
                    }
                    drop(inner);
                    self.queue.push_back(task);
                }
                _ => unreachable!(),
            }
        }
        None
    }
}
