use core::fmt::Display;

use alloc::boxed::Box;
use lazy_static::lazy_static;
use log::info;
use riscv::register::sstatus::SPP;

use super::stack::allocate_stack;
use crate::{
    stdlib::{cell::STCell, tools::align_size},
    trap::{context::{TrapContext, push_trap_context}, __restore},
};

const S_REG_NUMS: usize = 12;

lazy_static! {
    static ref UID_COUNT: STCell<usize> = STCell::new(1000);
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct TaskContex {
    pub ra: usize,
    pub ksp: usize,
    pub sreg: [usize; S_REG_NUMS],
}

impl Display for TaskContex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ra: {:#X}, ksp: {:#X}", self.ra, self.ksp)
    }
}

// #[derive(Debug)]
pub struct Tigger {
    inner: Box<dyn Fn(*mut Task) + Sync + Send>,
}

impl Tigger {
    pub fn new(f: Box<dyn Fn(*mut Task) + Sync + Send>) -> Self {
        Self {
            inner: f,
        }
    }

    pub fn pull(&self, task: *mut Task) {
        (self.inner)(task);
    }
}

impl Default for Tigger {
    fn default() -> Self {
        Self { inner: Box::new(|_task|{}) }
    }
}

// #[derive(Debug)]
pub struct Task {
    pub raw_priority: i8,
    pub priority: i8,
    pub uid: usize,
    pub state: TaskStatus,
    pub cx: TaskContex,
    pub trigger: Tigger,
}

impl Display for Task {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{uid: {}, state: {:?}\ncx: {{{}}}}}", self.uid, self.state, self.cx)
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        info!("Drop task: {}", self.uid);
    }
}

impl Task {
    pub fn new(entry: usize, priority: i8, privilege_level: SPP) -> Self {
        // (user stack, kernel stack)
        let (usp, mut ksp) = allocate_stack();
        ksp = push_trap_context(ksp, TrapContext::init(entry, usp, privilege_level)) as usize;
        *(UID_COUNT.borrow_mut()) += 1;
        Task {
            uid: *UID_COUNT.borrow(),
            state: TaskStatus::Ready,
            cx: TaskContex {
                ra: __restore as usize,
                ksp,
                sreg: [0; S_REG_NUMS],
            },
            raw_priority: priority,
            priority,
            trigger: Tigger::default(),
        }
    }
    pub fn pull(&mut self) {
        self.trigger.pull(self as *const _ as *mut Task);
    }
    pub fn empty_task() -> Self {
        Task::default()
    }
    pub fn set_state(&mut self, state: TaskStatus) {
        self.state = state
    }
    pub fn reset_priority(&mut self) {
        self.priority = self.raw_priority;
    }
    pub fn down(&mut self) {
        self.priority = self.priority.wrapping_sub(1);
    }
}

impl Default for Task {
    fn default() -> Self {
        Task {
            uid: 0,
            state: TaskStatus::Exited,
            cx: TaskContex::default(),
            raw_priority: 0,
            priority: 0,
            trigger: Tigger::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum TaskStatus {
    #[default]
    Exited,
    Ready,
    Running,
    Block,
}

