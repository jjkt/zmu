use bit_field::BitField;
use crate::core::instruction::Instruction;
use crate::core::instruction::Imm32Carry;
use crate::core::operation::thumb_expand_imm_c;
use crate::core::register::Reg;
use crate::core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_MVN_reg_t1(opcode: u16) -> Instruction {
    Instruction::MVN_reg {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
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
    let rd: u8 = opcode.get_bits(8..12) as u8;
    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm8: u8 = opcode.get_bits(0..8) as u8;
    let i: u8 = opcode.get_bit(26) as u8;

    let params = [i, imm3, imm8];
    let lengths = [1, 3, 8];

    Instruction::MVN_imm {
        rd: Reg::from(rd),
        imm32: Imm32Carry::Carry {
            imm32_c0: thumb_expand_imm_c(&params, &lengths, false),
            imm32_c1: thumb_expand_imm_c(&params, &lengths, true),
        },
        setflags: true,
    }
}
