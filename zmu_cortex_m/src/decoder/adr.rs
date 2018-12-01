use core::bits::*;
use core::instruction::Instruction;
use core::operation::zero_extend;
use core::register::Reg;

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADR_t1(command: u16) -> Instruction {
    Instruction::ADR {
        rd: From::from(bits_8_11(command)),
        imm32: u32::from(bits_0_8(command)) << 2,
        thumb32: false
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADR_t2(_opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADR_t3(opcode: u32) -> Instruction {
    let i: u8 = opcode.get_bits(26, 26);
    let imm3: u8 = opcode.get_bits(12, 14);
    let rd: u8 = opcode.get_bits(8, 11);
    let imm8: u8 = opcode.get_bits(0, 7);

    let params = [i, imm3, imm8];
    let lengths = [1, 3, 8];
    Instruction::ADR {
        rd: Reg::from(rd),
        imm32: zero_extend(&params, &lengths),
        thumb32: true
    }
}
