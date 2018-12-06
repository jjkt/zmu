use crate::core::bits::*;
use crate::core::instruction::Instruction;
use crate::core::register::{Reg, SpecialReg};


#[allow(non_snake_case)]
pub fn decode_MRS_t1(opcode: u32) -> Instruction {
    let reg_rd: u8 = opcode.get_bits(8, 12);
    let spec_reg: u8 = opcode.get_bits(0, 8);
    Instruction::MRS {
        rd: Reg::from(reg_rd),
        spec_reg: SpecialReg::from(spec_reg),
    }
}
