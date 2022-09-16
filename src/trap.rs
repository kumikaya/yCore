use core::arch::{global_asm, asm};

use riscv::register::{
    scause::{self, Exception, Trap},
    sstatus::{self, Sstatus, SPP},
    stval, stvec, utvec::TrapMode,
};

use crate::{syscall::syscall, println, batch::run_next_app};

#[derive(Debug)]
#[repr(C)]
pub struct TrapContext {
    pub regs: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.regs[2] = sp;
    }
    pub fn init(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut result = Self {
            regs: [0; 32],
            sstatus,
            sepc: entry,
        };
        result.set_sp(sp);
        result
    }
}

global_asm!(include_str!("trap.S"));

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let status = scause::read();
    let stval = stval::read();
    match status.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.regs[10] = syscall(cx.regs[17], cx.regs[10], cx.regs[11], cx.regs[12]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_app();
        },
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app();
        },
        _ => panic!(
            "Unsupported trap {:?}, stval = {:#x}!",
            status.cause(),
            stval
        ),
    }
    cx
}


pub fn init() {
    extern "C" {
        fn __trap_entry();
    }
    unsafe {
        stvec::write(__trap_entry as usize, TrapMode::Direct)
    }
}
