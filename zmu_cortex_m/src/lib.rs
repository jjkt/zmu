//! Utilities for random number generation
//!
//! Processor simulator for ARM Cortex-M processors.
//!
//! 
//#![warn(missing_docs)]
//#![warn(missing_debug_implementations)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]


extern crate byteorder;
extern crate enum_set;

pub mod bus;
pub mod core;
pub mod decoder;
pub mod memory;
pub mod peripheral;
pub mod semihosting;
pub mod system;

use crate::core::exception::Exception;
use crate::core::register::{Apsr, BaseReg, Control, Reg, PSR};
use crate::memory::flash::FlashMemory;
use crate::memory::ram::RAM;
use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;

use crate::core::exception::ExceptionState;
use std::collections::HashMap;
use std::fmt;
use std::io;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ProcessorMode {
    ThreadMode,
    HandlerMode,
}

pub struct Processor {
    /* 13 of 32-bit general purpose registers. */
    pub r0_12: [u32; 13],

    msp: u32, //MSP, virtual reg r[13]
    psp: u32, //PSP, virtual reg r[13]
    lr: u32,
    pc: u32,

    pub cycle_count: u64,

    /* Processor state register, status flags. */
    pub psr: PSR,

    /* interrupt primary mask, a 1 bit mask register for
    global interrupt masking. */
    primask: bool,
    faultmask: bool,
    basepri: u8,

    /* Control bits: currently used stack and execution privilege if core.mode == ThreadMode */
    control: Control,

    /* Processor mode: either handler or thread mode. */
    mode: ProcessorMode,

    /* Is the core simulation currently running or not.*/
    pub running: bool,

    pub exceptions: HashMap<usize, ExceptionState>,
    pub pending_exception_count: u32,

    pub execution_priority: i16,

    itstate: u8,

    pub code: FlashMemory,
    pub sram: RAM,

    semihost_func: Box<FnMut(&SemihostingCommand) -> SemihostingResponse>,

    pub cpuid: u32,
    pub icsr: u32,
    pub vtor: u32,
    pub aircr: u32,
    pub scr: u32,
    pub ccr: u32,
    pub shpr1: u32,
    pub shpr2: u32,
    pub shpr3: u32,
    pub shcsr: u32,
    pub cfsr: u32,
    pub hfsr: u32,
    pub dfsr: u32,
    pub mmfar: u32,
    pub bfar: u32,
    pub afsr: u32,
    pub cpacr: u32,

    pub fpccr: u32,
    pub fpcar: u32,
    pub fpdscr: u32,

    pub mvfr0: u32,
    pub mvfr1: u32,
    pub mvfr2: u32,

    pub ictr: u32,
    pub actlr: u32,

    pub nvic_interrupt_enabled: [u32; 16],
    pub nvic_interrupt_pending: [u32; 16],

    pub nvic_interrupt_priority: [u8; 124 * 4],

    pub dwt_ctrl: u32,
    pub dwt_cyccnt: u32,

    pub syst_rvr: u32,
    pub syst_cvr: u32,
    pub syst_csr: u32,

    pub itm_file: Option<Box<io::Write + 'static>>,
}

fn make_default_exception_priorities() -> HashMap<usize, ExceptionState> {
    let mut priorities = HashMap::new();

    priorities.insert(
        Exception::Reset.into(),
        ExceptionState::new(Exception::Reset, -3),
    );
    priorities.insert(
        Exception::NMI.into(),
        ExceptionState::new(Exception::NMI, -2),
    );
    priorities.insert(
        Exception::HardFault.into(),
        ExceptionState::new(Exception::HardFault, -1),
    );

    priorities.insert(
        Exception::MemoryManagementFault.into(),
        ExceptionState::new(Exception::MemoryManagementFault, 0),
    );

    priorities.insert(
        Exception::BusFault.into(),
        ExceptionState::new(Exception::BusFault, 0),
    );

    priorities.insert(
        Exception::UsageFault.into(),
        ExceptionState::new(Exception::UsageFault, 0),
    );

    priorities.insert(
        Exception::DebugMonitor.into(),
        ExceptionState::new(Exception::DebugMonitor, 0),
    );

    priorities.insert(
        Exception::SVCall.into(),
        ExceptionState::new(Exception::SVCall, 0),
    );

    priorities.insert(
        Exception::PendSV.into(),
        ExceptionState::new(Exception::PendSV, 0),
    );

    priorities.insert(
        Exception::SysTick.into(),
        ExceptionState::new(Exception::SysTick, 0),
    );

    for irqn in 0..20 {
        let irq = Exception::Interrupt { n: irqn };
        priorities.insert(irq.into(), ExceptionState::new(irq, 0));
    }

    priorities
}

