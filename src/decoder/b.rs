use bit_field::BitField;

use core::condition::Condition;
use core::instruction::Instruction;
use core::operation::sign_extend;

#[allow(non_snake_case)]
pub fn decode_B_t1(command: u16) -> Instruction {
    let cond = command.get_bits(8..12);
    if cond == 0b1111 {
        return Instruction::SVC;
    }
    if cond == 0b1110 {
        return Instruction::UDF;
    }

    Instruction::B {
        cond: Condition::from_u16(cond).unwrap(),
        imm32: sign_extend((command.get_bits(0..8) as u32) << 1, 8, 32),
    }
}

#[allow(non_snake_case)]
pub fn decode_B_t2(command: u16) -> Instruction {
    Instruction::B {
        cond: Condition::AL,
        imm32: sign_extend((command.get_bits(0..11) as u32) << 1, 11, 32),
    }
}
