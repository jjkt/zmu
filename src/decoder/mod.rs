use bit_field::BitField;
use enum_set::EnumSet;

use register::Reg;
use instruction::Op;
use condition::Condition;
use operation::sign_extend;

mod mov;
mod ldr;
mod add;

use decoder::mov::*;
use decoder::ldr::*;
use decoder::add::*;

pub fn is_thumb32(word: u16) -> bool {
    match word >> 11 {
        0b11101 | 0b11110 | 0b11111 => true,
        _ => false,
    }
}



#[allow(non_snake_case)]
fn decode_CMP_imm_t1(command: u16) -> Op {
    Op::CMP_imm {
        rn: Reg::from_u16(command.get_bits(7..10)).unwrap(),
        imm32: command.get_bits(0..8) as u32,
    }
}


#[allow(non_snake_case)]
fn decode_SUBS_imm_t1(command: u16) -> Op {
    Op::SUBS_imm {
        rd: Reg::from_u16(command.get_bits(0..3)).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6)).unwrap(),
        imm32: command.get_bits(6..9) as i32,
    }
}


#[allow(non_snake_case)]
fn decode_BX(command: u16) -> Op {

    Op::BX { rm: Reg::from_u16((command >> 3) & 0xf).unwrap() }
}




#[allow(non_snake_case)]
fn decode_POP(command: u16) -> Op {
    let mut regs: EnumSet<Reg> = EnumSet::new();
    let reg_bits = command.get_bits(0..8);

    if reg_bits & 1 == 1 {
        regs.insert(Reg::R0);
    }
    if reg_bits & 2 == 2 {
        regs.insert(Reg::R1);
    }
    if reg_bits & 4 == 4 {
        regs.insert(Reg::R2);
    }
    if reg_bits & 8 == 8 {
        regs.insert(Reg::R3);
    }
    if reg_bits & 16 == 16 {
        regs.insert(Reg::R4);
    }
    if reg_bits & 32 == 32 {
        regs.insert(Reg::R5);
    }
    if reg_bits & 64 == 64 {
        regs.insert(Reg::R6);
    }
    if reg_bits & 128 == 128 {
        regs.insert(Reg::R7);
    }

    if command.get_bit(8) {
        regs.insert(Reg::LR);
    }

    Op::POP { registers: regs }
}

#[allow(non_snake_case)]
fn decode_PUSH(command: u16) -> Op {
    let mut regs: EnumSet<Reg> = EnumSet::new();
    let reg_bits = command.get_bits(0..8);

    if reg_bits & 1 == 1 {
        regs.insert(Reg::R0);
    }
    if reg_bits & 2 == 2 {
        regs.insert(Reg::R1);
    }
    if reg_bits & 4 == 4 {
        regs.insert(Reg::R2);
    }
    if reg_bits & 8 == 8 {
        regs.insert(Reg::R3);
    }
    if reg_bits & 16 == 16 {
        regs.insert(Reg::R4);
    }
    if reg_bits & 32 == 32 {
        regs.insert(Reg::R5);
    }
    if reg_bits & 64 == 64 {
        regs.insert(Reg::R6);
    }
    if reg_bits & 128 == 128 {
        regs.insert(Reg::R7);
    }
    if command.get_bit(8) {
        regs.insert(Reg::LR);
    }

    Op::PUSH { registers: regs }
}

#[allow(non_snake_case)]
fn decode_B_t1(command: u16) -> Op {
    let cond = command.get_bits(8..12);
    if cond == 0b1111 {
        return Op::SVC;
    }
    if cond == 0b1110 {
        return Op::UDF;
    }

    Op::B {
        cond: Condition::from_u16(cond).unwrap(),
        imm32: sign_extend((command.get_bits(0..8) as u32) << 1, 8, 32),
    }
}

#[allow(non_snake_case)]
fn decode_B_t2(command: u16) -> Op {
    Op::B {
        cond: Condition::AL,
        imm32: sign_extend((command.get_bits(0..11) as u32) << 1, 11, 32),
    }
}

