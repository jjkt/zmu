use bit_field::BitField;
use core::instruction::Instruction;
use core::instruction::SRType;
use core::operation::decode_imm_shift;
use core::operation::thumb_expand_imm;
use core::operation::zero_extend;
use core::register::Reg;

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_reg_t1(opcode: u16) -> Instruction {
    Instruction::ADD_reg {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rn: Reg::from(opcode.get_bits(3..6) as u8),
        rm: Reg::from(opcode.get_bits(6..9) as u8),
        setflags: true,
        shift_t: SRType::LSL,
        shift_n: 0,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_reg_t2_ADD_SP_reg(opcode: u16) -> Instruction {
    let rdn = Reg::from(((opcode.get_bit(7) as u8) << 3) + opcode.get_bits(0..3) as u8);

    Instruction::ADD_reg {
        rm: Reg::from(opcode.get_bits(3..7) as u8),
        rd: rdn,
        rn: rdn,
        setflags: false,
        shift_t: SRType::LSL,
        shift_n: 0,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_ADD_reg_t3(opcode: u32) -> Instruction {
    let rn: u8 = opcode.get_bits(16..20) as u8;
    let rm: u8 = opcode.get_bits(0..4) as u8;
    let rd: u8 = opcode.get_bits(8..12) as u8;
    let s: u8 = opcode.get_bit(20) as u8;

    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm2: u8 = opcode.get_bits(6..8) as u8;
    let type_: u8 = opcode.get_bits(4..6) as u8;

    let (shift_t, shift_n) = decode_imm_shift(type_, (imm3 << 2) + imm2);

    Instruction::ADD_reg {
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
pub fn decode_ADD_imm_t1(opcode: u16) -> Instruction {
    Instruction::ADD_imm {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rn: Reg::from(opcode.get_bits(3..6) as u8),
        imm32: opcode.get_bits(6..9) as u32,
        setflags: true,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_imm_t2(opcode: u16) -> Instruction {
    Instruction::ADD_imm {
        rn: Reg::from(opcode.get_bits(8..11) as u8),
        rd: Reg::from(opcode.get_bits(8..11) as u8),
        imm32: opcode.get_bits(0..8) as u32,
        setflags: true,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_SP_imm_t1(opcode: u16) -> Instruction {
    Instruction::ADD_imm {
        rd: Reg::from(opcode.get_bits(8..11) as u8),
        rn: Reg::SP,
        imm32: (opcode.get_bits(0..8) as u32) << 2,
        setflags: false,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_SP_imm_t2(opcode: u16) -> Instruction {
    Instruction::ADD_imm {
        rd: Reg::SP,
        rn: Reg::SP,
        imm32: (opcode.get_bits(0..7) as u32) << 2,
        setflags: false,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_imm_t3(opcode: u32) -> Instruction {
    let rd: u8 = opcode.get_bits(8..12) as u8;
    let rn: u8 = opcode.get_bits(16..20) as u8;
    let s = opcode.get_bit(20);

    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm8: u8 = opcode.get_bits(0..8) as u8;
    let i: u8 = opcode.get_bit(26) as u8;

    let params = [i, imm3, imm8];
    let lengths = [1, 3, 8];

    Instruction::ADD_imm {
        rd: Reg::from(rd),
        rn: Reg::from(rn),
        imm32: thumb_expand_imm(&params, &lengths),
        setflags: s,
        thumb32: true,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_imm_t4(opcode: u32) -> Instruction {
    let rd: u8 = opcode.get_bits(8..12) as u8;
    let rn: u8 = opcode.get_bits(16..20) as u8;

    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm8: u8 = opcode.get_bits(0..8) as u8;
    let i: u8 = opcode.get_bit(26) as u8;

    let params = [i, imm3, imm8];
    let lengths = [1, 3, 8];

    Instruction::ADD_imm {
        rd: Reg::from(rd),
        rn: Reg::from(rn),
        imm32: zero_extend(&params, &lengths),
        setflags: false,
        thumb32: true,
    }
}
