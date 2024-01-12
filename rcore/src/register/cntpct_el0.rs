//! Holds the 64-bit physical count value.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Counter-timer Physical Count register
    pub CNTPCT_EL0 [
        /// Physical count value.
        Bits OFFSET(0) NUMBITS(32) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CNTPCT_EL0::Register;
    sys_register_read_raw!(u64, "CNTPCT_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTPCT_EL0::Register;
    sys_register_write_raw!(u64, "CNTPCT_EL0", "x");
}

pub const CNTPCT_EL0: Reg = Reg {};
