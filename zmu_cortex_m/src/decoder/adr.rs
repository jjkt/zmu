use core::instruction::Instruction;
use core::bits::*;
use core::register::Reg;
use core::operation::zero_extend;

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
pub fn decode_ADR_t2(_opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADR_t3(opcode: u32) -> Instruction {

    let _i : u8 =  opcode.get_bits(26,26);
    let _imm3 : u8 =  opcode.get_bits(12,14);
    let rd : u8 =  opcode.get_bits(8,11);
    let _imm8 : u8 =  opcode.get_bits(0,7);

    Instruction::ADR {
        rd: Reg::from(rd),
        imm32: zero_extend(/*{i, 1}, {imm3, 3}, {imm8, 8}, 32*/),
    }
}
