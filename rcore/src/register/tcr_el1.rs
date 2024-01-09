//! Determines which Translation Base Registers defines the base address 
//! register for a translation table walk required for stage 1 translation of a 
//! memory access from EL0 or EL1 and holds cacheability and shareability 
//! information.
//! TCR_EL1 is part of the Virtual memory control registers functional group.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Translation Control Register (EL1)
    pub TCR_EL1 [
        /// Top Byte Ignored. Indicates whether the top byte of the input 
        /// address is used for address match for the TTBR1_EL1 region.
        TBI1 OFFSET(38) NUMBITS(1) [
            Used        = 0b0,
            Ignored     = 0b1
        ],

        /// Top Byte Ignored. Indicates whether the top byte of the input 
        /// address is used for address match for the TTBR0_EL1 region.
        TBI0 OFFSET(37) NUMBITS(1) [
            Used        = 0b0,
            Ignored     = 0b1
        ],

        /// ASID size.
        AS OFFSET(36) NUMBITS(1) [
            Bit8        = 0b0,
            Bit16       = 0b1
        ],

        /// Intermediate Physical Address Size.
        IPS OFFSET(32) NUMBITS(3) [
            Bit32        = 0b000,
            Bit36        = 0b001,
            Bit40        = 0b010,
            Reserved     = 0b011
        ],

        /// TTBR1_EL1 granule size.
        TG1 OFFSET(30) NUMBITS(2) [
            KB4         = 0b00,
            KB64        = 0b01,
            Reserved    = 0b11
        ],

        /// Shareability attribute for memory associated with translation 
        /// table walks using TTBR1_EL1.
        SH1 OFFSET(28) NUMBITS(2) [
            NonShareable       = 0b00,
            Reserved           = 0b01,
            OuterShareable     = 0b10,
            InnerShareable     = 0b11
        ],

        /// Outer cacheability attribute for memory associated with translation
        /// table walks using TTBR1_EL1.
        ORGN1 OFFSET(26) NUMBITS(2) [
            NonCacheable                            = 0b00,
            WriteBackWriteAllocateCacheable         = 0b01,
            WriteThroughCacheable                   = 0b10,
            WriteBacknoWriteAllocateCacheable       = 0b11
        ],

        /// Inner cacheability attribute for memory associated with translation
        /// table walks using TTBR1_EL1.
        IRGN1 OFFSET(24) NUMBITS(2) [
            NonCacheable                            = 0b00,
            WriteBackWriteAllocateCacheable         = 0b01,
            WriteThroughCacheable                   = 0b10,
            WriteBacknoWriteAllocateCacheable       = 0b11
        ],

        /// Translation table walk disable for translations using TTBR1_EL1.
        EPD1 OFFSET(23) NUMBITS(1) [
            Normal        = 0b0,
            Fault         = 0b1
        ],

        /// Selects whether TTBR0_EL1 or TTBR1_EL1 defines the ASID.
        A1 OFFSET(22) NUMBITS(1) [
            TTBR0_EL1_ASID       = 0b0,
            TTBR1_EL1_ASID       = 0b1
        ],

        /// Size offset of the memory region addressed by TTBR1_EL1.
        T1SZ OFFSET(16) NUMBITS(6) [],

        /// TTBR0_EL1 granule size.
        TG0 OFFSET(14) NUMBITS(2) [
            KB4         = 0b00,
            KB64        = 0b01,
            Reserved    = 0b11
        ],

        /// Shareability attribute for memory associated with translation 
        /// table walks using TTBR0_EL1.
        SH0 OFFSET(12) NUMBITS(2) [
            NonShareable       = 0b00,
            Reserved           = 0b01,
            OuterShareable     = 0b10,
            InnerShareable     = 0b11
        ],

        /// Outer cacheability attribute for memory associated with translation
        /// table walks using TTBR0_EL1.
        ORGN0 OFFSET(10) NUMBITS(2) [
            NonCacheable                            = 0b00,
            WriteBackWriteAllocateCacheable         = 0b01,
            WriteThroughCacheable                   = 0b10,
            WriteBacknoWriteAllocateCacheable       = 0b11
        ],

        /// Inner cacheability attribute for memory associated with translation
        /// table walks using TTBR0_EL1.
        IRGN0 OFFSET(8) NUMBITS(2) [
            NonCacheable                            = 0b00,
            WriteBackWriteAllocateCacheable         = 0b01,
            WriteThroughCacheable                   = 0b10,
            WriteBacknoWriteAllocateCacheable       = 0b11
        ],

        /// Translation table walk disable for translations using TTBR0_EL1.
        EPD0 OFFSET(7) NUMBITS(1) [
            Normal        = 0b0,
            Fault         = 0b1
        ],

        /// Size offset of the memory region addressed by TTBR0_EL1.
        T0SZ OFFSET(0) NUMBITS(6) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = TCR_EL1::Register;
    sys_register_read_raw!(u64, "TCR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = TCR_EL1::Register;
    sys_register_write_raw!(u64, "TCR_EL1", "x");
}

pub const TCR_EL1: Reg = Reg {};
