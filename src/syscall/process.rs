use alloc::string::String;
use bitflags::bitflags;

use crate::{
    fs::inode::open_app,
    mm::{
        address::VirtAddr,
        page_table::{translated_refmut, translated_string},
    },
    syscall_unwarp,
    task::{
        process::Process,
        processor::Schedule,
        scheduler::add_task,
        signal::{is_handle_by_kernel, SignalFlags, MAX_SIG},
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
    fn sys_kill(&self, pid: usize) -> isize;
    fn sys_sigprocmask(&self, mask: u32) -> isize;
    fn sys_sigreturn(&self) -> isize;
    fn sys_sigaction(&self, signum: u32, action: *const usize, old_action: *mut usize) -> isize;
}

bitflags! {
    struct ExecFlags: u32 {
        const EMPTY      = 0;
        /// 继承管道
        const INHERIT    = 1 << 0;

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
        let mut args =
            unsafe { syscall_unwarp!(translated_string(current_task.space(), ptr, len)) };
        let path: String = args
            .drain(..args.find('\0').unwrap_or(args.len()))
            .collect();
        if let Some((child_process, child_task)) = open_app(&path, &args[1.min(args.len())..]) {
            unsafe { child_process.set_parent(&current_task.process) };
            let pid = child_process.get_pid();
            if flags.contains(ExecFlags::INHERIT) {
                child_process.inner.write().fd_table =
                    current_task.process.inner.read().fd_table.clone();
            }
            add_task(child_task);
            pid
        } else {
            EXEC_FAIL
        }
    }

    fn sys_waitpid(&self, pid: isize, exit_code_ptr: *mut i32) -> isize {
        let current_task = self.current_task();
        let current_process = &current_task.process;
        let idx: usize;
        let waitee_process: Process;
        if pid == -1 {
            self.blocking_current(ChildrenWaiter::new(current_process));
            let children = &current_process.inner.read().tree.children;
            if children.is_empty() {
                return EXEC_FAIL;
            }
            (idx, waitee_process) = children
                .iter()
                .enumerate()
                .find(|(_, child)| child.exit_code().is_some())
                .map(|(idx, task)| (idx, task.clone()))
                .unwrap();
        } else if let Some(val) = unsafe { current_process.find_child(pid) } {
            (idx, waitee_process) = val;
            // let shared_state = waitee_task.shared_state.clone();
            // let tigger = Tigger::new(move || {
            //     *(shared_state.state.lock()) == TaskStatus::Exited
            // });
            // self.blocking_current(tigger);
            self.blocking_current(TaskWaiter::new(&waitee_process));
        } else {
            return EXEC_FAIL;
        }
        let code = waitee_process.exit_code().unwrap();
        // info!("App {} wait app {} done!", current_task.get_pid(), waitee_task.get_pid());
        current_task.process.inner.write().tree.children.remove(idx);
        unsafe {
            let ptr = syscall_unwarp!(translated_refmut(current_task.space(), exit_code_ptr));
            *ptr = code;
        }
        waitee_process.get_pid()
    }

    fn sys_get_pid(&self) -> isize {
        self.current_task().process.get_pid()
    }

    fn sys_fork(&self) -> isize {
        let tid = self.current_task().tid;
        let current_process = &self.current_task().process;
        let new_process = unsafe { current_process.fork() };
        unsafe {
            new_process
                .get_task(tid)
                .unwrap()
                .trap_context()
                .set_return(0);
        }
        for task in new_process.inner.read().tasks.iter_elem() {
            add_task(task.clone());
        }
        new_process.get_pid()
    }

    fn sys_kill(&self, _pid: usize) -> isize {
        todo!()
    }

    fn sys_sigprocmask(&self, mask: u32) -> isize {
        let current_task = self.current_task();
        let mut local = current_task.process.inner.write();
        if let Some(mask) = SignalFlags::from_bits(mask) {
            local.signal.mask = mask;
            EXEC_SUCCEE
        } else {
            EXEC_FAIL
        }
    }

    fn sys_sigreturn(&self) -> isize {
        let current_task = self.current_task();
        let mut local = current_task.process.inner.write();
        // 允许接收信号
        local.signal.global_mask = true;
        let trap_cx = unsafe { current_task.trap_context() };
        *trap_cx = *current_task
            .local
            .borrow_mut()
            .trap_cx_backup
            .take()
            .unwrap();
        // 恢复之前的a0寄存器
        trap_cx.reg_file.a[0] as isize
    }

    fn sys_sigaction(&self, signum: u32, action: *const usize, old_action: *mut usize) -> isize {
        if signum > MAX_SIG as u32 + 1 {
            return EXEC_FAIL;
        }
        let current_task = self.current_task();
        let mut local = current_task.process.inner.write();
        if let Some(flag) = SignalFlags::from_bits(1 << signum) {
            if is_handle_by_kernel(flag) || action as usize == 0 {
                return EXEC_FAIL;
            }
            let act = &mut local.signal.actions[signum as usize];
            unsafe {
                let ptr = syscall_unwarp!(translated_refmut(current_task.space(), old_action));
                *ptr = *act;
            }
            *act = action as usize;
            EXEC_SUCCEE
        } else {
            EXEC_FAIL
        }
    }
}

pub fn sys_get_time() -> isize {
    timer::get_time_ms() as isize
}
