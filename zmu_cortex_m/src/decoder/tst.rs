use bit_field::BitField;
use core::register::Reg;
use core::instruction::Instruction;
use core::bits::*;
#[allow(non_snake_case)]
pub fn decode_TST_reg_t1(command: u16) -> Instruction {
    Instruction::TST_reg {
        rn: From::from(bits_0_3(command)),
        rm: Reg::from_u16(command.get_bits(3..6)).unwrap(),
    }
}
