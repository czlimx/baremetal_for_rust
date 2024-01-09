//! Contains the address that execution starts from after reset when executing 
//! in the AArch64 state.
//! RVBAR_EL3 is part of the Reset management registers functional group.

use tock_registers::{
    register_bitfields, 
    interfaces::Readable
};

register_bitfields! {u64,
    /// Reset Vector Base Address Register (EL3)
    pub RVBAR_EL3 [
        // Reset Vector Base Address.
        RVBA OFFSET(0) NUMBITS(64) [],
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = RVBAR_EL3::Register;
    sys_register_read_raw!(u64, "RVBAR_EL3", "x");
}

pub const RVBAR_EL3: Reg = Reg {};
