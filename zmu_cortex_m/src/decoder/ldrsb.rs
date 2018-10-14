use core::bits::*;
use core::instruction::Instruction;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_LDRSB_reg_t1(command: u16) -> Instruction {
    Instruction::LDRSB_reg {
        rt: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        rm: From::from(bits_6_9(command)),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRSB_imm_t1(opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
pub fn decode_LDRSB_imm_t2(opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
pub fn decode_LDRSB_lit_t1(opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
pub fn decode_LDRSB_reg_t2(opcode: u32) -> Instruction {
    unimplemented!()
}
