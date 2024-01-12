//! Control register for the secure physical timer, usually accessible at EL3 but configurably accessible 
//! at EL1 in Secure state.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Counter-timer Physical Secure Timer Control register
    pub CNTPS_CTL_EL1 [
        /// The status of the timer.
        ISTATUS OFFSET(2) NUMBITS(1) [
            NotMet          = 0b0,
            Met             = 0b1
        ],

        /// Timer interrupt mask bit.
        IMASK OFFSET(1) NUMBITS(1) [
            NotMasked       = 0b0,
            Masked          = 0b1
        ],

        /// Enables the timer.
        ENABLE OFFSET(0) NUMBITS(1) [
            Disabled        = 0b0,
            Enabled         = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CNTPS_CTL_EL1::Register;
    sys_register_read_raw!(u64, "CNTPS_CTL_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTPS_CTL_EL1::Register;
    sys_register_write_raw!(u64, "CNTPS_CTL_EL1", "x");
}

pub const CNTPS_CTL_EL1: Reg = Reg {};
