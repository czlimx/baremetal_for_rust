//! Controls access to ThumbEE and coprocessor registers at lower exception 
//! levels in AArch32.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Hyp System Trap Register
    pub HSTR_EL2 [
        /// Trap T32EE.
        TTEE OFFSET(16) NUMBITS(1) [
            NonTrapped      = 0b0,
            Trapped         = 0b1
        ],

        /// Trap coprocessor primary register CRn = 15.
        T15 OFFSET(15) NUMBITS(1) [
            NonTrapped      = 0b0,
            Trapped         = 0b1
        ],

        /// Trap coprocessor primary register CRn = 13.
        T13 OFFSET(13) NUMBITS(1) [
            NonTrapped      = 0b0,
            Trapped         = 0b1
        ],

        /// Trap coprocessor primary register CRn = 12.
        T12 OFFSET(12) NUMBITS(1) [
            NonTrapped      = 0b0,
            Trapped         = 0b1
        ],

        /// Trap coprocessor primary register CRn = 11.
        T11 OFFSET(11) NUMBITS(1) [
            NonTrapped      = 0b0,
            Trapped         = 0b1
        ],

        /// Trap coprocessor primary register CRn = 10.
        T10 OFFSET(10) NUMBITS(1) [
            NonTrapped      = 0b0,
            Trapped         = 0b1
        ],

        /// Trap coprocessor primary register CRn = 9.
        T9 OFFSET(9) NUMBITS(1) [
            NonTrapped      = 0b0,
            Trapped         = 0b1
        ],

        /// Trap coprocessor primary register CRn = 8.
        T8 OFFSET(8) NUMBITS(1) [
            NonTrapped      = 0b0,
            Trapped         = 0b1
        ],

        /// Trap coprocessor primary register CRn = 7.
        T7 OFFSET(7) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Trap coprocessor primary register CRn = 6.
        T6 OFFSET(6) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Trap coprocessor primary register CRn = 5.
        T5 OFFSET(5) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Trap coprocessor primary register CRn = 3.
        T3 OFFSET(3) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Trap coprocessor primary register CRn = 2.
        T2 OFFSET(2) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Trap coprocessor primary register CRn = 1.
        T1 OFFSET(1) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Trap coprocessor primary register CRn = 0.
        T0 OFFSET(0) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = HSTR_EL2::Register;
    sys_register_read_raw!(u64, "HSTR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = HSTR_EL2::Register;
    sys_register_write_raw!(u64, "HSTR_EL2", "x");
}

pub const HSTR_EL2: Reg = Reg {};
