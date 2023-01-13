use alloc::sync::Arc;

use crate::{
    task::{__switch, task::TaskContex},
    tools::cell::{STCell, STRefCell},
};

use super::{manager::TaskManager, task::TaskControlBlock};

pub struct Processor {
    hartid: usize,
    current: STCell<Option<Arc<TaskControlBlock>>>,
    task_manager: Arc<TaskManager>,
}

impl Processor {
    pub fn new(hartid: usize, task_maneger: Arc<TaskManager>) -> Self {
        Self {
            hartid,
            current: STCell::new(None),
            task_manager: task_maneger,
        }
    }

    #[inline]
    pub fn set_current(&self, task: Arc<TaskControlBlock>) {
        task.run(self.hartid);
        self.current.set(Some(task));
    }

}

pub trait Hart {
    fn current_task(&self) -> Arc<TaskControlBlock>;
    fn entrap_task(&self) -> !;
    fn schedule(&self);
}

impl Hart for Processor {

    #[inline]
    fn current_task(&self) -> Arc<TaskControlBlock> {
        unsafe { (*self.current.as_ptr()).clone().unwrap() }
    }

    fn entrap_task(&self) -> ! {
        let next_task = self.task_manager.pop_spin();
        let next = next_task.task_context();
        self.set_current(next_task);
        static mut HOLE: TaskContex = TaskContex::default();
        unsafe {
            __switch(&mut HOLE as *mut TaskContex, next);
        }
        unreachable!()
    }

    #[inline]
    fn schedule(&self) {
        let current_task = self.current.take().unwrap();
        let current = current_task.task_context();

        self.task_manager.push(current_task);
        let next_task = self.task_manager.pop_spin();

        let next = next_task.task_context();
        self.set_current(next_task);
        unsafe {
            __switch(current, next);
        }
    }
}