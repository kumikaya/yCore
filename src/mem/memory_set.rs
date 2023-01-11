use core::{arch::asm, ops::Range};
use alloc::{vec::Vec, collections::BTreeMap};
use anyhow::Result;
use bitflags::bitflags;
use log::info;
use riscv::register::satp;
use xmas_elf::{program::ProgramHeader, ElfFile};

use crate::{
    config::{
        PAGE_SIZE, MEMORY_END, TRAMPOLINE, GUARD_PAGE_SIZE,
        USER_STACK_SIZE, TRAP_CONTEXT
    },
    board::MMIO, mem::address::PhysAddr,
};

use super::{
    address::{PhysPageNum, VPNRange, VirtAddr, VirtPageNum},
    page_table::{PTEFlags, PageTable, PageTableEntry}, frame_allocator::{FrameTracker, frame_alloc},
};

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn stack_top();
    fn stack_bottom();
    fn sbss();
    fn ebss();
    fn ekernel();
}

bitflags! {
    pub struct MapPerm: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const RWU = Self::R.bits | Self::W.bits | Self::U.bits;
        const RXU = Self::R.bits | Self::X.bits | Self::U.bits;
        const RW  = Self::R.bits | Self::W.bits;
        const RX  = Self::R.bits | Self::X.bits;
        const RWX = Self::R.bits | Self::W.bits | Self::X.bits;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MapType {
    Identical,
    Framed,
}

#[derive(Debug)]
pub struct MapArea {
    pub range: VPNRange,
    pub perm: MapPerm,
    pub map_type: MapType,
    data_frames: BTreeMap<VirtPageNum, FrameTracker>,
}

#[derive(Debug)]
pub struct MemorySet {
    page_table: PageTable,
    pub areas: Vec<MapArea>,
}

impl MapArea {

    pub fn new(va_start: VirtAddr, va_end: VirtAddr, perm: MapPerm, map_type: MapType) -> Self {
        Self {
            range: va_start.floor()..va_end.ceil(),
            perm,
            map_type,
            data_frames: BTreeMap::new(),
        }
    }

    pub fn from_range<T: Into<VirtAddr>>(range: Range<T>, perm: MapPerm, map_type: MapType) -> Self {
        let va_start: VirtAddr = range.start.into();
        let va_end: VirtAddr = range.end.into();
        Self::new(va_start, va_end, perm, map_type)
    }

    pub fn from_another(another: &MapArea) -> Self {
        Self {
            range: another.range.clone(),
            data_frames: BTreeMap::new(),
            ..*another
        }
    }

    pub fn copy_data(&self, page_table: &mut PageTable, data: &[u8]) {
        let mut vpn_iter = self.range.clone().into_iter();
        let mut start = 0;
        while start < data.len() {
            let pte = page_table
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

    pub fn map_area(&mut self, page_table: &mut PageTable) {
        for vpn in self.range.clone() {
            let ppn = match self.map_type {
                MapType::Identical => PhysPageNum::from(vpn.0),
                MapType::Framed => {
                    let frame = frame_alloc().unwrap();
                    let ppn = frame.ppn;
                    self.data_frames.insert(vpn, frame);
                    ppn
                }
            };
            page_table.map(vpn, ppn, PTEFlags::from_bits_truncate(self.perm.bits()));
        }
    }

    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        if let MapType::Framed = self.map_type {
            self.data_frames.remove(&vpn);
        }
        page_table.unmap_uncheck(vpn);
    }

    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.range.clone() {
            self.unmap_one(page_table, vpn);
        }
    }

    pub fn from_ph<'a>(ph: ProgramHeader<'a>) -> Self {
        let start_va = ph.virtual_addr() as usize;
        let end_va: VirtAddr = (start_va + ph.mem_size() as usize).into();
        let start_va: VirtAddr = start_va.into();
        let mut perm = MapPerm::U;
        let flags = ph.flags();
        if flags.is_read() {
            perm |= MapPerm::R;
        }
        if flags.is_write() {
            perm |= MapPerm::W;
        }
        if flags.is_execute() {
            perm |= MapPerm::X;
        }
        MapArea::new(start_va, end_va, perm, MapType::Framed)
    }

}


impl MemorySet {

    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    pub fn from_existed_user(user_space: &MemorySet) -> MemorySet {
        let mut memory_set = Self::new_bare();
        // map trampoline
        memory_set.map_trampoline();
        // copy data sections/trap_context/user_stack
        for area in user_space.areas.iter() {
            let new_area = MapArea::from_another(area);
            memory_set.push(new_area, None);
            // copy data from another space
            for vpn in area.range.clone() {
                let src_ppn = user_space.translate(vpn).unwrap().ppn();
                let dst_ppn = memory_set.translate(vpn).unwrap().ppn();
                unsafe {
                    dst_ppn.as_pte_array().copy_from_slice(src_ppn.as_pte_array());
                }
            }
        }
        memory_set
    }

