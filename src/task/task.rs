use core::mem::size_of;
use crate::trap::TrapContext;
use super::stack::allocate_stack;

const S_REG_NUMS: usize = 12;

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct TaskContex {
    pub ra: usize,
    pub sp: usize,
    pub sreg: [usize; S_REG_NUMS],
}

#[derive(Debug, Clone, Copy)]
pub struct Task {
    pub uid: usize,
    pub state: Status,
    pub cx: TaskContex,
}

impl Task {
    pub fn new(entry: usize, uid: usize) -> Self {
        let (usp, mut ksp) = allocate_stack();
        ksp = push_context(ksp, TrapContext::init(entry, usp));
        Task {
            uid,
            state: Status::Init,
            cx: TaskContex {
                ra: 0,
                sp: ksp,
                sreg: [0; S_REG_NUMS],
            },
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Task {
            uid: 0,
            state: Status::Block,
            cx: TaskContex {
                ra: 0,
                sp: 0,
                sreg: [0; S_REG_NUMS],
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Status {
    #[default]
    Empty,
    Init,
    Ready,
    Running,
    Block,
}

fn push_context(sp: usize, cx: TrapContext) -> usize {
    let sp = sp - size_of::<TrapContext>();
    let cx_ptr = sp as *mut TrapContext;
    unsafe {
        *cx_ptr = cx;
        cx_ptr as usize
    }
}
