use alloc::vec::Vec;
use spin::{Lazy, Mutex};

use crate::config::PID_START;

#[derive(Debug)]
pub struct PidHandle {
    pub id: isize,
}

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
            PidHandle { id: pid }
        } else {
            self.current += 1;
            PidHandle {
                id: self.current - 1,
            }
        }
    }
    pub fn dealloc(&mut self, pid: isize) {
        assert!(pid < self.current);
        assert!(
            !self.recycled.iter().any(|&rpid| rpid == pid),
            "pid {pid} has been deallocated!"
        );
        self.recycled.push(pid);
    }
}

static PID_ALLOCATOR: Lazy<Mutex<PidAllocator>> = Lazy::new(|| Mutex::new(PidAllocator::new()));

pub fn pid_alloc() -> PidHandle {
    PID_ALLOCATOR.lock().alloc()
}

impl Drop for PidHandle {
    fn drop(&mut self) {
        PID_ALLOCATOR.lock().dealloc(self.id);
    }
}
