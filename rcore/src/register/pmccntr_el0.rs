//! Holds the value of the processor Cycle Counter, CCNT, that counts processor clock cycles. See Time 
//! as measured by the Performance Monitors cycle counter on page D12-5998 for more information.
//! PMCCFILTR_EL0 determines the modes and states in which the PMCCNTR_EL0 can increment.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Performance Monitors Cycle Count Register
    pub PMCCNTR_EL0 [
        /// Cycle count. Depending on the values of PMCR_EL0.{LC,D}, 
        /// this field increments in one of the following ways.
        CCNT OFFSET(0) NUMBITS(64) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PMCCNTR_EL0::Register;
    sys_register_read_raw!(u64, "PMCCNTR_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PMCCNTR_EL0::Register;
    sys_register_write_raw!(u64, "PMCCNTR_EL0", "x");
}

pub const PMCCNTR_EL0: Reg = Reg {};
