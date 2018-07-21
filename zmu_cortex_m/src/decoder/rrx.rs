use core::bits::*;
use core::instruction::Instruction;
use core::ThumbCode;

#[allow(non_snake_case)]
fn decode_RRX_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
