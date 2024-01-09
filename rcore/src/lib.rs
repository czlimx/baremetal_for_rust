//! Cortex-A53 related CPU core mode implementation.

// not used std interface
#![no_std]

/// Define armv8 related architecture data types
pub mod types;

/// Implement Cortex-A53 CPU register bit field information
pub mod register;

/// System related function implementation
pub mod system;

/// Cache related function implementation
pub mod cache;