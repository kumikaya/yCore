use core::ops::Range;

use crate::{config::*, stdlib::cell::STCell};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(i8)]
enum PageDsc {
    #[default]
    FREE,
    USED,
}

struct MemMap {
    map: [PageDsc; PAGE_NUMS],
}

impl MemMap {
    pub fn is_free(&self, range: Range<usize>) -> bool {
        self.map[range].iter().all(|&dsc| dsc == PageDsc::FREE)
    }
    pub fn take(&mut self, range: Range<usize>) {
        self.map[range].iter_mut().for_each(|dsc| {
            *dsc = PageDsc::USED;
        });
    }
    pub fn free(&mut self, range: Range<usize>) {
        self.map[range].iter_mut().for_each(|dsc| {
            *dsc = PageDsc::FREE;
        });
    }
    pub fn to_index(addr: usize) -> usize {
        (addr - MEMORY_START) / PAGE_SIZE
    }

    pub fn align(addr: usize) -> bool {
        (addr & (PAGE_SIZE - 1)) == 0
    }
}

const MEMORY_SIZE: usize = MEMORY_END - MEMORY_START;
const PAGE_NUMS: usize = MEMORY_SIZE / PAGE_SIZE;

use lazy_static::*;
use log::info;

lazy_static! {
    static ref MEM_MAP: STCell<MemMap> = {
        STCell::new(MemMap {
            map: [PageDsc::default(); PAGE_NUMS],
        })
    };
}

pub fn init() {
    let mut mem_map = MEM_MAP.borrow_mut();
    mem_map.take(0..(KERNEL_MEMORY_END - MEMORY_START) / PAGE_SIZE);
}

pub fn malloc_at(addr: usize, len: usize) -> bool {
    assert!(MemMap::align(addr));
    let start = MemMap::to_index(addr);
    let trail = if len % PAGE_SIZE == 0 { 0 } else { 1 };
    let range = start..(start + len / PAGE_SIZE + trail);
    let mem_map = &mut MEM_MAP.borrow_mut();
    let free = mem_map.is_free(range.clone());
    if free {
        info!(
            "malloc memory succeed at [{:x}-{:x}]",
            addr,
            addr + PAGE_SIZE * range.end
        );
        mem_map.take(range);
    }
    free
}
