pub mod context;

use core::arch::{asm, global_asm};

use log::warn;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, sstatus, stval, stvec,
};

use crate::{
    config::{TRAMPOLINE, TRAP_CONTEXT},
    kernel::{Schedule, KERNEL},
    syscall::Syscall,
    task::processor::Hart,
    timer::set_next_trigger,
};

#[inline]
fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE.into(), TrapMode::Direct);
    }
}

#[inline]
fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(kernel_trap_entry as usize, TrapMode::Direct);
    }
}

#[allow(unused)]
/// 该函数内的强引用可能需要手动释放
pub unsafe extern "C" fn trap_handler(hartid: usize) -> ! {
    set_kernel_trap_entry();
    let status = scause::read();
    let env = &KERNEL.processor;
    // let cx = KERNEL.current_task().trap_context();
    let cx = env.current_task().trap_context();
    match status.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            let result = env.syscall(cx.syscall_id(), cx.syscall_args());
            // let result = syscall(cx.syscall_id(), cx.syscall_args());
            cx.set_result(result as usize);
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            env._yield();
            // _yield();
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            warn!("PageFault[{:#x}]", stval::read());
            env.exit_current(-1);
            // exit_current(-1);
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            warn!("IllegalInstruction[{:#x}]", stval::read());
            env.exit_current(-1);
            // exit_current(-1);
        }
        trap => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", trap, stval::read());
        }
    }
    let satp = env.current_task().space().token();
    unsafe { user_trap_return(satp) }
}

#[repr(align(4))]
pub fn kernel_trap_entry() {
    panic!("a trap from kernel!");
}

// 定义从栈上保存或恢复寄存器的汇编宏
global_asm! {r"
    .macro LOAD reg, idx, offset
        ld \reg\idx, \offset*8(sp)
    .endm
    .macro LOADS reg, size, offset
        .set i, 0
        .rept \size
            LOAD \reg, %i, %(i+\offset)
            .set i, i+1
        .endr
    .endm

    .macro STORE reg, idx, offset
        sd \reg\idx, \offset*8(sp)
    .endm
    .macro STORES reg, size, offset
        .set i, 0
        .rept \size
            STORE \reg, %i, %(i+\offset)
            .set i, i+1
        .endr
    .endm
"}

#[naked]
#[link_section = ".text.trampoline.entry"]
pub unsafe extern "C" fn user_trap_entry() {
    asm! {r"
        .altmacro
        # 保存sp寄存器，同时获取 `TrapContext` 用户空间指针
        csrrw sp, sscratch, sp
        sd ra, 0*8(sp)
        sd gp, 2*8(sp)
        STORES a, 8, 4
        STORES s, 12, 12
        STORES t, 7, 24
        csrr t0, sstatus
        csrr t1, sepc
        sd t0, 31*8(sp)
        sd t1, 32*8(sp)
        csrr t2, sscratch
        sd t2, 1*8(sp)
        ld t0, 34*8(sp)
        ld t1, 35*8(sp)
        # hartid 作为参数
        ld a0, 36*8(sp)
        # 切换到内核栈
        ld sp, 33*8(sp)
        # 切换到内核空间
        csrw satp, t0
        sfence.vma
        # 跳转到 trap_handler
        jr t1",
        options(noreturn)
    }
}

#[naked]
pub unsafe extern "C" fn init_app_trap_return() {
    asm! {r"
        mv a0, s0
        mv s0, zero
        j {trap_return}
        ",
        trap_return = sym user_trap_return,
        options(noreturn)
    }
}

#[inline]
pub unsafe fn user_trap_return(satp: usize) -> ! {
    set_user_trap_entry();
    // let satp = KERNEL.user_space().token();
    let restore = (user_restore as usize - user_trap_entry as usize) + usize::from(TRAMPOLINE);
    let trap_context: usize = TRAP_CONTEXT.into();
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
        # 切换到用户空间
        csrw satp, a1
        sfence.vma
        # 保存 `TrapContext` 用户空间指针到 sscratch 寄存器
        csrw sscratch, a0
        mv sp, a0
        ld t0, 31*8(sp)
        ld t1, 32*8(sp)
        csrw sstatus, t0
        csrw sepc, t1
        ld ra, 0*8(sp)
        ld gp, 2*8(sp)
        LOADS a, 8, 4
        LOADS s, 12, 12
        LOADS t, 7, 24
        ld sp, 1*8(sp)
        sret
        ",
        options(noreturn)
    }
}

pub fn init() {
    set_user_trap_entry();
    enable_timer_interrupt();
}

pub fn enable_timer_interrupt() {
    unsafe {
        sstatus::clear_sie();
        sie::set_stimer();
    }
    set_next_trigger();
}
