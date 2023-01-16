use core::{arch::asm, cell::RefCell, fmt::Debug};

use alloc::{
    string::String,
    sync::{Arc, Weak},
    vec,
    vec::Vec,
};
use spin::Mutex;
use xmas_elf::ElfFile;

use crate::{
    config::{kernel_stack_position, KERNEL_INIT_STACK_SIZE, KERNEL_STACK_SIZE, TRAP_CONTEXT},
    fs::{
        stdio::{Stdin, Stdout},
        FileBox,
    },
    mm::{
        address::{PhysAddr, VirtAddr},
        memory_set::{kernel_token, push_kernel_stack, remove_kernel_stack, MemorySet},
    },
    trap::{context::TrapContext, init_app_trap_return},
    KERNEL_STACK,
};

use super::{
    pid::{pid_alloc, PidHandle},
    processor::switch_trampoline,
    scheduler::get_hartid,
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TaskContext {
    ra: usize,      // 0
    ksp: usize,     // 1
    s: [usize; 12], // 2
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

    pub fn switch_trampoline() -> Self {
        let ksp = unsafe { &KERNEL_STACK as *const u8 as usize }
            + (get_hartid() + 1) * KERNEL_INIT_STACK_SIZE;
        Self {
            ra: switch_trampoline as usize,
            ksp,
            s: [0; 12],
        }
    }
}

pub type Task = Arc<TaskControlBlock>;

pub struct TaskControlBlock {
    pid: PidHandle,
    pub shared_state: Arc<SharedStatus>,
    pub tree: RefCell<ProcessTree>,
    context: RefCell<Context>,
    pub fd_table: RefCell<FdTable>,
}

// unsafe impl Sync for TaskControlBlock {}
// unsafe impl Send for TaskControlBlock {}


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Exited,
    #[default]
    Ready,
    Running,
    Wait,
}

#[derive(Default)]
pub struct SharedStatus {
    pub state: Mutex<TaskStatus>,
    pub exit_code: Mutex<Option<i32>>,
}

impl SharedStatus {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(TaskStatus::Ready),
            exit_code: Mutex::new(None),
        }
    }
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
    table: Vec<Option<FileBox>>,
}

impl FdTable {
    pub fn new() -> Self {
        Self {
            table: vec![
                Some(Arc::new(Stdin)),  // 0 stdin
                Some(Arc::new(Stdout)), // 1 stdout
                Some(Arc::new(Stdout)), // 2 stderr
            ],
        }
    }

    pub fn push_fd(&mut self, file: FileBox) -> usize {
        let fd = self
            .table
            .iter()
            .enumerate()
            .find(|(idx, file)| *idx >= 2 && file.is_none())
            .map(|(idx, _)| idx);
        if let Some(idx) = fd {
            self.table[idx] = Some(file);
            idx
        } else {
            self.table.push(Some(file));
            self.table.len() - 1
        }
    }
    pub fn close(&mut self, fd: usize) -> Option<FileBox> {
        if let Some(file) = self.table.get_mut(fd) {
            file.take()
        } else {
            None
        }
    }
    pub fn get(&self, fd: usize) -> Option<&FileBox> {
        if let Some(file) = self.table.get(fd) {
            file.as_ref()
        } else {
            None
        }
    }
}

#[inline(always)]
fn get_sp() -> usize {
    let ret: usize;
    unsafe {
        asm! {r"
            mv {ret}, sp
            ",
            ret = out(reg) ret
        }
    }
    ret
}

