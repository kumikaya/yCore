use core::{hint, task::Poll, cell::Cell};

use alloc::collections::VecDeque;
use spin::Mutex;

use crate::{
    task::{__switch, scheduler::get_processor, task::TaskContext},
};

use super::{
    task::{Task, TaskStatus},
    tigger::{Future, FutureBox},
};

pub struct Processor {
    pub current: Cell<Option<Task>>,
    queue: TaskQueue,
}

pub struct BlockedTask {
    pub task: Task,
    pub tigger: FutureBox,
}

impl BlockedTask {
    pub fn new(task: Task, tigger: FutureBox) -> Self {
        Self { task, tigger }
    }
    pub fn poll(&self) -> Option<Task> {
        match self.tigger.poll() {
            Poll::Ready(_) => {
                self.task.set_state(TaskStatus::Ready);
                Some(self.task.clone())
            },
            Poll::Pending => None,
        }
    }
}

pub struct TaskQueue {
    pub queue: Mutex<VecDeque<Task>>,
    pub wait_queue: Mutex<VecDeque<BlockedTask>>,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            // hartid,
            current: Cell::new(None),
            queue: TaskQueue::new(),
            // task_manager: task_maneger,
        }
    }

    #[inline]
    pub fn set_current(&self, new: Option<Task>) {
        if let Some(task) = &new {
            task.set_state(TaskStatus::Running);
        }
        self.current.set(new);
    }

    pub fn ready_task_num(&self) -> usize {
        self.queue.ready_task_num()
    }
    pub fn add_task(&self, task: Task) {
        // match *task.state.lock() {
        //     TaskStatus::Ready => self.queue.push_ready(task),
        //     TaskStatus::Wait => self.queue.push_wait(task),
        //     _ => unreachable!(),
        // }
        assert!(task.is_ready());
        self.queue.push_task(task, None)
    }
    pub fn fetch_task(&self) -> Option<Task> {
        if self.queue.ready_task_num() > 1 {
            self.queue.pop_ready()
        } else {
            None
        }
    }
}

pub unsafe fn switch_trampoline() {
    let processor = get_processor();
    processor.set_current(None);
    processor.entrap_task()
}

impl Processor {
    pub fn entrap_task(&self) -> ! {
        let next: *mut TaskContext;
        if unsafe { (*self.current.as_ptr()).is_none() } {
            let task = self.queue.pop_spin();
            next = task.task_context();
            self.set_current(Some(task));
        } else {
            // 当无法找到下一个任务时切换到初始化栈 ,避免在当前栈退出任务
            next = &mut TaskContext::switch_trampoline() as *mut TaskContext;
        }
        static mut HOLE: TaskContext = TaskContext::default();
        unsafe { __switch(&mut HOLE as *mut TaskContext, next) };
        unreachable!()
    }

    /// 如果传入 `tigger` 为 `Some` 则将当前任务置为 `Wait`
    #[inline]
    pub fn schedule(&self, tigger: Option<FutureBox>) {

        let current_task = self.current.take().unwrap();
        let current = current_task.task_context();
        self.queue.push_task(current_task, tigger);
        
        let next_task = self.queue.pop_spin();
        let next = next_task.task_context();

        self.set_current(Some(next_task));
        unsafe { __switch(current, next) };
    }
}

pub trait Schedule {
    fn current_task(&self) -> &Task;
    fn exit_current(&self, code: i32) -> !;
    fn blocking_current<T>(&self, tigger: T)
    where
        T: Future<Output = ()> + Send + Sync + 'static;
    fn yield_(&self);
}

impl Schedule for Processor {
    #[inline]
    fn current_task(&self) -> &Task {
        unsafe { (*self.current.as_ptr()).as_ref().unwrap() }
    }
    fn exit_current(&self, code: i32) -> ! {
        self.current_task().exit(code);
        self.entrap_task()
    }

    fn blocking_current<F>(&self, tigger: F)
    where
        F: Future<Output = ()> + Send + Sync + 'static,
    {
        // 当前任务在被其它线程获取之前必须保存完 `TaskContext`
        self.schedule(Some(box tigger));
    }

    fn yield_(&self) {
        self.schedule(None);
    }
}

pub fn yield_() {
    get_processor().yield_()
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            wait_queue: Mutex::new(VecDeque::new()),
        }
    }
    #[inline]
    pub fn ready_task_num(&self) -> usize {
        self.queue.lock().len()
    }
    #[inline]
    pub fn wait_task_num(&self) -> usize {
        self.wait_queue.lock().len()
    }
    #[inline]
    pub fn push_task(&self, task: Task, tigger: Option<FutureBox>) {
        if let Some(tigger) = tigger {
            task.set_state(TaskStatus::Wait);
            self.wait_queue.lock().push_back(BlockedTask::new(task, tigger))
        } else {
            task.set_state(TaskStatus::Ready);
            self.queue.lock().push_back(task)
        }
        // self.queue.lock().push_back(task);
    }

    pub fn poll_all_wait(&self) {
        let mut wait_queue = self.wait_queue.lock();
        for _ in 0..wait_queue.len() {
            if let Some(wait_task) = wait_queue.pop_front() {
                if let Some(task) = wait_task.poll() {
                    self.push_task(task, None);
                } else {
                    wait_queue.push_back(wait_task);
                }
            }
        }
    }

    #[inline]
    pub fn pop_ready(&self) -> Option<Task> {
        self.queue.lock().pop_front()
    }
    #[inline]
    pub fn pop_spin(&self) -> Task {
        loop {
            if let Some(task) = self.pop_ready() {
                break task;
            }
            for _ in 0..16 {
                hint::spin_loop();
            }
            self.poll_all_wait();
        }
    }
}
