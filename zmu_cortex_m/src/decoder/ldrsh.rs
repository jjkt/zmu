use bit_field::BitField;
use core::instruction::Instruction;
use core::instruction::SRType;
use core::register::Reg;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_LDRSH_reg_t1(opcode: u16) -> Instruction {
    Instruction::LDRSH_reg {
        rt: Reg::from(opcode.get_bits(0..3) as u8),
        rn: Reg::from(opcode.get_bits(3..6) as u8),
        rm: Reg::from(opcode.get_bits(6..9) as u8),
        shift_t: SRType::LSL,
        shift_n: 0,
        index: true,
        add: true,
        wback: false,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRSH_imm_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRSH_imm_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRSH_lit_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDRSH_reg_t2(opcode: u32) -> Instruction {
    Instruction::LDRSH_reg {
        rm: Reg::from(opcode.get_bits(0..4) as u8),
        rt: Reg::from(opcode.get_bits(12..16) as u8),
        rn: Reg::from(opcode.get_bits(16..20) as u8),
        shift_t: SRType::LSL,
        shift_n: opcode.get_bits(4..6) as u8,
        index: true,
        add: true,
        wback: false,
        thumb32: true,
    }
}
