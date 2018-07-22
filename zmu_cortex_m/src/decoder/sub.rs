use core::bits::*;
use core::register::Reg;
use core::instruction::Instruction;

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_imm_t1(command: u16) -> Instruction {
    Instruction::SUB_imm {
        rd: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        setflags: true,
        imm32: bits_6_9(command) as u32,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_imm_t2(command: u16) -> Instruction {
    Instruction::SUB_imm {
        rd: From::from(bits_8_11(command)),
        rn: From::from(bits_8_11(command)),
        setflags: true,
        imm32: bits_0_8(command) as u32,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_SP_imm_t1(command: u16) -> Instruction {
    Instruction::SUB_imm {
        rn: Reg::SP,
        rd: Reg::SP,
        imm32: (bits_0_7(command) as u32) << 2,
        setflags: false,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_reg_t1(command: u16) -> Instruction {
    Instruction::SUB_reg {
        rd: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        rm: From::from(bits_6_9(command)),
        setflags: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_SUB_reg_t2(_opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_imm_t3(opcode: u32) -> Instruction {
    unimplemented!()
}


#[allow(non_snake_case)]
#[inline]
pub fn decode_SUB_imm_t4(opcode: u32) -> Instruction {
    unimplemented!()
}
