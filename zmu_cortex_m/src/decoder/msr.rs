use bit_field::BitField;

use core::register::{Reg, SpecialReg};
use core::instruction::Instruction;



#[allow(non_snake_case)]
pub fn decode_msr_reg(op1: u16, op2: u16) -> Instruction {
    Instruction::MSR_reg {
        rn: Reg::from_u16(op1.get_bits(0..4) as u16).unwrap(),
        spec_reg: SpecialReg::from_u16(op2.get_bits(0..8) as u16).unwrap(),
    }
}
