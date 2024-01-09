//! Identifies:
//! • The type of cache, or caches, implemented at each level.
//! • The Level of Coherency and Level of Unification for the cache 
//! hierarchy.

use tock_registers::{
    register_bitfields, 
    interfaces::Readable
};

register_bitfields! {u64,
    /// Cache Level ID Register
    pub CLIDR_EL1 [
        /// Inner cache boundary.
        ICB OFFSET(30) NUMBITS(3) [],

        /// LoUU Indicates the Level of Unification Uniprocessor for the cache 
        /// hierarchy.
        LoUU OFFSET(27) NUMBITS(3) [],

        /// Indicates the Level of Coherency for the cache hierarchy.
        LoC OFFSET(24) NUMBITS(3) [
            L2NotImplement    = 0b001,
            L2Implement       = 0b010
        ],

        /// Indicates the Level of Unification Inner Shareable for the cache 
        /// hierarchy.
        LoUIS OFFSET(21) NUMBITS(3) [],

        /// Indicates the type of cache if the processor implements L3 cache.
        Ctype3 OFFSET(6) NUMBITS(3) [],

        /// Indicates the type of cache if the processor implements L2 cache.
        Ctype2 OFFSET(3) NUMBITS(3) [],

        /// Indicates the type of cache implemented at L1.
        Ctype1 OFFSET(0) NUMBITS(3) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CLIDR_EL1::Register;
    sys_register_read_raw!(u64, "CLIDR_EL1", "x");
}

pub const CLIDR_EL1: Reg = Reg {};
