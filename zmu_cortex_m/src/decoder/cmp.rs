use bit_field::BitField;
use core::register::Reg;
use core::instruction::Instruction;
use core::bits::*;

#[allow(non_snake_case)]
pub fn decode_CMP_imm_t1(command: u16) -> Instruction {
    Instruction::CMP_imm {
        rn: From::from(bits_8_11(command)),
        imm32: bits_0_8(command) as u32,
    }
}

#[allow(non_snake_case)]
pub fn decode_CMP_reg_t1(command: u16) -> Instruction {
    Instruction::CMP_reg {
        rn: Reg::from_u16(command.get_bits(0..3) as u16).unwrap(),
        rm: Reg::from_u16(command.get_bits(3..6) as u16).unwrap(),
    }
}

#[allow(non_snake_case)]
pub fn decode_CMP_reg_t2(command: u16) -> Instruction {
    Instruction::CMP_reg {
        rn: Reg::from_u16(command.get_bits(0..3) + ((command.get_bit(7) as u8) << 4) as u16)
            .unwrap(),
        rm: Reg::from_u16(command.get_bits(3..7) as u16).unwrap(),
    }
}
