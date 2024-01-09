//! Provides the memory attribute encodings corresponding to the possible 
//! AttrIndx values in a Long-descriptor format translation table entry for 
//! stage 1 translations at EL1.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Memory Attribute Indirection Register (EL1)
    pub MAIR_EL1 [
        Attr7 OFFSET(56) NUMBITS(8) [],
        Attr6 OFFSET(48) NUMBITS(8) [],
        Attr5 OFFSET(40) NUMBITS(8) [],
        Attr4 OFFSET(32) NUMBITS(8) [],
        Attr3 OFFSET(24) NUMBITS(8) [],
        Attr2 OFFSET(16) NUMBITS(8) [],
        Attr1 OFFSET(8)  NUMBITS(8) [],
        Attr0 OFFSET(0)  NUMBITS(8) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = MAIR_EL1::Register;
    sys_register_read_raw!(u64, "MAIR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = MAIR_EL1::Register;
    sys_register_write_raw!(u64, "MAIR_EL1", "x");
}

pub const MAIR_EL1: Reg = Reg {};
