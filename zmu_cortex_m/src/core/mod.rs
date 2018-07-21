pub mod bits;
pub mod condition;
pub mod exception;
pub mod executor;
pub mod fault;
pub mod instruction;
pub mod operation;
pub mod register;

use bus::Bus;
use core::bits::*;
use core::exception::Exception;
use core::executor::execute;
use core::instruction::Instruction;
use core::register::{Apsr, Control, Epsr, Ipsr, Reg, PSR};
use decoder::{decode_16, decode_32, is_thumb32};
use semihosting::SemihostingCommand;
use semihosting::SemihostingResponse;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum ProcessorMode {
    ThreadMode,
    HandlerMode,
}

#[derive(PartialEq, Debug)]
pub enum ThumbCode {
    Thumb32 { opcode: u32 },
    Thumb16 { half_word: u16 },
}

impl From<u16> for ThumbCode {
    fn from(value: u16) -> Self {
        ThumbCode::Thumb16 { half_word: value }
    }
}

impl From<u32> for ThumbCode {
    fn from(value: u32) -> Self {
        ThumbCode::Thumb32 { opcode: value }
    }
}
impl fmt::Display for ThumbCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ThumbCode::Thumb16 { half_word } => write!(f, "0x{:x}", half_word),
            ThumbCode::Thumb32 { opcode } => write!(f, "0x{:x}", opcode),
        }
    }
}

pub struct Core<'a, T: Bus + 'a> {
    /* 13 of 32-bit general purpose registers. */
    r0_12: [u32; 13],

    msp: u32, //MSP, virtual reg r[13]
    psp: u32, //PSP, virtual reg r[13]
    lr: u32,
    pc: u32,

    // TODO, vtor is in SCS
    vtor: u32,

    pub cycle_count: u64,

    /* Processor state register, status flags. */
    psr: PSR,

    /* interrupt primary mask, a 1 bit mask register for 
       global interrupt masking. */
    primask: bool,

    /* Control bits: currently used stack and execution privilege if core.mode == ThreadMode */
    control: Control,

    /* Processor mode: either handler or thread mode. */
    mode: ProcessorMode,

    /* Bus to which the core is connected. */
    pub bus: &'a mut T,

    /* Is the core simulation currently running or not.*/
    pub running: bool,

    /* One bool per exception on the system: fixed priority system exceptions, configurable priority system exceptions and external exceptions. */
    pub exception_active: [bool; 64],
}

