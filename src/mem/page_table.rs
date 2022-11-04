use crate::{config::PAGE_SIZE, println};
use alloc::{collections::BTreeMap, vec::Vec};
use bitflags::bitflags;
use riscv::register::satp;
use anyhow::Result;
use super::{
    address::{PhysAddr, PhysPageNum, VPNRange, VirtAddr, VirtPageNum},
    frame_allocator::{frame_alloc, FrameTracker},
    memory_set::{MapArea, MapType, MemorySet},
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
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }

    pub fn flags(self) -> PTEFlags {
        PTEFlags::from_bits_truncate(self.bits as u8)
    }

    pub fn is_valid(self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
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
#[derive(Debug)]
pub struct PageTable {
    pub root_ppn: PhysPageNum,
    pub frames: Vec<FrameTracker>,
    pub leafs: BTreeMap<VirtPageNum, FrameTracker>,
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
            leafs: BTreeMap::new(),
        }
    }

    pub fn token(&self) -> usize {
        let mode = (satp::Mode::Sv39 as usize) << 60;
        mode | self.root_ppn.0
    }

    pub fn from_token(satp: usize) -> Self {
        Self {
            root_ppn: PhysPageNum::from(satp & ((1usize << 44) - 1)),
            frames: Vec::new(),
            leafs: BTreeMap::new(),
        }
    }

    pub fn find_pte_entry(&mut self, vpn: VirtPageNum) -> &mut PageTableEntry {
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

    pub fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
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

    pub fn va_translate(&self, va: VirtAddr) -> Option<PhysAddr> {
        let vpn: VirtPageNum = va.floor();
        let ppn = self.translate(vpn);
        if let Some(ppn) = ppn {
            Some(PhysAddr::from(ppn.ppn()) + PhysAddr::from(va.page_offset()))
        } else {
            None
        }
    }

    pub fn copy_data(&self, range: VPNRange, data: &[u8]) {
        let mut vpn_iter = range.into_iter();
        let mut start = 0;
        while start < data.len() {
            let pte = self
                .translate(vpn_iter.next().unwrap())
                .expect("Unassigned frame for the virtual address.");
            let end = (start + PAGE_SIZE).min(data.len());
            let src = &data[start..end];
            unsafe {
                pte.ppn().as_bytes()[..src.len()].copy_from_slice(src);
            }
            start = end;
        }
    }

    pub fn map_area(&mut self, map_area: &MapArea) {
        for vpn in map_area.range {
            let ppn = match map_area.map_type {
                MapType::Identical => PhysPageNum::from(vpn.0),
                MapType::Framed => {
                    let frame = frame_alloc().unwrap();
                    let ppn = frame.ppn;
                    self.leafs.insert(vpn, frame);
                    ppn
                }
            };
            self.map(vpn, ppn, PTEFlags::from_bits_truncate(map_area.perm.bits()));
        }
    }

    pub fn malloc(&mut self, vpn: VirtPageNum) -> Result<()> {
        if vpn.0 == 0 || self.leafs.contains_key(&vpn) {
            return Err(anyhow::Error::msg("vpn malloc error!"));
        }
        let frame = frame_alloc().unwrap();
        let ppn = frame.ppn;
        self.leafs.insert(vpn, frame);
        self.map(vpn, ppn, PTEFlags::R | PTEFlags::W);
        Ok(())
    }

    pub fn free(&mut self, vpn: VirtPageNum) {
        self.unmap(vpn)
    } 

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte_entry = self.find_pte_entry(vpn);
        assert!(
            !pte_entry.is_valid(),
            "vpn {:?} is mapped before mapping",
            vpn
        );
        *pte_entry = PageTableEntry::new(ppn, flags | PTEFlags::V)
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
        *pte = PageTableEntry::empty();
        self.leafs.remove(&vpn);
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| *pte)
    }
}

pub fn translated_byte_buffer(space: &MemorySet, ptr: VirtAddr, len: usize) -> Vec<*const [u8]> {
    let mut start = ptr;
    let end = ptr.offset(len as isize);
    let mut result = Vec::new();
    while start < end {
        let vpn = start.floor();
        let ppn = space.translate(vpn).unwrap().ppn();
        let end_va = VirtAddr::from(vpn.offset(1)).min(end);
        let part =
            unsafe { &ppn.as_bytes()[start.page_offset()..end_va.page_offset()] as *const [u8] };
        result.push(part);
        start = end_va;
    }
    result
}
