use core::bits::*;
use enum_set::EnumSet;

use core::instruction::Instruction;
use core::register::Reg;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_POP_reg_t1(opcode: u16) -> Instruction {
    let mut regs: EnumSet<Reg> = EnumSet::new();

    if bit_0(opcode) == 1 {
        regs.insert(Reg::R0);
    }
    if bit_1(opcode) == 1 {
        regs.insert(Reg::R1);
    }
    if bit_2(opcode) == 1 {
        regs.insert(Reg::R2);
    }
    if bit_3(opcode) == 1 {
        regs.insert(Reg::R3);
    }
    if bit_4(opcode) == 1 {
        regs.insert(Reg::R4);
    }
    if bit_5(opcode) == 1 {
        regs.insert(Reg::R5);
    }
    if bit_6(opcode) == 1 {
        regs.insert(Reg::R6);
    }
    if bit_7(opcode) == 1 {
        regs.insert(Reg::R7);
    }
    if bit_8(opcode) == 1 {
        regs.insert(Reg::PC);
    }

    Instruction::POP { registers: regs }
}

#[allow(non_snake_case)]
pub fn decode_POP_W_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_POP_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_POP_t3(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
