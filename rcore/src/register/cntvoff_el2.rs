//! Holds the 64-bit virtual offset. This is the offset between the physical count value visible in 
//! CNTPCT_EL0 and the virtual count value visible in CNTVCT_EL0.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Counter-timer Virtual Offset register
    pub CNTVOFF_EL2 [
        /// Virtual offset.
        Bits OFFSET(0) NUMBITS(64) [
            NonAccessible   = 0b0,
            Accessible      = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CNTVOFF_EL2::Register;
    sys_register_read_raw!(u64, "CNTVOFF_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTVOFF_EL2::Register;
    sys_register_write_raw!(u64, "CNTVOFF_EL2", "x");
}

pub const CNTVOFF_EL2: Reg = Reg {};
