use core::{
    arch::asm,
    cell::RefCell,
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering}, hint, ops::Deref,
};

use alloc::{
    string::String,
    sync::{Arc, Weak},
    vec,
    vec::Vec, boxed::Box,
};
use anyhow::Result;

use os_tools::OsStr;
use spin::Mutex;
use xmas_elf::ElfFile;

use crate::{
    config::{kernel_stack_position, KERNEL_STACK_SIZE},
    fs::{
        stdio::{Stdin, Stdout},
        FileBox,
    },
    mm::{
        address::VirtAddr,
        memory_set::{kernel_token, push_kernel_stack, remove_kernel_stack, MemorySet},
        page_table::{translated_byte_buffer, BufferHandle},
    },
    tools::align_ceil,
    trap::context::TrapContext,
};

use super::{
    context::{Context, TaskContext},
    pid::{pid_alloc, PidHandle},
    signal::{SignalFlags, Signal}, scheduler::get_hartid,
};

pub type Task = Arc<TaskControlBlock>;

struct TaskWarp {
    inner: Arc<TaskControlBlock>,
}

impl Deref for TaskWarp {
    type Target = Arc<TaskControlBlock>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for TaskWarp {
    fn clone(&self) -> Self {
        if get_hartid() != unsafe { self.inner.trap_context().hartid } {
            while self.inner.send_lock.load(Ordering::Relaxed) == TASK_SEND_LOCK {
                hint::spin_loop();
            }
        }
        Self { inner: self.inner.clone() }
    }
}

pub const TASK_SEND_UNLOCK: usize = 0;
pub const TASK_SEND_LOCK: usize = 1;

pub struct TaskControlBlock {
    pid: PidHandle,
    /// 共享状态，允许多线程访问
    pub shared: Arc<SharedStatus>,
    /// 非共享状态，不允许多线程访问
    pub local: RefCell<LoaclStatus>,
    /// 线程间发送任务的锁，如果为0则可以在线程间发送任务。
    ///
    /// 持有任务的线程只会写入该变量，其它线程只会读取该变量
    pub send_lock: AtomicUsize,
}

pub struct LoaclStatus {
    pub tree: ProcessTree,
    pub fd_table: FdTable,
    pub signal: Signal,
    pub trap_cx_backup: Option<Box<TrapContext>>,
    context: Context,
}

impl LoaclStatus {
    pub fn new(context: Context) -> Self {
        Self {
            tree: ProcessTree::default(),
            context,
            fd_table: FdTable::new(),
            signal: Default::default(),
            // signal_mask: Default::default(),
            // signal_actions: Default::default(),
            trap_cx_backup: Default::default(),
        }
    }
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
    pub signals: Mutex<SignalFlags>,
    pub state: Mutex<TaskStatus>,
    pub exit_code: Mutex<Option<i32>>,
}

#[derive(Default)]
pub struct ProcessTree {
    pub parent: Option<Weak<TaskControlBlock>>,
    pub children: Vec<Task>,
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
            .find(|(_, file)| file.is_none())
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
    pub fn swap(&mut self, fd0: usize, fd1: usize) -> Result<()> {
        if self.get(fd0).is_some() && self.get(fd1).is_some() {
            self.table.swap(fd0, fd1);
            Ok(())
        } else {
            Err(anyhow!("exchange failed, some fd did not exist"))
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
        let ksp = unsafe { self.trap_context().ksp };
        assert!(!(ksp..(ksp + KERNEL_STACK_SIZE)).contains(&get_sp()));
        remove_kernel_stack(VirtAddr::from(ksp).into());
        // info!("Drop app {}", self.get_pid());
    }
}

// 向用户栈压入参数，返回新的用户栈地址
fn push_args(memory_set: &MemorySet, mut usp: usize, args: &str) -> usize {
    let args_len = args.len();
    // 8字节对齐
    // let all_len = align_ceil::<u64>(args_len + size_of::<usize>());
    let all_len = align_ceil::<u64>(args_len);
    usp -= all_len;
    unsafe {
        // 向栈写入参数长度
        // *translated_refmut(memory_set, usp as *const usize as *mut usize) = args_len;
        // let args_addr = usp + size_of::<usize>();
        let buffer =
            BufferHandle::new(translated_byte_buffer(memory_set, usp.into(), args_len).unwrap());
        // 写入参数
        for (dst, src) in buffer.into_iter().zip(args.as_bytes().iter()) {
            *dst = *src;
        }
    }
    usp
}

impl TaskControlBlock {
    pub fn new(memory_set: MemorySet, entry: usize, usp: usize) -> Task {
        let pid = pid_alloc();
        // 添加内核栈
        let (ksp_top, ksp_bottom) = kernel_stack_position(pid.id);
        push_kernel_stack(ksp_top.into(), ksp_bottom.into());
        let trap_cx = TrapContext::init(entry, usp, ksp_bottom, kernel_token());
        Arc::new(Self {
            pid,
            shared: Default::default(),
            send_lock: AtomicUsize::new(TASK_SEND_UNLOCK),
            local: RefCell::new(LoaclStatus::new(Context::new(memory_set, trap_cx))),
        })
    }

