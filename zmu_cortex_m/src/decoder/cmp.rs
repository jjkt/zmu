use crate::core::instruction::Instruction;
use crate::core::operation::thumb_expand_imm;
use crate::core::register::Reg;
use crate::core::ThumbCode;
use bit_field::BitField;

#[allow(non_snake_case)]
#[inline]
pub fn decode_CMP_imm_t1(opcode: u16) -> Instruction {
    Instruction::CMP_imm {
        rn: Reg::from(opcode.get_bits(8..11) as u8),
        imm32: opcode.get_bits(0..8) as u32,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_CMP_imm_t2(opcode: u32) -> Instruction {
    let rn: u8 = opcode.get_bits(16..20) as u8;

    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm8: u8 = opcode.get_bits(0..8) as u8;
    let i: u8 = opcode.get_bit(26) as u8;

    let params = [i, imm3, imm8];
    let lengths = [1, 3, 8];

    Instruction::CMP_imm {
        rn: Reg::from(rn),
        imm32: thumb_expand_imm(&params, &lengths),
        thumb32: true,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_CMP_reg_t1(opcode: u16) -> Instruction {
    Instruction::CMP_reg {
        rn: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_CMP_reg_t2(opcode: u16) -> Instruction {
    Instruction::CMP_reg {
        rn: Reg::from(((opcode.get_bit(7) as u8) << 3) + opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..7) as u8),
    }
}

#[allow(non_snake_case)]
pub fn decode_CMP_reg_t3(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
