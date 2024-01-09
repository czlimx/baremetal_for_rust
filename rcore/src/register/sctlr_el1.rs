//! Provides top level control of the system, including its memory system at 
//! EL1.
//! SCTLR_EL1 is part of the Virtual memory control registers functional 
//! group.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// System Control Register (EL1)
    pub SCTLR_EL1 [
        /// Enables EL0 access to the DC CVAU, DC CIVAC, DC CVAC and IC IVAU 
        /// instructions in the AArch64 Execution state.
        UCI OFFSET(26) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],

        /// Exception endianness.
        EE OFFSET(25) NUMBITS(1) [
            LittleEndian   = 0b0,
            BigEndian      = 0b1
        ],

        /// Endianness of explicit data access at EL0.
        E0E OFFSET(24) NUMBITS(1) [
            LittleEndian   = 0b0,
            BigEndian      = 0b1
        ],

        /// Force treatment of all memory regions with write permissions as XN.
        WXN OFFSET(19) NUMBITS(1) [
            NotForce    = 0b0,
            Force       = 0b1
        ],

        /// WFE non-trapping.
        nTWE OFFSET(18) NUMBITS(1) [
            Trapped    = 0b0,
            NonTrapped = 0b1
        ],

        /// WFI non-trapping.
        nTWI OFFSET(16) NUMBITS(1) [
            Trapped    = 0b0,
            NonTrapped = 0b1
        ],

        /// Enables EL0 access to the CTR_EL0 register in AArch64 Execution state.
        UCT OFFSET(15) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],

        ///  Enables access to the DC ZVA instruction at EL0.
        DZE OFFSET(14) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],

        /// Global instruction cache enable.
        I OFFSET(12) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],

        /// User Mask Access.
        UMA OFFSET(9) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],

        /// SETEND instruction disable
        SED OFFSET(8) NUMBITS(1) [
            Enable    = 0b0,
            Disable   = 0b1
        ],

        /// IT instruction disable.
        ITD OFFSET(7) NUMBITS(1) [
            Enable    = 0b0,
            Disable   = 0b1
        ],

        /// CP15 barrier enable.
        CP15BEN OFFSET(5) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],

        /// Enable EL0 stack alignment check.
        SA0 OFFSET(4) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],

        /// Enables stack alignment check.
        SA OFFSET(3) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],

        /// Global enable for data and unifies caches.
        C OFFSET(2) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],

        /// Enable alignment fault check
        A OFFSET(1) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],

        /// Enable for the EL1 and EL0 stage 1 MMU.
        M OFFSET(0) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = SCTLR_EL1::Register;
    sys_register_read_raw!(u64, "SCTLR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = SCTLR_EL1::Register;
    sys_register_write_raw!(u64, "SCTLR_EL1", "x");
}

pub const SCTLR_EL1: Reg = Reg {};
