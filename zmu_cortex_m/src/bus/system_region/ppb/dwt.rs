use crate::bus::system_region::ppb::PrivatePeripheralBus;

pub trait Dwt {
    fn write_ctrl(&mut self, value: u32);
    fn write_cyccnt(&mut self, value: u32);
}

impl Dwt for PrivatePeripheralBus {
    fn write_ctrl(&mut self, value: u32) {
        self.dwt_ctrl = value;
    }

    fn write_cyccnt(&mut self, value: u32) {
        self.dwt_cyccnt = value;
    }
}
