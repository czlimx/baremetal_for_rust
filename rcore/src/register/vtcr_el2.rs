//! Controls the translation table walks required for the stage 2 translation of 
//! memory accesses from Non-secure EL0 and EL1, and holds cacheability 
//! and shareability information for the accesses.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Virtualization Translation Control Register (EL2)
    pub VTCR_EL2 [
        /// Physical address size.
        PS OFFSET(16) NUMBITS(3) [
            Bit32        = 0b000,
            Bit36        = 0b001,
            Bit40        = 0b010,
            Reserved     = 0b111
        ],

        /// VTTBR_EL2 granule size.
        TG0 OFFSET(14) NUMBITS(2) [
            KB4         = 0b00,
            KB64        = 0b01,
            Reserved    = 0b11
        ],

        /// Shareability attribute for memory associated with translation 
        /// table walks using VTTBR_EL2.
        SH0 OFFSET(12) NUMBITS(2) [
            NonShareable       = 0b00,
            Reserved           = 0b01,
            OuterShareable     = 0b10,
            InnerShareable     = 0b11
        ],

        /// Outer cacheability attribute for memory associated with translation
        /// table walks using VTTBR_EL2.
        ORGN0 OFFSET(10) NUMBITS(2) [
            NonCacheable                            = 0b00,
            WriteBackWriteAllocateCacheable         = 0b01,
            WriteThroughCacheable                   = 0b10,
            WriteBacknoWriteAllocateCacheable       = 0b11
        ],

        /// Inner cacheability attribute for memory associated with translation
        /// table walks using VTTBR_EL2.
        IRGN0 OFFSET(8) NUMBITS(2) [
            NonCacheable                            = 0b00,
            WriteBackWriteAllocateCacheable         = 0b01,
            WriteThroughCacheable                   = 0b10,
            WriteBacknoWriteAllocateCacheable       = 0b11
        ],

        /// Starting level of the VTCR_EL2 addressed region.
        SL0 OFFSET(6) NUMBITS(2) [],

        /// Size offset of the memory region addressed by VTTBR_EL2.
        T0SZ OFFSET(0) NUMBITS(6) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = VTCR_EL2::Register;
    sys_register_read_raw!(u64, "VTCR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = VTCR_EL2::Register;
    sys_register_write_raw!(u64, "VTCR_EL2", "x");
}

pub const VTCR_EL2: Reg = Reg {};
