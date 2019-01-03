use crate::core::instruction::Instruction;

#[allow(non_snake_case)]
pub fn decode_STREXB_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: opcode.into(),
    }
}

#[allow(non_snake_case)]
pub fn decode_STREXH_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: opcode.into(),
    }
}

#[allow(non_snake_case)]
pub fn decode_STREX_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: opcode.into(),
    }
}
