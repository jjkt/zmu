use crate::core::bits::Bits;
use crate::core::instruction::{Instruction, VLoadAndStoreParams};
use crate::core::register::{DoubleReg, ExtensionReg, Reg, SingleReg};

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_VSTR_t1(opcode: u32) -> Instruction {
    Instruction::VSTR {
        params: VLoadAndStoreParams {
            dd: ExtensionReg::Double {
                reg: DoubleReg::from(
                    opcode.get_bits(12..16) as u8 + ((opcode.get_bit(22) as u8) << 4) as u8,
                ),
            },
            rn: Reg::from(opcode.get_bits(16..20) as u8),
            imm32: opcode.get_bits(0..8) << 2,
            add: opcode.get_bit(23),
        },
    }
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_VSTR_t2(opcode: u32) -> Instruction {
    Instruction::VSTR {
        params: VLoadAndStoreParams {
            dd: ExtensionReg::Single {
                reg: SingleReg::from(
                    opcode.get_bits(12..16) as u8 + ((opcode.get_bit(22) as u8) << 4) as u8,
                ),
            },
            rn: Reg::from(opcode.get_bits(16..20) as u8),
            imm32: opcode.get_bits(0..8) << 2,
            add: opcode.get_bit(23),
        },
    }
}
