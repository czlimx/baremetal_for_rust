//! Disables the Cycle Count Register, PMCCNTR_EL0, and any implemented event counters 
//! PMEVCNTR<n>_EL0. Reading this register shows which counters are enabled.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Performance Monitors Count Enable Clear register
    pub PMCNTENCLR_EL0 [
        /// PMCCNTR_EL0 disable.
        C OFFSET(31) NUMBITS(1) [
            Disable           = 0b0,
            Enable            = 0b1
        ],

        /// PMEVCNTR<n>_EL0 disable.
        P OFFSET(0) NUMBITS(30) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PMCNTENCLR_EL0::Register;
    sys_register_read_raw!(u64, "PMCNTENCLR_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PMCNTENCLR_EL0::Register;
    sys_register_write_raw!(u64, "PMCNTENCLR_EL0", "x");
}

pub const PMCNTENCLR_EL0: Reg = Reg {};
