//!
//! Generic device (no peripherals)
//!
//!

use crate::bus::Bus;
use crate::core::fault::Fault;

///
///
pub struct Device {}

impl Device {
    ///
    ///
    pub fn new() -> Self {
        Self {}
    }
}

impl Bus for Device {
    fn read8(&self, _bus_addr: u32) -> Result<u8, Fault> {
        Ok(0)
    }

    fn read16(&self, _bus_addr: u32) -> Result<u16, Fault> {
        Ok(0)
    }

    fn read32(&mut self, _bus_addr: u32) -> Result<u32, Fault> {
        Ok(0)
    }

    fn write32(&mut self, _addr: u32, _value: u32) -> Result<(), Fault> {
        Ok(())
    }

    fn write16(&mut self, _addr: u32, _value: u16) -> Result<(), Fault> {
        Ok(())
    }

    fn write8(&mut self, _addr: u32, _value: u8) -> Result<(), Fault> {
        Ok(())
    }

    #[allow(unused)]
    fn in_range(&self, _addr: u32) -> bool {
        false
    }
}
