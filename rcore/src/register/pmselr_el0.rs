//! Selects the current event counter PMEVCNTR<n>_EL1 or the cycle counter PMCCNTR.
//! Used in conjunction with PMXEVTYPER_EL0 to determine the event that increments a selected 
//! counter, and the modes and states in which the selected counter increments.
//! Used in conjunction with PMXEVCNTR_EL0 to determine the value of a selected counter.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Performance Monitors Event Counter Selection Register
    pub PMSELR_EL0 [
        /// Event counter select. Selects the counter accessed by subsequent 
        /// accesses to PMXEVTYPER_EL0 and PMXEVCNTR_EL0.
        SEL OFFSET(0) NUMBITS(5) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PMSELR_EL0::Register;
    sys_register_read_raw!(u64, "PMSELR_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PMSELR_EL0::Register;
    sys_register_write_raw!(u64, "PMSELR_EL0", "x");
}

pub const PMSELR_EL0: Reg = Reg {};
