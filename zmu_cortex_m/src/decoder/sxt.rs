use core::instruction::Instruction;
use core::bits::*;

#[allow(non_snake_case)]
pub fn decode_SXTB_t1(command: u16) -> Instruction {
    Instruction::SXTB {
        rd: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
    }
}

#[allow(non_snake_case)]
pub fn decode_SXTH_t1(command: u16) -> Instruction {
    Instruction::SXTH {
        rd: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
    }
}
