//! Controls the generation of an event stream from the physical counter, and access from EL1 to the 
//! physical counter and the EL1 physical timer.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Counter-timer Hypervisor Control register
    pub CNTHCTL_EL2 [
        /// When HCR_EL2.TGE is 0, traps EL0 and EL1 accesses to the E1 
        /// physical timer registers to EL2 when EL2 is enabled in the 
        /// current Security state.
        EL1PTEN OFFSET(11) NUMBITS(1) [
            Trap         = 0b0,
            NotTrap      = 0b1
        ],

        /// When HCR_EL2.TGE is 0, traps EL0 and EL1 accesses to the EL1 
        /// physical counter register to EL2 when EL2 is enabled in the 
        /// current Security state, as follows:
        /// • In AArch64 state, accesses to CNTPCT_EL0 are trapped to EL2, 
        /// reported using EC syndrome value 0x18.
        /// • In AArch32 state, MRRC or MCRR accesses to CNTPCT are trapped 
        /// to EL2, reported using EC syndrome value 0x04.
        EL1PCTEN OFFSET(10) NUMBITS(1) [
            Trap         = 0b0,
            NotTrap      = 0b1
        ],

        /// When HCR_EL2.TGE is 0, this control does not cause any instructions 
        /// to be trapped. When HCR_EL2.TGE is 1, traps EL0 accesses to the 
        /// physical timer registers to EL2.
        EL0PTEN OFFSET(9) NUMBITS(1) [
            Trap         = 0b0,
            NotTrap      = 0b1
        ],

        /// When HCR_EL2.TGE is 0, this control does not cause any instructions 
        /// to be trapped. When HCR_EL2.TGE is 1, traps EL0 accesses to the 
        /// virtual timer registers to EL2.
        EL0VTEN OFFSET(8) NUMBITS(1) [
            Trap         = 0b0,
            NotTrap      = 0b1
        ],

        /// Selects which bit of CNTPCT_EL0, as seen from EL2, is the trigger 
        /// for the event stream generated from that counter when that stream 
        /// is enabled. If FEAT_ECV is implemented, and CNTHCTL_EL2.EVNTIS is 1, 
        /// this field selects a trigger bit in the range 8 to 23 of CNTPCT_EL0.
        EVNTI OFFSET(4) NUMBITS(8) [],

        /// Controls which transition of the CNTPCT_EL0 trigger bit, as seen 
        /// from EL2 and defined by EVNTI, generates an event when the event 
        /// stream is enabled.
        EVNTDIR OFFSET(3) NUMBITS(1) [
            A0to1        = 0b0,
            A1to0        = 0b1
        ],

        /// Enables the generation of an event stream from CNTPCT_EL0 as seen 
        /// from EL2.
        EVNTEN OFFSET(2) NUMBITS(1) [
            Disables     = 0b0,
            Enables      = 0b1
        ],

        /// When HCR_EL2.TGE is 0, this control does not cause any instructions 
        /// to be trapped. When HCR_EL2.TGE is 1, traps EL0 accesses to the 
        /// frequency register and virtual counter register to EL2.
        EL0VCTEN OFFSET(1) NUMBITS(1) [
            Trap         = 0b0,
            NotTrap      = 0b1
        ],

        /// When HCR_EL2.TGE is 0, this control does not cause any instructions 
        /// to be trapped. When HCR_EL2.TGE is 1, traps EL0 accesses to the 
        /// frequency register and physical counter register to EL2.
        EL0PCTEN OFFSET(0) NUMBITS(1) [
            Trap         = 0b0,
            NotTrap      = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CNTHCTL_EL2::Register;
    sys_register_read_raw!(u64, "CNTHCTL_EL2", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTHCTL_EL2::Register;
    sys_register_write_raw!(u64, "CNTHCTL_EL2", "x");
}

pub const CNTHCTL_EL2: Reg = Reg {};
