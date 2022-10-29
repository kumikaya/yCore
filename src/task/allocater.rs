use core::ops::Range;

use crate::{config::*, stdlib::cell::STCell, println};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum PageDsc {
    FREE,
    USED,
}

impl Default for PageDsc {
    fn default() -> Self {
        Self::FREE
    }
}

struct MemMap {
    map: Vec<PageDsc>,
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

use alloc::vec::Vec;
use lazy_static::*;
use log::info;

lazy_static! {
    static ref MEM_MAP: STCell<MemMap> = {
        let mut map = Vec::with_capacity(PAGE_NUMS);
        for _ in 0..PAGE_NUMS {
            map.push(PageDsc::FREE);
        }
        STCell::new(MemMap {
            map
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
            addr + PAGE_SIZE * (range.end - range.start)
        );
        mem_map.take(range);
    }
    free
}
