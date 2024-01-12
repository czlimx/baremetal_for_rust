//! Control register for the virtual timer.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Counter-timer Virtual Timer Control register
    pub CNTV_CTL_EL0 [
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
    type R = CNTV_CTL_EL0::Register;
    sys_register_read_raw!(u64, "CNTV_CTL_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTV_CTL_EL0::Register;
    sys_register_write_raw!(u64, "CNTV_CTL_EL0", "x");
}

pub const CNTV_CTL_EL0: Reg = Reg {};
