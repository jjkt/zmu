use bit_field::BitField;

use core::register::{Reg, SpecialReg};
use core::instruction::Instruction;



#[allow(non_snake_case)]
#[allow(unused_variables)]
pub fn decode_mrs(op1: u16, op2: u16) -> Instruction {
    Instruction::MRS {
        rd: Reg::from_u16(op2.get_bits(8..12) as u16).unwrap(),
        spec_reg: SpecialReg::from_u16(op2.get_bits(0..8) as u16).unwrap(),
    }
}
