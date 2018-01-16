use core::instruction::Instruction;
use core::bits::*;

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADC_reg_t1(opcode: u16) -> Instruction {
    Instruction::ADC_reg {
        rn: From::from(bits_0_3(opcode)),
        rd: From::from(bits_0_3(opcode)),
        rm: From::from(bits_3_6(opcode)),
        setflags: true,
    }
}
