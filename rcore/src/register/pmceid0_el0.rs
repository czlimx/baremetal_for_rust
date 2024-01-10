//! Defines which common architectural and common microarchitectural 
//! feature events are implemented.

use tock_registers::{
    register_bitfields, 
    interfaces::Readable
};

register_bitfields! {u64,
    /// Performance Monitor Common Event Identification Register 0
    pub PMCEID0_EL0 [
        /// L1 Data cache allocate.
        L1D_CACHE_ALLOCATE OFFSET(31) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Chain. For odd-numbered counters, counts once for each 
        /// overflow of the preceding even-numbered counter. For 
        /// even-numbered counters, does not count.
        CHAIN OFFSET(30) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Bus cycle.
        BUS_CYCLES OFFSET(29) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// TTBR write, architecturally executed, condition check 
        /// pass - write to translation table base.
        TTBR_WRITE_RETIRED OFFSET(28) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Instruction speculatively executed.
        INST_SPEC OFFSET(27) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Local memory error.
        MEMORY_ERROR OFFSET(26) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Bus access.
        BUS_ACCESS OFFSET(25) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// L2 Data cache Write-Back.
        L2D_CACHE_WB OFFSET(24) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// L2 Data cache refill.
        L2D_CACHE_REFILL OFFSET(23) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// L2 Data cache access.
        L2D_CACHE OFFSET(22) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// L1 Data cache Write-Back.
        L1D_CACHE_WB OFFSET(21) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// L1 Instruction cache access.
        L1I_CACHE OFFSET(20) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Data memory access.
        MEM_ACCESS OFFSET(19) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Predictable branch speculatively executed.
        BR_PRED OFFSET(18) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Cycle.
        CPU_CYCLES OFFSET(17) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Mispredicted or not predicted branch speculatively executed.
        BR_MIS_PRED OFFSET(16) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Instruction architecturally executed, condition check 
        /// pass - unaligned load or store.
        UNALIGNED_LDST_RETIRED OFFSET(15) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Instruction architecturally executed, condition check 
        /// pass - procedure return.
        BR_RETURN_RETIRED OFFSET(14) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Instruction architecturally executed - immediate branch.
        BR_IMMED_RETIRED OFFSET(13) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Instruction architecturally executed, condition check 
        /// pass - software change of the PC.
        PC_WRITE_RETIRED OFFSET(12) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Instruction architecturally executed, condition check 
        /// pass - write to CONTEXTIDR.
        CID_WRITE_RETIRED OFFSET(11) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Instruction architecturally executed, condition check 
        /// pass - exception return.
        EXC_RETURN OFFSET(10) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Exception taken.
        EXC_TAKEN OFFSET(9) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Instruction architecturally executed.
        INST_RETIRED OFFSET(8) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Instruction architecturally executed, condition check pass - store.
        ST_RETIRED OFFSET(7) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Instruction architecturally executed, condition check pass - load.
        LD_RETIRED OFFSET(6) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// L1 Data TLB refill.
        L1D_TLB_REFILL OFFSET(5) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// L1 Data cache access.
        L1D_CACHE OFFSET(4) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// L1 Data cache refill.
        L1D_CACHE_REFILL OFFSET(3) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// L1 Instruction TLB refill.
        L1I_TLB_REFILL OFFSET(2) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// L1 Instruction cache refill.
        L1I_CACHE_REFILL OFFSET(1) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ],

        /// Instruction architecturally executed, condition check 
        /// pass - software increment.
        SW_INCR OFFSET(0) NUMBITS(1) [
            NotImplemented  = 0b0,
            Implemented     = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = PMCEID0_EL0::Register;
    sys_register_read_raw!(u64, "PMCEID0_EL0", "x");
}

pub const PMCEID0_EL0: Reg = Reg {};
