//! Holds the value of the Virtualization Processor ID. This is the value 
//! returned by Non-secure EL1 reads of MIDR. See MIDR_EL1 bit 
//! assignments on page 4-16.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Virtualization Processor ID Register
    pub VPIDR_EL2 [
        /// The Implementer code.
        Implementer OFFSET(24) NUMBITS(8) [],

        /// Indicates the variant number of the processor.
        Variant OFFSET(20) NUMBITS(4) [],

        /// Architecture version. Defined values
        Architecture OFFSET(16) NUMBITS(4) [],

        /// Primary Part Number for the device.
        PartNum OFFSET(4) NUMBITS(12) [],

        /// Revision number for the device.
        Revision OFFSET(0) NUMBITS(4) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = VPIDR_EL2::Register;
    sys_register_read_raw!(u64, "VPIDR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = VPIDR_EL2::Register;
    sys_register_write_raw!(u64, "VPIDR_EL2", "x");
}

pub const VPIDR_EL2: Reg = Reg {};
