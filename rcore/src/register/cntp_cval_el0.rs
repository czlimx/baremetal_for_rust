//! Holds the compare value for the EL2 physical timer.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Counter-timer Physical Timer CompareValue register
    pub CNTP_CVAL_EL0 [
        /// Holds the EL1 physical timer CompareValue.
        CompareValue OFFSET(0) NUMBITS(64) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CNTP_CVAL_EL0::Register;
    sys_register_read_raw!(u64, "CNTP_CVAL_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTP_CVAL_EL0::Register;
    sys_register_write_raw!(u64, "CNTP_CVAL_EL0", "x");
}

pub const CNTP_CVAL_EL0: Reg = Reg {};
