.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

.macro SAVE_A_REG n s
    sd a\n, \s*8(sp)
.endm

.macro SAVE_T_REG n s
    sd t\n, \s*8(sp)
.endm

.macro LOAD_A_REG n s
    ld a\n, \s*8(sp)
.endm

.macro LOAD_T_REG n s
    ld t\n, \s*8(sp)
.endm

.section .text
.global __trap_entry
.global __restore
.align 2

/* Context layout
low address
--------
0   none(top)
    ra
    # sp
    gp
    # tp
    t0~t2
    a0~a7
    t3~t6
    sstatus
25  sepc
--------
*/

__trap_entry:
    csrrw sp, sscratch, sp
    
    addi sp, sp, -26*8
    sd ra, 1*8(sp)
    sd gp, 2*8(sp)
    # save x5~x31
    .set n, 0
    .set s, 3
    .rept 3
        SAVE_T_REG %n %s
        .set n, n+1
        .set s, s+1
    .endr
    .set n, 0
    .rept 8
        SAVE_A_REG %n %s
        .set n, n+1
        .set s, s+1
    .endr
    .set n, 3
    .rept 4
        SAVE_T_REG %n %s
        .set n, n+1
        .set s, s+1
    .endr
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, s*8(sp)
    sd t1, (s+1)*8(sp)
    # csrr t2, sscratch
    # sd t2, 2*8(sp)
    mv a0, sp
    call trap_handler

__restore:
    mv sp, a0
    ld t0, 24*8(sp)
    ld t1, 25*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    ld ra, 1*8(sp)
    ld gp, 2*8(sp)
    # save x5~x31
    .set n, 0
    .set s, 3
    .rept 3
        LOAD_T_REG %n %s
        .set n, n+1
        .set s, s+1
    .endr
    .set n, 0
    .rept 8
        LOAD_A_REG %n %s
        .set n, n+1
        .set s, s+1
    .endr
    .set n, 3
    .rept 4
        LOAD_T_REG %n %s
        .set n, n+1
        .set s, s+1
    .endr
    addi sp, sp, 26*8
    csrrw sp, sscratch, sp
    sret
