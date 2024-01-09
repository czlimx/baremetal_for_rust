//! Allows access to the AArch32 IFSR register from AArch64 state only. Its 
//! value has no effect on execution in AArch64 state.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Instruction Fault Status Register (EL2)
    pub IFSR32_EL2 [
        /// External abort type.
        ExT OFFSET(12) NUMBITS(1) [
            DECERR   = 0b0,
            SLVERR   = 0b1
        ],

        /// Part of the Fault Status field.
        FS4 OFFSET(10) NUMBITS(1) [],

        /// Fault Status bits.
        FS OFFSET(0) NUMBITS(5) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = IFSR32_EL2::Register;
    sys_register_read_raw!(u64, "IFSR32_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = IFSR32_EL2::Register;
    sys_register_write_raw!(u64, "IFSR32_EL2", "x");
}

pub const IFSR32_EL2: Reg = Reg {};
