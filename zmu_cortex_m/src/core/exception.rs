//!
//! A Trait for representing a Cortex armv6-m exceptions.
//!
//!

use crate::bus::Bus;
use crate::core::bits::Bits;
use crate::core::register::{BaseReg, Ipsr, Reg};
use crate::core::Processor;
use crate::core::ProcessorMode;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Copy, Clone)]
pub struct ExceptionState {
    priority: i16,
    pending: bool,
    active: bool,
    exception: usize,
}

impl ExceptionState {
    pub fn new(exception: Exception, priority: i16) -> Self {
        ExceptionState {
            exception: usize::from(u8::from(exception)),
            priority,
            pending: false,
            active: false,
        }
    }
}

pub trait ExceptionHandling {
    fn get_execution_priority(&self) -> i16;

    fn set_exception_pending(&mut self, exception: Exception);
    fn get_pending_exception(&mut self) -> Option<Exception>;
    fn return_address(&self, exception_type: Exception, return_address: u32) -> u32;
    fn push_stack(&mut self, exception_type: Exception, return_address: u32);
    fn pop_stack(&mut self, frameptr: u32, exc_return: u32);

    fn exception_taken(&mut self, exception: Exception);

    fn exception_entry(&mut self, exception: Exception, return_address: u32);
    fn exception_active_bit_count(&self) -> usize;
    fn deactivate(&mut self, returning_exception_number: u8);
    fn invalid_exception_return(&mut self, returning_exception_number: u8, exc_return: u32);
    fn exception_return(&mut self, exc_return: u32);
    fn clear_pending_exception(&mut self, exception: Exception);
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Exception {
    Reset,
    NMI,
    HardFault,
    MemoryManagementFault,
    BusFault,
    UsageFault,
    Reserved4,
    Reserved5,
    Reserved6,
    DebugMonitor,
    SVCall,
    Reserved8,
    Reserved9,
    PendSV,
    SysTick,
    Interrupt { n: u8 },
}

impl ExceptionHandling for Processor {
    fn get_execution_priority(&self) -> i16 {
        let mut highestpri: i16 = 256;
        let mut boostedpri: i16 = 256;
        let subgroupshift = self.aircr.get_bits(8..11);
        let groupvalue = 2 << subgroupshift;

        for (_, exp) in self.exception.iter().filter(|&(_, e)| e.active) {
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
        if self.faultmask {
            boostedpri = -1;
        }

        if boostedpri < highestpri {
            boostedpri
        } else {
            highestpri
        }
    }

    fn set_exception_pending(&mut self, exception: Exception) {
        let index: u8 = exception.into();
        let mut exp = self.exception.get_mut(&(index as usize)).unwrap();

        if !exp.pending {
            exp.pending = true;
            self.pending_exception_count += 1;
        }
    }

    fn get_pending_exception(&mut self) -> Option<Exception> {
        if self.pending_exception_count > 0 {
            // self.execution_priority
            let mut possible_exceptions: Vec<ExceptionState> = self
                .exception
                .iter()
                .filter(|&(_, e)| e.pending && e.priority < self.execution_priority)
                .map(|(&_, &e)| e)
                .collect();

            if !possible_exceptions.is_empty() {
                possible_exceptions.sort_by(|a, b| b.priority.cmp(&a.priority));
                return Some(Exception::from(possible_exceptions[0].exception as u8));
            }
        }
        None
    }

    fn clear_pending_exception(&mut self, exception: Exception) {
        let index: u8 = exception.into();
        let exp = self.exception.get_mut(&(index as usize)).unwrap();
        if exp.pending {
            exp.pending = false;
            self.pending_exception_count -= 1;
        }
    }

    fn return_address(&self, exception_type: Exception, return_address: u32) -> u32 {
        match exception_type {
            Exception::NMI => return_address,
            Exception::HardFault => return_address,
            Exception::MemoryManagementFault => return_address,
            Exception::BusFault => return_address,
            Exception::UsageFault => return_address - 4,
            Exception::SVCall => return_address,
            Exception::DebugMonitor => return_address,
            Exception::PendSV => return_address,
            Exception::SysTick => return_address,
            Exception::Interrupt { .. } => return_address,
            _ => panic!("unsupported exception"),
        }
    }

    fn push_stack(&mut self, exception_type: Exception, return_address: u32) {
        const FRAME_SIZE: u32 = 0x20;

        //TODO FP extensions
        //TODO forcealign
        // forces 8 byte alignment on the stack
        let forcealign = true;
        let spmask = ((forcealign as u32) << 2) ^ 0xFFFF_FFFF;

        let (frameptr, frameptralign) =
            if self.control.sp_sel && self.mode == ProcessorMode::ThreadMode {
                let align = (self.psp.get_bit(2) & forcealign) as u32;
                self.set_psp((self.psp - FRAME_SIZE) & spmask);
                (self.psp, align)
            } else {
                let align = self.msp.get_bit(2) as u32;
                self.set_msp((self.msp - FRAME_SIZE) & spmask);
                (self.msp, align)
            };

        let r0 = self.get_r(Reg::R0);
        let r1 = self.get_r(Reg::R1);
        let r2 = self.get_r(Reg::R2);
        let r3 = self.get_r(Reg::R3);
        let r12 = self.get_r(Reg::R12);
        let lr = self.get_r(Reg::LR);

        let ret_addr = self.return_address(exception_type, return_address);

        self.write32(frameptr, r0);
        self.write32(frameptr + 0x4, r1);
        self.write32(frameptr + 0x8, r2);
        self.write32(frameptr + 0xc, r3);
        self.write32(frameptr + 0x10, r12);
        self.write32(frameptr + 0x14, lr);
        self.write32(frameptr + 0x18, ret_addr);
        let xpsr = (self.psr.value & 0b1111_1111_1111_1111_1111_1101_1111_1111)
            | (frameptralign << 9) as u32;
        self.write32(frameptr + 0x1c, xpsr);

        if self.mode == ProcessorMode::HandlerMode {
            self.lr = 0xFFFF_FFF1;
        } else if !self.control.sp_sel {
            self.lr = 0xFFFF_FFF9;
        } else {
            self.lr = 0xFFFF_FFFD;
        }
    }

    fn pop_stack(&mut self, frameptr: u32, exc_return: u32) {
        //TODO: fp extensions

        const FRAME_SIZE: u32 = 0x20;

        //let forcealign = ccr.stkalign;
        let forcealign = true;

        self.set_r(Reg::R0, self.read32(frameptr));
        self.set_r(Reg::R1, self.read32(frameptr + 0x4));
        self.set_r(Reg::R2, self.read32(frameptr + 0x8));
        self.set_r(Reg::R3, self.read32(frameptr + 0xc));
        self.set_r(Reg::R12, self.read32(frameptr + 0x10));
        self.set_r(Reg::LR, self.read32(frameptr + 0x14));
        let pc = self.read32(frameptr + 0x18);
        let psr = self.read32(frameptr + 0x1c);

        self.branch_write_pc(pc);

        let spmask = ((psr.get_bit(9) && forcealign) as u32) << 2;

        match exc_return.get_bits(0..4) {
            0b0001 | 0b1001 => {
                let msp = self.get_msp();
                self.set_msp((msp + FRAME_SIZE) | spmask);
            }
            0b1101 => {
                let psp = self.get_psp();
                self.set_psp((psp + FRAME_SIZE) | spmask);
            }
            _ => {
                panic!("wrong exc return");
            }
        }
        self.psr.value.set_bits(27..32, psr.get_bits(27..32));
        self.psr.value.set_bits(0..9, psr.get_bits(0..9));
        self.psr.value.set_bits(10..16, psr.get_bits(10..16));
        self.psr.value.set_bits(24..27, psr.get_bits(24..27));
    }

    fn exception_taken(&mut self, exception: Exception) {
        self.control.sp_sel = false;
        self.mode = ProcessorMode::HandlerMode;
        self.psr.set_exception_number(exception.into());
        self.exception
            .get_mut(&usize::from(u8::from(exception)))
            .unwrap()
            .active = true;

        self.execution_priority = self.get_execution_priority();

        // SetEventRegister();
        // InstructionSynchronizationBarrier();
        let vtor = self.vtor;
        let offset = u32::from(u8::from(exception)) * 4;
        let start = self.read32(vtor + offset);
        self.blx_write_pc(start);
    }

    fn exception_entry(&mut self, exception: Exception, return_address: u32) {
        self.push_stack(exception, return_address);
        self.exception_taken(exception);
    }

    fn exception_active_bit_count(&self) -> usize {
        self.exception
            .iter()
            .filter(|&(_, exp)| exp.active)
            .fold(0, |acc, _| acc + 1)
    }

    fn deactivate(&mut self, returning_exception_number: u8) {
        self.exception
            .get_mut(&usize::from(returning_exception_number))
            .unwrap()
            .active = false;
        if self.psr.get_exception_number() != 0b10 {
            //TODO
            //self.faultmask0 = 0;
        }
        self.execution_priority = self.get_execution_priority();
    }

    fn invalid_exception_return(&mut self, returning_exception_number: u8, exc_return: u32) {
        self.deactivate(returning_exception_number);
        //ufsr.invpc = true;
        self.set_r(Reg::LR, (0b1111 << 28) + exc_return);
        self.exception_taken(Exception::UsageFault);
    }

    fn exception_return(&mut self, exc_return: u32) {
        assert!(self.mode == ProcessorMode::HandlerMode);

        let returning_exception_number = self.psr.get_exception_number();
        let nested_activation = self.exception_active_bit_count();

        if !self.exception[&usize::from(returning_exception_number)].active {
            self.invalid_exception_return(returning_exception_number, exc_return);
            return;
        } else {
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
                        self.invalid_exception_return(returning_exception_number, exc_return);
                        return;
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
                        self.invalid_exception_return(returning_exception_number, exc_return);
                        return;
                    } else {
                        frameptr = self.get_psp();
                        self.mode = ProcessorMode::ThreadMode;
                        self.control.sp_sel = true;
                    }
                }
                _ => {
                    self.invalid_exception_return(returning_exception_number, exc_return);
                    return;
                }
            }

