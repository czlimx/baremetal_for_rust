//! Control register for the EL2 physical timer.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Counter-timer Hypervisor Physical Timer Control register
    pub CNTHP_CTL_EL2 [
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
    type R = CNTHP_CTL_EL2::Register;
    sys_register_read_raw!(u64, "CNTHP_CTL_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTHP_CTL_EL2::Register;
    sys_register_write_raw!(u64, "CNTHP_CTL_EL2", "x");
}

pub const CNTHP_CTL_EL2: Reg = Reg {};
