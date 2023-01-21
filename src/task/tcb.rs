use core::{
    cell::RefCell,
    fmt::Debug,
    mem::{align_of, size_of},
    ops::Range,
    sync::atomic::AtomicU32,
};

use alloc::{boxed::Box, sync::Arc};

use os_tools::OsStr;
use spin::Mutex;

use crate::{
    config::{GUARD_PAGE_SIZE, TRAP_CONTEXT, USER_STACK_SIZE},
    mm::{
        address::VirtAddr,
        memory_set::{kernel_token, MapArea, MapPerm, MapType, MemorySet},
        page_table::{translated_byte_buffer, BufferHandle},
    },
    tools::align_ceil,
    trap::context::TrapContext,
};

use super::{
    context::{Context, TaskContext},
    process::{Process, ProcessControlBlock},
    signal::SignalFlags,
    uid::{kstack_alloc, KernelStack},
};

pub type Task = Arc<TaskControlBlock>;

pub const TASK_SEND_UNLOCK: u32 = 0;
pub const TASK_SEND_LOCK: u32 = 1;

pub struct TaskControlBlock {
    pub tid: usize,
    /// 线程共享状态，允许多线程访问
    pub shared: Arc<SharedStatus>,
    /// 进程的线程间共享数据
    pub process: Arc<ProcessControlBlock>,
    /// 非共享状态，不允许多线程访问
    pub local: RefCell<ThreadLocal>,
    /// 线程间发送任务的锁，如果为0则可以在线程间发送任务。
    pub send_lock: AtomicU32,
}

impl Drop for TaskControlBlock {
    fn drop(&mut self) {
        ustack_dealloc(
            &mut self.process.inner.write().memory_set,
            self.local.borrow().ustack,
        );
        // 主线程退出则进程退出
        if self.tid == 0 {
            self.process.exit(self.shared.exit_code.lock().unwrap());
        }
    }
}

pub struct ThreadLocal {
    _ksp: KernelStack,
    token: usize,
    ustack: VirtAddr,
    context: Context,
    pub trap_cx_backup: Option<Box<TrapContext>>,
}

fn trap_context_addr(tid: usize) -> usize {
    let align = align_of::<TrapContext>().max(8);
    assert!(TRAP_CONTEXT % align == 0);
    TRAP_CONTEXT + tid * align_ceil(size_of::<TrapContext>(), align)
}

fn user_stack_addr(tid: usize, ustack_base: usize) -> Range<VirtAddr> {
    let bottom = ustack_base + (tid + 1) * (USER_STACK_SIZE + GUARD_PAGE_SIZE);
    let top = bottom - USER_STACK_SIZE;
    top.into()..bottom.into()
}

fn ustack_alloc(memory_set: &mut MemorySet, stack: Range<VirtAddr>) {
    memory_set.push(
        MapArea::from_range(stack, MapPerm::RWU, MapType::Framed),
        None,
    );
}

fn ustack_dealloc(memory_set: &mut MemorySet, bottom: VirtAddr) {
    memory_set.remove_area_with_end_vpn(bottom.into());
}

