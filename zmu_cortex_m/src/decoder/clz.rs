use crate::core::instruction::Instruction;
use crate::core::register::Reg;
use bit_field::*;

#[allow(non_snake_case)]
pub fn decode_CLZ_t1(opcode: u32) -> Instruction {
    Instruction::CLZ {
        rd: Reg::from(opcode.get_bits(8..12) as u8),
        rm: Reg::from(opcode.get_bits(0..4) as u8),
    }
}
