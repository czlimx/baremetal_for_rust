//! Provides the value of the Virtualization Multiprocessor ID. This is the 
//! value returned by Non-secure EL1 reads of MPIDR.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Virtualization Multiprocessor ID Register
    pub VMPIDR_EL2 [
        /// Affinity level 3.
        Aff3 OFFSET(32) NUMBITS(8) [],

        /// Traps WFI instructions.
        U OFFSET(30) NUMBITS(1) [
            Multiprocessor = 0b0,
            Uniprocessor   = 0b1
        ],

        /// Indicates whether the lowest level of affinity consists of logical 
        /// PEs that are implemented using a multithreading type approach.
        MT OFFSET(24) NUMBITS(1) [],

        /// Affinity level 2.
        Aff2 OFFSET(16) NUMBITS(8) [],

        /// Affinity level 1.
        Aff1 OFFSET(8) NUMBITS(8) [],

        /// Affinity level 0.
        Aff0 OFFSET(0) NUMBITS(8) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = VMPIDR_EL2::Register;
    sys_register_read_raw!(u64, "VMPIDR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = VMPIDR_EL2::Register;
    sys_register_write_raw!(u64, "VMPIDR_EL2", "x");
}

pub const VMPIDR_EL2: Reg = Reg {};
