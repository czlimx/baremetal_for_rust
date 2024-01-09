//! Provides an additional core identification mechanism for scheduling 
//! purposes in a cluster system.

use tock_registers::{
    register_bitfields, 
    interfaces::Readable
};

register_bitfields! {u64,
    /// Multiprocessor Affinity Register
    pub MPIDR_EL1 [
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
        Aff0 OFFSET(0) NUMBITS(8) [
            Core0       = 0x0,
            Core1       = 0x1,
            Core2       = 0x2,
            Core3       = 0x3
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = MPIDR_EL1::Register;
    sys_register_read_raw!(u64, "MPIDR_EL1", "x");
}

pub const MPIDR_EL1: Reg = Reg {};
