//! Holds the physical base address of the memory-mapped GIC CPU 
//! interface registers.

use tock_registers::{
    register_bitfields, 
    interfaces::Readable
};

register_bitfields! {u64,
    /// Configuration Base Address Register (EL1)
    pub CBAR_EL1 [
        /// The input PERIPHBASE[39:18] determines the reset value.
        PERIPHBASE OFFSET(18) NUMBITS(22) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CBAR_EL1::Register;
    sys_register_read_raw!(u64, "S3_1_C15_C3_0", "x");
}

pub const CBAR_EL1: Reg = Reg {};
