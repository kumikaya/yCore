use core::{hint, task::Poll};

use alloc::collections::VecDeque;
use log::warn;
use spin::Mutex;

use crate::{
    task::{__switch, task::TaskContext},
    tools::cell::STCell,
};

use super::{
    task::{Task, TaskStatus},
    tigger::Future,
};

pub struct Processor {
    pub hartid: usize,
    pub current: STCell<Option<Task>>,
    queue: TaskQueue,
}

pub struct TaskQueue {
    pub queue: Mutex<VecDeque<Task>>,
}

impl Processor {
    pub fn new(hartid: usize) -> Self {
        Self {
            hartid,
            current: STCell::new(None),
            queue: TaskQueue::new(),
            // task_manager: task_maneger,
        }
    }

    #[inline]
    pub fn set_current(&self, new: Option<Task>) {
        if let Some(task) = &new {
            task.run(self.hartid);
        }
        self.current.set(new);
    }
    pub fn task_num(&self) -> usize {
        self.queue.len()
    }
    pub fn wakee_num(&self) -> usize {
        self.queue.wakee_num()
    }
    pub fn add_task(&self, task: Task) {
        self.queue.push(task)
    }
    pub fn fetch_task(&self) -> Option<Task> {
        if self.queue.wakee_num() > 1 {
            self.queue.pop()
        } else {
            None
        }
    }
}

pub trait Hart {
    fn current_task(&self) -> &Task;
    fn entrap_task(&self) -> !;
    fn schedule(&self);
}

#[inline]
pub fn entrap(task: *mut TaskContext) -> ! {
    static mut HOLE: TaskContext = TaskContext::default();
    unsafe { __switch(&mut HOLE as *mut TaskContext, task) };
    unreachable!()
}

impl Hart for Processor {
    #[inline]
    fn current_task(&self) -> &Task {
        unsafe { (*self.current.as_ptr()).as_ref().unwrap() }
    }

    fn entrap_task(&self) -> ! {
        let next: *mut TaskContext;
        if let Some(task) = self.queue.pop() {
            next = task.task_context();
            self.set_current(Some(task));
        } else {
            // 当无法找到下一个任务时切换到 `processor_init` ,避免任务无法退出
            next = &mut TaskContext::goto_init(self.hartid) as *mut TaskContext;
        }
        entrap(next)
    }

    #[inline]
    fn schedule(&self) {
        // 当前任务在被其它线程获取之前必须保存完 `TaskContext`
        let current_task = self.current.take().unwrap();
        let current = current_task.task_context();
        self.add_task(current_task);

        let next_task = self.queue.pop_spin();
        let next = next_task.task_context();

        self.set_current(Some(next_task));
        unsafe { __switch(current, next) };
    }
}

pub trait Schedule {
    fn exit_current(&self, code: i32) -> !;
    fn blocking_current<T>(&self, tigger: T)
    where
        T: Future<Output = ()> + Sync + Send + 'static;
    fn yield_(&self);
}

impl<T: Hart> Schedule for T {
    fn exit_current(&self, code: i32) -> ! {
        self.current_task().exit(code);
        self.entrap_task()
    }

    fn blocking_current<F>(&self, tigger: F)
    where
        F: Future<Output = ()> + Sync + Send + 'static,
    {
        self.current_task().block(box tigger);
        self.schedule();
    }

    fn yield_(&self) {
        self.current_task().ready();
        self.schedule();
    }
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.queue.lock().len()
    }
    #[inline]
    pub fn wakee_num(&self) -> usize {
        let queue = self.queue.lock();
        queue
            .iter()
            .fold(0, |acc, task| if task.is_ready() { acc + 1 } else { acc })
    }
    #[inline]
    pub fn push(&self, task: Task) {
        self.queue.lock().push_back(task);
    }
    #[inline]
    pub fn pop(&self) -> Option<Task> {
        let mut queue = self.queue.lock();
        let len = queue.len();
        let mut i = 0;
        while let Some(task) = queue.pop_front() {
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
                            drop(state);
                            return Some(task);
                        }
                        Poll::Pending => (),
                    }
                    drop(state);
                    queue.push_back(task);
                }
                _ => {
                    warn!("The app({})'s parent process exits early.", task.get_pid())
                }
            }
            i += 1;
            if i >= len {
                break;
            }
        }
        None
    }
    #[inline]
    pub fn pop_spin(&self) -> Task {
        loop {
            if let Some(task) = self.pop() {
                break task;
            }
            hint::spin_loop();
        }
    }
}
