//!
//! Cortex System Control Block Simulation
//!

use crate::Processor;
use crate::core::bits::Bits;
use crate::core::exception::Exception;
use crate::core::exception::ExceptionHandling;
use crate::core::fault::{Fault, FaultStatusContext};

use crate::core::register::Ipsr;

pub(crate) const SHCSR_MEMFAULTENA: u32 = 1 << 16;
pub(crate) const SHCSR_BUSFAULTENA: u32 = 1 << 17;
pub(crate) const SHCSR_USGFAULTENA: u32 = 1 << 18;
const SHCSR_ENABLE_MASK: u32 = SHCSR_MEMFAULTENA | SHCSR_BUSFAULTENA | SHCSR_USGFAULTENA;
const SHCSR_MEMFAULTACT: u32 = 1 << 0;
const SHCSR_BUSFAULTACT: u32 = 1 << 1;
const SHCSR_USGFAULTACT: u32 = 1 << 3;
const CFSR_IACCVIOL: u32 = 1 << 0;
const CFSR_DACCVIOL: u32 = 1 << 1;
const CFSR_MSTKERR: u32 = 1 << 4;
const CFSR_MMARVALID: u32 = 1 << 7;
const CFSR_IBUSERR: u32 = 1 << 8;
const CFSR_PRECISERR: u32 = 1 << 9;
const CFSR_UNSTKERR: u32 = 1 << 11;
const CFSR_STKERR: u32 = 1 << 12;
const CFSR_BFARVALID: u32 = 1 << 15;
const CFSR_UNDEFINSTR: u32 = 1 << 16;
const CFSR_INVPC: u32 = 1 << 18;
const HFSR_VECTTBL: u32 = 1 << 1;
pub(crate) const HFSR_FORCED: u32 = 1 << 30;
const HFSR_WRITE_ONE_TO_CLEAR_MASK: u32 = (1 << 1) | HFSR_FORCED | (1 << 31);

impl Processor {
    pub(crate) fn read_shcsr(&self) -> u32 {
        self.shcsr
    }

    #[cfg(any(feature = "fpv4-sp-d16", feature = "fpv5-sp-d16", feature = "fpv5-d16"))]
    pub(crate) fn reset_fp_system_state(&mut self) {
        self.cpacr = 0;
        self.fpccr = 0;
        self.fpcar = 0;
        self.fpdscr = 0;
        self.mvfr0 = 0;
        self.mvfr1 = 0;
        self.mvfr2 = 0;
    }

    #[cfg(not(any(feature = "fpv4-sp-d16", feature = "fpv5-sp-d16", feature = "fpv5-d16")))]
    pub(crate) fn reset_fp_system_state(&mut self) {
        let _ = self;
    }

    pub(crate) fn reset_scb_fault_state(&mut self) {
        self.shcsr = 0;
        self.cfsr = 0;
        self.dfsr = 0;
        self.hfsr = 0;
        self.mmfar = 0;
        self.bfar = 0;
        self.afsr = 0;
    }

    pub(crate) fn set_shcsr_exception_active(&mut self, exception: Exception, active: bool) {
        let bit = match exception {
            Exception::MemoryManagementFault => SHCSR_MEMFAULTACT,
            Exception::BusFault => SHCSR_BUSFAULTACT,
            Exception::UsageFault => SHCSR_USGFAULTACT,
            _ => return,
        };

        if active {
            self.shcsr |= bit;
        } else {
            self.shcsr &= !bit;
        }
    }

    fn latch_mmfar(&mut self, address: u32) {
        self.mmfar = address;
        self.cfsr |= CFSR_MMARVALID;
    }

    fn latch_bfar(&mut self, address: u32) {
        self.bfar = address;
        self.cfsr |= CFSR_BFARVALID;
    }

