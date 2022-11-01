use core::arch::asm;

use alloc::{collections::BTreeMap, vec::Vec, format};
use bitflags::bitflags;
use lazy_static::lazy_static;
use log::info;
use riscv::register::satp;
use xmas_elf::{program::ProgramHeader, ElfFile};

use crate::{
    config::{
        PAGE_SIZE, MEMORY_END, TRAMPOLINE, GUARD_PAGE_SIZE,
        USER_STACK_SIZE, KERNEL_STACK_TOP, KERNEL_STACK_BOTTOM, TRAP_CONTEXT
    },
    board::MMIO, println, mem::address::PhysAddr,
    stdlib::cell::STCell
};

use super::{
    address::{PhysPageNum, VPNRange, VirtAddr, VirtPageNum},
    frame_allocator::{frame_alloc, FrameTracker},
    page_table::{PTEFlags, PageTable, PageTableEntry},
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

lazy_static! {
    pub static ref KERNEL_SPACE: STCell<MemorySet> = {
        STCell::new(
            MemorySet::build_kernel_space()
        )
    };
}

pub fn push_kernel_stack(va_start: VirtAddr, va_end: VirtAddr) {
    KERNEL_SPACE.borrow_mut().push(
        MapArea::new(
            va_start,
            va_end,
            MapPermission::R | MapPermission::W,
            MapType::Framed,
        ),
        None
    );
}

pub fn init_kernel_space() {
    KERNEL_SPACE.borrow_mut().activate();
}

pub fn kernel_token() -> usize {
    KERNEL_SPACE.borrow().token()
}

bitflags! {
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MapType {
    Identical,
    Framed,
}

#[derive(Debug)]
pub struct MapArea {
    range: VPNRange,
    map: Vec<FrameTracker>,
    perm: MapPermission,
    tp: MapType,
}

#[derive(Debug)]
pub struct MemorySet {
    pub page_table: PageTable,
    pub map_area: Vec<MapArea>,
}

impl MapArea {

    pub fn new(va_start: VirtAddr, va_end: VirtAddr, perm: MapPermission, tp: MapType) -> Self {
        Self {
            range: VPNRange::new(va_start.floor(), va_end.ceil()),
            map: Vec::new(),
            perm,
            tp,
        }
    }

    pub fn copy_data(&mut self, page_table: &PageTable, data: &[u8]) {
        let mut vpn_iter = self.range.into_iter();
        let mut start = 0;
        while start < data.len() {
            let pte = page_table
                .translate(
                    vpn_iter
                        .next()
                        .expect("Virtual address space range overflow"),
                )
                .expect("Unassigned frame for the virtual address.");
            let end = (start + PAGE_SIZE).min(data.len());
            let src = &data[start..end];
            unsafe {
                pte.ppn().as_bytes()[..src.len()].copy_from_slice(src);
            }
            start = end;
        }
    }

    pub fn from_ph<'a>(ph: ProgramHeader<'a>) -> Self {
        let start_va = ph.virtual_addr() as usize;
        let end_va: VirtAddr = (start_va + ph.mem_size() as usize).into();
        let start_va: VirtAddr = start_va.into();
        let mut perm = MapPermission::U;
        let flags = ph.flags();
        if flags.is_read() {
            perm |= MapPermission::R;
        }
        if flags.is_write() {
            perm |= MapPermission::W;
        }
        if flags.is_execute() {
            perm |= MapPermission::X;
        }
        MapArea::new(start_va, end_va, perm, MapType::Framed)
    }

    pub fn map(&mut self, page_table: &mut PageTable) {
        let tp = self.tp;
        for vpn in self.range {
            let ppn = match tp {
                MapType::Identical => PhysPageNum::from(vpn.0),
                MapType::Framed => {
                    let frame = frame_alloc().unwrap();
                    let ppn = frame.ppn;
                    self.map.push(frame);
                    // self.map.insert(vpn, frame);
                    ppn
                }
            };
            page_table.map(vpn, ppn, PTEFlags::from_bits_truncate(self.perm.bits));
        }
    }

    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.range {
            page_table.unmap(vpn);
        }
        self.map = Vec::new();
    }
    pub fn check(&self, page_table: &PageTable) {
        for vpn in self.range {
            if None == page_table.va_translate(vpn.into()) {
                log::error!("Should map vpn: {:#X}, but not!", vpn.0);
            }
        }
    }
}

impl MemorySet {

