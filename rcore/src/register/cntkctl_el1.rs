//! When FEAT_VHE is implemented and HCR_EL2.{E2H, TGE} is {1, 1}, this register does not 
//! cause any event stream from the virtual counter to be generated, and does not control access to the 
//! counters and timers. The access to counters and timers at EL0 is controlled by CNTHCTL_EL2.
//! When FEAT_VHE is not implemented, or when HCR_EL2.{E2H, TGE} is not {1, 1}, this register 
//! controls the generation of an event stream from the virtual counter, and access from EL0 to the 
//! physical counter, virtual counter, EL1 physical timers, and the virtual timer.

use tock_registers::{
    register_bitfields, 
    interfaces::{Writeable, Readable}
};

register_bitfields! {u64,
    /// Counter-timer Kernel Control register
    pub CNTKCTL_EL1 [
        /// Traps EL0 accesses to the physical timer registers to EL1, or to 
        /// EL2 when it is implemented and enabled for the current Security 
        /// state and HCR_EL2.TGE is 1, as follows:
        /// • In AArch64 state, the following registers are trapped, reported 
        /// using EC syndrome value 0x18:
        /// — CNTP_CTL_EL0, CNTP_CVAL_EL0, and CNTP_TVAL_EL0.
        /// • In AArch32 state, MRC and MCR accesses to the following registers 
        /// are trapped, reported using EC syndrome value 0x03, MRRC and MCRR 
        /// accesses are trapped, reported using EC syndrome value 0x04:
        /// — CNTP_CTL, CNTP_CVAL, CNTP_TVAL.
        EL0PTEN OFFSET(9) NUMBITS(1) [
            Trap            = 0b0,
            NotTrap         = 0b1
        ],

        /// Traps EL0 accesses to the virtual timer registers to EL1, or to EL2 when it is implemented and 
        /// enabled for the current Security state and HCR_EL2.TGE is 1, as follows:
        /// • In AArch64 state, accesses to the following registers are trapped, reported using EC 
        /// syndrome value 0x18:
        /// — CNTV_CTL_EL0, CNTV_CVAL_EL0, and CNTV_TVAL_EL0.
        /// • In AArch32 state, MRC and MCR accesses to the following registers are trapped and 
        /// reported using EC syndrome value 0x03, MRRC and MCRR accesses are trapped using EC 
        /// syndrome value 0x04:
        /// — CNTV_CTL, CNTV_CVAL, and CNTV_TVAL.        
        EL0VTEN OFFSET(8) NUMBITS(1) [
            Trap            = 0b0,
            NotTrap         = 0b1
        ],

        /// Selects which bit of CNTVCT_EL0, as seen from EL1, is the trigger for the event stream generated 
        /// from that counter when that stream is enabled.
        /// If FEAT_ECV is implemented, and CNTKCTL_EL1.EVNTIS is 1, this field selects a trigger bit in 
        /// the range 8 to 23 of CNTVCT_EL0.
        EVNTI OFFSET(4) NUMBITS(8) [],

        /// Controls which transition of the CNTVCT_EL0 trigger bit, as seen from EL1 and defined by 
        /// EVNTI, generates an event when the event stream is enabled.
        EVNTDIR OFFSET(3) NUMBITS(1) [
            A0to1        = 0b0,
            A1to0        = 0b1
        ],

        /// When FEAT_VHE is not implemented, or when HCR_EL2.{E2H, TGE} is not {1, 1}, enables the 
        /// generation of an event stream from CNTVCT_EL0 as seen from EL1.
        EVNTEN OFFSET(2) NUMBITS(1) [
            Disables     = 0b0,
            Enables      = 0b1
        ],

        /// Traps EL0 accesses to the frequency register and virtual counter register to EL1, or to EL2 when it 
        /// is implemented and enabled for the current Security state and HCR_EL2.TGE is 1, as follows:
        /// • In AArch64 state, accesses to the following registers are trapped and reported using EC 
        /// syndrome value 0x18:
        /// — CNTVCT_EL0 and if CNTKCTL_EL1.EL0PCTEN is 0, CNTFRQ_EL0.
        /// • In AArch32 state, MRC and MCR accesses to the following registers are trapped and 
        /// reported using EC syndrome value 0x03, MRRC and MCRR accesses are trapped and 
        /// reported using EC syndrome value 0x04:
        /// — CNTVCT and if CNTKCTL_EL1.EL0PCTEN is 0, CNTFRQ.
        EL0VCTEN OFFSET(1) NUMBITS(1) [
            Trap            = 0b0,
            NotTrap         = 0b1
        ],
        
        /// Traps EL0 accesses to the frequency register and physical counter register to EL1, or to EL2 when 
        /// it is implemented and enabled for the current Security state and HCR_EL2.TGE is 1, as follows:
        /// • In AArch64 state, the following registers are trapped, reported using EC syndrome value 
        /// 0x18:
        /// — CNTPCT_EL0 and if CNTKCTL_EL1.EL0VCTEN is 0, CNTFRQ_EL0.
        /// • In AArch32 state, MCR or MRC accesses the following registers are trapped, reported using 
        /// EC syndrome value 0x03, MCRR or MRRC accesses are trapped and reported using EC 
        /// syndrome value 0x04:
        /// — CNTPCT and if CNTKCTL_EL1.EL0VCTEN is 0, CNTFRQ.
        EL0PCTEN OFFSET(0) NUMBITS(1) [
            Trap            = 0b0,
            NotTrap         = 0b1
        ]
    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CNTKCTL_EL1::Register;
    sys_register_read_raw!(u64, "CNTKCTL_EL1", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CNTKCTL_EL1::Register;
    sys_register_write_raw!(u64, "CNTKCTL_EL1", "x");
}

pub const CNTKCTL_EL1: Reg = Reg {};
