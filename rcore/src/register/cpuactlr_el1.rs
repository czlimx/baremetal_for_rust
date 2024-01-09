//! Provides IMPLEMENTATION DEFINED configuration and control options for 
//! the processor. There is one 64-bit CPU Auxiliary Control Register for each 
//! core in the cluster.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// CPU Auxiliary Control Register (EL1)
    pub CPUACTLR_EL1 [
        // Enable data cache clean as data cache clean/invalidate.
        ENDCCASCI OFFSET(44) NUMBITS(1) [
            Disable         = 0b0,
            Enable          = 0b1
        ],

        // Disable floating-point dual issue.
        FPDIDIS OFFSET(30) NUMBITS(1) [
            Enable         = 0b0,
            Disable        = 0b1
        ],

        // Disable Dual Issue.
        DIDIS OFFSET(29) NUMBITS(1) [
            Enable         = 0b0,
            Disable        = 0b1
        ],

        // Write streaming no-allocate threshold.
        RADIS OFFSET(27) NUMBITS(2) [
            CacheLine16          = 0b00,
            CacheLine128         = 0b01,         
            CacheLine512         = 0b10,
            DisablesStreaming    = 0b11
        ],

        // Write streaming no-L1-allocate threshold.
        L1RADIS OFFSET(25) NUMBITS(2) [
            CacheLine4           = 0b00,
            CacheLine64          = 0b01,         
            CacheLine128         = 0b10,
            DisablesStreaming    = 0b11
        ],

        // Disable transient and no-read-allocate hints for loads.
        DTAH OFFSET(24) NUMBITS(1) [
            Disable         = 0b0,
            Enable          = 0b1
        ],

        // Disable ReadUnique request for prefetch streams initiated by 
        // STB accesses.
        STBPFRS OFFSET(23) NUMBITS(1) [
            ReadUnique      = 0b0,
            ReadShared      = 0b1
        ],

        // Disable prefetch streams initiated from STB accesses.
        STBPFDIS OFFSET(22) NUMBITS(1) [
            Enable          = 0b0,
            Disable         = 0b1
        ],

        // IFU fetch throttle disabled.
        IFUTHDIS OFFSET(21) NUMBITS(1) [
            Enable          = 0b0,
            Disable         = 0b1
        ],

        // Number of independent data prefetch streams.
        NPFSTRM OFFSET(19) NUMBITS(2) [
            Stream1         = 0b00,
            Stream2         = 0b01,
            Stream3         = 0b10,
            Stream4         = 0b11
        ],

        // Enable device split throttle.
        DSTDIS OFFSET(18) NUMBITS(1) [
            Disable         = 0b0,
            Enable          = 0b1
        ],

        // Configure the sequence length that triggers data prefetch streams.
        STRIDE OFFSET(17) NUMBITS(1) [
            linefills2      = 0b0,
            linefills3      = 0b1
        ],

        // L1 Data prefetch control.
        L1PCTL OFFSET(13) NUMBITS(3) [
            PrefetchDisabled            = 0b000,
            OutstandingPrefetch1        = 0b001,
            OutstandingPrefetch2        = 0b010,
            OutstandingPrefetch3        = 0b011,
            OutstandingPrefetch4        = 0b100,
            OutstandingPrefetch5        = 0b101,
            OutstandingPrefetch6        = 0b110,
            OutstandingPrefetch8        = 0b111
        ],

        // Disable optimized Data Memory Barrier behavior.
        DODMBS OFFSET(10) NUMBITS(1) [
            Enable          = 0b0,
            Disable         = 0b1
        ],

        // L1 D-cache data RAM error injection enable.
        L1DEIEN OFFSET(6) NUMBITS(1) [
            Enable          = 0b0,
            Disable         = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CPUACTLR_EL1::Register;
    sys_register_read_raw!(u64, "S3_1_C15_C2_0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CPUACTLR_EL1::Register;
    sys_register_write_raw!(u64, "S3_1_C15_C2_0", "x");
}

pub const CPUACTLR_EL1: Reg = Reg {};