    pub fn va_translate(&self, va: VirtAddr) -> Option<PhysAddr> {
        self.page_table.va_translate(va)
    }

    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.page_table.translate(vpn)
    }

    pub fn token(&self) -> usize {
        self.page_table.token()
    }

    pub fn activate(&self) {
        unsafe {
            satp::write(self.page_table.token());
            asm!("sfence.vma");
        }
    }

    pub fn malloc(&mut self, vpn: VirtPageNum, flags: PTEFlags) -> Result<()> {
        self.page_table.malloc(vpn, flags)
    }

    pub fn free(&mut self, vpn: VirtPageNum) -> Result<()> {
        self.page_table.free(vpn)
    }

    pub fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {
        map_area.map_area(&mut self.page_table);
        if let Some(data) = data {
            map_area.copy_data(&mut self.page_table, data)
        }
        self.areas.push(map_area);
    }

    pub fn remove_area_with_start_vpn(&mut self, start_vpn: VirtPageNum) {
        if let Some((idx, area)) = self
            .areas
            .iter_mut()
            .enumerate()
            .find(|(_, area)| area.range.start == start_vpn)
        {
            area.unmap(&mut self.page_table);
            self.areas.remove(idx);
        }
    }

    pub fn remove_area_with_end_vpn(&mut self, end_vpn: VirtPageNum) {
        if let Some((idx, area)) = self
            .areas
            .iter_mut()
            .enumerate()
            .find(|(_, area)| area.range.end == end_vpn)
        {
            area.unmap(&mut self.page_table);
            self.areas.remove(idx);
        }
    }

    fn map_trampoline(&mut self) {
        extern "C" {
            fn strampoline();
        }
        self.page_table.map(
            TRAMPOLINE.into(),
            PhysAddr(strampoline as usize).into(),
            PTEFlags::R | PTEFlags::X
        );
        
    }

    pub fn from_elf(elf: &ElfFile) -> (Self, usize, usize) {
        let mut result = Self::new_bare();
        result.map_trampoline();
        let header = elf.header;
        let entry_point = header.pt2.entry_point() as usize;
        let magic = header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        let ph_count = header.pt2.ph_count();
        let mut program_vpn_end = VirtPageNum(0);
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            match ph.get_type().unwrap() {
                xmas_elf::program::Type::Load => {
                    let map_area = MapArea::from_ph(ph);
                    let offset = ph.offset() as usize;
                    let data = &elf.input[offset..(offset + ph.file_size() as usize)];
                    let vpn_end = map_area.range.end;
                    if vpn_end > program_vpn_end {
                        program_vpn_end = vpn_end;
                    }
                    result.push(map_area, Some(data));
                },
                _ => (),
            }
        }
        assert_ne!(program_vpn_end.0, 0, "empty program ");
        let user_stack_top = VirtAddr::from(program_vpn_end) + VirtAddr::from(GUARD_PAGE_SIZE);
        let user_stack_bottom = user_stack_top + VirtAddr::from(USER_STACK_SIZE);
        // maping user stack
        result.push(
            MapArea::new(
                user_stack_top,
                user_stack_bottom,
                MapPerm::RWU,
                MapType::Framed,
            ),
            None,
        );
        // map TrapContext
        result.push(
            MapArea::new(
                TRAP_CONTEXT,
                TRAP_CONTEXT.offset(1),
                MapPerm::RW,
                MapType::Framed,
            ),
            None,
        );
        (result, entry_point, user_stack_bottom.into())
    }

    pub fn build_kernel_space() -> Self {
        let mut result = Self::new_bare();
        result.map_trampoline();
        let text = (stext as usize)..(etext as usize);
        let rodata = (srodata as usize)..(erodata as usize);
        let data =     (sdata as usize)..(edata as usize);
        let bss =      (sbss as usize)..(ebss as usize);
        let phys_mem = (ekernel as usize)..(MEMORY_END & !PAGE_SIZE);
        let stack =    (stack_top as usize)..(stack_bottom as usize);
        assert!(text.start %     PAGE_SIZE == 0);
        assert!(rodata.start %   PAGE_SIZE == 0);
        assert!(data.start %     PAGE_SIZE == 0);
        assert!(bss.start %      PAGE_SIZE == 0);
        assert!(phys_mem.start % PAGE_SIZE == 0);
        // info!(".text:   [{:#x}, {:#x}), {}kb", text.0,     text.1,     (text.1 -     text.0) /     1024);
        // info!(".rodata: [{:#x}, {:#x}), {}kb", rodata.0,   rodata.1,   (rodata.1 -   rodata.0) /   1024);
        // info!(".data:   [{:#x}, {:#x}), {}kb", data.0,     data.1,     (data.1 -     data.0) /     1024);
        // info!(".stack:  [{:#x}, {:#x}), {}kb", stack.0,    stack.1,    (stack.1 -    stack.0) /    1024);
        // info!(".bss:    [{:#x}, {:#x}), {}kb", stack.1,    bss.1,      (bss.1 -      bss.0) /      1024);
        // info!(".other:  [{:#x}, {:#x}), {}kb", phys_mem.0, phys_mem.1, (phys_mem.1 - phys_mem.0) / 1024);
        // maping text segment
        result.push(
            MapArea::from_range(
                text,
                MapPerm::RX,
                MapType::Identical,
            ),
            None,
        );
        // maping rodata segment
        result.push(
            MapArea::from_range(
                rodata,
                MapPerm::R,
                MapType::Identical,
            ),
            None,
        );
        result.push(
            MapArea::from_range(
                data,
                MapPerm::RW,
                MapType::Identical,
            ),
            None,
        );
        // maping bss segment
        result.push(
            MapArea::from_range(
                bss,
                MapPerm::RW,
                MapType::Identical,
            ),
            None,
        );
        // maping physical memory
        result.push(
            MapArea::from_range(
                phys_mem,
                MapPerm::RW,
                MapType::Identical,
            ),
            None,
        );
        for (idx, pair) in MMIO.iter().enumerate() {
            // info!(".mmio_{}: [{:#x}, {:#x}), {}kb", idx, pair.0, pair.0 + pair.1, pair.1 / 1024);
            let range = (pair.0)..(pair.0 + pair.1);
            result.push(
                MapArea::from_range(
                    range,
                    MapPerm::RW,
                    MapType::Identical,
                ),
                None,
            );
        }
        // info!("kernel size: {}kb", (phys_mem.0 - text.0) / 1024);
        info!("kernel table size:  {}kb", result.page_table.size() / 1024);
        result
    }
}

