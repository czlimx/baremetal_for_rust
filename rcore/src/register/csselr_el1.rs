//! Selects the current Cache Size ID Register on page 4-172, by specifying:
//! • The required cache level.
//! • The cache type, either instruction or data cache.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Cache Size Selection Register
    pub CSSELR_EL1 [
        /// Cache level of required cache.
        Level OFFSET(1) NUMBITS(3) [],

        /// Instruction not Data bit.
        InD OFFSET(0) NUMBITS(1) [
            Data          = 0b0,
            Instruction   = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CSSELR_EL1::Register;
    sys_register_read_raw!(u64, "CSSELR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CSSELR_EL1::Register;
    sys_register_write_raw!(u64, "CSSELR_EL1", "x");
}

pub const CSSELR_EL1: Reg = Reg {};
