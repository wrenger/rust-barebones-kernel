.section .text.boot
.globl _start
_start:
    ldr     x30, =init_stack
    mov     sp, x30
    bl      kmain

.section .bss
    .align 8
	.space 0x4000
init_stack:
