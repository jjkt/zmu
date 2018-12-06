use crate::core::bits::*;
use crate::core::instruction::Instruction;

#[allow(non_snake_case)]
#[inline]
pub fn decode_BX_t1(command: u16) -> Instruction {
    Instruction::BX {
        rm: From::from(bits_3_7(command)),
    }
}
