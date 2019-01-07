use bit_field::BitField;
use enum_set::CLike;
use std::fmt;
use std::mem;

pub struct PSR {
    pub value: u32,
}

pub trait Apsr {
    fn get_n(&self) -> bool;
    fn set_n(&mut self, result: u32);

    fn get_z(&self) -> bool;
    fn set_z(&mut self, result: u32);

    fn get_c(&self) -> bool;
    fn set_c(&mut self, c: bool);

    fn get_v(&self) -> bool;
    fn set_v(&mut self, v: bool);

    fn get_q(&self) -> bool;
    fn set_q(&mut self, q: bool);

    //DSP extensions: GE
    fn set_ge0(&mut self, bit: bool);
    fn set_ge1(&mut self, bit: bool);
    fn set_ge2(&mut self, bit: bool);
    fn set_ge3(&mut self, bit: bool);

    fn get_ge0(&self) -> bool;
    fn get_ge1(&self) -> bool;
    fn get_ge2(&self) -> bool;
    fn get_ge3(&self) -> bool;
}

pub trait Ipsr {
    fn get_exception_number(&self) -> u8;
    fn set_exception_number(&mut self, exception_number: u8);
}

// Execution Program Status register
//
// A view to PSR register containing the data.
pub trait Epsr {
    fn set_t(&mut self, t: bool);
    fn get_t(&self) -> bool;
}

impl Apsr for PSR {
    fn get_n(&self) -> bool {
        (*self).value.get_bit(31)
    }

    fn set_n(&mut self, result: u32) {
        (*self).value &= 0x7fff_ffff;
        (*self).value |= result & 0x8000_0000;
    }

    fn get_z(&self) -> bool {
        (*self).value.get_bit(30)
    }
    fn set_z(&mut self, result: u32) {
        if result == 0 {
            (*self).value |= 0x4000_0000;
        } else {
            (*self).value &= 0x4000_0000 ^ 0xffff_ffff;
        }
    }

    fn get_c(&self) -> bool {
        (*self).value.get_bit(29)
    }
    fn set_c(&mut self, c: bool) {
        if c {
            (*self).value |= 0x2000_0000;
        } else {
            (*self).value &= 0x2000_0000 ^ 0xffff_ffff;
        }
    }
    fn get_v(&self) -> bool {
        (*self).value.get_bit(28)
    }
    fn set_v(&mut self, v: bool) {
        if v {
            (*self).value |= 0x1000_0000;
        } else {
            (*self).value &= 0x1000_0000 ^ 0xffff_ffff;
        }
    }

    fn get_q(&self) -> bool {
        (*self).value.get_bit(27)
    }
    fn set_q(&mut self, q: bool) {
        (*self).value.set_bit(27, q);
    }

    fn set_ge0(&mut self, bit: bool) {
        (*self).value.set_bit(16, bit);
    }
    fn set_ge1(&mut self, bit: bool) {
        (*self).value.set_bit(17, bit);
    }
    fn set_ge2(&mut self, bit: bool) {
        (*self).value.set_bit(18, bit);
    }
    fn set_ge3(&mut self, bit: bool) {
        (*self).value.set_bit(19, bit);
    }

    fn get_ge0(&self) -> bool {
        (*self).value.get_bit(16)
    }
    fn get_ge1(&self) -> bool {
        (*self).value.get_bit(17)
    }
    fn get_ge2(&self) -> bool {
        (*self).value.get_bit(18)
    }
    fn get_ge3(&self) -> bool {
        (*self).value.get_bit(19)
    }
}

impl Epsr for PSR {
    fn get_t(&self) -> bool {
        (*self).value.get_bit(24)
    }
    fn set_t(&mut self, n: bool) {
        (*self).value.set_bit(24, n);
    }
}

