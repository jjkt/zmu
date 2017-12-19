use bit_field::BitField;
use core::bits::*;
use core::instruction::Instruction;


#[allow(non_snake_case)]
pub fn decode_LDRH_reg_t1(command: u16) -> Instruction {
    Instruction::LDRH_reg {
        rt: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        rm: From::from(bits_6_9(command)),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRH_imm_t1(command: u16) -> Instruction {
    Instruction::LDRH_imm {
        rt: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        imm32: (command.get_bits(6..11) as u32) << 1,
    }
}
