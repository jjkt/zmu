use crate::core::bits::*;
use crate::core::instruction::Instruction;

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_REVSH_t1(opcode: u16) -> Instruction {
    Instruction::REVSH {
        rd: From::from(bits_0_3(opcode)),
        rm: From::from(bits_3_6(opcode)),
    }
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_REV_t1(opcode: u16) -> Instruction {
    Instruction::REV {
        rd: From::from(bits_0_3(opcode)),
        rm: From::from(bits_3_6(opcode)),
    }
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_REV16_t1(opcode: u16) -> Instruction {
    Instruction::REV16 {
        rd: From::from(bits_0_3(opcode)),
        rm: From::from(bits_3_6(opcode)),
    }
}

#[allow(non_snake_case)]
pub fn decode_REV16_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: opcode.into(),
    }
}

#[allow(non_snake_case)]
pub fn decode_REVSH_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: opcode.into(),
    }
}

#[allow(non_snake_case)]
pub fn decode_REV_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: opcode.into(),
    }
}
