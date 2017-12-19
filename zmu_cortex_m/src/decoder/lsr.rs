use bit_field::BitField;
use core::bits::*;
use core::register::Reg;
use core::instruction::Instruction;


#[allow(non_snake_case)]
pub fn decode_LSR_imm_t1(command: u16) -> Instruction {
    Instruction::LSR_imm {
        rd: From::from(bits_0_3(command)),
        rm: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        imm5: command.get_bits(6..11) as u8,
        setflags: true,
    }
}
#[allow(non_snake_case)]
pub fn decode_LSR_reg_t1(command: u16) -> Instruction {
    Instruction::LSR_reg {
        rd: From::from(bits_0_3(command)),
        rn: From::from(bits_0_3(command)),
        rm: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        setflags: true,
    }
}
