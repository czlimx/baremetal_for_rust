//! Controls write access to IMPLEMENTATION DEFINED registers in 
//! Non-secure EL1 modes, such as CPUACTLR, CPUECTLR, L2CTLR, 
//! L2ECTLR and L2ACTLR.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Auxiliary Control Register (EL2)
    pub ACTLR_EL2 [
        /// L2ACTLR_EL1 write access control.
        L2ACTLR_EL1 OFFSET(6) NUMBITS(1) [
            NonAccessible   = 0b0,
            Accessible      = 0b1
        ],

        /// L2ECTLR_EL1 write access control.
        L2ECTLR_EL1 OFFSET(5) NUMBITS(1) [
            NonAccessible   = 0b0,
            Accessible      = 0b1
        ],

        /// L2CTLR_EL1 write access control.
        L2CTLR_EL1 OFFSET(4) NUMBITS(1) [
            NonAccessible   = 0b0,
            Accessible      = 0b1
        ],

        /// CPUECTLR_EL1 write access control.
        CPUECTLR_EL1 OFFSET(1) NUMBITS(1) [
            NonAccessible   = 0b0,
            Accessible      = 0b1
        ],

        /// CPUACTLR_EL1 write access control.
        CPUACTLR_EL1 OFFSET(0) NUMBITS(1) [
            NonAccessible   = 0b0,
            Accessible      = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ACTLR_EL2::Register;
    sys_register_read_raw!(u64, "ACTLR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ACTLR_EL2::Register;
    sys_register_write_raw!(u64, "ACTLR_EL2", "x");
}

pub const ACTLR_EL2: Reg = Reg {};
