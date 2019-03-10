//!
//! Cortex System Control Block Simulation
//!

#[cfg(any(armv7m, armv7em))]
use crate::core::bits::Bits;
#[cfg(any(armv7m, armv7em))]
use crate::core::exception::Exception;
#[cfg(any(armv7m, armv7em))]
use crate::core::exception::ExceptionHandling;
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
    /// Write System Handler Priority Register 1
    ///
    #[cfg(any(armv7m, armv7em))]
    fn write_shpr1(&mut self, value: u32);

    ///
    /// Write System Handler Priority Register 2
    ///
    #[cfg(any(armv7m, armv7em))]
    fn write_shpr2(&mut self, value: u32);

    ///
    /// Write System Handler Priority Register 3
    ///
    #[cfg(any(armv7m, armv7em))]
    fn write_shpr3(&mut self, value: u32);

    ///
    /// Write System Handler Priority Register 1, 8-bit access
    ///
    #[cfg(any(armv7m, armv7em))]
    fn write_shpr1_u8(&mut self, offset: usize, value: u8);

    ///
    /// Write System Handler Priority Register 2, 8-bit access
    ///
    #[cfg(any(armv7m, armv7em))]
    fn write_shpr2_u8(&mut self, offset: usize, value: u8);

    ///
    /// Write System Handler Priority Register 3, 8-bit access
    ///
    #[cfg(any(armv7m, armv7em))]
    fn write_shpr3_u8(&mut self, offset: usize, value: u8);

    ///
    /// Write System Handler Priority Register 1, 16-bit access
    ///
    #[cfg(any(armv7m, armv7em))]
    fn write_shpr1_u16(&mut self, offset: usize, value: u16);

    ///
    /// Write System Handler Priority Register 2, 16-bit access
    ///
    #[cfg(any(armv7m, armv7em))]
    fn write_shpr2_u16(&mut self, offset: usize, value: u16);

    ///
    /// Write System Handler Priority Register 3, 16-bit access
    ///
    #[cfg(any(armv7m, armv7em))]
    fn write_shpr3_u16(&mut self, offset: usize, value: u16);

    ///
    /// Read System Handler Priority Register 1
    ///
    #[cfg(any(armv7m, armv7em))]
    fn read_shpr1(&self) -> u32;

    ///
    /// Read System Handler Priority Register 2
    ///
    #[cfg(any(armv7m, armv7em))]
    fn read_shpr2(&self) -> u32;

    ///
    /// Read System Handler Priority Register 3
    ///
    #[cfg(any(armv7m, armv7em))]
    fn read_shpr3(&self) -> u32;

    ///
    /// Read System Handler Priority Register 1, 8-bit access
    ///
    #[cfg(any(armv7m, armv7em))]
    fn read_shpr1_u8(&self, offset: usize) -> u8;

    ///
    /// Read System Handler Priority Register 2, 8-bit access
    ///
    #[cfg(any(armv7m, armv7em))]
    fn read_shpr2_u8(&self, offset: usize) -> u8;

    ///
    /// Read System Handler Priority Register 3, 8-bit access
    ///
    #[cfg(any(armv7m, armv7em))]
    fn read_shpr3_u8(&self, offset: usize) -> u8;

    ///
    /// Read System Handler Priority Register 1, 16-bit access
    ///
    #[cfg(any(armv7m, armv7em))]
    fn read_shpr1_u16(&self, offset: usize) -> u16;

    ///
    /// Read System Handler Priority Register 2, 16-bit access
    ///
    #[cfg(any(armv7m, armv7em))]
    fn read_shpr2_u16(&self, offset: usize) -> u16;

    ///
    /// Read System Handler Priority Register 3, 16-bit access
    ///
    #[cfg(any(armv7m, armv7em))]
    fn read_shpr3_u16(&self, offset: usize) -> u16;

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

    ///
    /// Write "Software Triggered Interrupt Register"
    ///
    #[cfg(any(armv7m, armv7em))]
    fn write_stir(&mut self, value: u32);
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

    #[cfg(any(armv7m, armv7em))]
    fn write_shpr1(&mut self, value: u32) {
        self.write_shpr1_u8(0, value.get_bits(0..8) as u8);
        self.write_shpr1_u8(1, value.get_bits(8..16) as u8);
        self.write_shpr1_u8(2, value.get_bits(16..24) as u8);
        self.write_shpr1_u8(3, value.get_bits(24..32) as u8);
    }

    #[cfg(any(armv7m, armv7em))]
    fn write_shpr2(&mut self, value: u32) {
        self.write_shpr2_u8(0, value.get_bits(0..8) as u8);
        self.write_shpr2_u8(1, value.get_bits(8..16) as u8);
        self.write_shpr2_u8(2, value.get_bits(16..24) as u8);
        self.write_shpr2_u8(3, value.get_bits(24..32) as u8);
    }

    #[cfg(any(armv7m, armv7em))]
    fn write_shpr3(&mut self, value: u32) {
        self.write_shpr3_u8(0, value.get_bits(0..8) as u8);
        self.write_shpr3_u8(1, value.get_bits(8..16) as u8);
        self.write_shpr3_u8(2, value.get_bits(16..24) as u8);
        self.write_shpr3_u8(3, value.get_bits(24..32) as u8);
    }

    #[cfg(any(armv7m, armv7em))]
    fn write_shpr1_u16(&mut self, offset: usize, value: u16) {
        match offset {
            0 | 1 => {
                let offset_base = offset * 2;
                self.write_shpr1_u8(offset_base, value.get_bits(0..8) as u8);
                self.write_shpr1_u8(offset_base + 1, value.get_bits(8..16) as u8);
            }
            _ => (),
        }
    }

    #[cfg(any(armv7m, armv7em))]
    fn write_shpr2_u16(&mut self, offset: usize, value: u16) {
        match offset {
            0 | 1 => {
                let offset_base = offset * 2;
                self.write_shpr2_u8(offset_base, value.get_bits(0..8) as u8);
                self.write_shpr2_u8(offset_base + 1, value.get_bits(8..16) as u8);
            }
            _ => (),
        }
    }

    #[cfg(any(armv7m, armv7em))]
    fn write_shpr3_u16(&mut self, offset: usize, value: u16) {
        match offset {
            0 | 1 => {
                let offset_base = offset * 2;
                self.write_shpr3_u8(offset_base, value.get_bits(0..8) as u8);
                self.write_shpr3_u8(offset_base + 1, value.get_bits(8..16) as u8);
            }
            _ => (),
        }
    }

    #[cfg(any(armv7m, armv7em))]
    fn write_shpr1_u8(&mut self, offset: usize, value: u8) {
        match offset {
            0 => self.set_exception_priority(Exception::MemoryManagementFault, value),
            1 => self.set_exception_priority(Exception::BusFault, value),
            2 => self.set_exception_priority(Exception::UsageFault, value),
            _ => (),
        }
    }

    #[cfg(any(armv7m, armv7em))]
    fn write_shpr2_u8(&mut self, offset: usize, value: u8) {
        if 3 == offset {
            self.set_exception_priority(Exception::SVCall, value);
        }
    }

    #[cfg(any(armv7m, armv7em))]
    fn write_shpr3_u8(&mut self, offset: usize, value: u8) {
        match offset {
            0 => self.set_exception_priority(Exception::DebugMonitor, value),
            2 => self.set_exception_priority(Exception::PendSV, value),
            3 => self.set_exception_priority(Exception::SysTick, value),
            _ => (),
        }
    }

    fn write_scr(&mut self, value: u32) {
        self.scr = value
    }

    fn write_demcr(&mut self, _value: u32) {}

    #[cfg(any(armv7m, armv7em))]
    fn read_shpr1(&self) -> u32 {
        (u32::from(self.read_shpr1_u8(3)) << 24)
            + (u32::from(self.read_shpr1_u8(2)) << 16)
            + (u32::from(self.read_shpr1_u8(1)) << 8)
            + u32::from(self.read_shpr1_u8(0))
    }

    #[cfg(any(armv7m, armv7em))]
    fn read_shpr2(&self) -> u32 {
        (u32::from(self.read_shpr2_u8(3)) << 24)
            + (u32::from(self.read_shpr2_u8(2)) << 16)
            + (u32::from(self.read_shpr2_u8(1)) << 8)
            + u32::from(self.read_shpr2_u8(0))
    }

    #[cfg(any(armv7m, armv7em))]
    fn read_shpr3(&self) -> u32 {
        (u32::from(self.read_shpr3_u8(3)) << 24)
            + (u32::from(self.read_shpr3_u8(2)) << 16)
            + (u32::from(self.read_shpr3_u8(1)) << 8)
            + u32::from(self.read_shpr3_u8(0))
    }

    #[cfg(any(armv7m, armv7em))]
    fn read_shpr1_u8(&self, offset: usize) -> u8 {
        match offset {
            0 => self.get_exception_priority(Exception::MemoryManagementFault) as u8,
            1 => self.get_exception_priority(Exception::BusFault) as u8,
            2 => self.get_exception_priority(Exception::UsageFault) as u8,
            _ => 0,
        }
    }

    #[cfg(any(armv7m, armv7em))]
    fn read_shpr2_u8(&self, offset: usize) -> u8 {
        match offset {
            3 => self.get_exception_priority(Exception::SVCall) as u8,
            _ => 0,
        }
    }

    #[cfg(any(armv7m, armv7em))]
    fn read_shpr3_u8(&self, offset: usize) -> u8 {
        match offset {
            0 => self.get_exception_priority(Exception::DebugMonitor) as u8,
            2 => self.get_exception_priority(Exception::PendSV) as u8,
            3 => self.get_exception_priority(Exception::SysTick) as u8,
            _ => 0,
        }
    }

    #[cfg(any(armv7m, armv7em))]
    fn read_shpr1_u16(&self, offset: usize) -> u16 {
        match offset {
            0 | 1 => {
                (u16::from(self.read_shpr1_u8((offset * 2) + 1)) << 8)
                    + u16::from(self.read_shpr1_u8(offset * 2))
            }
            _ => 0,
        }
    }

    #[cfg(any(armv7m, armv7em))]
    fn read_shpr2_u16(&self, offset: usize) -> u16 {
        match offset {
            0 | 1 => {
                (u16::from(self.read_shpr2_u8((offset * 2) + 1)) << 8)
                    + u16::from(self.read_shpr2_u8(offset * 2))
            }
            _ => 0,
        }
    }

    #[cfg(any(armv7m, armv7em))]
    fn read_shpr3_u16(&self, offset: usize) -> u16 {
        match offset {
            0 | 1 => {
                (u16::from(self.read_shpr3_u8((offset * 2) + 1)) << 8)
                    + u16::from(self.read_shpr3_u8(offset * 2))
            }
            _ => 0,
        }
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

    #[cfg(any(armv7m, armv7em))]
    fn write_stir(&mut self, value: u32) {
        self.set_exception_pending(Exception::Interrupt {
            n: value.get_bits(0..9) as usize,
        });
    }
}

