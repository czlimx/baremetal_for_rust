//! Provides configuration control for virtualization, including whether 
//! various Non-secure operations are trapped to EL2.
//! HCR_EL2 is part of the Hypervisor and virtualization registers functional 
//! group.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Hypervisor Configuration Register
    pub HCR_EL2 [
        /// Disables stage 2 instruction cache.
        ID OFFSET(33) NUMBITS(1) [
            NoEffect      = 0b0,
            NonCacheable  = 0b1
        ],

        /// Disables stage 2 data cache.
        CD OFFSET(32) NUMBITS(1) [
            NoEffect      = 0b0,
            NonCacheable  = 0b1
        ],

        /// Register width control for lower exception levels.
        RW OFFSET(31) NUMBITS(1) [
            AArch32 = 0b0,
            AArch64 = 0b1
        ],

        /// Trap reads of Virtual Memory controls.
        TRVM OFFSET(30) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps DC ZVA instruction.
        TDZ OFFSET(28) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps general exceptions.
        TGE OFFSET(27) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Trap virtual memory controls.
        TVM OFFSET(26) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps TLB maintenance instructions.
        TTLB OFFSET(25) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps cache maintenance instructions to Point of Unification (POU).
        TPU OFFSET(24) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps data or unified cache maintenance instructions to Point of 
        /// Coherency (POC).
        TPC OFFSET(23) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps data or unified cache maintenance instructions by Set or Way.
        TSW OFFSET(22) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps Auxiliary Control registers.
        TACR OFFSET(21) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Trap Implementation Dependent functionality.
        TIDCP OFFSET(20) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps SMC instruction.
        TSC OFFSET(19) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps ID group 3 registers.
        TID3 OFFSET(18) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps ID group 2 registers.
        TID2 OFFSET(17) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps ID group 1 registers.
        TID1 OFFSET(16) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps ID group 0 registers.
        TID0 OFFSET(15) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps WFE instruction if it would cause suspension of execution.
        TWE OFFSET(14) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Traps WFI instruction if it causes suspension of execution.
        TWI OFFSET(13) NUMBITS(1) [
            Notrapp = 0b0,
            Trapp   = 0b1
        ],

        /// Default cacheable.
        DC OFFSET(12) NUMBITS(1) [
            Disable = 0b0,
            Enable  = 0b1
        ],

        /// Barrier shareability upgrade.
        BSU OFFSET(10) NUMBITS(2) [
            NoEffect         = 0b00,
            InnerShareable   = 0b01,
            OuterShareable   = 0b10,
            FullSystem       = 0b11
        ],

        /// Forces broadcast.
        FB OFFSET(9) NUMBITS(1) [
            NotBroadcast = 0b0,
            Broadcast    = 0b1
        ],

        /// Virtual System Error/Asynchronous Abort.
        VSE OFFSET(8) NUMBITS(1) [
            NotPending = 0b0,
            Pending    = 0b1
        ],

        /// Virtual IRQ interrupt.
        VI OFFSET(7) NUMBITS(1) [
            NotPending = 0b0,
            Pending    = 0b1
        ],

        /// Virtual FIQ interrupt.
        VF OFFSET(6) NUMBITS(1) [
            NotPending = 0b0,
            Pending    = 0b1
        ],

        /// Asynchronous abort and error interrupt routing.
        AMO OFFSET(5) NUMBITS(1) [
            NonEl2    = 0b0,
            EL2       = 0b1
        ],

        /// Physical IRQ routing.
        IMO OFFSET(4) NUMBITS(1) [
            NonEl2    = 0b0,
            EL2       = 0b1
        ],

        /// Physical FIQ routing.
        FMO OFFSET(3) NUMBITS(1) [
            NonEl2    = 0b0,
            EL2       = 0b1
        ],

        /// Protected Table Walk.
        PTW OFFSET(2) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],

        /// Set/Way Invalidation Override.
        SWIO OFFSET(1) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ],

        /// Enables second stage of translation.
        VM OFFSET(0) NUMBITS(1) [
            Disable   = 0b0,
            Enable    = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = HCR_EL2::Register;
    sys_register_read_raw!(u64, "HCR_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = HCR_EL2::Register;
    sys_register_write_raw!(u64, "HCR_EL2", "x");
}

pub const HCR_EL2: Reg = Reg {};
