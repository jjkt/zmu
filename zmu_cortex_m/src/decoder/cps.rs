use crate::core::bits::*;
use crate::core::instruction::{CpsEffect, Instruction};

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_CPS_t1(opcode: u16) -> Instruction {
    Instruction::CPS {
        im: if opcode.get_bit(4) {
            CpsEffect::ID
        } else {
            CpsEffect::IE
        },
    }
}
