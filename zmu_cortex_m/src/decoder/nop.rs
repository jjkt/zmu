use core::instruction::Instruction;

#[allow(non_snake_case)]
pub fn decode_NOP_t1(_: u16) -> Instruction {
    Instruction::NOP
}