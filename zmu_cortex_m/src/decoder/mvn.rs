use core::bits::*;
use core::instruction::Instruction;
use core::operation::thumb_expand_imm_c;
use core::register::Reg;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_MVN_reg_t1(command: u16) -> Instruction {
    Instruction::MVN_reg {
        rd: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
        setflags: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_MVN_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_MVN_imm_t1(opcode: u32) -> Instruction {
    let rd: u8 = opcode.get_bits(8, 11);
    Instruction::MVN_imm {
        rd: Reg::from(rd),
        imm32: thumb_expand_imm_c(),
        setflags: true,
    }
}