    pub fn from_elf(elf: ElfFile, args: &str) -> Task {
        let (memory_set, entry, mut usp) = MemorySet::from_elf(&elf);
        usp = push_args(&memory_set, usp, args);
        let result = Self::new(memory_set, entry, usp);
        let trap_cx = unsafe { result.trap_context() };
        unsafe {
            trap_cx.set_args(OsStr::from_raw_parts_unchecked(
                usp as *const u8,
                args.len(),
            ));
        }
        result
    }

    pub fn exec(&self, elf: ElfFile, _args: Vec<String>) {
        let (memory_set, entry, usp) = MemorySet::from_elf(&elf);
        let trap_cx = TrapContext::init(
            entry,
            usp,
            unsafe { self.trap_context().ksp },
            kernel_token(),
        );
        self.local.borrow_mut().context = Context::new(memory_set, trap_cx);
        todo!()
    }

    pub fn fork(self: &Task) -> Task {
        let pid = pid_alloc();
        let (ksp_top, ksp_bottom) = kernel_stack_position(pid.id);
        push_kernel_stack(ksp_top.into(), ksp_bottom.into());
        let memory_set = MemorySet::from_existed(&self.local.borrow().context.memory_set);
        let mut trap_cx = unsafe { *self.trap_context() };
        // 设置初始内核栈，所以是安全的
        trap_cx.ksp = ksp_bottom;
        trap_cx.set_return(0);
        let result = Arc::new(Self {
            pid,
            shared: Default::default(),
            send_lock: AtomicUsize::new(TASK_SEND_UNLOCK),
            local: RefCell::new(LoaclStatus::new(Context::new(memory_set, trap_cx))),
        });
        result.local.borrow_mut().fd_table = self.local.borrow().fd_table.clone();
        // 初始化，安全
        unsafe { result.set_parent(self) };
        result
    }

    pub fn is_ready(&self) -> bool {
        matches!(*self.shared.state.lock(), TaskStatus::Ready)
    }

    pub fn exit(&self, code: i32) {
        let mut local = self.local.borrow_mut();
        // 提前释放部分非共享数据
        local.fd_table.table.clear();
        local.tree.children.clear();
        // info!("App {} exit with code {code}", self.get_pid());
        *self.shared.exit_code.lock() = Some(code);
        *self.shared.state.lock() = TaskStatus::Exited;
        // *self.state.lock() = TaskStatus::Exited;
    }

    pub fn set_state(&self, state: TaskStatus) {
        assert_ne!(state, TaskStatus::Exited);
        *self.shared.state.lock() = state;
    }

    /// 获取其它线程的任务是不安全的，因为 `TaskControlBlock` 不是线程安全的
    pub unsafe fn find_child(&self, pid: isize) -> Option<(usize, Task)> {
        self.local
            .borrow()
            .tree
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
        unsafe { &mut (*self.local.as_ptr()).context.task_cx as *mut TaskContext }
    }
    #[inline]
    pub unsafe fn trap_context(&self) -> &mut TrapContext {
        (*self.local.as_ptr()).context.trap_cx.as_type()
    }
    #[inline]
    pub fn get_pid(&self) -> isize {
        self.pid.id
    }

    /// 父子任务必须是同一个线程的
    pub unsafe fn set_parent(self: &Task, parent: &Task) {
        parent.local.borrow_mut().tree.children.push(self.clone());
        // parent.inner.borrow_mut().children.push(self.clone());
        self.local.borrow_mut().tree.parent = Some(Arc::downgrade(parent));
    }

    pub fn exit_code(&self) -> Option<i32> {
        *self.shared.exit_code.lock()
    }
    #[inline]
    pub fn space(&self) -> &mut MemorySet {
        unsafe { &mut (*self.local.as_ptr()).context.memory_set }
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
