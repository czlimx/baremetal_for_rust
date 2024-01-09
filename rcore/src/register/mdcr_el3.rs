//! Provides configuration options for Security to self-hosted debug.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Monitor Debug Configuration Register (EL3)
    pub MDCR_EL3 [
        /// External debugger access to Performance Monitors registers disabled.
        EPMAD OFFSET(21) NUMBITS(1) [
            Permitted   = 0b0,
            NoPermitted = 0b1
        ],

        /// External debugger access to breakpoint and watchpoint registers 
        /// disabled.
        EDAD OFFSET(20) NUMBITS(1) [
            Permitted   = 0b0,
            NoPermitted = 0b1
        ],

        /// Secure performance monitors enable.
        SPME OFFSET(17) NUMBITS(1) [
            NoPermitted = 0b0,
            Permitted   = 0b1
        ],

        /// AArch64 secure debug disable.
        SDD OFFSET(16) NUMBITS(1) [
            Enable      = 0b0,
            Disable     = 0b1
        ],

        /// AArch32 secure privileged debug. Enables or disables debug 
        /// exceptions from Secure state if Secure EL1 is using AArch32, 
        /// other than Software breakpoint instructions.
        SPD32 OFFSET(14) NUMBITS(2) [
            LegacyMode  = 0b00,
            Reserved    = 0b01,
            Disable     = 0b10,
            Enable      = 0b11
        ],

        /// Trap accesses to the OS debug system registers, OSLAR_EL1, 
        /// OSLSR_EL1, OSDLR_EL1, and DBGPRCR_EL1 OS.
        TDOSA OFFSET(10) NUMBITS(1) [
            NonTrapped  = 0b0,
            Trapped     = 0b1
        ],

        /// Trap accesses to the remaining sets of debug registers to EL3.
        TDA OFFSET(9) NUMBITS(1) [
            NonTrapped  = 0b0,
            Trapped     = 0b1
        ],

        /// Trap Performance Monitors accesses.
        TPM OFFSET(6) NUMBITS(1) [
            NonTrapped  = 0b0,
            Trapped     = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = MDCR_EL3::Register;
    sys_register_read_raw!(u64, "MDCR_EL3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = MDCR_EL3::Register;
    sys_register_write_raw!(u64, "MDCR_EL3", "x");
}

pub const MDCR_EL3: Reg = Reg {};
