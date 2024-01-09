//! Provides additional IMPLEMENTATION DEFINED configuration and control 
//! options for the processor.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// CPU Extended Control Register (EL1)
    pub CPUECTLR_EL1 [
        /// Enable hardware management of data coherency with other cores in 
        /// the cluster.
        SMPEN OFFSET(6) NUMBITS(1) [
            Disable       = 0b0,
            Enable        = 0b1
        ],

        /// Advanced SIMD and Floating-point retention control.
        FPURetentionControl OFFSET(3) NUMBITS(3) [
            Disable         = 0b000,
            Ticks2          = 0b001,
            Ticks8          = 0b010,
            Ticks32         = 0b011,
            Ticks64         = 0b100,
            Ticks128        = 0b101,
            Ticks256        = 0b110,
            Ticks512        = 0b111
        ],

        /// CPU retention control.
        CPURetentionControl OFFSET(0) NUMBITS(3) [
            Disable         = 0b000,
            Ticks2          = 0b001,
            Ticks8          = 0b010,
            Ticks32         = 0b011,
            Ticks64         = 0b100,
            Ticks128        = 0b101,
            Ticks256        = 0b110,
            Ticks512        = 0b111
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CPUECTLR_EL1::Register;
    sys_register_read_raw!(u64, "S3_1_C15_C2_1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CPUECTLR_EL1::Register;
    sys_register_write_raw!(u64, "S3_1_C15_C2_1", "x");
}

pub const CPUECTLR_EL1: Reg = Reg {};
