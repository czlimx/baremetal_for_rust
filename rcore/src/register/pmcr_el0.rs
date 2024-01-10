//! Provides details of the Performance Monitors implementation, including 
//! the number of counters implemented, and configures and controls the 
//! counters.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Performance Monitors Control Register
    pub PMCR_EL0 [
        /// Implementer code.
        IMP OFFSET(24) NUMBITS(8) [],

        /// Identification code.
        IDCODE OFFSET(16) NUMBITS(8) [],

        /// Number of event counters.
        N OFFSET(11) NUMBITS(5) [],

        /// Long cycle count enable. Determines which PMCCNTR_EL0 bit 
        /// generates an overflow recorded in PMOVSR[31].
        LC OFFSET(6) NUMBITS(1) [
            Overflow31        = 0b0,
            Overflow63        = 0b1
        ],

        /// Disable cycle counter, PMCCNTR_EL0 when event counting is prohibited.
        DP OFFSET(5) NUMBITS(1) [
            Enable            = 0b0,
            Disable           = 0b1
        ],

        /// Export enable.
        X OFFSET(4) NUMBITS(1) [
            Disable           = 0b0,
            Enable            = 0b1
        ],

        /// Clock divider.
        D OFFSET(3) NUMBITS(1) [
            Cycle1           = 0b0,
            Cycle64          = 0b1
        ],

        /// Clock counter reset.
        C OFFSET(2) NUMBITS(1) [
            NoAction         = 0b0,
            Reset            = 0b1
        ],

        /// Event counter reset.
        P OFFSET(1) NUMBITS(1) [
            NoAction         = 0b0,
            Reset            = 0b1
        ],

        /// Enable.
        E OFFSET(0) NUMBITS(1) [
            Disable           = 0b0,
            Enable            = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PMCR_EL0::Register;
    sys_register_read_raw!(u64, "PMCR_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PMCR_EL0::Register;
    sys_register_write_raw!(u64, "PMCR_EL0", "x");
}

pub const PMCR_EL0: Reg = Reg {};
