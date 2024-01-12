// not used std and main interface
#![no_std]
#![no_main]
#![allow(dead_code)]

// add startup code
use core::arch::global_asm;

global_asm!(include_str!("asm/vectors.s"));
global_asm!(include_str!("asm/startup.s"));

// override the default panic
mod panic;

// export rcore interface
use rcore::*;
//use rcore::*;

// Entry point for this program.
#[no_mangle]
pub unsafe extern fn _early_init() {
    cache::icache::invalidate_all();
    cache::dcache::flush_all();
    cache::dcache::flush_range(0x15, 128);

    pmu::init();
    pmu::cycle::enable();
    pmu::event::enable(3);

    loop {
        
    }
}
