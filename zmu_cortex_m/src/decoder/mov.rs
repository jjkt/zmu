use bit_field::BitField;
use core::bits::*;
use core::register::Reg;
use core::instruction::Instruction;

#[allow(non_snake_case)]
pub fn decode_MOV_imm_t1(command: u16) -> Instruction {
    Instruction::MOV_imm {
        rd: From::from(bits_8_11(command)),
        imm32: bits_0_8(command) as u32,
        setflags: true,
    }
}
#[allow(non_snake_case)]
pub fn decode_MOV_reg_t1(command: u16) -> Instruction {
    Instruction::MOV_reg {
        rd: Reg::from_u16(((command.get_bit(7) as u16) << 3) + command.get_bits(0..3)).unwrap(),
        rm: Reg::from_u16(command.get_bits(3..7)).unwrap(),
        setflags: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_MOV_reg_t2_LSL_imm_t1(command: u16) -> Instruction {

    let imm5 = command.get_bits(6..11) as u8;

    if imm5 == 0
    {
        Instruction::MOV_reg {
            rd: From::from(bits_0_3(command)),
            rm: From::from(bits_3_6(command)),
            setflags: true,
        }
    }
    else
    {
    Instruction::LSL_imm {
        rd: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
        imm5: imm5,
        setflags: true,
    }
    }
}


#[test]
fn test_decode_mov_reg() {}
