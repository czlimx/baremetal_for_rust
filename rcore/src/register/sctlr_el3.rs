//! Provides top level control of the system, including its memory system at 
//! EL3.
//! SCTLR_EL3 is part of the Virtual memory control registers functional 
//! group.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// System Control Register (EL3)
    pub SCTLR_EL3 [
        /// Exception endianness.
        EE OFFSET(25) NUMBITS(1) [
            LittleEndian   = 0b0,
            BigEndian      = 0b1
        ],

        /// Force treatment of all memory regions with write permissions as XN.
        WXN OFFSET(19) NUMBITS(1) [
            NotForce    = 0b0,
            Force       = 0b1
        ],

        /// Global instruction cache enable.
        I OFFSET(12) NUMBITS(1) [
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

        /// Global enable for the EL3 MMU.
        M OFFSET(0) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = SCTLR_EL3::Register;
    sys_register_read_raw!(u64, "SCTLR_EL3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = SCTLR_EL3::Register;
    sys_register_write_raw!(u64, "SCTLR_EL3", "x");
}

pub const SCTLR_EL3: Reg = Reg {};
