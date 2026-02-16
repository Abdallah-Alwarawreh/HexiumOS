.set ALIGN,      (1 << 0)
.set MEMINFO,    (1 << 1)
.set FLAGS,      (ALIGN | MEMINFO)
.set MAGIC,      0x1BADB002
.set CHECKSUM,   (0 - (MAGIC + FLAGS))

.section .multiboot
    .align 4
    .long MAGIC
    .long FLAGS
    .long CHECKSUM

.section .bss
    .align 16
stack_bottom:
    .skip 16384
stack_top:

.section .text
    .global _start
    .type _start, @function
_start:
    lea stack_top, %esp
    pushl $0
    popf
    mov %cr0, %eax
    andl $0xFFFFFFFB, %eax
    orl  $0x2, %eax
    mov %eax, %cr0
    call kernel_main
.hang:
    cli
    hlt
    jmp .hang

    .size _start, . - _start

.section .note.GNU-stack, "", @progbits
