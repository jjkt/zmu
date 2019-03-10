//!
//! Functionality for representing Cortex Exceptions.
//!
//!

use crate::bus::Bus;
use crate::core::bits::Bits;
use crate::core::fault::Fault;
use crate::core::register::{BaseReg, Ipsr, Reg};
use crate::core::reset::Reset;
use crate::Processor;
use crate::ProcessorMode;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Copy, Clone)]
///
/// Status information for an exception
///
pub struct ExceptionState {
    priority: i16,
    pending: bool,
    active: bool,
    exception: usize,
}

impl ExceptionState {
    ///
    /// Create a state information for specific exception with given priority
    ///
    pub fn new(exception: Exception, priority: i16) -> Self {
        Self {
            exception: usize::from(exception),
            priority,
            pending: false,
            active: false,
        }
    }
}

///
/// Trait for interacting with exceptions
///
pub trait ExceptionHandling {
    ///
    /// Get the current processor execution priority (EP). Execution priority determines which
    /// exceptions can pre-empt the current execution. Lower priority number has higher urgency.
    ///
    fn get_execution_priority(&self) -> i16;

    ///
    /// Set exception pending if it is not already pending
    ///
    fn set_exception_pending(&mut self, exception: Exception);

    ///
    /// Get the currently highest priority pending exception
    ///
    fn get_pending_exception(&mut self) -> Option<Exception>;

    ///
    /// Clear the pending status of an exception
    ///
    fn clear_pending_exception(&mut self, exception: Exception);

    ///
    /// Enter an exception.
    ///
    /// Return adress is the address to which the execution should return when this exception is returned from.
    ///
    fn exception_entry(&mut self, exception: Exception, return_address: u32) -> Result<(), Fault>;

    ///
    /// Return from an exception.
    ///     
    /// exc_return determines the mode to which to return to.
    ///
    /// Exception return happens when processor is in HandlerMode and exc_return value is loaded to PC using
    /// LDM, POP, LDR, or BX instructions
    ///
    fn exception_return(&mut self, exc_return: u32) -> Result<(), Fault>;

    ///
    /// Check if given exception is currently active
    ///          
    fn exception_active(&self, exception: Exception) -> bool;

    ///
    /// Set priority of an exception. Smaller priority number has higher urgency.
    ///          
    fn set_exception_priority(&mut self, exception: Exception, priority: u8);

    ///
    /// Get priority of an exception. Smaller priority number has higher urgency.
    ///          
    fn get_exception_priority(&self, exception: Exception) -> i16;

    ///
    /// Clear exceptions to reset state
    ///
    fn exceptions_reset(&mut self);
}

trait ExceptionHandlingHelpers {
    fn exception_taken(&mut self, exception: Exception) -> Result<(), Fault>;
    fn deactivate(&mut self, returning_exception_number: usize);
    fn invalid_exception_return(
        &mut self,
        returning_exception_number: usize,
        exc_return: u32,
    ) -> Result<(), Fault>;
    fn return_address(&self, exception_type: Exception, return_address: u32) -> u32;
    fn push_stack(&mut self, exception_type: Exception, return_address: u32) -> Result<(), Fault>;
    fn pop_stack(&mut self, frameptr: u32, exc_return: u32) -> Result<(), Fault>;
    fn exception_active_bit_count(&self) -> usize;
}

#[derive(PartialEq, Debug, Copy, Clone)]
///
/// List of supported Exceptions
///
/// Interrupts are controlled by NVIC, but still are generally handled like other exceptions (being sorted by priority)
///
pub enum Exception {
    /// Special exception to reset the processor
    Reset,
    /// Highest priority exception (for except the reset) that cannot be ever masked away.
    /// Can be triggered by a peripheral or triggered by software.
    NMI,
    /// Denotes a error during exception processing, or because an exception cannot be
    /// handled by any other exception handling mechanism.
    HardFault,
    /// Memory protection related fault
    MemoryManagementFault,
    /// Memory related fault (bus access error either for instructions or data)
    BusFault,
    /// Instruction execution faults for multiple underlying reasons. Example: undefined instructions.
    UsageFault,
    /// Reserved for future
    Reserved4,
    /// Reserved for future
    Reserved5,
    /// Reserved for future
    Reserved6,
    /// Debugging related exceptions
    DebugMonitor,
    /// Supervisor call exception, used typically for OS supervisor API handling.
    /// SVC instruction triggers SVCall exception.
    SVCall,
    /// Reserved for future
    Reserved8,
    /// Reserved for future
    Reserved9,
    /// Request for system level service, used typically for context switching in OS.
    PendSV,
    /// System timer exception, used for generating timer ticks.
    SysTick,
    /// Exception from a peripheral or software triggered interrupt
    Interrupt {
        /// Interrupt number, 0..
        n: usize,
    },
}

