use bit_field::BitField;
use core::instruction::Instruction;
use core::ThumbCode;

#[cfg(test)]
use core::register::Reg;

#[cfg(test)]
use core::condition::Condition;

mod and;
mod adc;
mod add;
mod adr;
mod asr;

mod b;
mod bic;
mod blx;
mod bx;
mod bl;
mod bkpt;

mod cmn;
mod cmp;

mod eor;

mod ldm;
mod ldr;
mod ldrb;
mod ldrh;
mod ldrsh;
mod ldrsb;
mod lsl;
mod lsr;
mod mov;
mod mvn;
mod mul;

mod nop;
mod orr;

mod push;
mod pop;

mod rsb;
mod ror;

mod sbc;
mod stm;
mod str;
mod sub;
mod sxt;
mod tst;
mod uxt;

use decoder::adc::*;
use decoder::add::*;
use decoder::adr::*;
use decoder::and::*;
use decoder::asr::*;

use decoder::b::*;
use decoder::bic::*;
use decoder::bx::*;
use decoder::blx::*;
use decoder::bl::*;
use decoder::bkpt::*;

use decoder::cmn::*;
use decoder::cmp::*;

use decoder::eor::*;

use decoder::ldm::*;
use decoder::ldr::*;
use decoder::ldrb::*;
use decoder::ldrh::*;
use decoder::ldrsh::*;
use decoder::ldrsb::*;
use decoder::lsl::*;
use decoder::lsr::*;

use decoder::mov::*;
use decoder::mul::*;
use decoder::mvn::*;

use decoder::nop::*;
use decoder::orr::*;
use decoder::push::*;
use decoder::pop::*;

use decoder::rsb::*;
use decoder::ror::*;
use decoder::sbc::*;
use decoder::sub::*;
use decoder::stm::*;
use decoder::str::*;
use decoder::sxt::*;
use decoder::tst::*;
use decoder::uxt::*;

pub fn is_thumb32(word: u16) -> bool {
    match word.get_bits(11..16) {
        0b11101 | 0b11110 | 0b11111 => true,
        _ => false,
    }
}

