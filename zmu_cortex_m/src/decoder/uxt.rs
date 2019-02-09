use crate::core::instruction::Instruction;
use crate::core::register::Reg;
use crate::core::bits::Bits;

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_UXTB_t1(opcode: u16) -> Instruction {
    Instruction::UXTB {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
        thumb32: false,
        rotation: 0,
    }
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_UXTH_t1(opcode: u16) -> Instruction {
    Instruction::UXTH {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
        rotation: 0,
        thumb32: false,
    }
}
#[allow(non_snake_case)]
pub fn decode_UXTB_t2(opcode: u32) -> Instruction {
    Instruction::UXTB {
        rm: Reg::from(opcode.get_bits(0..4) as u8),
        rd: Reg::from(opcode.get_bits(8..12) as u8),
        rotation: ((opcode.get_bits(4..6) as u8) << 3) as usize,
        thumb32: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_UXTH_t2(opcode: u32) -> Instruction {
    Instruction::UXTH {
        rm: Reg::from(opcode.get_bits(0..4) as u8),
        rd: Reg::from(opcode.get_bits(8..12) as u8),
        rotation: ((opcode.get_bits(4..6) as u8) << 3) as usize,
        thumb32: true,
    }
}
