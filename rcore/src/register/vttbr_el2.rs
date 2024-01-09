//! Holds the base address of the translation table for the initial lookup for 
//! stage 2 of an address translation in the EL1&0 translation regime, and other 
//! information for this translation regime.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Virtualization Translation Table Base Register
    pub VTTBR_EL2 [
        /// Translation table base address, A[47:x] or A[51:x], bits[47:1].
        BADDR OFFSET(1) NUMBITS(48) [],

        /// Common not Private.
        CnP OFFSET(0) NUMBITS(1) [
            Private = 0b0,
            Common  = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = VTTBR_EL2::Register;
    sys_register_read_raw!(u64, "VTTBR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = VTTBR_EL2::Register;
    sys_register_write_raw!(u64, "VTTBR_EL2", "x");
}

pub const VTTBR_EL2: Reg = Reg {};
