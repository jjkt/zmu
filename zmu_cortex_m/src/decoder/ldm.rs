use bit_field::BitField;

use crate::core::instruction::Instruction;
use crate::core::operation::get_reglist;
use crate::core::register::Reg;
use crate::core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_LDM_t1(opcode: u16) -> Instruction {
    let regs = get_reglist(opcode & 0b_1111_1111);

    Instruction::LDM {
        registers: regs,
        rn: Reg::from(opcode.get_bits(8..11) as u8),
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_LDMDB_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_LDM_t2(opcode: u32) -> Instruction {
    let regs = get_reglist((opcode & 0b1101_1111_1111_1111) as u16);

    Instruction::LDM {
        registers: regs,
        rn: Reg::from(opcode.get_bits(16..20) as u8),
        thumb32: true,
    }
}
