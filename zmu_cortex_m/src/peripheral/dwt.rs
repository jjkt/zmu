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
    fn dwt_write_ctrl(&mut self, value: u32);

    ///
    /// write cycle counter value
    ///
    fn dwt_write_cyccnt(&mut self, value: u32);

    ///
    /// Clock dwt block ```cycles```.
    ///
    ///
    fn dwt_tick(&mut self, cycles: u32);
}

const DWT_CTRL_CYCCNTENA: u32 = 1 << 0;

impl Dwt for Processor {
    fn dwt_write_ctrl(&mut self, value: u32) {
        self.dwt_ctrl.set_bits(16..23, value.get_bits(16..23));
        self.dwt_ctrl.set_bits(0..13, value.get_bits(0..13));
    }

    fn dwt_write_cyccnt(&mut self, value: u32) {
        self.dwt_cyccnt = value;
    }

    #[inline(always)]
    fn dwt_tick(&mut self, cycles: u32) {
        self.dwt_cyccnt = self.dwt_cyccnt.wrapping_add(cycles * (self.dwt_ctrl & DWT_CTRL_CYCCNTENA));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::reset::Reset;
    use crate::semihosting::SemihostingCommand;
    use crate::semihosting::SemihostingResponse;
    use std::io::Result;
    use std::io::Write;
    struct TestWriter {}

    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }


    #[test]
    fn test_dwt_tick() {
        // Arrange

        let mut processor = Processor::new(
            Some(Box::new(TestWriter {})),
            &[0; 65536],
            Box::new(
                |_semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                    panic!("shoud not happen")
                },
            ),
        );

        // Arrange
        processor.reset().unwrap();
        assert_eq!(processor.dwt_cyccnt, 0);

        // Act
        processor.dwt_write_ctrl(DWT_CTRL_CYCCNTENA);
        processor.dwt_tick(42);

        // Act
        assert_eq!(processor.dwt_cyccnt, 42);
    }

}
