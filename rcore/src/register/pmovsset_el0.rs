//! Sets the state of the overflow bit for the Cycle Count Register, PMCCNTR_EL0, and each of the 
//! implemented event counters PMEVCNTR<n>_EL0.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Performance Monitors Overflow Flag Status Set register
    pub PMOVSSET_EL0 [
        /// Unsigned overflow flag for PMCCNTR_EL0 set.
        C OFFSET(31) NUMBITS(1) [
            NotOverflowed     = 0b0,
            Overflowed        = 0b1
        ],

        /// Unsigned overflow flag for PMEVCNTR<n>_EL0 set.
        P OFFSET(0) NUMBITS(30) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PMOVSSET_EL0::Register;
    sys_register_read_raw!(u64, "PMOVSSET_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PMOVSSET_EL0::Register;
    sys_register_write_raw!(u64, "PMOVSSET_EL0", "x");
}

pub const PMOVSSET_EL0: Reg = Reg {};
