use bit_field::BitField;
use core::bits::*;
use core::register::Reg;
use core::instruction::Instruction;

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_reg_t1(command: u16) -> Instruction {
    Instruction::ADD_reg {
        rm: From::from(bits_6_9(command)),
        rn: From::from(bits_3_6(command)),
        rd: From::from(bits_0_3(command)),
        setflags: true,
    }
}

#[allow(non_snake_case)]
#[inline]
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
#[inline]
pub fn decode_ADD_imm_t1(command: u16) -> Instruction {
    Instruction::ADD_imm {
        rd: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        imm32: bits_6_9(command) as u32,
        setflags: true,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_imm_t2(command: u16) -> Instruction {
    Instruction::ADD_imm {
        rn: From::from(bits_8_11(command)),
        rd: From::from(bits_8_11(command)),
        imm32: bits_0_8(command) as u32,
        setflags: true,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_SP_imm_t1(command: u16) -> Instruction {
    Instruction::ADD_imm {
        rd: From::from(bits_8_11(command)),
        rn: Reg::SP,
        imm32: (bits_0_8(command) as u32) << 2,
        setflags: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ADD_SP_imm_t2(command: u16) -> Instruction {
    Instruction::ADD_imm {
        rd: Reg::SP,
        rn: Reg::SP,
        imm32: (bits_0_7(command) as u32) << 2,
        setflags: false,
    }
}
