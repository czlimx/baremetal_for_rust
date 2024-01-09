    .global _start
    .extern _early_init
    .extern _el1_vectors
    .extern _el2_vectors
    .extern _el3_vectors
    .extern _core0_el3_stack_top

    .section .text.startup, "ax"
    .balign 4

_start:
    /* Setup the vector base address */
    ldr x0, = _el1_vectors
    msr VBAR_EL1, x0

    ldr x0, = _el2_vectors
    msr VBAR_EL2, x0

    ldr x0, = _el3_vectors
    msr VBAR_EL3, x0

    /* Setup the Stack for EL3, stack size 4K */
    mrs x0, MPIDR_EL1
    and x0, x0, #0xFF
    ldr x1, = _core0_el3_stack_top
    add x1, x1, x0, lsl #12
    mov sp, x1

    /* Disable MMU */
    mrs x0, SCTLR_EL3
    bic x0, x0, #(1 << 0)
    msr SCTLR_EL3, x0

    /* Disable Cache */
    mrs x0, SCTLR_EL3
    bic x0, x0, #(1 << 2)
    bic x0, x0, #(1 << 12)
    msr SCTLR_EL3, x0

    /* Synchronous completion of the above instructions */
    isb

    /* Goto early init for high level language */
    bl _early_init
