use crate::core::instruction::Instruction;
use crate::core::instruction::SetFlags;
use crate::core::operation::decode_imm_shift;
use crate::core::operation::thumb_expand_imm;
use crate::core::register::Reg;
use bit_field::BitField;

#[allow(non_snake_case)]
#[inline]
pub fn decode_RSB_imm_t1(opcode: u16) -> Instruction {
    Instruction::RSB_imm {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rn: Reg::from(opcode.get_bits(3..6) as u8),
        imm32: 0,
        setflags: SetFlags::NotInITBlock,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_RSB_reg_t1(opcode: u32) -> Instruction {
    let rn: u8 = opcode.get_bits(16..20) as u8;
    let rm: u8 = opcode.get_bits(0..4) as u8;
    let rd: u8 = opcode.get_bits(8..12) as u8;
    let s: u8 = opcode.get_bit(20) as u8;

    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm2: u8 = opcode.get_bits(6..8) as u8;
    let type_: u8 = opcode.get_bits(4..6) as u8;

    let (shift_t, shift_n) = decode_imm_shift(type_, (imm3 << 2) + imm2);

    Instruction::RSB_reg {
        rd: Reg::from(rd),
        rn: Reg::from(rn),
        rm: Reg::from(rm),
        setflags: s == 1,
        shift_t: shift_t,
        shift_n: shift_n,
        thumb32: true,
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
        setflags: if s == 1 {
            SetFlags::True
        } else {
            SetFlags::False
        },
        thumb32: true,
    }
}
