use bit_field::BitField;

use register::Reg;
use instruction::Op;

#[allow(non_snake_case)]
pub fn decode_MOV_imm_t1(command: u16) -> Op {
    Op::MOV_imm {
        rd: Reg::from_u16(command.get_bits(7..10)).unwrap(),
        imm32: command.get_bits(0..8) as u32,
    }
}
#[allow(non_snake_case)]
pub fn decode_MOV_reg_t1(command: u16) -> Op {
    Op::MOV_reg {
        rd: Reg::from_u16(((command.get_bit(7) as u16) << 3) + command.get_bits(0..3)).unwrap(),
        rm: Reg::from_u16(command.get_bits(3..7)).unwrap(),
        setflags: false,
    }
}

#[allow(non_snake_case)]
pub fn decode_MOV_reg_t2(command: u16) -> Op {
    Op::MOV_reg {
        rd: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rm: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        setflags: true,
    }
}


#[test]
fn test_decode_mov_reg() {

}