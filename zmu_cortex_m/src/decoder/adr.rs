use bit_field::BitField;

use core::register::Reg;
use core::instruction::Instruction;

#[allow(non_snake_case)]
pub fn decode_ADR_t1(command: u16) -> Instruction {
    Instruction::ADR {
        rd: Reg::from_u16(command.get_bits(8..11)).unwrap(),
        imm32: u32::from(command.get_bits(0..8)) << 2,
    }
}