pub fn decode_16(command: u16) -> Instruction {
    match command.get_bits(14..16) {
        0b00 => {
            // Shift (immediate), add, substract, move and compare
            match command.get_bits(9..14) {
                0b000_01 | 0b000_10 | 0b000_11 => decode_LSL_imm_t1(command),
                0b001_00 | 0b001_01 | 0b001_10 | 0b001_11 => decode_LSR_imm_t1(command),
                0b010_00 | 0b010_01 | 0b010_10 | 0b010_11 => decode_ASR_imm_t1(command),
                0b011_00 => decode_ADD_reg_t1(command),
                0b011_01 => decode_SUB_reg_t1(command),
                0b011_10 => decode_ADD_imm_t1(command),
                0b011_11 => decode_SUB_imm_t1(command),
                0b100_00 | 0b100_01 | 0b100_10 | 0b100_11 => decode_MOV_imm_t1(command),
                0b101_00 | 0b101_01 | 0b101_10 | 0b101_11 => decode_CMP_imm_t1(command),
                0b110_00 | 0b110_01 | 0b110_10 | 0b110_11 => decode_ADD_imm_t2(command),
                0b111_00 | 0b111_01 | 0b111_10 | 0b111_11 => decode_SUB_imm_t2(command),
                0 => match command.get_bits(6..11) {
                    0 => decode_MOV_reg_t2(command),
                    _ => decode_LSL_imm_t1(command),
                },

                _ => Instruction::UDF {
                    imm32: 0,
                    opcode: ThumbCode::from(command),
                },
            }
        }
        0b01 => {
            // data process, special data, load from lp...
            if !command.get_bit(11) {
                match command.get_bits(13..16) {
                    0b010 => match command.get_bits(6..13) {
                        0b000_0000 => decode_AND_reg_t1(command),
                        0b000_0001 => decode_EOR_reg_t1(command),
                        0b000_0010 => decode_LSL_reg_t1(command),
                        0b000_0011 => decode_LSR_reg_t1(command),
                        0b000_0100 => decode_ASR_reg_t1(command),
                        0b000_0101 => decode_ADC_reg_t1(command),
                        0b000_0110 => decode_SBC_reg_t1(command),
                        0b000_0111 => decode_ROR_reg_t1(command),
                        0b000_1000 => decode_TST_reg_t1(command),
                        0b000_1001 => decode_RSB_imm_t1(command),
                        0b000_1010 => decode_CMP_reg_t1(command),
                        0b000_1011 => decode_CMN_reg_t1(command),
                        0b000_1100 => decode_ORR_reg_t1(command),
                        0b000_1101 => decode_MUL_reg_t1(command),
                        0b000_1110 => decode_BIC_reg_t1(command),
                        0b000_1111 => decode_MVN_reg_t1(command),

                        0b001_0000 | 0b001_0001 | 0b001_0010 | 0b001_0011 => {
                            decode_ADD_reg_t2(command)
                        }

                        0b001_0101 | 0b001_0110 | 0b001_0111 => decode_CMP_reg_t2(command),
                        0b001_1000 | 0b001_1001 | 0b001_1010 | 0b001_1011 => {
                            decode_MOV_reg_t1(command)
                        }
                        0b001_1101 | 0b001_1100 => decode_BX(command),
                        0b001_1110 | 0b001_1111 => decode_BLX(command),
                        0b101_0000 |
                        0b101_0001 |
                        0b101_0010 |
                        0b101_0011 |
                        0b101_0100 |
                        0b101_0101 |
                        0b101_0110 |
                        0b101_0111 => decode_STRB_reg_t1(command),

                        0b101_1000 |
                        0b101_1001 |
                        0b101_1010 |
                        0b101_1011 |
                        0b101_1100 |
                        0b101_1101 |
                        0b101_1110 |
                        0b101_1111 => decode_LDRSB_reg_t1(command),

                        0b100_0000 |
                        0b100_0001 |
                        0b100_0010 |
                        0b100_0011 |
                        0b100_0100 |
                        0b100_0101 |
                        0b100_0110 |
                        0b100_0111 => decode_STR_reg_t1(command),
                        0b100_1000 |
                        0b100_1001 |
                        0b100_1010 |
                        0b100_1011 |
                        0b100_1100 |
                        0b100_1101 |
                        0b100_1110 |
                        0b100_1111 => decode_STRH_reg_t1(command),
                        _ => Instruction::UDF {
                            imm32: 0,
                            opcode: ThumbCode::from(command),
                        },
                    },
                    0b011 => if command.get_bit(12) {
                        decode_STRB_imm_t1(command)
                    } else {
                        decode_STR_imm_t1(command)
                    },
                    _ => Instruction::UDF {
                        imm32: 0,
                        opcode: ThumbCode::from(command),
                    },
                }
            } else {
                match command.get_bits(9..16) {
                    0b0101_100 => decode_LDR_reg_t1(command),
                    0b0101_101 => decode_LDRH_reg_t1(command),
                    0b0101_110 => decode_LDRB_reg_t1(command),
                    0b0101_111 => decode_LDRSH_reg_t1(command),
                    0b0100_010 => decode_ADD_reg_t2(command),
                    _ => match command.get_bits(11..16) {
                        0b01001 => decode_LDR_lit_t1(command),
                        0b01011 => decode_LDR_reg_t1(command),
                        0b01101 => decode_LDR_imm_t1(command),
                        0b01111 => decode_LDRB_imm_t1(command),
                        _ => Instruction::UDF {
                            imm32: 0,
                            opcode: ThumbCode::from(command),
                        },
                    },
                }
            }
        }
        0b10 => {
            // generate pc relative addr, sp rela, misc
            match command.get_bits(11..16) {
                0b10000 => decode_STRH_imm_t1(command),
                0b10001 => decode_LDRH_imm_t1(command),
                0b10010 => decode_STR_imm_t2(command),
                0b10011 => decode_LDR_imm_t2(command),
                0b10100 => decode_ADR_t1(command),
                0b10101 => decode_ADD_SP_imm_t1(command),
                _ => match command.get_bits(7..16) {
                    0b101100000 => decode_ADD_SP_imm_t2(command),
                    0b101100001 => decode_SUB_SP_imm_t1(command),
                    0b101111110 => decode_NOP_t1(command),
                    0b101100100 => if command.get_bit(6) {
                        decode_SXTB_t1(command)
                    } else {
                        decode_SXTH_t1(command)
                    },
                    0b101100101 => if command.get_bit(6) {
                        decode_UXTB_t1(command)
                    } else {
                        decode_UXTH_t1(command)
                    },
                    0b101111100 | 0b101111101 => decode_BKPT_t1(command),
                    _ => match command.get_bits(9..14) {
                        0b11110 => decode_POP(command),
                        0b11010 => decode_PUSH(command),
                        _ => Instruction::UDF {
                            imm32: 0,
                            opcode: ThumbCode::from(command),
                        },
                    },
                },
            }
        }
        0b11 => {
            // store, load multiple, branch, svc, uncond branch
            match command.get_bits(12..16) {
                0b1101 => decode_B_t1(command),
                0b1110 => decode_B_t2(command),
                0b1100 => if command.get_bit(11) {
                    decode_LDM_t1(command)
                } else {
                    decode_STM_t1(command)
                },
                _ => Instruction::UDF {
                    imm32: 0,
                    opcode: ThumbCode::from(command),
                },
            }
        }

        _ => Instruction::UDF {
            imm32: 0,
            opcode: ThumbCode::from(command),
        },
    }
}

