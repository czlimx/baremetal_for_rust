//! Disables the generation of interrupt requests on overflows from the cycle counter, 
//! PMCCNTR_EL0, and the event counters PMEVCNTR<n>_EL0. Reading the register shows which 
//! overflow interrupt requests are enabled.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Performance Monitors Interrupt Enable Clear register
    pub PMINTENCLR_EL1 [
        /// Interrupt request on unsigned overflow of PMCCNTR_EL0 disable.
        C OFFSET(31) NUMBITS(1) [
            Disable     = 0b0,
            Enable      = 0b1
        ],

        /// Interrupt request on unsigned overflow of PMEVCNTR<n>_EL0 disable.
        P OFFSET(0) NUMBITS(30) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PMINTENCLR_EL1::Register;
    sys_register_read_raw!(u64, "PMINTENCLR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PMINTENCLR_EL1::Register;
    sys_register_write_raw!(u64, "PMINTENCLR_EL1", "x");
}

pub const PMINTENCLR_EL1: Reg = Reg {};
