.section .text.entry
.globl _start
.globl boot_stack
.globl boot_stack_top
_start:
	# 初始化内核栈
	la sp, boot_stack_top
	call rust_main


.section .bss.stack
boot_stack:
	.space 16 * 4096 # 16kb
boot_stack_top: