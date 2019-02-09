use crate::core::bits::*;
use crate::core::instruction::{CpsEffect, Instruction};

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_CPS_t1(opcode: u16) -> Instruction {
    Instruction::CPS {
        im: if bit_4(opcode) == 1 {
            CpsEffect::ID
        } else {
            CpsEffect::IE
        },
    }
}
