use core::instruction::Instruction;
use core::bits::*;

#[allow(non_snake_case)]
fn decode_TEQ_reg_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
