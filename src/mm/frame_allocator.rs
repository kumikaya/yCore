use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{mm::address::PhysAddr, config::MEMORY_END};

use super::address::PhysPageNum;


lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<StackFrameAllocator> = {
        extern "C" {
            fn ekernel();
        }
        Mutex::new(
            StackFrameAllocator::new(PhysAddr::from(ekernel as usize).ceil(), PhysAddr::from(MEMORY_END).floor())
        )
    };
}


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
    /// Create frame allocator for `PhysPageNum` in [start, end)
    pub fn new(start: PhysPageNum, end: PhysPageNum) -> Self {
        Self {
            current: start.into(),
            end: end.into(),
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
            Some(ppn.into())
        } else if self.current < self.end {
            self.current += 1;
            Some((self.current - 1).into())
        } else {
            None
        }
        
    }

    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn: usize = ppn.into();
        if ppn >= self.current || self.recycled.iter().any(|&v| v == ppn) {
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
            for x in ppn.as_bytes().iter() {
                if *x != 0 {
                    panic!();
                }
            }
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
    FRAME_ALLOCATOR.lock().alloc().map(|frame| {
        FrameTracker::new(frame)
    })
}

pub fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.lock().dealloc(ppn);
}

fn free_frame_num() -> usize {
    FRAME_ALLOCATOR.lock().free_frame_num()
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