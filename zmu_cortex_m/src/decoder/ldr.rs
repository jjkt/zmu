use bit_field::BitField;
use core::bits::*;
use core::register::Reg;
use core::instruction::Instruction;

#[allow(non_snake_case)]
pub fn decode_LDR_imm_t2(command: u16) -> Instruction {
    Instruction::LDR_imm {
        rt: From::from(bits_8_11(command)),
        rn: Reg::SP,
        imm32: (bits_0_8(command) as u32) << 2,
    }
}
#[allow(non_snake_case)]
pub fn decode_LDR_imm_t1(command: u16) -> Instruction {
    Instruction::LDR_imm {
        rt: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        imm32: (command.get_bits(6..11) as u32) << 2,
    }
}

#[allow(non_snake_case)]
pub fn decode_LDR_lit_t1(command: u16) -> Instruction {
    Instruction::LDR_lit {
        rt: From::from(bits_8_11(command)),
        imm32: (bits_0_8(command) as u32) << 2,
    }
}

#[allow(non_snake_case)]
pub fn decode_LDR_reg_t1(command: u16) -> Instruction {
    Instruction::LDR_reg {
        rt: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        rm: From::from(bits_6_9(command)),
    }
}