/// 退出任务时不能在退出任务的内核栈上
impl Drop for TaskControlBlock {
    fn drop(&mut self) {
        let ksp = self.trap_context().get_ksp_bottom();
        assert!(!(ksp..(ksp + KERNEL_STACK_SIZE)).contains(&get_sp()));
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
            shared_state: Arc::new(SharedStatus::new()),
            // state: Mutex::new(TaskStatus::Ready),
            tree: RefCell::new(ProcessTree::default()),
            fd_table: RefCell::new(FdTable::new()),
            context: RefCell::new(Context::new(memory_set, trap_cx)),
            // exit_code: Mutex::new(None),
        })
    }

    pub fn from_elf(elf: ElfFile) -> Arc<Self> {
        let (memory_set, entry, usp) = MemorySet::from_elf(&elf);
        Self::new(memory_set, entry, usp)
    }

    pub fn exec(&self, elf: ElfFile, _args: Vec<String>) {
        let (memory_set, entry, usp) = MemorySet::from_elf(&elf);
        let trap_cx = TrapContext::init(
            entry,
            usp,
            self.trap_context().get_ksp_bottom(),
            kernel_token(),
        );
        *self.context.borrow_mut() = Context::new(memory_set, trap_cx);
        todo!()
    }

    pub fn fork(self: &Arc<Self>) -> Arc<Self> {
        let pid = pid_alloc();
        let (ksp_top, ksp_bottom) = kernel_stack_position(pid.id);
        push_kernel_stack(ksp_top.into(), ksp_bottom.into());
        let memory_set = MemorySet::from_existed(&self.context.borrow().memory_set);
        let mut trap_cx = self.trap_context().clone();
        // 设置初始内核栈，所以是安全的
        unsafe { trap_cx.set_ksp_bottom(ksp_bottom) };
        trap_cx.set_return(0);
        let result = Arc::new(Self {
            pid,
            shared_state: Arc::new(SharedStatus::new()),
            // state: Mutex::new(TaskStatus::Ready),
            tree: RefCell::new(ProcessTree::default()),
            fd_table: RefCell::new(self.fd_table.borrow().clone()),
            context: RefCell::new(Context::new(memory_set, trap_cx)),
            // exit_code: Mutex::new(None),
        });
        // 初始化，安全
        unsafe { result.set_parent(self) };
        result
    }

    // pub fn run(&self, hartid: usize) {
    //     self.trap_context().hartid = hartid;
    //     *self.state.lock() = TaskStatus::Running;
    //     // processor.set_current(self)
    // }

    // pub fn ready(&self) {
    //     *self.state.lock() = TaskStatus::Ready;
    //     // TASK_MANAGER.push(self);
    //     // self.inner.borrow_mut().set_state(TaskStatus::Ready);
    // }

    pub fn is_ready(&self) -> bool {
        matches!(*self.shared_state.state.lock(), TaskStatus::Ready)
    }

    pub fn exit(&self, code: i32) {
        *self.shared_state.state.lock() = TaskStatus::Exited;
        // *self.state.lock() = TaskStatus::Exited;
        *self.shared_state.exit_code.lock() = Some(code);
    }

    pub fn set_state(&self, state: TaskStatus) {
        assert_ne!(state, TaskStatus::Exited);
        *self.shared_state.state.lock() = state;
    }

    // pub fn block(self: &Arc<Self>, tigger: FutureBox) {
    //     *self.state.lock() = TaskStatus::Wait(tigger);
    // }

    // pub fn exit(&self, code: i32) {
    //     *self.state.lock() = TaskStatus::Exited(code);
    // }

    /// 获取其它线程的任务是不安全的，因为 `TaskControlBlock` 不是线程安全的
    pub unsafe fn find_child(&self, pid: isize) -> Option<(usize, Arc<Self>)> {
        self.tree
            .borrow()
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

    pub fn task_context(&self) -> *mut TaskContext {
        &self.context.borrow().task_cx as *const _ as *mut TaskContext
    }
    #[inline]
    pub fn trap_context(&self) -> &mut TrapContext {
        unsafe { self.context.borrow().trap_cx.as_type() }
    }
    #[inline]
    pub fn get_pid(&self) -> isize {
        self.pid.id
    }

    /// 父子任务必须是同一个线程的
    pub unsafe fn set_parent(self: &Arc<Self>, parent: &Arc<Self>) {
        parent.tree.borrow_mut().children.push(self.clone());
        // parent.inner.borrow_mut().children.push(self.clone());
        self.tree.borrow_mut().parent = Some(Arc::downgrade(parent));
    }

    pub fn exit_code(&self) -> Option<i32> {
        *self.shared_state.exit_code.lock()
    }
    #[inline]
    pub fn space(&self) -> &mut MemorySet {
        unsafe { &mut (*self.context.as_ptr()).memory_set }
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
            task_cx: TaskContext::goto_trap_return(trap_cx.get_ksp_bottom(), satp),
            trap_cx: cx_pa,
        }
    }
}


// impl Debug for TaskStatus {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         match self {
//             Self::Exited(arg0) => f.debug_tuple("Exited").field(arg0).finish(),
//             Self::Ready => write!(f, "Ready"),
//             Self::Running => write!(f, "Running"),
//             Self::Wait => write!(f, "Blocked"),
//         }
//     }
// }

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

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
//             TaskStatus::Wait(_) => TaskStatusDisc::Blocked,
//         }
//     }
// }
