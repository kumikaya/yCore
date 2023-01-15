use crate::config::HART_NUMBER;

use super::{processor::Processor, task::Task};
use alloc::vec::Vec;
use lazy_static::lazy_static;


lazy_static! {
    pub static ref GLOBAL_SCHEDULER: Scheduler = Scheduler::new(HART_NUMBER);
}

pub struct Scheduler {
    group: Vec<Processor>,
}

impl Scheduler {
    pub fn new(hart_num: usize) -> Self {
        let mut group = Vec::with_capacity(hart_num);
        for hartid in 0..hart_num {
            group.push(Processor::new(hartid));
        }
        Self { group }
    }

    #[inline]
    pub fn get_processor(&self, hartid: usize) -> &Processor {
        &self.group[hartid]
    }

    pub fn add_task(&self, task: Task) {
        // self.group[0].add_task(task);
        let processor = self
            .group
            .iter()
            .min_by(|x, y| x.wakee_num().cmp(&y.wakee_num()))
            .unwrap();
        processor.add_task(task);
    }

    pub fn fetch_task(&self) -> Option<Task> {
        let iter = self.group.iter();
        if let Some(processor) = iter.max_by(|x, y| x.wakee_num().cmp(&y.wakee_num())) {
            processor.fetch_task()
        } else {
            None
        }
    }

    pub fn balance(&self) {
        todo!()
    }
}

pub fn get_processor(hartid: usize) -> &'static Processor {
    GLOBAL_SCHEDULER.get_processor(hartid)
}

pub fn add_task(task: Task) {
    GLOBAL_SCHEDULER.add_task(task);
}

