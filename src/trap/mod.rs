pub mod context;

use core::arch::asm;

use log::warn;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, sstatus, stval, stvec,
};

use crate::{
    config::{TRAMPOLINE, TRAP_CONTEXT},
    mem::address::VirtAddr,
    stdlib::ansi::Colour,
    syscall::syscall,
    task::{
        current_task_trap_cx, exit_and_run_next, get_task, to_yield,
        user_token,
    },
    timer::set_next_trigger,
};

use self::context::TrapContext;

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE.0, TrapMode::Direct);
    }
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(__kernel_trap_entry as usize, TrapMode::Direct);
    }
}

pub fn get_trap(uid: usize) -> &'static TrapContext {
    let task = get_task(1001).unwrap();
    unsafe { &*((*task).trap_cx.0 as *const TrapContext) }
}

pub unsafe fn trap_handler() -> ! {
    set_kernel_trap_entry();
    let status = scause::read();
    let stval = stval::read();
    let cx = &mut *current_task_trap_cx();
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
            warn!("PageFault[{:#x}] in application, kernel killed it.", stval::read());
            exit_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            warn!("IllegalInstruction[{:#x}] in application, kernel killed it.", stval::read());
            exit_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                status.cause(),
                stval
            );
        }
    }
    unsafe {
        user_trap_return();
    }
}

#[repr(align(4))]
pub fn __kernel_trap_entry() {
    panic!("a trap from kernel!");
}

#[naked]
#[link_section = ".text.trampoline.entry"]
pub unsafe extern "C" fn user_trap_entry() {
    asm! {r"
        .altmacro
        .macro SAVE_GP n
            sd x\n, \n*8(sp)
        .endm
        ",
        // 保存上下文
        "
        csrrw sp, sscratch, sp
        sd ra, 1*8(sp)
        sd gp, 3*8(sp)
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
        sd t2, 2*8(sp)",
        // 切换到内核地址空间
        "
        ld t0, 35*8(sp)
        ld t1, 36*8(sp)
        ld sp, 34*8(sp)
        csrw satp, t0
        sfence.vma
        jr t1",
        options(noreturn)
    }
}

pub unsafe fn user_trap_return() -> ! {
    set_user_trap_entry();
    let satp = user_token();
    let restore = (user_restore as usize - user_trap_entry as usize) + TRAMPOLINE.0;
    let trap_context = TRAP_CONTEXT.0;
    asm! {
        "
        jr {restore}",
        restore = in(reg) restore,
        in("a0") trap_context,
        in("a1") satp,
        options(noreturn)
    }
}

#[naked]
#[link_section = ".text.trampoline"]
pub unsafe extern "C" fn user_restore(va_cx: usize, satp: usize) {
    asm! {r"
        .altmacro
        .macro LOAD_GP n
            ld x\n, \n*8(sp)
        .endm
        ",
        // 切换到用户虚拟地址空间，恢复寄存器
        "
        csrw satp, a1
        sfence.vma
        csrw sscratch, a0
        mv sp, a0
        ld t0, 32*8(sp)
        ld t1, 33*8(sp)
        csrw sstatus, t0
        csrw sepc, t1
        ld ra, 1*8(sp)
        ld gp, 3*8(sp)",
        // restore x5~x31
        "
        .set n, 5
        .rept 27
            LOAD_GP %n
            .set n, n+1
        .endr
        ld sp, 2*8(sp)
        sret
        ",
        options(noreturn)
    }
}

pub fn init() {
    set_user_trap_entry();
}

pub fn enable_timer_interrupt() {
    unsafe {
        sstatus::clear_sie();
        sie::set_stimer();
    }
    set_next_trigger();
}
