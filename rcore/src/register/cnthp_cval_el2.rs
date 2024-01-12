//! Holds the compare value for the EL2 physical timer.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Counter-timer Physical Timer CompareValue register (EL2)
    pub CNTHP_CVAL_EL2 [
        /// Holds the EL2 physical timer CompareValue.
        CompareValue OFFSET(0) NUMBITS(64) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CNTHP_CVAL_EL2::Register;
    sys_register_read_raw!(u64, "CNTHP_CVAL_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTHP_CVAL_EL2::Register;
    sys_register_write_raw!(u64, "CNTHP_CVAL_EL2", "x");
}

pub const CNTHP_CVAL_EL2: Reg = Reg {};
