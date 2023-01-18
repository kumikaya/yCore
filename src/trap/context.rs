use core::default::default;

use super::trap_handler;
use riscv::register::sstatus::{self, Sstatus, SPP};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TrapContext {
    reg_file: RegFile,   // 0
    sstatus: Sstatus,    // 31
    sepc: usize,         // 32
    ksp: usize,          // 33
    /// 内核空间的token
    satp: usize,         // 34
    trap_handler: usize, // 35
    hartid: usize,       // 36
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
struct RegFile {
    pub ra: usize,      // 0
    pub sp: usize,      // 1
    pub gp: usize,      // 2
    pub tp: usize,      // 3
    pub a: [usize; 8],  // 4
    pub s: [usize; 12], // 12
    pub t: [usize; 7],  // 24
}

impl TrapContext {
    pub fn init(entry: usize, usp: usize, ksp: usize, satp: usize) -> Self {
        // `spp` 保存发生中断前的特权级
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        Self {
            reg_file: RegFile {
                sp: usp,
                ..default()
            },
            sstatus,
            sepc: entry,
            ksp,
            satp,
            trap_handler: trap_handler as usize,
            hartid: 0,
        }
    }

    #[inline]
    pub fn set_return(&mut self, val: usize) {
        self.reg_file.a[0] = val;
    }

    #[inline(always)]
    pub fn syscall_id(&self) -> usize {
        self.reg_file.a[7]
    }

    #[inline(always)]
    pub fn syscall_args(&self) -> [usize; 6] {
        let a = &self.reg_file.a;
        [a[0], a[1], a[2], a[3], a[4], a[5]]
    }

    pub unsafe fn set_hartid(&mut self, hartid: usize) {
        self.hartid = hartid
    }

    #[inline(always)]
    pub unsafe fn set_next_sepc(&mut self) {
        self.sepc += 4;
    }
    pub fn get_ksp_bottom(&self) -> usize {
        self.ksp
    }
    pub unsafe fn set_ksp_bottom(&mut self, ksp: usize) {
        self.ksp = ksp
    }
    pub fn get_usp(&self) -> usize {
        self.reg_file.sp
    }
}
