//! Controls trapping to EL3 for accesses to CPACR, Trace functionality and 
//! registers associated with Advanced SIMD and Floating-point execution. 
//! Controls EL3 access to this functionality.
//! CPTR_EL3 is part of the Security registers functional group.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Architectural Feature Trap Register (EL3)
    pub CPTR_EL3 [
        /// This causes a direct access to the CPACR_EL1 from EL1 or the 
        /// CPTR_EL2 from EL2 to trap to EL3 unless it is trapped at EL2.
        TCPAC OFFSET(31) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// This causes instructions that access the registers associated with 
        /// Advanced SIMD or floating-point execution to trap to EL3 when 
        /// executed from any exception level, unless trapped to EL1 or EL2.
        TFP OFFSET(10) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CPTR_EL3::Register;
    sys_register_read_raw!(u64, "CPTR_EL3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CPTR_EL3::Register;
    sys_register_write_raw!(u64, "CPTR_EL3", "x");
}

pub const CPTR_EL3: Reg = Reg {};
