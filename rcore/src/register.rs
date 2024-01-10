//! Cortex-A53 Core register bit field information

#[macro_use]
mod macros;

// AArch64 identification registers
mod midr_el1;
pub use midr_el1::MIDR_EL1;
mod mpidr_el1;
pub use mpidr_el1::MPIDR_EL1;
mod ccsidr_el1;
pub use ccsidr_el1::CCSIDR_EL1;
mod clidr_el1;
pub use clidr_el1::CLIDR_EL1;
mod csselr_el1;
pub use csselr_el1::CSSELR_EL1;
mod ctr_el0;
pub use ctr_el0::CTR_EL0;
mod dczid_el0;
pub use dczid_el0::DCZID_EL0;

// AArch64 exception handling registers
mod esr_el1;
pub use esr_el1::ESR_EL1;
mod esr_el2;
pub use esr_el2::ESR_EL2;
mod esr_el3;
pub use esr_el3::ESR_EL3;
mod far_el1;
pub use far_el1::FAR_EL1;
mod far_el2;
pub use far_el2::FAR_EL2;
mod far_el3;
pub use far_el3::FAR_EL3;
mod hpfar_el2;
pub use hpfar_el2::HPFAR_EL2;
mod ifsr32_el2;
pub use ifsr32_el2::IFSR32_EL2;
mod vbar_el1;
pub use vbar_el1::VBAR_EL1;
mod vbar_el2;
pub use vbar_el2::VBAR_EL2;
mod isr_el1;
pub use isr_el1::ISR_EL1;

// AArch64 virtual memory control registers
mod sctlr_el1;
pub use sctlr_el1::SCTLR_EL1;
mod sctlr_el3;
pub use sctlr_el3::SCTLR_EL3;
mod ttbr0_el1;
pub use ttbr0_el1::TTBR0_EL1;
mod ttbr0_el3;
pub use ttbr0_el3::TTBR0_EL3;
mod ttbr1_el1;
pub use ttbr1_el1::TTBR1_EL1;
mod tcr_el1;
pub use tcr_el1::TCR_EL1;
mod tcr_el2;
pub use tcr_el2::TCR_EL2;
mod tcr_el3;
pub use tcr_el3::TCR_EL3;
mod vtcr_el2;
pub use vtcr_el2::VTCR_EL2;
mod vttbr_el2;
pub use vttbr_el2::VTTBR_EL2;
mod mair_el1;
pub use mair_el1::MAIR_EL1;
mod mair_el2;
pub use mair_el2::MAIR_EL2;
mod mair_el3;
pub use mair_el3::MAIR_EL3;

//  AArch64 other system control registers
mod cpacr_el1;
pub use cpacr_el1::CPACR_EL1;
mod actlr_el2;
pub use actlr_el2::ACTLR_EL2;
mod actlr_el3;
pub use actlr_el3::ACTLR_EL3;

// AArch64 implementation defined registers
mod cbar_el1;
pub use cbar_el1::CBAR_EL1;

// AArch64 address translation operations
mod par_el1;
pub use par_el1::PAR_EL1;

// AArch64 reset registers
mod rmr_el3;
pub use rmr_el3::RMR_EL3;
mod rvbar_el3;
pub use rvbar_el3::RVBAR_EL3;

// AArch64 secure registers
mod scr_el3;
pub use scr_el3::SCR_EL3;
mod sder32_el3;
pub use sder32_el3::SDER32_EL3;
mod cptr_el3;
pub use cptr_el3::CPTR_EL3;
mod mdcr_el3;
pub use mdcr_el3::MDCR_EL3;
mod vbar_el3;
pub use vbar_el3::VBAR_EL3;

// AArch64 virtualization registers
mod vpidr_el2;
pub use vpidr_el2::VPIDR_EL2;
mod cptr_el2;
pub use cptr_el2::CPTR_EL2;
mod vmpidr_el2;
pub use vmpidr_el2::VMPIDR_EL2;
mod sctlr_el2;
pub use sctlr_el2::SCTLR_EL2;
mod mdcr_el2;
pub use mdcr_el2::MDCR_EL2;
mod dacr32_el2;
pub use dacr32_el2::DACR32_EL2;
mod hcr_el2;
pub use hcr_el2::HCR_EL2;
mod hstr_el2;
pub use hstr_el2::HSTR_EL2;

// AArch64 GIC system registers
mod icc_sre_el1;
pub use icc_sre_el1::ICC_SRE_EL1;
mod icc_sre_el2;
pub use icc_sre_el2::ICC_SRE_EL2;
mod icc_sre_el3;
pub use icc_sre_el3::ICC_SRE_EL3;

// AArch64 implementation defined registers
mod l2actlr_el1;
pub use l2actlr_el1::L2ACTLR_EL1;
mod l2ctlr_el1;
pub use l2ctlr_el1::L2CTLR_EL1;
mod l2ectlr_el1;
pub use l2ectlr_el1::L2ECTLR_EL1;
mod l2merrsr_el1;
pub use l2merrsr_el1::L2MERRSR_EL1;
mod cpuactlr_el1;
pub use cpuactlr_el1::CPUACTLR_EL1;
mod cpuectlr_el1;
pub use cpuectlr_el1::CPUECTLR_EL1;
mod cpumerrsr_el1;
pub use cpumerrsr_el1::CPUMERRSR_EL1;

// AArch64 performance monitor registers
mod pmccfiltr_el0;
pub use pmccfiltr_el0::PMCCFILTR_EL0;
mod pmccntr_el0;
pub use pmccntr_el0::PMCCNTR_EL0;
mod pmceid0_el0;
pub use pmceid0_el0::PMCEID0_EL0;
mod pmceid1_el0;
pub use pmceid1_el0::PMCEID1_EL0;
mod pmcntenclr_el0;
pub use pmcntenclr_el0::PMCNTENCLR_EL0;
mod pmcntenset_el0;
pub use pmcntenset_el0::PMCNTENSET_EL0;
mod pmcr_el0;
pub use pmcr_el0::PMCR_EL0;
mod pmintenclr_el1;
pub use pmintenclr_el1::PMINTENCLR_EL1;
mod pmintenset_el1;
pub use pmintenset_el1::PMINTENSET_EL1;
mod pmovsclr_el0;
pub use pmovsclr_el0::PMOVSCLR_EL0;
mod pmovsset_el0;
pub use pmovsset_el0::PMOVSSET_EL0;
mod pmselr_el0;
pub use pmselr_el0::PMSELR_EL0;
mod pmswinc_el0;
pub use pmswinc_el0::PMSWINC_EL0;
mod pmuserenr_el0;
pub use pmuserenr_el0::PMUSERENR_EL0;
mod pmxevcntr_el0;
pub use pmxevcntr_el0::PMXEVCNTR_EL0;
mod pmxevtyper_el0;
pub use pmxevtyper_el0::PMXEVTYPER_EL0;
