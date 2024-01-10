//! Enables or disables EL0 access to the Performance Monitors.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Performance Monitors User Enable Register
    pub PMUSERENR_EL0 [
        /// Event counters Read enable.
        ER OFFSET(3) NUMBITS(1) [
            Disable     = 0b0,
            Enable      = 0b1
        ],

        /// Cycle counter Read enable.
        CR OFFSET(2) NUMBITS(1) [
            Disable     = 0b0,
            Enable      = 0b1
        ],

        /// Software increment register Write enable
        SW OFFSET(1) NUMBITS(1) [
            Disable     = 0b0,
            Enable      = 0b1
        ],

        /// Enables EL0 read/write access to PMU registers.
        EN OFFSET(0) NUMBITS(1) [
            Disable     = 0b0,
            Enable      = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PMUSERENR_EL0::Register;
    sys_register_read_raw!(u64, "PMUSERENR_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PMUSERENR_EL0::Register;
    sys_register_write_raw!(u64, "PMUSERENR_EL0", "x");
}

pub const PMUSERENR_EL0: Reg = Reg {};
