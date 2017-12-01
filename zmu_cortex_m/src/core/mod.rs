
pub mod condition;
pub mod executor;
pub mod instruction;
pub mod operation;
pub mod register;

use bus::Bus;
use core::executor::execute;
use decoder::{decode_16, decode_32, is_thumb32};
use core::register::{Reg, PSR, Epsr, Apsr, Control};
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


impl From<u16> for ThumbCode {
    fn from(value : u16) -> Self{
        ThumbCode::Thumb16 {half_word : value}
    }
}

impl From<u32> for ThumbCode {
    fn from(value : u32)-> Self{
        ThumbCode::Thumb32 {half_word : (value & 0xffff) as u16, half_word2: ((value >> 16)& 0xffff) as u16}
    }
}
impl fmt::Display for ThumbCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ThumbCode::Thumb16 {half_word} => write!(f, "0x{:x}", half_word),
            ThumbCode::Thumb32 {half_word, half_word2}=> write!(f, "0x{}, 0x{}", half_word, half_word2),
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

    /* Processor state register, status flags. */ 
    pub psr: PSR,

    /* interrupt primary mask, a 1 bit mask register for 
       global interrupt masking. */ 
    pub primask: bool,

    /* Control bits: currently used stack and execution privilege if core.mode == ThreadMode */ 
    pub control: Control,

    /* Processor mode: either handler or thread mode. */ 
    pub mode: ProcessorMode,

    /* Bus to which the core is connected. */ 
    pub bus: &'a mut T,
}

impl<'a, T: Bus> Core<'a, T> {
    pub fn new(bus: &'a mut T) -> Core<'a, T> {
        Core {
            mode: ProcessorMode::ThreadMode,
            psr: PSR { value: 0 },
            primask: false,
            control: Control { nPriv: false, spSel : false},
            r0_12: [0; 13],
            pc : 0,
            msp : 0,
            psp : 0,
            lr : 0,
            bus: bus,
        }
    }


    //
    // Getter for registers
    //
    pub fn get_r(&self, r : &Reg) -> u32 {
        match *r {
                Reg::R0|Reg::R1|Reg::R2|Reg::R3|Reg::R4|Reg::R5|Reg::R6|Reg::R7|Reg::R8|Reg::R9|Reg::R10|Reg::R11|Reg::R12 => self.r0_12[r.value()],
    Reg::SP => if self.control.spSel {self.msp} else { self.psp},
    Reg::LR => self.lr, 
    Reg::PC => self.pc

        }
    }
    //
    // Setter for registers
    //
    pub fn set_r(&mut self, r : &Reg, value: u32) {
        match *r {
                Reg::R0|Reg::R1|Reg::R2|Reg::R3|Reg::R4|Reg::R5|Reg::R6|Reg::R7|Reg::R8|Reg::R9|Reg::R10|Reg::R11|Reg::R12 => self.r0_12[r.value()] = value,
    Reg::SP => if self.control.spSel {self.msp = value} else { self.psp = value},
    Reg::LR => self.lr = value, 
    Reg::PC => self.pc = value

        };
    }

    pub fn add_pc(&mut self, value: u32) {
        self.pc += value;
    }

    //
    // Setter for registers
    //
    pub fn add_r(&mut self, r : &Reg, value: u32) {
        match *r {
                Reg::R0|Reg::R1|Reg::R2|Reg::R3|Reg::R4|Reg::R5|Reg::R6|Reg::R7|Reg::R8|Reg::R9|Reg::R10|Reg::R11|Reg::R12 => self.r0_12[r.value()] += value,
    Reg::SP => if self.control.spSel {self.msp = value} else { self.psp += value},
    Reg::LR => self.lr += value, 
    Reg::PC => self.pc += value

        };
    }

    //
    // Reset the cpu core
    //
    pub fn reset(&mut self) {
        let reset_vector = self.bus.read32(4);
        //println!("\nRESET");

        self.set_r(&Reg::PC, reset_vector & 0xffff_fffe);
        self.psr.set_t((reset_vector & 1) == 1);
        let sp = self.bus.read32(0);
        self.set_r(&Reg::SP, sp);
    }

    // Fetch next Thumb2-coded instruction from current
    // PC location. Depending on instruction type, fetches
    // one or two half-words.
    pub fn fetch(&mut self) -> ThumbCode {
        let hw = self.bus.read16(self.pc);

        if is_thumb32(hw) {
            let hw2 = self.bus.read16(self.pc + 2);
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

