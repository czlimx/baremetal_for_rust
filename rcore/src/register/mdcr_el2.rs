//! Controls the trapping to Hyp mode of Non-secure accesses, at EL1 or 
//! lower, to functions provided by the debug and trace architectures and the 
//! Performance Monitor.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Hyp Debug Control Register
    pub MDCR_EL2 [
        /// Trap debug ROM address register access.
        TDRA OFFSET(11) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Trap Debug OS-related register access.
        TDOSA OFFSET(10) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Trap Debug Access.
        TDA OFFSET(9) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Trap software debug exceptions.
        TDE OFFSET(8) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Hypervisor Performance Monitor Enable.
        HPME OFFSET(7) NUMBITS(1) [
            Disable = 0b0,
            Enable  = 0b1
        ],

        /// Trap Performance Monitor accesses.
        TPM OFFSET(6) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Trap PMCR_EL0 accesses.
        TPMCR OFFSET(5) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Hyp Performance Monitor count.
        HPMN OFFSET(0) NUMBITS(5) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = MDCR_EL2::Register;
    sys_register_read_raw!(u64, "MDCR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = MDCR_EL2::Register;
    sys_register_write_raw!(u64, "MDCR_EL2", "x");
}

pub const MDCR_EL2: Reg = Reg {};
