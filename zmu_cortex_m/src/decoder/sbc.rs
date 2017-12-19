use bit_field::BitField;
use core::bits::*;
use core::register::Reg;
use core::instruction::Instruction;



#[allow(non_snake_case)]
pub fn decode_SBC_reg_t1(command: u16) -> Instruction {
    Instruction::SBC_reg {
        rn: From::from(bits_0_3(command)),
        rd: From::from(bits_0_3(command)),
        rm: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        setflags: true,
    }
}
