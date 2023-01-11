use alloc::vec;
use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};
use core::fmt::Debug;
use log::info;
use riscv::register::sstatus::SPP;
use xmas_elf::ElfFile;

use crate::trap::init_app_trap_return;
use crate::{
    config::{kernel_stack_position, TRAP_CONTEXT},
    fs::{
        stdio::{Stdin, Stdout},
        FileBox,
    },
    kernel::KERNEL,
    mem::{
        address::{PhysAddr, VirtAddr},
        memory_set::MemorySet,
    },
    tools::cell::STRefCell,
    trap::{context::TrapContext, user_trap_return},
};

use super::{
    pid::{pid_alloc, PidHandle},
    tigger::FutureBox,
};

const S_REG_NUMS: usize = 12;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TaskContex {
    pub ra: usize,              // 0
    pub ksp: usize,             // 1
    pub s: [usize; S_REG_NUMS], // 2
}

impl const Default for TaskContex {
    fn default() -> Self {
        Self {
            ra: 0,
            ksp: 0,
            s: [0; S_REG_NUMS],
        }
    }
}

impl TaskContex {
    pub fn goto_trap_return(ksp: usize, satp: usize) -> Self {
        let mut result = Self {
            ra: init_app_trap_return as usize,
            ksp,
            s: [0; S_REG_NUMS],
        };
        result.s[0] = satp;
        result
    }
}

pub struct TaskControlBlock {
    pid: PidHandle,
    pub inner: STRefCell<TaskControlBlockInner>,
}

pub struct TaskControlBlockInner {
    state: TaskStatus,
    memory_set: MemorySet,
    task_cx: TaskContex,
    trap_cx: PhysAddr,
    pub fd_table: Vec<Option<FileBox>>,
    pub parent: Option<Weak<TaskControlBlock>>,
    pub children: Vec<Arc<TaskControlBlock>>,
}

impl Drop for TaskControlBlock {
    fn drop(&mut self) {
        let ksp = self.trap_context().ksp;
        KERNEL.remove_stack(VirtAddr::from(ksp).into());
        info!("Drop app {}", self.get_pid());
    }
}

impl TaskControlBlock {
    pub fn new(memory_set: MemorySet, entry: usize, usp: usize) -> Arc<Self> {
        let pid = pid_alloc();
        // 添加内核栈
        let (ksp_top, ksp_bottom) = kernel_stack_position(pid.0);
        KERNEL.push_stack(ksp_top.into(), ksp_bottom.into());
        let trap_cx = TrapContext::init(entry, usp, ksp_bottom, KERNEL.token());
        Arc::new(Self {
            pid,
            inner: STRefCell::new(TaskControlBlockInner::new(memory_set, trap_cx)),
        })
    }

    pub fn from_elf(elf: ElfFile) -> Arc<Self> {
        let (memory_set, entry, usp) = MemorySet::from_elf(&elf);
        Self::new(memory_set, entry, usp)
    }

    // pub fn fork(self: &Arc<Self>) -> Arc<Self> {
    //     let pid = pid_alloc();
    //     let (ksp_top, ksp_bottom) = kernel_stack_position(pid.0);
    //     push_kernel_stack(VirtAddr::from(ksp_top), VirtAddr::from(ksp_bottom));
    //     let memory_set = MemorySet::from_existed_user(user_space());
    //     // let trap_cx_pa = memory_set.va_translate(TRAP_CONTEXT).unwrap();
    //     let mut trap_cx = unsafe { *self.trap_context() };
    //     trap_cx.ksp = ksp_bottom;
    //     trap_cx.x[10] = 0;
    //     let result = Arc::new(Self {
    //         pid,
    //         inner: STCell::new(TaskControlBlockInner::new(memory_set, trap_cx)),
    //     })
    //     .with_parent(self);
    //     self.inner.borrow_mut().children.push(result.clone());
    //     result
    // }

    pub fn run(&self, hartid: usize) {
        self.trap_context().hartid = hartid;
        self.inner.borrow_mut().set_state(TaskStatus::Running);
    }

    pub fn ready(&self) {
        self.inner.borrow_mut().set_state(TaskStatus::Ready);
    }

    pub fn blocking(&self, tigger: FutureBox) {
        self.inner
            .borrow_mut()
            .set_state(TaskStatus::Blocked(tigger));
    }

