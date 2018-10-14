use bit_field::BitField;
use core::instruction::Instruction;
use core::operation::decode_imm_shift;
use core::register::Reg;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_ASR_imm_t1(opcode: u16) -> Instruction {
    let imm5 = opcode.get_bits(6..11) as u8;
    let (_, shift_n) = decode_imm_shift(0b10, imm5);

    Instruction::ASR_imm {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
        shift_n,
        setflags: true,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ASR_reg_t1(opcode: u16) -> Instruction {
    Instruction::ASR_reg {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rn: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
        setflags: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_ASR_imm_t2(opcode: u32) -> Instruction {
    let rm: u8 = opcode.get_bits(0..4) as u8;
    let rd: u8 = opcode.get_bits(8..12) as u8;
    let s: u8 = opcode.get_bit(20) as u8;

    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm2: u8 = opcode.get_bits(6..8) as u8;

    let (_, shift_n) = decode_imm_shift(0b10, (imm3 << 2) + imm2);

    Instruction::ASR_imm {
        rd: Reg::from(rd),
        rm: Reg::from(rm),
        setflags: s == 1,
        shift_n: shift_n,
        thumb32: true,
    }

}

#[allow(non_snake_case)]
pub fn decode_ASR_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
