use core::instruction::Instruction;

#[allow(non_snake_case)]
pub fn decode_WFE_t1(_opcode: u16) -> Instruction {
    Instruction::WFE {}
}

#[allow(non_snake_case)]
pub fn decode_WFE_t2(_opcode: u32) -> Instruction {
    Instruction::WFE {}
}
