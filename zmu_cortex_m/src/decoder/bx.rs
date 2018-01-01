use core::instruction::Instruction;
use core::bits::*;

#[allow(non_snake_case)]
#[inline]
pub fn decode_BX_t1(command: u16) -> Instruction {
    Instruction::BX {
        rm: From::from(bits_3_7(command)),
    }
}
