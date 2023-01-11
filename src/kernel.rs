use alloc::sync::Arc;
use lazy_static::lazy_static;

use crate::{
    mem::{
        address::{VirtAddr, VirtPageNum},
        memory_set::{MapArea, MapPerm, MapType, MemorySet},
    },
    task::{
        app_info::get_app_data,
        manager::TaskManager,
        processor::{Processor, Hart},
        task::TaskControlBlock,
        tigger::Future,
    },
    tools::cell::STRefCell,
};

lazy_static! {
    pub static ref KERNEL: Kernel = Kernel::new();
}

pub struct Kernel {
    pub task_manager: Arc<STRefCell<TaskManager>>,
    pub processor: Processor,
    pub kernel_space: STRefCell<MemorySet>,
}

pub fn init() {
    KERNEL.init();
}

impl Kernel {
    pub fn new() -> Self {
        let task_manager = Arc::new(STRefCell::new(TaskManager::new()));
        let processor = Processor::new(0, task_manager.clone());
        let kernel_space = STRefCell::new(MemorySet::build_kernel_space());
        Self {
            task_manager,
            processor,
            kernel_space,
        }
    }

    pub fn init(&self) {
        self.kernel_space.borrow_mut().activate();
        // 执行第一个程序
        let initproc_elf = get_app_data("initproc").unwrap();
        self.push_task(TaskControlBlock::from_elf(initproc_elf));
    }

    pub fn push_stack(&self, va_start: VirtAddr, va_end: VirtAddr) {
        self.kernel_space.borrow_mut().push(
            MapArea::new(va_start, va_end, MapPerm::RW, MapType::Framed),
            None,
        );
    }

    pub fn push_task(&self, task: Arc<TaskControlBlock>) {
        self.task_manager.borrow_mut().push(task);
    }

    pub fn remove_stack(&self, end_vpn: VirtPageNum) {
        self.kernel_space
            .borrow_mut()
            .remove_area_with_end_vpn(end_vpn);
    }

    pub fn token(&self) -> usize {
        self.kernel_space.borrow().token()
    }
}

pub fn run_first_app() -> ! {
    KERNEL.processor.entrap_task()
}

pub trait Schedule {
    fn exit_current(&self, code: i32) -> !;
    fn blocking_current<T>(&self, tigger: T)
    where
        T: Future<Output = ()> + Sync + Send + 'static;
    fn _yield(&self);
}

impl<T: Hart> Schedule for T {
    fn exit_current(&self, code: i32) -> ! {
        self.current_task().exit(code);
        self.entrap_task()
    }

    fn blocking_current<F>(&self, tigger: F)
    where
        F: Future<Output = ()> + Sync + Send + 'static,
    {
        self.current_task().blocking(box tigger);
        self.schedule();
    }

    fn _yield(&self) {
        self.current_task().ready();
        self.schedule();
    }
}
