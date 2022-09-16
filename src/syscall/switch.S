
.altmacro
.macro SAVE_S_REG n
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_S_REG n
    ld s\n, (\n+2)*8(a1)
.endm

.global __switch

.section .text
__switch:
    # Store current task context
    sd ra, 0*8(a0)
    sd sp, 1*8(a0)
    .set n, 0
    .rept 12
        SAVE_S_REG %n
        .set n, n + 1
    .endr

    # Restore next task context
    ld ra, 0*8(a1)
    ld sp, 1*8(a1)
    .set n, 0
    .rept 12
        LOAD_S_REG %n
        .set n, n + 1
    .endr

    ret