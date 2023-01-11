use core::default::default;

use super::trap_handler;
use riscv::register::sstatus::{self, Sstatus, SPP};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TrapContext {
    pub reg_file: RegFile,   // 0
    pub sstatus: Sstatus,    // 31
    pub sepc: usize,         // 32
    pub ksp: usize,          // 33
    /// 内核空间的token
    pub satp: usize,         // 34
    pub trap_handler: usize, // 35
    pub hartid: usize,       // 36
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct RegFile {
    pub ra: usize,      // 0
    pub sp: usize,      // 1
    pub gp: usize,      // 2
    pub tp: usize,      // 3
    pub a: [usize; 8],  // 4
    pub s: [usize; 12], // 12
    pub t: [usize; 7],  // 24
}

// const REG_NAME: &[&'static str] = &["none", "ra", "sp", "gp"];

// impl Display for TrapContext {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "TrapContext: {{")?;
//         for (idx, &name) in REG_NAME.iter().enumerate() {
//             write!(f, "{}: {:#X}, ", name, self.x[idx])?;
//         }
//         write!(f, "sstatus: {:#X}, sepc: {:#X}}}", self.sstatus.bits(), self.sepc)
//     }
// }

impl TrapContext {
    // pub fn set_sp(&mut self, usp: usize) {
    //     self.x[2] = usp;
    // }
    pub fn init(entry: usize, usp: usize, ksp: usize, satp: usize) -> Self {
        // `spp` 保存发生中断前的特权级
        // unsafe { set_spp(SPP::User) };
        let sstatus = sstatus::read();
        assert_eq!(
            sstatus.spp(),
            SPP::User,
            "S mode applications are not supported"
        );
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
            hartid: 1234,
        }
    }

    #[inline]
    pub fn set_result(&mut self, val: usize) {
        self.reg_file.a[0] = val;
    }

    #[inline]
    pub fn syscall_id(&self) -> usize {
        self.reg_file.a[7]
    }

    #[inline]
    pub fn syscall_args(&self) -> (usize, usize, usize) {
        let a = &self.reg_file.a;
        (a[0], a[1], a[2])
    }
}
