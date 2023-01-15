use core::{fmt::Debug, arch::asm};

use alloc::{
    string::String,
    sync::{Arc, Weak},
    vec,
    vec::Vec,
};
use log::info;
use spin::Mutex;
use xmas_elf::ElfFile;

use crate::{
    config::{kernel_stack_position, TRAP_CONTEXT},
    fs::{
        stdio::{Stdin, Stdout},
        FileArc,
    },
    mm::{
        address::{PhysAddr, VirtAddr},
        memory_set::{kernel_token, push_kernel_stack, remove_kernel_stack, MemorySet},
    },
    tools::cell::STRefCell,
    trap::{context::TrapContext, init_app_trap_return},
    KERNEL_STACK, STACK_SIZE, task::{scheduler::{GLOBAL_SCHEDULER, get_processor}, entrap_task, processor::Hart},
};

use super::{
    pid::{pid_alloc, PidHandle},
    tigger::FutureBox,
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TaskContext {
    pub ra: usize,      // 0
    pub ksp: usize,     // 1
    pub s: [usize; 12], // 2
}

impl const Default for TaskContext {
    fn default() -> Self {
        Self {
            ra: 0,
            ksp: 0,
            s: [0; 12],
        }
    }
}

impl TaskContext {
    pub fn goto_trap_return(ksp: usize, satp: usize) -> Self {
        let mut result = Self {
            ra: init_app_trap_return as usize,
            ksp,
            s: [0; 12],
        };
        result.s[0] = satp;
        result
    }

    pub fn goto_init(hartid: usize) -> Self {
        let ksp = unsafe { &KERNEL_STACK as *const u8 as usize + hartid * STACK_SIZE };
        let mut result = Self {
            ra: processor_init as usize,
            ksp,
            s: [0; 12],
        };
        result.s[0] = hartid;
        result
    }
}

pub unsafe fn processor_init() {
    let hartid: usize;
    asm! {r"
        mv {hartid}, s0
        ",
        hartid = out(reg) hartid
    }
    let processor = get_processor(hartid);
    processor.set_current(None);
    processor.entrap_task()
}

pub type Task = Arc<TaskControlBlock>;

pub struct TaskControlBlock {
    pid: PidHandle,
    // ksp:
    pub state: Mutex<TaskStatus>,
    pub tree: Mutex<ProcessTree>,
    pub context: STRefCell<Context>,
    pub fd_table: STRefCell<FdTable>,
}

#[derive(Default)]
pub struct ProcessTree {
    pub parent: Option<Weak<TaskControlBlock>>,
    pub children: Vec<Arc<TaskControlBlock>>,
}

pub struct Context {
    memory_set: MemorySet,
    task_cx: TaskContext,
    trap_cx: PhysAddr,
}

#[derive(Clone)]
pub struct FdTable {
    ports: Vec<Option<FileArc>>,
}

impl FdTable {
    pub fn new() -> Self {
        let mut ports: Vec<Option<FileArc>> = Vec::with_capacity(3);
        ports.push(Some(Arc::new(Stdin)));
        ports.push(Some(Arc::new(Stdout)));
        ports.push(Some(Arc::new(Stdout)));
        // Self {
        //     ports: vec![
        //         Some(Arc::new(Stdin)),  // 0 stdin
        //         Some(Arc::new(Stdout)), // 1 stdout
        //         Some(Arc::new(Stdout)), // 2 stderr
        //     ],
        // }
        Self { ports }
    }

    pub fn push_fd(&mut self, file: FileArc) -> usize {
        let fd = self
            .ports
            .iter()
            .enumerate()
            .find(|(idx, file)| *idx >= 2 && file.is_none())
            .map(|(idx, _)| idx);
        if let Some(idx) = fd {
            self.ports[idx] = Some(file);
            idx
        } else {
            self.ports.push(Some(file));
            self.ports.len() - 1
        }
    }
    pub fn close(&mut self, fd: usize) -> Option<FileArc> {
        if let Some(file) = self.ports.get_mut(fd) {
            file.take()
        } else {
            None
        }
    }
    pub fn get(&self, fd: usize) -> Option<&FileArc> {
        if let Some(file) = self.ports.get(fd) {
            file.as_ref()
        } else {
            None
        }
    }
}

// 子进程必须由父进程退出，如果子进程退出自己将发生异常
impl Drop for TaskControlBlock {
    fn drop(&mut self) {
        let ksp = self.trap_context().ksp;
        remove_kernel_stack(VirtAddr::from(ksp).into());
        // info!("Drop app {}", self.get_pid());
    }
}

impl TaskControlBlock {
    pub fn new(memory_set: MemorySet, entry: usize, usp: usize) -> Arc<Self> {
        let pid = pid_alloc();
        // 添加内核栈
        let (ksp_top, ksp_bottom) = kernel_stack_position(pid.id);
        push_kernel_stack(ksp_top.into(), ksp_bottom.into());
        let trap_cx = TrapContext::init(entry, usp, ksp_bottom, kernel_token());
        Arc::new(Self {
            pid,
            state: Mutex::new(TaskStatus::Ready),
            tree: Mutex::new(ProcessTree::default()),
            fd_table: STRefCell::new(FdTable::new()),
            context: STRefCell::new(Context::new(memory_set, trap_cx)),
        })
    }

    pub fn from_elf(elf: ElfFile) -> Arc<Self> {
        let (memory_set, entry, usp) = MemorySet::from_elf(&elf);
        Self::new(memory_set, entry, usp)
    }

    pub fn exec(&self, elf: ElfFile, _args: Vec<String>) {
        let (memory_set, entry, usp) = MemorySet::from_elf(&elf);
        let trap_cx = TrapContext::init(entry, usp, self.trap_context().ksp, kernel_token());
        *self.context.borrow_mut() = Context::new(memory_set, trap_cx);
    }

    pub fn fork(self: &Arc<Self>) -> Arc<Self> {
        let pid = pid_alloc();
        let (ksp_top, ksp_bottom) = kernel_stack_position(pid.id);
        push_kernel_stack(ksp_top.into(), ksp_bottom.into());
        let memory_set = MemorySet::from_existed(&self.context.borrow().memory_set);
        let mut trap_cx = self.trap_context().clone();
        trap_cx.ksp = ksp_bottom;
        trap_cx.reg_file.a[0] = 0;
        let result = Arc::new(Self {
            pid,
            state: Mutex::new(TaskStatus::Ready),
            tree: Mutex::new(ProcessTree::default()),
            fd_table: STRefCell::new(self.fd_table.borrow().clone()),
            context: STRefCell::new(Context::new(memory_set, trap_cx)),
        });
        result.set_parent(self);
        result
    }

    pub fn run(&self, hartid: usize) {
        self.trap_context().hartid = hartid;
        *self.state.lock() = TaskStatus::Running;
        // processor.set_current(self)
    }

    pub fn ready(&self) {
        *self.state.lock() = TaskStatus::Ready;
        // TASK_MANAGER.push(self);
        // self.inner.borrow_mut().set_state(TaskStatus::Ready);
    }

    pub fn is_ready(&self) -> bool {
        matches!(*self.state.lock(), TaskStatus::Ready)
    }

    pub fn block(self: &Arc<Self>, tigger: FutureBox) {
        *self.state.lock() = TaskStatus::Blocked(tigger);
    }

    pub fn exit(&self, code: i32) {
        let mut state = self.state.lock();
        match *state {
            TaskStatus::Exited(_) => (),
            _ => {
                // info!("App[{}] exit with code {}", self.get_pid(), code);
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
    pub fn task_context(&self) -> *mut TaskContext {
        &self.context.borrow().task_cx as *const _ as *mut TaskContext
    }
    #[inline]
    pub fn trap_context(&self) -> &'static mut TrapContext {
        unsafe { self.context.borrow().trap_cx.as_type() }
    }
    #[inline]
    pub fn get_pid(&self) -> isize {
        self.pid.id
    }

    pub fn set_parent(self: &Arc<Self>, parent: &Arc<Self>) {
        parent.tree.lock().children.push(self.clone());
        // parent.inner.borrow_mut().children.push(self.clone());
        self.tree.lock().parent = Some(Arc::downgrade(parent));
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

impl Context {
    pub fn new(memory_set: MemorySet, trap_cx: TrapContext) -> Self {
        let cx_pa = memory_set.va_translate((TRAP_CONTEXT).into()).unwrap();
        let satp = memory_set.token();
        unsafe {
            *cx_pa.as_type() = trap_cx;
        }
        Self {
            memory_set,
            task_cx: TaskContext::goto_trap_return(trap_cx.ksp, satp),
            trap_cx: cx_pa,
        }
    }
}

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
            Self::Blocked(_) => write!(f, "Blocked"),
        }
    }
}

// impl TaskStatus {
//     pub fn replace(&mut self, new: TaskStatus) -> TaskStatus {
//         todo!()
//     }
//     pub fn take_tigger(&mut self) -> Option<FutureBox> {
//         match mem::replace(self, Self::Sleep) {
//             TaskStatus::Blocked(tigger) => Some(tigger),
//             _ => None,
//         }
//     }
// }

// pub enum TaskStatusDisc {
//     Exited,
//     Ready,
//     Running,
//     Blocked,
// }

// impl From<&TaskStatus> for TaskStatusDisc {
//     fn from(value: &TaskStatus) -> Self {
//         match value {
//             TaskStatus::Exited(_) => TaskStatusDisc::Exited,
//             TaskStatus::Ready => TaskStatusDisc::Ready,
//             TaskStatus::Running => TaskStatusDisc::Running,
//             TaskStatus::Blocked(_) => TaskStatusDisc::Blocked,
//         }
//     }
// }
