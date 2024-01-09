//! Holds the exception base address for any exception that is taken to EL1.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Vector Base Address Register (EL1)
    pub VBAR_EL1 [
        // Base address of the exception vectors for exceptions taken in this 
        // exception level.
        VectorBaseAddress OFFSET(11) NUMBITS(53) [],
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = VBAR_EL1::Register;
    sys_register_read_raw!(u64, "VBAR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = VBAR_EL1::Register;
    sys_register_write_raw!(u64, "VBAR_EL1", "x");
}

pub const VBAR_EL1: Reg = Reg {};
