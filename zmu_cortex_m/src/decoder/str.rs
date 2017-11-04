use bit_field::BitField;

use core::register::Reg;
use core::instruction::Instruction;


#[allow(non_snake_case)]
pub fn decode_STR_imm_t1(command: u16) -> Instruction {
    Instruction::STR_imm {
        rt: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        imm32: (command.get_bits(6..11) as u32) << 2,
    }
}

#[allow(non_snake_case)]
pub fn decode_STR_imm_t2(command: u16) -> Instruction {
    Instruction::STR_imm {
        rn: Reg::SP,
        rt: Reg::from_u16(command.get_bits(8..11)).unwrap(),
        imm32: (command.get_bits(0..8) as u32) << 2,
    }
}

#[allow(non_snake_case)]
/*pub fn decode_STR_reg_t1(command: u16) -> Instruction {
    Instruction::STR_reg {
        rt: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        rm: Reg::from_u16(command.get_bits(6..9)).unwrap(),
    }
}*/


#[allow(non_snake_case)]
pub fn decode_STRB_imm_t1(command: u16) -> Instruction {
    Instruction::STRB_imm {
        rt: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        imm32: (command.get_bits(6..11) as u32),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRH_imm_t1(command: u16) -> Instruction {
    Instruction::STRH_imm {
        rt: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        imm32: (command.get_bits(6..11) as u32) << 1,
    }
}
