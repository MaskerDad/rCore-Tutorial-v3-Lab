    .section .text.entry
    .globl _start
    .equ NCPU, 8
_start:
    # set up a stack for rust
    # boot_stack_top is a initial top of stack
    # with a 4096-byte stack per CPU
    # sp = boot_stack_top + (hartid * 4096)
    la sp, boot_stack_top
    li a0, 1024*4
    csrr a1, mhartid
    addi a1, a1, 1
    mul a0, a0, a1
    add sp, sp, a0
    # jump to rust_main in main.c
    call rust_main

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * NCPU
    .globl boot_stack_top
boot_stack_top:
