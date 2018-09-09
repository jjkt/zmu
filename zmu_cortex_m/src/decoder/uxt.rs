use bit_field::BitField;
use core::instruction::Instruction;
use core::ThumbCode;
use core::register::Reg;

#[allow(non_snake_case)]
#[inline]
pub fn decode_UXTB_t1(opcode: u16) -> Instruction {
    Instruction::UXTB {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn decode_UXTH_t1(opcode: u16) -> Instruction {
    Instruction::UXTH {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
        rotation: 0,
        thumb32: false,
    }
}
#[allow(non_snake_case)]
pub fn decode_UXTB_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
pub fn decode_UXTH_t2(opcode: u32) -> Instruction {
    Instruction::UXTH {
        rd: Reg::from(opcode.get_bits(0..3) as u8),
        rm: Reg::from(opcode.get_bits(3..6) as u8),
        rotation: 0,
        thumb32: true,
    }
}
