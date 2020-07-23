use crate::core::{bits::Bits, instruction::{Reg643232Params, Instruction}, register::Reg};

#[allow(non_snake_case)]
pub fn decode_SMLAL_t1(opcode: u32) -> Instruction {
    let reg_rm: u8 = opcode.get_bits(0..4) as u8;
    let reg_rd_hi: u8 = opcode.get_bits(8..12) as u8;
    let reg_rd_lo: u8 = opcode.get_bits(12..16) as u8;
    let reg_rn: u8 = opcode.get_bits(16..20) as u8;
    Instruction::SMLAL {
        params: Reg643232Params {
            rm: Reg::from(reg_rm),
            rdlo: Reg::from(reg_rd_hi),
            rdhi: Reg::from(reg_rd_lo),
            rn: Reg::from(reg_rn),
        },
    }
}
