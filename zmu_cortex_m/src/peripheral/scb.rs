use crate::core::bits::Bits;
use crate::Processor;

pub trait SystemControlBlock {
    fn read_icsr(&self) -> u32;
    fn write_icsr(&mut self, value: u32);
    fn write_vtor(&mut self, value: u32);
    fn write_shpr3(&mut self, value: u32);
    fn write_shpr3_u8(&mut self, offset: u8, value: u8);
    fn write_scr(&mut self, value: u32);
    fn write_demcr(&mut self, value: u32);
    fn read_shpr3(&self) -> u32;
    fn read_scr(&self) -> u32;
    fn read_vtor(&self) -> u32;
    fn read_demcr(&self) -> u32;
}

impl SystemControlBlock for Processor {
    fn read_icsr(&self) -> u32 {
        self.icsr
    }

    fn write_icsr(&mut self, value: u32) {
        self.icsr = value
    }

    fn write_vtor(&mut self, value: u32) {
        self.vtor = value
    }

    fn write_shpr3(&mut self, value: u32) {
        self.shpr3 = value
    }
    fn write_shpr3_u8(&mut self, offset: u8, value: u8) {
        let lowbits = (offset * 8) as usize;
        self.shpr3.set_bits(lowbits..(lowbits + 8), value.into());
    }

    fn write_scr(&mut self, value: u32) {
        self.scr = value
    }

    fn write_demcr(&mut self, _value: u32) {}

    fn read_shpr3(&self) -> u32 {
        self.shpr3
    }

    fn read_scr(&self) -> u32 {
        0
    }
    fn read_vtor(&self) -> u32 {
        self.vtor
    }

    fn read_demcr(&self) -> u32 {
        0
    }
}
