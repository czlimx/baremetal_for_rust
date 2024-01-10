//! Defines which common architectural and common microarchitectural 
//! feature events are implemented.

use tock_registers::{
    register_bitfields, 
    interfaces::Readable
};

register_bitfields! {u64,
    /// Performance Monitor Common Event Identification Register 0
    pub PMCEID1_EL0 [
        /// L2 Data cache allocate.
        L2D_CACHE_ALLOCATE OFFSET(0) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PMCEID1_EL0::Register;
    sys_register_read_raw!(u64, "PMCEID1_EL0", "x");
}

pub const PMCEID1_EL0: Reg = Reg {};