    pub fn check(&self) {
        for map_area in self.map_area.iter() {
            map_area.check(&self.page_table);
        }
    }

    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            map_area: Vec::new(),
        }
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

    pub fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {
        map_area.map(&mut self.page_table);
        if let Some(data) = data {
            map_area.copy_data(&self.page_table, data);
        }
        self.map_area.push(map_area);
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
        // let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
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
                    let vpn_end = map_area.range.get_end();
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
                MapPermission::R | MapPermission::W | MapPermission::U,
                MapType::Framed,
            ),
            None,
        );
        // map TrapContext
        result.push(
            MapArea::new(
                TRAP_CONTEXT,
                TRAMPOLINE,
                MapPermission::R | MapPermission::W,
                MapType::Framed,
            ),
            None,
        );
        // maping kernel stack
        // result.push(
        //     MapArea::new(
        //         KERNEL_STACK_TOP,
        //         KERNEL_STACK_BOTTOM,
        //         MapPermission::R | MapPermission::W,
        //         MapType::Framed,
        //     ),
        //     None,
        // );
        (result, entry_point, user_stack_bottom.0)
    }

    pub fn build_kernel_space() -> Self {
        let mut result = Self::new_bare();
        result.map_trampoline();
        let text =     (stext as usize,     etext as usize);
        let rodata =   (srodata as usize,   erodata as usize);
        let data =     (sdata as usize,     edata as usize);
        let bss =      (sbss as usize,      ebss as usize);
        let phys_mem = (ekernel as usize,   (MEMORY_END & !PAGE_SIZE));
        let stack =    (stack_top as usize, stack_bottom as usize);
        assert!(text.0 %     PAGE_SIZE == 0);
        assert!(rodata.0 %   PAGE_SIZE == 0);
        assert!(data.0 %     PAGE_SIZE == 0);
        assert!(bss.0 %      PAGE_SIZE == 0);
        assert!(phys_mem.0 % PAGE_SIZE == 0);
        info!(".text:   [{:#x}, {:#x}), {}kb", text.0,     text.1,     (text.1 -     text.0) /     1024);
        info!(".rodata: [{:#x}, {:#x}), {}kb", rodata.0,   rodata.1,   (rodata.1 -   rodata.0) /   1024);
        info!(".data:   [{:#x}, {:#x}), {}kb", data.0,     data.1,     (data.1 -     data.0) /     1024);
        info!(".stack:  [{:#x}, {:#x}), {}kb", stack.0,    stack.1,    (stack.1 -    stack.0) /    1024);
        info!(".bss:    [{:#x}, {:#x}), {}kb", stack.1,    bss.1,      (bss.1 -      bss.0) /      1024);
        info!(".other:  [{:#x}, {:#x}), {}kb", phys_mem.0, phys_mem.1, (phys_mem.1 - phys_mem.0) / 1024);
        // maping text segment
        result.push(
            MapArea::new(
                text.0.into(),
                text.1.into(),
                MapPermission::R | MapPermission::X,
                MapType::Identical,
            ),
            None,
        );
        // maping rodata segment
        result.push(
            MapArea::new(
                rodata.0.into(),
                rodata.1.into(),
                MapPermission::R,
                MapType::Identical,
            ),
            None,
        );
        // maping stack segment
        // result.push(
        //     MapArea::new(
        //         stack.0.into(),
        //         stack.1.into(),
        //         MapPermission::R | MapPermission::W,
        //         MapType::Identical,
        //     ),
        //     None,
        // );
        // maping data segment
        result.push(
            MapArea::new(
                data.0.into(),
                data.1.into(),
                MapPermission::R | MapPermission::W,
                MapType::Identical,
            ),
            None,
        );
        // maping bss segment
        result.push(
            MapArea::new(
                bss.0.into(),
                bss.1.into(),
                MapPermission::R | MapPermission::W,
                MapType::Identical,
            ),
            None,
        );
        // maping physical memory
        result.push(
            MapArea::new(
                phys_mem.0.into(),
                phys_mem.1.into(),
                MapPermission::R | MapPermission::W,
                MapType::Identical,
            ),
            None,
        );
        info!("kernel size: {}kb", (phys_mem.0 - text.0) / 1024);
        info!("kernel table size:  {}kb", result.page_table.size() / 1024);
        for &pair in MMIO {
            result.push(
                MapArea::new(
                    pair.0.into(),
                    (pair.0 + pair.1).into(),
                    MapPermission::R | MapPermission::W,
                    MapType::Identical,
                ),
                None,
            );
        }
        result
    }
}

#[cfg(feature = "debug_test")]
pub fn identical_map_test() {
    use crate::stdlib::ansi::{Colour, Color};

    let text =     VirtAddr(stext as usize);
    let rodata =   VirtAddr(srodata as usize);
    let data =     VirtAddr(sdata as usize);
    let bss =      VirtAddr(sbss as usize);
    let phys_mem = VirtAddr(ekernel as usize);
    let kernel_pt = &mut KERNEL_SPACE.borrow_mut().page_table;

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
    use crate::stdlib::ansi::{Colour, Color};

    let range = (VirtAddr(stext as usize), VirtAddr(stext as usize + 7 * PAGE_SIZE));
    let data: Vec<u8> = (range.0.0..range.1.0).map(|x| x as u8).collect();
    let mut mem_set = MemorySet::new_bare();
    mem_set.push(
        MapArea::new(
            range.0,
            range.1,
            MapPermission::R | MapPermission::W,
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