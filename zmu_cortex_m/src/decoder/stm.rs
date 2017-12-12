use bit_field::BitField;
use enum_set::EnumSet;

use core::register::Reg;
use core::instruction::Instruction;

#[allow(non_snake_case)]
pub fn decode_STM_t1(command: u16) -> Instruction {
    let mut regs: EnumSet<Reg> = EnumSet::new();

    if command.get_bit(0) {
        regs.insert(Reg::R0);
    }
    if command.get_bit(1) {
        regs.insert(Reg::R1);
    }
    if command.get_bit(2) {
        regs.insert(Reg::R2);
    }
    if command.get_bit(3) {
        regs.insert(Reg::R3);
    }
    if command.get_bit(4) {
        regs.insert(Reg::R4);
    }
    if command.get_bit(5) {
        regs.insert(Reg::R5);
    }
    if command.get_bit(6) {
        regs.insert(Reg::R6);
    }
    if command.get_bit(7) {
        regs.insert(Reg::R7);
    }

    Instruction::STM {
        registers: regs,
        rn: Reg::from_u16(command.get_bits(8..11)).unwrap(),
    }
}
