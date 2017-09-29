use bit_field::BitField;
use enum_set::EnumSet;

use register::Reg;
use instruction::Op;

#[allow(non_snake_case)]
pub fn decode_POP(command: u16) -> Op {
    let mut regs: EnumSet<Reg> = EnumSet::new();
    let reg_bits = command.get_bits(0..8);

    if reg_bits & 1 == 1 {
        regs.insert(Reg::R0);
    }
    if reg_bits & 2 == 2 {
        regs.insert(Reg::R1);
    }
    if reg_bits & 4 == 4 {
        regs.insert(Reg::R2);
    }
    if reg_bits & 8 == 8 {
        regs.insert(Reg::R3);
    }
    if reg_bits & 16 == 16 {
        regs.insert(Reg::R4);
    }
    if reg_bits & 32 == 32 {
        regs.insert(Reg::R5);
    }
    if reg_bits & 64 == 64 {
        regs.insert(Reg::R6);
    }
    if reg_bits & 128 == 128 {
        regs.insert(Reg::R7);
    }

    if command.get_bit(8) {
        regs.insert(Reg::LR);
    }

    Op::POP { registers: regs }
}