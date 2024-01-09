//! Controls the execution state that the processor boots into and allows 
//! request of a warm reset.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Reset Management Register
    pub RMR_EL3 [
        // Reset Request.
        RR OFFSET(1) NUMBITS(1) [
            Normal      = 0b0,
            WarmReset   = 0b1
        ],

        // Determines which execution state the processor boots into after a 
        // warm reset.
        AA64 OFFSET(0) NUMBITS(1) [
            AArch32     = 0b0,
            AArch64     = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = RMR_EL3::Register;
    sys_register_read_raw!(u64, "RMR_EL3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = RMR_EL3::Register;
    sys_register_write_raw!(u64, "RMR_EL3", "x");
}

pub const RMR_EL3: Reg = Reg {};