impl ExceptionHandlingHelpers for Processor {
    fn exception_taken(&mut self, exception: Exception) -> Result<(), Fault> {
        self.control.sp_sel = false;
        self.mode = ProcessorMode::HandlerMode;
        self.psr.set_isr_number(exception.into());
        self.exceptions.get_mut(&exception.into()).unwrap().active = true;

        self.execution_priority = self.get_execution_priority();

        // SetEventRegister();
        // InstructionSynchronizationBarrier();
        let vtor = self.vtor;
        let offset: u32 = usize::from(exception) as u32 * 4;
        let start = self.read32(vtor + offset)?;
        self.blx_write_pc(start);
        Ok(())
    }

    fn deactivate(&mut self, returning_exception_number: usize) {
        self.exceptions
            .get_mut(&returning_exception_number)
            .unwrap()
            .active = false;
        if self.psr.get_isr_number() != 0b10 {
            //TODO
            //self.faultmask0 = 0;
        }
        self.execution_priority = self.get_execution_priority();
    }

    fn invalid_exception_return(
        &mut self,
        returning_exception_number: usize,
        exc_return: u32,
    ) -> Result<(), Fault> {
        self.deactivate(returning_exception_number);
        //ufsr.invpc = true;
        self.set_r(Reg::LR, (0b1111 << 28) + exc_return);
        self.exception_taken(Exception::UsageFault)
    }

    fn exception_active_bit_count(&self) -> usize {
        self.exceptions
            .iter()
            .filter(|&(_, exp)| exp.active)
            .fold(0, |acc, _| acc + 1)
    }
    fn return_address(&self, exception_type: Exception, return_address: u32) -> u32 {
        match exception_type {
            Exception::NMI
            | Exception::HardFault
            | Exception::MemoryManagementFault
            | Exception::BusFault
            | Exception::SVCall
            | Exception::DebugMonitor
            | Exception::PendSV
            | Exception::SysTick
            | Exception::Interrupt { .. } => return_address,
            Exception::UsageFault => return_address - 4,
            _ => panic!("unsupported exception"),
        }
    }
    fn push_stack(&mut self, exception_type: Exception, return_address: u32) -> Result<(), Fault> {
        const FRAME_SIZE: u32 = 0x20;

        //TODO FP extensions
        //TODO forcealign
        // forces 8 byte alignment on the stack
        let forcealign = true;
        let spmask = ((forcealign as u32) << 2) ^ 0xFFFF_FFFF;

        let (frameptr, frameptralign) =
            if self.control.sp_sel && self.mode == ProcessorMode::ThreadMode {
                let align = (self.psp.get_bit(2) & forcealign) as u32;
                self.set_psp((self.psp.wrapping_sub(FRAME_SIZE)) & spmask);
                (self.psp, align)
            } else {
                let align = self.msp.get_bit(2) as u32;
                self.set_msp((self.msp.wrapping_sub(FRAME_SIZE)) & spmask);
                (self.msp, align)
            };

        let r0 = self.get_r(Reg::R0);
        let r1 = self.get_r(Reg::R1);
        let r2 = self.get_r(Reg::R2);
        let r3 = self.get_r(Reg::R3);
        let r12 = self.get_r(Reg::R12);
        let lr = self.get_r(Reg::LR);

        let ret_addr = self.return_address(exception_type, return_address);

        self.write32(frameptr, r0)?;
        self.write32(frameptr.wrapping_add(0x4), r1)?;
        self.write32(frameptr.wrapping_add(0x8), r2)?;
        self.write32(frameptr.wrapping_add(0xc), r3)?;
        self.write32(frameptr.wrapping_add(0x10), r12)?;
        self.write32(frameptr.wrapping_add(0x14), lr)?;
        self.write32(frameptr.wrapping_add(0x18), ret_addr)?;
        let xpsr = (self.psr.value & 0b1111_1111_1111_1111_1111_1101_1111_1111)
            | (frameptralign << 9) as u32;
        self.write32(frameptr.wrapping_add(0x1c), xpsr)?;

        if self.mode == ProcessorMode::HandlerMode {
            self.lr = 0xFFFF_FFF1;
        } else if self.control.sp_sel {
            self.lr = 0xFFFF_FFFD;
        } else {
            self.lr = 0xFFFF_FFF9;
        }
        Ok(())
    }

