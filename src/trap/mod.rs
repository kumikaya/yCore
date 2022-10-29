pub mod context;

use core::arch::asm;

use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec, sstatus::{SPP, self},
};

use crate::{
    println,
    syscall::syscall,
    task::{exit_and_run_next, get_current_task, to_yield},
    timer::set_next_trigger,
};

use self::context::TrapContext;

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) {
    let status = scause::read();
    let stval = stval::read();
    match status.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.regs[10] = syscall(cx.regs[17], cx.regs[10], cx.regs[11], cx.regs[12]) as usize;
        }

        Trap::Interrupt(Interrupt::SupervisorExternal) => {
            println!("Interrupt at supervisor.");
        }

        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            to_yield();
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            // debug_task_info(&cx);
            println!("[kernel] PageFault in application, kernel killed it.");
            exit_and_run_next()
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            // debug_task_info(&cx);
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_and_run_next()
        }
        _ => {
            debug_task_info(&cx);
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                status.cause(),
                stval
            );
        }
    }
}

pub fn debug_task_info(cx: &TrapContext) {
    let task = get_current_task();
    println!("task: {}", unsafe { &*task });
    println!("{}", cx);
}

#[naked]
#[repr(align(4))]
pub unsafe extern "C" fn __trap_entry() {
    asm! {r"
        .altmacro
        .macro SAVE_GP n
            sd x\n, \n*8(sp)
        .endm
        ",
        "
        csrrw sp, sscratch, sp
        addi sp, sp, -34*8
        sd ra, 1*8(sp)
        sd gp, 3*8(sp)",
        // save x5~x31
        "
        .set n, 5
        .rept 27
            SAVE_GP %n
            .set n, n+1
        .endr",
        "
        csrr t0, sstatus
        csrr t1, sepc
        sd t0, 32*8(sp)
        sd t1, 33*8(sp)
        csrr t2, sscratch
        sd t2, 2*8(sp)
        mv a0, sp
        call {trap_handler}
        j {restore}
        ",
        trap_handler = sym trap_handler,
        restore = sym __restore,
        options(noreturn)
    }
}

#[naked]
pub unsafe extern "C" fn __restore() {
    asm! {r"
        .altmacro
        .macro LOAD_GP n
            ld x\n, \n*8(sp)
        .endm
        ",
        "
        ld t0, 32*8(sp)
        ld t1, 33*8(sp)
        ld t2, 2*8(sp)
        csrw sstatus, t0
        csrw sepc, t1
        csrw sscratch, t2
        ld ra, 1*8(sp)
        ld gp, 3*8(sp)",
        // restore x5~x31
        "
        .set n, 5
        .rept 27
            LOAD_GP %n
            .set n, n+1
        .endr
        addi sp, sp, 34*8
        csrrw sp, sscratch, sp
        sret
        ",
        options(noreturn)
    }
}

pub fn init() {
    unsafe {
        let trap_entry = __trap_entry as usize;
        stvec::write(trap_entry, TrapMode::Direct);
        assert!(trap_entry & 0b11 == 0, "the __trap_entry has not aligned.");
    }
}

pub fn enable_timer_interrupt() {
    unsafe { sie::set_stimer() };
    set_next_trigger();
}
