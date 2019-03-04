use crate::core::Core;

pub trait Dwt {
    fn write_ctrl(&mut self, value: u32);
    fn write_cyccnt(&mut self, value: u32);
}

impl Dwt for Core {
    fn write_ctrl(&mut self, value: u32) {
        self.dwt_ctrl = value;
    }

    fn write_cyccnt(&mut self, value: u32) {
        self.dwt_cyccnt = value;
    }
}
