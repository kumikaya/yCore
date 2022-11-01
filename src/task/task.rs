use core::fmt::Display;

use alloc::boxed::Box;
use lazy_static::lazy_static;
use log::info;
use riscv::register::sstatus::SPP;
use xmas_elf::ElfFile;

use crate::{
    config::{kernel_stack_position, KERNEL_STACK_BOTTOM, PAGE_SIZE, TRAP_CONTEXT},
    mem::{
        address::{PhysAddr, VirtAddr},
        memory_set::{kernel_token, push_kernel_stack, MemorySet, KERNEL_SPACE},
    },
    println,
    stdlib::cell::STCell,
    task::{get_current_task, get_task},
    trap::{
        context::{push_trap_context, TrapContext},
        user_trap_return,
    },
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
        Self { inner: f }
    }

    pub fn pull(&self, task: *mut Task) {
        (self.inner)(task);
    }
}

impl Default for Tigger {
    fn default() -> Self {
        Self {
            inner: Box::new(|_task| {}),
        }
    }
}

// #[derive(Debug)]
pub struct Task {
    pub raw_priority: i8,
    pub priority: i8,
    pub uid: usize,
    pub state: TaskStatus,
    pub memory_set: MemorySet,
    pub trap_cx: PhysAddr,
    pub cx: TaskContex,
    pub trigger: Tigger,
}

impl Display for Task {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{uid: {}, state: {:?}, cx: {{{}}}, satp: {:#x}}}",
            self.uid,
            self.state,
            self.cx,
            self.memory_set.token()
        )
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
    pub fn from_elf(elf: ElfFile) -> Self {
        *(UID_COUNT.borrow_mut()) += 1;
        let uid = *UID_COUNT.borrow();
        let (memory_set, entry, usp) = MemorySet::from_elf(&elf);
        let (ksp_top, ksp_bottom) = kernel_stack_position(uid);
        println!("alloc kernel stack: [{:#x}..{:#x})", ksp_top, ksp_bottom);
        push_kernel_stack(VirtAddr::from(ksp_top), VirtAddr::from(ksp_bottom));

        let cx = TrapContext::init(entry, usp, ksp_bottom, kernel_token(), SPP::User);

        let cx_pa = memory_set.va_translate(TRAP_CONTEXT).unwrap();
        unsafe {
            (*(cx_pa.0 as *mut TrapContext)) = cx;
        }
        // let trap_cx = unsafe { push_trap_context(ksp_bottom, entry, usp, kernel_token()) as usize };

        Task {
            raw_priority: 0,
            priority: 0,
            uid,
            state: TaskStatus::Ready,
            memory_set,
            trap_cx: cx_pa,
            cx: TaskContex {
                ra: user_trap_return as usize,
                ksp: ksp_bottom,
                sreg: [0; S_REG_NUMS],
            },
            trigger: Tigger::default(),
        }
    }

    pub fn pull(&mut self) {
        self.trigger.pull(self as *const _ as *mut Task);
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
            memory_set: MemorySet::new_bare(),
            trap_cx: PhysAddr::default(),
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
