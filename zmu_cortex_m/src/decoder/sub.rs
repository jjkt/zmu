use bit_field::BitField;

use core::register::Reg;
use core::instruction::Instruction;

#[allow(non_snake_case)]
pub fn decode_SUB_imm_t1(command: u16) -> Instruction {
    Instruction::SUB_imm {
        rd: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        setflags: true,
        imm32: command.get_bits(6..9) as u32,
    }
}

#[allow(non_snake_case)]
pub fn decode_SUB_imm_t2(command: u16) -> Instruction {
    Instruction::SUB_imm {
        rd: Reg::from_u16(command.get_bits(8..11)).unwrap(),
        rn: Reg::from_u16(command.get_bits(8..11)).unwrap(),
        setflags: true,
        imm32: command.get_bits(0..8) as u32,
    }
}

#[allow(non_snake_case)]
pub fn decode_SUB_SP_imm_t1(command: u16) -> Instruction {
    Instruction::SUB_imm {
        rn: Reg::SP,
        rd: Reg::SP,
        imm32: (command.get_bits(0..7) as u32) << 2,
        setflags: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_SUBS_reg_t1(command: u16) -> Instruction {
    Instruction::SUBS_reg {
        rd: Reg::from_u16(command.get_bits(0..3) as u16).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6) as u16).unwrap(),
        rm: Reg::from_u16(command.get_bits(6..9) as u16).unwrap(),
    }
}
