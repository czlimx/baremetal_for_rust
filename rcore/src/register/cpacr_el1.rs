//! Controls access to trace functionality and access to registers associated 
//! with Advanced SIMD and Floating-point execution.
//! CPACR_EL1 is part of the Other system registers functional group.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Architectural Feature Access Control Register
    pub CPACR_EL1 [
        /// Traps instructions that access registers associated with Advanced 
        /// SIMD and Floating-point execution to trap to EL1 when executed from 
        /// EL0 or EL1.
        FPEN OFFSET(20) NUMBITS(2) [
            EL0EL1Trap   = 0b00,
            EL0ETrap     = 0b01,
            NoTrap       = 0b11
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CPACR_EL1::Register;
    sys_register_read_raw!(u64, "CPACR_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CPACR_EL1::Register;
    sys_register_write_raw!(u64, "CPACR_EL1", "x");
}

pub const CPACR_EL1: Reg = Reg {};
