use bit_field::BitField;
use register::Reg;
use instruction::Op;

#[allow(non_snake_case)]
pub fn decode_TST_reg_t1(command: u16) -> Op {
    Op::TST_reg {
        rn: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rm: Reg::from_u16(command.get_bits(3..6)).unwrap(),
    }
}