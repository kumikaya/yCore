use alloc::{collections::BTreeMap, vec::Vec};
use bitflags::bitflags;
use lazy_static::lazy_static;
use log::info;

use crate::{config::{PAGE_SIZE, MEMORY_END, PAGE_WIDTH}, println, mem::address::{PhysAddr, SimpleRange}, board::MMIO, stdlib::cell::STCell};

use super::{
    address::{PhysPageNum, VPNRange, VirtAddr, VirtPageNum},
    frame_allocator::{frame_alloc, FrameTracker},
    page_table::{PTEFlags, PageTable},
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
    static ref KERNEL_SPACE: STCell<MemorySet> = {
        STCell::new(
            MemorySet::build_kernel_space()
        )
    };
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

struct MapArea {
    range: VPNRange,
    map: BTreeMap<VirtPageNum, FrameTracker>,
    perm: MapPermission,
    tp: MapType,
}

struct MemorySet {
    page_table: PageTable,
    map_area: Vec<MapArea>,
}


impl MapArea {
    pub fn size(&self) -> usize {
        (self.range.get_end().0 - self.range.get_start().0) * PAGE_SIZE
    }
    pub fn new(start_va: VirtAddr, end_va: VirtAddr, perm: MapPermission, tp: MapType) -> Self {
        Self {
            range: VPNRange::new(start_va.floor(), end_va.ceil()),
            map: BTreeMap::new(),
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
            unsafe {
                pte.ppn().as_bytes().copy_from_slice(&data[start..end]);
            }
            start = end;
        }
    }

    pub fn map(&mut self, page_table: &mut PageTable) {
        let tp = self.tp;
        for vpn in self.range {
            let ppn = match tp {
                MapType::Identical => PhysPageNum::from(vpn.0),
                MapType::Framed => {
                    let frame = frame_alloc().unwrap();
                    let ppn = frame.ppn;
                    self.map.insert(vpn, frame);
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
        self.map = BTreeMap::new();
    }
}

impl MemorySet {
    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            map_area: Vec::new(),
        }
    }
    pub fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {
        map_area.map(&mut self.page_table);
        if let Some(data) = data {
            map_area.copy_data(&self.page_table, data);
        }
        self.map_area.push(map_area);
    }

    pub fn build_kernel_space() -> Self {
        let mut result = Self::new_bare();
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
        info!(".text:   [{:#x}, {:#x}), {}kb", text.0,        text.1,        (text.1 -        text.0) / 1024);
        info!(".rodata: [{:#x}, {:#x}), {}kb", rodata.0,      rodata.1,      (rodata.1 -      rodata.0) / 1024);
        info!(".stack:  [{:#x}, {:#x}), {}kb", stack.0,       stack.1,       (stack.1 -       stack.0) / 1024);
        info!(".data:   [{:#x}, {:#x}), {}kb", data.0,        data.1,        (data.1 -        data.0) / 1024);
        info!(".bss:    [{:#x}, {:#x}), {}kb", bss.0,         bss.1,         (bss.1 -         bss.0) / 1024);
        info!(".other:  [{:#x}, {:#x}), {}kb", phys_mem.0,    phys_mem.1,    (phys_mem.1 -    phys_mem.0) / 1024);
        // map text segment
        result.push(
            MapArea::new(
                text.0.into(),
                text.1.into(),
                MapPermission::R | MapPermission::X,
                MapType::Identical,
            ),
            None,
        );
        // map rodata segment
        result.push(
            MapArea::new(
                rodata.0.into(),
                rodata.1.into(),
                MapPermission::R,
                MapType::Identical,
            ),
            None,
        );
        // map data segment
        result.push(
            MapArea::new(
                data.0.into(),
                data.1.into(),
                MapPermission::R | MapPermission::W,
                MapType::Identical,
            ),
            None,
        );
        // map bss segment
        result.push(
            MapArea::new(
                stack.0.into(),
                bss.1.into(),
                MapPermission::R | MapPermission::W,
                MapType::Identical,
            ),
            None,
        );
        // map physical memory
        result.push(
            MapArea::new(
                phys_mem.0.into(),
                phys_mem.1.into(),
                MapPermission::R | MapPermission::W,
                MapType::Identical,
            ),
            None,
        );
        info!("kernel memory size: {}k", (phys_mem.1 - text.0) / 1024);
        info!("kernel table size:  {}k", result.page_table.size() / 1024);
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
    let text =     VirtAddr(stext as usize);
    let rodata =   VirtAddr(srodata as usize);
    let data =     VirtAddr(sdata as usize);
    let bss =      VirtAddr(sbss as usize);
    let phys_mem = VirtAddr(ekernel as usize);
    let kernel_pt = &mut KERNEL_SPACE.borrow_mut().page_table;

    assert_eq!(kernel_pt.translate(text.floor()).unwrap().flags(),     PTEFlags::R | PTEFlags::X);
    assert_eq!(kernel_pt.translate(rodata.floor()).unwrap().flags(),   PTEFlags::R);
    assert_eq!(kernel_pt.translate(data.floor()).unwrap().flags(),     PTEFlags::R | PTEFlags::W);
    assert_eq!(kernel_pt.translate(bss.floor()).unwrap().flags(),      PTEFlags::R | PTEFlags::W);
    assert_eq!(kernel_pt.translate(phys_mem.floor()).unwrap().flags(), PTEFlags::R | PTEFlags::W);

    let vpn_range = VPNRange::new(VirtAddr(stext as usize).floor(), VirtAddr(MEMORY_END as usize).floor());
    for vpn in vpn_range {
        let pte = kernel_pt.translate(vpn).unwrap();
        let vaddr = VirtAddr::from(vpn).0;
        let maddr = PhysAddr::from(pte.ppn()).0;
        assert_eq!(vaddr, maddr);
    }
    println!("[passed] kernel_map_test");
}

#[cfg(feature = "debug_test")]
pub fn framed_map_test() {
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
        let start = PAGE_SIZE * (vpn.0 - vpn_start.0);
        assert_eq!(map_data, &data[start..(start + PAGE_SIZE)]);
    }
    println!("[passed] framed_map_data_test");
}