#[cfg(test)]
#[cfg(any(armv7m, armv7em))]
mod tests {
    use super::*;
    use crate::core::exception::Exception;
    use crate::core::exception::ExceptionHandling;
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
    #[cfg(any(armv7m, armv7em))]
    fn test_shpr_read_write_32() {
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

        // Act
        processor.write_shpr1(0xffeeccbb);
        processor.write_shpr2(0xaa998877);
        processor.write_shpr3(0x66554433);

        // Assert
        assert_eq!(
            processor.get_exception_priority(Exception::UsageFault),
            0xee
        );
        assert_eq!(processor.get_exception_priority(Exception::BusFault), 0xcc);
        assert_eq!(
            processor.get_exception_priority(Exception::MemoryManagementFault),
            0xbb
        );
        assert_eq!(processor.read_shpr1(), 0x00eeccbb);

        assert_eq!(processor.get_exception_priority(Exception::SVCall), 0xaa);

        assert_eq!(processor.read_shpr2(), 0xaa000000);

        assert_eq!(processor.get_exception_priority(Exception::SysTick), 0x66);
        assert_eq!(processor.get_exception_priority(Exception::PendSV), 0x55);
        assert_eq!(
            processor.get_exception_priority(Exception::DebugMonitor),
            0x33
        );

        assert_eq!(processor.read_shpr3(), 0x66550033);
    }

