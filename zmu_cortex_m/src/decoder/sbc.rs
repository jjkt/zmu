use core::bits::*;
use core::instruction::Instruction;

#[allow(non_snake_case)]
#[inline]
pub fn decode_SBC_reg_t1(command: u16) -> Instruction {
    Instruction::SBC_reg {
        rn: From::from(bits_0_3(command)),
        rd: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
        setflags: true,
    }
}
