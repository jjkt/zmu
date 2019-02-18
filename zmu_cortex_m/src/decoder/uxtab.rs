use crate::core::bits::Bits;
use crate::core::instruction::Instruction;

#[allow(non_snake_case)]
pub fn decode_UXTAB_t1(opcode: u32) -> Instruction {
    Instruction::UXTAB {
        rd: opcode.get_bits(8..12).into(),
        rn: opcode.get_bits(16..20).into(),
        rm: opcode.get_bits(0..4).into(),
        rotation: (opcode.get_bits(4..6) << 3) as usize,
    }
}
