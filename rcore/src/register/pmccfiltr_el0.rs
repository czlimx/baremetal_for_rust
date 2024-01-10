//! Determines the modes in which the Cycle Counter, PMCCNTR_EL0, increments.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Performance Monitors Cycle Count Filter Register
    pub PMCCFILTR_EL0 [
        /// EL1 filtering. Controls counting cycles in EL1.
        P OFFSET(31) NUMBITS(1) [
            NotFilter       = 0b0,
            FilterCycle     = 0b1
        ],

        /// EL0 filtering. Controls counting cycles in EL0.
        U OFFSET(30) NUMBITS(1) [
            NotFilter       = 0b0,
            FilterCycle     = 0b1
        ],

        /// Non-secure EL1 filtering.
        NSK OFFSET(29) NUMBITS(1) [
            NotFilter       = 0b0,
            FilterCycle     = 0b1
        ],

        /// Non-secure EL0 filtering.
        NSU OFFSET(28) NUMBITS(1) [
            NotFilter       = 0b0,
            FilterCycle     = 0b1
        ],

        /// EL2 filtering. Controls counting cycles in EL2.
        NSH OFFSET(27) NUMBITS(1) [
            FilterCycle     = 0b0,
            NotFilter       = 0b1
        ],

        /// EL3 filtering. Controls counting cycles in EL3.
        M OFFSET(26) NUMBITS(1) [
            NotFilter       = 0b0,
            FilterCycle     = 0b1
        ],

        /// Secure EL2 filtering. Controls counting cycles in Secure EL2.
        SH OFFSET(24) NUMBITS(1) [
            NotFilter       = 0b0,
            FilterCycle     = 0b1
        ],

        /// Transactional state filtering bit.
        T OFFSET(23) NUMBITS(1) [
            NotFilter       = 0b0,
            FilterCycle     = 0b1
        ],

        /// Realm EL1 (kernel) filtering bit.
        RLK OFFSET(22) NUMBITS(1) [
            NotFilter       = 0b0,
            FilterCycle     = 0b1
        ],

        /// Realm EL0 (unprivileged) filtering bit.
        RLU OFFSET(21) NUMBITS(1) [
            NotFilter       = 0b0,
            FilterCycle     = 0b1
        ],

        /// Realm EL2 filtering bit. Controls counting in Realm EL2.
        RLH OFFSET(20) NUMBITS(1) [
            NotFilter       = 0b0,
            FilterCycle     = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PMCCFILTR_EL0::Register;
    sys_register_read_raw!(u64, "PMCCFILTR_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = PMCCFILTR_EL0::Register;
    sys_register_write_raw!(u64, "PMCCFILTR_EL0", "x");
}

pub const PMCCFILTR_EL0: Reg = Reg {};
