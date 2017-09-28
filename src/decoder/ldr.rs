use bit_field::BitField;

use register::Reg;
use instruction::Op;

#[allow(non_snake_case)]
pub fn decode_LDR_imm_t2(command: u16) -> Op {
    Op::LDR_imm {
        rt: Reg::from_u16(command.get_bits(8..11)).unwrap(),
        rn: Reg::PC,
        imm32: (command.get_bits(0..8) as u32) << 2,
    }
}
#[allow(non_snake_case)]
pub fn decode_LDR_imm_t1(command: u16) -> Op {
    Op::LDR_imm {
        rt: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        imm32: (command.get_bits(6..11) as u32) << 2,
    }
}

#[allow(non_snake_case)]
pub fn decode_LDR_lit_t1(command: u16) -> Op {
    Op::LDR_lit {
        rt: Reg::from_u16(command.get_bits(8..11)).unwrap(),
        imm32: (command.get_bits(0..8) as u32) << 2,
    }
}

#[allow(non_snake_case)]
pub fn decode_LDR_reg_t1(command: u16) -> Op {
    Op::LDR_reg {
        rt: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        rm: Reg::from_u16(command.get_bits(6..9)).unwrap(),
    }
}

