
pub mod condition;
pub mod executor;
pub mod instruction;
pub mod operation;
pub mod register;

use bus::Bus;
use core::executor::execute;
use decoder::{decode_16, decode_32, is_thumb32};
use core::register::{Reg, PSR, Epsr, Apsr};
use core::instruction::Instruction;
use std::fmt;

pub enum ProcessorMode {
    ThreadMode,
    HandlerMode,
}

pub enum ThumbCode {
    Thumb32 { half_word: u16, half_word2: u16 },
    Thumb16 { half_word: u16 },
}

pub struct Core<'a, T: Bus + 'a> {
    pub r: [u32; 16],

    pub psr: PSR,

    pub primask: u32,
    pub control: u32,

    pub mode: ProcessorMode,
    pub bus: &'a mut T,
}

impl<'a, T: Bus> Core<'a, T> {
    pub fn new(bus: &'a mut T) -> Core<'a, T> {
        Core {
            mode: ProcessorMode::ThreadMode,
            psr: PSR { value: 0 },
            primask: 0,
            control: 0,
            r: [0; 16],
            bus: bus,
        }
    }


    //
    // Getter for Stack pointer.
    // Depending on the control more, the SP is MSP or PSP
    //
    pub fn get_sp(&self) -> u32 {
        self.r[Reg::SP.value()]
    }

    //
    // Setter for Stack pointer.
    // Depending on the control more, the SP is MSP or PSP
    //
    pub fn set_sp(&mut self, value: u32) {
        self.r[Reg::SP.value()] = value;
    }


    //
    // Reset the cpu core
    //
    pub fn reset(&mut self) {
        let reset_vector = self.bus.read32(4);
        println!("\nRESET");

        self.r[Reg::PC.value()] = reset_vector & 0xffff_fffe;
        self.psr.set_t((reset_vector & 1) == 1);
        let sp = self.bus.read32(0);
        self.set_sp(sp);
    }

    // Fetch next Thumb2-coded instruction from current
    // PC location. Depending on instruction type, fetches
    // one or two half-words.
    pub fn fetch(&mut self) -> ThumbCode {
        let hw = self.bus.read16(self.r[Reg::PC.value()]);

        if is_thumb32(hw) {
            let hw2 = self.bus.read16(self.r[Reg::PC.value()] + 2);
            ThumbCode::Thumb32 {
                half_word: hw,
                half_word2: hw2,
            }
        } else {
            ThumbCode::Thumb16 { half_word: hw }
        }
    }

    // Decode ThumbCode into Instruction
    pub fn decode(&self, code: &ThumbCode) -> Instruction {
        match *code {
            ThumbCode::Thumb32 { half_word, half_word2 } => decode_32(half_word, half_word2),
            ThumbCode::Thumb16 { half_word } => decode_16(half_word),
        }
    }

    // Run single instruction on core
    // bkpt_func: 
    pub fn step<F>(&mut self, instruction: &Instruction, bkpt_func: F)
        where F: FnMut(u32, u32, u32)
    {

        execute(self, instruction, bkpt_func);
    }
}

impl<'a, T: Bus> fmt::Display for Core<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "PC:{:08X} {}{}{}{}{} R0:{:08X} R1:{:08X} R2:{:08X} R3:{:08X} R4:{:08X} R5:{:08X} \
                  R6:{:08X} R7:{:08X} R8:{:08X} R9:{:08X} R10:{:08X} R11:{:08X} R12:{:08X} SP:{:08X} LR:{:08X}",
                 self.r[Reg::PC.value()],
                 if self.psr.get_z() {'Z'} else {'z'},
                 if self.psr.get_n() {'N'} else {'n'},
                 if self.psr.get_c() {'C'} else {'c'},
                 if self.psr.get_v() {'V'} else {'v'},
                 if self.psr.get_q() {'Q'} else {'q'},
                 self.r[Reg::R0.value()],
                 self.r[Reg::R1.value()],
                 self.r[Reg::R2.value()],
                 self.r[Reg::R3.value()],
                 self.r[Reg::R4.value()],
                 self.r[Reg::R5.value()],
                 self.r[Reg::R6.value()],
                 self.r[Reg::R7.value()],
                 self.r[Reg::R8.value()],
                 self.r[Reg::R9.value()],
                 self.r[Reg::R10.value()],
                 self.r[Reg::R11.value()],
                 self.r[Reg::R12.value()],
                 self.r[Reg::SP.value()],
                 self.r[Reg::LR.value()])
    }
}