    fn pop_stack(&mut self, frameptr: u32, exc_return: u32) -> Result<(), Fault> {
        //TODO: fp extensions

        const FRAME_SIZE: u32 = 0x20;

        //let forcealign = ccr.stkalign;
        let forcealign = true;

        self.set_r(Reg::R0, self.read32(frameptr)?);
        self.set_r(Reg::R1, self.read32(frameptr.wrapping_add(0x4))?);
        self.set_r(Reg::R2, self.read32(frameptr.wrapping_add(0x8))?);
        self.set_r(Reg::R3, self.read32(frameptr.wrapping_add(0xc))?);
        self.set_r(Reg::R12, self.read32(frameptr.wrapping_add(0x10))?);
        self.set_r(Reg::LR, self.read32(frameptr.wrapping_add(0x14))?);
        let pc = self.read32(frameptr.wrapping_add(0x18))?;
        let psr = self.read32(frameptr.wrapping_add(0x1c))?;

        self.branch_write_pc(pc);

        let spmask = ((psr.get_bit(9) && forcealign) as u32) << 2;

        match exc_return.get_bits(0..4) {
            0b0001 | 0b1001 => {
                let msp = self.get_msp();
                self.set_msp((msp.wrapping_add(FRAME_SIZE)) | spmask);
            }
            0b1101 => {
                let psp = self.get_psp();
                self.set_psp((psp.wrapping_add(FRAME_SIZE)) | spmask);
            }
            _ => {
                panic!("wrong exc return");
            }
        }
        self.psr.value.set_bits(27..32, psr.get_bits(27..32));
        self.psr.value.set_bits(0..9, psr.get_bits(0..9));
        self.psr.value.set_bits(10..16, psr.get_bits(10..16));
        self.psr.value.set_bits(24..27, psr.get_bits(24..27));
        Ok(())
    }
}

impl ExceptionHandling for Processor {
    fn exceptions_reset(&mut self) {
        for exception in self.exceptions.values_mut() {
            exception.pending = false;
            exception.active = false;

            if exception.exception > Exception::HardFault.into() {
                exception.priority = 0;
            }
        }
    }
    fn exception_active(&self, exception: Exception) -> bool {
        self.exceptions[&usize::from(exception)].active
    }

    fn set_exception_priority(&mut self, exception: Exception, priority: u8) {
        self.exceptions.get_mut(&exception.into()).unwrap().priority = i16::from(priority);
    }

    fn get_exception_priority(&self, exception: Exception) -> i16 {
        self.exceptions[&exception.into()].priority
    }

    fn get_execution_priority(&self) -> i16 {
        let mut highestpri: i16 = 256;
        let mut boostedpri: i16 = 256;
        let subgroupshift = self.aircr.get_bits(8..11);
        let groupvalue = 2 << subgroupshift;

        for (_, exp) in self.exceptions.iter().filter(|&(_, e)| e.active) {
            if exp.priority < highestpri {
                highestpri = exp.priority;
                let subgroupvalue = highestpri % groupvalue;
                highestpri -= subgroupvalue;
            }
        }
        if self.basepri != 0 {
            boostedpri = i16::from(self.basepri);
            let subgroupvalue = boostedpri % groupvalue;
            boostedpri -= subgroupvalue;
        }
        if self.primask {
            boostedpri = 0;
        }
        #[cfg(any(armv7m, armv7em))]
        {
            if self.faultmask {
                boostedpri = -1;
            }
        }

        if boostedpri < highestpri {
            boostedpri
        } else {
            highestpri
        }
    }

    fn set_exception_pending(&mut self, exception: Exception) {
        let mut exp = self.exceptions.get_mut(&exception.into()).unwrap();

        if !exp.pending {
            exp.pending = true;
            self.pending_exception_count += 1;
        }
    }

