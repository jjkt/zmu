use core::instruction::Instruction;
use core::bits::*;

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADR_t1(command: u16) -> Instruction {
    Instruction::ADR {
        rd: From::from(bits_8_11(command)),
        imm32: u32::from(bits_0_8(command)) << 2,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADR_t2(opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADR_t3(opcode: u32) -> Instruction {
    unimplemented!()
}
