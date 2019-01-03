use crate::core::instruction::Instruction;
use crate::core::instruction::SRType;
use crate::core::register::Reg;
use bit_field::*;

#[allow(non_snake_case)]
#[inline]
pub fn decode_LDRSB_reg_t1(opcode: u16) -> Instruction {
    Instruction::LDRSB_reg {
        rt: Reg::from(opcode.get_bits(0..3) as u8),
        rn: Reg::from(opcode.get_bits(3..6) as u8),
        rm: Reg::from(opcode.get_bits(6..9) as u8),
        index: true,
        add: true,
        wback: false,
        shift_t: SRType::LSL,
        shift_n: 0,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRSB_reg_t2(opcode: u32) -> Instruction {
    Instruction::LDRSB_reg {
        rm: Reg::from(opcode.get_bits(0..4) as u8),
        rt: Reg::from(opcode.get_bits(12..16) as u8),
        rn: Reg::from(opcode.get_bits(16..20) as u8),
        shift_t: SRType::LSL,
        shift_n: opcode.get_bits(4..6) as u8,
        index: true,
        add: true,
        wback: false,
        thumb32: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRSB_imm_t1(opcode: u32) -> Instruction {
    Instruction::LDRSB_imm {
        rt: From::from(opcode.get_bits(12..16) as u8),
        rn: From::from(opcode.get_bits(16..20) as u8),
        imm32: opcode.get_bits(0..12),
        index: true,
        add: true,
        wback: false,
        thumb32: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRSB_imm_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: opcode.into(),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRSB_lit_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: opcode.into(),
    }
}
