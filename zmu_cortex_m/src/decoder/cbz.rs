use crate::core::bits::Bits;
use crate::core::instruction::Instruction;
use crate::core::register::Reg;

#[allow(non_snake_case)]
pub fn decode_CBZ_t1(opcode: u16) -> Instruction {
    Instruction::CBZ {
        rn: Reg::from(opcode.get_bits(0..3) as u8),
        nonzero: opcode.get_bit(11),
        imm32: ((opcode.get_bit(9) as u32) << 6) + (u32::from(opcode.get_bits(3..8)) << 1),
    }
}
