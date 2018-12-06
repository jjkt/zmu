use crate::core::bits::*;
use crate::core::instruction::Instruction;
use crate::core::register::{Reg, SpecialReg};

#[allow(non_snake_case)]
#[inline]
pub fn decode_MSR_reg_t1(opcode: u32) -> Instruction {
    let reg_rn: u8 = opcode.get_bits(16, 19);
    let sysm: u8 = opcode.get_bits(0, 7);
    Instruction::MSR_reg {
        rn: Reg::from(reg_rn),
        spec_reg: SpecialReg::from(sysm),
    }
}
