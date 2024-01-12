//! This register is provided so that software can discover the frequency of the system counter. It must 
//! be programmed with this value as part of system initialization. The value of the register is not 
//! interpreted by hardware.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Counter-timer Frequency register
    pub CNTFRQ_EL0 [
        /// Clock frequency. Indicates the system counter clock frequency, in Hz.
        Bits OFFSET(0) NUMBITS(32) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CNTFRQ_EL0::Register;
    sys_register_read_raw!(u64, "CNTFRQ_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTFRQ_EL0::Register;
    sys_register_write_raw!(u64, "CNTFRQ_EL0", "x");
}

pub const CNTFRQ_EL0: Reg = Reg {};
