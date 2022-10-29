use crate::{config::PAGE_SIZE, println};
use alloc::vec::Vec;
use bitflags::bitflags;

use super::{
    address::{PhysAddr, PhysPageNum, VirtPageNum},
    frame_allocator::{frame_alloc, FrameTracker},
};

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        Self {
            bits: ppn.0 << 10 | flags.bits as usize,
        }
    }

    pub fn ppn(self) -> PhysPageNum {
        PhysPageNum(self.bits >> 10 & ((1usize << 44) - 1))
    }

    pub fn flags(self) -> PTEFlags {
        PTEFlags::from_bits_truncate(self.bits as u8)
    }

    pub fn is_valid(self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }

    pub fn is_leaf(self) -> bool {
        self.flags() & (PTEFlags::R | PTEFlags::W | PTEFlags::X) != PTEFlags::empty()
    }

    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }
    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }
    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }

    pub fn empty() -> Self {
        Self::default()
    }
}

impl Default for PageTableEntry {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
}

impl PageTable {
    pub fn size(&self) -> usize {
        self.frames.len() * PAGE_SIZE
    }
    pub fn new() -> Self {
        let root_frame = frame_alloc().unwrap();
        Self {
            root_ppn: root_frame.ppn,
            frames: alloc::vec![root_frame],
        }
    }

    pub fn from_token(satp: usize) -> Self {
        Self {
            root_ppn: PhysPageNum::from(satp & ((1usize << 44) - 1)),
            frames: Vec::new(),
        }
    }

    fn get_pte_entry(&mut self, vpn: VirtPageNum) -> &mut PageTableEntry {
        let indexs = vpn.indexs();
        let mut ppn = self.root_ppn;
        for (count, &idx) in indexs.iter().enumerate() {
            let pte = unsafe { &mut ppn.as_pte_array()[idx] };
            if count == indexs.len() - 1 {
                return pte;
            }
            if !pte.is_valid() {
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        unreachable!()
    }

    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let indexs = vpn.indexs();
        let mut ppn = self.root_ppn;
        for (count, &idx) in indexs.iter().enumerate() {
            let pte = unsafe { &mut ppn.as_pte_array()[idx] };
            if count == indexs.len() - 1 {
                return Some(pte);
            }
            if !pte.is_valid() {
                return None;
            }
            ppn = pte.ppn();
        }
        unreachable!()
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte_entry = self.get_pte_entry(vpn);
        assert!(
            !pte_entry.is_valid(),
            "vpn {:?} is mapped before mapping",
            vpn
        );
        *pte_entry = PageTableEntry::new(ppn, flags)
    }
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
        *pte = PageTableEntry::empty();
    }
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| *pte)
    }
}
