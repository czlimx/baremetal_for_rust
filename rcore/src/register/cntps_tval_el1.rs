//! Holds the timer value for the secure physical timer, usually accessible at EL3 but configurably 
//! accessible at EL1 in Secure state.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Counter-timer Physical Secure Timer TimerValue register
    pub CNTPS_TVAL_EL1 [
        /// The TimerValue view of the secure physical timer.
        TimerValue OFFSET(0) NUMBITS(32) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CNTPS_TVAL_EL1::Register;
    sys_register_read_raw!(u64, "CNTPS_TVAL_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTPS_TVAL_EL1::Register;
    sys_register_write_raw!(u64, "CNTPS_TVAL_EL1", "x");
}

pub const CNTPS_TVAL_EL1: Reg = Reg {};
