use crate::core::instruction::Instruction;
use crate::core::operation::decode_imm_shift;
use crate::core::register::Reg;
use crate::core::bits::Bits;

#[allow(non_snake_case)]
pub fn decode_TEQ_reg_t1(opcode: u32) -> Instruction {
    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm2: u8 = opcode.get_bits(6..8) as u8;
    let type_: u8 = opcode.get_bits(4..6) as u8;

    let (shift_t, shift_n) = decode_imm_shift(type_, (imm3 << 2) + imm2);
    Instruction::TEQ_reg {
        rm: Reg::from(opcode.get_bits(0..4)),
        rn: Reg::from(opcode.get_bits(16..20)),
        shift_t: shift_t,
        shift_n: shift_n,
    }
}

#[allow(non_snake_case)]
pub fn decode_TEQ_imm_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: opcode.into(),
    }
}
