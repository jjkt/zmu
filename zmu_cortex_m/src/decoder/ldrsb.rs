use bit_field::BitField;

use core::register::Reg;
use core::instruction::Instruction;


#[allow(non_snake_case)]
pub fn decode_LDRSB_reg_t1(command: u16) -> Instruction {
    Instruction::LDRSB_reg {
        rt: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        rm: Reg::from_u16(command.get_bits(6..9)).unwrap(),
    }
}