    pub(crate) fn record_fault_status(&mut self, fault: Fault, status: FaultStatusContext) {
        match fault {
            Fault::IAccViol => {
                self.cfsr |= CFSR_IACCVIOL;
                if let Some(address) = status.fault_address {
                    self.latch_mmfar(address);
                }
            }
            Fault::DAccViol => {
                self.cfsr |= CFSR_DACCVIOL;
                if let Some(address) = status.fault_address {
                    self.latch_mmfar(address);
                }
            }
            Fault::Msunskerr => self.cfsr |= CFSR_UNSTKERR,
            Fault::Mstkerr => self.cfsr |= CFSR_MSTKERR,
            Fault::IBusErr => {
                self.cfsr |= CFSR_IBUSERR;
                if let Some(address) = status.fault_address {
                    self.latch_bfar(address);
                }
            }
            Fault::Preciserr => {
                self.cfsr |= CFSR_PRECISERR;
                if let Some(address) = status.fault_address {
                    self.latch_bfar(address);
                }
            }
            Fault::Stkerr => self.cfsr |= CFSR_STKERR,
            Fault::UndefInstr => self.cfsr |= CFSR_UNDEFINSTR,
            Fault::InvPc => self.cfsr |= CFSR_INVPC,
            Fault::VectorTable => self.hfsr |= HFSR_VECTTBL,
            _ => {}
        }
    }
}

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
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr1(&mut self, value: u32);

    ///
    /// Write System Handler Priority Register 2
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr2(&mut self, value: u32);

    ///
    /// Write System Handler Priority Register 3
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr3(&mut self, value: u32);

    ///
    /// Write System Handler Priority Register 1, 8-bit access
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr1_u8(&mut self, offset: usize, value: u8);

    ///
    /// Write System Handler Priority Register 2, 8-bit access
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr2_u8(&mut self, offset: usize, value: u8);

    ///
    /// Write System Handler Priority Register 3, 8-bit access
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr3_u8(&mut self, offset: usize, value: u8);

    ///
    /// Write System Handler Priority Register 1, 16-bit access
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr1_u16(&mut self, offset: usize, value: u16);

    ///
    /// Write System Handler Priority Register 2, 16-bit access
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr2_u16(&mut self, offset: usize, value: u16);

    ///
    /// Write System Handler Priority Register 3, 16-bit access
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr3_u16(&mut self, offset: usize, value: u16);

    ///
    /// Read System Handler Priority Register 1
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr1(&self) -> u32;

    ///
    /// Read System Handler Priority Register 2
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr2(&self) -> u32;

    ///
    /// Read System Handler Priority Register 3
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr3(&self) -> u32;

    ///
    /// Read System Handler Priority Register 1, 8-bit access
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr1_u8(&self, offset: usize) -> u8;

    ///
    /// Read System Handler Priority Register 2, 8-bit access
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr2_u8(&self, offset: usize) -> u8;

    ///
    /// Read System Handler Priority Register 3, 8-bit access
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr3_u8(&self, offset: usize) -> u8;

    ///
    /// Read System Handler Priority Register 1, 16-bit access
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr1_u16(&self, offset: usize) -> u16;

    ///
    /// Read System Handler Priority Register 2, 16-bit access
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr2_u16(&self, offset: usize) -> u16;

    ///
    /// Read System Handler Priority Register 3, 16-bit access
    ///
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr3_u16(&self, offset: usize) -> u16;

    ///
    /// Write System Control Register
    ///
    fn write_scr(&mut self, value: u32);

    /// Write System Handler Control and State Register.
    fn write_shcsr(&mut self, value: u32);

    /// Read System Handler Control and State Register.
    fn read_shcsr(&self) -> u32;

    /// Write Configurable Fault Status Register.
    fn write_cfsr(&mut self, value: u32);

    /// Write `HardFault Status Register`.
    fn write_hfsr(&mut self, value: u32);

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
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_stir(&mut self, value: u32);
}

impl SystemControlBlock for Processor {
    fn read_icsr(&self) -> u32 {
        let mut value: u32 = 0;

        value.set_bits(0..9, self.psr.get_isr_number() as u32);

        if let Some(exception) = self.get_pending_exception() {
            value.set_bits(12..21, usize::from(exception) as u32);
        }

        value
    }

    fn write_icsr(&mut self, value: u32) {
        if value.get_bit(31) {
            self.set_exception_pending(Exception::NMI);
        }
        if value.get_bit(28) {
            self.set_exception_pending(Exception::PendSV);
        } else if value.get_bit(27) {
            self.clear_pending_exception(Exception::PendSV);
        }
        if value.get_bit(26) {
            self.set_exception_pending(Exception::SysTick);
        } else if value.get_bit(25) {
            self.clear_pending_exception(Exception::SysTick);
        }
    }

