//! Holds the base address of the translation table for the stage 1 translation 
//! of memory accesses from EL3.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Translation Table Base Register 0 (EL3)
    pub TTBR0_EL3 [
        /// Translation table base address, bits[47:x]. Bits [x-1:0] are RES0.
        /// x is based on the value of TCR_EL3.T0SZ, the stage of translation, 
        /// and the memory translation granule size.
        BADDR OFFSET(0) NUMBITS(48) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = TTBR0_EL3::Register;
    sys_register_read_raw!(u64, "TTBR0_EL3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = TTBR0_EL3::Register;
    sys_register_write_raw!(u64, "TTBR0_EL3", "x");
}

pub const TTBR0_EL3: Reg = Reg {};
