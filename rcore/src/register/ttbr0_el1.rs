//! Holds the base address of translation table 0, and information about the 
//! memory it occupies. This is one of the translation tables for the stage 1 
//! translation of memory accesses from modes other than Hyp mode.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Translation Table Base Register 0, EL1
    pub TTBR0_EL1 [
        /// An ASID for the translation table base address.
        /// The TCR_EL1.A1 field selects either TTBR0_EL1.ASID or TTBR1_EL1.ASID.
        ASID OFFSET(48) NUMBITS(16) [],

        /// Translation table base address, bits[47:x]. Bits [x-1:0] are RES0.
        /// x is based on the value of TCR_EL1.T0SZ, the stage of translation, 
        /// and the memory translation granule size.
        BADDR OFFSET(0) NUMBITS(48) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = TTBR0_EL1::Register;
    sys_register_read_raw!(u64, "TTBR0_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = TTBR0_EL1::Register;
    sys_register_write_raw!(u64, "TTBR0_EL1", "x");
}

pub const TTBR0_EL1: Reg = Reg {};