    fn write_vtor(&mut self, value: u32) {
        self.vtor = value;
    }

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr1(&mut self, value: u32) {
        self.write_shpr1_u8(0, value.get_bits(0..8) as u8);
        self.write_shpr1_u8(1, value.get_bits(8..16) as u8);
        self.write_shpr1_u8(2, value.get_bits(16..24) as u8);
        self.write_shpr1_u8(3, value.get_bits(24..32) as u8);
    }

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr2(&mut self, value: u32) {
        self.write_shpr2_u8(0, value.get_bits(0..8) as u8);
        self.write_shpr2_u8(1, value.get_bits(8..16) as u8);
        self.write_shpr2_u8(2, value.get_bits(16..24) as u8);
        self.write_shpr2_u8(3, value.get_bits(24..32) as u8);
    }

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr3(&mut self, value: u32) {
        self.write_shpr3_u8(0, value.get_bits(0..8) as u8);
        self.write_shpr3_u8(1, value.get_bits(8..16) as u8);
        self.write_shpr3_u8(2, value.get_bits(16..24) as u8);
        self.write_shpr3_u8(3, value.get_bits(24..32) as u8);
    }

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
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

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
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

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
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

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr1_u8(&mut self, offset: usize, value: u8) {
        match offset {
            0 => self.set_exception_priority(Exception::MemoryManagementFault, value),
            1 => self.set_exception_priority(Exception::BusFault, value),
            2 => self.set_exception_priority(Exception::UsageFault, value),
            _ => (),
        }
    }

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr2_u8(&mut self, offset: usize, value: u8) {
        if 3 == offset {
            self.set_exception_priority(Exception::SVCall, value);
        }
    }

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_shpr3_u8(&mut self, offset: usize, value: u8) {
        match offset {
            0 => self.set_exception_priority(Exception::DebugMonitor, value),
            2 => self.set_exception_priority(Exception::PendSV, value),
            3 => self.set_exception_priority(Exception::SysTick, value),
            _ => (),
        }
    }

    fn write_scr(&mut self, value: u32) {
        self.scr = value;
    }

    fn write_shcsr(&mut self, value: u32) {
        self.shcsr = (self.shcsr & !SHCSR_ENABLE_MASK) | (value & SHCSR_ENABLE_MASK);
    }

    fn read_shcsr(&self) -> u32 {
        Processor::read_shcsr(self)
    }

    fn write_cfsr(&mut self, value: u32) {
        self.cfsr &= !value;
    }

    fn write_hfsr(&mut self, value: u32) {
        self.hfsr &= !(value & HFSR_WRITE_ONE_TO_CLEAR_MASK);
    }

    fn write_demcr(&mut self, _value: u32) {}

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr1(&self) -> u32 {
        (u32::from(self.read_shpr1_u8(3)) << 24)
            + (u32::from(self.read_shpr1_u8(2)) << 16)
            + (u32::from(self.read_shpr1_u8(1)) << 8)
            + u32::from(self.read_shpr1_u8(0))
    }

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr2(&self) -> u32 {
        (u32::from(self.read_shpr2_u8(3)) << 24)
            + (u32::from(self.read_shpr2_u8(2)) << 16)
            + (u32::from(self.read_shpr2_u8(1)) << 8)
            + u32::from(self.read_shpr2_u8(0))
    }

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr3(&self) -> u32 {
        (u32::from(self.read_shpr3_u8(3)) << 24)
            + (u32::from(self.read_shpr3_u8(2)) << 16)
            + (u32::from(self.read_shpr3_u8(1)) << 8)
            + u32::from(self.read_shpr3_u8(0))
    }

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr1_u8(&self, offset: usize) -> u8 {
        match offset {
            0 => self.get_exception_priority(Exception::MemoryManagementFault) as u8,
            1 => self.get_exception_priority(Exception::BusFault) as u8,
            2 => self.get_exception_priority(Exception::UsageFault) as u8,
            _ => 0,
        }
    }

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr2_u8(&self, offset: usize) -> u8 {
        match offset {
            3 => self.get_exception_priority(Exception::SVCall) as u8,
            _ => 0,
        }
    }

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr3_u8(&self, offset: usize) -> u8 {
        match offset {
            0 => self.get_exception_priority(Exception::DebugMonitor) as u8,
            2 => self.get_exception_priority(Exception::PendSV) as u8,
            3 => self.get_exception_priority(Exception::SysTick) as u8,
            _ => 0,
        }
    }

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr1_u16(&self, offset: usize) -> u16 {
        match offset {
            0 | 1 => {
                (u16::from(self.read_shpr1_u8((offset * 2) + 1)) << 8)
                    + u16::from(self.read_shpr1_u8(offset * 2))
            }
            _ => 0,
        }
    }

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn read_shpr2_u16(&self, offset: usize) -> u16 {
        match offset {
            0 | 1 => {
                (u16::from(self.read_shpr2_u8((offset * 2) + 1)) << 8)
                    + u16::from(self.read_shpr2_u8(offset * 2))
            }
            _ => 0,
        }
    }

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
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

    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn write_stir(&mut self, value: u32) {
        self.set_exception_pending(Exception::Interrupt {
            n: value.get_bits(0..9) as usize,
        });
    }
}

