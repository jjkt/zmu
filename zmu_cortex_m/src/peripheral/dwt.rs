//!
//! Cortex Debug and Trace unit simulation
//!

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
}

impl Dwt for Processor {
    fn write_ctrl(&mut self, value: u32) {
        self.dwt_ctrl = value;
    }

    fn write_cyccnt(&mut self, value: u32) {
        self.dwt_cyccnt = value;
    }
}
