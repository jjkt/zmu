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
    /// Clock dwt block ```cycles```.
    ///
    ///
    fn dwt_tick(&mut self, cycles: u32);
}

impl Dwt for Processor {
    fn write_ctrl(&mut self, value: u32) {
        self.dwt_ctrl.set_bits(16..23, value.get_bits(16..23));
        self.dwt_ctrl.set_bits(0..13, value.get_bits(0..13));
    }

    fn write_cyccnt(&mut self, value: u32) {
        self.dwt_cyccnt = value;
    }

    #[inline(always)]
    fn dwt_tick(&mut self, cycles: u32) {
        self.dwt_cyccnt += cycles * (self.dwt_ctrl & 1);
    }
}
