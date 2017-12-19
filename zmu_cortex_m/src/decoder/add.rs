use bit_field::BitField;
use core::bits::*;
use core::register::Reg;
use core::instruction::Instruction;

#[allow(non_snake_case)]
pub fn decode_ADD_reg_t1(command: u16) -> Instruction {
    Instruction::ADD_reg {
        rm: Reg::from_u16(command.get_bits(6..9)).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        rd: From::from(bits_0_3(command)),
        setflags: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_ADD_reg_t2_ADD_SP_reg(command: u16) -> Instruction {
    let rdn = Reg::from_u16(((command.get_bit(7) as u16) << 3) + command.get_bits(0..3)).unwrap();

    Instruction::ADD_reg {
        rm: Reg::from_u16(command.get_bits(3..7)).unwrap(),
        rd: rdn,
        rn: rdn,
        setflags: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_ADD_imm_t1(command: u16) -> Instruction {
    Instruction::ADD_imm {
        rd: From::from(bits_0_3(command)),
        rn: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        imm32: command.get_bits(6..9) as u32,
        setflags: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_ADD_imm_t2(command: u16) -> Instruction {
    Instruction::ADD_imm {
        rn: Reg::from_u16(command.get_bits(8..11)).unwrap(),
        rd: Reg::from_u16(command.get_bits(8..11)).unwrap(),
        imm32: command.get_bits(0..8) as u32,
        setflags: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_ADD_SP_imm_t1(command: u16) -> Instruction {
    Instruction::ADD_imm {
        rd: Reg::from_u16(command.get_bits(8..11)).unwrap(),
        rn: Reg::SP,
        imm32: (command.get_bits(0..8) as u32) << 2,
        setflags: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_ADD_SP_imm_t2(command: u16) -> Instruction {
    Instruction::ADD_imm {
        rd: Reg::SP,
        rn: Reg::SP,
        imm32: (command.get_bits(0..7) as u32) << 2,
        setflags: false,
    }
}
