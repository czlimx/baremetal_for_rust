//! Holds information about ECC errors on the: 
//! • L2 data RAMs.
//! • L2 tag RAMs.
//! • SCU snoop filter RAMs.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// L2 Memory Error Syndrome Register
    pub L2MERRSR_EL1 [
        /// Fatal bit. This bit is set to 1 on the first memory error that 
        /// caused a data abort. It is a sticky bit so that after it is set, 
        /// it remains set until the register is written.
        Fatal OFFSET(63) NUMBITS(1) [
            InValid             = 0b0,
            Valid               = 0b1
        ],

        /// This field is set to 0 on the first memory error and is incremented 
        /// on any memory error that does not match the RAMID and Bank/Way 
        /// information in this register while the sticky Valid bit is set.
        OtherErrorCount OFFSET(40) NUMBITS(8) [],

        /// This field is set to 0 on the first memory error and is incremented 
        /// on any memory error that exactly matches the RAMID and Bank/Way 
        /// information in this register while the sticky Valid bit is set.
        RepeatErrorCount OFFSET(32) NUMBITS(8) [],

        /// Valid bit. This bit is set to 1 on the first memory error.
        Valid OFFSET(31) NUMBITS(1) [
            InValid             = 0b0,
            Valid               = 0b1
        ],

        /// RAM Identifier. Indicates the RAM in which the first memory error.
        RAMID OFFSET(24) NUMBITS(7) [
            L2Tag               = 0x10,
            L2Data              = 0x11,
            SCU                 = 0x12
        ],

        /// Indicates the RAM where the first memory error occurred.
        CPUIDWay OFFSET(18) NUMBITS(3) [],

        /// Indicates the index address of the first memory error.
        RAMAddress OFFSET(3) NUMBITS(14) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = L2MERRSR_EL1::Register;
    sys_register_read_raw!(u64, "S3_1_C15_C2_3", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = L2MERRSR_EL1::Register;
    sys_register_write_raw!(u64, "S3_1_C15_C2_3", "x");
}

pub const L2MERRSR_EL1: Reg = Reg {};
