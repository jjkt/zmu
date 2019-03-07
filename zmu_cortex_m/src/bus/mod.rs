//!
//! Processor Bus related operations
//! 

use crate::Processor;

use crate::peripheral::dwt::Dwt;
use crate::peripheral::itm::InstrumentationTraceMacrocell;
use crate::peripheral::nvic::NVIC;
use crate::peripheral::scb::SystemControlBlock;
use crate::peripheral::systick::SysTick;

///
/// Trait for reading and writing via a memory bus.
/// 
pub trait Bus {
    /// Reads a 32 bit value via the bus from the given address.
    ///
    fn read32(&self, addr: u32) -> u32;

    /// Reads a 16 bit value via the bus from the given address.
    ///
    fn read16(&self, addr: u32) -> u16;

    /// Reads a 8 bit value via the bus from the given address.
    ///
    fn read8(&self, addr: u32) -> u8;

    /// Writes a 32 bit value to the bus targeting the given address.
    ///
    fn write32(&mut self, addr: u32, value: u32);

    /// Writes a 16 bit value to the bus targeting the given address.
    ///
    fn write16(&mut self, addr: u32, value: u16);

    /// Writes a 8 bit value to the bus targeting the given address.
    ///
    fn write8(&mut self, addr: u32, value: u8);

    /// Checks if given address can be reached via the bus.
    ///
    fn in_range(&self, addr: u32) -> bool;
}

/*const PPB_START: u32 = 0xE000_0000;
const PPB_END: u32 = 0xE00F_FFFF;
const SYSTEM_REGION_START: u32 = 0xE000_0000;
const SYSTEM_REGION_END: u32 = 0xF000_0000 - 1;
*/

impl Bus for Processor {
    fn read8(&self, addr: u32) -> u8 {
        if self.code.in_range(addr) {
            self.code.read8(addr)
        } else if self.sram.in_range(addr) {
            self.sram.read8(addr)
        } else {
            panic!("bus access fault read8 addr 0x{:x}", addr);
        }
    }

    fn read16(&self, addr: u32) -> u16 {
        /*
        FIXME: LDR{S}H{T}, STRH{T} support non-halfword aligned access.
        FIXME: TBH support non-hw aligned access
        FIXME: LDR{T}, STR{T} support non-hw aligned access

        if addr & 1 == 1 {
            panic!("unaliged read16 addr 0x{:x}", addr);
        }*/

        if self.code.in_range(addr) {
            self.code.read16(addr)
        } else if self.sram.in_range(addr) {
            self.sram.read16(addr)
        } else {
            panic!("bus access fault read16 addr 0x{:x}", addr);
        }
    }

    fn read32(&self, addr: u32) -> u32 {
        match addr {
            0xE000_0000 => self.read_stim0(),

            0xE000_1004 => self.dwt_cyccnt,

            0xE000_E004 => self.ictr,
            0xE000_E008 => self.actlr,
            0xE000_E010 => self.read_syst_csr(),
            0xE000_E014 => self.read_syst_rvr(),
            0xE000_E018 => self.read_syst_cvr(),
            0xE000_E01C => self.read_syst_calib(),

            0xE000_ED00 => self.cpuid,
            0xE000_ED04 => self.read_icsr(),
            0xE000_ED08 => self.read_vtor(),
            0xE000_ED0C => self.aircr,
            0xE000_ED10 => self.read_scr(),
            0xE000_ED14 => self.ccr,
            0xE000_ED18 => self.shpr1,
            0xE000_ED1C => self.shpr2,
            0xE000_ED20 => self.read_shpr3(),
            0xE000_ED24 => self.shcsr,
            0xE000_ED28 => self.cfsr,
            0xE000_ED2C => self.hfsr,
            0xE000_ED30 => self.dfsr,
            0xE000_ED34 => self.mmfar,
            0xE000_ED38 => self.bfar,
            0xE000_ED3C => self.afsr,

            0xE000_ED88 => self.cpacr,

            0xE000_EF34 => self.fpccr,
            0xE000_EF38 => self.fpcar,
            0xE000_EF3C => self.fpdscr,

            0xE000_EF40 => self.mvfr0,
            0xE000_EF44 => self.mvfr1,
            0xE000_EF48 => self.mvfr2,

            0xE000_EDFC => self.read_demcr(),

            // DWT
            0xE000_1000 => self.dwt_ctrl,
            _ => {
                if self.code.in_range(addr) {
                    self.code.read32(addr)
                } else if self.sram.in_range(addr) {
                    self.sram.read32(addr)
                } else {
                    panic!("bus access fault read32 addr 0x{:x}", addr);
                }
            }
        }
        /*
        FIXME: LDR{S}H{T}, STRH{T} support non-halfword aligned access.
        FIXME: TBH support non-hw aligned access
        FIXME: LDR{T}, STR{T} support non-hw aligned access
        if addr & 3 != 0 {
            panic!("unaliged read32 addr 0x{:x}", addr);
        }
        */
    }

