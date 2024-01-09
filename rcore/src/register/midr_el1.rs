//! Provides identification information for the PE, including an implementer 
//! code for the device and a device ID number.

use tock_registers::{
    register_bitfields, 
    interfaces::Readable
};

register_bitfields! {u64,
    /// Main ID Register
    pub MIDR_EL1 [
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
    type R = MIDR_EL1::Register;
    sys_register_read_raw!(u64, "MIDR_EL1", "x");
}

pub const MIDR_EL1: Reg = Reg {};
