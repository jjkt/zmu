use core::instruction::Instruction;
use core::bits::*;

#[allow(non_snake_case)]
#[inline]
pub fn decode_BLX_t1(command: u16) -> Instruction {
    Instruction::BLX {
        rm: From::from(bits_3_7(command)),
    }
}
