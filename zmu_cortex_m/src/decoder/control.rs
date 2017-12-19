use core::instruction::Instruction;



#[allow(non_snake_case)]
#[allow(unused_variables)]
#[inline]
pub fn decode_control(op1: u16, op2: u16) -> Instruction {
    Instruction::ISB
}
