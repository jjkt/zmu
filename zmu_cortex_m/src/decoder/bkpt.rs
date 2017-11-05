use bit_field::BitField;
use core::instruction::Instruction;

#[allow(non_snake_case)]
pub fn decode_BKPT_t1(command: u16) -> Instruction {
    Instruction::BKPT {
        imm32: u32::from(command.get_bits(0..8)),
    }
}
