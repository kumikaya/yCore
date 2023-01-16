use core::task::Poll;

use alloc::{boxed::Box, sync::Arc, vec::Vec};

use crate::timer::get_time_ms;

use super::task::{TaskStatus, SharedStatus, Task};

pub type FutureBox = Box<dyn Future<Output = ()> + Send + Sync + 'static>;

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
    state: TaskStatus,
    shared_data: Arc<SharedStatus>,
}

impl TaskWaiter {
    pub fn new(task: Arc<SharedStatus>, state: TaskStatus) -> Self {
        Self { shared_data: task, state }
    }
}

impl Future for TaskWaiter {
    type Output = ();

    fn poll(&self) -> Poll<Self::Output> {
        if *self.shared_data.state.lock() == self.state {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub struct ChildrenWaiter {
    shared_datas: Vec<Arc<SharedStatus>>,
}


impl ChildrenWaiter {
    pub fn new(parent: Task) -> Self {
        let children = &parent.tree.borrow().children;
        let mut shared_datas = Vec::with_capacity(children.len());
        for child in children {
            shared_datas.push(child.shared_state.clone());
        }
        Self { shared_datas }
    }
}

impl Future for ChildrenWaiter {
    type Output = ();

    fn poll(&self) -> Poll<Self::Output> {
        if self.shared_datas.is_empty() {
            return Poll::Ready(());
        }
        for state in self.shared_datas.iter() {
            if state.exit_code.lock().is_some() {
                return Poll::Ready(());
            }
        }
        Poll::Pending
    }
}
