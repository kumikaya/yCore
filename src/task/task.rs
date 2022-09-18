use log::info;

use super::stack::allocate_stack;
use crate::{trap::{TrapContext, __restore}, stdlib::tools::align_size, println};

const S_REG_NUMS: usize = 12;

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct TaskContex {
    pub ra: usize,
    // pub sp: usize,
    pub sreg: [usize; S_REG_NUMS],
}

#[derive(Debug, Clone, Copy)]
pub enum TaskSp {
    Task(usize),
    Trap(usize),
    None,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub uid: usize,
    pub state: TaskStatus,
    pub sp: usize, // pub cx: *mut TaskContex,
}

impl Drop for Task {
    fn drop(&mut self) {
        info!("Drop task: {}", self.uid);
    }
}

impl Task {
    pub fn new(entry: usize, uid: usize) -> Self {
        let (usp, mut ksp) = allocate_stack();
        ksp = push_context(ksp, TrapContext::init(entry, usp));
        let task_cx = (ksp - align_size::<TaskContex>(16)) as *mut TaskContex;
        unsafe { (*task_cx).ra = __restore as usize };
        Task {
            uid,
            state: TaskStatus::Ready,
            sp: task_cx as usize,
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Task {
            uid: 0,
            state: TaskStatus::Exited,
            sp: 0,
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

fn push_context(sp: usize, cx: TrapContext) -> usize {
    let sp = sp - align_size::<TrapContext>(16);
    let cx_ptr = sp as *mut TrapContext;
    unsafe {
        *cx_ptr = cx;
        cx_ptr as usize
    }
}
