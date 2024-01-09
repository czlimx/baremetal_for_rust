//! Provides information about the architecture of the caches.

use tock_registers::{
    register_bitfields, 
    interfaces::Readable
};

register_bitfields! {u64,
    /// Cache Type Register
    pub CTR_EL0 [
        /// Cache Write-Back granule.
        CWG OFFSET(24) NUMBITS(4) [],

        /// Exclusives Reservation Granule.
        ERG OFFSET(20) NUMBITS(4) [],

        /// Log2 of the number of words in the smallest cache line of all the 
        /// data and unified caches that the processor controls.
        DminLine OFFSET(16) NUMBITS(4) [],

        /// Cache level of required cache.
        L1lp OFFSET(14) NUMBITS(2) [],

        /// Log2 of the number of words in the smallest cache line of all the 
        /// instruction caches that the processor controls.
        IminLine OFFSET(0) NUMBITS(4) []
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CTR_EL0::Register;
    sys_register_read_raw!(u64, "CTR_EL0", "x");
}

pub const CTR_EL0: Reg = Reg {};
