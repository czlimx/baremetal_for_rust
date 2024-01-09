//! Controls whether the System register interface or the memory-mapped 
//! interface to the GIC CPU interface is used for EL1.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Interrupt Controller System Register Enable register (EL1)
    pub ICC_SRE_EL1 [
        /// Disable IRQ bypass.
        DIB OFFSET(2) NUMBITS(1) [
            Enable    = 0b0,
            Disable   = 0b1
        ],

        /// Disable FIQ bypass.
        DFB OFFSET(1) NUMBITS(1) [
            Enable    = 0b0,
            Disable   = 0b1
        ],

        /// System Register Enable.
        SRE OFFSET(0) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = ICC_SRE_EL1::Register;
    sys_register_read_raw!(u64, "ICC_SRE_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = ICC_SRE_EL1::Register;
    sys_register_write_raw!(u64, "ICC_SRE_EL1", "x");
}

pub const ICC_SRE_EL1: Reg = Reg {};
