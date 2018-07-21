use core::instruction::Instruction;
use core::bits::*;

#[allow(non_snake_case)]
#[inline]
pub fn decode_UXTB_t1(command: u16) -> Instruction {
    Instruction::UXTB {
        rd: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_UXTH_t1(command: u16) -> Instruction {
    Instruction::UXTH {
        rd: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
    }
}
#[allow(non_snake_case)]
pub fn decode_UXTB_t2(_opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
pub fn decode_UXTH_t2(_opcode: u32) -> Instruction {
    unimplemented!()
}

