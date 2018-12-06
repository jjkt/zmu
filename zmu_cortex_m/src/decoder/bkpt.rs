use crate::core::instruction::Instruction;
use crate::core::bits::*;

#[allow(non_snake_case)]
#[inline]
pub fn decode_BKPT_t1(command: u16) -> Instruction {
    Instruction::BKPT {
        imm32: u32::from(bits_0_8(command)),
    }
}
