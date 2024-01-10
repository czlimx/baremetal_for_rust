//! Increments a counter that is configured to count the Software increment event, 
//! event 0x00. For more information, see SW_INCR.

use tock_registers::{
    register_bitfields, 
    interfaces::Writeable
};

register_bitfields! {u64,
    /// Performance Monitors Software Increment register
    pub PMSWINC_EL0 [
        /// Software increment.
        P OFFSET(0) NUMBITS(30) [
            Ignored     = 0b0,
            Increment   = 0b1
        ]
    ]
}

pub struct Reg;

impl Writeable for Reg {
    type T = u64;
    type R = PMSWINC_EL0::Register;
    sys_register_write_raw!(u64, "PMSWINC_EL0", "x");
}

pub const PMSWINC_EL0: Reg = Reg {};
