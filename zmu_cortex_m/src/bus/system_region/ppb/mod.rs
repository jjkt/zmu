pub mod dwt;
pub mod itm;
pub mod nvic;
pub mod scb;
//pub mod mpu;
pub mod systick;

#[cfg(test)]
mod nvic_tests;

use crate::bus::system_region::ppb::dwt::Dwt;
use crate::bus::system_region::ppb::itm::InstrumentationTraceMacrocell;
use crate::bus::system_region::ppb::nvic::NVIC;
use crate::bus::system_region::ppb::scb::SystemControlBlock;
use crate::bus::system_region::ppb::systick::SysTick;
use crate::bus::Bus;
use crate::bus::BusStepResult;
use std::io;

pub struct PrivatePeripheralBus {
    cpuid: u32,
    icsr: u32,
    vtor: u32,
    aircr: u32,
    scr: u32,
    ccr: u32,
    shpr1: u32,
    shpr2: u32,
    shpr3: u32,
    shcsr: u32,
    cfsr: u32,
    hfsr: u32,
    dfsr: u32,
    mmfar: u32,
    bfar: u32,
    afsr: u32,
    cpacr: u32,

    fpccr: u32,
    fpcar: u32,
    fpdscr: u32,

    mvfr0: u32,
    mvfr1: u32,
    mvfr2: u32,

    ictr: u32,
    actlr: u32,

    nvic_interrupt_enabled: [u32; 16],
    nvic_interrupt_pending: [u32; 16],
    nvic_interrupt_active: [u32; 16],

    nvic_interrupt_priority: [u8; 124 * 4],

    dwt_ctrl: u32,
    dwt_cyccnt: u32,

    syst_rvr: u32,
    syst_cvr: u32,
    syst_csr: u32,

    itm_file: Option<Box<io::Write + 'static>>,
}

impl PrivatePeripheralBus {
    pub fn new(itm_file: Option<Box<io::Write + 'static>>) -> PrivatePeripheralBus {
        PrivatePeripheralBus {
            cpuid: 0,
            icsr: 0,
            vtor: 0,
            aircr: 0,
            scr: 0,
            ccr: 0,
            shpr1: 0,
            shpr2: 0,
            shpr3: 0,
            shcsr: 0,
            cfsr: 0,
            dfsr: 0,
            hfsr: 0,
            mmfar: 0,
            bfar: 0,
            afsr: 0,
            cpacr: 0,

            fpccr: 0,
            fpcar: 0,
            fpdscr: 0,
            mvfr0: 0,
            mvfr1: 0,
            mvfr2: 0,

            ictr: 0,
            actlr: 0,

            itm_file: itm_file,
            dwt_ctrl: 0x4000_0000,
            dwt_cyccnt: 0,

            nvic_interrupt_enabled: [0; 16],
            nvic_interrupt_pending: [0; 16],
            nvic_interrupt_active: [0; 16],
            nvic_interrupt_priority: [0; 124 * 4],

            //nvic_exception_pending: 0,
            //nvic_exception_active: 0,
            syst_rvr: 0,
            syst_cvr: 0,
            syst_csr: 0,
        }
    }
}

const PPB_START: u32 = 0xE000_0000;
const PPB_END: u32 = 0xE00F_FFFF;

impl Bus for PrivatePeripheralBus {
    fn read8(&self, addr: u32) -> u8 {
        panic!("byte access read to system area 0x{:x}", addr);
    }

    fn read16(&self, addr: u32) -> u16 {
        panic!("half-word access read to system area 0x{:x}", addr);
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
            _ => panic!("bus access fault read addr 0x{:x}", addr),
        }
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
            _ => panic!(
                "unsupported u32 access write to system area 0x{:x}->{}",
                addr, value
            ),
        }
    }

    fn write16(&mut self, addr: u32, value: u16) {
        match addr {
            0xE000_0000..=0xE000_007C => {
                self.write_stim_u16(((addr - 0xE000_0000) >> 2) as u8, value)
            }
            _ => panic!(
                "unsupported u16 access write to system area 0x{:x}->{}",
                addr, value
            ),
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

            _ => panic!(
                "unsupported u8 access write to system area 0x{:x}->{}",
                addr, value
            ),
        }
    }

    fn in_range(&self, addr: u32) -> bool {
        (addr >= PPB_START) && (addr <= PPB_END)
    }

    fn step(&mut self) -> BusStepResult {
        /*if let BusStepResult::Exception { exception } = self.syst_step() {
            self.nvic_set_pend(exception)
        };

        self.nvic_step()*/

        BusStepResult::Nothing
    }
}
