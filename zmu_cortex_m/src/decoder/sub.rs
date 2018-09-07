use core::bits::{bits_0_3, bits_0_7, bits_0_8, bits_3_6, bits_6_9, bits_8_11, Bits};
use core::instruction::Instruction;
use core::instruction::SRType;
use core::operation::decode_imm_shift;
use core::operation::thumb_expand_imm;
use core::operation::zero_extend;
use core::register::Reg;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_imm_t1(command: u16) -> Instruction {
    Instruction::SUB_imm {
        rd: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        setflags: true,
        imm32: bits_6_9(command) as u32,
        thumb32: false,
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
        thumb32: false,
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
        thumb32: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_SP_imm_t2(opcode: u32) -> Instruction {
    let i: u8 = opcode.get_bits(26, 26);
    let s: u8 = opcode.get_bits(20, 20);

    let rd: u8 = opcode.get_bits(8, 11);
    let imm3: u8 = opcode.get_bits(12, 14);
    let imm8: u8 = opcode.get_bits(0, 7);

    let params = [i, imm3, imm8];
    let lengths = [1, 3, 8];

    Instruction::SUB_imm {
        rd: Reg::from(rd),
        rn: Reg::SP,
        imm32: thumb_expand_imm(&params, &lengths),
        setflags: s == 1,
        thumb32: true,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_SP_imm_t3(opcode: u32) -> Instruction {
    let i: u8 = opcode.get_bits(26, 26);

    let rd: u8 = opcode.get_bits(8, 11);
    let imm3: u8 = opcode.get_bits(12, 14);
    let imm8: u8 = opcode.get_bits(0, 7);

    let params = [i, imm3, imm8];
    let lengths = [1, 3, 8];
    Instruction::SUB_imm {
        rd: Reg::from(rd),
        rn: Reg::SP,
        imm32: zero_extend(&params, &lengths),
        setflags: false,
        thumb32: true,
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
        thumb32: false,
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
        thumb32: true,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_imm_t3(opcode: u32) -> Instruction {
    let i: u8 = opcode.get_bits(26, 26);
    let s: u8 = opcode.get_bits(20, 20);

    let rn: u8 = opcode.get_bits(16, 19);
    let rd: u8 = opcode.get_bits(8, 11);
    let imm3: u8 = opcode.get_bits(12, 14);
    let imm8: u8 = opcode.get_bits(0, 7);

    let params = [i, imm3, imm8];
    let lengths = [1, 3, 8];

    Instruction::SUB_imm {
        rd: Reg::from(rd),
        rn: Reg::from(rn),
        imm32: thumb_expand_imm(&params, &lengths),
        setflags: s == 1,
        thumb32: true,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_imm_t4(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
