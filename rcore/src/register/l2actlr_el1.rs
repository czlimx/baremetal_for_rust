//! Provides configuration and control options for the L2 memory system.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// L2 Auxiliary Control Register (EL1)
    pub L2ACTLR_EL1 [
        /// L2 cache data RAM error injection enable.
        L2DEIEN OFFSET(29) NUMBITS(1) [
            Disable         = 0b0,
            Enable          = 0b1
        ],

        /// L2 cache tag RAM error injection enable.
        L2TEIEN OFFSET(24) NUMBITS(1) [
            Disable         = 0b0,
            Enable          = 0b1
        ],

        /// Enables sending of WriteEvict transactions for UniqueClean evictions
        /// with data. WriteEvict transactions update downstream caches that are 
        /// outside the cluster. Enable WriteEvict transactions only if there is 
        /// an L3 or system cache implemented in the system.
        EnableUniqueClean OFFSET(14) NUMBITS(1) [
            Disable         = 0b0,
            Enable          = 0b1
        ],

        /// Disables sending of Evict transactions for clean cache lines that
        /// are evicted from the processor. This is required only if the 
        /// external interconnect contains a snoop filter that requires 
        /// notification when the processor evicts the cache line.
        DisableCleanEvict OFFSET(3) NUMBITS(1) [
            Enable         = 0b0,
            Disable        = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = L2ACTLR_EL1::Register;
    sys_register_read_raw!(u64, "S3_1_C15_C0_0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = L2ACTLR_EL1::Register;
    sys_register_write_raw!(u64, "S3_1_C15_C0_0", "x");
}

pub const L2ACTLR_EL1: Reg = Reg {};
