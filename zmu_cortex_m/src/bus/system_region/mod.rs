pub mod ppb;
use crate::bus::system_region::ppb::PrivatePeripheralBus;
use crate::bus::Bus;
use crate::bus::BusStepResult;
use std::io;

pub struct SystemRegion {
    ppb: PrivatePeripheralBus,
}

impl SystemRegion {
    pub fn new(itm_file: Option<Box<io::Write + 'static>>) -> SystemRegion {
        SystemRegion {
            ppb: PrivatePeripheralBus::new(itm_file),
        }
    }
}

const SYSTEM_REGION_START: u32 = 0xE000_0000;
const SYSTEM_REGION_END: u32 = 0xF000_0000 - 1;

impl Bus for SystemRegion {
    fn read8(&self, addr: u32) -> u8 {
        self.ppb.read8(addr)
    }

    fn read16(&self, addr: u32) -> u16 {
        self.ppb.read16(addr)
    }

    fn read32(&self, addr: u32) -> u32 {
        self.ppb.read32(addr)
    }

    fn write8(&mut self, addr: u32, value: u8) {
        self.ppb.write8(addr, value)
    }

    fn write16(&mut self, addr: u32, value: u16) {
        self.ppb.write16(addr, value)
    }

    fn write32(&mut self, addr: u32, value: u32) {
        self.ppb.write32(addr, value)
    }

    fn in_range(&self, addr: u32) -> bool {
        (addr >= SYSTEM_REGION_START) && (addr <= SYSTEM_REGION_END)
    }

    fn step(&mut self) -> BusStepResult {
        self.ppb.step()
    }
}
