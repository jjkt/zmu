use core::bits::*;
use core::instruction::Instruction;
use core::register::Reg;
use core::ThumbCode;

#[allow(non_snake_case)]
fn decode_STC2_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_STC_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
