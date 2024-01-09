use tock_registers::interfaces::Writeable;
use crate::register::*;

/// Enable Secure instruction fetches
unsafe fn el3_init () {
    // Secure state instruction fetches from Non-secure memory are permitted.
    SCR_EL3.write(SCR_EL3::SIF::Permitted);
}

/// Setting Virtualized cache behavior
unsafe fn el2_init () {
    // Has no effect on stage 2 EL1/EL0 translation regime for instruction
    // accesses.
    HCR_EL2.write(HCR_EL2::ID::NoEffect);

    // Has no effect on stage 2 EL1/EL0 translation regime for data access or 
    // translation table walks.
    HCR_EL2.write(HCR_EL2::CD::NoEffect);

    // SCTLR_EL1.M and HCR_EL2.VM As a reference setting.
    HCR_EL2.write(HCR_EL2::DC::Disable);

    // Non-secure EL1 or EL0 barrier executed No effect.
    HCR_EL2.write(HCR_EL2::BSU::NoEffect);

    // Instructions are not broadcast.
    HCR_EL2.write(HCR_EL2::FB::NotBroadcast);

    // DC ISW is treated as DC CISW.
    HCR_EL2.write(HCR_EL2::SWIO::Enable);
}

/// Initialize Cache related features under different ELx. 
pub unsafe fn init () {

    // Initialize cache behavior
    el3_init();
    el2_init();

    // Enable hardware management of data coherency with other cores 
    // in the cluster.
    CPUECTLR_EL1.write(CPUECTLR_EL1::SMPEN::Enable);
    core::arch::asm!("isb");
}

/// Instruction Cache related features
pub mod icache {
    use tock_registers::interfaces::Readable;
    use tock_registers::interfaces::Writeable;
    use crate::register::*;
    use crate::types::*;

    /// enable Instruction cache
    pub unsafe fn enable (elx:PrivilegeLevel) {
        match elx {
            PrivilegeLevel::EL3 => SCTLR_EL1.write(SCTLR_EL1::I::Enable),
            PrivilegeLevel::EL2 => SCTLR_EL1.write(SCTLR_EL1::I::Enable),
            _ => SCTLR_EL1.write(SCTLR_EL1::I::Enable)
        }
    }

    /// disable Instruction cache
    pub unsafe fn disable (elx:PrivilegeLevel) {
        match elx {
            PrivilegeLevel::EL3 => SCTLR_EL1.write(SCTLR_EL1::I::Disable),
            PrivilegeLevel::EL2 => SCTLR_EL1.write(SCTLR_EL1::I::Disable),
            _ => SCTLR_EL1.write(SCTLR_EL1::I::Disable)
        }
    }

    /// Invalidate instructions caches by virtual address
    pub unsafe fn invalidate_range (start:u64, size:u64) {
        // get the minimum D-cache line size
        let min:u64 = CTR_EL0.read(CTR_EL0::IminLine) << 4;

        // align start and end address
        let mut curr:u64 = start & min;
        let ends:u64 = curr + size;

        while curr < ends {
            // Instruction cache invalidate by virtual address (VA) to PoU
            core::arch::asm!("ic ivau, {0}", in(reg) curr,);
            curr += min;
        }

        // ensure ordering with previous memory accesses
        core::arch::asm!("isb sy");
    }

    /// Invalidate all instructions caches
    pub unsafe fn invalidate_all () {
        // ensure ordering with previous memory accesses
        core::arch::asm!("isb sy");

        // Instruction cache invalidate all to PoUa Inner Shareable
        core::arch::asm!("ic ialluis");

        // ensure ordering with previous memory accesses
        core::arch::asm!("isb sy");
    }
}

/// Data Cache related features
pub mod dcache {
    use tock_registers::interfaces::Readable;
    use tock_registers::interfaces::Writeable;
    use crate::register::*;
    use crate::types::*;
    use crate::system::*;

    /// enable Data cache
    pub unsafe fn enable (elx:PrivilegeLevel) {
        match elx {
            PrivilegeLevel::EL3 => SCTLR_EL1.write(SCTLR_EL1::C::Enable),
            PrivilegeLevel::EL2 => SCTLR_EL1.write(SCTLR_EL1::C::Enable),
            _ => SCTLR_EL1.write(SCTLR_EL1::C::Enable)
        }
    }

    /// disable Data cache
    pub unsafe fn disable (elx:PrivilegeLevel) {
        match elx {
            PrivilegeLevel::EL3 => SCTLR_EL1.write(SCTLR_EL1::C::Disable),
            PrivilegeLevel::EL2 => SCTLR_EL1.write(SCTLR_EL1::C::Disable),
            _ => SCTLR_EL1.write(SCTLR_EL1::C::Disable)
        }
    }

