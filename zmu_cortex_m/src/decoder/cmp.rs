use core::instruction::Instruction;
use core::bits::*;

#[allow(non_snake_case)]
#[inline]
pub fn decode_CMP_imm_t1(command: u16) -> Instruction {
    Instruction::CMP_imm {
        rn: From::from(bits_8_11(command)),
        imm32: bits_0_8(command) as u32,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_CMP_reg_t1(command: u16) -> Instruction {
    Instruction::CMP_reg {
        rn: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_CMP_reg_t2(command: u16) -> Instruction {
    Instruction::CMP_reg {
        rn: From::from((bit_7(command) << 4) + bits_0_3(command)),
        rm: From::from(bits_3_7(command)),
    }
}
