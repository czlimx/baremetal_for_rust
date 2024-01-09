//! The Physical Address returned from an address translation.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Physical Address Register (EL1)
    pub PAR_EL1 [
        /// Memory attributes for the returned output address. This field uses 
        /// the same encoding as the Attr<n>fields in MAIR_EL1, MAIR_EL2, 
        /// and MAIR_EL3.
        Attr OFFSET(56) NUMBITS(8) [],

        /// Physical address. The Physical Address corresponding to the 
        /// supplied Virtual Address.
        PA OFFSET(12) NUMBITS(36) [],

        /// Non-secure. The NS attribute for a translation table entry read 
        /// from Secure state.
        NS OFFSET(9) NUMBITS(1) [
            Secure              = 0b0,
            NonSecure           = 0b1
        ],

        /// Shareability attribute for the Physical Address returned from a 
        /// translation table entry.
        SHA OFFSET(7) NUMBITS(2) [
            NonShareable        = 0b00,
            Reserved            = 0b01,
            OuterShareable      = 0b10,
            InnerShareable      = 0b11
        ],

        /// Pass/Fail bit. Indicates whether the conversion completed 
        /// successfully.
        F OFFSET(0) NUMBITS(1) [
            Pass                = 0b00,
            Fail                = 0b01
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PAR_EL1::Register;
    sys_register_read_raw!(u64, "PAR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PAR_EL1::Register;
    sys_register_write_raw!(u64, "PAR_EL1", "x");
}

pub const PAR_EL1: Reg = Reg {};
