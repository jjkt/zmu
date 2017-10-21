
pub mod condition;
pub mod executor;
pub mod instruction;
pub mod operation;
pub mod register;

use bus::Bus;
use core::executor::execute;
use decoder::{decode_16, decode_32, is_thumb32};
use core::register::{Reg, PSR, Epsr};
use core::instruction::Instruction;

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

    pub fn decode(&self, code: &ThumbCode) -> Option<Instruction> {
        match *code {
            ThumbCode::Thumb32 { half_word, half_word2 } => decode_32(half_word, half_word2),
            ThumbCode::Thumb16 { half_word } => decode_16(half_word),
        }
    }

    pub fn step<F>(&mut self, instruction: Instruction, bkpt_func: F)
        where F: FnMut(u32, u32, u32)
    {

        execute(self, instruction, bkpt_func);
    }
}
