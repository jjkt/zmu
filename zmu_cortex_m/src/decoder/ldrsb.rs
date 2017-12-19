use bit_field::BitField;
use core::bits::*;
use core::register::Reg;
use core::instruction::Instruction;


#[allow(non_snake_case)]
pub fn decode_LDRSB_reg_t1(command: u16) -> Instruction {
    Instruction::LDRSB_reg {
        rt: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        rm: Reg::from_u16(command.get_bits(6..9)).unwrap(),
    }
}
