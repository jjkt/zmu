use bit_field::BitField;
use core::bits::*;
use core::register::Reg;
use core::instruction::Instruction;



#[allow(non_snake_case)]
pub fn decode_RSB_imm_t1(command: u16) -> Instruction {
    Instruction::RSB_imm {
        rd: From::from(bits_0_3(command)),
        rn: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        imm32: 0,
        setflags: true,
    }
}
