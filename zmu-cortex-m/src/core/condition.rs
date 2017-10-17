use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Condition {
    EQ, // Equal
    NE, // Not Equal
    CS, // Carry Set
    CC, // Carry clear
    MI, // Minus, negative
    PL, // Plus, positive or zero
    VS, // Overflow
    VC, // No overflow
    HI, // Unsigned higher
    LS, // Unsigned lower or same
    GE, // Signer greater than or equal
    LT, // Signed less than
    GT, // Signed greater than
    LE, // Signed less than or equal
    AL, // None or (AL = optional mnemonic extension for always)
}

impl Condition {
    pub fn value(&self) -> usize {
        match *self {
            Condition::EQ => 0b0000, 
            Condition::NE => 0b0001, 
            Condition::CS => 0b0010, 
            Condition::CC => 0b0011, 
            Condition::MI => 0b0100, 
            Condition::PL => 0b0101, 
            Condition::VS => 0b0110, 
            Condition::VC => 0b0111, 
            Condition::HI => 0b1000, 
            Condition::LS => 0b1001, 
            Condition::GE => 0b1010, 
            Condition::LT => 0b1011, 
            Condition::GT => 0b1100, 
            Condition::LE => 0b1101, 
            Condition::AL => 0b1110, 
        }
    }

    pub fn from_u16(n: u16) -> Option<Condition> {
        match n {
            0 => Some(Condition::EQ),
            1 => Some(Condition::NE),
            2 => Some(Condition::CS),
            3 => Some(Condition::CC),
            4 => Some(Condition::MI),
            5 => Some(Condition::PL),
            6 => Some(Condition::VS),
            7 => Some(Condition::VC),
            8 => Some(Condition::HI),
            9 => Some(Condition::LS),
            10 => Some(Condition::GE),
            11 => Some(Condition::LT),
            12 => Some(Condition::GT),
            13 => Some(Condition::LE),
            14 => Some(Condition::AL),
            _ => None,
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Condition::EQ => write!(f, "EQ"), 
            Condition::NE => write!(f, "NE"), 
            Condition::CS => write!(f, "CS"), 
            Condition::CC => write!(f, "CC"), 
            Condition::MI => write!(f, "MI"), 
            Condition::PL => write!(f, "PL"), 
            Condition::VS => write!(f, "VS"), 
            Condition::VC => write!(f, "VC"), 
            Condition::HI => write!(f, "HI"), 
            Condition::LS => write!(f, "LS"), 
            Condition::GE => write!(f, "GE"), 
            Condition::LT => write!(f, "LT"), 
            Condition::GT => write!(f, "GT"), 
            Condition::LE => write!(f, "LE"), 
            Condition::AL => write!(f, ""), 
        }
    }
}