use crate::core::{
    bits::Bits,
    condition::Condition,
    instruction::{Instruction, VSelParamsf32},
    register::SingleReg,
};

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_VSEL_t1(opcode: u32) -> Instruction {
    let d = u8::from(opcode.get_bit(22));
    let vn = opcode.get_bits(16..20) as u8;
    let vd = opcode.get_bits(12..16) as u8;
    let n = u8::from(opcode.get_bit(7));
    let m = u8::from(opcode.get_bit(5));
    let vm = opcode.get_bits(0..4) as u8;

    Instruction::VSEL_f32 {
        params: VSelParamsf32 {
            sd: SingleReg::from((vd << 1) | d),
            sn: SingleReg::from((vn << 1) | n),
            sm: SingleReg::from((vm << 1) | m),
            cond: Condition::GT,
        },
    }
}
