use tock_registers::interfaces::Writeable;
use crate::register::*;

/// Secure User Debug Enable
pub unsafe fn init() {
    // Non-invasive debug permitted in Secure EL0 mode.
    SDER32_EL3.write(SDER32_EL3::SUNIDEN::Permitted);
    
    // Invasive debug permitted in Secure EL0 mode.
    SDER32_EL3.write(SDER32_EL3::SUIDEN::Permitted);
}