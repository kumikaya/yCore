use super::{
    address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum},
    frame_allocator::{frame_alloc, FrameTracker},
    memory_set::MemorySet,
};
use crate::config::PAGE_SIZE;
use alloc::{collections::BTreeMap, string::String, vec, vec::Vec};
use anyhow::{anyhow, Result};
use bitflags::bitflags;
use riscv::register::satp;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        Self {
            bits: usize::from(ppn) << 10 | flags.bits() as usize,
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
            frames: vec![root_frame],
            leafs: BTreeMap::new(),
        }
    }

    #[inline]
    pub fn token(&self) -> usize {
        let mode = (satp::Mode::Sv39 as usize) << 60;
        mode | usize::from(self.root_ppn)
    }

    // pub fn from_token(satp: usize) -> Self {
    //     Self {
    //         root_ppn: PhysPageNum::from(satp & ((1usize << 44) - 1)),
    //         frames: Vec::new(),
    //         leafs: BTreeMap::new(),
    //     }
    // }

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

    pub fn va_translate(&self, va: VirtAddr) -> Result<PhysAddr> {
        let vpn: VirtPageNum = va.floor();
        let ppn = self.translate(vpn);
        if let Some(ppn) = ppn {
            Ok(PhysAddr::from(ppn.ppn()) + PhysAddr::from(va.page_offset()))
        } else {
            Err(anyhow!("virt addr {} is invalid", va))
        }
    }

    pub fn malloc(&mut self, vpn: VirtPageNum, flags: PTEFlags) -> Result<()> {
        if usize::from(vpn) == 0 || self.leafs.contains_key(&vpn) {
            return Err(anyhow!("vpn {} already malloc", vpn));
        }
        let frame = frame_alloc()?;
        let ppn = frame.ppn;
        self.leafs.insert(vpn, frame);
        self.map(vpn, ppn, flags)
    }

    pub fn free(&mut self, vpn: VirtPageNum) -> Result<()> {
        if self.leafs.remove(&vpn).is_some() {
            self.unmap_uncheck(vpn)
        } else {
            Err(anyhow!("vpn {} is not malloc", vpn))
        }
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) -> Result<()> {
        let pte_entry = self.find_pte_entry(vpn);
        if !pte_entry.is_valid() {
            *pte_entry = PageTableEntry::new(ppn, flags | PTEFlags::V);
            Ok(())
        } else {
            Err(anyhow!("vpn {} is mapped before mapping", vpn))
        }
        // assert!(
        //     !pte_entry.is_valid(),
        //     "vpn {:?} is mapped before mapping",
        //     vpn
        // );
    }

    pub fn unmap_uncheck(&mut self, vpn: VirtPageNum) -> Result<()> {
        let pte = self.find_pte(vpn).unwrap();
        // assert_ne!(pte.flags() & PTEFlags::U, PTEFlags::empty());
        if pte.is_valid() {
            *pte = PageTableEntry::empty();
            Ok(())
        } else {
            Err(anyhow!("vpn {} is not mapped", vpn))
        }
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| *pte)
    }
}

pub unsafe fn translated_byte_buffer(
    space: &MemorySet,
    ptr: VirtAddr,
    len: usize,
) -> Result<Vec<&mut [u8]>> {
    let mut start = ptr;
    let end = ptr.offset(len as isize);
    let mut result = Vec::new();
    while start < end {
        let vpn = start.floor();
        let ppn = {
            if let Some(entry) = space.translate(vpn) {
                entry.ppn()
            } else {
                return Err(anyhow!("illegal address {}", start));
            }
        };
        let end_va = VirtAddr::from(vpn.offset(1));
        let end_offset = if end_va < end {
            PAGE_SIZE
        } else {
            end.page_offset()
        };
        let part = &mut ppn.as_bytes()[start.page_offset()..end_offset];
        result.push(part);
        start = end_va;
    }
    Ok(result)
}

pub unsafe fn translated_string(space: &MemorySet, ptr: VirtAddr, len: usize) -> Result<String> {
    let raw_buffer = translated_byte_buffer(space, ptr, len)?;
    let buffer = raw_buffer
        .iter()
        .fold(Vec::<u8>::with_capacity(len), |mut acc, x| {
            acc.extend(x.iter());
            acc
        });
    String::from_utf8(buffer).map_err(|err| anyhow!("{}", err))
}

pub unsafe fn translated_refmut<T: 'static>(space: &MemorySet, ptr: *mut T) -> Result<&mut T> {
    //println!("into translated_refmut!");
    let va = ptr as usize;
    space.va_translate(VirtAddr::from(va)).map(|x| x.as_type())
}

///Array of u8 slice that user communicate with os
pub struct BufferHandle<'a> {
    ///U8 vec
    pub buffers: Vec<&'a mut [u8]>,
}

impl<'a> BufferHandle<'a> {
    ///Create a `UserBuffer` by parameter
    pub fn new(buffers: Vec<&'a mut [u8]>) -> Self {
        Self { buffers }
    }
    ///Length of `UserBuffer`
    pub fn len(&self) -> usize {
        let mut total: usize = 0;
        for b in self.buffers.iter() {
            total += b.len();
        }
        total
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        let mut data_idx: usize = 0;
        for buffer in self.buffers.iter_mut() {
            for x in buffer.iter_mut() {
                if data_idx >= data.len() {
                    return data_idx;
                }
                *x = data[data_idx];
                data_idx += 1;
            }
        }
        data_idx
    }
}

impl<'a> IntoIterator for BufferHandle<'a> {
    type Item = &'a mut u8;
    type IntoIter = UserBufferIterator<'a>;
    fn into_iter(self) -> UserBufferIterator<'a> {
        UserBufferIterator {
            buffers: self.buffers,
            current_buffer: 0,
            current_idx: 0,
        }
    }
}
/// Iterator of `UserBuffer`
pub struct UserBufferIterator<'a> {
    buffers: Vec<&'a mut [u8]>,
    current_buffer: usize,
    current_idx: usize,
}

impl<'a> Iterator for UserBufferIterator<'a> {
    type Item = &'a mut u8;
    fn next<'b>(&'b mut self) -> Option<&'a mut u8> {
        if self.current_buffer >= self.buffers.len() {
            None
        } else {
            let r = (&mut self.buffers[self.current_buffer][self.current_idx]) as *mut u8;
            if self.current_idx + 1 == self.buffers[self.current_buffer].len() {
                self.current_idx = 0;
                self.current_buffer += 1;
            } else {
                self.current_idx += 1;
            }
            Some(unsafe { &mut *r })
        }
    }
}
