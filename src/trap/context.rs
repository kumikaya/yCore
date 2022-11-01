use core::{fmt::Display, mem::size_of};

use riscv::register::sstatus::{Sstatus, SPP, self};

use crate::{stdlib::tools::aligned_size, mem::{address::{PhysAddr, VirtAddr}, memory_set::MemorySet}, config::TRAP_CONTEXT};

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
        sstatus.set_sie(true);
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


pub unsafe fn push_trap_context(memory_set: &MemorySet, cx: TrapContext) {
    let cx_pa = memory_set.va_translate(TRAP_CONTEXT).unwrap();
    // let cx = TrapContext::init(entry, usp, ksp, memory_set.token(), SPP::User);
    assert_eq!(aligned_size::<TrapContext>(16), 38 * size_of::<usize>());
    unsafe {
        (*(cx_pa.0 as *mut TrapContext)) = cx;
    }
}

pub unsafe fn __push_trap_context(ksp: usize, entry: usize, usp: usize, satp: usize) -> *mut TrapContext {
    let ksp = ksp - aligned_size::<TrapContext>(16);
    let cx = TrapContext::init(entry, usp, ksp, satp, SPP::User);
    assert_eq!(aligned_size::<TrapContext>(16), 38 * size_of::<usize>());
    let cx_ptr = ksp as *mut TrapContext;
    unsafe {
        *cx_ptr = cx;
        cx_ptr
    }
}
