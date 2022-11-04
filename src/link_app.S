
.align 4
.section .data
.global _num_app
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

_num_app:
    .quad 5
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_4_start
    .quad app_4_end

.align 4
app_0_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/00yield"
app_0_end:

.align 4
app_1_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/01sleep"
app_1_end:

.align 4
app_2_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/02sort"
app_2_end:

.align 4
app_3_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/03priv_inst"
app_3_end:

.align 4
app_4_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/04priv_csr"
app_4_end:
