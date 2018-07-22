use core::bits::*;
use core::condition::Condition;
use core::instruction::Instruction;
use core::operation::sign_extend;
use core::ThumbCode;

#[allow(non_snake_case)]
#[inline]
pub fn decode_B_t1_SVC_t1(opcode: u16) -> Instruction {
    let cond = opcode.get_bits(8, 11);
    if cond == 0b1111 {
        return Instruction::SVC {
            imm32: bits_0_8(opcode) as u32,
        };
    }
    if cond == 0b1110 {
        return Instruction::UDF {
            imm32: 0,
            opcode: ThumbCode::from(opcode),
        };
    }

    Instruction::B {
        cond: Condition::from_u16(cond).unwrap(),
        imm32: sign_extend((bits_0_8(opcode) as u32) << 1, 8, 32) as i32,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_B_t2(opcode: u16) -> Instruction {
    Instruction::B {
        cond: Condition::AL,
        imm32: sign_extend((opcode.get_bits(0, 10) as u32) << 1, 11, 32) as i32,
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_B_t3(opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_B_t4(opcode: u32) -> Instruction {
    unimplemented!()
}