#[allow(non_snake_case)]
fn decode_CMP_t1(command: u16) -> Op {
    Op::CMP {
        rn: Reg::from_u16(command.get_bits(0..3) as u16).unwrap(),
        rm: Reg::from_u16(command.get_bits(3..6) as u16).unwrap(),
    }
}

#[allow(non_snake_case)]
fn decode_CMP_t2(command: u16) -> Op {
    Op::CMP {
        rn: Reg::from_u16(command.get_bits(0..3) + ((command.get_bit(7) as u8) << 4) as u16)
            .unwrap(),
        rm: Reg::from_u16(command.get_bits(3..7) as u16).unwrap(),
    }
}

#[allow(non_snake_case)]
fn decode_BLX(command: u16) -> Op {
    Op::BLX { rm: Reg::from_u16(command.get_bits(3..7) as u16).unwrap() }
}

#[allow(non_snake_case)]
fn decode_SUBS_reg_t1(command: u16) -> Op {
    Op::SUBS_reg {
        rm: Reg::from_u16(command.get_bits(6..9) as u16).unwrap(),
        rn: Reg::from_u16(command.get_bits(3..6) as u16).unwrap(),
        rd: Reg::from_u16(command.get_bits(0..4) as u16).unwrap(),
    }
}

pub fn decode_16(command: u16) -> Option<Op> {
    match command & 0xc000 {
        0b0000_0000_0000_0000_u16 => {
            // Shift (immediate), add, substract, move and compare
            match command.get_bits(9..14) {
                0b000_01 | 0b000_10 | 0b000_11 => Some(Op::LSL_imm),
                0b001_00 | 0b001_01 | 0b001_10 | 0b001_11 => Some(Op::LSR_imm),
                0b010_00 | 0b010_01 | 0b010_10 | 0b010_11 => Some(Op::ASR),
                0b011_00 => Some(decode_ADDS(command)),
                0b011_01 => Some(decode_SUBS_reg_t1(command)),
                0b011_10 => Some(decode_ADDS_imm_t1(command)),
                0b011_11 => Some(decode_SUBS_imm_t1(command)),
                0b100_00 | 0b100_01 | 0b100_10 | 0b100_11 => Some(decode_MOV_imm_t1(command)),
                0b101_00 | 0b101_01 | 0b101_10 | 0b101_11 => Some(decode_CMP_imm_t1(command)),
                0b110_00 | 0b110_01 | 0b110_10 | 0b110_11 => Some(decode_ADDS_imm_t2(command)), 
                0b111_00 | 0b111_01 | 0b111_10 | 0b111_11 => Some(decode_ADDS_imm_t2(command)), 
                0 => Some(decode_MOV_reg_t2(command)),
                _ => None,
            }
        }
        0b0100_0000_0000_0000_u16 => {
            // data process, special data, load from lp...
            if (command & 0x800) == 0 {
                match command & 0xffc0 {
                    0b010000_0000_000000_u16 => Some(Op::AND),
                    0b010000_0001_000000_u16 => Some(Op::EOR),
                    0b010000_0010_000000_u16 => Some(Op::LSL_imm),
                    0b010000_0011_000000_u16 => Some(Op::LSR_imm),
                    0b010000_0100_000000_u16 => Some(Op::ASR),
                    0b010000_0101_000000_u16 => Some(Op::ADC),
                    0b010000_0110_000000_u16 => Some(Op::SBC),
                    0b010000_0111_000000_u16 => Some(Op::ROR),
                    0b010000_1000_000000_u16 => Some(Op::TST),
                    0b010000_1001_000000_u16 => Some(Op::RSB),
                    0b010000_1010_000000_u16 => Some(decode_CMP_t1(command)),
                    0b010000_1011_000000_u16 => Some(Op::CMN),
                    0b010000_1100_000000_u16 => Some(Op::ORR),
                    0b010000_1101_000000_u16 => Some(Op::MUL),
                    0b010000_1110_000000_u16 => Some(Op::BIC),
                    0b010000_1111_000000_u16 => Some(Op::MVN_reg),

                    0b010001_0000_000000_u16 |
                    0b010001_0001_000000_u16 |
                    0b010001_0010_000000_u16 |
                    0b010001_0011_000000_u16 => Some(decode_ADD(command)),

                    0b010001_0100_000000_u16 => None,
                    0b010001_0101_000000_u16 |
                    0b010001_0110_000000_u16 |
                    0b010001_0111_000000_u16 => Some(decode_CMP_t2(command)),
                    0b010001_1000_000000_u16 |
                    0b010001_1001_000000_u16 |
                    0b010001_1010_000000_u16 |
                    0b0100_0110_1100_0000_u16 => Some(decode_MOV_reg_t1(command)),
                    0b0100_0111_0100_0000_u16 => Some(decode_BX(command)),
                    0b010001_1110_000000_u16 |
                    0b010001_1111_000000_u16 => Some(decode_BLX(command)),
                    _ => None,

                }
            } else {

                match command.get_bits(11..16) {
                    0b01101 => Some(decode_LDR_imm_t1(command)),
                    0b01001 => Some(decode_LDR_lit_t1(command)),
                    0b01011 => Some(decode_LDR_reg_t1(command)),
                    _ => None,
                }
            }
        }
        0b1000_0000_0000_0000_u16 => {
            // generate pc relative addr, sp rela, misc
            // LDR_imm_t2 = 10011
            match command.get_bits(11..16) {
                0b10011 => Some(decode_LDR_imm_t2(command)),
                _ => {
                    match command.get_bits(9..14) {
                        0b11110 => Some(decode_POP(command)),
                        0b11010 => Some(decode_PUSH(command)),
                        _ => None,
                    }
                }
            }

        }
        0b1100_0000_0000_0000_u16 => {
            // store, load multiple, branch, svc, uncond branch
            match command.get_bits(12..16) {
                0b1101 => Some(decode_B_t1(command)),
                0b1110 => Some(decode_B_t2(command)),
                _ => None,

            }
        }

        _ => None,
    }
}
//A 5.3.1 Branch and misc (thumb32)
pub fn decode_branch_and_misc(t1: u16, t2: u16) -> Option<Op> {
    let op1 = (t1 >> 4) & 0x7f;
    let op2 = (t2 >> 12) & 0x07;

    match op2 {
        0x7 | 0x5 => {
            let s = ((t1 >> 10) & 1) as u32;
            let imm10 = (t1 & 0x3ff) as u32;

            let j1 = ((t2 >> 13) & 1) as u32;
            let j2 = ((t2 >> 11) & 1) as u32;
            let imm11 = (t2 & 0x7ff) as u32;


            let i1 = ((j1 ^ s) ^ 1) as u32;
            let i2 = ((j2 ^ s) ^ 1) as u32;

            let imm = sign_extend((imm11 << 1) + (imm10 << 12) + (i2 << 22) + (i1 << 23) +
                                  (s << 24),
                                  24,
                                  32);

            Some(Op::BL { imm32: imm as i32 })
        }
        _ => None,
    }


}

