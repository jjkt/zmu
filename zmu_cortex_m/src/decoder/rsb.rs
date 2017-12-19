use core::bits::*;
use core::instruction::Instruction;



#[allow(non_snake_case)]
pub fn decode_RSB_imm_t1(command: u16) -> Instruction {
    Instruction::RSB_imm {
        rd: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        imm32: 0,
        setflags: true,
    }
}