            self.deactivate(returning_exception_number);
            self.pop_stack(frameptr, exc_return);
            if self.mode == ProcessorMode::HandlerMode && self.psr.get_exception_number() == 0 {
                //ufsr.invpc = true;
                self.push_stack(Exception::UsageFault, exc_return); // to negate pop_stack
                self.set_r(Reg::LR, (0b1111 << 28) + exc_return);
                self.exception_taken(Exception::UsageFault);
                return;
            }

            if self.mode == ProcessorMode::ThreadMode && self.psr.get_exception_number() != 0 {
                //ufsr.invpc = true;
                self.push_stack(Exception::UsageFault, exc_return); // to negate pop_stack
                self.set_r(Reg::LR, (0b1111 << 28) + exc_return);
                self.exception_taken(Exception::UsageFault);
                return;
            }

            //self.clear_exclusive_local(processor_id());
            //self.set_event_register();
            //self.instruction_synchronization_barrier();
            /*if self.mode == ProcessorMode::ThreadMode && !nested_activation && scr.sleeponexit{
                self.sleep_on_exit();
            }*/
        }
    }
}

impl From<Exception> for u8 {
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

impl From<u8> for Exception {
    fn from(value: u8) -> Self {
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

/*    fn set_pend(&mut self, exception: Exception) {
        match exception {
            Exception::Reset => {
                self.nvic_exception_pending.set_bit(0, true);
            }
            Exception::NMI => {
                self.nvic_exception_pending.set_bit(1, true);
            }
            Exception::HardFault => {
                self.nvic_exception_pending.set_bit(2, true);
            }
            Exception::MemoryManagementFault => {
                self.nvic_exception_pending.set_bit(3, true);
            }
            Exception::BusFault => {
                self.nvic_exception_pending.set_bit(4, true);
            }
            Exception::UsageFault => {
                self.nvic_exception_pending.set_bit(5, true);
            }
            Exception::Reserved4 => {
                self.nvic_exception_pending.set_bit(6, true);
            }
            Exception::Reserved5 => {
                self.nvic_exception_pending.set_bit(7, true);
            }
            Exception::Reserved6 => {
                self.nvic_exception_pending.set_bit(8, true);
            }
            Exception::DebugMonitor => {
                self.nvic_exception_pending.set_bit(9, true);
            }
            Exception::SVCall => {
                self.nvic_exception_pending.set_bit(10, true);
            }
            Exception::Reserved8 => {
                self.nvic_exception_pending.set_bit(11, true);
            }
            Exception::Reserved9 => {
                self.nvic_exception_pending.set_bit(12, true);
            }
            Exception::PendSV => {
                self.nvic_exception_pending.set_bit(13, true);
            }
            Exception::SysTick => {
                self.nvic_exception_pending.set_bit(14, true);
            }
            Exception::Interrupt { n } => {
                let index = n / 32;
                let bit = n % 32;
                self.nvic_interrupt_pending[index as usize].set_bit(bit as usize, true);
            }
        }
    }
*/

/*
            // setting PRIMASK to 1 raises execution priority to "0"
            // setting BASEPRI changes the priority level required for exception pre-emption
            // Has effect only if BASEPRI < current unmasked priority
            // FAULTMASK 1 raises masked execution priority to "-1"


            // When no exception is active, software in Thread or handler mode is executing
            // at a execution priority (max supported priority + 1)
            // this is the "base level of execution"

            // from base level, the execution priority is the greatest urgency of:
            // - base level of execution priority
            // - greatest urgency of all active exceptions, including any that the current exception pre-empted
            // - the impact of PRIMASK, FAULTMASK, BASEPRI

            // Reset, NMI or HardFault pending?
            if (self.nvic_exception_pending & 0b111) != 0 {
                if self.nvic_exception_active.get_bit(0) {
                    return BusStepResult::Nothing;
                }

                if self.nvic_exception_pending.get_bit(0) {
                    self.nvic_exception_active.set_bit(0, true);
                    return BusStepResult::Exception {
                        exception: Exception::Reset,
                    };
                }

                if self.nvic_exception_active.get_bit(1) {
                    return BusStepResult::Nothing;
                }

                if self.nvic_exception_pending.get_bit(1) {
                    self.nvic_exception_active.set_bit(1, true);
                    return BusStepResult::Exception {
                        exception: Exception::NMI,
                    };
                }
                if self.nvic_exception_active.get_bit(2) {
                    return BusStepResult::Nothing;
                }

                if self.nvic_exception_pending.get_bit(2) {
                    self.nvic_exception_active.set_bit(2, true);
                    return BusStepResult::Exception {
                        exception: Exception::HardFault,
                    };
                }
            }

            // check the priorities from shrp1, shrp2, shrp3
*/
