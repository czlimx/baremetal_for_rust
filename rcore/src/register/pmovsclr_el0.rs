//! Contains the state of the overflow bit for the Cycle Count Register, PMCCNTR_EL0, and each of 
//! the implemented event counters PMEVCNTR<n>_EL0. Writing to this register clears these bits.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Performance Monitors Overflow Flag Status Clear Register
    pub PMOVSCLR_EL0 [
        /// Unsigned overflow flag for PMCCNTR_EL0 clear.
        C OFFSET(31) NUMBITS(1) [
            NotOverflowed      = 0b0,
            Overflowed         = 0b1
        ],

        /// Unsigned overflow flag for PMEVCNTR<n>_EL0 clear.
        P OFFSET(0) NUMBITS(30) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PMOVSCLR_EL0::Register;
    sys_register_read_raw!(u64, "PMOVSCLR_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PMOVSCLR_EL0::Register;
    sys_register_write_raw!(u64, "PMOVSCLR_EL0", "x");
}

pub const PMOVSCLR_EL0: Reg = Reg {};