#[cfg(feature = "debug_test")]
pub fn identical_map_test() {
    use crate::println;
    use crate::tools::ansi::{Colour, Color};
    use crate::kernel::KERNEL;
    let text =     VirtAddr(stext as usize);
    let rodata =   VirtAddr(srodata as usize);
    let data =     VirtAddr(sdata as usize);
    let bss =      VirtAddr(sbss as usize);
    let phys_mem = VirtAddr(ekernel as usize);
    let kernel_pt = KERNEL.kernel_space.borrow_mut();

    assert!(kernel_pt.translate(text.floor()).unwrap().flags().contains(    PTEFlags::V | PTEFlags::R | PTEFlags::X));
    assert!(kernel_pt.translate(rodata.floor()).unwrap().flags().contains(  PTEFlags::V | PTEFlags::R));
    assert!(kernel_pt.translate(data.floor()).unwrap().flags().contains(    PTEFlags::V | PTEFlags::R | PTEFlags::W));
    assert!(kernel_pt.translate(bss.floor()).unwrap().flags().contains(     PTEFlags::V | PTEFlags::R | PTEFlags::W));
    assert!(kernel_pt.translate(phys_mem.floor()).unwrap().flags().contains(PTEFlags::V | PTEFlags::R | PTEFlags::W));

    let vpn_range = VPNRange::new(VirtAddr(stext as usize).floor(), VirtAddr(MEMORY_END as usize).floor());
    for vpn in vpn_range {
        let pte = kernel_pt.translate(vpn).unwrap();
        let vaddr = VirtAddr::from(vpn).0;
        let maddr = PhysAddr::from(pte.ppn()).0;
        assert_eq!(vaddr, maddr);
    }
    println!("[{}] kernel_map_test", "passed".dye(Color::GreenB));
}

#[cfg(feature = "debug_test")]
pub fn framed_map_test() {
    use crate::println;
    use crate::tools::ansi::{Colour, Color};

    let range = (VirtAddr(stext as usize), VirtAddr(stext as usize + 7 * PAGE_SIZE));
    let data: Vec<u8> = (range.0.0..range.1.0).map(|x| x as u8).collect();
    let mut mem_set = MemorySet::new_bare();
    mem_set.push(
        MapArea::new(
            range.0,
            range.1,
            MapPerm::RW,
            MapType::Framed,
        ),
        Some(data.as_slice()),
    );
    let vpn_start = range.0.floor();
    for vpn in VPNRange::new(vpn_start, range.1.ceil()) {
        let map_data = unsafe {
            mem_set.page_table.translate(vpn).unwrap().ppn().as_bytes()
        };
        let start = PAGE_SIZE * (vpn - vpn_start).0;
        assert_eq!(map_data, &data[start..(start + PAGE_SIZE)]);
    }
    println!("[{}] framed_map_data_test", "passed".dye(Color::GreenB));
}