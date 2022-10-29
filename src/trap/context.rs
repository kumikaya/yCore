use core::{fmt::{Display, Formatter, Write}, mem::size_of};

use alloc::{fmt::format, format, string::String};
use riscv::register::sstatus::{Sstatus, SPP, self};

use crate::stdlib::tools::align_size;


#[derive(Debug)]
#[repr(C)]
pub struct TrapContext {
    pub regs: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
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
    pub fn init(entry: usize, usp: usize, privilege_level: SPP) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(privilege_level);
        sstatus.set_sie(true);
        let mut result = Self {
            regs: [0; 32],
            sstatus,
            sepc: entry,
        };
        result.set_sp(usp);
        result
    }
}

pub fn push_trap_context(ksp: usize, cx: TrapContext) -> *mut TrapContext {
    assert_eq!(align_size::<TrapContext>(16), 34 * size_of::<usize>());
    let ksp = ksp - align_size::<TrapContext>(16);
    let cx_ptr = ksp as *mut TrapContext;
    unsafe {
        *cx_ptr = cx;
        cx_ptr
    }
}
