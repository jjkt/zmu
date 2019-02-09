use crate::core::bits::*;
use crate::core::instruction::Instruction;

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_BLX_t1(command: u16) -> Instruction {
    Instruction::BLX {
        rm: From::from(bits_3_7(command)),
    }
}