#[cfg(test)]
#[cfg(any(feature = "armv7m", feature = "armv7em"))]
mod tests {
    use super::*;
    use crate::bus::Bus;
    use crate::core::exception::Exception;
    use crate::core::exception::ExceptionHandling;
    use crate::core::fault::FaultTrapMode;
    use crate::core::register::{BaseReg, Reg};
    use crate::core::reset::Reset;
    use crate::executor::Executor;

    const SHCSR_MEMFAULTENA: u32 = 1 << 16;
    const SHCSR_BUSFAULTENA: u32 = 1 << 17;
    const SHCSR_USGFAULTENA: u32 = 1 << 18;
    const SHCSR_PRESERVED_RAW_BITS: u32 = 1 << 2;

    const HFSR_VECTTBL: u32 = 1 << 1;
    const HFSR_FORCED: u32 = 1 << 30;
    const HFSR_DEBUGEVT: u32 = 1 << 31;

    fn reset_test_image() -> Box<[u8]> {
        vec![
            0x00, 0x01, 0x00, 0x20, // initial SP = 0x2000_0100
            0x09, 0x00, 0x00, 0x00, // reset vector = 0x0000_0009
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
        .into_boxed_slice()
    }

    #[test]
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn test_shpr_read_write_32() {
        // Arrange
        let mut processor = Processor::new();

        // Act
        processor.write_shpr1(0xffee_ccbb);
        processor.write_shpr2(0xaa99_8877);
        processor.write_shpr3(0x6655_4433);

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
        assert_eq!(processor.read_shpr1(), 0x00ee_ccbb);

        assert_eq!(processor.get_exception_priority(Exception::SVCall), 0xaa);

        assert_eq!(processor.read_shpr2(), 0xaa00_0000);

        assert_eq!(processor.get_exception_priority(Exception::SysTick), 0x66);
        assert_eq!(processor.get_exception_priority(Exception::PendSV), 0x55);
        assert_eq!(
            processor.get_exception_priority(Exception::DebugMonitor),
            0x33
        );

        assert_eq!(processor.read_shpr3(), 0x6655_0033);
    }

    #[test]
    #[cfg(any(feature = "armv7m", feature = "armv7em"))]
    fn test_shpr_read_write_16() {
        // Arrange
        let mut processor = Processor::new();

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
        assert_eq!(processor.read_shpr1(), 0x00ee_ccbb);

        assert_eq!(processor.read_shpr1_u16(0), 0xccbb);
        assert_eq!(processor.read_shpr1_u16(1), 0x00ee);

        assert_eq!(processor.get_exception_priority(Exception::SVCall), 0xaa);

        assert_eq!(processor.read_shpr2(), 0xaa00_0000);
        assert_eq!(processor.read_shpr2_u16(0), 0x0000);
        assert_eq!(processor.read_shpr2_u16(1), 0xaa00);

        assert_eq!(processor.get_exception_priority(Exception::SysTick), 0x66);
        assert_eq!(processor.get_exception_priority(Exception::PendSV), 0x55);
        assert_eq!(
            processor.get_exception_priority(Exception::DebugMonitor),
            0x33
        );

        assert_eq!(processor.read_shpr3(), 0x6655_0033);
        assert_eq!(processor.read_shpr3_u16(0), 0x0033);
        assert_eq!(processor.read_shpr3_u16(1), 0x6655);
    }

    #[test]
    fn test_shcsr_enable_bits_are_writable_via_bus32() {
        let mut processor = Processor::new();

        processor.shcsr = SHCSR_PRESERVED_RAW_BITS;

        processor
            .write32(
                0xE000_ED24,
                SHCSR_MEMFAULTENA | SHCSR_BUSFAULTENA | SHCSR_USGFAULTENA,
            )
            .unwrap();

        assert_eq!(
            processor.read32(0xE000_ED24).unwrap(),
            SHCSR_PRESERVED_RAW_BITS | SHCSR_MEMFAULTENA | SHCSR_BUSFAULTENA | SHCSR_USGFAULTENA
        );
    }

    #[test]
    fn test_shcsr_active_fault_bits_reflect_live_exception_state() {
        for (exception, expected_bit) in [
            (Exception::MemoryManagementFault, SHCSR_MEMFAULTACT),
            (Exception::BusFault, SHCSR_BUSFAULTACT),
            (Exception::UsageFault, SHCSR_USGFAULTACT),
        ] {
            let mut processor = Processor::new();

            processor.set_msp(0x2000_0100);
            processor.set_pc(0x1004);
            processor.exception_entry(exception, 0x1004).unwrap();

            assert_eq!(
                processor.read32(0xE000_ED24).unwrap() & expected_bit,
                expected_bit
            );
        }
    }

    #[test]
    fn test_shcsr_active_fault_bits_clear_on_exception_return() {
        let mut processor = Processor::new();

        processor.set_msp(0x2000_0100);
        processor.set_pc(0x1000);

        processor
            .exception_entry(Exception::BusFault, 0x1000)
            .unwrap();
        assert_eq!(
            processor.read32(0xE000_ED24).unwrap() & SHCSR_BUSFAULTACT,
            SHCSR_BUSFAULTACT
        );

        processor.exception_return(0xFFFF_FFF9).unwrap();

        assert_eq!(
            processor.read32(0xE000_ED24).unwrap() & SHCSR_BUSFAULTACT,
            0
        );
    }

    #[test]
    fn test_shcsr_usagefault_active_bit_is_visible_to_handler_code() {
        let mut image = vec![0; 0x100].into_boxed_slice();

        image[0..4].copy_from_slice(&0x2000_0100_u32.to_le_bytes());
        image[4..8].copy_from_slice(&0x0000_0041_u32.to_le_bytes());
        image[24..28].copy_from_slice(&0x0000_0081_u32.to_le_bytes());

        image[0x40..0x42].copy_from_slice(&0xde00_u16.to_le_bytes());

        image[0x80..0x82].copy_from_slice(&0x4801_u16.to_le_bytes());
        image[0x82..0x84].copy_from_slice(&0x6801_u16.to_le_bytes());
        image[0x84..0x86].copy_from_slice(&0xbf00_u16.to_le_bytes());
        image[0x88..0x8c].copy_from_slice(&0xE000_ED24_u32.to_le_bytes());

        let mut processor = Processor::new();
        processor.fault_trap_mode(FaultTrapMode::none());
        processor.flash_memory(image.len(), &image);
        processor.reset().unwrap();

        processor.write32(0xE000_ED24, SHCSR_USGFAULTENA).unwrap();

        processor.step();

        assert!(processor.exception_active(Exception::UsageFault));
        assert_eq!(
            processor.read32(0xE000_ED24).unwrap() & SHCSR_USGFAULTACT,
            SHCSR_USGFAULTACT
        );

        processor.step();
        processor.step();

        assert_eq!(processor.get_r(Reg::R0), 0xE000_ED24);
        assert_eq!(
            processor.get_r(Reg::R1) & SHCSR_USGFAULTACT,
            SHCSR_USGFAULTACT,
            "handler-side SHCSR read should observe UsageFault active bit"
        );
    }

    #[test]
    fn test_shcsr_memmanage_active_bit_is_visible_to_handler_code() {
        let mut image = vec![0; 0x100].into_boxed_slice();

        image[0..4].copy_from_slice(&0x2000_0100_u32.to_le_bytes());
        image[4..8].copy_from_slice(&0x0000_0041_u32.to_le_bytes());
        image[16..20].copy_from_slice(&0x0000_0081_u32.to_le_bytes());

        image[0x40..0x42].copy_from_slice(&0x4800_u16.to_le_bytes());
        image[0x42..0x44].copy_from_slice(&0x6801_u16.to_le_bytes());
        image[0x44..0x48].copy_from_slice(&0x6000_0000_u32.to_le_bytes());

        image[0x80..0x82].copy_from_slice(&0x4801_u16.to_le_bytes());
        image[0x82..0x84].copy_from_slice(&0x6801_u16.to_le_bytes());
        image[0x84..0x86].copy_from_slice(&0xbf00_u16.to_le_bytes());
        image[0x88..0x8c].copy_from_slice(&0xE000_ED24_u32.to_le_bytes());

        let mut processor = Processor::new();
        processor.fault_trap_mode(FaultTrapMode::none());
        processor.flash_memory(image.len(), &image);
        processor.reset().unwrap();

        processor.write32(0xE000_ED24, SHCSR_MEMFAULTENA).unwrap();

        processor.step();
        processor.step();

        assert!(processor.exception_active(Exception::MemoryManagementFault));
        assert_eq!(
            processor.read32(0xE000_ED24).unwrap() & SHCSR_MEMFAULTACT,
            SHCSR_MEMFAULTACT
        );

        processor.step();
        processor.step();

        assert_eq!(processor.get_r(Reg::R0), 0xE000_ED24);
        assert_eq!(
            processor.get_r(Reg::R1) & SHCSR_MEMFAULTACT,
            SHCSR_MEMFAULTACT,
            "handler-side SHCSR read should observe MemManage active bit"
        );
    }

    #[test]
    fn test_cfsr_write_one_to_clear_via_bus32() {
        let mut processor = Processor::new();

        processor.cfsr = 0x0103_0005;

        processor.write32(0xE000_ED28, 0x0001_0001).unwrap();

        assert_eq!(processor.read32(0xE000_ED28).unwrap(), 0x0102_0004);
    }

    #[test]
    fn test_hfsr_write_one_to_clear_via_bus32() {
        let mut processor = Processor::new();

        processor.hfsr = HFSR_VECTTBL | HFSR_FORCED | HFSR_DEBUGEVT;

        processor
            .write32(0xE000_ED2C, HFSR_VECTTBL | HFSR_DEBUGEVT)
            .unwrap();

        assert_eq!(processor.read32(0xE000_ED2C).unwrap(), HFSR_FORCED);
    }

    #[test]
    fn test_reset_restores_scb_fault_status_boot_defaults() {
        let image = reset_test_image();

        let mut processor = Processor::new();
        processor.flash_memory(image.len(), &image);

        processor.shcsr = SHCSR_MEMFAULTENA | SHCSR_BUSFAULTENA | SHCSR_USGFAULTENA | 0x0b;
        processor.cfsr = 0x0103_9187;
        processor.dfsr = 0x1f;
        processor.hfsr = HFSR_VECTTBL | HFSR_FORCED | HFSR_DEBUGEVT;
        processor.mmfar = 0x2000_1234;
        processor.bfar = 0x4000_5678;
        processor.afsr = 0x89ab_cdef;

        processor.reset().unwrap();

        assert_eq!(processor.read32(0xE000_ED24).unwrap(), 0);
        assert_eq!(processor.read32(0xE000_ED28).unwrap(), 0);
        assert_eq!(processor.read32(0xE000_ED2C).unwrap(), 0);
        assert_eq!(processor.read32(0xE000_ED30).unwrap(), 0);
        assert_eq!(processor.read32(0xE000_ED34).unwrap(), 0);
        assert_eq!(processor.read32(0xE000_ED38).unwrap(), 0);
        assert_eq!(processor.read32(0xE000_ED3C).unwrap(), 0);
    }

    #[test]
    #[cfg(any(feature = "fpv4-sp-d16", feature = "fpv5-sp-d16", feature = "fpv5-d16"))]
    fn test_reset_restores_fp_system_register_defaults() {
        let image = reset_test_image();

        let mut processor = Processor::new();
        processor.flash_memory(image.len(), &image);

        processor.cpacr = 0x00f0_0000;
        processor.fpccr = 0xc000_0039;
        processor.fpcar = 0x2000_0100;
        processor.fpdscr = 0x00ab_0000;
        processor.mvfr0 = 0x1011_0021;
        processor.mvfr1 = 0x1100_0011;
        processor.mvfr2 = 0x0000_0040;

        processor.reset().unwrap();

        assert_eq!(processor.cpacr, 0);
        assert_eq!(processor.fpccr, 0);
        assert_eq!(processor.fpcar, 0);
        assert_eq!(processor.fpdscr, 0);
        assert_eq!(processor.mvfr0, 0);
        assert_eq!(processor.mvfr1, 0);
        assert_eq!(processor.mvfr2, 0);
    }
}
