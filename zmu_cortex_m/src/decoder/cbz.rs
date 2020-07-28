use crate::core::bits::Bits;
use crate::core::instruction::{Instruction, ParamsCbnz, ParamsCbz};
use crate::core::register::Reg;

#[allow(non_snake_case)]
pub fn decode_CBZ_t1(opcode: u16) -> Instruction {
    let nonzero = opcode.get_bit(11);
    let rn = Reg::from(opcode.get_bits(0..3) as u8);
    let imm32 = ((opcode.get_bit(9) as u32) << 6) + (u32::from(opcode.get_bits(3..8)) << 1);

    if nonzero {
        Instruction::CBNZ {
            params: ParamsCbnz { rn, imm32 },
        }
    } else {
        Instruction::CBZ {
            params: ParamsCbz { rn, imm32 },
        }
    }
}
