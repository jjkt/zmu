use enum_set::EnumSet;

use bit_field::BitField;
use core::instruction::Instruction;
use core::register::Reg;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_PUSH_t1(opcode: u16) -> Instruction {
    let mut regs: EnumSet<Reg> = EnumSet::new();

    if opcode.get_bit(0) {
        regs.insert(Reg::R0);
    }
    if opcode.get_bit(1) {
        regs.insert(Reg::R1);
    }
    if opcode.get_bit(2) {
        regs.insert(Reg::R2);
    }
    if opcode.get_bit(3) {
        regs.insert(Reg::R3);
    }
    if opcode.get_bit(4) {
        regs.insert(Reg::R4);
    }
    if opcode.get_bit(5) {
        regs.insert(Reg::R5);
    }
    if opcode.get_bit(6) {
        regs.insert(Reg::R6);
    }
    if opcode.get_bit(7) {
        regs.insert(Reg::R7);
    }

    if opcode.get_bit(8) {
        regs.insert(Reg::LR);
    }

    Instruction::PUSH {
        registers: regs,
        thumb32: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_PUSH_t2(opcode: u32) -> Instruction {
    let mut regs: EnumSet<Reg> = EnumSet::new();

    if opcode.get_bit(0) {
        regs.insert(Reg::R0);
    }
    if opcode.get_bit(1) {
        regs.insert(Reg::R1);
    }
    if opcode.get_bit(2) {
        regs.insert(Reg::R2);
    }
    if opcode.get_bit(3) {
        regs.insert(Reg::R3);
    }
    if opcode.get_bit(4) {
        regs.insert(Reg::R4);
    }
    if opcode.get_bit(5) {
        regs.insert(Reg::R5);
    }
    if opcode.get_bit(6) {
        regs.insert(Reg::R6);
    }
    if opcode.get_bit(7) {
        regs.insert(Reg::R7);
    }
    if opcode.get_bit(8) {
        regs.insert(Reg::R8);
    }
    if opcode.get_bit(9) {
        regs.insert(Reg::R9);
    }
    if opcode.get_bit(10) {
        regs.insert(Reg::R10);
    }
    if opcode.get_bit(11) {
        regs.insert(Reg::R11);
    }
    if opcode.get_bit(12) {
        regs.insert(Reg::R12);
    }
    if opcode.get_bit(14) {
        regs.insert(Reg::LR);
    }

    Instruction::PUSH {
        registers: regs,
        thumb32: true,
    }
}

#[allow(non_snake_case)]
pub fn decode_PUSH_t3(_opcode: u32) -> Instruction {
    unimplemented!()
}
