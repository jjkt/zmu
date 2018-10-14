use core::bits::*;
use core::instruction::Instruction;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_ROR_reg_t1(command: u16) -> Instruction {
    Instruction::ROR_reg {
        rd: From::from(bits_0_3(command)),
        rn: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
        setflags: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_ROR_imm_t1(opcode: u32) -> Instruction {
       unimplemented!()

}

#[allow(non_snake_case)]
pub fn decode_ROR_reg_t2(opcode: u32) -> Instruction {
        unimplemented!()

}
