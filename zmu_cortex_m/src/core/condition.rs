//!
//! Instruction conditionals
//!

use std::fmt;

///
/// Condition variants used for conditional execution
///
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Condition {
    /// Equal
    EQ,
    /// Not Equal
    NE,
    /// Carry Set
    CS,
    /// Carry clear
    CC,
    /// Minus, negative
    MI,
    /// Plus, positive or zero
    PL,
    /// Overflow
    VS,
    /// No overflow
    VC,
    /// Unsigned higher
    HI,
    /// Unsigned lower or same
    LS,
    /// Signed greater than or equal
    GE,
    /// Signed less than
    LT,
    /// Signed greater than
    GT,
    /// Signed less than or equal
    LE,
    /// None or (AL = optional mnemonic extension for always)
    AL,
}

impl Condition {
    ///
    /// Condition encoding as bitvalue
    ///
    pub fn value(self) -> usize {
        match self {
            Self::EQ => 0b0000,
            Self::NE => 0b0001,
            Self::CS => 0b0010,
            Self::CC => 0b0011,
            Self::MI => 0b0100,
            Self::PL => 0b0101,
            Self::VS => 0b0110,
            Self::VC => 0b0111,
            Self::HI => 0b1000,
            Self::LS => 0b1001,
            Self::GE => 0b1010,
            Self::LT => 0b1011,
            Self::GT => 0b1100,
            Self::LE => 0b1101,
            Self::AL => 0b1110,
        }
    }

    ///
    /// bitvalue conversion to Condition
    ///
    pub fn from_u16(n: u16) -> Option<Self> {
        match n {
            0 => Some(Self::EQ),
            1 => Some(Self::NE),
            2 => Some(Self::CS),
            3 => Some(Self::CC),
            4 => Some(Self::MI),
            5 => Some(Self::PL),
            6 => Some(Self::VS),
            7 => Some(Self::VC),
            8 => Some(Self::HI),
            9 => Some(Self::LS),
            10 => Some(Self::GE),
            11 => Some(Self::LT),
            12 => Some(Self::GT),
            13 => Some(Self::LE),
            14 => Some(Self::AL),
            _ => None,
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::EQ => write!(f, "eq"),
            Self::NE => write!(f, "ne"),
            Self::CS => write!(f, "cs"),
            Self::CC => write!(f, "cc"),
            Self::MI => write!(f, "mi"),
            Self::PL => write!(f, "pl"),
            Self::VS => write!(f, "vs"),
            Self::VC => write!(f, "vc"),
            Self::HI => write!(f, "hi"),
            Self::LS => write!(f, "ls"),
            Self::GE => write!(f, "ge"),
            Self::LT => write!(f, "lt"),
            Self::GT => write!(f, "gt"),
            Self::LE => write!(f, "le"),
            Self::AL => write!(f, ""),
        }
    }
}
