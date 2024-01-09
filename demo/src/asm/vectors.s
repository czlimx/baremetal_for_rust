    .global _el1_vectors
    .global _el2_vectors
    .global _el3_vectors
    .section .text.vectors, "ax", %progbits

    /* Vector tables must be placed at a 2KB-aligned address */
    .align	11
_el1_vectors:
    /* Current EL with SP0 */
    .align 7
	b	_el1_sp0_sync		            // Synchronous
	.align 7
	b	_el1_sp0_irq		            // IRQ/vIRQ
	.align 7
	b	_el1_sp0_fiq		            // FIQ/vFIQ
	.align 7
	b	_el1_sp0_serror		            // SError/vSError

    /* Current EL with SPx */
    .align 7
	b	_el1_spx_sync		            // Synchronous
	.align 7
	b	_el1_spx_irq		            // IRQ/vIRQ
	.align 7
	b	_el1_spx_fiq		            // FIQ/vFIQ
	.align 7
	b	_el1_spx_serror		            // SError/vSError

    /* Lower EL using AArch64 */
	.align 7
	b	_el1_aarch64_sync               // Synchronous
	.align 7
	b	_el1_aarch64_irq                // IRQ/vIRQ
	.align 7
	b	_el1_aarch64_fiq                // FIQ/vFIQ
	.align 7
	b	_el1_aarch64_serror             // SError/vSError

    /* Lower EL using AArch32 */
	.align 7
	b	_el1_aarch32_sync               // Synchronous
	.align 7
	b	_el1_aarch32_irq                // IRQ/vIRQ
	.align 7
	b	_el1_aarch32_fiq                // FIQ/vFIQ
	.align 7
	b	_el1_aarch32_serror             // SError/vSError

    /* Vector tables must be placed at a 2KB-aligned address */
    .align	11
_el2_vectors:
    /* Current EL with SP0 */
    .align 7
	b	_el2_sp0_sync		            // Synchronous
	.align 7
	b	_el2_sp0_irq		            // IRQ/vIRQ
	.align 7
	b	_el2_sp0_fiq		            // FIQ/vFIQ
	.align 7
	b	_el2_sp0_serror		            // SError/vSError

    /* Current EL with SPx */
    .align 7
	b	_el2_spx_sync		            // Synchronous
	.align 7
	b	_el2_spx_irq		            // IRQ/vIRQ
	.align 7
	b	_el2_spx_fiq		            // FIQ/vFIQ
	.align 7
	b	_el2_spx_serror		            // SError/vSError

    /* Lower EL using AArch64 */
	.align 7
	b	_el2_aarch64_sync               // Synchronous
	.align 7
	b	_el2_aarch64_irq                // IRQ/vIRQ
	.align 7
	b	_el2_aarch64_fiq                // FIQ/vFIQ
	.align 7
	b	_el2_aarch64_serror             // SError/vSError

    /* Lower EL using AArch32 */
	.align 7
	b	_el2_aarch32_sync               // Synchronous
	.align 7
	b	_el2_aarch32_irq                // IRQ/vIRQ
	.align 7
	b	_el2_aarch32_fiq                // FIQ/vFIQ
	.align 7
	b	_el2_aarch32_serror             // SError/vSError

    /* Vector tables must be placed at a 2KB-aligned address */
    .align	11
_el3_vectors:
    /* Current EL with SP0 */
    .align 7
	b	_el3_sp0_sync		            // Synchronous
	.align 7
	b	_el3_sp0_irq		            // IRQ/vIRQ
	.align 7
	b	_el3_sp0_fiq		            // FIQ/vFIQ
	.align 7
	b	_el3_sp0_serror		            // SError/vSError

    /* Current EL with SPx */
    .align 7
	b	_el3_spx_sync		            // Synchronous
	.align 7
	b	_el3_spx_irq		            // IRQ/vIRQ
	.align 7
	b	_el3_spx_fiq		            // FIQ/vFIQ
	.align 7
	b	_el3_spx_serror		            // SError/vSError

    /* Lower EL using AArch64 */
	.align 7
	b	_el3_aarch64_sync               // Synchronous
	.align 7
	b	_el3_aarch64_irq                // IRQ/vIRQ
	.align 7
	b	_el3_aarch64_fiq                // FIQ/vFIQ
	.align 7
	b	_el3_aarch64_serror             // SError/vSError

    /* Lower EL using AArch32 */
	.align 7
	b	_el3_aarch32_sync               // Synchronous
	.align 7
	b	_el3_aarch32_irq                // IRQ/vIRQ
	.align 7
	b	_el3_aarch32_fiq                // FIQ/vFIQ
	.align 7
	b	_el3_aarch32_serror             // SError/vSError

_el1_sp0_sync:
_el1_sp0_irq:
_el1_sp0_fiq:
_el1_sp0_serror:
_el1_spx_sync:
_el1_spx_irq:
_el1_spx_fiq:
_el1_spx_serror:
_el1_aarch64_sync:
_el1_aarch64_irq:
_el1_aarch64_fiq:
_el1_aarch64_serror:
_el1_aarch32_sync:
_el1_aarch32_irq:
_el1_aarch32_fiq:
_el1_aarch32_serror:
    b .

_el2_sp0_sync:
_el2_sp0_irq:
_el2_sp0_fiq:
_el2_sp0_serror:
_el2_spx_sync:
_el2_spx_irq:
_el2_spx_fiq:
_el2_spx_serror:
_el2_aarch64_sync:
_el2_aarch64_irq:
_el2_aarch64_fiq:
_el2_aarch64_serror:
_el2_aarch32_sync:
_el2_aarch32_irq:
_el2_aarch32_fiq:
_el2_aarch32_serror:
    b .

_el3_sp0_sync:
_el3_sp0_irq:
_el3_sp0_fiq:
_el3_sp0_serror:
_el3_spx_sync:
_el3_spx_irq:
_el3_spx_fiq:
_el3_spx_serror:
_el3_aarch64_sync:
_el3_aarch64_irq:
_el3_aarch64_fiq:
_el3_aarch64_serror:
_el3_aarch32_sync:
_el3_aarch32_irq:
_el3_aarch32_fiq:
_el3_aarch32_serror:
    b .
