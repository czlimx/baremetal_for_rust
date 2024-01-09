//! Indicates the block size written with byte values of zero by the DC ZVA
//! (Cache Zero by Address), system instruction.

use tock_registers::{
    register_bitfields, 
    interfaces::Readable
};

register_bitfields! {u64,
    /// Cache Size Selection Register
    pub DCZID_EL0 [
        /// Prohibit the DC ZVA instruction.
        DZP OFFSET(4) NUMBITS(1) [
            Permitted       = 0b0,
            Prohibited      = 0b1
        ],

        /// Log2 of the block size in words.
        BlockSize OFFSET(0) NUMBITS(4) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = DCZID_EL0::Register;
    sys_register_read_raw!(u64, "DCZID_EL0", "x");
}

pub const DCZID_EL0: Reg = Reg {};
