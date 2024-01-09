//! Holds the exception base address for any exception that is taken to EL3.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Vector Base Address Register (EL3)
    pub VBAR_EL3 [
        // Base address of the exception vectors for exceptions taken in this 
        // exception level.
        VectorBaseAddress OFFSET(11) NUMBITS(53) [],
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = VBAR_EL3::Register;
    sys_register_read_raw!(u64, "VBAR_EL3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = VBAR_EL3::Register;
    sys_register_write_raw!(u64, "VBAR_EL3", "x");
}

pub const VBAR_EL3: Reg = Reg {};
