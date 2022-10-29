use crate::{
    println,
    task::task::{Task, TaskContex},
};
use core::arch::asm;

#[naked]
pub unsafe extern "C" fn __switch(current: *mut TaskContex, next: *mut TaskContex) {
    asm! {r"
        .altmacro
        .macro SAVE_SN n
            sd s\n, (\n+2)*8(a0)
        .endm
        .macro LOAD_SN n
            ld s\n, (\n+2)*8(a1)
        .endm
        ",
        // save kernel stack of current task
        "
        sd sp, 8(a0)
        ",
        // save ra & s0~s11 of current execution
        "
        sd ra, 0(a0)
        .set n, 0
        .rept 12
            SAVE_SN %n
            .set n, n + 1
        .endr
        ",
        // restore ra & s0~s11 of next execution
        "
        ld ra, 0(a1)
        .set n, 0
        .rept 12
            LOAD_SN %n
            .set n, n + 1
        .endr
        ",
        // restore kernel stack of next task
        "
        ld sp, 8(a1)
        ret
        ",
        options(noreturn)
    }
}
