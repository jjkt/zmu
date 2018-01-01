use core::register::{Reg, SpecialReg};
use core::instruction::Instruction;
use core::bits::*;



#[allow(non_snake_case)]
#[allow(unused_variables)]
#[inline]
pub fn decode_msr_reg(op1: u16, op2: u16) -> Instruction {
    Instruction::MSR_reg {
        rn: Reg::from(bits_0_4(op2)),
        spec_reg: SpecialReg::from(bits_0_8(op2)),
    }
}
