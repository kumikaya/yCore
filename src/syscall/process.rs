use core::hint::spin_loop;

use alloc::sync::Arc;

use crate::{
    fs::inode::open_app,
    mm::{
        address::VirtAddr,
        page_table::{translated_refmut, translated_string},
    },
    task::{
        processor::{Hart, Schedule},
        scheduler::add_task,
        task::TaskControlBlock,
        tigger::{ChildrenWaiter, TaskWaiter},
    },
    timer,
};

pub(super) trait SysProcess {
    fn sys_exit(&self, code: i32) -> !;
    fn sys_yield(&self) -> isize;
    fn sys_exec(&self, ptr: VirtAddr, len: usize) -> isize;
    fn sys_fork(&self) -> isize;
    fn sys_get_pid(&self) -> isize;
    fn sys_waitpid(&self, pid: isize, exit_code_ptr: *mut i32) -> isize;
}

impl<T: Hart> SysProcess for T {
    fn sys_exit(&self, code: i32) -> ! {
        self.current_task().exit(code);
        self.entrap_task()
    }

    fn sys_yield(&self) -> isize {
        self.current_task().ready();
        self.schedule();
        0
    }

    fn sys_exec(&self, ptr: VirtAddr, len: usize) -> isize {
        let current_task = self.current_task();
        let path = unsafe { translated_string(current_task.space(), ptr, len) };
        // println!("exec {}", path);
        if let Some(task) = open_app(&path, Some(&current_task)) {
            let pid = task.get_pid();
            add_task(task);
            pid as isize
        } else {
            -1
        }
    }

    fn sys_waitpid(&self, pid: isize, exit_code_ptr: *mut i32) -> isize {
        let current_task = self.current_task();
        let idx: usize;
        let waitee_task: Arc<TaskControlBlock>;
        if pid == -1 {
            self.blocking_current(ChildrenWaiter::new(current_task.clone()));
            let children = &current_task.tree.lock().children;
            if children.is_empty() {
                return -1;
            }
            (idx, waitee_task) = children
                .iter()
                .enumerate()
                .find(|(_, child)| child.exit_code().is_some())
                .map(|(idx, task)| (idx, task.clone()))
                .unwrap();
        } else if let Some(val) = current_task.find_child(pid) {
            (idx, waitee_task) = val;
            self.blocking_current(TaskWaiter::new(waitee_task.clone()));
        } else {
            return -1;
        }
        let code = waitee_task.exit_code().unwrap();
        current_task.tree.lock().children.remove(idx);
        while Arc::strong_count(&waitee_task) != 1 {
            self.yield_();
        }
        // assert_eq!(Arc::strong_count(&waitee_task), 1);
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
