//! Allows access to the AArch32 DACR register from AArch64 state only. 
//! Its value has no effect on execution in AArch64 state.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    ///  Domain Access Control Register
    pub DACR32_EL2 [
        /// Domain n access permission.
        D15 OFFSET(30) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],

        /// Domain n access permission.
        D14 OFFSET(28) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],

        /// Domain n access permission.
        D13 OFFSET(26) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],

        /// Domain n access permission.
        D12 OFFSET(24) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],

        /// Domain n access permission.
        D11 OFFSET(22) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],

        /// Domain n access permission.
        D10 OFFSET(20) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],

        /// Domain n access permission.
        D9 OFFSET(18) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],

        /// Domain n access permission.
        D8 OFFSET(16) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],

        /// Domain n access permission.
        D7 OFFSET(14) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],

        /// Domain n access permission.
        D6 OFFSET(12) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],

        /// Domain n access permission.
        D5 OFFSET(10) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],

        /// Domain n access permission.
        D4 OFFSET(8) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],

        /// Domain n access permission.
        D3 OFFSET(6) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],
        /// Domain n access permission.
        D2 OFFSET(4) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],

        /// Domain n access permission.
        D1 OFFSET(2) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ],

        /// Domain n access permission.
        D0 OFFSET(0) NUMBITS(2) [
            NoAccess    = 0b00,
            Client      = 0b01,
            Reserved    = 0b10,
            Manager     = 0b11
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = DACR32_EL2::Register;
    sys_register_read_raw!(u64, "DACR32_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = DACR32_EL2::Register;
    sys_register_write_raw!(u64, "DACR32_EL2", "x");
}

pub const DACR32_EL2: Reg = Reg {};
