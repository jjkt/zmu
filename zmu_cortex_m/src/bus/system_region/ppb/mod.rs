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

//
//TODO: Move this to NVIC
#[derive(Clone, Copy, Debug, PartialEq)]
enum ExceptionState {
    Inactive,
    Pending,
    Active,
    ActivePending,
}

pub struct PrivatePeripheralBus {
    ctrl: u32,
    cyccnt: u32,

    //nvic_iser: [u32; 16],
    exception_state: [ExceptionState; 32],
    pending_count: u8,

    icsr: u32,
    vtor: u32, // TODO

    //shpr1: u32,
    //shpr2: u32,
    shpr3: u32,
    scr: u32,

    syst_rvr: u32,
    syst_cvr: u32,
    syst_csr: u32,

    itm_file: Option<Box<io::Write + 'static>>,
}

impl PrivatePeripheralBus {
    pub fn new(itm_file: Option<Box<io::Write + 'static>>) -> PrivatePeripheralBus {
        PrivatePeripheralBus {
            itm_file: itm_file,
            ctrl: 0x4000_0000,
            cyccnt: 0,
            exception_state: [ExceptionState::Inactive; 32],
            pending_count: 0,
            icsr: 0,
            vtor: 0,
            shpr3: 0,
            scr: 0,
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

            0xE000_1004 => self.read_cyccnt(),

            0xE000_E010 => self.read_syst_csr(),
            0xE000_E014 => self.read_syst_rvr(),
            0xE000_E018 => self.read_syst_cvr(),
            0xE000_E01C => self.read_syst_calib(),

            0xE000_ED04 => self.read_icsr(),
            0xE000_ED08 => self.read_vtor(),
            0xE000_ED10 => self.read_scr(),
            0xE000_ED20 => self.read_shpr3(),

            0xE000_EDFC => self.read_demcr(),

            // DWT
            0xE000_1000 => self.read_ctrl(),
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
            0xE000_E100..=0xE000_E13C => self.write_iser(((addr - 0xE000_E100) >> 2) as u8, value),
            0xE000_E200..=0xE000_E23C => self.write_ispr(((addr - 0xE000_E200) >> 2) as u8, value),
            0xE000_E400..=0xE000_E5EC => self.write_ipr(((addr - 0xE000_E400) >> 2) as u8, value),
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
                self.write_ipr_u8(((addr - 0xE000_E400) >> 2) as u8, value)
            }
            0xE000_ED20..=0xE000_ED23 => self.write_shpr3_u8((addr - 0xE000_ED20) as u8, value),

            _ => panic!(
                "unsupported u8 access write to system area 0x{:x}->{}",
                addr, value
            ),
        }
    }

    fn in_range(&self, addr: u32) -> bool {
        if (addr >= PPB_START) && (addr <= PPB_END) {
            return true;
        }
        false
    }

    fn step(&mut self) -> BusStepResult {
        if let BusStepResult::Exception { exception } = self.syst_step() {
            self.nvic_set_pend(exception)
        };

        self.nvic_step()
    }
}
