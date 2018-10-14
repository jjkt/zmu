use bit_field::*;
use core::instruction::Instruction;
use core::register::Reg;
use core::ThumbCode;

#[allow(non_snake_case)]
pub fn decode_LDRB_reg_t1(opcode: u16) -> Instruction {
    Instruction::LDRB_reg {
        rt: Reg::from(opcode.get_bits(0..3) as u8),
        rn: Reg::from(opcode.get_bits(3..6) as u8),
        rm: Reg::from(opcode.get_bits(6..9) as u8),
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_LDRB_imm_t1(opcode: u16) -> Instruction {
    Instruction::LDRB_imm {
        rt: Reg::from(opcode.get_bits(0..3) as u8),
        rn: Reg::from(opcode.get_bits(3..6) as u8),
        imm32: opcode.get_bits(6..11) as u32,
        index: true,
        add: true,
        wback: false,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRB_imm_t2(opcode: u32) -> Instruction {
    Instruction::LDRB_imm {
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
pub fn decode_LDRB_imm_t3(opcode: u32) -> Instruction {
    // ARMv7-M
    Instruction::LDRB_imm {
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
pub fn decode_LDRB_lit_t1(opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
pub fn decode_LDRB_reg_t2(opcode: u32) -> Instruction {
    unimplemented!()
}
