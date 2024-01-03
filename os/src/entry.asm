    .section .text.entry
    .globl _start
_start:
    # set up a stack for rust
    # boot_stack_top is a initial top of stack
    # with a 4096-byte stack per CPU
    # sp = boot_stack_top + (hartid + 1) * 4096
    csrr    a0, mhartid
    slli    t0, a0, 12
    sub     sp, sp, t0
    # jump to rust_main in main.c
    call rust_main

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 8
    .globl boot_stack_top
boot_stack_top:
