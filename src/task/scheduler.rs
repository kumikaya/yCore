use core::arch::asm;

use crate::config::HART_NUMBER;
use super::{processor::Processor, task_block::Task};
use alloc::vec::Vec;
use spin::Lazy;


pub static GLOBAL_SCHEDULER: Lazy<Scheduler> = Lazy::new(|| Scheduler::new(HART_NUMBER));

pub struct Scheduler {
    group: Vec<Processor>,
}

unsafe impl Sync for Scheduler {}
unsafe impl Send for Scheduler {}

impl Scheduler {
    pub fn new(hart_num: usize) -> Self {
        let mut group = Vec::with_capacity(hart_num);
        for hartid in 0..hart_num {
            group.push(Processor::new(hartid));
        }
        Self { group }
    }

    /// 获取其它线程的 `Processor` 是不安全的
    #[inline]
    pub unsafe fn get_processor(&self, hartid: usize) -> &Processor {
        &self.group[hartid]
    }

    pub fn add_task(&self, task: Task) {
        // unsafe { task.trap_context().set_hartid(0) };
        // self.group[0].add_task(task);
        let (hartid, processor) = self
            .group
            .iter()
            .enumerate()
            .min_by(|(_, x), (_, y)| x.ready_task_num().cmp(&y.ready_task_num()))
            .unwrap();
        unsafe { task.trap_context().hartid = hartid };
        processor.add_task(task);
    }

    pub fn fetch_task(&self) -> Option<Task> {
        let iter = self.group.iter();
        if let Some(processor) = iter.max_by(|x, y| x.ready_task_num().cmp(&y.ready_task_num())) {
            processor.fetch_task()
        } else {
            None
        }
    }

    pub fn balance(&self) {
        todo!()
    }
}

// #[inline]
// pub fn set_hartid(hartid: usize) {
//     unsafe {
//         asm! {r"
//             mv tp, {x}",
//             x = in(reg) hartid
//         }
//     };
// }

#[inline]
pub fn get_hartid() -> usize {
    let hartid: usize;
    unsafe {
        asm! {r"
            mv {x}, tp",
            x = out(reg) hartid
        }
    };
    hartid
}

#[inline]
pub fn get_processor() -> &'static Processor {
    unsafe {
        GLOBAL_SCHEDULER.get_processor(get_hartid())
    }
}

pub fn add_task(task: Task) {
    GLOBAL_SCHEDULER.add_task(task);
}
