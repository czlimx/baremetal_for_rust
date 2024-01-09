
macro_rules! _register_read_raw {
    ($types:ty, $instruction:tt, $register:tt, $bit:tt) => {
        /// Reads the raw bits of the CPU register.
        #[inline]
        fn get(&self) -> $types {
            let reg;
            unsafe {
                core::arch::asm!(concat!($instruction, " {reg:", $bit, "}, ", $register), reg = out(reg) reg, options(nomem, nostack));
            }
            reg
        }
    };
}

macro_rules! _register_write_raw {
    ($types:ty, $instruction:tt, $register:tt, $bit:tt) => {
        /// Writes raw bits to the CPU register.
        #[inline]
        fn set(&self, value: $types) {
            unsafe {
                core::arch::asm!(concat!($instruction, " ", $register, ", {reg:", $bit, "}"), reg = in(reg) value, options(nomem, nostack))
            }
        }
    };
}

/// Raw read from system coprocessor registers.
macro_rules! sys_register_read_raw {
    ($width:ty, $asm_reg_name:tt, $asm_width:tt) => {
        _register_read_raw!($width, "mrs", $asm_reg_name, $asm_width);
    };
}

/// Raw write to system coprocessor registers.
macro_rules! sys_register_write_raw {
    ($width:ty, $asm_reg_name:tt, $asm_width:tt) => {
        _register_write_raw!($width, "msr", $asm_reg_name, $asm_width);
    };
}

/*
/// Raw read from (ordinary) registers.
macro_rules! mem_register_read_raw {
    ($width:ty, $asm_reg_name:tt, $asm_width:tt) => {
        _register_read_raw!($width, "mov", $asm_reg_name, $asm_width);
    };
}
/// Raw write to (ordinary) registers.
macro_rules! mem_register_write_raw {
    ($width:ty, $asm_reg_name:tt, $asm_width:tt) => {
        _register_write_raw!($width, "mov", $asm_reg_name, $asm_width);
    };
}
*/