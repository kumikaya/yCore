use core::task::Poll;

use alloc::{boxed::Box, sync::Arc};

use crate::timer::get_time_ms;

use super::task::{TaskControlBlock, TaskStatus};

pub type FutureBox = Box<dyn Future<Output = ()> + Sync + Send>;

pub trait Future {
    type Output;
    fn poll(&self) -> Poll<Self::Output>;
}

pub struct Timer {
    expire_time: usize,
}

impl Timer {
    pub fn new(time: usize) -> Self {
        Self {
            expire_time: time + get_time_ms(),
        }
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(&self) -> Poll<Self::Output> {
        if get_time_ms() >= self.expire_time {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub struct TaskWaiter {
    task: Arc<TaskControlBlock>,
}

impl TaskWaiter {
    pub fn new(task: Arc<TaskControlBlock>) -> Self {
        Self { task }
    }
}

impl Future for TaskWaiter {
    type Output = ();

    fn poll(&self) -> Poll<Self::Output> {
        match self.task.inner.borrow().get_state() {
            TaskStatus::Exited(_) => Poll::Ready(()),
            _ => Poll::Pending,
        }
    }
}

pub struct ChildrenWaiter {
    parent: Arc<TaskControlBlock>,
}

impl ChildrenWaiter {
    pub fn new(parent: Arc<TaskControlBlock>) -> Self {
        Self { parent }
    }
}

impl Future for ChildrenWaiter {
    type Output = ();

    fn poll(&self) -> Poll<Self::Output> {
        let children = &self.parent.inner.borrow_mut().children;
        if children.iter().any(|task| task.exit_code().is_some()) || children.is_empty() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
