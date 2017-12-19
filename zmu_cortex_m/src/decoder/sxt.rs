use bit_field::BitField;
use core::register::Reg;
use core::instruction::Instruction;
use core::bits::*;
#[allow(non_snake_case)]
pub fn decode_SXTB_t1(command: u16) -> Instruction {
    Instruction::SXTB {
        rd: From::from(bits_0_3(command)),
        rm: Reg::from_u16(command.get_bits(3..6)).unwrap(),
    }
}

#[allow(non_snake_case)]
pub fn decode_SXTH_t1(command: u16) -> Instruction {
    Instruction::SXTH {
        rd: From::from(bits_0_3(command)),
        rm: Reg::from_u16(command.get_bits(3..6)).unwrap(),
    }
}
