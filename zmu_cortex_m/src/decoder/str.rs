use core::bits::*;
use core::instruction::Instruction;
use core::register::Reg;
use core::ThumbCode;

#[allow(non_snake_case)]
pub fn decode_STR_imm_t1(command: u16) -> Instruction {
    Instruction::STR_imm {
        rt: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        imm32: (bits_6_11(command) as u32) << 2,
    }
}

#[allow(non_snake_case)]
pub fn decode_STR_imm_t2(command: u16) -> Instruction {
    Instruction::STR_imm {
        rn: Reg::SP,
        rt: From::from(bits_8_11(command)),
        imm32: (bits_0_8(command) as u32) << 2,
    }
}

#[allow(non_snake_case)]
pub fn decode_STR_reg_t1(command: u16) -> Instruction {
    Instruction::STR_reg {
        rt: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        rm: From::from(bits_6_9(command)),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRB_imm_t1(command: u16) -> Instruction {
    Instruction::STRB_imm {
        rt: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        imm32: (bits_6_11(command) as u32),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRB_reg_t1(command: u16) -> Instruction {
    Instruction::STRB_reg {
        rt: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        rm: From::from(bits_6_9(command)),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRH_imm_t1(command: u16) -> Instruction {
    Instruction::STRH_imm {
        rt: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        imm32: (bits_6_11(command) as u32) << 1,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_STRH_reg_t1(command: u16) -> Instruction {
    Instruction::STRH_reg {
        rt: From::from(bits_0_3(command)),
        rn: From::from(bits_3_6(command)),
        rm: From::from(bits_6_9(command)),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRB_imm_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRB_imm_t3(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRB_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRH_imm_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_STRH_imm_t3(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_STR_imm_t3(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_STR_imm_t4(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_STR_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_STRD_imm_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_STREXB_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_STREXH_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_STREX_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
