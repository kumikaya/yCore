use alloc::{
    sync::{Arc, Weak},
    vec,
    vec::Vec,
};
use core::fmt::Debug;
use log::info;
use spin::Mutex;
use xmas_elf::ElfFile;

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
    trap::{context::TrapContext, init_app_trap_return},
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
    pub state: Mutex<TaskStatus>,
    pub tree: Mutex<ProcessTree>,
    /// 任务上下文只被单线程访问
    pub context: STRefCell<TaskControlBlockInner>,
}

#[derive(Default)]
pub struct ProcessTree {
    pub parent: Option<Weak<TaskControlBlock>>,
    pub children: Vec<Arc<TaskControlBlock>>,
}

pub struct TaskControlBlockInner {
    memory_set: MemorySet,
    task_cx: TaskContex,
    trap_cx: PhysAddr,
    pub fd_table: Vec<Option<FileBox>>,
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
            state: Mutex::new(TaskStatus::Ready),
            tree: Mutex::new(ProcessTree::default()),
            context: STRefCell::new(TaskControlBlockInner::new(memory_set, trap_cx)),
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
        *self.state.lock() = TaskStatus::Running;
    }

    pub fn ready(&self) {
        *self.state.lock() = TaskStatus::Ready
        // self.inner.borrow_mut().set_state(TaskStatus::Ready);
    }

    pub fn blocking(&self, tigger: FutureBox) {
        *self.state.lock() = TaskStatus::Blocked(tigger);
        // self.inner
        //     .borrow_mut()
        //     .set_state(TaskStatus::Blocked(tigger));
    }

    pub fn exit(&self, code: i32) {
        let mut state = self.state.lock();
        match *state {
            TaskStatus::Exited(_) => (),
            _ => {
                info!("App[{}] exit with code {}", self.get_pid(), code);
                let children = &mut self.tree.lock().children;
                while let Some(task) = children.pop() {
                    task.exit(code);
                }
                *state = TaskStatus::Exited(code)
            }
        }
    }

    pub fn find_child(&self, pid: isize) -> Option<(usize, Arc<Self>)> {
        self.tree
            .lock()
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
    #[inline]
    fn memory_set(&self) -> *mut MemorySet {
        &self.context.borrow().memory_set as *const _ as *mut MemorySet
    }
    pub fn task_context(&self) -> *mut TaskContex {
        &self.context.borrow().task_cx as *const _ as *mut TaskContex
    }
    #[inline]
    pub fn trap_context(&self) -> &'static mut TrapContext {
        unsafe { self.context.borrow().trap_cx.as_type() }
    }
    #[inline]
    pub fn get_pid(&self) -> isize {
        self.pid.0
    }

    pub fn with_parent(self: Arc<Self>, parent: &Arc<Self>) -> Arc<Self> {
        parent.tree.lock().children.push(self.clone());
        // parent.inner.borrow_mut().children.push(self.clone());
        self.tree.lock().parent = Some(Arc::downgrade(parent));
        self
    }

    pub fn exit_code(&self) -> Option<i32> {
        if let TaskStatus::Exited(code) = *self.state.lock() {
            Some(code)
        } else {
            None
        }
    }
    #[inline]
    pub fn space(&self) -> &'static mut MemorySet {
        unsafe { &mut *self.memory_set() }
    }
}

impl TaskControlBlockInner {
    pub fn new(memory_set: MemorySet, trap_cx: TrapContext) -> Self {
        // memory_set.push(map_area, data)
        let cx_pa = memory_set.va_translate(TRAP_CONTEXT).unwrap();
        let satp = memory_set.token();
        unsafe {
            *cx_pa.as_type() = trap_cx;
        }
        Self {
            memory_set,
            task_cx: TaskContex::goto_trap_return(trap_cx.ksp, satp),
            trap_cx: cx_pa,
            fd_table: vec![Some(Arc::new(Stdin)), Some(Arc::new(Stdout))],
        }
    }

    // pub fn get_state(&self) -> &TaskStatus {
    //     &self.state
    // }

    // #[inline]
    // pub fn set_state(&mut self, state: TaskStatus) {
    //     if let TaskStatus::Exited(_) = state {
    //         panic!("Don't set state to exited directly!");
    //     }
    //     self.state = state
    // }
}

// #[derive(Debug)]
pub enum TaskStatus {
    Exited(i32),
    Ready,
    Running,
    Blocked(FutureBox),
}

// pub enum TaskStatusDisc {
//     Exited,
//     Ready,
//     Running,
//     Blocked,
// }

// impl From<TaskStatus> for TaskStatusDisc {
//     fn from(value: TaskStatus) -> Self {
//         match value {
//             TaskStatus::Exited(_) => TaskStatusDisc::Exited,
//             TaskStatus::Ready => TaskStatusDisc::Ready,
//             TaskStatus::Running => TaskStatusDisc::Running,
//             TaskStatus::Blocked(_) => TaskStatusDisc::Blocked,
//         }
//     }
// }

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
