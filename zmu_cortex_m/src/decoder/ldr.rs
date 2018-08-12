//use core::bits::*;
use bit_field::*;
use core::instruction::Instruction;
use core::register::Reg;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_LDR_imm_t1(command: u16) -> Instruction {
    Instruction::LDR_imm {
        rt: Reg::from(command.get_bits(0..3) as u8),
        rn: Reg::from(command.get_bits(3..6) as u8),
        imm32: (command.get_bits(6..11) as u32) << 2,
        index: true,
        add: true,
        wback: false,
        thumb32: false
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_LDR_imm_t2(command: u16) -> Instruction {
    Instruction::LDR_imm {
        rt: From::from(command.get_bits(8..11) as u8),
        rn: Reg::SP,
        imm32: (command.get_bits(0..8) as u32) << 2,
        index: true,
        add: true,
        wback: false,
        thumb32: false
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_LDR_lit_t1(command: u16) -> Instruction {
    Instruction::LDR_lit {
        rt: Reg::from(command.get_bits(8..11) as u8),
        imm32: (command.get_bits(0..8) as u32) << 2,
        thumb32: false
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_LDR_reg_t1(command: u16) -> Instruction {
    Instruction::LDR_reg {
        rt: Reg::from(command.get_bits(0..3) as u8),
        rn: Reg::from(command.get_bits(3..6) as u8),
        rm: Reg::from(command.get_bits(6..9) as u8),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDR_imm_t3(opcode: u32) -> Instruction {
    // ARMv7-M

    Instruction::LDR_imm {
        rt: From::from(opcode.get_bits(12..16) as u8),
        rn: From::from(opcode.get_bits(16..20) as u8),
        imm32: opcode.get_bits(0..12),
        index: true,
        add: true,
        wback: false,
        thumb32: true
    }
}

#[allow(non_snake_case)]
pub fn decode_LDR_imm_t4(opcode: u32) -> Instruction {
    // ARMv7-M
    Instruction::LDR_imm {
        rt: From::from(opcode.get_bits(12..16) as u8),
        rn: From::from(opcode.get_bits(16..20) as u8),
        imm32: opcode.get_bits(0..8),
        index: opcode.get_bit(10),
        add: opcode.get_bit(9),
        wback: opcode.get_bit(8),
        thumb32: true
    }
}

#[allow(non_snake_case)]
pub fn decode_LDR_lit_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDR_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRBT_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRD_imm_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRD_lit_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDREXB_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDREXH_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDREX_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRHT_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRSBT_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRSHT(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRT_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
