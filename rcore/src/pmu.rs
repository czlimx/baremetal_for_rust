use tock_registers::interfaces::Writeable;
use crate::register::*;

/// Enable some privileged instructions of EL0
pub unsafe fn init() {
    // Overflow on increment that changes PMCCNTR_EL0[63] from 1 to 0.
    PMCR_EL0.write(PMCR_EL0::LC::Overflow63);

    // Cycle counter is disabled if non-invasive debug is not permitted and enabled.
    PMCR_EL0.write(PMCR_EL0::DP::Disable);

    // Export of events is enabled.
    PMCR_EL0.write(PMCR_EL0::X::Enable);

    // PMCCNTR_EL0 counts every 64 clock cycles.
    PMCR_EL0.write(PMCR_EL0::D::Cycle64);

    // Reset PMCCNTR_EL0 to 0.
    PMCR_EL0.write(PMCR_EL0::C::Reset);

    // Reset all event counters, not including PMCCNTR_EL0, to zero.
    PMCR_EL0.write(PMCR_EL0::P::Reset);

    // All counters are enabled.
    PMCR_EL0.write(PMCR_EL0::E::Enable);

    // EL1 filtering no effect
    PMCCFILTR_EL0.write(PMCCFILTR_EL0::P::NotFilter);

    // EL0 filtering no effect
    PMCCFILTR_EL0.write(PMCCFILTR_EL0::U::NotFilter);

    // Non-secure EL1 filtering no effect
    PMCCFILTR_EL0.write(PMCCFILTR_EL0::NSK::NotFilter);

    // Non-secure EL0 filtering no effect
    PMCCFILTR_EL0.write(PMCCFILTR_EL0::NSU::NotFilter);

    // EL2 filtering no effect
    PMCCFILTR_EL0.write(PMCCFILTR_EL0::NSH::NotFilter);

    // EL3 filtering no effect
    PMCCFILTR_EL0.write(PMCCFILTR_EL0::M::NotFilter);

    // EL0 reads of the event counters and EL0 reads and writes of 
    // the select register are enabled
    PMUSERENR_EL0.write(PMUSERENR_EL0::ER::Enable);

    // EL0 reads of the cycle counter are enabled
    PMUSERENR_EL0.write(PMUSERENR_EL0::CR::Enable);

    // EL0 writes to the Software increment register are enabled
    PMUSERENR_EL0.write(PMUSERENR_EL0::SW::Enable);

    // EL0 accesses to the specified PMU System registers are enabled
    PMUSERENR_EL0.write(PMUSERENR_EL0::EN::Enable);
}

/// Performance Monitors Cycle Count
pub mod cycle {
    use tock_registers::interfaces::{Writeable, Readable};
    use crate::register::*;

    pub unsafe fn enable () {
        // On writes, allows software to enable PMCCNTR_EL0.
        PMCNTENSET_EL0.write(PMCNTENSET_EL0::C::Enable);
    } 

    pub unsafe fn disable () {
        // On writes, allows software to disable PMCCNTR_EL0.
        PMCNTENCLR_EL0.write(PMCNTENCLR_EL0::C::Disable);
    }

    pub unsafe fn get_overflow_flag () ->bool {
        // On reads, returns the unsigned overflow 
        // flag for PMEVCNTR<n>_EL0 overflow status.
        if 1 == PMOVSCLR_EL0.read(PMOVSCLR_EL0::C) {
            return true;
        } else {
            return false;
        }
    }

    pub unsafe fn clear_overflow_flag () {
        // On writes, allows software to clear the 
        // unsigned overflow flag for PMEVCNTR<n>_EL0 to 0.
        PMOVSCLR_EL0.set(PMOVSCLR_EL0.get() | (1 << 31));
    }

    pub unsafe fn set (value:u64) {
        // writes the Cycle count
        PMCCNTR_EL0.set(value)
    }

    pub unsafe fn get () -> u64{
        // Reads the Cycle count
        PMCCNTR_EL0.get()
    }

    pub unsafe fn interrupt_enable () {
        // Interrupt request on unsigned overflow of PMCCNTR_EL0 enabled.
        PMINTENSET_EL1.write(PMINTENSET_EL1::C::Enable);
    } 

    pub unsafe fn interrupt_disable () {
        // Interrupt request on unsigned overflow of PMCCNTR_EL0 disabled.
        PMINTENSET_EL1.write(PMINTENSET_EL1::C::Disable);
    }

}

/// Performance Monitors Event Counter
pub mod event {
    use tock_registers::interfaces::{Writeable, Readable};
    use crate::register::*;

    pub unsafe fn enable (counter:u64) {
        // On writes, allows software to enable PMEVCNTR<n>_EL0.
        PMCNTENSET_EL0.set(PMCNTENSET_EL0.get() | (1 << counter));
    } 

    pub unsafe fn disable (counter:u64) {
        // On writes, allows software to disable PMEVCNTR<n>_EL0.
        PMCNTENCLR_EL0.set(PMCNTENCLR_EL0.get() | (1 << counter));
    }

    pub unsafe fn get_overflow_flag (counter:u64) ->bool {
        // On reads, returns the unsigned overflow 
        // flag for PMEVCNTR<n>_EL0 overflow status.
        if 1 == ((PMOVSCLR_EL0.get() >> counter) & 0x1) {
            return true;
        } else {
            return false;
        }
    }

    pub unsafe fn clr_overflow_flag (counter:u64) {
        // On writes, allows software to clear the 
        // unsigned overflow flag for PMEVCNTR<n>_EL0 to 0.
        PMOVSCLR_EL0.set(PMOVSCLR_EL0.get() | (1 << counter));
    }

    pub unsafe fn increment (counter:u64) {
        // Increment PMEVCNTR<n>_EL0, if PMEVCNTR<n>_EL0 is configured to count 
        // software increment events.
        PMSWINC_EL0.set(1 << counter);
    }

    pub unsafe fn select (counter:u64, event:u64) {
        // Selects the counter accessed by subsequent accesses to 
        // PMXEVTYPER_EL0 and PMXEVCNTR_EL0.
        PMSELR_EL0.set(counter);

        // Selected Event Type
        PMXEVTYPER_EL0.set(event);
    }

    pub unsafe fn set (counter:u64, value:u64) {
        // Selects the counter accessed by subsequent accesses to 
        // PMXEVTYPER_EL0 and PMXEVCNTR_EL0.
        PMSELR_EL0.set(counter);

        // writes the value of the selected event counter
        PMXEVCNTR_EL0.set(value)
    }

    pub unsafe fn get (counter:u64) -> u64{
        // Selects the counter accessed by subsequent accesses to 
        // PMXEVTYPER_EL0 and PMXEVCNTR_EL0.
        PMSELR_EL0.set(counter);

        // Reads the value of the selected event counter
        PMXEVCNTR_EL0.get()
    }

    pub unsafe fn interrupt_enable (counter:u64) {
        // Interrupt request on unsigned overflow of PMEVCNTR<n>_EL0 enabled.
        PMINTENSET_EL1.set(PMINTENSET_EL1.get() | (1 << counter));
    } 

    pub unsafe fn interrupt_disable (counter:u64) {
        // Interrupt request on unsigned overflow of PMEVCNTR<n>_EL0 disabled.
        PMINTENSET_EL1.set(PMINTENSET_EL1.get() & (!(1 << counter)));
    }
}