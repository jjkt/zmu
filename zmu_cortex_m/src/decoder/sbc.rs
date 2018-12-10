use crate::core::instruction::Instruction;
use crate::core::instruction::SRType;
use crate::core::operation::decode_imm_shift;
use crate::core::register::Reg;
use crate::core::ThumbCode;
use bit_field::BitField;

#[allow(non_snake_case)]
#[inline]
pub fn decode_SBC_reg_t1(opcode: u16) -> Instruction {
    Instruction::SBC_reg {
        rn: Reg::from(opcode.get_bits(0..3) as u8),
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
        setflags: true,
        thumb32: false,
        shift_t: SRType::LSL,
        shift_n: 0,
    }
}

#[allow(non_snake_case)]
pub fn decode_SBC_reg_t2(opcode: u32) -> Instruction {
    let rn: u8 = opcode.get_bits(16..20) as u8;
    let rm: u8 = opcode.get_bits(0..4) as u8;
    let rd: u8 = opcode.get_bits(8..12) as u8;
    let s: u8 = opcode.get_bit(20) as u8;

    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm2: u8 = opcode.get_bits(6..8) as u8;
    let type_: u8 = opcode.get_bits(4..6) as u8;

    let (shift_t, shift_n) = decode_imm_shift(type_, (imm3 << 2) + imm2);

    Instruction::SBC_reg {
        rn: Reg::from(rn),
        rd: Reg::from(rd),
        rm: Reg::from(rm),
        setflags: s == 1,
        thumb32: true,
        shift_t: shift_t,
        shift_n: shift_n,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SBC_imm_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
