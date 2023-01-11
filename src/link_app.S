
.align 3
.section .data
.global _app_num
.global _app_names
.global app_0_start
.global app_0_end
.global app_1_start
.global app_1_end
.global app_2_start
.global app_2_end
.global app_3_start
.global app_3_end
.global app_4_start
.global app_4_end
.global app_5_start
.global app_5_end
.global app_6_start
.global app_6_end

_app_num:
    .quad 7
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_4_start
    .quad app_5_start
    .quad app_6_start
    .quad app_6_end

_app_names:
    .string "initproc"
    .string "priv_csr"
    .string "priv_inst"
    .string "sleep"
    .string "sort"
    .string "user_shell"
    .string "yield"

.align 3
app_0_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/initproc"
app_0_end:

.align 3
app_1_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/priv_csr"
app_1_end:

.align 3
app_2_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/priv_inst"
app_2_end:

.align 3
app_3_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/sleep"
app_3_end:

.align 3
app_4_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/sort"
app_4_end:

.align 3
app_5_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/user_shell"
app_5_end:

.align 3
app_6_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/yield"
app_6_end:
