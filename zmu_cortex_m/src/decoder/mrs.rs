use core::register::{Reg, SpecialReg};
use core::instruction::Instruction;
use core::bits::*;


#[allow(non_snake_case)]
#[allow(unused_variables)]
#[inline]
pub fn decode_mrs(op1: u16, op2: u16) -> Instruction {
    Instruction::MRS {
        rd: Reg::from(bits_8_12(op2)),
        spec_reg: SpecialReg::from(bits_0_8(op2)),
    }
}
