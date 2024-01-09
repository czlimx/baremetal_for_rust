//! Shows whether an IRQ, FIQ, or external abort is pending. An indicated 
//! pending abort might be a physical abort or a virtual abort.

use tock_registers::{
    register_bitfields, 
    interfaces::Readable
};

register_bitfields! {u64,
    /// Interrupt Status Register
    pub ISR_EL1 [
        // External abort pending bit.
        A OFFSET(8) NUMBITS(1) [
            NoPending     = 0b0,
            Pending       = 0b1
        ],

        // IRQ pending bit.
        I OFFSET(7) NUMBITS(1) [
            NoPending     = 0b0,
            Pending       = 0b1
        ],

        // FIQ pending bit.
        F OFFSET(6) NUMBITS(1) [
            NoPending     = 0b0,
            Pending       = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ISR_EL1::Register;
    sys_register_read_raw!(u64, "ISR_EL1", "x");
}

pub const ISR_EL1: Reg = Reg {};
