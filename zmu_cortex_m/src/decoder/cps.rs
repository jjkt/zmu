use bit_field::BitField;
use core::register::Reg;
use core::instruction::{CpsEffect, Instruction};

#[allow(non_snake_case)]
pub fn decode_CPS_t1(command: u16) -> Instruction {
    Instruction::CPS {
        im: if command.get_bit(4) {
            CpsEffect::ID
        } else {
            CpsEffect::IE
        },
    }
}
