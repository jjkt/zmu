use bit_field::BitField;
use crate::core::instruction::Instruction;
use crate::core::operation::thumb_expand_imm;
use crate::core::register::Reg;
use crate::core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_RSB_imm_t1(opcode: u16) -> Instruction {
    Instruction::RSB_imm {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rn: Reg::from(opcode.get_bits(3..6) as u8),
        imm32: 0,
        setflags: true,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_RSB_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_RSB_imm_t2(opcode: u32) -> Instruction {
    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm8: u8 = opcode.get_bits(0..8) as u8;
    let i: u8 = opcode.get_bit(26) as u8;
    let s: u8 = opcode.get_bit(20) as u8;

    let params = [i, imm3, imm8];
    let lengths = [1, 3, 8];

    Instruction::RSB_imm {
        rd: Reg::from(opcode.get_bits(8..12) as u8),
        rn: Reg::from(opcode.get_bits(16..20) as u8),
        imm32: thumb_expand_imm(&params, &lengths),
        setflags: s == 1,
        thumb32: true,
    }
}
