use core::instruction::Instruction;

#[allow(non_snake_case)]
pub fn decode_YIELD_t1(_opcode: u16) -> Instruction {
    Instruction::YIELD {}
}

#[allow(non_snake_case)]
pub fn decode_YIELD_t2(_opcode: u32) -> Instruction {
    Instruction::YIELD {}
}
