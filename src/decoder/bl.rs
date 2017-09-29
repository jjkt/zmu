use instruction::Op;
use operation::sign_extend;

pub fn decode_bl(t1: u16, t2: u16) -> Op {
    let s = ((t1 >> 10) & 1) as u32;
    let imm10 = (t1 & 0x3ff) as u32;

    let j1 = ((t2 >> 13) & 1) as u32;
    let j2 = ((t2 >> 11) & 1) as u32;
    let imm11 = (t2 & 0x7ff) as u32;


    let i1 = ((j1 ^ s) ^ 1) as u32;
    let i2 = ((j2 ^ s) ^ 1) as u32;

    let imm = sign_extend((imm11 << 1) + (imm10 << 12) + (i2 << 22) + (i1 << 23) + (s << 24),
                          24,
                          32);

    Op::BL { imm32: imm as i32 }
}
