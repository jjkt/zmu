use crate::bus::system_region::ppb::PrivatePeripheralBus;

pub trait Dwt {
    fn write_ctrl(&mut self, value: u32);
    fn write_cyccnt(&mut self, value: u32);
    fn read_ctrl(&self) -> u32;
    fn read_cyccnt(&self) -> u32;
}

impl Dwt for PrivatePeripheralBus {
    fn write_ctrl(&mut self, value: u32) {
        self.ctrl = value;
    }

    fn write_cyccnt(&mut self, value: u32) {
        self.cyccnt = value;
    }

    fn read_ctrl(&self) -> u32 {
        self.ctrl
    }

    fn read_cyccnt(&self) -> u32 {
        self.cyccnt
    }
}
