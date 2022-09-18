use core::arch::global_asm;

use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie,
    sstatus::{self, Sstatus, SPP},
    stval, stvec,
};

use crate::{
    println,
    syscall::syscall,
    task::{exit_and_run_next, to_yield},
    timer::set_next_trigger,
};

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
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            to_yield();
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            exit_and_run_next()
            // run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_and_run_next()
            // run_next_app();
        }
        _ => panic!(
            "Unsupported trap {:?}, stval = {:#x}!",
            status.cause(),
            stval
        ),
    }
    cx
}

extern "C" {
    pub fn __trap_entry();
    pub fn __restore();
}

pub fn init() {
    unsafe { stvec::write(__trap_entry as usize, TrapMode::Direct) }
}

pub fn enable_timer_interrupt() {
    unsafe { sie::set_stimer() };
    set_next_trigger();
}
