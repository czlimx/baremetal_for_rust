//! Holds the faulting Virtual Address for all synchronous instruction or data 
//! aborts, or exceptions from a misaligned PC or a Watchpoint debug event, 
//! taken to EL2.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Fault Address Register (EL2)
    pub FAR_EL2 [
        /// The faulting Virtual Address for all synchronous instruction or 
        /// data aborts, or an exception from a misaligned PC, taken in EL2.
        VA OFFSET(0) NUMBITS(64) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = FAR_EL2::Register;
    sys_register_read_raw!(u64, "FAR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = FAR_EL2::Register;
    sys_register_write_raw!(u64, "FAR_EL2", "x");
}

pub const FAR_EL2: Reg = Reg {};
