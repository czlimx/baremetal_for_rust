//! Holds the timer value for the EL1 virtual timer.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Counter-timer Virtual Timer TimerValue register
    pub CNTV_TVAL_EL0 [
        /// The TimerValue view of the EL1 virtual timer.
        TimerValue OFFSET(0) NUMBITS(32) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CNTV_TVAL_EL0::Register;
    sys_register_read_raw!(u64, "CNTV_TVAL_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTV_TVAL_EL0::Register;
    sys_register_write_raw!(u64, "CNTV_TVAL_EL0", "x");
}

pub const CNTV_TVAL_EL0: Reg = Reg {};
