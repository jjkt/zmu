use crate::core::instruction::Instruction;
use crate::core::instruction::SRType;
use crate::core::register::Reg;
use crate::core::ThumbCode;
use bit_field::BitField;

#[allow(non_snake_case)]
#[inline]
pub fn decode_LDRSH_reg_t1(opcode: u16) -> Instruction {
    Instruction::LDRSH_reg {
        rt: Reg::from(opcode.get_bits(0..3) as u8),
        rn: Reg::from(opcode.get_bits(3..6) as u8),
        rm: Reg::from(opcode.get_bits(6..9) as u8),
        shift_t: SRType::LSL,
        shift_n: 0,
        index: true,
        add: true,
        wback: false,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRSH_reg_t2(opcode: u32) -> Instruction {
    Instruction::LDRSH_reg {
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
pub fn decode_LDRSH_imm_t1(opcode: u32) -> Instruction {
    Instruction::LDRSH_imm {
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
pub fn decode_LDRSH_imm_t2(opcode: u32) -> Instruction {
    Instruction::LDRSH_imm {
        rt: From::from(opcode.get_bits(12..16) as u8),
        rn: From::from(opcode.get_bits(16..20) as u8),
        imm32: opcode.get_bits(0..8),
        index: opcode.get_bit(10),
        add: opcode.get_bit(9),
        wback: opcode.get_bit(8),
        thumb32: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRSH_lit_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