impl Ipsr for PSR {
    fn get_exception_number(&self) -> u8 {
        //TODO: diff between cortex m0 and m3+
        (*self).value.get_bits(0..6) as u8
    }
    fn set_exception_number(&mut self, exception_number: u8) {
        self.value = (self.value & 0xffff_ffc0) | u32::from(exception_number & 0b11_1111);
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

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum SpecialReg {
    APSR,
    IAPSR,
    EAPSR,
    XPSR,
    IPSR,
    EPSR,
    IEPSR,
    MSP,
    PSP,
    PRIMASK,
    CONTROL,
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
    pub fn value(self) -> usize {
        match self {
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

impl From<u8> for Reg {
    fn from(value: u8) -> Self {
        match value & 0xf {
            0 => Reg::R0,
            1 => Reg::R1,
            2 => Reg::R2,
            3 => Reg::R3,
            4 => Reg::R4,
            5 => Reg::R5,
            6 => Reg::R6,
            7 => Reg::R7,
            8 => Reg::R8,
            9 => Reg::R9,
            10 => Reg::R10,
            11 => Reg::R11,
            12 => Reg::R12,
            13 => Reg::SP,
            14 => Reg::LR,
            15 => Reg::PC,
            _ => Reg::R0,
        }
    }
}

impl From<u16> for Reg {
    fn from(value: u16) -> Self {
        match value & 0xf {
            0 => Reg::R0,
            1 => Reg::R1,
            2 => Reg::R2,
            3 => Reg::R3,
            4 => Reg::R4,
            5 => Reg::R5,
            6 => Reg::R6,
            7 => Reg::R7,
            8 => Reg::R8,
            9 => Reg::R9,
            10 => Reg::R10,
            11 => Reg::R11,
            12 => Reg::R12,
            13 => Reg::SP,
            14 => Reg::LR,
            15 => Reg::PC,
            _ => Reg::R0,
        }
    }
}

impl From<u32> for Reg {
    fn from(value: u32) -> Self {
        match value & 0xf {
            0 => Reg::R0,
            1 => Reg::R1,
            2 => Reg::R2,
            3 => Reg::R3,
            4 => Reg::R4,
            5 => Reg::R5,
            6 => Reg::R6,
            7 => Reg::R7,
            8 => Reg::R8,
            9 => Reg::R9,
            10 => Reg::R10,
            11 => Reg::R11,
            12 => Reg::R12,
            13 => Reg::SP,
            14 => Reg::LR,
            15 => Reg::PC,
            _ => Reg::R0,
        }
    }
}

impl From<Reg> for u8 {
    fn from(value: Reg) -> Self {
        match value {
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
}

impl From<Reg> for usize {
    fn from(value: Reg) -> Self {
        match value {
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
}

impl SpecialReg {
    pub fn from_u16(n: u16) -> Option<SpecialReg> {
        match n {
            0 => Some(SpecialReg::APSR),
            1 => Some(SpecialReg::IAPSR),
            2 => Some(SpecialReg::EAPSR),
            3 => Some(SpecialReg::XPSR),
            5 => Some(SpecialReg::IPSR),
            6 => Some(SpecialReg::EPSR),
            7 => Some(SpecialReg::IEPSR),
            8 => Some(SpecialReg::MSP),
            9 => Some(SpecialReg::PSP),
            16 => Some(SpecialReg::PRIMASK),
            20 => Some(SpecialReg::CONTROL),
            _ => None,
        }
    }
}

impl From<u8> for SpecialReg {
    fn from(value: u8) -> Self {
        match value & 0x1f {
            0 => SpecialReg::APSR,
            1 => SpecialReg::IAPSR,
            2 => SpecialReg::EAPSR,
            3 => SpecialReg::XPSR,
            5 => SpecialReg::IPSR,
            6 => SpecialReg::EPSR,
            7 => SpecialReg::IEPSR,
            8 => SpecialReg::MSP,
            9 => SpecialReg::PSP,
            16 => SpecialReg::PRIMASK,
            20 => SpecialReg::CONTROL,
            _ => SpecialReg::APSR,
        }
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Reg::R0 => write!(f, "r0"),
            Reg::R1 => write!(f, "r1"),
            Reg::R2 => write!(f, "r2"),
            Reg::R3 => write!(f, "r3"),
            Reg::R4 => write!(f, "r4"),
            Reg::R5 => write!(f, "r5"),
            Reg::R6 => write!(f, "r6"),
            Reg::R7 => write!(f, "r7"),
            Reg::R8 => write!(f, "r8"),
            Reg::R9 => write!(f, "r9"),
            Reg::R10 => write!(f, "r10"),
            Reg::R11 => write!(f, "r11"),
            Reg::R12 => write!(f, "r12"),
            Reg::SP => write!(f, "sp"),
            Reg::LR => write!(f, "lr"),
            Reg::PC => write!(f, "pc"),
        }
    }
}

impl fmt::Display for SpecialReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpecialReg::APSR => write!(f, "APSR"),
            SpecialReg::IAPSR => write!(f, "IAPSR"),
            SpecialReg::EAPSR => write!(f, "EAPSR"),
            SpecialReg::XPSR => write!(f, "XPSR"),
            SpecialReg::IPSR => write!(f, "IPSR"),
            SpecialReg::EPSR => write!(f, "EPSR"),
            SpecialReg::IEPSR => write!(f, "IEPSR"),
            SpecialReg::MSP => write!(f, "MSP"),
            SpecialReg::PSP => write!(f, "PSP"),
            SpecialReg::PRIMASK => write!(f, "PRIMASK"),
            SpecialReg::CONTROL => write!(f, "CONTROL"),
        }
    }
}

pub struct Control {
    pub n_priv: bool,
    pub sp_sel: bool,
}

impl From<Control> for u8 {
    fn from(control: Control) -> Self {
        control.n_priv as u8 + ((control.sp_sel as u8) << 1)
    }
}
