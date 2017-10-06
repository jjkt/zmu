use bit_field::BitField;

use core::register::Reg;
use core::instruction::Op;

#[allow(non_snake_case)]
pub fn decode_SUBS_imm_t1(command: u16) -> Op {
    Op::SUBS_imm {
        rd: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        imm32: command.get_bits(6..9) as i32,
    }
}

#[allow(non_snake_case)]
pub fn decode_SUBS_reg_t1(command: u16) -> Op {
    Op::SUBS_reg {
        rm: Reg::from_u16(command.get_bits(6..9) as u16).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6) as u16).unwrap(),
        rd: Reg::from_u16(command.get_bits(0..4) as u16).unwrap(),
    }
}