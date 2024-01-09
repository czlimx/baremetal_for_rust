//! Provides additional IMPLEMENTATION DEFINED control options for the L2 
//! memory system. This register is used for dynamically changing, but 
//! implementation specific, control bits.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// L2 Extended Control Register
    pub L2ECTLR_EL1 [
        /// L2 internal asynchronous error caused by L2 RAM double-bit ECC error.
        L2InternalAsynchronousError OFFSET(30) NUMBITS(1) [
            NoOccurred      = 0b0,
            Occurred        = 0b1
        ],

        /// AXI or CHI asynchronous error indication.
        AXIorCHIAsynchronousError OFFSET(29) NUMBITS(1) [
            NoOccurred      = 0b0,
            Occurred        = 0b1
        ],

        /// L2 dynamic retention control.
        L2DynamicRetentionControl OFFSET(0) NUMBITS(3) [
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
    type R = L2ECTLR_EL1::Register;
    sys_register_read_raw!(u64, "S3_1_C11_C0_3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = L2ECTLR_EL1::Register;
    sys_register_write_raw!(u64, "S3_1_C11_C0_3", "x");
}

pub const L2ECTLR_EL1: Reg = Reg {};
