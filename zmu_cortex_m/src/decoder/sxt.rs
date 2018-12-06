use bit_field::BitField;
use crate::core::instruction::Instruction;
use crate::core::register::Reg;

#[allow(non_snake_case)]
#[inline]
pub fn decode_SXTB_t1(opcode: u16) -> Instruction {
    Instruction::SXTB {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
        thumb32: false,
        rotation: 0,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SXTH_t1(opcode: u16) -> Instruction {
    Instruction::SXTH {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
        rotation: 0,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_SXTB_t2(opcode: u32) -> Instruction {
    Instruction::SXTB {
        rm: Reg::from(opcode.get_bits(0..4) as u8),
        rd: Reg::from(opcode.get_bits(8..12) as u8),
        rotation: ((opcode.get_bits(4..6) as u8) << 3) as usize,
        thumb32: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_SXTH_t2(opcode: u32) -> Instruction {
    Instruction::SXTH {
        rm: Reg::from(opcode.get_bits(0..4) as u8),
        rd: Reg::from(opcode.get_bits(8..12) as u8),
        rotation: ((opcode.get_bits(4..6) as u8) << 3) as usize,
        thumb32: true,
    }
}
