use core::bits::*;
use core::instruction::Instruction;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_REVSH_t1(opcode: u16) -> Instruction {
    Instruction::REVSH {
        rd: From::from(bits_0_3(opcode)),
        rm: From::from(bits_3_6(opcode)),
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_REV_t1(opcode: u16) -> Instruction {
    Instruction::REV {
        rd: From::from(bits_0_3(opcode)),
        rm: From::from(bits_3_6(opcode)),
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_REV16_t1(opcode: u16) -> Instruction {
    Instruction::REV16 {
        rd: From::from(bits_0_3(opcode)),
        rm: From::from(bits_3_6(opcode)),
    }
}

#[allow(non_snake_case)]
pub fn decode_REV16_t2(opcode: u32) -> Instruction {
        unimplemented!()

}

#[allow(non_snake_case)]
pub fn decode_REVSH_t2(opcode: u32) -> Instruction {
        unimplemented!()

}

#[allow(non_snake_case)]
pub fn decode_REV_t2(opcode: u32) -> Instruction {
       unimplemented!()

}
