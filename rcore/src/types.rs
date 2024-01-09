//! Armv8 CPU Core status information

/// Define armv8 execution status data structure
pub enum ExecutionState {
    AArch64,
    AArch32
}

/// Define armv8 privilege level data type
pub enum PrivilegeLevel {
    EL0,
    EL1,
    EL2,
    EL3
}

/// Define armv8 exception routing data type
pub struct ExcepationRoute {
    pub abort:bool,
    pub fiq:bool,
    pub irq:bool,
}
