use std::fmt;

#[derive(PartialEq, Debug, Copy, Clone)]
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
