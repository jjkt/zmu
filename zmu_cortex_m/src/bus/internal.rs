use crate::bus::Bus;
use crate::bus::BusStepResult;
use crate::peripheral::systick::SysTick;
use crate::peripheral::scid::SystemControlAndID;

#[derive(Default)]
struct Dwt {
    pub ctrl: u32,
}

#[derive(Default)]
pub struct InternalBus {
    syst: SysTick,
    scid: SystemControlAndID,
    dwt: Dwt,               
}

impl InternalBus {

    pub fn new() -> InternalBus {
        InternalBus {
            syst: SysTick::default(),
            scid: SystemControlAndID::default(),
            dwt: Dwt { ctrl: 0x4000_0000 },
        }
    }
}

const INTERNAL_BUS_START: u32 = 0xE000_0000;
const INTERNAL_BUS_END: u32 = 0xF000_0000;

impl Bus for InternalBus {
    fn read8(&self, addr: u32) -> u8 {
        panic!("byte access read to system area 0x{:x}", addr);
    }

    fn read16(&self, addr: u32) -> u16 {
        panic!("half-word access read to system area 0x{:x}", addr);
    }

    fn read32(&self, addr: u32) -> u32 {
        match addr {
            0xE000_E010 => self.syst.read_syst_csr(),
            0xE000_E014 => self.syst.read_syst_rvr(),
            0xE000_E018 => self.syst.read_syst_cvr(),
            0xE000_E01C => self.syst.read_syst_calib(),

            0xE000_ED04 => self.scid.read_icsr(),
            0xE000_ED08 => self.scid.read_vtor(),

            // DWT
            0xE000_1000 => self.dwt.ctrl,
            _ => panic!("bus access fault read addr 0x{:x}", addr),
        }
    }

    fn write32(&mut self, addr: u32, value: u32) {
        match addr {
            0xE000_1000 => self.dwt.ctrl = value,
            
            0xE000_ED04 => self.scid.write_icsr(value),
            0xE000_ED08 => self.scid.write_vtor(value),
            0xE000_ED20 => self.scid.write_shpr3(value),

            0xE000_E010 => self.syst.write_syst_csr(value),
            0xE000_E014 => self.syst.write_syst_rvr(value),
            0xE000_E018 => self.syst.write_syst_cvr(value),
            _ => panic!("bus access fault write addr 0x{:x}={}", addr, value),
        }
    }

    fn write16(&mut self, addr: u32, value: u16) {
        panic!(
            "half-word access write to system area 0x{:x}->{}",
            addr, value
        );
    }

    fn write8(&mut self, addr: u32, value: u8) {
        panic!("byte access write to system area 0x{:x}->{}", addr, value);
    }

    fn in_range(&self, addr: u32) -> bool {
        if (addr >= INTERNAL_BUS_START) && (addr < INTERNAL_BUS_END) {
            return true;
        }
        false
    }

    fn step(&mut self) -> BusStepResult {
        self.syst.step()
    }
}
