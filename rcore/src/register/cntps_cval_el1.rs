//! Holds the compare value for the secure physical timer, usually accessible at EL3 but configurably 
//! accessible at EL1 in Secure state.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Counter-timer Physical Secure Timer CompareValue register
    pub CNTPS_CVAL_EL1 [
        /// Holds the secure physical timer CompareValue.
        CompareValue OFFSET(0) NUMBITS(64) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CNTPS_CVAL_EL1::Register;
    sys_register_read_raw!(u64, "CNTPS_CVAL_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTPS_CVAL_EL1::Register;
    sys_register_write_raw!(u64, "CNTPS_CVAL_EL1", "x");
}

pub const CNTPS_CVAL_EL1: Reg = Reg {};
