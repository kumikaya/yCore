use alloc::{sync::Arc, string::String};
use bitflags::bitflags;

use crate::{
    fs::inode::open_app,
    mm::{
        address::VirtAddr,
        page_table::{translated_refmut, translated_string},
    },
    syscall_unwarp,
    task::{
        processor::Schedule,
        scheduler::add_task,
        task::{TaskControlBlock, TaskStatus},
        tigger::{ChildrenWaiter, TaskWaiter},
    },
    timer,
};

use super::{EXEC_FAIL, EXEC_SUCCEE};

pub(super) trait SysProcess {
    fn sys_exit(&self, code: i32) -> !;
    fn sys_yield(&self) -> isize;
    fn sys_exec(&self, ptr: VirtAddr, len: usize, flags: u32) -> isize;
    fn sys_fork(&self) -> isize;
    fn sys_get_pid(&self) -> isize;
    fn sys_waitpid(&self, pid: isize, exit_code_ptr: *mut i32) -> isize;
}

bitflags! {
    struct ExecFlags: u32 {
        const EMPTY      = 0;
        /// 继承管道
        const INHERIT    = 1 << 0;
        /// 子进程优先执行
        const ORDER_ASC  = 1 << 1;
        /// 父进程优先执行
        const ORDER_DESC = 1 << 2;
    }
}

impl<T: Schedule> SysProcess for T {
    fn sys_exit(&self, code: i32) -> ! {
        self.exit_current(code)
    }

    fn sys_yield(&self) -> isize {
        self.yield_();
        EXEC_SUCCEE
    }

    fn sys_exec(&self, ptr: VirtAddr, len: usize, flags: u32) -> isize {
        let flags = ExecFlags::from_bits_truncate(flags);
        let current_task = self.current_task();
        let mut args = unsafe { syscall_unwarp!(translated_string(current_task.space(), ptr, len)) };
        let path: String = args.drain(..args.find('\0').unwrap_or(args.len())).collect();
        if let Some(task) = open_app(&path, "hello world") {
            unsafe { task.set_parent(&current_task) };
            let pid = task.get_pid();
            if flags.contains(ExecFlags::INHERIT) {
                *task.fd_table.borrow_mut() = current_task.fd_table.borrow().clone();
            }
            add_task(task);
            pid as isize
        } else {
            EXEC_FAIL
        }
    }

    fn sys_waitpid(&self, pid: isize, exit_code_ptr: *mut i32) -> isize {
        let current_task = self.current_task();
        let idx: usize;
        let waitee_task: Arc<TaskControlBlock>;
        if pid == -1 {
            self.blocking_current(ChildrenWaiter::new(current_task.clone()));
            let children = &current_task.tree.borrow().children;
            if children.is_empty() {
                return EXEC_FAIL;
            }
            (idx, waitee_task) = children
                .iter()
                .enumerate()
                .find(|(_, child)| child.exit_code().is_some())
                .map(|(idx, task)| (idx, task.clone()))
                .unwrap();
        } else if let Some(val) = unsafe { current_task.find_child(pid) } {
            (idx, waitee_task) = val;
            self.blocking_current(TaskWaiter::new(
                waitee_task.shared_state.clone(),
                TaskStatus::Exited,
            ));
        } else {
            return EXEC_FAIL;
        }
        let code = waitee_task.exit_code().unwrap();
        current_task.tree.borrow_mut().children.remove(idx);
        unsafe {
            *translated_refmut(current_task.space(), exit_code_ptr) = code;
        }
        waitee_task.get_pid()
    }

    fn sys_get_pid(&self) -> isize {
        self.current_task().get_pid()
    }

    fn sys_fork(&self) -> isize {
        let child = self.current_task().fork();
        let pid = child.get_pid();
        add_task(child);
        pid
    }
}

pub fn sys_get_time() -> isize {
    timer::get_time_ms() as isize
}
