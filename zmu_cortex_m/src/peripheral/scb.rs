//!
//! Cortex System Control Block Simulation
//!

use crate::core::bits::Bits;
use crate::Processor;

///
/// Register based API to SCB
///
pub trait SystemControlBlock {
    ///
    /// Read Interrupt Control and State Register
    ///
    fn read_icsr(&self) -> u32;

    ///
    /// Write Interrupt Control and State Register
    ///
    fn write_icsr(&mut self, value: u32);

    ///
    /// Write Vector Table Offset
    ///
    fn write_vtor(&mut self, value: u32);

    ///
    /// Write System Handler Priority Register 3
    ///
    fn write_shpr3(&mut self, value: u32);

    ///
    /// Read System Handler Priority Register 3
    ///
    fn read_shpr3(&self) -> u32;

    ///
    /// Write System Handler Priority Register 3, 8-bit access
    ///
    fn write_shpr3_u8(&mut self, offset: u8, value: u8);

    ///
    /// Write System Control Register
    ///
    fn write_scr(&mut self, value: u32);

    ///
    /// Write Debug Exception and Monitor Control Register
    ///
    fn write_demcr(&mut self, value: u32);

    ///
    /// Read Debug Exception and Monitor Control Register
    ///
    fn read_demcr(&self) -> u32;

    ///
    /// Read Vector Table Offset
    ///
    fn read_vtor(&self) -> u32;

    ///
    /// Read System Control Register
    ///
    fn read_scr(&self) -> u32;
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
