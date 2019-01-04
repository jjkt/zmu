use crate::core::instruction::Instruction;
use bit_field::*;

#[allow(non_snake_case)]
pub fn decode_TBH_t1(opcode: u32) -> Instruction {
    Instruction::TBH {
        rn: opcode.get_bits(16..20).into(),
        rm: opcode.get_bits(0..4).into(),
    }
}
