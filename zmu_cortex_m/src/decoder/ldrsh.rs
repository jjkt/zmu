use core::bits::*;
use core::instruction::Instruction;


#[allow(non_snake_case)]
#[inline]
pub fn decode_LDRSH_reg_t1(command: u16) -> Instruction {
    Instruction::LDRSH_reg {
        rt: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        rm: From::from(bits_6_9(command)),
    }
}
