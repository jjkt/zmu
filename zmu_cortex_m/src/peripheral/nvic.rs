//!
//! Cortex Nested Vectored Interrupt Controller simulation
//!

use crate::core::bits::Bits;
use crate::core::exception::Exception;
use crate::core::exception::ExceptionHandling;
use crate::Processor;

///
/// Register API for NVIC
///
pub trait NVIC {
    ///
    /// Write Interrupt Set Enable
    ///
    fn nvic_write_iser(&mut self, index: usize, value: u32);
    ///
    /// Read Interrupt Set Enable
    ///
    fn nvic_read_iser(&self, index: usize) -> u32;

    ///
    /// Write Interrupt Clear Enable
    ///
    fn nvic_write_icer(&mut self, index: usize, value: u32);
    ///
    /// Read Interrupt Clear Enable
    ///
    fn nvic_read_icer(&self, index: usize) -> u32;

    ///
    /// Write interrupt Set Pending
    ///
    fn nvic_write_ispr(&mut self, index: usize, value: u32);
    ///
    /// Read interrupt Set Pending
    ///
    fn nvic_read_ispr(&self, index: usize) -> u32;
    ///
    /// Write interrupt Clear Pending
    ///
    fn nvic_write_icpr(&mut self, index: usize, value: u32);
    ///
    /// Read interrupt Clear Pending
    ///
    fn nvic_read_icpr(&self, index: usize) -> u32;

    ///
    /// Read Interrupt Active Bit Register.
    /// ```index``` is the 32 bit set of irqs to list. Value 0 means irqs 0..=31.
    ///
    fn nvic_read_iabr(&self, index: usize) -> u32;

    ///
    /// 32bit write to interrupt priority register
    ///
    fn nvic_write_ipr(&mut self, index: usize, value: u32);

    ///
    /// 32bit read from interrupt priority register
    ///
    fn nvic_read_ipr(&self, index: usize) -> u32;

    ///
    /// 8bit write to interrupt priority register
    ///
    fn nvic_write_ipr_u8(&mut self, index: usize, value: u8);

    ///
    /// 8bit read from interrupt priority register
    ///
    fn nvic_read_ipr_u8(&self, index: usize) -> u8;
}

trait NVICHelper {
    fn nvic_activate_pending(&mut self, index: usize);
}

fn set_bits_array(array: &mut [u32; 16], index: usize, value: u32) {
    if index == 15 {
        array[index] |= value & 0xffff;
    } else {
        array[index] |= value;
    }
}

fn clear_bits_array(array: &mut [u32; 16], index: usize, value: u32) {
    if index == 15 {
        array[index] &= (value & 0xffff) ^ 0xFFFF_FFFF;
    } else {
        array[index] &= value ^ 0xFFFF_FFFF;
    }
}

impl NVICHelper for Processor {
    fn nvic_activate_pending(&mut self, index: usize) {
        let mut active = self.nvic_interrupt_pending[index] & self.nvic_interrupt_enabled[index];
        let mut irqn = index * 4;
        while active != 0 {
            if active & 1 != 0 {
                self.set_exception_pending(Exception::Interrupt { n: irqn });
            }
            active >>= 1;
            irqn += 1;
        }
    }
}

impl NVIC for Processor {
    fn nvic_write_iser(&mut self, index: usize, value: u32) {
        set_bits_array(&mut self.nvic_interrupt_enabled, index, value);
        self.nvic_activate_pending(index);
    }

    fn nvic_read_iser(&self, index: usize) -> u32 {
        self.nvic_interrupt_enabled[index]
    }

    fn nvic_write_icer(&mut self, index: usize, value: u32) {
        clear_bits_array(&mut self.nvic_interrupt_enabled, index, value);
        //self.nvic_clear_pending(index);
    }

    fn nvic_read_icer(&self, index: usize) -> u32 {
        self.nvic_interrupt_enabled[index] ^ 0xFFFF_FFFF
    }

    fn nvic_write_ispr(&mut self, index: usize, value: u32) {
        set_bits_array(&mut self.nvic_interrupt_pending, index, value);
        self.nvic_activate_pending(index);
    }

    fn nvic_read_ispr(&self, index: usize) -> u32 {
        self.nvic_interrupt_pending[index]
    }

    fn nvic_write_icpr(&mut self, index: usize, value: u32) {
        clear_bits_array(&mut self.nvic_interrupt_pending, index, value);
        //self.nvic_clear_pending(index);
    }

    fn nvic_read_icpr(&self, index: usize) -> u32 {
        self.nvic_interrupt_pending[index] ^ 0xFFFF_FFFF
    }

    fn nvic_read_iabr(&self, index: usize) -> u32 {
        let first_irqn = index * 32;
        let mut active = 0;
        for irqn in first_irqn..first_irqn + 32 {
            if self.exception_active(Exception::Interrupt { n: irqn }) {
                active |= 1;
            }
            active <<= 1;
        }
        active
    }

    fn nvic_write_ipr(&mut self, index: usize, value: u32) {
        self.nvic_write_ipr_u8(index, value.get_bits(0..8) as u8);
        self.nvic_write_ipr_u8(index + 1, value.get_bits(8..16) as u8);
        self.nvic_write_ipr_u8(index + 2, value.get_bits(16..24) as u8);
        self.nvic_write_ipr_u8(index + 3, value.get_bits(24..32) as u8);
    }

    fn nvic_read_ipr(&self, index: usize) -> u32 {
        u32::from(self.nvic_read_ipr_u8(index))
            + (u32::from(self.nvic_read_ipr_u8(index + 1)) << 8)
            + (u32::from(self.nvic_read_ipr_u8(index + 2)) << 16)
            + (u32::from(self.nvic_read_ipr_u8(index + 3)) << 24)
    }

    fn nvic_read_ipr_u8(&self, index: usize) -> u8 {
        let priority = self.get_exception_priority(Exception::Interrupt { n: index });
        assert!(priority > 0 && priority < 256);
        priority as u8
    }

    fn nvic_write_ipr_u8(&mut self, index: usize, value: u8) {
        self.set_exception_priority(Exception::Interrupt { n: index }, value);
    }
}
