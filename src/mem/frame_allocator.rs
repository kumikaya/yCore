use alloc::vec::Vec;
use lazy_static::lazy_static;
use log::info;

use crate::{tools::cell::STRefCell, mem::address::PhysAddr, config::MEMORY_END, println};

use super::address::PhysPageNum;


lazy_static! {
    pub static ref FRAME_ALLOCATOR: STRefCell<StackFrameAllocator> = {
        extern "C" {
            fn ekernel();
        }
        STRefCell::new(
            StackFrameAllocator::new(PhysAddr(ekernel as usize).ceil(), PhysAddr(MEMORY_END).floor())
        )
    };
}

// pub fn init_frame_allocator() {
//     FRAME_ALLOCATOR.borrow();
// }

trait FrameAllocator {
    // fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl StackFrameAllocator {
    /// Create frame allocator for [PhysPageNum] in [start, end)
    pub fn new(start: PhysPageNum, end: PhysPageNum) -> Self {
        Self {
            current: start.0,
            end: end.0,
            recycled: Vec::new(),
        }
    }

    pub fn free_frame_num(&self) -> usize {
        self.end - self.current + self.recycled.len()
    }
}

impl FrameAllocator for StackFrameAllocator {
    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(PhysPageNum(ppn))
        } else {
            if self.current < self.end {
                self.current += 1;
                Some(PhysPageNum(self.current - 1))
            } else {
                None
            }
        }
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        if ppn >= self.current || self.recycled.iter().find(|&v| *v == ppn).is_some() {
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }
        self.recycled.push(ppn);
    }
}

#[derive(Debug)]
pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    fn new(ppn: PhysPageNum) -> Self {
        unsafe {
            ppn.as_bytes().fill(0);   
        }
        Self { ppn }
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn)
    }
}

pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR.borrow_mut().alloc().map(|frame| {
        FrameTracker::new(frame)
    })
}

fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.borrow_mut().dealloc(ppn);
}

fn free_frame_num() -> usize {
    FRAME_ALLOCATOR.borrow().free_frame_num()
}

#[cfg(feature = "debug_test")]
pub fn frame_allocator_test() {
    use crate::tools::ansi::{Colour, Color};

    let mut v: Vec<FrameTracker> = Vec::new();
    let frame_num = free_frame_num();
    const ALLOC_NUM: usize = 1024;
    for i in 0..ALLOC_NUM {
        let frame = frame_alloc().unwrap();
        v.push(frame);
    }
    assert_eq!(frame_num - ALLOC_NUM, free_frame_num());
    v.clear();
    assert_eq!(frame_num, free_frame_num());
    println!("[{}] frame_allocator_test", "passed".dye(Color::GreenB));
}