use core::bits::*;
use core::instruction::Instruction;
use core::operation::thumb_expand_imm_c;
use core::register::Reg;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_ORR_reg_t1(command: u16) -> Instruction {
    Instruction::ORR_reg {
        rd: From::from(bits_0_3(command)),
        rn: From::from(bits_0_3(command)),
        rm: From::from(bits_3_6(command)),
        setflags: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_ORR_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_ORR_imm_t1(opcode: u32) -> Instruction {
    let rd: u8 = opcode.get_bits(8, 11);
    let rn: u8 = opcode.get_bits(16, 19);

    let s : u8 = opcode.get_bits(20, 20);

    Instruction::ORR_imm {
        rd: Reg::from(rd),
        rn: Reg::from(rn),
        imm32: thumb_expand_imm_c(/*i, imm3, imm8*/),
        setflags: s == 1,
    }
}
