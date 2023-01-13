use alloc::sync::Arc;

use crate::{
    kernel::{Schedule, KERNEL},
    mem::{
        address::VirtAddr,
        page_table::{translated_refmut, translated_string},
    },
    task::{
        app_info::get_app_data,
        processor::Hart,
        task::TaskControlBlock,
        tigger::{ChildrenWaiter, TaskWaiter},
    },
    timer,
};

pub(super) trait Process {
    fn sys_exit(&self, code: i32) -> !;
    fn sys_yield(&self) -> isize;
    fn sys_exec(&self, ptr: VirtAddr, len: usize) -> isize;
    fn sys_get_pid(&self) -> isize;
    fn sys_waitpid(&self, pid: isize, exit_code_ptr: *mut i32) -> isize;
}

impl<T: Hart> Process for T {
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
        // println!("{}", path);
        if let Some(elf) = get_app_data(path.as_str()) {
            let task = TaskControlBlock::from_elf(elf).with_parent(&current_task);
            let pid = task.get_pid();
            KERNEL.push_task(task);
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
                .find(|(_, task)| task.exit_code().is_some())
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
        unsafe {
            *translated_refmut(current_task.space(), exit_code_ptr) = code;
        }
        waitee_task.get_pid()
    }

    fn sys_get_pid(&self) -> isize {
        self.current_task().get_pid()
    }
}

pub fn sys_get_time() -> isize {
    timer::get_time_ms() as isize
}
