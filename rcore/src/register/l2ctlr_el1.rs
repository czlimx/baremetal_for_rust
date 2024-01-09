//! Provides information about the IMPLEMENTATION DEFINED configuration 
//! options of the L2 memory system.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// L2 Control Register
    pub L2CTLR_EL1 [
        /// Number of cores present.
        NumberOfCores OFFSET(24) NUMBITS(2) [
            One         = 0b00,
            Two         = 0b01,
            Three       = 0b10,
            Four        = 0b11
        ],

        /// CPU Cache Protection.
        CPUCacheProtection OFFSET(22) NUMBITS(1) [
            WithoutECC  = 0b0,
            WithECC     = 0b1
        ],

        /// CPU Cache Protection.
        SCUL2CacheProtection OFFSET(21) NUMBITS(1) [
            WithoutECC  = 0b0,
            WithECC     = 0b1
        ],

        /// CPU Cache Protection.
        L2DataRAMInputLatency OFFSET(5) NUMBITS(1) [
            Cycle1     = 0b0,
            Cycle2     = 0b1
        ],

        /// CPU Cache Protection.
        L2DataRAMOutputLatency OFFSET(0) NUMBITS(1) [
            Cycle2     = 0b0,
            Cycle3     = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = L2CTLR_EL1::Register;
    sys_register_read_raw!(u64, "S3_1_C11_C0_2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = L2CTLR_EL1::Register;
    sys_register_write_raw!(u64, "S3_1_C11_C0_2", "x");
}

pub const L2CTLR_EL1: Reg = Reg {};
