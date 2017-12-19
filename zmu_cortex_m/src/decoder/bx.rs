//use bit_field::BitField;

use core::register::Reg;
use core::instruction::Instruction;

#[allow(non_snake_case)]
#[inline]
pub fn decode_BX_t1(command: u16) -> Instruction {
    Instruction::BX {
        rm: Reg::from_u16((command >> 3) & 0xf).unwrap(),
    }
}
