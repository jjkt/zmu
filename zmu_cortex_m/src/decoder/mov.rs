use core::bits::*;
use core::register::Reg;
use core::instruction::Instruction;

#[allow(non_snake_case)]
#[inline]
pub fn decode_MOV_imm_t1(command: u16) -> Instruction {
    Instruction::MOV_imm {
        rd: Reg::from(bits_8_11(command)),
        imm32: bits_0_8(command) as u32,
        setflags: true,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_MOV_reg_t1(command: u16) -> Instruction {
    Instruction::MOV_reg {
        rd: Reg::from((bit_7(command) << 3) + bits_0_3(command)),
        rm: Reg::from(bits_3_7(command)),
        setflags: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_MOV_reg_t2_LSL_imm_t1(command: u16) -> Instruction {
    let imm5 = bits_6_11(command) as u8;

    if imm5 == 0 {
        Instruction::MOV_reg {
            rd: Reg::from(bits_0_3(command)),
            rm: Reg::from(bits_3_6(command)),
            setflags: true,
        }
    } else {
        Instruction::LSL_imm {
            rd: Reg::from(bits_0_3(command)),
            rm: Reg::from(bits_3_6(command)),
            imm5: imm5,
            setflags: true,
        }
    }
}
