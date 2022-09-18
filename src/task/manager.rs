use core::{cmp::Reverse, ptr, mem::swap};

use crate::print;

use super::task::{Task, TaskStatus};
use alloc::{
    boxed::Box,
    collections::{BinaryHeap, VecDeque},
};
use log::info;
pub type TaskBox = Box<Task>;

const LOWEST_PRIORITY: i8 = 4;
const HIGHEST_PRIORITY: i8 = 0;

pub struct TaskManager {
    pub current: Option<TaskBox>,
    pub priority_queue: BinaryHeap<Reverse<TaskBox>>,
}

impl TaskManager {
    pub fn push(&mut self, task: TaskBox) {
        self.priority_queue.push(Reverse(task));
        // self.queue.push_back(task);
    }

    fn init_heap(&mut self) {
        // info!("INIT HEAP!");
        let mut uninit_heap = BinaryHeap::with_capacity(self.priority_queue.len());
        swap(&mut self.priority_queue, &mut uninit_heap);
        let mut uninit_heap = uninit_heap.into_vec();
        uninit_heap.iter_mut().for_each(|Reverse(task)| task.reset_priority());
        self.priority_queue = BinaryHeap::from(uninit_heap);
    }

    pub fn fetch_next_set_current(&mut self, new_state: TaskStatus) -> *mut Task {
        if let Some(mut task) = self.current.take() {
            task.state = new_state;
            self.priority_queue.push(Reverse(task));
            let Reverse(top_task) = unsafe { self.priority_queue.peek().unwrap_unchecked() };
            if top_task.priority >= LOWEST_PRIORITY {
                self.init_heap()
            }
        }
        // info!("{:#?}", self.priority_queue);
        while let Some(Reverse(mut task)) = self.priority_queue.pop() {
            match task.state {
                TaskStatus::Exited => (),
                TaskStatus::Ready => {
                    // info!("{:?}", task);
                    task.down();
                    task.state = TaskStatus::Running;
                    self.current = Some(task);
                    break;
                }
                _ => {
                    task.down();
                    self.priority_queue.push(Reverse(task))
                }
            }
        }
        self.current.as_mut().expect("No Task!").as_mut() as *mut Task
    }
    pub fn get_current(&mut self) -> *mut Task {
        unsafe { self.current.as_mut().unwrap_unchecked().as_mut() as *mut Task }
    }
}
