use core::arch::asm;

use self::task::TaskContex;

pub mod app_info;
// pub mod old_manager;
// pub mod stack;
pub mod pid;
// pub mod processor;
pub mod manager;
pub mod processor;
pub mod task;
pub mod tigger;

#[naked]
pub unsafe extern "C" fn __switch(current: *mut TaskContex, next: *mut TaskContex) {
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

