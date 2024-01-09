//! Allows access to the AArch32 register SDER only from AArch64 state. 
//! Its value has no effect on execution in AArch64 state.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Secure Debug Enable Register
    pub SDER32_EL3 [
        /// Secure User Non-invasive Debug Enable Secure EL0.
        SUNIDEN OFFSET(1) NUMBITS(1) [
            NotPermitted = 0b0,
            Permitted    = 0b1
        ],

        /// Secure User Invasive Debug Enable Secure EL0.
        SUIDEN OFFSET(0) NUMBITS(1) [
            NotPermitted = 0b0,
            Permitted    = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = SDER32_EL3::Register;
    sys_register_read_raw!(u64, "SDER32_EL3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = SDER32_EL3::Register;
    sys_register_write_raw!(u64, "SDER32_EL3", "x");
}

pub const SDER32_EL3: Reg = Reg {};