impl ThreadLocal {
    pub fn new(context: Context, ksp: KernelStack, ustack: VirtAddr, token: usize) -> Self {
        Self {
            _ksp: ksp,
            ustack,
            context,
            token,
            trap_cx_backup: None,
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
    // Running,
    Wait,
}

#[derive(Default)]
pub struct SharedStatus {
    pub signals: Mutex<SignalFlags>,
    pub state: Mutex<TaskStatus>,
    pub exit_code: Mutex<Option<i32>>,
}

// 向用户栈压入参数，返回新的用户栈地址
fn push_args(memory_set: &mut MemorySet, mut usp: VirtAddr, args: &str) -> VirtAddr {
    let args_len = args.len();
    // 8字节对齐
    // let all_len = align_ceil::<u64>(args_len + size_of::<usize>());
    let all_len = align_ceil(args_len, 8) as isize;
    usp = usp.offset(-all_len);
    unsafe {
        // 向栈写入参数长度
        // *translated_refmut(memory_set, usp as *const usize as *mut usize) = args_len;
        // let args_addr = usp + size_of::<usize>();
        let buffer = BufferHandle::new(translated_byte_buffer(memory_set, usp, args_len).unwrap());
        // 写入参数
        for (dst, src) in buffer.into_iter().zip(args.as_bytes().iter()) {
            *dst = *src;
        }
    }
    usp
}

impl TaskControlBlock {
    pub fn new(
        process: &Process,
        tid: usize,
        entry: usize,
        ustack_base: usize,
        args: &str,
    ) -> Task {
        // let pid = pid_alloc();
        // // 添加内核栈
        // let (ksp_top, ksp_bottom) = kernel_stack_position(pid.id);
        let kstack = kstack_alloc();
        let ustack = user_stack_addr(tid, ustack_base);
        let memory_set = &mut process.inner.write().memory_set;
        ustack_alloc(memory_set, ustack.clone());

        let usp = push_args(memory_set, ustack.end, args);
        // push_kernel_stack(ksp_top.into(), ksp_bottom.into());
        let mut trap_cx = TrapContext::new(entry, usp.into(), kstack.bottom(), kernel_token());
        // let trap_cx = unsafe { task.trap_context() };

        unsafe {
            trap_cx.set_args(OsStr::from_raw_parts_unchecked(
                usize::from(usp) as *const u8,
                args.len(),
            ));
        }

        // let (trap_cx_pa, task_cx) = build_user_context(&memory_set, trap_cx);
        let context = Context::build(memory_set, trap_cx, trap_context_addr(tid).into());
        // drop(memory_set);
        Arc::new(Self {
            tid,
            shared: Default::default(),
            send_lock: AtomicU32::new(TASK_SEND_UNLOCK),
            process: process.clone(),
            local: RefCell::new(ThreadLocal::new(
                context,
                kstack,
                ustack.end,
                memory_set.token(),
            )),
        })
    }

    pub fn trap_context_va(&self) -> usize {
        trap_context_addr(self.tid)
    }

    // pub fn from_elf(elf: ElfFile, args: &str) -> Task {
    //     let (memory_set, entry, mut usp) = MemorySet::from_elf(&elf);
    //     usp = push_args(&memory_set, usp, args);
    //     let result = Self::new(memory_set, entry, usp);
    //     let trap_cx = unsafe { result.trap_context() };
    //     unsafe {
    //         trap_cx.set_args(OsStr::from_raw_parts_unchecked(
    //             usp as *const u8,
    //             args.len(),
    //         ));
    //     }
    //     result
    // }

    // pub fn exec(&self, elf: ElfFile, _args: Vec<String>) {
    //     let (memory_set, entry, usp) = MemorySet::from_elf(&elf);
    //     let trap_cx = TrapContext::new(
    //         entry,
    //         usp,
    //         unsafe { self.trap_context().ksp },
    //         kernel_token(),
    //     );
    //     // self.process_local.borrow_mut().context = Context::new(memory_set, trap_cx);
    //     todo!()
    // }

    pub fn fork(self: &Task, process: &Process) -> Task {
        // let pid = pid_alloc();
        // let (ksp_top, ksp_bottom) = kernel_stack_position(pid.id);
        // add_kernel_stack(ksp_top.into(), ksp_bottom.into());
        let ksp = kstack_alloc();
        // let memory_set = MemorySet::from_existed(&self.process.inner.read().memory_set);
        let mut trap_cx = unsafe { *self.trap_context() };
        // 设置初始内核栈，所以是安全的
        trap_cx.ksp = ksp.bottom();
        // trap_cx.set_return(0);
        let memory_set = &process.inner.read().memory_set;
        let context = Context::build(memory_set, trap_cx, TRAP_CONTEXT.into());
        let result = Arc::new(Self {
            tid: self.tid,
            shared: Default::default(),
            send_lock: AtomicU32::new(TASK_SEND_UNLOCK),
            process: process.clone(),
            local: RefCell::new(ThreadLocal::new(
                context,
                ksp,
                self.local.borrow().ustack,
                memory_set.token(),
            )),
        });
        // result.process.inner.write().fd_table = self.process.inner.read().fd_table.clone();
        // 初始化，安全
        // unsafe { result.process.set_parent(&self.process) };
        result
    }

    pub fn is_ready(&self) -> bool {
        matches!(*self.shared.state.lock(), TaskStatus::Ready)
    }

    pub fn exit(&self, code: i32) {
        // let mut process_inner = self.process.inner.write();
        // 提前释放部分非共享数据
        // process_inner.fd_table.clear();
        // process_inner.tree.children.clear();
        self.process.remove_task(self.tid);
        // info!("App {} exit with code {code}", self.get_pid());
        *self.shared.exit_code.lock() = Some(code);
        *self.shared.state.lock() = TaskStatus::Exited;
        // *self.state.lock() = TaskStatus::Exited;
    }

    pub fn set_state(&self, state: TaskStatus) {
        assert_ne!(state, TaskStatus::Exited);
        *self.shared.state.lock() = state;
    }

    pub fn task_context(&self) -> *mut TaskContext {
        unsafe { &mut (*self.local.as_ptr()).context.task_cx as *mut TaskContext }
    }
    #[inline]
    pub unsafe fn trap_context(&self) -> &mut TrapContext {
        (*self.local.as_ptr()).context.trap_cx.as_type()
    }

    pub fn exit_code(&self) -> Option<i32> {
        *self.shared.exit_code.lock()
    }
    #[inline]
    /// 不是线程安全的
    pub unsafe fn space(&self) -> &mut MemorySet {
        unsafe { &mut (*self.process.inner.as_mut_ptr()).memory_set }
    }
    pub fn token(&self) -> usize {
        unsafe { (*self.local.as_ptr()).token }
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
