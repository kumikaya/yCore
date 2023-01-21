use core::arch::naked_asm;

use self::{context::TaskContext, scheduler::add_task};
use crate::{fs::inode::open_app, task::scheduler::get_processor};

pub mod context;
pub mod process;
pub mod processor;
pub mod scheduler;
pub mod signal;
pub mod tcb;
pub mod tigger;
pub mod uid;

#[naked]
pub unsafe extern "C" fn __switch(
    current: *mut TaskContext,
    next: *mut TaskContext,
    send_lock: *mut u32,
) {
    naked_asm! {r"
        .altmacro
        .macro SAVE_S n
            sd s\n, (\n+2)*8(a0)
        .endm
        .macro LOAD_S n
            ld s\n, (\n+2)*8(a1)
        .endm
        sd sp, 8(a0)
        sd ra, 0(a0)
        .set n, 0
        .rept 12
            SAVE_S %n
            .set n, n + 1
        .endr
        amoswap.w.rl zero, zero, (a2)
        # sd zero, 0(a2)
        ld ra, 0(a1)
        .set n, 0
        .rept 12
        LOAD_S %n
        .set n, n + 1
        .endr
        ld sp, 8(a1)
        ret
        ",
        options()
    }
}

pub fn add_initproc() {
    // 添加初始程序
    let (_, initproc) = open_app("initproc", "").unwrap();
    add_task(initproc);
}

pub fn entrap_task() -> ! {
    // info!("hart {} into task", hartid);
    get_processor().entrap_task()
    // KERNEL.processors[hartid].entrap_task()
}