    pub fn exit(&self, code: i32) {
        let mut inner = self.inner.borrow_mut();
        match inner.state {
            TaskStatus::Exited(_) => (),
            _ => {
                info!("App[{}] exit with code {}", self.get_pid(), code);
                let children = &mut inner.children;
                while let Some(task) = children.pop() {
                    task.exit(code);
                }
                inner.state = TaskStatus::Exited(code)
            }
        }
    }

    pub fn find_child(&self, pid: isize) -> Option<(usize, Arc<Self>)> {
        // self.clear_children();
        self.inner
            .borrow_mut()
            .children
            .iter()
            .enumerate()
            .find_map(|(idx, task)| {
                if task.get_pid() == pid {
                    Some((idx, task.clone()))
                } else {
                    None
                }
            })
    }

    pub fn memory_set(&self) -> *mut MemorySet {
        &self.inner.borrow().memory_set as *const _ as *mut MemorySet
    }
    pub fn task_context(&self) -> *mut TaskContex {
        &self.inner.borrow().task_cx as *const _ as *mut TaskContex
    }
    #[inline]
    pub fn trap_context(&self) -> &'static mut TrapContext {
        unsafe { self.inner.borrow().trap_cx.as_type() }
    }

    pub fn get_pid(&self) -> isize {
        self.pid.0
    }

    pub fn with_parent(self: Arc<Self>, parent: &Arc<Self>) -> Arc<Self> {
        parent.inner.borrow_mut().children.push(self.clone());
        self.inner.borrow_mut().parent = Some(Arc::downgrade(parent));
        self
    }

    pub fn exit_code(&self) -> Option<i32> {
        if let TaskStatus::Exited(code) = self.inner.borrow().state {
            Some(code)
        } else {
            None
        }
    }

    pub fn space(&self) -> &'static mut MemorySet {
        unsafe { &mut *self.memory_set() }
    }

    // pub fn set_tigger(&self, tigger: Tigger) {
    //     self.inner.borrow_mut().trigger = tigger;
    // }
}

impl TaskControlBlockInner {
    // pub fn new(memory_set: MemorySet, entry: usize, usp: usize, ksp: usize) -> Self {
    //     // 初始化Trap上下文
    //     let cx = TrapContext::init(entry, usp, ksp, kernel_token(), SPP::User);
    //     let cx_pa = memory_set.va_translate(TRAP_CONTEXT).unwrap();
    //     unsafe {
    //         (*(cx_pa.0 as *mut TrapContext)) = cx;
    //     }

    //     Self {
    //         state: TaskStatus::Ready(None),
    //         memory_set,
    //         task_cx: TaskContex::goto_trap_return(ksp),
    //         trap_cx: cx_pa,
    //         // trigger: Tigger::default(),
    //         parent: None,
    //         children: Vec::new(),
    //     }
    // }

    pub fn new(memory_set: MemorySet, trap_cx: TrapContext) -> Self {
        // memory_set.push(map_area, data)
        let cx_pa = memory_set.va_translate(TRAP_CONTEXT).unwrap();
        let satp = memory_set.token();
        unsafe {
            *cx_pa.as_type() = trap_cx;
        }
        Self {
            state: TaskStatus::Ready,
            memory_set,
            task_cx: TaskContex::goto_trap_return(trap_cx.ksp, satp),
            trap_cx: cx_pa,
            fd_table: vec![Some(Arc::new(Stdin)), Some(Arc::new(Stdout))],
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn get_state(&self) -> &TaskStatus {
        &self.state
    }

    #[inline]
    pub fn set_state(&mut self, state: TaskStatus) {
        if let TaskStatus::Exited(_) = state {
            panic!("Don't set state to exited directly!");
        }
        self.state = state
    }
}

// #[derive(Debug)]
pub enum TaskStatus {
    Exited(i32),
    Ready,
    Running,
    Blocked(FutureBox),
}

impl Debug for TaskStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Exited(arg0) => f.debug_tuple("Exited").field(arg0).finish(),
            Self::Ready => write!(f, "Ready"),
            Self::Running => write!(f, "Running"),
            Self::Blocked(_) => f.debug_tuple("Blocked").finish(),
        }
    }
}
