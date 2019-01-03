use crate::core::instruction::Instruction;

#[allow(non_snake_case)]
pub fn decode_SMULL_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: opcode.into(),
    }
}
