.globl _start

.section ".text.boot"

_start:
    ldr     x30, =init_stack
    mov     sp, x30
    bl      kmain

.equ PSCI_SYSTEM_OFF, 0x84000008
.globl system_off
system_off:
    ldr     x0, =PSCI_SYSTEM_OFF
    hvc     #0

.section .bss
	.space 0x1000*4
init_stack:
