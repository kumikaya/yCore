
.altmacro
.macro SAVE_S_REG n
    sd s\n, (\n+1)*8(sp)
.endm
.macro LOAD_S_REG n
    ld s\n, (\n+1)*8(sp)
.endm

.global __switch
.section .text
__switch:
    # Store current task context
    addi sp, sp, -14*8
    sd sp, (a0)
    sd ra, 0*8(sp)
    .set n, 0
    .rept 12
        SAVE_S_REG %n
        .set n, n + 1
    .endr
    # Restore next task context
    mv sp, a1
    ld ra, 0*8(sp)
    # ld sp, 1*8(a1)
    .set n, 0
    .rept 12
        LOAD_S_REG %n
        .set n, n + 1
    .endr
    addi sp, sp, 14*8
    # mv a0, sp
    ret