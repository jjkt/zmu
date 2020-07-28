use crate::core::bits::Bits;
use crate::core::instruction::{BfiParams, Instruction};
use crate::core::register::Reg;

#[allow(non_snake_case)]
pub fn decode_BFI_t1(opcode: u32) -> Instruction {
    let rn: u8 = opcode.get_bits(16..20) as u8;
    let rd: u8 = opcode.get_bits(8..12) as u8;

    let imm3: u8 = opcode.get_bits(12..15) as u8;
    let imm2: u8 = opcode.get_bits(6..8) as u8;

    let lsbit = u32::from((imm3 << 2) + imm2);
    let msbit = opcode.get_bits(0..5);

    // msbit = lsbit + width -1   <=>
    // width = msbit - lsbit + 1
    let width = msbit - lsbit + 1;

    Instruction::BFI {
        params: BfiParams {
            rd: Reg::from(rd),
            rn: Reg::from(rn),
            lsbit: lsbit as usize,
            width: width as usize,
        },
    }
}
