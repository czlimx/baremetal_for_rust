//! Defines the configuration of the security state. SCR_EL3 specifies:
//! • Security state of EL0 and EL1, either Secure or Non-secure.
//! • Register width at lower exception levels.
//! • The exception level that the processor takes exceptions at, if an IRQ, 
//! FIQ, or external abort occurs.
//! SCR_EL3 is part of the Security registers functional group.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Secure Configuration Register
    pub SCR_EL3 [
        /// Traps WFE instructions.
        TWE OFFSET(13) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps WFI instructions.
        TWI OFFSET(12) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Enable Secure EL1 access to CNTPS_TVAL_EL1, CNTS_CTL_EL1, 
        /// and CNTPS_CVAL_EL1 registers.
        ST OFFSET(11) NUMBITS(1) [
            Disable = 0b0,
            Enable  = 0b1
        ],

        /// Register width control for lower exception levels.
        RW OFFSET(10) NUMBITS(1) [
            AArch32 = 0b0,
            AArch64 = 0b1
        ],

        /// Secure Instruction Fetch.
        SIF OFFSET(9) NUMBITS(1) [
            Permitted   = 0b0,
            NoPermitted = 0b1
        ],

        /// Hyp Call enable. This bit enables the use of HVC instructions.
        HCE OFFSET(8) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],

        /// SMC instruction disable.
        SMD OFFSET(7) NUMBITS(1) [
            Enable    = 0b0,
            Disable   = 0b1
        ],

        /// External Abort and SError interrupt Routing.
        EA OFFSET(3) NUMBITS(1) [
            NonEl3    = 0b0,
            EL3       = 0b1
        ],

        /// Physical FIQ Routing.
        FIQ OFFSET(2) NUMBITS(1) [
            NonEl3    = 0b0,
            EL3       = 0b1
        ],

        /// Physical IRQ Routing.
        IRQ OFFSET(1) NUMBITS(1) [
            NonEl3    = 0b0,
            EL3       = 0b1
        ],

        /// Non-secure bit.
        NS OFFSET(0) NUMBITS(1) [
            Secure    = 0b0,
            NonSecure = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = SCR_EL3::Register;
    sys_register_read_raw!(u64, "SCR_EL3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = SCR_EL3::Register;
    sys_register_write_raw!(u64, "SCR_EL3", "x");
}

pub const SCR_EL3: Reg = Reg {};
