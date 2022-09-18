use super::task::{Task, TaskStatus};
use alloc::{boxed::Box, collections::VecDeque};
pub type TaskBox = Box<Task>;

pub struct TaskManager {
    pub queue: VecDeque<TaskBox>,
}

impl TaskManager {
    pub fn push(&mut self, task: TaskBox) {
        self.queue.push_back(task);
    }
    pub fn fetch_next_set_current(&mut self, new_state: TaskStatus) -> *mut Task {
        let mut current = self.queue.pop_front().unwrap();
        current.state = new_state;
        self.queue.push_back(current);
        while let Some(mut task) = self.queue.pop_front() {
            match task.state {
                TaskStatus::Ready => {
                    task.state = TaskStatus::Running;
                    self.queue.push_front(task);
                    break;
                }
                TaskStatus::Exited => (),
                _ => {
                    self.queue.push_back(task);
                }
            }
        }
        self.queue.front().unwrap().as_ref() as *const _ as *mut Task
    }
    pub fn get_current(&mut self) -> *mut Task {
        self.queue.front().unwrap().as_ref() as *const _ as *mut Task
    }
}
