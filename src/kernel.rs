use crate::{
    _start,
    config::HART_NUMBER,
    mem::{
        address::{VirtAddr, VirtPageNum},
        memory_set::{MapArea, MapPerm, MapType, MemorySet},
    },
    println,
    task::{
        app_info::get_app_data,
        manager::TaskManager,
        processor::{Hart, Processor},
        task::TaskControlBlock,
        tigger::Future,
    },
};
use alloc::{sync::Arc, vec, vec::Vec};
use lazy_static::lazy_static;
use log::info;
use spin::RwLock;

lazy_static! {
    pub static ref KERNEL: Kernel = Kernel::new(2);
}

pub struct Kernel {
    task_manager: Arc<TaskManager>,
    pub processors: Vec<Processor>,
    pub kernel_space: RwLock<MemorySet>,
}

pub fn add_initproc() {
    // 添加初始程序
    let initproc_elf = get_app_data("initproc").unwrap();
    KERNEL.push_task(TaskControlBlock::from_elf(initproc_elf));
}

pub fn init_kernel_space() {
    KERNEL.kernel_space.read().activate()
}

impl Kernel {
    pub fn new(hart_num: usize) -> Self {
        let task_manager = Arc::new(TaskManager::new());
        let mut processors = Vec::with_capacity(hart_num);
        for hartid in 0..hart_num {
            processors.push(Processor::new(hartid, task_manager.clone()));
        }
        let kernel_space = RwLock::new(MemorySet::build_kernel_space());
        Self {
            task_manager,
            processors,
            kernel_space,
        }
    }

    pub fn push_stack(&self, va_start: VirtAddr, va_end: VirtAddr) {
        self.kernel_space.write().push(
            MapArea::new(va_start, va_end, MapPerm::RW, MapType::Framed),
            None,
        );
    }

    pub fn push_task(&self, task: Arc<TaskControlBlock>) {
        self.task_manager.push(task);
    }

    pub fn remove_stack(&self, end_vpn: VirtPageNum) {
        self.kernel_space.write().remove_area_with_end_vpn(end_vpn);
    }

    pub fn token(&self) -> usize {
        self.kernel_space.read().token()
    }

    pub fn get_processor(&self, hartid: usize) -> &Processor {
        &self.processors[hartid]
    }
}

pub fn entrap_task(hartid: usize) -> ! {
    if hartid != 0 {
        info!("hart[{}] spin", hartid);
        loop {}
    }
    KERNEL.processors[hartid].entrap_task()
}

pub fn hart_start() {
    for id in 0..HART_NUMBER {
        sbi_rt::hart_start(id, _start as usize, 0);
    }
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
