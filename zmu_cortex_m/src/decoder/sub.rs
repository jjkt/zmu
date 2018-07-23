use core::bits::*;
use core::instruction::Instruction;
use core::instruction::SRType;
use core::operation::decode_imm_shift;
use core::operation::thumb_expand_imm;
use core::register::Reg;

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_imm_t1(command: u16) -> Instruction {
    Instruction::SUB_imm {
        rd: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        setflags: true,
        imm32: bits_6_9(command) as u32,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_imm_t2(command: u16) -> Instruction {
    Instruction::SUB_imm {
        rd: From::from(bits_8_11(command)),
        rn: From::from(bits_8_11(command)),
        setflags: true,
        imm32: bits_0_8(command) as u32,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_SP_imm_t1(command: u16) -> Instruction {
    Instruction::SUB_imm {
        rn: Reg::SP,
        rd: Reg::SP,
        imm32: (bits_0_7(command) as u32) << 2,
        setflags: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_reg_t1(command: u16) -> Instruction {
    Instruction::SUB_reg {
        rd: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        rm: From::from(bits_6_9(command)),
        setflags: true,
        shift_t: SRType::LSL,
        shift_n: 0,
    }
}

#[allow(non_snake_case)]
pub fn decode_SUB_reg_t2(opcode: u32) -> Instruction {
    let rn: u8 = opcode.get_bits(16, 19);
    let rm: u8 = opcode.get_bits(0, 3);
    let rd: u8 = opcode.get_bits(8, 11);
    let s: u8 = opcode.get_bits(20, 20);

    let imm3: u8 = opcode.get_bits(12, 14);
    let imm2: u8 = opcode.get_bits(6, 7);
    let type_: u8 = opcode.get_bits(4, 5);

    let (shift_t, shift_n) = decode_imm_shift(type_, (imm3 << 2) + imm2);

    Instruction::SUB_reg {
        rd: Reg::from(rd),
        rn: Reg::from(rn),
        rm: Reg::from(rm),
        setflags: s == 1,
        shift_t: shift_t,
        shift_n: shift_n,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_imm_t3(opcode: u32) -> Instruction {
    let _i: u8 = opcode.get_bits(26, 26);
    let s: u8 = opcode.get_bits(20, 20);

    let rn: u8 = opcode.get_bits(16, 19);
    let rd: u8 = opcode.get_bits(8, 11);
    let _imm3: u8 = opcode.get_bits(12, 14);
    let _imm8: u8 = opcode.get_bits(0, 7);

    Instruction::SUB_imm {
        rd: Reg::from(rd),
        rn: Reg::from(rn),
        imm32: thumb_expand_imm(/*i, 1, imm3, 3, imm8, 8*/),
        setflags: s == 1,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_imm_t4(_opcode: u32) -> Instruction {
    unimplemented!()
}
