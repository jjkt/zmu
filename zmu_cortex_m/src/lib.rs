//!
//! Processor simulator for ARM Cortex-M processors.
//!
//!
#![deny(missing_docs)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::pub_enum_variant_names)]
#![allow(clippy::inline_always)]
// TODO: check these case by case, add unit tests
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
// TODO: check these case by case, there might be need to add more error handling
#![allow(clippy::match_same_arms)]
// TODO check if some filtering can be simplified
#![allow(clippy::filter_map)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::new_without_default)]

extern crate byteorder;
extern crate enum_set;

pub mod bus;
pub mod core;
pub mod decoder;
pub mod memory;
pub mod peripheral;
pub mod semihosting;
pub mod system;
pub mod device;



use crate::core::instruction::instruction_size;

use crate::core::exception::Exception;
use crate::core::fetch::Fetch;
use crate::core::instruction::Instruction;
use crate::core::register::{Apsr, BaseReg, Control, Reg, PSR};
use crate::decoder::Decoder;
use crate::memory::flash::FlashMemory;
use crate::memory::map::MemoryMapConfig;
use crate::memory::ram::RAM;
use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;

use crate::core::exception::ExceptionState;
use std::collections::HashMap;
use std::fmt;
use std::io;

#[cfg(feature = "stm32f103")]
use crate::device::stm32f1xx::Device as Device;

#[cfg(feature = "generic-device")]
use crate::device::generic::Device as Device;


#[derive(PartialEq, Debug, Copy, Clone)]
/// Main execution mode of the processor
pub enum ProcessorMode {
    /// Thread mode
    ThreadMode,
    /// Handler mode
    HandlerMode,
}

///
/// Representation of all Processor related data
///
#[allow(missing_docs)]
pub struct Processor {
    /// 13 of 32-bit general purpose registers.
    pub r0_12: [u32; 13],

    /// MSP, virtual reg r[13]
    msp: u32,
    /// PSP, virtual reg r[13]
    psp: u32,
    lr: u32,
    pc: u32,

    /// Total number of processor clock cycles run
    pub cycle_count: u64,
    pub instruction_count: u64,

    /// Processor state register, status flags.
    pub psr: PSR,

    ///
    /// interrupt primary mask, a 1 bit mask register for
    /// global interrupt masking
    ///
    primask: bool,
    ///
    /// interrupt fault mask, a 1 bit mask register for
    /// global interrupt masking
    ///
    #[cfg(any(armv7m, armv7em))]
    faultmask: bool,
    ///
    /// basepri for selection of executed interrupt priorities
    ///
    basepri: u8,

    ///
    /// Control bits: currently used stack and execution privilege if core.mode == ThreadMode
    ///
    control: Control,

    ///
    /// Processor mode: either handler or thread mode.
    ///
    mode: ProcessorMode,

    ///
    /// processor simulation state
    ///
    /// bit 0 : 1= simulation running, 0 : simulation terminating
    /// bit 1 : 1= processor sleeping, 0 : processor awake
    pub state: u32,

    ///
    /// lookup table for exceptions and their states
    ///
    pub exceptions: HashMap<usize, ExceptionState>,

    ///
    /// number of exceptions currently pending, used for optimization purposes
    ///
    pub pending_exception_count: u32,

    ///
    /// cached current execution priority, used for optimization
    /// of exception activation rule resolving
    ///
    pub execution_priority: i16,

    itstate: u8,

    ///
    /// flash memory data
    ///
    pub code: FlashMemory,
    ///
    /// ram data
    ///
    pub sram: RAM,

    pub cpuid: u32,
    pub icsr: u32,
    pub vtor: u32,
    pub aircr: u32,
    pub scr: u32,
    pub ccr: u32,
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

    pub dwt_ctrl: u32,
    pub dwt_cyccnt: u32,

    pub syst_rvr: u32,
    pub syst_cvr: u32,
    pub syst_csr: u32,

    ///
    /// file handle to which to write ITM data
    ///
    pub itm_file: Option<Box<io::Write + 'static>>,

    ///
    /// semihosting plug
    ///
    semihost_func: Option<Box<FnMut(&SemihostingCommand) -> SemihostingResponse>>,

    instruction_cache: Vec<(Instruction, usize)>,

    pub last_pc: u32,

    mem_map: Option<MemoryMapConfig>,

    pub device : Device
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

    for irqn in 0..32 {
        let irq = Exception::Interrupt { n: irqn };
        priorities.insert(irq.into(), ExceptionState::new(irq, 0));
    }

    priorities
}

impl Processor {
    ///
    /// Create processor with default data
    ///
    pub fn new() -> Self {
        Self {
            mode: ProcessorMode::ThreadMode,
            vtor: 0,
            psr: PSR { value: 0 },
            primask: false,
            #[cfg(any(armv7m, armv7em))]
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
            code: FlashMemory::new(65536, &[0; 65536]),
            // TODO make RAM size configurable
            sram: RAM::new_with_fill(0x2000_0000, 128 * 1024, 0xcd),
            itm_file: None,
            state: 0,
            cycle_count: 0,
            instruction_count: 0,
            exceptions: make_default_exception_priorities(),
            execution_priority: 0,
            pending_exception_count: 0,
            itstate: 0,
            semihost_func: None,
            cpuid: 0,
            icsr: 0,
            aircr: 0,
            scr: 0,
            ccr: 0,
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
            syst_rvr: 0,
            syst_cvr: 0,
            syst_csr: 0,
            instruction_cache: Vec::new(),
            last_pc: 0,
            mem_map: None,
            device : Device::new()
        }
    }

    /// Configure flash memory
    pub fn flash_memory<'a>(&'a mut self, flash_size: usize, code: &[u8]) -> &'a mut Self {
        self.code = FlashMemory::new(flash_size, code);
        self
    }

    /// Configure memory mapping
    pub fn memory_map(&mut self, map: Option<MemoryMapConfig>) -> &mut Self {
        self.mem_map = map;
        self
    }

    /// Configure itm output file
    pub fn itm<'a>(&'a mut self, file: Option<Box<io::Write + 'static>>) -> &'a mut Self {
        self.itm_file = file;
        self
    }

    /// Configure semihosting
    pub fn semihost<'a>(
        &'a mut self,
        func: Option<Box<FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>>,
    ) -> &'a mut Self {
        self.semihost_func = func;
        self
    }

    ///
    /// Pre cache (decode) instructions to speed up simulation
    ///
    pub fn cache_instructions(&mut self) {
        // pre-cache the decoded instructions
        {
            let mut pc = 0;

            while pc < (self.code.len() as u32) {
                let thumb = self.fetch(pc).unwrap();
                let instruction = self.decode(thumb);
                self.instruction_cache
                    .push((instruction, instruction_size(&instruction)));
                pc += 2;
            }
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

