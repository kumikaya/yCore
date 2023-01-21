use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};

use spin::{Mutex, RwLock};
use xmas_elf::ElfFile;

use crate::{
    fs::{
        stdio::{Stdin, Stdout},
        FileBox,
    },
    mm::memory_set::MemorySet,
    tools::Table,
};

use super::{
    signal::{Signal, SignalFlags},
    tcb::{Task, TaskControlBlock},
    uid::{pid_alloc, Pid},
};

pub type Process = Arc<ProcessControlBlock>;

pub struct ProcessControlBlock {
    pid: Pid,
    ustack_base: usize,
    pub shared: Arc<ProcessSharedStatus>,
    pub inner: RwLock<ProcessControlBlockInner>,
}

pub struct ProcessControlBlockInner {
    pub tree: ProcessTree,
    pub fd_table: Table<FileBox>,
    pub signal: Signal,
    pub memory_set: MemorySet,
    pub tasks: Table<Task>,
}

#[derive(Default)]
pub struct ProcessSharedStatus {
    pub signals: Mutex<SignalFlags>,
    pub state: Mutex<ProcessStatus>,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ProcessStatus {
    #[default]
    Running,
    Exit(i32),
}

#[derive(Default)]
pub struct ProcessTree {
    pub parent: Option<Weak<ProcessControlBlock>>,
    pub children: Vec<Process>,
}

impl ProcessControlBlockInner {
    pub fn new(memory_set: MemorySet) -> Self {
        Self {
            tree: ProcessTree::default(),
            memory_set,
            fd_table: Table::<FileBox>::new()
                .with(Arc::new(Stdin))
                .with(Arc::new(Stdout))
                .with(Arc::new(Stdout)),
            signal: Default::default(),
            tasks: Table::new(),
        }
    }
}

impl ProcessControlBlock {
    pub fn new(memory_set: MemorySet, ustack_base: usize) -> Arc<Self> {
        Arc::new(Self {
            pid: pid_alloc(),
            ustack_base,
            shared: Default::default(),
            inner: RwLock::new(ProcessControlBlockInner::new(memory_set)),
        })
    }

    pub fn add_task(self: &Process, entry: usize, args: &str) -> Task {
        let tid = self.inner.write().tasks.alloc_id();
        let task = TaskControlBlock::new(self, tid, entry, self.ustack_base, args);
        *self.inner.write().tasks.get_entry(tid) = Some(task.clone());
        task
    }

    pub fn from_elf(elf: ElfFile, args: &str) -> (Arc<Self>, Task) {
        let (memory_set, entry, ustack_base) = MemorySet::from_elf(&elf);
        // let usp = push_args(&memory_set, ustack_base, args);
        let result = Self::new(memory_set, ustack_base);
        let task = result.add_task(entry, args);
        (result, task)
    }

    /// 注意：当前实现在多线程下是不正确的，会出现不可预知的问题
    pub unsafe fn fork(self: &Process) -> Arc<Self> {
        let memory_set = MemorySet::from_existed(&self.inner.read().memory_set);
        let new_process = Self::new(memory_set, self.ustack_base);
        let fd_table = &self.inner.read().fd_table.clone();
        new_process.inner.write().fd_table = fd_table.clone();
        let tasks: Table<Task> = self
            .inner
            .read()
            .tasks
            .iter()
            .map(|task| task.as_ref().map(|x| x.fork(&new_process)))
            .collect();
        new_process.set_parent(self);
        new_process.inner.write().tasks = tasks;
        new_process
    }

    #[inline]
    pub fn get_pid(&self) -> isize {
        self.pid.id
    }

    /// 获取其它线程的任务是不安全的，因为 `TaskControlBlock` 不是线程安全的
    pub unsafe fn find_child(&self, pid: isize) -> Option<(usize, Process)> {
        self.inner
            .read()
            .tree
            .children
            .iter()
            .enumerate()
            .find_map(|(idx, process)| {
                if process.get_pid() == pid {
                    Some((idx, process.clone()))
                } else {
                    None
                }
            })
    }
    /// 父子任务必须是同一个线程的
    pub unsafe fn set_parent(self: &Process, parent: &Process) {
        parent.inner.write().tree.children.push(self.clone());
        self.inner.write().tree.parent = Some(Arc::downgrade(parent));
    }
    pub fn exit_code(&self) -> Option<i32> {
        if let ProcessStatus::Exit(code) = *self.shared.state.lock() {
            Some(code)
        } else {
            None
        }
    }
    pub fn exit(&self, code: i32) {
        self.clear_res();
        *self.shared.state.lock() = ProcessStatus::Exit(code);
    }

    pub fn get_task(&self, tid: usize) -> Option<Task> {
        self.inner.read().tasks.get(tid).cloned()
    }

    pub fn remove_task(&self, tid: usize) {
        self.inner.write().tasks.remove(tid);
    }

    pub fn clear_res(&self) {
        let mut inner = self.inner.write();
        inner.fd_table.clear();
        inner.tree.children.clear();
        inner.tasks.clear();
    }

    // pub fn alloc_tid(&self) -> (usize, &mut Option<Task>) {
    //     self.inner.write().tasks.alloc_id()
    // }

    // pub fn add_task(&self, task: &Task) {
    //     self.inner.write().tasks[task.tid] = Arc::downgrade(task);
    // }

    // #[inline]
    // pub fn add_task(&self, mut task: TaskControlBlock) -> Task {
    //     let mut inner = self.inner.write();
    //     let idx = inner
    //         .tasks
    //         .iter()
    //         .enumerate()
    //         .find(|(_, task)| task.upgrade().is_none())
    //         .map(|(idx, _)| idx);
    //     if let Some(idx) = idx {
    //         task.tid = idx;
    //         let task = Arc::new(task);
    //         inner.tasks[idx] = Arc::downgrade(&task);
    //         task
    //     } else {
    //         let idx = inner.tasks.len();
    //         task.tid = idx;
    //         let task = Arc::new(task);
    //         inner.tasks.push(Arc::downgrade(&task));
    //         task
    //     }
    // }
}
