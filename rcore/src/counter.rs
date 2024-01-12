use tock_registers::interfaces::Writeable;
use crate::register::*;

/// Initializing the system counter
pub unsafe fn init(frq:u64) {
    // Clock frequency. Indicates the system counter clock frequency, in Hz.
    CNTFRQ_EL0.set(frq);
}

pub mod physical {
    use tock_registers::interfaces::Writeable;
    use crate::register::*;

    pub enum PhysicalTimerElx {
        EL1,
        EL2,
        SecureEL2,
        EL3
    }

    pub unsafe fn get () -> u64 {
        // ensure ordering with previous memory accesses
        core::arch::asm!("dsb");
        core::arch::asm!("isb");


    }
}

pub mod r#virtual {
    use tock_registers::interfaces::Writeable;
    use crate::register::*;

    pub enum VirtualTimerElx {
        EL1,
        EL2,
        SecureEL2
    }

    pub unsafe fn get () -> u64 {
        // ensure ordering with previous memory accesses
        core::arch::asm!("dsb");
        core::arch::asm!("isb");

        
    }

    pub unsafe fn set (value:u64) {

    }

    pub unsafe fn compare (value:u64) {

    }

    pub unsafe fn control () {

    }
}