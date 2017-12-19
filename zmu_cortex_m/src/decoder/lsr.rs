use core::bits::*;
use core::instruction::Instruction;


#[allow(non_snake_case)]
#[inline]
pub fn decode_LSR_imm_t1(command: u16) -> Instruction {
    Instruction::LSR_imm {
        rd: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
        imm5: bits_6_11(command) as u8,
        setflags: true,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_LSR_reg_t1(command: u16) -> Instruction {
    Instruction::LSR_reg {
        rd: From::from(bits_0_3(command)),
        rn: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
        setflags: true,
    }
}