//A 5.3.1 Branch and misc (thumb32)
pub fn decode_branch_and_misc(t1: u16, t2: u16) -> Instruction {
    let op2 = (t2 >> 12) & 0x07;

    match op2 {
        0x7 | 0x5 => decode_bl(t1, t2),
        _ => Instruction::UDF {
            imm32: 0,
            opcode: ThumbCode::from(((t1 as u32) << 16) + t2 as u32),
        },
    }
}

// A5.3 check thumb32 encodings
pub fn decode_32(t1: u16, t2: u16) -> Instruction {
    let op1 = (t1 >> 11) & 0x03;
    let op = (t2 >> 15) & 0x01;

    if op1 != 0x2 {
        return Instruction::UDF {
            imm32: 0,
            opcode: ThumbCode::from(((t1 as u32) << 16) + t2 as u32),
        };
    }
    if op != 0x01 {
        return Instruction::UDF {
            imm32: 0,
            opcode: ThumbCode::from(((t1 as u32) << 16) + t2 as u32),
        };
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
fn test_decode_mov() {
    match decode_16(0x4600) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R0);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4608) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R1);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4610) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R2);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4618) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R3);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4620) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R4);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4628) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R5);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4630) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R6);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4638) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R7);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4640) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R8);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4648) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R9);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4650) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R10);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4658) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R11);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4660) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R12);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4668) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::SP);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4670) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::LR);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4678) {
        Instruction::MOV_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::PC);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }

    match decode_16(0x0001) {
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
    match decode_16(0x2001) {
        Instruction::MOV_imm {
            rd,
            imm32,
            setflags,
        } => {
            assert!(rd == Reg::R0);
            assert!(imm32 == 1);
            assert!(setflags);
        }
        _ => {
            assert!(false);
        }
    }
    //MOVS (mov immediate)
    match decode_16(0x2101) {
        Instruction::MOV_imm {
            rd,
            imm32,
            setflags,
        } => {
            assert!(rd == Reg::R1);
            assert!(imm32 == 1);
            assert!(setflags);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_bx() {
    //BX LR
    match decode_16(0x4770) {
        Instruction::BX { rm } => {
            assert!(rm == Reg::LR);
        }
        _ => {
            assert!(false);
        }
    }
    //BX R1
    match decode_16(0x4708) {
        Instruction::BX { rm } => {
            assert!(rm == Reg::R1);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_cmp() {
    //CMP R0, R0
    match decode_16(0x2800) {
        Instruction::CMP_imm { rn, imm32 } => {
            assert!(rn == Reg::R0);
            assert!(imm32 == 0);
        }
        _ => {
            assert!(false);
        }
    }
    // CMP R1, R4
    match decode_16(0x42a1) {
        Instruction::CMP_reg { rn, rm } => {
            assert!(rn == Reg::R1);
            assert!(rm == Reg::R4);
        }
        _ => {
            assert!(false);
        }
    }
    // CMP R2, #0
    match decode_16(0x2a00) {
        Instruction::CMP_imm { rn, imm32 } => {
            assert!(rn == Reg::R2);
            assert!(imm32 == 0);
        }
        _ => {
            assert!(false);
        }
    }
}


#[test]
fn test_decode_b() {
    // BEQ.N
    match decode_16(0xd001) {
        Instruction::B { cond, imm32 } => {
            assert!(cond == Condition::EQ);
            assert!(imm32 == (1 << 1));
        }
        _ => {
            assert!(false);
        }
    }
    // BNE.N
    match decode_16(0xd1f8) {
        Instruction::B { cond, imm32 } => {
            assert!(cond == Condition::NE);
            assert!(imm32 == -16);
        }
        _ => {
            assert!(false);
        }
    }
    // B.N (PC + 8)
    match decode_16(0xE004) {
        Instruction::B { cond, imm32 } => {
            assert!(cond == Condition::AL);
            assert!(imm32 == (4 << 1));
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_push() {
    // PUSH  {R4, LR}
    match decode_16(0xb510) {
        Instruction::PUSH { registers } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R4, Reg::LR], elems);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_pop() {
    // POP  {R4, LR}
    match decode_16(0xbd10) {
        Instruction::POP { registers } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R4, Reg::PC], elems);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_ldr() {
    // LDR.N R1, [PC, 0x1c]
    match decode_16(0x4907) {
        Instruction::LDR_lit { rt, imm32 } => {
            assert!(rt == Reg::R1);
            assert!(imm32 == (7 << 2));
        }
        _ => {
            assert!(false);
        }
    }
    // LDR R2, [R1]
    match decode_16(0x680a) {
        Instruction::LDR_imm { rt, rn, imm32 } => {
            assert!(rn == Reg::R1);
            assert!(rt == Reg::R2);
            assert!(imm32 == 0);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_add_reg_pc() {
    // ADD R1,R1, PC
    match decode_16(0x4479) {
        Instruction::ADD_reg {
            rd,
            rn,
            rm,
            setflags,
        } => {
            assert_eq!(rd, Reg::R1);
            assert_eq!(rn, Reg::R1);
            assert_eq!(rm, Reg::PC);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
}
#[test]
fn test_decode_add_reg_imm() {
    // ADDS R1, R1, 24
    match decode_16(0x3118) {
        Instruction::ADD_imm {
            rn,
            rd,
            imm32,
            setflags,
        } => {
            assert!(rn == Reg::R1);
            assert!(rd == Reg::R1);
            assert!(imm32 == 24);
            assert!(setflags);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_add_reg_sp() {
    // ADD R1, SP, #0xc
    match decode_16(0xa903) {
        Instruction::ADD_imm {
            rn,
            rd,
            imm32,
            setflags,
        } => {
            assert!(rn == Reg::SP);
            assert!(rd == Reg::R1);
            assert!(imm32 == 0xc);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_sub() {
    // SUB SP,SP, #0x8
    match decode_16(0xb082) {
        Instruction::SUB_imm {
            rd,
            rn,
            imm32,
            setflags,
        } => {
            assert!(rd == Reg::SP);
            assert!(rn == Reg::SP);
            assert!(imm32 == 0x8);
            assert!(setflags == false);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_sub2() {
    // SUBS R2,R2,#48
    match decode_16(0x3a30) {
        Instruction::SUB_imm {
            rd,
            rn,
            imm32,
            setflags,
        } => {
            assert!(rd == Reg::R2);
            assert!(rn == Reg::R2);
            assert!(imm32 == 48);
            assert!(setflags == true);
        }
        _ => {
            assert!(false);
        }
    }
}


#[test]
fn test_decode_tst() {
    // TST R4, R1
    match decode_16(0x420c) {
        Instruction::TST_reg { rn, rm } => {
            assert!(rn == Reg::R4);
            assert!(rm == Reg::R1);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_ldrb() {
    // LDRB R0, [R0m 0]
    match decode_16(0x7800) {
        Instruction::LDRB_imm { rt, rn, imm32 } => {
            assert!(rt == Reg::R0);
            assert!(rn == Reg::R0);
            assert!(imm32 == 0);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_ldrb2() {
    // LDRB R2, [R0, 0x10]
    match decode_16(0x7c02) {
        Instruction::LDRB_imm { rt, rn, imm32 } => {
            assert!(rt == Reg::R2);
            assert!(rn == Reg::R0);
            assert!(imm32 == 0x10);
        }
        _ => {
            assert!(false);
        }
    }
}


#[test]
fn test_decode_mvns() {
    // MVNS R5,R5
    match decode_16(0x43ed) {
        Instruction::MVN_reg { rd, rm, setflags } => {
            assert!(rd == Reg::R5);
            assert!(rm == Reg::R5);
            assert!(setflags);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_lsls() {
    // LSLS R1, R4, #2
    match decode_16(0x00a1) {
        Instruction::LSL_imm {
            rd,
            rm,
            imm5,
            setflags,
        } => {
            assert!(rd == Reg::R1);
            assert!(rm == Reg::R4);
            assert!(imm5 == 2);
            assert!(setflags);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_adr() {
    // ADR R0, PC, #(7<<2)
    match decode_16(0xa007) {
        Instruction::ADR { rd, imm32 } => {
            assert!(rd == Reg::R0);
            assert!(imm32 == 7 << 2);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_bkpt() {
    // BKPT #0xab
    match decode_16(0xbeab) {
        Instruction::BKPT { imm32 } => {
            assert!(imm32 == 0xab);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_strb() {
    // STRB R0, [R1]
    match decode_16(0x7008) {
        Instruction::STRB_imm { rt, rn, imm32 } => {
            assert!(rt == Reg::R0);
            assert!(rn == Reg::R1);
            assert!(imm32 == 0x0);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_str_reg() {
    // STR R0, [R1, R2]
    match decode_16(0x5088) {
        Instruction::STR_reg { rt, rn, rm } => {
            assert!(rt == Reg::R0);
            assert!(rn == Reg::R1);
            assert!(rm == Reg::R2);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_nop() {
    // NOP
    match decode_16(0xbf00) {
        Instruction::NOP => {}
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_mul() {
    // MULS R4, R0, R4
    match decode_16(0x4344) {
        Instruction::MUL {
            rd,
            rn,
            rm,
            setflags,
        } => {
            assert!(rd == Reg::R4);
            assert!(rn == Reg::R0);
            assert!(rm == Reg::R4);
            assert!(setflags);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_orr() {
    // ORRS R3, R3, R1
    match decode_16(0x430b) {
        Instruction::ORR {
            rd,
            rn,
            rm,
            setflags,
        } => {
            assert!(rd == Reg::R3);
            assert!(rn == Reg::R3);
            assert!(rm == Reg::R1);
            assert!(setflags);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_lsr_imm() {
    // LSRS R3, R0, #8
    match decode_16(0x0a03) {
        Instruction::LSR_imm {
            rd,
            rm,
            imm5,
            setflags,
        } => {
            assert!(rd == Reg::R3);
            assert!(rm == Reg::R0);
            assert!(imm5 == 8);
            assert!(setflags);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_lsr_reg() {
    // LSRS R1, R1, R4
    match decode_16(0x40e1) {
        Instruction::LSR_reg {
            rd,
            rn,
            rm,
            setflags,
        } => {
            assert_eq!(rd, Reg::R1);
            assert_eq!(rn, Reg::R1);
            assert_eq!(rm, Reg::R4);
            assert!(setflags);
        }
        _ => {
            assert!(false);
        }
    }
}


#[test]
fn test_decode_adc_reg() {
    // ADCS R2,R2,R2
    match decode_16(0x4152) {
        Instruction::ADC_reg {
            rd,
            rm,
            rn,
            setflags,
        } => {
            assert!(rd == Reg::R2);
            assert!(rm == Reg::R2);
            assert!(rn == Reg::R2);
            assert!(setflags);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_asr_imm() {
    // ASR R2,R2,#8
    match decode_16(0x1212) {
        Instruction::ASR_imm {
            rd,
            rm,
            imm5,
            setflags,
        } => {
            assert!(rd == Reg::R2);
            assert!(rm == Reg::R2);
            assert!(imm5 == 8);
            assert!(setflags);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_strh_imm() {
    // STRH R0, [R1, #0x38]
    match decode_16(0x8708) {
        Instruction::STRH_imm { rt, rn, imm32 } => {
            assert!(rt == Reg::R0);
            assert!(rn == Reg::R1);
            assert!(imm32 == 0x38);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_uxtb() {
    // UXTB R1,R1
    match decode_16(0xb2c9) {
        Instruction::UXTB { rd, rm } => {
            assert!(rd == Reg::R1);
            assert!(rm == Reg::R1);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_bic() {
    // BICS R2,R2,R0
    match decode_16(0x4382) {
        Instruction::BIC_reg {
            rd,
            rm,
            rn,
            setflags,
        } => {
            assert!(rd == Reg::R2);
            assert!(rn == Reg::R2);
            assert!(rm == Reg::R0);
            assert!(setflags);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_ldm() {
    // LDM R2!, {R0, R1}
    match decode_16(0xca03) {
        Instruction::LDM { rn, registers } => {
            assert!(rn == Reg::R2);
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R0, Reg::R1], elems);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_ldm2() {
    // LDM R1!, {R3}
    match decode_16(0xc908) {
        Instruction::LDM { rn, registers } => {
            assert!(rn == Reg::R1);
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R3], elems);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_stm() {
    // STM R2!, {R0, R1}
    match decode_16(0xc203) {
        Instruction::STM { rn, registers } => {
            assert!(rn == Reg::R2);
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R0, Reg::R1], elems);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_ldrh() {
    // LDRH R0,[R0, #0x38]
    match decode_16(0x8f00) {
        Instruction::LDRH_imm { rn, rt, imm32 } => {
            assert!(rn == Reg::R0);
            assert!(rt == Reg::R0);
            assert!(imm32 == 0x38);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_and() {
    // ANDS R2,R2,R3
    match decode_16(0x401a) {
        Instruction::AND_reg {
            rd,
            rn,
            rm,
            setflags,
        } => {
            assert!(rd == Reg::R2);
            assert!(rn == Reg::R2);
            assert!(rm == Reg::R3);
            assert!(setflags);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_cmn() {
    // CMN R4,R5
    match decode_16(0x42ec) {
        Instruction::CMN_reg { rn, rm } => {
            assert!(rn == Reg::R4);
            assert!(rm == Reg::R5);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_sbc() {
    // SBCS R5, R5, R3
    match decode_16(0x419d) {
        Instruction::SBC_reg {
            rd,
            rn,
            rm,
            setflags,
        } => {
            assert!(rd == Reg::R5);
            assert!(rn == Reg::R5);
            assert!(rm == Reg::R3);
            assert!(setflags);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_strb2() {
    // STRB R2, [R0, R5]
    match decode_16(0x5542) {
        Instruction::STRB_reg { rt, rn, rm } => {
            assert!(rt == Reg::R2);
            assert!(rn == Reg::R0);
            assert!(rm == Reg::R5);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_ldrsh() {
    // LDRSH R0, [R6, R0]
    match decode_16(0x5e30) {
        Instruction::LDRSH_reg { rt, rn, rm } => {
            assert!(rt == Reg::R0);
            assert!(rn == Reg::R6);
            assert!(rm == Reg::R0);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_strh_reg() {
    // STRH R4, [R6, R1]
    match decode_16(0x5274) {
        Instruction::STRH_reg { rt, rn, rm } => {
            assert_eq!(rt, Reg::R4);
            assert_eq!(rn, Reg::R6);
            assert_eq!(rm, Reg::R1);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_eor_reg() {
    // EOR R0, R0, R4
    match decode_16(0x4060) {
        Instruction::EOR_reg {
            rd,
            rn,
            rm,
            setflags,
        } => {
            assert_eq!(rd, Reg::R0);
            assert_eq!(rn, Reg::R0);
            assert_eq!(rm, Reg::R4);
            assert_eq!(setflags, true);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_ldrsb_reg() {
    // LDRSB R4, [R4, R0]
    match decode_16(0x5624) {
        Instruction::LDRSB_reg { rt, rn, rm } => {
            assert_eq!(rt, Reg::R4);
            assert_eq!(rn, Reg::R4);
            assert_eq!(rm, Reg::R0);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_sxth_reg() {
    // SXTH R1,R1
    match decode_16(0xb209) {
        Instruction::SXTH { rd, rm } => {
            assert_eq!(rd, Reg::R1);
            assert_eq!(rm, Reg::R1);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_rsb_imm() {
    // RSB R2, R0, #0
    match decode_16(0x4242) {
        Instruction::RSB_imm {rd, rn, imm32, setflags} =>{
            assert_eq!(rd, Reg::R2);
            assert_eq!(rn, Reg::R0);
            assert_eq!(imm32, 0);
            assert_eq!(setflags,true);
        }
        _ => {
            assert!(false);
        }
    }
}