    fn get_pending_exception(&mut self) -> Option<Exception> {
        if self.pending_exception_count > 0 {
            let mut possible_exceptions: Vec<ExceptionState> = self
                .exceptions
                .iter()
                .filter(|&(_, e)| e.pending && e.priority < self.execution_priority)
                .map(|(&_, &e)| e)
                .collect();

            if !possible_exceptions.is_empty() {
                possible_exceptions.sort_by(|a, b| a.priority.cmp(&b.priority));
                return Some(possible_exceptions[0].exception.into());
            }
        }
        None
    }

    fn clear_pending_exception(&mut self, exception: Exception) {
        let exp = self.exceptions.get_mut(&exception.into()).unwrap();
        if exp.pending {
            exp.pending = false;
            self.pending_exception_count -= 1;
        }
    }

    fn exception_entry(&mut self, exception: Exception, return_address: u32) -> Result<(), Fault> {
        if exception == Exception::Reset {
            self.reset()
        } else {
            self.push_stack(exception, return_address)?;
            self.exception_taken(exception)
        }
    }

    fn exception_return(&mut self, exc_return: u32) -> Result<(), Fault> {
        assert!(self.mode == ProcessorMode::HandlerMode);

        let returning_exception_number = self.psr.get_isr_number();
        let nested_activation = self.exception_active_bit_count();

        if self.exceptions[&returning_exception_number].active {
            let frameptr;
            match exc_return.get_bits(0..4) {
                0b0001 => {
                    // return to handler
                    frameptr = self.get_msp();
                    self.mode = ProcessorMode::HandlerMode;
                    self.control.sp_sel = false;
                }
                0b1001 => {
                    // returning to thread using main stack
                    if nested_activation == 0
                    /*&& !self.ccr.nonbasethreadena*/
                    {
                        return self
                            .invalid_exception_return(returning_exception_number, exc_return);
                    } else {
                        frameptr = self.get_msp();
                        self.mode = ProcessorMode::ThreadMode;
                        self.control.sp_sel = false;
                    }
                }
                0b1101 => {
                    // returning to thread using process stack
                    if nested_activation == 0
                    /*&& !self.ccr.nonbasethreadena*/
                    {
                        return self
                            .invalid_exception_return(returning_exception_number, exc_return);
                    } else {
                        frameptr = self.get_psp();
                        self.mode = ProcessorMode::ThreadMode;
                        self.control.sp_sel = true;
                    }
                }
                _ => {
                    return self.invalid_exception_return(returning_exception_number, exc_return);
                }
            }

            self.deactivate(returning_exception_number);
            self.pop_stack(frameptr, exc_return)?;
            if self.mode == ProcessorMode::HandlerMode && self.psr.get_isr_number() == 0 {
                //ufsr.invpc = true;
                self.push_stack(Exception::UsageFault, exc_return)?; // to negate pop_stack
                self.set_r(Reg::LR, (0b1111 << 28) + exc_return);
                return self.exception_taken(Exception::UsageFault);
            }

            if self.mode == ProcessorMode::ThreadMode && self.psr.get_isr_number() != 0 {
                //ufsr.invpc = true;
                self.push_stack(Exception::UsageFault, exc_return)?; // to negate pop_stack
                self.set_r(Reg::LR, (0b1111 << 28) + exc_return);
                return self.exception_taken(Exception::UsageFault);
            }

            Ok(())
        } else {
            self.invalid_exception_return(returning_exception_number, exc_return)
        }
    }
}

impl From<Exception> for usize {
    fn from(value: Exception) -> Self {
        match value {
            Exception::Reset => 1,
            Exception::NMI => 2,
            Exception::HardFault => 3,
            Exception::MemoryManagementFault => 4,
            Exception::BusFault => 5,
            Exception::UsageFault => 6,
            Exception::Reserved4 => 7,
            Exception::Reserved5 => 8,
            Exception::Reserved6 => 9,
            Exception::DebugMonitor => 10,
            Exception::SVCall => 11,
            Exception::Reserved8 => 12,
            Exception::Reserved9 => 13,
            Exception::PendSV => 14,
            Exception::SysTick => 15,
            Exception::Interrupt { n } => 16 + n,
        }
    }
}

