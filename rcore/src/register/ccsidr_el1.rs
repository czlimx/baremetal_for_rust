//! Provides information about the architecture of the caches.

use tock_registers::{
    register_bitfields, 
    interfaces::Readable
};

register_bitfields! {u64,
    /// Cache Size ID Register
    pub CCSIDR_EL1 [
        /// Indicates support for write-through.
        WT OFFSET(31) NUMBITS(1) [
            NotSupport    = 0b0,
            Support       = 0b1
        ],

        /// Indicates support for write-back.
        WB OFFSET(30) NUMBITS(1) [
            NotSupport    = 0b0,
            Support       = 0b1
        ],

        /// Indicates support for Read-Allocation.
        RA OFFSET(29) NUMBITS(1) [
            NotSupport    = 0b0,
            Support       = 0b1
        ],

        /// Indicates support for Write-Allocation.
        WA OFFSET(28) NUMBITS(1) [
            NotSupport    = 0b0,
            Support       = 0b1
        ],

        /// Indicates the number of sets in cache - 1.
        NumSets OFFSET(13) NUMBITS(15) [],

        /// Indicates the associativity of cache - 1.
        Associativity OFFSET(3) NUMBITS(10) [],

        /// Indicates the (log2 (number of words in cache line)) - 2.
        LineSize OFFSET(0) NUMBITS(3) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CCSIDR_EL1::Register;
    sys_register_read_raw!(u64, "CCSIDR_EL1", "x");
}

pub const CCSIDR_EL1: Reg = Reg {};
