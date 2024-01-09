//! Holds syndrome information for an exception taken to EL3.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Exception Syndrome Register (EL2)
    pub ESR_EL3 [
        /// Exception Class.
        EC OFFSET(26) NUMBITS(6) [],

        /// Instruction Length for synchronous exceptions.
        IL OFFSET(25) NUMBITS(1) [
            Bit16      = 0b0,
            Bit32      = 0b1
        ],

        /// Syndrome valid.
        ISSValid OFFSET(24) NUMBITS(1) [
            NotValid    = 0b0,
            Valid       = 0b1
        ],

        /// Syndrome information.
        ISS OFFSET(0) NUMBITS(24) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ESR_EL3::Register;
    sys_register_read_raw!(u64, "ESR_EL3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ESR_EL3::Register;
    sys_register_write_raw!(u64, "ESR_EL3", "x");
}

pub const ESR_EL3: Reg = Reg {};
