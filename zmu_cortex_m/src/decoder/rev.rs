use crate::core::bits::*;
use crate::core::instruction::Instruction;

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_REVSH_t1(opcode: u16) -> Instruction {
    Instruction::REVSH {
        rd: From::from(opcode.get_bits(0..3)),
        rm: From::from(opcode.get_bits(3..6)),
        thumb32: false,
    }
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_REV_t1(opcode: u16) -> Instruction {
    Instruction::REV {
        rd: From::from(opcode.get_bits(0..3)),
        rm: From::from(opcode.get_bits(3..6)),
        thumb32: false,
    }
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_REV16_t1(opcode: u16) -> Instruction {
    Instruction::REV16 {
        rd: From::from(opcode.get_bits(0..3)),
        rm: From::from(opcode.get_bits(3..6)),
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_REV16_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: opcode.into(),
        thumb32: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_REVSH_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: opcode.into(),
        thumb32: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_REV_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: opcode.into(),
        thumb32: true,
    }
}