// A5.3 check thumb32 encodings
pub fn decode_32(t1: u16, t2: u16) -> Option<Op> {

    let op1 = (t1 >> 11) & 0x03;
    let op = (t2 >> 15) & 0x01;

    if op1 != 0x2 {
        return None;
    }
    if op != 0x01 {
        return None;
    }

    decode_branch_and_misc(t1, t2)
}



#[test]
fn test_is_thumb32() {
    assert!(is_thumb32(0b1110100000000000));
    assert!(is_thumb32(0b1111000000000000));
    assert!(is_thumb32(0b1111100000000000));
    assert!(is_thumb32(0b1110000000000000) == false);
    assert!(is_thumb32(0b1111111111111111));
    assert!(is_thumb32(0b0000000000000001) == false);
}

#[test]
fn test_decode_thumb16() {
    match decode_16(0x4600).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R0);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4608).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R1);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4610).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R2);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4618).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R3);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4620).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R4);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4628).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R5);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4630).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R6);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4638).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R7);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4640).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R8);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4648).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R9);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4650).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R10);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4658).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R11);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4660).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R12);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4668).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::SP);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4670).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::LR);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4678).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::PC);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }

    match decode_16(0x0001).unwrap() {
        Op::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R1);
            assert!(rm == Reg::R0);
            assert!(setflags == true);
        }
        _ => {
            assert!(false);
        }
    }
    //MOVS (mov immediate)
    match decode_16(0x2001).unwrap() {
        Op::MOV_imm { rd, imm32 } => {
            assert!(rd == Reg::R0);
            assert!(imm32 == 1);
        }
        _ => {
            assert!(false);
        }
    }

    //BX LR
    match decode_16(0x4770).unwrap() {
        Op::BX { rm } => {
            assert!(rm == Reg::LR);
        }
        _ => {
            assert!(false);
        }
    }
    //CMP R0, R0
    match decode_16(0x2800).unwrap() {
        Op::CMP_imm { rn, imm32 } => {
            assert!(rn == Reg::R0);
            assert!(imm32 == 0);
        }
        _ => {
            assert!(false);
        }
    }
    // BEQ.N
    match decode_16(0xd001).unwrap() {
        Op::B { cond, imm32 } => {
            assert!(cond == Condition::EQ);
            assert!(imm32 == (1 << 1));
        }
        _ => {
            assert!(false);
        }
    }
    // BNE.N
    match decode_16(0xd1f8).unwrap() {
        Op::B { cond, imm32 } => {
            assert!(cond == Condition::NE);
            assert!(imm32 == -16);
        }
        _ => {
            assert!(false);
        }
    }

    // PUSH  {R4, LR}
    match decode_16(0xb510).unwrap() {
        Op::PUSH { registers } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R4, Reg::LR], elems);
        }
        _ => {
            assert!(false);
        }
    }
    // LDR.N R1, [PC, 0x1c]
    match decode_16(0x4907).unwrap() {
        Op::LDR_lit { rt, imm32 } => {
            assert!(rt == Reg::R1);
            assert!(imm32 == (7 << 2));
        }
        _ => {
            assert!(false);
        }
    }
    // ADD R1,R1, PC
    match decode_16(0x4479).unwrap() {
        Op::ADD { rdn, rm } => {
            assert!(rdn == Reg::R1);
            assert!(rm == Reg::PC);
        }
        _ => {
            assert!(false);
        }
    }
    // B.N (PC + 8)
    match decode_16(0xE004).unwrap() {
        Op::B { cond, imm32 } => {
            assert!(cond == Condition::AL);
            assert!(imm32 == (4 << 1));
        }
        _ => {
            assert!(false);
        }
    }
    // CMP R1, R4
    match decode_16(0x42a1).unwrap() {
        Op::CMP { rn, rm } => {
            assert!(rn == Reg::R1);
            assert!(rm == Reg::R4);
        }
        _ => {
            assert!(false);
        }
    }
    // ADDS R1, R1, 24
    match decode_16(0x3118).unwrap() {
        Op::ADDS_imm { rn, rd, imm32 } => {
            assert!(rn == Reg::R1);
            assert!(rd == Reg::R1);
            assert!(imm32 == 24);
        }
        _ => {
            assert!(false);
        }
    }

    // LDR R2, [R1]
    match decode_16(0x680a).unwrap() {
        Op::LDR_imm { rt, rn, imm32 } => {
            assert!(rn == Reg::R1);
            assert!(rt == Reg::R2);
            assert!(imm32 == 0);
        }
        _ => {
            assert!(false);
        }
    }

}
