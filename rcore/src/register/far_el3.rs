//! Holds the faulting Virtual Address for all synchronous instruction or data 
//! aborts, or exceptions from a misaligned PC, taken to EL3.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Fault Address Register (EL3)
    pub FAR_EL3 [
        /// The faulting Virtual Address for all synchronous instruction or 
        /// data aborts, or an exception from a misaligned PC, taken in EL3.
        VA OFFSET(0) NUMBITS(64) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = FAR_EL3::Register;
    sys_register_read_raw!(u64, "FAR_EL3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = FAR_EL3::Register;
    sys_register_write_raw!(u64, "FAR_EL3", "x");
}

pub const FAR_EL3: Reg = Reg {};
