use core::bits::*;
use core::instruction::Instruction;
use core::register::Reg;

#[allow(non_snake_case)]
pub fn decode_UDIV_t1(opcode: u32) -> Instruction {
    let reg_rm: u8 = opcode.get_bits(0, 3);
    let reg_rd: u8 = opcode.get_bits(8, 11);
    let reg_rn: u8 = opcode.get_bits(16, 19);
    Instruction::UDIV {
        rm: Reg::from(reg_rm),
        rd: Reg::from(reg_rd),
        rn: Reg::from(reg_rn),
    }
}
