use core::bits::*;
use core::instruction::Instruction;
use core::register::Reg;

#[allow(non_snake_case)]
pub fn decode_LDC_imm_t1(opcode: u32) -> Instruction {
    let reg_rn: u8 = opcode.get_bits(16, 19);
    Instruction::LDC_imm {
        coproc: opcode.get_bits(8, 11),
        imm32: opcode.get_bits(0, 7),
        crd: opcode.get_bits(12, 15),
        rn: Reg::from(reg_rn),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDC2_imm_t2(opcode: u32) -> Instruction {
    let reg_rn: u8 = opcode.get_bits(16, 19);
    Instruction::LDC2_imm {
        coproc: opcode.get_bits(8, 11),
        imm32: opcode.get_bits(0, 7),
        crd: opcode.get_bits(12, 15),
        rn: Reg::from(reg_rn),
    }
}
