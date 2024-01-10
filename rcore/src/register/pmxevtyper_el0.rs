//! When PMSELR_EL0.SEL selects an event counter, this accesses a PMEVTYPER<n>_EL0
//! register. When PMSELR_EL0.SEL selects the cycle counter, this accesses PMCCFILTR_EL0.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Performance Monitors Selected Event Type Register
    pub PMXEVTYPER_EL0 [
        /// When PMSELR_EL0.SEL == 31, this register accesses PMCCFILTR_EL0.
        /// Otherwise, this register accesses PMEVTYPER<n>_EL0 where n is the 
        /// value in PMSELR_EL0.SEL.
        Bits OFFSET(0) NUMBITS(64) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PMXEVTYPER_EL0::Register;
    sys_register_read_raw!(u64, "PMXEVTYPER_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PMXEVTYPER_EL0::Register;
    sys_register_write_raw!(u64, "PMXEVTYPER_EL0", "x");
}

pub const PMXEVTYPER_EL0: Reg = Reg {};
