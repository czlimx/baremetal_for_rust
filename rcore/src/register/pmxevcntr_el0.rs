//! Reads or writes the value of the selected event counter, PMEVCNTR<n>_EL0. 
//! PMSELR_EL0.SEL determines which event counter is selected.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Performance Monitors Selected Event Count Register
    pub PMXEVCNTR_EL0 [
        /// Value of the selected event counter, PMEVCNTR<n>_EL0, where n
        /// is the value stored in PMSELR_EL0.SEL.
        PMEVCNTR OFFSET(0) NUMBITS(64) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PMXEVCNTR_EL0::Register;
    sys_register_read_raw!(u64, "PMXEVCNTR_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PMXEVCNTR_EL0::Register;
    sys_register_write_raw!(u64, "PMXEVCNTR_EL0", "x");
}

pub const PMXEVCNTR_EL0: Reg = Reg {};
