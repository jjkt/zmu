//!
//! Cortex Debug and Trace unit simulation
//!

use crate::core::bits::Bits;
use crate::Processor;

/// Register API to Debug and Trace peripheral
pub trait Dwt {
    ///
    /// write ctrl register value
    ///
    fn write_ctrl(&mut self, value: u32);

    ///
    /// write cycle counter value
    ///
    fn write_cyccnt(&mut self, value: u32);

    ///
    /// clock dwt block
    ///
    fn dwt_tick(&mut self);
}

impl Dwt for Processor {
    fn write_ctrl(&mut self, value: u32) {
        self.dwt_ctrl.set_bits(16..23, value.get_bits(16..23));
        self.dwt_ctrl.set_bits(0..13, value.get_bits(0..13));
    }

    fn write_cyccnt(&mut self, value: u32) {
        self.dwt_cyccnt = value;
    }

    fn dwt_tick(&mut self) {
        if self.dwt_ctrl.get_bit(0) {
            self.dwt_cyccnt += 1;
        }
    }
}
