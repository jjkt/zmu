use crate::core::instruction::Instruction;
use crate::core::operation::decode_imm_shift;
use crate::core::register::Reg;
use bit_field::BitField;
use crate::core::instruction::{SetFlags};

#[allow(non_snake_case)]
#[inline]
pub fn decode_LSR_imm_t1(opcode: u16) -> Instruction {
    let imm5 = opcode.get_bits(6..11) as u8;

    let (_, shift_n) = decode_imm_shift(0b01, imm5);
    Instruction::LSR_imm {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
        shift_n: shift_n,
        setflags: SetFlags::NotInITBlock,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_LSR_reg_t1(opcode: u16) -> Instruction {
    Instruction::LSR_reg {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rn: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
        setflags: SetFlags::NotInITBlock,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_LSR_imm_t2(opcode: u32) -> Instruction {
    let rm: u8 = opcode.get_bits(0..4) as u8;
    let rd: u8 = opcode.get_bits(8..12) as u8;
    let s: u8 = opcode.get_bit(20) as u8;

    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm2: u8 = opcode.get_bits(6..8) as u8;

    let (_, shift_n) = decode_imm_shift(0b_01, (imm3 << 2) + imm2);

    Instruction::LSR_imm {
        rd: Reg::from(rd),
        rm: Reg::from(rm),
        setflags: if s == 1 {SetFlags::True} else {SetFlags::False},
        shift_n: shift_n,
        thumb32: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_LSR_reg_t2(opcode: u32) -> Instruction {
    let rn: u8 = opcode.get_bits(16..20) as u8;
    let rm: u8 = opcode.get_bits(0..4) as u8;
    let rd: u8 = opcode.get_bits(8..12) as u8;
    let s: u8 = opcode.get_bit(20) as u8;

    Instruction::LSR_reg {
        rd: Reg::from(rd as u8),
        rn: Reg::from(rn as u8),
        rm: Reg::from(rm as u8),
        setflags: if s == 1 {SetFlags::True} else {SetFlags::False},
        thumb32: true,
    }
}
