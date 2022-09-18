.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

.macro LOAD_A_REG n
    ld a\n, \n*8(sp)
.endm

.macro LOAD_T_REG n
    ld t\n, \n*8(sp)
.endm

.section .text
.global __trap_entry
.global __restore
.align 2

/* Context layout
low address
--------
|    none(top)
|    ra
|    sp
|    gp
|    none
|    x5~x31
|    sstatus
|    sepc
--------
*/

__trap_entry:
    csrrw sp, sscratch, sp
    
    addi sp, sp, -34*8
    sd ra, 1*8(sp)
    sd gp, 3*8(sp)
    # save x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    csrr t2, sscratch
    sd t2, 2*8(sp)
    mv a0, sp
    call trap_handler

__restore:
    # mv sp, a0
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    ld ra, 1*8(sp)
    ld gp, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    addi sp, sp, 34*8
    csrrw sp, sscratch, sp
    sret
