use bit_field::BitField;
use core::instruction::Instruction;


#[cfg(test)]
use core::register::Reg;

#[cfg(test)]
use core::condition::Condition;

mod mov;
mod ldr;
mod str;
mod add;
mod bx;
mod cmp;
mod sub;
mod b;
mod blx;
mod bl;
mod push;
mod pop;
mod tst;

use decoder::mov::*;
use decoder::ldr::*;
use decoder::add::*;
use decoder::bx::*;
use decoder::cmp::*;
use decoder::sub::*;
use decoder::b::*;
use decoder::blx::*;
use decoder::bl::*;
use decoder::push::*;
use decoder::pop::*;
use decoder::tst::*;
use decoder::str::*;

pub fn is_thumb32(word: u16) -> bool {
    match word.get_bits(11..16) {
        0b11101 | 0b11110 | 0b11111 => true,
        _ => false,
    }
}

pub fn decode_16(command: u16) -> Option<Instruction> {
    match command.get_bits(14..16) {
        0b00 => {
            // Shift (immediate), add, substract, move and compare
            match command.get_bits(9..14) {
                0b000_01 | 0b000_10 | 0b000_11 => Some(Instruction::LSL_imm),
                0b001_00 | 0b001_01 | 0b001_10 | 0b001_11 => Some(Instruction::LSR_imm),
                0b010_00 | 0b010_01 | 0b010_10 | 0b010_11 => Some(Instruction::ASR),
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
        0b01 => {
            // data process, special data, load from lp...
            if command.get_bit(11) == false {

                match command.get_bits(13..16) {
                    0b010 => {
                        match command.get_bits(6..13) {
                            0b000_0000 => Some(Instruction::AND),
                            0b000_0001 => Some(Instruction::EOR),
                            0b000_0010 => Some(Instruction::LSL_imm),
                            0b000_0011 => Some(Instruction::LSR_imm),
                            0b000_0100 => Some(Instruction::ASR),
                            0b000_0101 => Some(Instruction::ADC),
                            0b000_0110 => Some(Instruction::SBC),
                            0b000_0111 => Some(Instruction::ROR),
                            0b000_1000 => Some(decode_TST_reg_t1(command)),
                            0b000_1001 => Some(Instruction::RSB),
                            0b000_1010 => Some(decode_CMP_t1(command)),
                            0b000_1011 => Some(Instruction::CMN),
                            0b000_1100 => Some(Instruction::ORR),
                            0b000_1101 => Some(Instruction::MUL),
                            0b000_1110 => Some(Instruction::BIC),
                            0b000_1111 => Some(Instruction::MVN_reg),

                            0b001_0000 | 0b001_0001 | 0b001_0010 | 0b001_0011 => {
                                Some(decode_ADD(command))
                            }

                            0b001_0100 => None,
                            0b001_0101 | 0b001_0110 | 0b001_0111 => Some(decode_CMP_t2(command)),
                            0b001_1000 | 0b001_1001 | 0b001_1010 | 0b001_1011 => {
                                Some(decode_MOV_reg_t1(command))
                            }
                            0b001_1101 => Some(decode_BX(command)),
                            0b001_1110 | 0b001_1111 => Some(decode_BLX(command)),
                            _ => None,
                        }
                    }
                    0b011 => Some(decode_STR_imm_t1(command)),
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
        0b10 => {
            // generate pc relative addr, sp rela, misc
            match command.get_bits(11..16) {
                0b10011 => Some(decode_LDR_imm_t2(command)),
                0b10010 => Some(decode_STR_imm_t2(command)),
                _ => {
                    match command.get_bits(9..14) {
                        0b11110 => Some(decode_POP(command)),
                        0b11010 => Some(decode_PUSH(command)),
                        _ => None,
                    }
                }
            }

        }
        0b11 => {
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
pub fn decode_branch_and_misc(t1: u16, t2: u16) -> Option<Instruction> {
    let op1 = (t1 >> 4) & 0x7f;
    let op2 = (t2 >> 12) & 0x07;

    match op2 {
        0x7 | 0x5 => Some(decode_bl(t1, t2)),
        _ => None,
    }
}

// A5.3 check thumb32 encodings
pub fn decode_32(t1: u16, t2: u16) -> Option<Instruction> {

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
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R0);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4608).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R1);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4610).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R2);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4618).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R3);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4620).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R4);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4628).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R5);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4630).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R6);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4638).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R7);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4640).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R8);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4648).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R9);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4650).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R10);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4658).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R11);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4660).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R12);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4668).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::SP);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4670).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::LR);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4678).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::PC);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }

    match decode_16(0x0001).unwrap() {
        Instruction::MOV_reg { rd, rm, setflags } => {
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
        Instruction::MOV_imm { rd, imm32 } => {
            assert!(rd == Reg::R0);
            assert!(imm32 == 1);
        }
        _ => {
            assert!(false);
        }
    }

    //BX LR
    match decode_16(0x4770).unwrap() {
        Instruction::BX { rm } => {
            assert!(rm == Reg::LR);
        }
        _ => {
            assert!(false);
        }
    }
    //CMP R0, R0
    match decode_16(0x2800).unwrap() {
        Instruction::CMP_imm { rn, imm32 } => {
            assert!(rn == Reg::R0);
            assert!(imm32 == 0);
        }
        _ => {
            assert!(false);
        }
    }
    // BEQ.N
    match decode_16(0xd001).unwrap() {
        Instruction::B { cond, imm32 } => {
            assert!(cond == Condition::EQ);
            assert!(imm32 == (1 << 1));
        }
        _ => {
            assert!(false);
        }
    }
    // BNE.N
    match decode_16(0xd1f8).unwrap() {
        Instruction::B { cond, imm32 } => {
            assert!(cond == Condition::NE);
            assert!(imm32 == -16);
        }
        _ => {
            assert!(false);
        }
    }

    // PUSH  {R4, LR}
    match decode_16(0xb510).unwrap() {
        Instruction::PUSH { registers } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R4, Reg::LR], elems);
        }
        _ => {
            assert!(false);
        }
    }
    // LDR.N R1, [PC, 0x1c]
    match decode_16(0x4907).unwrap() {
        Instruction::LDR_lit { rt, imm32 } => {
            assert!(rt == Reg::R1);
            assert!(imm32 == (7 << 2));
        }
        _ => {
            assert!(false);
        }
    }
    // ADD R1,R1, PC
    match decode_16(0x4479).unwrap() {
        Instruction::ADD { rdn, rm } => {
            assert!(rdn == Reg::R1);
            assert!(rm == Reg::PC);
        }
        _ => {
            assert!(false);
        }
    }
    // B.N (PC + 8)
    match decode_16(0xE004).unwrap() {
        Instruction::B { cond, imm32 } => {
            assert!(cond == Condition::AL);
            assert!(imm32 == (4 << 1));
        }
        _ => {
            assert!(false);
        }
    }
    // CMP R1, R4
    match decode_16(0x42a1).unwrap() {
        Instruction::CMP { rn, rm } => {
            assert!(rn == Reg::R1);
            assert!(rm == Reg::R4);
        }
        _ => {
            assert!(false);
        }
    }
    // ADDS R1, R1, 24
    match decode_16(0x3118).unwrap() {
        Instruction::ADDS_imm { rn, rd, imm32 } => {
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
        Instruction::LDR_imm { rt, rn, imm32 } => {
            assert!(rn == Reg::R1);
            assert!(rt == Reg::R2);
            assert!(imm32 == 0);
        }
        _ => {
            assert!(false);
        }
    }

    // TST R4, R1
    match decode_16(0x420c).unwrap() {
        Instruction::TST_reg { rn, rm } => {
            assert!(rn == Reg::R4);
            assert!(rm == Reg::R1);
        }
        _ => {
            assert!(false);
        }
    }

}