    /// Flush all data and unified caches for Set/Way
    pub unsafe fn flush_all () {
        // ensure ordering with previous memory accesses
        core::arch::asm!("dsb sy");

        // get the LOC:Indicates the Level of Coherency for the cache hierarchy
        let loc: u64 = CLIDR_EL1.read(CLIDR_EL1::LoC);
        for i in 0..loc {
            // just clean data or unified cache
            let selection:u64 = (i << 1) + 0;
            CSSELR_EL1.set(selection);

            // ensure ordering with previous memory accesses
            core::arch::asm!("isb sy");

            // get the number of sets in cache
            let set:u64 = CCSIDR_EL1.read(CCSIDR_EL1::NumSets);

            // get the associativity of cache
            let way:u64 = CCSIDR_EL1.read(CCSIDR_EL1::Associativity);

            // get the set offset
            let set_offset:u64 = CCSIDR_EL1.read(CCSIDR_EL1::LineSize) + 4u64;

            // get the way offset
            let way_offset:u64 = clz(32, way);
            for x in (0..=set).rev()  {
                for y in (0..=way).rev()  {
                    let value:u64 = (y << way_offset) | (x << set_offset) | selection;
                    // Clean and Invalidate data cache by set/way.
                    core::arch::asm!("dc cisw, {0}", in(reg) value,);
                }
            }
        }

        // ensure ordering with previous memory accesses
        core::arch::asm!("dsb sy", "isb");
    }

    /// Clean all data and unified caches for Set/Way
    pub unsafe fn clean_all () {
        // ensure ordering with previous memory accesses
        core::arch::asm!("dsb sy");

        // get the LOC:Indicates the Level of Coherency for the cache hierarchy
        let loc: u64 = CLIDR_EL1.read(CLIDR_EL1::LoC);
        for i in 0..loc {
            // just clean data or unified cache
            let selection:u64 = (i << 1) + 0;
            CSSELR_EL1.set(selection);

            // ensure ordering with previous memory accesses
            core::arch::asm!("isb sy");

            // get the number of sets in cache
            let set:u64 = CCSIDR_EL1.read(CCSIDR_EL1::NumSets);

            // get the associativity of cache
            let way:u64 = CCSIDR_EL1.read(CCSIDR_EL1::Associativity);

            // get the set offset
            let set_offset:u64 = CCSIDR_EL1.read(CCSIDR_EL1::LineSize) + 4u64;

            // get the way offset
            let way_offset:u64 = clz(32, way);
            for x in (0..=set).rev()  {
                for y in (0..=way).rev()  {
                    let value:u64 = (y << way_offset) | (x << set_offset) | selection;
                    // Clean and Invalidate data cache by set/way.
                    core::arch::asm!("dc csw, {0}", in(reg) value,);
                }
            }
        }

        // ensure ordering with previous memory accesses
        core::arch::asm!("dsb sy", "isb");
    }

    /// Invalidate all data and unified caches for Set/Way
    pub unsafe fn invalidate_all () {
        // ensure ordering with previous memory accesses
        core::arch::asm!("dsb sy");

        // get the LOC:Indicates the Level of Coherency for the cache hierarchy
        let loc: u64 = CLIDR_EL1.read(CLIDR_EL1::LoC);
        for i in 0..loc {
            // just clean data or unified cache
            let selection:u64 = (i << 1) + 0;
            CSSELR_EL1.set(selection);

            // ensure ordering with previous memory accesses
            core::arch::asm!("isb sy");

            // get the number of sets in cache
            let set:u64 = CCSIDR_EL1.read(CCSIDR_EL1::NumSets);

            // get the associativity of cache
            let way:u64 = CCSIDR_EL1.read(CCSIDR_EL1::Associativity);

            // get the set offset
            let set_offset:u64 = CCSIDR_EL1.read(CCSIDR_EL1::LineSize) + 4u64;

            // get the way offset
            let way_offset:u64 = clz(32, way);
            for x in (0..=set).rev()  {
                for y in (0..=way).rev()  {
                    let value:u64 = (y << way_offset) | (x << set_offset) | selection;
                    // Clean and Invalidate data cache by set/way.
                    core::arch::asm!("dc isw, {0}", in(reg) value,);
                }
            }
        }

        // ensure ordering with previous memory accesses
        core::arch::asm!("dsb sy", "isb");
    }

    /// Invalidate data and unified caches by virtual address
    pub unsafe fn invalidate_range (start:u64, size:u64) {
        // get the minimum D-cache line size
        let min:u64 = CTR_EL0.read(CTR_EL0::DminLine) << 4;

        // align start and end address
        let mut curr:u64 = start & min;
        let ends:u64 = curr + size;

        while curr < ends {
            // Invalidate data cache by virtual address.
            core::arch::asm!("dc ivac, {0}", in(reg) curr,);
            curr += min;
        }

        // ensure ordering with previous memory accesses
        core::arch::asm!("dsb sy");
    }

    /// Clean data and unified caches by virtual address
    pub unsafe fn clean_range (start:u64, size:u64) {
        // get the minimum D-cache line size
        let min:u64 = CTR_EL0.read(CTR_EL0::DminLine) << 4;

        // align start and end address
        let mut curr:u64 = start & min;
        let ends:u64 = curr + size;

        while curr < ends {
            // Clean data cache by virtual address.
            core::arch::asm!("dc cvac, {0}", in(reg) curr,);
            curr += min;
        }

        // ensure ordering with previous memory accesses
        core::arch::asm!("dsb sy");
    }

    /// Flush data and unified caches by virtual address
    pub unsafe fn flush_range (start:u64, size:u64) {
        // get the minimum D-cache line size
        let min:u64 = CTR_EL0.read(CTR_EL0::DminLine) << 4;

        // align start and end address
        let mut curr:u64 = start & min;
        let ends:u64 = curr + size;

        while curr < ends {
            // Clean and Invalidate data cache by virtual address.
            core::arch::asm!("dc civac, {0}", in(reg) curr,);
            curr += min;
        }

        // ensure ordering with previous memory accesses
        core::arch::asm!("dsb sy");
    }
}