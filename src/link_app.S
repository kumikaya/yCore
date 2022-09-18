
.align 3
.section .data
.global _num_app
.global app_0_start
.global app_0_end
.global app_1_start
.global app_1_end
.global app_2_start
.global app_2_end

_num_app:
    .quad 3
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_2_end

app_0_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/00yield.bin"
app_0_end:

app_1_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/01yield.bin"
app_1_end:

app_2_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/02yield.bin"
app_2_end:
