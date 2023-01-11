use alloc::vec::Vec;
use lazy_static::lazy_static;

use crate::{config::PID_START, tools::cell::STRefCell};

#[derive(Debug)]
pub struct PidHandle(pub isize);

struct PidAllocator {
    current: isize,
    recycled: Vec<isize>,
}

impl PidAllocator {
    pub fn new() -> Self {
        PidAllocator {
            current: PID_START as isize,
            recycled: Vec::new(),
        }
    }
    pub fn alloc(&mut self) -> PidHandle {
        if let Some(pid) = self.recycled.pop() {
            PidHandle(pid)
        } else {
            self.current += 1;
            PidHandle(self.current - 1)
        }
    }
    pub fn dealloc(&mut self, pid: isize) {
        assert!(pid < self.current);
        assert!(
            self.recycled.iter().find(|ppid| **ppid == pid).is_none(),
            "pid {} has been deallocated!",
            pid
        );
        self.recycled.push(pid);
    }
}

lazy_static! {
    static ref PID_ALLOCATOR: STRefCell<PidAllocator> = STRefCell::new(PidAllocator::new());
}

pub fn pid_alloc() -> PidHandle {
    PID_ALLOCATOR.borrow_mut().alloc()
}

impl Drop for PidHandle {
    fn drop(&mut self) {
        PID_ALLOCATOR.borrow_mut().dealloc(self.0);
    }
}
