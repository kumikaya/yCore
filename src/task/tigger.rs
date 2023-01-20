use core::task::Poll;

use alloc::{boxed::Box, sync::Arc, vec::Vec};

use crate::timer::get_time_ms;

use super::{task_block::{SharedStatus, Task, TaskStatus}, signal::SignalFlags};

pub type FutureBox = Box<dyn Future<Output = ()> + Send + Sync + 'static>;

pub trait Future {
    type Output;
    fn poll(&self) -> Poll<Self::Output>;
}

pub struct Tigger<F> {
    f: F,
}

impl<F> Tigger<F>
where
    F: Fn() -> bool,
{
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

impl<F> Future for Tigger<F>
where
    F: Fn() -> bool,
{
    type Output = ();

    fn poll(&self) -> Poll<Self::Output> {
        if (self.f)() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
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
    pub fn new(task: &Task, state: TaskStatus) -> Self {
        Self {
            shared_data: task.shared.clone(),
            state,
        }
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
    pub fn new(parent: &Task) -> Self {
        let children = &parent.local.borrow().tree.children;
        let mut shared_datas = Vec::with_capacity(children.len());
        for child in children {
            shared_datas.push(child.shared.clone());
        }
        Self { shared_datas }
    }
}

impl Future for ChildrenWaiter {
    type Output = ();

    fn poll(&self) -> Poll<Self::Output> {
        if self.shared_datas.is_empty()
            || self
                .shared_datas
                .iter()
                .any(|state| state.exit_code.lock().is_some())
        {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub struct SignalWaiter {
    flag: SignalFlags,
    shared_data: Arc<SharedStatus>,
}

impl SignalWaiter {
    pub fn new(task: &Task, flag: SignalFlags) -> Self {
        Self {
            flag,
            shared_data: task.shared.clone(),
        }
    }
}

impl Future for SignalWaiter {
    type Output = ();

    fn poll(&self) -> Poll<Self::Output> {
        let mut signals = self.shared_data.signals.lock();
        if signals.contains(self.flag) {
            *signals ^= self.flag;
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}