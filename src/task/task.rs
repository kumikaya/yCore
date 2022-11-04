use core::fmt::Debug;
use alloc::boxed::Box;
use lazy_static::lazy_static;
use log::info;
use riscv::register::sstatus::SPP;
use xmas_elf::ElfFile;

use crate::{
    config::{kernel_stack_position, TRAP_CONTEXT, UID_START},
    mem::{
        address::{PhysAddr, VirtAddr},
        memory_set::{kernel_token, push_kernel_stack, MemorySet},
    },
    println,
    stdlib::cell::STCell,
    timer::get_time_ms,
    trap::{context::TrapContext, user_trap_return},
};

const S_REG_NUMS: usize = 12;

lazy_static! {
    static ref UID_COUNT: STCell<usize> = STCell::new(UID_START);
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct TaskContex {
    pub ra: usize,
    pub ksp: usize,
    pub sreg: [usize; S_REG_NUMS],
}

pub struct Tigger {
    inner: Box<dyn Fn() -> TaskStatus + Sync + Send>,
}

impl Debug for Tigger {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Tigger").finish()
    }
}

impl Tigger {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() -> TaskStatus + Sync + Send + 'static,
    {
        Self { inner: Box::new(f) }
    }

    pub fn poll(&self) -> TaskStatus {
        (self.inner)()
    }

    pub fn timer(time: usize) -> Self {
        let expire_time = get_time_ms() + time;
        Self::new(move || {
            if get_time_ms() >= expire_time {
                TaskStatus::Ready
            } else {
                TaskStatus::Block
            }
        })
    }
}

impl Default for Tigger {
    fn default() -> Self {
        Self::new(|| TaskStatus::Ready)
    }
}

#[derive(Debug)]
pub struct Task {
    pub uid: usize,
    pub state: TaskStatus,
    pub memory_set: MemorySet,
    pub task_cx: TaskContex,
    pub trap_cx: PhysAddr,
    pub trigger: Tigger,
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

        // 添加内核栈
        let (ksp_top, ksp_bottom) = kernel_stack_position(uid);
        push_kernel_stack(VirtAddr::from(ksp_top), VirtAddr::from(ksp_bottom));

        // 初始化Trap上下文
        let cx = TrapContext::init(entry, usp, ksp_bottom, kernel_token(), SPP::User);
        let cx_pa = memory_set.va_translate(TRAP_CONTEXT).unwrap();
        unsafe {
            (*(cx_pa.0 as *mut TrapContext)) = cx;
        }

        Task {
            uid,
            state: TaskStatus::Ready,
            memory_set,
            task_cx: TaskContex {
                ra: user_trap_return as usize,
                ksp: ksp_bottom,
                sreg: [0; S_REG_NUMS],
            },
            trap_cx: cx_pa,
            trigger: Tigger::default(),
        }
    }

    pub fn poll(&mut self) {
        self.state = self.trigger.poll();
    }

    pub fn set_state(&mut self, state: TaskStatus) {
        self.state = state
    }
}

impl Default for Task {
    fn default() -> Self {
        Task {
            uid: 0,
            state: TaskStatus::Exited,
            memory_set: MemorySet::new_bare(),
            task_cx: TaskContex::default(),
            trap_cx: PhysAddr::default(),
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
