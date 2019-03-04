pub mod bits;
pub mod condition;
pub mod exception;
pub mod executor;
pub mod fault;
pub mod fetch;
pub mod instruction;
pub mod operation;
pub mod register;
pub mod reset;
pub mod thumb;

use crate::core::condition::Condition;
use crate::core::register::{Apsr, BaseReg, Control, Reg, PSR};
use crate::memory::flash::FlashMemory;
use crate::memory::ram::RAM;
use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;

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

    /* Control bits: currently used stack and execution privilege if core.mode == ThreadMode */
    control: Control,

    /* Processor mode: either handler or thread mode. */
    mode: ProcessorMode,

    /* Is the core simulation currently running or not.*/
    pub running: bool,

    /* One boolean per exception on the system: fixed priority system exceptions,
    configurable priority system exceptions and external exceptions. */
    pub exception_active: [bool; 64],

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
    pub nvic_interrupt_active: [u32; 16],

    pub nvic_interrupt_priority: [u8; 124 * 4],

    pub dwt_ctrl: u32,
    pub dwt_cyccnt: u32,

    pub syst_rvr: u32,
    pub syst_cvr: u32,
    pub syst_csr: u32,

    pub itm_file: Option<Box<io::Write + 'static>>,
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
            exception_active: [false; 64],
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
            nvic_interrupt_active: [0; 16],
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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::bus::Bus;
    use crate::core::exception::ExceptionHandling;
    use crate::core::exception::Exception;
    use crate::core::register::Ipsr;
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
            core.push_stack(Exception::HardFault, 99);

            assert_eq!(core.msp, STACK_START - 32);
            core.get_r(Reg::LR)
        };

        // values pushed on to stack
        assert_eq!(core.read32(STACK_START - 0x20), 42);
        assert_eq!(core.read32(STACK_START - 0x20 + 4), 43);
        assert_eq!(core.read32(STACK_START - 0x20 + 8), 44);
        assert_eq!(core.read32(STACK_START - 0x20 + 12), 45);
        assert_eq!(core.read32(STACK_START - 0x20 + 16), 46);
        assert_eq!(core.read32(STACK_START - 0x20 + 20), 47);
        assert_eq!(core.read32(STACK_START - 0x20 + 24), 99);
        assert_eq!(
            core.read32(STACK_START - 0x20 + 28),
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
        core.exception_taken(Exception::BusFault);

        // Assert
        assert_eq!(core.control.sp_sel, false);
        assert_eq!(core.mode, ProcessorMode::HandlerMode);
        assert_eq!(core.psr.get_exception_number(), Exception::BusFault.into());
        assert_eq!(
            core.exception_active[u8::from(Exception::BusFault) as usize],
            true
        );
    }

}
