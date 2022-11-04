use core::fmt::Display;

use riscv::register::sstatus::{Sstatus, SPP, self};
use super::trap_handler;


#[derive(Debug)]
#[repr(C)]
pub struct TrapContext {
    pub regs: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
    pub ksp: usize,
    pub satp: usize,
    pub trap_handler: usize,
}

const REG_NAME: &[&'static str] = &[
    "none",
    "ra",
    "sp",
    "gp",
];

impl Display for TrapContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TrapContext: {{")?;
        for (idx, &name) in REG_NAME.iter().enumerate() {
            write!(f, "{}: {:#X}, ", name, self.regs[idx])?;
        }
        write!(f, "sstatus: {:#X}, sepc: {:#X}}}", self.sstatus.bits(), self.sepc)
    }
}

impl TrapContext {
    pub fn set_sp(&mut self, usp: usize) {
        self.regs[2] = usp;
    }
    pub fn init(entry: usize, usp: usize, ksp: usize, satp: usize, privilege_level: SPP) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(privilege_level);
        let mut result = Self {
            regs: [0; 32],
            sstatus,
            sepc: entry,
            ksp,
            satp,
            trap_handler: trap_handler as usize,
        };
        result.set_sp(usp);
        result
    }
}

