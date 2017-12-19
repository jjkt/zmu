use bit_field::BitField;

use core::register::Reg;
use core::instruction::Instruction;

#[allow(non_snake_case)]
pub fn decode_MUL_t1(command: u16) -> Instruction {
    Instruction::MUL {
        rd: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rm: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        setflags: true,
    }
}
