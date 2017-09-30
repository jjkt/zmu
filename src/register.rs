use std::fmt;

use enum_set::CLike;
use bit_field::BitField;
use std::mem;

pub enum StackPointer {
    MSP(u32),
    PSP(u32),
}


pub trait Apsr {
    fn get_n(&self) -> bool;
    fn set_n(&mut self, n: bool);

    fn get_z(&self) -> bool;
    fn set_z(&mut self, z: bool);

    fn get_c(&self) -> bool;
    fn set_c(&mut self, c: bool);

    fn get_v(&self) -> bool;
    fn set_v(&mut self, v: bool);

    fn get_q(&self) -> bool;
    fn set_q(&mut self, q: bool);
}

pub trait Ipsr {
    fn get_exception_number(&self) -> u8;
}

pub trait Primask {
    fn get_primask(&self) -> bool;
}

pub trait ControlRegister {
    fn get_active_stack_pointer(&self) -> StackPointer;
}


impl Apsr for u32 {
    fn get_n(&self) -> bool {
        (*self).get_bit(31)
    }
    fn set_n(&mut self, n: bool) {
        (*self).set_bit(31, n);
    }

    fn get_z(&self) -> bool {
        (*self).get_bit(30)
    }
    fn set_z(&mut self, z: bool) {
        (*self).set_bit(30, z);
    }

    fn get_c(&self) -> bool {
        (*self).get_bit(29)
    }
    fn set_c(&mut self, c: bool) {
        (*self).set_bit(29, c);
    }
    fn get_v(&self) -> bool {
        (*self).get_bit(28)
    }
    fn set_v(&mut self, v: bool) {
        (*self).set_bit(28, v);
    }

    fn get_q(&self) -> bool {
        (*self).get_bit(27)
    }
    fn set_q(&mut self, q: bool) {
        (*self).set_bit(27, q);
    }
}


#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum Reg {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    SP,
    LR,
    PC,
}

impl CLike for Reg {
    fn to_u32(&self) -> u32 {
        *self as u32
    }

    unsafe fn from_u32(v: u32) -> Reg {
        mem::transmute(v)
    }
}

impl Reg {
    pub fn value(&self) -> usize {
        match *self {
            Reg::R0 => 0,
            Reg::R1 => 1,
            Reg::R2 => 2,
            Reg::R3 => 3,
            Reg::R4 => 4,
            Reg::R5 => 5,
            Reg::R6 => 6,
            Reg::R7 => 7,
            Reg::R8 => 8,
            Reg::R9 => 9,
            Reg::R10 => 10,
            Reg::R11 => 11,
            Reg::R12 => 12,
            Reg::SP => 13,
            Reg::LR => 14,
            Reg::PC => 15,
        }
    }

    pub fn from_u16(n: u16) -> Option<Reg> {
        match n {
            0 => Some(Reg::R0),
            1 => Some(Reg::R1),
            2 => Some(Reg::R2),
            3 => Some(Reg::R3),
            4 => Some(Reg::R4),
            5 => Some(Reg::R5),
            6 => Some(Reg::R6),
            7 => Some(Reg::R7),
            8 => Some(Reg::R8),
            9 => Some(Reg::R9),
            10 => Some(Reg::R10),
            11 => Some(Reg::R11),
            12 => Some(Reg::R12),
            13 => Some(Reg::SP),
            14 => Some(Reg::LR),
            15 => Some(Reg::PC),
            _ => None,
        }
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Reg::R0 => write!(f, "R0"),
            Reg::R1 => write!(f, "R1"),
            Reg::R2 => write!(f, "R2"),
            Reg::R3 => write!(f, "R3"),
            Reg::R4 => write!(f, "R4"),
            Reg::R5 => write!(f, "R5"),
            Reg::R6 => write!(f, "R6"),
            Reg::R7 => write!(f, "R7"),
            Reg::R8 => write!(f, "R8"),
            Reg::R9 => write!(f, "R9"),
            Reg::R10 => write!(f, "R10"),
            Reg::R11 => write!(f, "R11"),
            Reg::R12 => write!(f, "R12"),
            Reg::SP => write!(f, "SP"),
            Reg::LR => write!(f, "LR"),
            Reg::PC => write!(f, "PC"),
        }
    }
}