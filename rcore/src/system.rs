use tock_registers::interfaces::{Writeable, Readable};
use crate::register::*;

/// Enable some privileged instructions of EL1, EL2, and EL3
unsafe fn el3_init() {
    // CNTPS_TVAL_EL1, CNTS_CTL_EL1, and CNTPS_CVAL_EL1 registers accessible in 
    // EL3 and EL1 when SCR_EL3.NS is 0.
    SCR_EL3.write(SCR_EL3::ST::Enable);

    // The HVC instruction is enabled at EL1, EL2 or EL3.
    SCR_EL3.write(SCR_EL3::HCE::Enable);

    // The SMC instruction is enabled at EL1, EL2, and EL3.
    SCR_EL3.write(SCR_EL3::SMD::Enable);

    // L2ACTLR_EL1 write access control write accessible from EL2.
    ACTLR_EL3.write(ACTLR_EL3::L2ACTLR_EL1::Accessible);

    // L2ECTLR_EL1 write access control write accessible from EL2.
    ACTLR_EL3.write(ACTLR_EL3::L2ECTLR_EL1::Accessible);

    // L2CTLR_EL1 write access control write accessible from EL2.
    ACTLR_EL3.write(ACTLR_EL3::L2CTLR_EL1::Accessible);

    // CPUECTLR_EL1 write access control write accessible from EL2.
    ACTLR_EL3.write(ACTLR_EL3::CPUECTLR_EL1::Accessible);

    // CPUACTLR_EL1 write access control write accessible from EL2.
    ACTLR_EL3.write(ACTLR_EL3::CPUACTLR_EL1::Accessible);
}

/// Enable some privileged instructions of EL1
unsafe fn el2_init () {
    // L2ACTLR_EL1 write access control write accessible from Non-secure EL1.
    ACTLR_EL2.write(ACTLR_EL2::L2ACTLR_EL1::Accessible);

    // L2ECTLR_EL1 write access control write accessible from Non-secure EL1.
    ACTLR_EL2.write(ACTLR_EL2::L2ECTLR_EL1::Accessible);

    // L2CTLR_EL1 write access control write accessible from Non-secure EL1.
    ACTLR_EL2.write(ACTLR_EL2::L2CTLR_EL1::Accessible);

    // CPUECTLR_EL1 write access control write accessible from Non-secure EL1.
    ACTLR_EL2.write(ACTLR_EL2::CPUECTLR_EL1::Accessible);

    // CPUACTLR_EL1 write access control write accessible from Non-secure EL1.
    ACTLR_EL2.write(ACTLR_EL2::CPUACTLR_EL1::Accessible);
}

/// Enable some privileged instructions of EL1
unsafe fn el1_init () {
    // Enables EL0 access to the DC CVAU, DC CIVAC, DC CVAC and IC IVAU 
    // instructions in the AArch64 Execution state.
    SCTLR_EL1.write(SCTLR_EL1::UCI::Enable);

    // Enables EL0 access to the CTR_EL0 register.
    SCTLR_EL1.write(SCTLR_EL1::UCT::Enable);

    // Enables execution access to the DC ZVA instruction at EL0.
    SCTLR_EL1.write(SCTLR_EL1::DZE::Enable);

    // The SETEND instruction is enabled.
    SCTLR_EL1.write(SCTLR_EL1::SED::Enable);

    // The IT instruction functionality is enabled.
    SCTLR_EL1.write(SCTLR_EL1::ITD::Enable);

    // CP15 barrier operations enabled.
    SCTLR_EL1.write(SCTLR_EL1::CP15BEN::Enable);
}

/// Enable instructions and virtualization ID information under each 
/// privilege level
pub unsafe fn init () {
    // Initialize privileged instructions
    el3_init();
    el2_init();
    el1_init();

    // Update Virtualization Multiprocessor ID Register
    VMPIDR_EL2.set(MPIDR_EL1.get());
    // Update Virtualization Processor ID Register
    VPIDR_EL2.set(MIDR_EL1.get());
}

/// Get the current core ID
pub unsafe fn coreid() -> u64{ 
    MPIDR_EL1.read(MPIDR_EL1::Aff0)
}

/// Get the current affinity value
pub unsafe fn affinity() -> u64{
    MPIDR_EL1.get()
}

/// The instruction returns the number of first 1 and first 0 in the binary 
/// encoding of the operand.
pub fn clz (bit_width:u64, val:u64) -> u64 {
    let mut bit = 64;

    for x in (0..64u64).rev() {
        if 1u64 == (val >> x) & 1u64 {
            bit = x + 1;
            break;
        }
    }

    return bit_width - bit;
}