impl<'a, T: Bus> Core<'a, T> {
    pub fn new(bus: &'a mut T) -> Core<'a, T> {
        Core {
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
            bus: bus,
            running: true,
            cycle_count: 0,
            exception_active: [false; 64],
        }
    }

    pub fn branch_write_pc(&mut self, address: u32) {
        self.set_pc(address & 0xffff_fffe);
    }

    //
    // interworking branch
    //
    pub fn blx_write_pc(&mut self, address: u32) {
        self.psr.set_t((address & 1) == 1);
        self.branch_write_pc(address);
    }

    //
    // interworking branch
    //
    pub fn bx_write_pc(&mut self, address: u32) {
        if self.mode == ProcessorMode::HandlerMode && (bits_31_28(address) == 0b1111) {
            self.exception_return(bits_27_0(address));
        } else {
            self.blx_write_pc(address);
        }
    }

    //
    // Getter for registers
    //
    pub fn get_r(&self, r: &Reg) -> u32 {
        match *r {
            Reg::R0
            | Reg::R1
            | Reg::R2
            | Reg::R3
            | Reg::R4
            | Reg::R5
            | Reg::R6
            | Reg::R7
            | Reg::R8
            | Reg::R9
            | Reg::R10
            | Reg::R11
            | Reg::R12 => {
                let reg: usize = From::from(*r);
                self.r0_12[reg]
            }
            Reg::SP => if self.control.sp_sel {
                self.psp
            } else {
                self.msp
            },
            Reg::LR => self.lr,
            Reg::PC => self.pc + 4,
        }
    }
    //
    // Setter for registers
    //
    pub fn set_r(&mut self, r: &Reg, value: u32) {
        match *r {
            Reg::R0
            | Reg::R1
            | Reg::R2
            | Reg::R3
            | Reg::R4
            | Reg::R5
            | Reg::R6
            | Reg::R7
            | Reg::R8
            | Reg::R9
            | Reg::R10
            | Reg::R11
            | Reg::R12 => {
                let reg: usize = From::from(*r);
                self.r0_12[reg] = value;
            }
            Reg::SP => if self.control.sp_sel {
                self.psp = value
            } else {
                self.msp = value
            },
            Reg::LR => self.lr = value,
            Reg::PC => panic!("use branch commands instead"),
        };
    }

    pub fn set_msp(&mut self, value: u32) {
        self.msp = value;
    }

    pub fn set_psp(&mut self, value: u32) {
        self.psp = value;
    }

    pub fn add_pc(&mut self, value: u32) {
        self.pc += value;
    }

    pub fn get_pc(&mut self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, value: u32) {
        self.pc = value
    }

    //
    // Add value to register
    //
    pub fn add_r(&mut self, r: &Reg, value: u32) {
        match *r {
            Reg::R0
            | Reg::R1
            | Reg::R2
            | Reg::R3
            | Reg::R4
            | Reg::R5
            | Reg::R6
            | Reg::R7
            | Reg::R8
            | Reg::R9
            | Reg::R10
            | Reg::R11
            | Reg::R12 => {
                let reg: usize = From::from(*r);
                self.r0_12[reg] += value;
            }
            Reg::SP => if self.control.sp_sel {
                self.psp = value
            } else {
                self.msp += value
            },
            Reg::LR => self.lr += value,
            Reg::PC => self.pc += value,
        };
    }

    //
    // Reset Exception
    //
    pub fn reset(&mut self) {
        // All basic registers to zero.
        self.r0_12[0] = 0;
        self.r0_12[1] = 0;
        self.r0_12[2] = 0;
        self.r0_12[3] = 0;
        self.r0_12[4] = 0;
        self.r0_12[5] = 0;
        self.r0_12[6] = 0;
        self.r0_12[7] = 0;
        self.r0_12[8] = 0;
        self.r0_12[9] = 0;
        self.r0_12[10] = 0;
        self.r0_12[11] = 0;

        // Main stack pointer is read via vector table
        let vtor = self.vtor;
        let sp = self.bus.read32(vtor) & 0xffff_fffc;
        self.set_msp(sp);

        // Process stack pointer to zero
        self.set_psp(0);

        // Link Register
        self.lr = 0;

        // Mode
        self.mode = ProcessorMode::ThreadMode;

        // Apsr, ipsr
        self.psr = PSR { value: 0 };
        self.primask = false;
        self.control.sp_sel = false;
        self.control.n_priv = false;

        //TODO self.scs.reset();
        //TODOself.exceptions.clear();

        //self.event_reg.clear();

        let reset_vector = self.bus.read32(vtor + 4);

        self.blx_write_pc(reset_vector);
    }

    fn push_stack(&mut self, return_address: u32) {
        const FRAME_SIZE: u32 = 0x20;

        let (frameptr, frameptralign) =
            if self.control.sp_sel && self.mode == ProcessorMode::ThreadMode {
                let align = bit_2(self.psp as u16) as u32;
                println!("thread mode");
                // forces 8 byte alignment on the stack
                self.psp = (self.psp - FRAME_SIZE) & (4 ^ 0xFFFF_FFFF);
                (self.psp, align)
            } else {
                let align = bit_2(self.msp as u16) as u32;
                // forces 8 byte alignment on the stack
                self.msp = (self.msp - FRAME_SIZE) & (4 ^ 0xFFFF_FFFF);
                println!("processor mode");
                (self.msp, align)
            };

        let r0 = self.get_r(&Reg::R0);
        let r1 = self.get_r(&Reg::R1);
        let r2 = self.get_r(&Reg::R2);
        let r3 = self.get_r(&Reg::R3);
        let r12 = self.get_r(&Reg::R12);
        let lr = self.get_r(&Reg::LR);

        self.bus.write32(frameptr, r0);
        self.bus.write32(frameptr + 0x4, r1);
        self.bus.write32(frameptr + 0x8, r2);
        self.bus.write32(frameptr + 0xc, r3);
        self.bus.write32(frameptr + 0x10, r12);
        self.bus.write32(frameptr + 0x14, lr);
        self.bus.write32(frameptr + 0x18, return_address);
        let xpsr = (self.psr.value & 0b1111_1111_1111_1111_1111_1101_1111_1111)
            | (frameptralign << 9) as u32;
        self.bus.write32(frameptr + 0x1c, xpsr);

        if self.mode == ProcessorMode::HandlerMode {
            self.lr = 0xFFFF_FFF1;
        } else {
            if self.control.sp_sel == false {
                self.lr = 0xFFFF_FFF9;
            } else {
                self.lr = 0xFFFF_FFFD;
            }
        }
    }

    pub fn exception_taken(&mut self, exception_number: u8) {
        self.control.sp_sel = false;
        self.mode = ProcessorMode::HandlerMode;
        self.psr.set_exception_number(exception_number);
        self.exception_active[exception_number as usize] = true;

        // SetEventRegister();
        // InstructionSynchronizationBarrier();
        let vtor = self.vtor;
        let start = self.bus.read32(vtor + u32::from(exception_number) * 4);
        self.blx_write_pc(start);
    }

    pub fn exception_entry(&mut self, exception_number: u8, return_address: u32) {
        self.push_stack(return_address);
        self.exception_taken(exception_number);
    }

    #[allow(unused_variables)]
    pub fn exception_return(&mut self, exc_return: u32) {
        unimplemented!();
    }

    // Fetch next Thumb2-coded instruction from current
    // PC location. Depending on instruction type, fetches
    // one or two half-words.
    pub fn fetch(&mut self) -> ThumbCode {
        let hw = self.bus.read16(self.pc);

        if is_thumb32(hw) {
            let hw2 = self.bus.read16(self.pc + 2);
            ThumbCode::Thumb32 {
                opcode: ((hw as u32) << 16) + hw2 as u32,
            }
        } else {
            ThumbCode::Thumb16 { half_word: hw }
        }
    }

    // Decode ThumbCode into Instruction
    pub fn decode(&self, code: &ThumbCode) -> Instruction {
        match *code {
            ThumbCode::Thumb32 { opcode } => decode_32(opcode),
            ThumbCode::Thumb16 { half_word } => decode_16(half_word),
        }
    }

    // Run single instruction on core
    pub fn step<F>(&mut self, instruction: &Instruction, semihost_func: F)
    where
        F: FnMut(&SemihostingCommand) -> SemihostingResponse,
    {
        match execute(self, instruction, semihost_func) {
            Some(_fault) => {
                // all faults are mapped to hardfaults on armv6m
                let pc = self.get_pc();
                self.exception_entry(u8::from(Exception::HardFault), pc);
            }
            None => {}
        }
    }
}

impl<'a, T: Bus> fmt::Display for Core<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PC:{:08X} {}{}{}{}{} R0:{:08X} R1:{:08X} R2:{:08X} R3:{:08X} R4:{:08X} R5:{:08X} \
                  R6:{:08X} R7:{:08X} R8:{:08X} R9:{:08X} R10:{:08X} R11:{:08X} R12:{:08X} SP:{:08X} LR:{:08X}",
                 self.get_r(&Reg::PC),
                 if self.psr.get_z() {'Z'} else {'z'},
                 if self.psr.get_n() {'N'} else {'n'},
                 if self.psr.get_c() {'C'} else {'c'},
                 if self.psr.get_v() {'V'} else {'v'},
                 if self.psr.get_q() {'Q'} else {'q'},
                 self.get_r(&Reg::R0),
                 self.get_r(&Reg::R1),
                 self.get_r(&Reg::R2),
                 self.get_r(&Reg::R3),
                 self.get_r(&Reg::R4),
                 self.get_r(&Reg::R5),
                 self.get_r(&Reg::R6),
                 self.get_r(&Reg::R7),
                 self.get_r(&Reg::R8),
                 self.get_r(&Reg::R9),
                 self.get_r(&Reg::R10),
                 self.get_r(&Reg::R11),
                 self.get_r(&Reg::R12),
                 self.get_r(&Reg::SP),
                 self.get_r(&Reg::LR))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use memory::ram::*;

    #[test]
    fn test_push_stack() {
        // arrange
        let mut bus = RAM::new(0, 1000);
        let lr = {
            let mut core = Core::new(&mut bus);
            //    if self.control.sp_sel && self.mode == ProcessorMode::ThreadMode {
            core.control.sp_sel = false;
            //core.mode = ProcessorMode::ThreadMode;
            core.set_r(&Reg::R0, 42);
            core.set_r(&Reg::R1, 43);
            core.set_r(&Reg::R2, 44);
            core.set_r(&Reg::R3, 45);
            core.set_r(&Reg::R12, 46);
            core.set_r(&Reg::LR, 47);
            core.set_psp(0);
            core.set_msp(0x100);
            core.psr.value = 0xffff_ffff;

            // act
            core.push_stack(99);

            assert_eq!(core.msp, 0xe0);
            core.get_r(&Reg::LR)
        };

        // values pushed on to stuck
        assert_eq!(bus.read32(0x100 - 0x20), 42);
        assert_eq!(bus.read32(0x100 - 0x20 + 4), 43);
        assert_eq!(bus.read32(0x100 - 0x20 + 8), 44);
        assert_eq!(bus.read32(0x100 - 0x20 + 12), 45);
        assert_eq!(bus.read32(0x100 - 0x20 + 16), 46);
        assert_eq!(bus.read32(0x100 - 0x20 + 20), 47);
        assert_eq!(bus.read32(0x100 - 0x20 + 24), 99);
        assert_eq!(
            bus.read32(0x100 - 0x20 + 28),
            0b1111_1111_1111_1111_1111_1101_1111_1111
        );
        assert_eq!(lr, 0xffff_fff9);
    }

    #[test]
    fn test_exception_taken() {
        // Arrange
        let mut bus = RAM::new(0, 1000);
        let mut core = Core::new(&mut bus);

        core.control.sp_sel = true;
        core.mode = ProcessorMode::ThreadMode;
        core.psr.value = 0xffff_ffff;

        // Act
        core.exception_taken(5);

        // Assert
        assert_eq!(core.control.sp_sel, false);
        assert_eq!(core.mode, ProcessorMode::HandlerMode);
        assert_eq!(core.psr.get_exception_number(), 5);
        assert_eq!(core.exception_active[5], true);
    }

}
