use core::instruction::Instruction;
use core::bits::*;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_CMN_reg_t1(command: u16) -> Instruction {
    Instruction::CMN_reg {
        rn: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
    }
}

#[allow(non_snake_case)]
pub fn decode_CMN_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
