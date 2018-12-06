use crate::core::instruction::Instruction;
use crate::core::ThumbCode;

#[allow(non_snake_case)]
pub fn decode_RRX_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
