//! Controls trapping to EL2 for accesses to CPACR, Trace functionality and 
//! registers associated with Advanced SIMD and Floating-point execution. 
//! Controls EL2 access to this functionality.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Architectural Feature Trap Register (EL2)
    pub CPTR_EL2 [
        /// Traps direct access to CPACR from Non-secure EL1 to EL2.
        TCPAC OFFSET(31) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps instructions that access registers associated with Advanced 
        /// SIMD and Floating-point execution from a lower exception level to 
        /// EL2, unless trapped to EL1.
        TFP OFFSET(10) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CPTR_EL2::Register;
    sys_register_read_raw!(u64, "CPTR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CPTR_EL2::Register;
    sys_register_write_raw!(u64, "CPTR_EL2", "x");
}

pub const CPTR_EL2: Reg = Reg {};
