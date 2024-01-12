//! Holds the 64-bit virtual count value. The virtual count value is equal to the physical count value 
//! minus the virtual offset visible in CNTVOFF_EL2.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Counter-timer Virtual Count register
    pub CNTVCT_EL0 [
        /// Virtual count value.
        Bits OFFSET(0) NUMBITS(32) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CNTVCT_EL0::Register;
    sys_register_read_raw!(u64, "CNTVCT_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTVCT_EL0::Register;
    sys_register_write_raw!(u64, "CNTVCT_EL0", "x");
}

pub const CNTVCT_EL0: Reg = Reg {};
