//! Controls translation table walks required for stage 1 translation of memory 
//! accesses from EL3 and holds cacheability and shareability information for 
//! the accesses.
//! TCR_EL3 is part of the Virtual memory control registers functional group.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Translation Control Register (EL3)
    pub TCR_EL3 [
        /// Top Byte Ignored. Indicates whether the top byte of the input 
        /// address is used for address match.
        TBI OFFSET(20) NUMBITS(1) [
            Used        = 0b0,
            Ignored     = 0b1
        ],

        /// Physical address size.
        PS OFFSET(16) NUMBITS(3) [
            Bit32        = 0b000,
            Bit36        = 0b001,
            Bit40        = 0b010,
            Reserved     = 0b111
        ],

        /// TTBR0_EL3 granule size.
        TG0 OFFSET(14) NUMBITS(2) [
            KB4         = 0b10,
            KB64        = 0b01,
            Reserved    = 0b11
        ],

        /// Shareability attribute for memory associated with translation 
        /// table walks using TTBR0_EL3.
        SH0 OFFSET(12) NUMBITS(2) [
            NonShareable       = 0b00,
            Reserved           = 0b01,
            OuterShareable     = 0b10,
            InnerShareable     = 0b11
        ],

        /// Outer cacheability attribute for memory associated with translation
        /// table walks using TTBR0_EL3.
        ORGN0 OFFSET(10) NUMBITS(2) [
            NonCacheable                            = 0b00,
            WriteBackWriteAllocateCacheable         = 0b01,
            WriteThroughCacheable                   = 0b10,
            WriteBacknoWriteAllocateCacheable       = 0b11
        ],

        /// Inner cacheability attribute for memory associated with translation
        /// table walks using TTBR0_EL3.
        IRGN0 OFFSET(8) NUMBITS(2) [
            NonCacheable                            = 0b00,
            WriteBackWriteAllocateCacheable         = 0b01,
            WriteThroughCacheable                   = 0b10,
            WriteBacknoWriteAllocateCacheable       = 0b11
        ],

        /// Size offset of the memory region addressed by TTBR0_EL3.
        T0SZ OFFSET(0) NUMBITS(6) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = TCR_EL3::Register;
    sys_register_read_raw!(u64, "TCR_EL3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = TCR_EL3::Register;
    sys_register_write_raw!(u64, "TCR_EL3", "x");
}

pub const TCR_EL3: Reg = Reg {};