impl From<usize> for Exception {
    fn from(value: usize) -> Self {
        match value {
            1 => Exception::Reset,
            2 => Exception::NMI,
            3 => Exception::HardFault,
            4 => Exception::MemoryManagementFault,
            5 => Exception::BusFault,
            6 => Exception::UsageFault,
            7 => Exception::Reserved4,
            8 => Exception::Reserved5,
            9 => Exception::Reserved6,
            10 => Exception::DebugMonitor,
            11 => Exception::SVCall,
            12 => Exception::Reserved8,
            13 => Exception::Reserved9,
            14 => Exception::PendSV,
            15 => Exception::SysTick,
            _ => Exception::Interrupt { n: value - 16 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::Bus;
    #[cfg(any(armv7m, armv7em))]
    use crate::core::exception::Exception;
    use crate::core::exception::ExceptionHandling;
    #[cfg(any(armv7m, armv7em))]
    use crate::core::executor::Executor;
    #[cfg(any(armv7m, armv7em))]
    use crate::core::instruction::Instruction;
    use crate::core::register::Ipsr;
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
    fn test_push_stack() {
        const STACK_START: u32 = 0x2000_0100;
        let code = [0; 65536];
        let mut core = Processor::new(
            Some(Box::new(TestWriter {})),
            &code,
            Box::new(
                |_semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                    panic!("shoud not happen")
                },
            ),
        );

        // arrange
        let lr = {
            //    if self.control.sp_sel && self.mode == ProcessorMode::ThreadMode {
            core.control.sp_sel = false;
            //core.mode = ProcessorMode::ThreadMode;
            core.set_r(Reg::R0, 42);
            core.set_r(Reg::R1, 43);
            core.set_r(Reg::R2, 44);
            core.set_r(Reg::R3, 45);
            core.set_r(Reg::R12, 46);
            core.set_r(Reg::LR, 47);
            core.set_psp(0);
            core.set_msp(STACK_START);
            core.psr.value = 0xffff_ffff;

            // act
            core.push_stack(Exception::HardFault, 99).unwrap();

            assert_eq!(core.msp, STACK_START - 32);
            core.get_r(Reg::LR)
        };

        // values pushed on to stack
        assert_eq!(core.read32(STACK_START - 0x20).unwrap(), 42);
        assert_eq!(core.read32(STACK_START - 0x20 + 4).unwrap(), 43);
        assert_eq!(core.read32(STACK_START - 0x20 + 8).unwrap(), 44);
        assert_eq!(core.read32(STACK_START - 0x20 + 12).unwrap(), 45);
        assert_eq!(core.read32(STACK_START - 0x20 + 16).unwrap(), 46);
        assert_eq!(core.read32(STACK_START - 0x20 + 20).unwrap(), 47);
        assert_eq!(core.read32(STACK_START - 0x20 + 24).unwrap(), 99);
        assert_eq!(
            core.read32(STACK_START - 0x20 + 28).unwrap(),
            0b1111_1111_1111_1111_1111_1101_1111_1111
        );
        assert_eq!(lr, 0xffff_fff9);
    }

    #[test]
    fn test_exception_taken() {
        // Arrange
        let code = [0; 65536];
        let mut core = Processor::new(
            Some(Box::new(TestWriter {})),
            &code,
            Box::new(
                |_semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                    panic!("shoud not happen")
                },
            ),
        );

        core.control.sp_sel = true;
        core.mode = ProcessorMode::ThreadMode;
        core.psr.value = 0xffff_ffff;

        // Act
        core.exception_taken(Exception::BusFault).unwrap();

        // Assert
        assert_eq!(core.control.sp_sel, false);
        assert_eq!(core.mode, ProcessorMode::HandlerMode);
        assert_eq!(core.psr.get_isr_number(), Exception::BusFault.into());
        assert_eq!(core.exception_active(Exception::BusFault), true);
    }

    #[test]
    fn test_exception_priority() {
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

        processor.reset().unwrap();

        // Act
        processor.set_exception_pending(Exception::Reset);
        processor.set_exception_pending(Exception::NMI);
        processor.set_exception_pending(Exception::HardFault);

        // Assert
        assert_eq!(processor.get_pending_exception(), Some(Exception::Reset));
    }

    #[cfg(any(armv7m, armv7em))]
    #[test]
    fn test_faultmask_priority() {
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

        processor.reset().unwrap();

        // Act
        processor.set_exception_pending(Exception::HardFault);

        processor.step(
            &Instruction::CPS {
                im: true,
                affect_pri: false,
                affect_fault: true,
            },
            2,
        );

        // Assert
        assert_eq!(processor.get_pending_exception(), None);
    }

}
