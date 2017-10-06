use bit_field::BitField;
use core::register::Reg;
use core::instruction::Op;

#[allow(non_snake_case)]
pub fn decode_CMP_imm_t1(command: u16) -> Op {
    Op::CMP_imm {
        rn: Reg::from_u16(command.get_bits(7..10)).unwrap(),
        imm32: command.get_bits(0..8) as u32,
    }
}

#[allow(non_snake_case)]
pub fn decode_CMP_t1(command: u16) -> Op {
    Op::CMP {
        rn: Reg::from_u16(command.get_bits(0..3) as u16).unwrap(),
        rm: Reg::from_u16(command.get_bits(3..6) as u16).unwrap(),
    }
}

#[allow(non_snake_case)]
pub fn decode_CMP_t2(command: u16) -> Op {
    Op::CMP {
        rn: Reg::from_u16(command.get_bits(0..3) + ((command.get_bit(7) as u8) << 4) as u16)
            .unwrap(),
        rm: Reg::from_u16(command.get_bits(3..7) as u16).unwrap(),
    }
}