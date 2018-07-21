use core::bits::*;
use core::instruction::Instruction;
use core::register::Reg;

#[allow(non_snake_case)]
pub fn decode_UMLAL_t1(opcode: u32) -> Instruction {
    let reg_rm: u8 = opcode.get_bits(0, 3);
    let reg_rd_hi: u8 = opcode.get_bits(8, 11);
    let reg_rd_lo: u8 = opcode.get_bits(12, 15);
    let reg_rn: u8 = opcode.get_bits(16, 19);
    Instruction::UMLAL {
        rm: Reg::from(reg_rm),
        rdlo: Reg::from(reg_rd_hi),
        rdhi: Reg::from(reg_rd_lo),
        rn: Reg::from(reg_rn),
    }
}
