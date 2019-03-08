//!
//! Cortex core register operations
//!

use crate::core::bits::Bits;
use crate::core::exception::ExceptionHandling;
use crate::core::fault::Fault;
use crate::Processor;
use crate::ProcessorMode;
use enum_set::CLike;
use std::fmt;
use std::mem;

///
/// Base register manipulation
///
pub trait BaseReg {
    ///
    /// set PC without touching the T bit
    ///
    fn branch_write_pc(&mut self, address: u32);

    ///
    /// interworking branch
    ///
    fn blx_write_pc(&mut self, address: u32);

    ///
    /// interworking branch
    ///
    fn bx_write_pc(&mut self, address: u32) -> Result<(), Fault>;

    ///
    /// alias for bx_write_pc
    ///
    fn load_write_pc(&mut self, address: u32) -> Result<(), Fault>;

    ///
    /// Getter for registers
    ///
    fn get_r(&self, r: Reg) -> u32;

    ///
    /// Setter for registers
    ///
    fn set_r(&mut self, r: Reg, value: u32);

    ///
    /// Setter for MSP
    ///
    fn set_msp(&mut self, value: u32);

    ///
    /// Setter for PSP
    ///
    fn set_psp(&mut self, value: u32);

    ///
    /// Getter for MSP
    fn get_msp(&self) -> u32;

    ///
    /// Getter for PSP
    ///
    fn get_psp(&self) -> u32;

    ///
    /// Increment PC by a value
    ///
    fn add_pc(&mut self, value: u32);

    ///
    /// Get current PC value
    ///
    fn get_pc(&mut self) -> u32;

    ///
    /// Set current PC value with no side effects
    ///
    fn set_pc(&mut self, value: u32);

    ///
    /// add value to register
    ///
    fn add_r(&mut self, r: Reg, value: u32);

    ///
    /// substract value from a register
    ///
    fn sub_r(&mut self, r: Reg, value: u32);
}

impl BaseReg for Processor {
    fn branch_write_pc(&mut self, address: u32) {
        self.set_pc(address & 0xffff_fffe);
    }

    fn blx_write_pc(&mut self, address: u32) {
        self.psr.set_t((address & 1) == 1);
        self.branch_write_pc(address);
    }

    fn bx_write_pc(&mut self, address: u32) -> Result<(), Fault> {
        if self.mode == ProcessorMode::HandlerMode && (address.get_bits(28..32) == 0b1111) {
            self.exception_return(address.get_bits(0..28))
        } else {
            self.blx_write_pc(address);
            Ok(())
        }
    }

    fn load_write_pc(&mut self, address: u32) -> Result<(), Fault> {
        self.bx_write_pc(address)
    }

    fn get_r(&self, r: Reg) -> u32 {
        match r {
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
                let reg: usize = From::from(r);
                self.r0_12[reg]
            }
            Reg::SP => {
                if self.control.sp_sel {
                    self.psp
                } else {
                    self.msp
                }
            }
            Reg::LR => self.lr,
            Reg::PC => self.pc + 4,
        }
    }

    fn set_r(&mut self, r: Reg, value: u32) {
        match r {
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
                let reg: usize = From::from(r);
                self.r0_12[reg] = value;
            }
            Reg::SP => {
                if self.control.sp_sel {
                    self.set_psp(value)
                } else {
                    self.set_msp(value)
                }
            }
            Reg::LR => {
                self.lr = value;
            }
            Reg::PC => panic!("use branch commands instead"),
        };
    }

    fn set_msp(&mut self, value: u32) {
        self.msp = value;
    }

    fn set_psp(&mut self, value: u32) {
        self.psp = value;
    }
    fn get_msp(&self) -> u32 {
        self.msp
    }

    fn get_psp(&self) -> u32 {
        self.psp
    }

    fn add_pc(&mut self, value: u32) {
        self.pc += value;
    }

    fn get_pc(&mut self) -> u32 {
        self.pc
    }

    fn set_pc(&mut self, value: u32) {
        self.pc = value
    }

    //
    // Add value to register
    //
    fn add_r(&mut self, r: Reg, value: u32) {
        match r {
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
                let reg: usize = From::from(r);
                self.r0_12[reg] += value;
            }
            Reg::SP => {
                if self.control.sp_sel {
                    self.psp += value
                } else {
                    self.msp += value
                }
            }
            Reg::LR => self.lr += value,
            Reg::PC => self.pc += value,
        };
    }
    //
    // Substract value from register
    //
    fn sub_r(&mut self, r: Reg, value: u32) {
        match r {
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
                let reg: usize = From::from(r);
                self.r0_12[reg] -= value;
            }
            Reg::SP => {
                if self.control.sp_sel {
                    self.psp -= value
                } else {
                    self.msp -= value
                }
            }
            Reg::LR => self.lr -= value,
            Reg::PC => self.pc -= value,
        };
    }
}

#[derive(Debug)]
///
/// Processor Status Registers
/// A combination of multiple sub registers: APSR, IPSR, EPSR
pub struct PSR {
    /// raw register content
    pub value: u32,
}

/// Trait for accessing the sub parts of Application Program Status Register
pub trait Apsr {
    ///
    /// Get "N"egative flag value
    ///
    fn get_n(&self) -> bool;

    ///
    /// Set "N"egative flag value
    ///
    fn set_n(&mut self, result: u32);

    ///
    /// Get "Z"ero flag value
    ///
    fn get_z(&self) -> bool;
    ///
    /// Set "Z"ero flag value
    ///
    fn set_z(&mut self, result: u32);

