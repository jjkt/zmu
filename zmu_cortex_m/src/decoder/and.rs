use bit_field::BitField;
use core::instruction::Instruction;
use core::operation::thumb_expand_imm_c;
use core::register::Reg;
use core::instruction::Imm32Carry;


#[allow(non_snake_case)]
#[inline]
pub fn decode_AND_reg_t1(opcode: u16) -> Instruction {
    Instruction::AND_reg {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rn: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
        setflags: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_AND_reg_t2(opcode: u32) -> Instruction {
        unimplemented!()

}

#[allow(non_snake_case)]
pub fn decode_AND_imm_t1(opcode: u32) -> Instruction {
    let rd: u8 = opcode.get_bits(8..12) as u8;
    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm8: u8 = opcode.get_bits(0..8) as u8;
    let i: u8 = opcode.get_bit(26) as u8;
    let rn: u8 = opcode.get_bits(16..20) as u8;
    let s: bool = opcode.get_bit(20);

    let params = [i, imm3, imm8];
    let lengths = [1, 3, 8];

    Instruction::AND_imm {
        rd: Reg::from(rd),
        rn: Reg::from(rn),
        imm32: Imm32Carry::Carry {
            imm32_c0: thumb_expand_imm_c(&params, &lengths, false),
            imm32_c1: thumb_expand_imm_c(&params, &lengths, true),
        },
        setflags: s,
    }
}
