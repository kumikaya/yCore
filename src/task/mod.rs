use core::arch::asm;

use log::info;

use crate::{
    fs::inode::open_app,
    task::scheduler::get_processor,
};

use self::{scheduler::add_task, task::TaskContext};

pub mod pid;
pub mod processor;
pub mod scheduler;
pub mod task;
pub mod tigger;

#[naked]
pub unsafe extern "C" fn __switch(current: *mut TaskContext, next: *mut TaskContext) {
    asm! {r"
        .altmacro
        .macro SAVE_S n
            sd s\n, (\n+2)*8(a0)
        .endm
        .macro STORE_S n
            ld s\n, (\n+2)*8(a1)
        .endm
        sd sp, 8(a0)
        sd ra, 0(a0)
        .set n, 0
        .rept 12
            SAVE_S %n
            .set n, n + 1
        .endr
        ld ra, 0(a1)
        .set n, 0
        .rept 12
        STORE_S %n
        .set n, n + 1
        .endr
        ld sp, 8(a1)
        ret
        ",
        options(noreturn)
    }
}

pub fn add_initproc() {
    // 添加初始程序
    let initproc = open_app("initproc", "").unwrap();
    add_task(initproc);
}

pub fn entrap_task() -> ! {
    // info!("hart {} into task", hartid);
    get_processor().entrap_task()
    // KERNEL.processors[hartid].entrap_task()
}
