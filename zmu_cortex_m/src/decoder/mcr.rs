use crate::core::bits::*;
use crate::core::instruction::Instruction;
use crate::core::register::Reg;
use crate::core::ThumbCode;

#[allow(non_snake_case)]
pub fn decode_MCR2_t2(opcode: u32) -> Instruction {
    let reg: u8 = opcode.get_bits(12, 15);
    Instruction::MCR2 {
        rt: Reg::from(reg),
        coproc: opcode.get_bits(8, 11),
        opc1: opcode.get_bits(21, 23),
        opc2: opcode.get_bits(5, 7),
        crn: opcode.get_bits(16, 19),
        crm: opcode.get_bits(0, 3),
    }
}

#[allow(non_snake_case)]
pub fn decode_MCR_t1(opcode: u32) -> Instruction {
    let reg: u8 = opcode.get_bits(12, 15);
    Instruction::MCR {
        rt: Reg::from(reg),
        coproc: opcode.get_bits(8, 11),
        opc1: opcode.get_bits(21, 23),
        opc2: opcode.get_bits(5, 7),
        crn: opcode.get_bits(16, 19),
        crm: opcode.get_bits(0, 3),
    }
}

#[allow(non_snake_case)]
pub fn decode_MCRR2_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_MCRR_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_MRC2_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_MRC_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
