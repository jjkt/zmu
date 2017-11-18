use bit_field::BitField;

use core::register::Reg;
use core::instruction::Instruction;


#[allow(non_snake_case)]
pub fn decode_LSL_imm_t1(command: u16) -> Instruction {
    Instruction::LSL_imm {
        rd: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rm: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        imm5: command.get_bits(6..11) as u8,
        setflags: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_LSL_reg_t1(command: u16) -> Instruction {
    Instruction::LSL_reg {
        rd: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rn: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rm: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        setflags: true,
    }
}