impl Processor {
    pub fn new(
        itm_file: Option<Box<io::Write + 'static>>,
        code: &[u8],
        semihost_func: Box<FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>,
    ) -> Processor {
        Processor {
            mode: ProcessorMode::ThreadMode,
            vtor: 0,
            psr: PSR { value: 0 },
            primask: false,
            faultmask: false,
            basepri: 0,
            control: Control {
                n_priv: false,
                sp_sel: false,
            },
            r0_12: [0; 13],
            pc: 0,
            msp: 0,
            psp: 0,
            lr: 0,
            code: FlashMemory::new(0, 65536, code),
            sram: RAM::new_with_fill(0x2000_0000, 128 * 1024, 0xcd),
            itm_file: itm_file,
            running: true,
            cycle_count: 0,
            exceptions: make_default_exception_priorities(),
            execution_priority: 0,
            pending_exception_count: 0,
            itstate: 0,
            semihost_func: semihost_func,
            cpuid: 0,
            icsr: 0,
            aircr: 0,
            scr: 0,
            ccr: 0,
            shpr1: 0,
            shpr2: 0,
            shpr3: 0,
            shcsr: 0,
            cfsr: 0,
            dfsr: 0,
            hfsr: 0,
            mmfar: 0,
            bfar: 0,
            afsr: 0,
            cpacr: 0,

            fpccr: 0,
            fpcar: 0,
            fpdscr: 0,
            mvfr0: 0,
            mvfr1: 0,
            mvfr2: 0,

            ictr: 0,
            actlr: 0,

            dwt_ctrl: 0x4000_0000,
            dwt_cyccnt: 0,

            nvic_interrupt_enabled: [0; 16],
            nvic_interrupt_pending: [0; 16],
            nvic_interrupt_priority: [0; 124 * 4],

            //nvic_exception_pending: 0,
            //nvic_exception_active: 0,
            syst_rvr: 0,
            syst_cvr: 0,
            syst_csr: 0,
        }
    }
}

impl fmt::Display for Processor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PC:{:08X} {}{}{}{}{} R0:{:08X} R1:{:08X} R2:{:08X} R3:{:08X} R4:{:08X} R5:{:08X} \
                  R6:{:08X} R7:{:08X} R8:{:08X} R9:{:08X} R10:{:08X} R11:{:08X} R12:{:08X} SP:{:08X} LR:{:08X}",
                 self.get_r(Reg::PC),
                 if self.psr.get_z() {'Z'} else {'z'},
                 if self.psr.get_n() {'N'} else {'n'},
                 if self.psr.get_c() {'C'} else {'c'},
                 if self.psr.get_v() {'V'} else {'v'},
                 if self.psr.get_q() {'Q'} else {'q'},
                 self.get_r(Reg::R0),
                 self.get_r(Reg::R1),
                 self.get_r(Reg::R2),
                 self.get_r(Reg::R3),
                 self.get_r(Reg::R4),
                 self.get_r(Reg::R5),
                 self.get_r(Reg::R6),
                 self.get_r(Reg::R7),
                 self.get_r(Reg::R8),
                 self.get_r(Reg::R9),
                 self.get_r(Reg::R10),
                 self.get_r(Reg::R11),
                 self.get_r(Reg::R12),
                 self.get_r(Reg::SP),
                 self.get_r(Reg::LR))
    }
}