    ///
    /// Get "C"arry flag value
    ///
    fn get_c(&self) -> bool;
    ///
    /// Set "C"arry flag value
    ///
    fn set_c(&mut self, c: bool);

    ///
    /// Get Overflow flag value
    ///
    fn get_v(&self) -> bool;
    ///
    /// Set Overflow flag value
    ///
    fn set_v(&mut self, v: bool);

    ///
    /// Get Saturation flag value
    ///
    fn get_q(&self) -> bool;
    ///
    /// Set Saturation flag value
    ///
    fn set_q(&mut self, q: bool);

    ///
    /// DSP extensions: set GE0 value
    ///
    fn set_ge0(&mut self, bit: bool);
    ///
    /// DSP extensions: set GE1 value
    ///
    fn set_ge1(&mut self, bit: bool);
    ///
    /// DSP extensions: set GE2 value
    ///
    fn set_ge2(&mut self, bit: bool);
    ///
    /// DSP extensions: set GE3 value
    ///
    fn set_ge3(&mut self, bit: bool);

    ///
    /// DSP extensions: get GE0 value
    ///
    fn get_ge0(&self) -> bool;
    ///
    /// DSP extensions: get GE1 value
    ///
    fn get_ge1(&self) -> bool;
    ///
    /// DSP extensions: get GE2 value
    ///
    fn get_ge2(&self) -> bool;
    ///
    /// DSP extensions: get GE3 value
    ///
    fn get_ge3(&self) -> bool;
}

/// Trait for accessing Interrupt Program Status Register subparts
pub trait Ipsr {
    /// get the exception type number of current interrupt service routine
    fn get_isr_number(&self) -> usize;
    /// set the exception type number of current interrupt service routine
    fn set_isr_number(&mut self, exception_number: usize);
}

/// Execution Program Status register
///
/// A view to PSR register containing the data.
pub trait Epsr {
    ///
    /// Set thumb state bit
    ///
    fn set_t(&mut self, t: bool);
    ///
    /// Get thumb state bit
    ///
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
    fn get_isr_number(&self) -> usize {
        //TODO: diff between cortex m0 and m3+
        (*self).value.get_bits(0..6) as usize
    }
    fn set_isr_number(&mut self, exception_number: usize) {
        self.value = (self.value & 0xffff_ffc0) | (exception_number as u32 & 0b11_1111);
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u32)]
///
/// Basic registers
///
pub enum Reg {
    /// General purpose register 0, also known as a1 (argument 1 register)
    R0,
    /// General purpose register 1, also known as a2 (argument 2 register)
    R1,
    /// General purpose register 2, also known as a3 (argument 3 register)
    R2,
    /// General purpose register 3, also known as a4 (argument 4 register)
    R3,
    /// General purpose register 4, also known as v1 (variable 1 register)
    R4,
    /// General purpose register 5, also known as v2 (variable 2 register)
    R5,
    /// General purpose register 6, also known as v3 (variable 3 register)
    R6,
    /// General purpose register 7, also known as v4 (variable 4 register)
    R7,
    /// General purpose register 8, also known as v5 (variable 5 register)
    R8,
    /// General purpose register 9, also known as v6 (variable 6 register)
    /// Another alias is "sb", static base, used for relocatable code base register.
    R9,
    /// General purpose register 10, also known as v7 (variable 7 register)
    R10,
    /// General purpose register 11, also known as v8 (variable 8 register)
    R11,
    /// General purpose register 12,
    /// also known as IP (Intra procedure call scratch register)
    R12,
    ///
    /// Stack Pointer, alias for R13
    ///
    SP,
    ///
    /// Link Register, alias for R14
    ///
    LR,
    ///
    /// Program Counter, alias for R15
    ///
    PC,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u32)]
/// Declarations of Special registers, of which some are overlays of same contents
pub enum SpecialReg {
    /// Application Program Status Register
    APSR,
    ///
    IAPSR,
    ///
    EAPSR,
    ///
    XPSR,
    /// Interrupt Program Status Register
    IPSR,
    /// Execution Program Status Register
    EPSR,
    ///
    IEPSR,
    /// Refers to Master Stack Pointer
    MSP,
    /// Refers to Process Stack Pointer
    PSP,
    /// Priority Mask Register
    PRIMASK,
    /// Fault Mask Register
    FAULTMASK,
    /// CONTROL Register
    CONTROL,
}

impl CLike for Reg {
    fn to_u32(&self) -> u32 {
        *self as u32
    }

    unsafe fn from_u32(v: u32) -> Self {
        mem::transmute(v)
    }
}

impl Reg {
    /// convert register to numeric index value
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

    /// convert numeric representation to register
    pub fn from_u16(n: u16) -> Option<Self> {
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
    /// decode 16 bit value to Special Register designator
    pub fn from_u16(n: u16) -> Option<Self> {
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
            SpecialReg::FAULTMASK => write!(f, "FAULTMASK"),
            SpecialReg::CONTROL => write!(f, "CONTROL"),
        }
    }
}

#[derive(Debug)]
/// CONTROL register parts
pub struct Control {
    /// Thread mode priviledge level
    pub n_priv: bool,
    /// selection of current active stack pointer, true = PSP, false = MSP
    pub sp_sel: bool,
}

impl From<Control> for u8 {
    fn from(control: Control) -> Self {
        control.n_priv as Self + ((control.sp_sel as Self) << 1)
    }
}