    fn write32(&mut self, addr: u32, value: u32) {
        match addr {
            0xE000_0000..=0xE000_007C => {
                self.write_stim_u32(((addr - 0xE000_0000) >> 2) as u8, value)
            }

            0xE000_1000 => self.write_ctrl(value),
            0xE000_1004 => self.write_cyccnt(value),

            0xE000_ED04 => self.write_icsr(value),
            0xE000_ED08 => self.write_vtor(value),
            0xE000_ED10 => self.write_scr(value),
            0xE000_ED20 => self.write_shpr3(value),

            0xE000_EDFC => self.write_demcr(value),

            0xE000_E010 => self.write_syst_csr(value),
            0xE000_E014 => self.write_syst_rvr(value),
            0xE000_E018 => self.write_syst_cvr(value),
            0xE000_E100..=0xE000_E13C => {
                self.nvic_write_iser(((addr - 0xE000_E100) >> 2) as usize, value)
            }
            0xE000_E200..=0xE000_E23C => {
                self.nvic_write_ispr(((addr - 0xE000_E200) >> 2) as usize, value)
            }
            0xE000_E400..=0xE000_E5EC => {
                self.nvic_write_ipr(((addr - 0xE000_E400) >> 2) as usize, value)
            }
            _ => {
                if self.code.in_range(addr) {
                    self.code.write32(addr, value);
                } else if self.sram.in_range(addr) {
                    self.sram.write32(addr, value);
                } else {
                    panic!("bus access fault write addr 0x{:x}", addr);
                }
            }
        }
    }

    fn write16(&mut self, addr: u32, value: u16) {
        match addr {
            0xE000_0000..=0xE000_007C => {
                self.write_stim_u16(((addr - 0xE000_0000) >> 2) as u8, value)
            }
            _ => {
                if self.code.in_range(addr) {
                    self.code.write16(addr, value);
                } else if self.sram.in_range(addr) {
                    self.sram.write16(addr, value);
                } else {
                    panic!("bus access fault write addr 0x{:x}", addr);
                }
            }
        }
    }

    fn write8(&mut self, addr: u32, value: u8) {
        match addr {
            0xE000_0000..=0xE000_007C => {
                self.write_stim_u8(((addr - 0xE000_0000) >> 2) as u8, value)
            }
            0xE000_E400..=0xE000_E5EC => {
                self.nvic_write_ipr_u8(((addr - 0xE000_E400) >> 4) as usize, value)
            }
            0xE000_ED20..=0xE000_ED23 => self.write_shpr3_u8((addr - 0xE000_ED20) as u8, value),

            _ => {
                if self.code.in_range(addr) {
                    self.code.write8(addr, value);
                } else if self.sram.in_range(addr) {
                    self.sram.write8(addr, value);
                } else {
                    panic!("bus access fault write addr 0x{:x}", addr);
                }
            }
        }
    }

    #[allow(unused)]
    fn in_range(&self, addr: u32) -> bool {
        self.code.in_range(addr) || self.sram.in_range(addr)
    }
}