    #[test]
    #[cfg(any(armv7m, armv7em))]
    fn test_shpr_read_write_16() {
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

        // Act
        processor.write_shpr1_u16(0, 0xccbb);
        processor.write_shpr1_u16(1, 0xffee);

        processor.write_shpr2_u16(0, 0x8877);
        processor.write_shpr2_u16(1, 0xaa99);

        processor.write_shpr3_u16(0, 0x4433);
        processor.write_shpr3_u16(1, 0x6655);

        // Assert
        assert_eq!(
            processor.get_exception_priority(Exception::UsageFault),
            0xee
        );
        assert_eq!(processor.get_exception_priority(Exception::BusFault), 0xcc);
        assert_eq!(
            processor.get_exception_priority(Exception::MemoryManagementFault),
            0xbb
        );
        assert_eq!(processor.read_shpr1(), 0x00eeccbb);

        assert_eq!(processor.read_shpr1_u16(0), 0xccbb);
        assert_eq!(processor.read_shpr1_u16(1), 0x00ee);

        assert_eq!(processor.get_exception_priority(Exception::SVCall), 0xaa);

        assert_eq!(processor.read_shpr2(), 0xaa000000);
        assert_eq!(processor.read_shpr2_u16(0), 0x0000);
        assert_eq!(processor.read_shpr2_u16(1), 0xaa00);

        assert_eq!(processor.get_exception_priority(Exception::SysTick), 0x66);
        assert_eq!(processor.get_exception_priority(Exception::PendSV), 0x55);
        assert_eq!(
            processor.get_exception_priority(Exception::DebugMonitor),
            0x33
        );

        assert_eq!(processor.read_shpr3(), 0x66550033);
        assert_eq!(processor.read_shpr3_u16(0), 0x0033);
        assert_eq!(processor.read_shpr3_u16(1), 0x6655);
    }

}
