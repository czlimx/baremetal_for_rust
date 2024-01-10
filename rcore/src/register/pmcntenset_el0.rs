//! Enables the Cycle Count Register, PMCCNTR_EL0, and any implemented event counters 
//! PMEVCNTR<n>_EL0. Reading this register shows which counters are enabled.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Performance Monitors Count Enable Set register
    pub PMCNTENSET_EL0 [
        /// PMCCNTR_EL0 enable.
        C OFFSET(31) NUMBITS(1) [
            Disable           = 0b0,
            Enable            = 0b1
        ],

        /// PMEVCNTR<n>_EL0 enable.
        P OFFSET(0) NUMBITS(30) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PMCNTENSET_EL0::Register;
    sys_register_read_raw!(u64, "PMCNTENSET_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PMCNTENSET_EL0::Register;
    sys_register_write_raw!(u64, "PMCNTENSET_EL0", "x");
}

pub const PMCNTENSET_EL0: Reg = Reg {};
