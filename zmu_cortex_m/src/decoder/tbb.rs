use crate::core::instruction::Instruction;
use crate::core::register::Reg;
use bit_field::BitField;

#[allow(non_snake_case)]
pub fn decode_TBB_t1(opcode: u32) -> Instruction {
    let rn = opcode.get_bits(16..20);
    let rm = opcode.get_bits(0..4);

    Instruction::TBB {
        rn: Reg::from(rn),
        rm: Reg::from(rm),
    }
}
