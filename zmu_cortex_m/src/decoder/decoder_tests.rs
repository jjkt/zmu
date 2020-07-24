use crate::core::instruction::{
    Imm32Carry, Reg2ImmCarryParams, Reg2ImmParams, Reg2Params, Reg2ShiftNParams,
    Reg2ShiftNoSetFlagsParams, Reg2ShiftParams, Reg2UsizeParams, Reg3HighParams,
    Reg3NoSetFlagsParams, Reg3Params, Reg3ShiftParams, Reg3UsizeParams, Reg4HighParams,
    Reg4NoSetFlagsParams, Reg643232Params, RegImmCarryNoSetFlagsParams, RegImmCarryParams,
    RegImmParams, SRType, SetFlags,
};

use super::*;
use crate::core::register::{DoubleReg, ExtensionReg, Reg};

#[test]
fn test_is_thumb32() {
    assert!(is_thumb32(0b1110100000000000));
    assert!(is_thumb32(0b1111000000000000));
    assert!(is_thumb32(0b1111100000000000));
    assert_eq!(is_thumb32(0b1110000000000000), false);
    assert!(is_thumb32(0b1111111111111111));
    assert_eq!(is_thumb32(0b0000000000000001), false);
}

#[test]
fn test_decode_mov() {
    match decode_16(0x4600) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R0);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4608) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R1);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4610) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R2);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4618) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R3);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4620) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R4);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4628) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R5);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4630) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R6);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4638) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R7);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4640) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R8);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4648) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R9);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4650) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R10);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4658) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R11);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4660) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::R12);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4668) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::SP);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4670) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::LR);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4678) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R0);
            assert!(params.rm == Reg::PC);
            assert!(params.setflags == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }

    match decode_16(0x0001) {
        Instruction::MOV_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R1);
            assert!(params.rm == Reg::R0);
            assert!(params.setflags == true);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    //MOVS (mov immediate)
    assert_eq!(
        decode_16(0x2001),
        Instruction::MOV_imm {
            params: RegImmCarryParams {
                rd: Reg::R0,
                imm32: Imm32Carry::NoCarry { imm32: 1 },
                setflags: SetFlags::NotInITBlock,
            },
            thumb32: false,
        }
    );

    assert_eq!(
        decode_16(0x2101),
        Instruction::MOV_imm {
            params: RegImmCarryParams {
                rd: Reg::R1,
                imm32: Imm32Carry::NoCarry { imm32: 1 },
                setflags: SetFlags::NotInITBlock,
            },
            thumb32: false,
        }
    );
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
    assert_eq!(
        decode_16(0x2800),
        Instruction::CMP_imm {
            params: RegImmParams {
                r: Reg::R0,
                imm32: 0,
            },
            thumb32: false,
        }
    );
    // CMP R1, R4
    match decode_16(0x42a1) {
        Instruction::CMP_reg { params, thumb32 } => {
            assert!(params.rn == Reg::R1);
            assert!(params.rm == Reg::R4);
            assert!(params.shift_t == SRType::LSL);
            assert!(params.shift_n == 0);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
    // CMP R2, #0
    assert_eq!(
        decode_16(0x2a00),
        Instruction::CMP_imm {
            params: RegImmParams {
                r: Reg::R2,
                imm32: 0,
            },
            thumb32: false,
        }
    );
    // CMP LR, R4
    assert_eq!(
        decode_16(0x45A6),
        Instruction::CMP_reg {
            params: Reg2ShiftNoSetFlagsParams {
                rn: Reg::LR,
                rm: Reg::R4,
                shift_t: SRType::LSL,
                shift_n: 0,
            },
            thumb32: false
        }
    );
}

#[test]
fn test_decode_b() {
    // BEQ.N
    match decode_16(0xd001) {
        Instruction::B_t13 {
            cond,
            imm32,
            thumb32,
        } => {
            assert_eq!(cond, Condition::EQ);
            assert_eq!(imm32, (1 << 1));
            assert_eq!(thumb32, false);
        }
        _ => {
            println!(" {}", decode_16(0xd001));
            assert!(false);
        }
    }
    // BNE.N
    match decode_16(0xd1f8) {
        Instruction::B_t13 {
            cond,
            imm32,
            thumb32,
        } => {
            assert!(cond == Condition::NE);
            assert!(imm32 == -16);
            assert_eq!(thumb32, false);
        }
        _ => {
            assert!(false);
        }
    }
    // B.N (PC + 8)
    match decode_16(0xE004) {
        Instruction::B_t24 { imm32, thumb32 } => {
            assert!(imm32 == (4 << 1));
            assert_eq!(thumb32, false);
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
        Instruction::PUSH { registers, thumb32 } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R4, Reg::LR], elems);
            assert_eq!(thumb32, false);
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
        Instruction::POP { registers, thumb32 } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R4, Reg::PC], elems);
            assert_eq!(thumb32, false);
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
        Instruction::LDR_lit {
            rt,
            imm32,
            thumb32,
            add,
        } => {
            assert!(rt == Reg::R1);
            assert!(imm32 == (7 << 2));
            assert!(thumb32 == false);
            assert!(add);
        }
        _ => {
            assert!(false);
        }
    }
    // LDR R2, [R1]
    match decode_16(0x680a) {
        Instruction::LDR_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            assert!(rn == Reg::R1);
            assert!(rt == Reg::R2);
            assert!(imm32 == 0);
            assert!(index == true);
            assert!(add == true);
            assert!(wback == false);
            assert!(thumb32 == false);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_add_reg_pc() {
    // ADD R1,R1, PC
    assert_eq!(
        decode_16(0x4479),
        Instruction::ADD_reg {
            params: Reg3ShiftParams {
                rd: Reg::R1,
                rn: Reg::R1,
                rm: Reg::PC,
                setflags: SetFlags::False,
                shift_t: SRType::LSL,
                shift_n: 0,
            },
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_add_reg_imm() {
    // ADDS R1, R1, 24
    match decode_16(0x3118) {
        Instruction::ADD_imm { params, thumb32 } => {
            assert!(params.rn == Reg::R1);
            assert!(params.rd == Reg::R1);
            assert!(params.imm32 == 24);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(thumb32 == false);
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
        Instruction::ADD_imm { params, thumb32 } => {
            assert!(params.rn == Reg::SP);
            assert!(params.rd == Reg::R1);
            assert!(params.imm32 == 0xc);
            assert!(params.setflags == SetFlags::False);
            assert!(thumb32 == false);
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
        Instruction::SUB_imm { params, thumb32 } => {
            assert!(params.rd == Reg::SP);
            assert!(params.rn == Reg::SP);
            assert!(params.imm32 == 0x8);
            assert!(params.setflags == SetFlags::False);
            assert!(thumb32 == false);
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
        Instruction::SUB_imm { params, thumb32 } => {
            assert!(params.rd == Reg::R2);
            assert!(params.rn == Reg::R2);
            assert!(params.imm32 == 48);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(thumb32 == false);
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
        Instruction::TST_reg { params, thumb32 } => {
            assert!(params.rn == Reg::R4);
            assert!(params.rm == Reg::R1);
            assert!(params.shift_t == SRType::LSL);
            assert!(params.shift_n == 0);
            assert!(!thumb32);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_ldrb_imm() {
    // LDRB R0, [R0m 0]
    assert_eq!(
        decode_16(0x7800),
        Instruction::LDRB_imm {
            rt: Reg::R0,
            rn: Reg::R0,
            imm32: 0,
            index: true,
            add: true,
            wback: false,
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_ldrb_imm2() {
    // LDRB R2, [R0, 0x10]
    assert_eq!(
        decode_16(0x7c02),
        Instruction::LDRB_imm {
            rt: Reg::R2,
            rn: Reg::R0,
            imm32: 0x10,
            index: true,
            add: true,
            wback: false,
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_mvns() {
    // MVNS R5,R5
    match decode_16(0x43ed) {
        Instruction::MVN_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R5);
            assert!(params.rm == Reg::R5);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(params.shift_t == SRType::LSL);
            assert!(params.shift_n == 0);
            assert!(!thumb32);
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
        Instruction::LSL_imm { params, thumb32 } => {
            assert!(params.rd == Reg::R1);
            assert!(params.rm == Reg::R4);
            assert!(params.shift_n == 2);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(thumb32 == false);
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
        Instruction::ADR { params, thumb32 } => {
            assert!(params.r == Reg::R0);
            assert!(params.imm32 == 7 << 2);
            assert!(!thumb32);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_bkpt() {
    // BKPT #0xab
    assert_eq!(decode_16(0xbeab), Instruction::BKPT { imm32: 0xab });
}

#[test]
fn test_decode_strb() {
    // STRB R0, [R1]
    assert_eq!(
        decode_16(0x7008),
        Instruction::STRB_imm {
            rt: Reg::R0,
            rn: Reg::R1,
            imm32: 0,
            index: true,
            add: true,
            wback: false,
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_str_reg() {
    // STR R0, [R1, R2]
    assert_eq!(
        decode_16(0x5088),
        Instruction::STR_reg {
            rt: Reg::R0,
            rn: Reg::R1,
            rm: Reg::R2,
            shift_t: SRType::LSL,
            shift_n: 0,
            index: true,
            add: true,
            wback: false,
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_nop() {
    // NOP
    match decode_16(0xbf00) {
        Instruction::NOP { thumb32 } => {
            assert_eq!(thumb32, false);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_mul() {
    // MULS R4, R0, R4
    match decode_16(0x4344) {
        Instruction::MUL { params, thumb32 } => {
            assert!(params.rd == Reg::R4);
            assert!(params.rn == Reg::R0);
            assert!(params.rm == Reg::R4);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(!thumb32);
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
        Instruction::ORR_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R3);
            assert!(params.rn == Reg::R3);
            assert!(params.rm == Reg::R1);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert_eq!(thumb32, false);
            assert_eq!(params.shift_t, SRType::LSL);
            assert_eq!(params.shift_n, 0);
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
        Instruction::LSR_imm { params, thumb32 } => {
            assert!(params.rd == Reg::R3);
            assert!(params.rm == Reg::R0);
            assert!(params.shift_n == 8);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(thumb32 == false);
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
        Instruction::LSR_reg { params, thumb32 } => {
            assert_eq!(params.rd, Reg::R1);
            assert_eq!(params.rn, Reg::R1);
            assert_eq!(params.rm, Reg::R4);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(!thumb32);
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
        Instruction::ADC_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R2);
            assert!(params.rm == Reg::R2);
            assert!(params.rn == Reg::R2);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(params.shift_t == SRType::LSL);
            assert!(params.shift_n == 0);
            assert!(!thumb32);
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
        Instruction::ASR_imm { params, thumb32 } => {
            assert!(params.rd == Reg::R2);
            assert!(params.rm == Reg::R2);
            assert!(params.shift_n == 8);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(!thumb32);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_strh_imm() {
    // STRH R0, [R1, #0x38]
    assert_eq!(
        decode_16(0x8708),
        Instruction::STRH_imm {
            rt: Reg::R0,
            rn: Reg::R1,
            imm32: 0x38,
            thumb32: false,
            index: true,
            add: true,
            wback: false,
        }
    );
}

#[test]
fn test_decode_uxtb() {
    // UXTB R1,R1
    match decode_16(0xb2c9) {
        Instruction::UXTB { params, thumb32 } => {
            assert!(params.rd == Reg::R1);
            assert!(params.rm == Reg::R1);
            assert!(!thumb32);
            assert!(params.rotation == 0);
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
        Instruction::BIC_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R2);
            assert!(params.rn == Reg::R2);
            assert!(params.rm == Reg::R0);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert_eq!(thumb32, false);
            assert_eq!(params.shift_t, SRType::LSL);
            assert_eq!(params.shift_n, 0);
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
        Instruction::LDM {
            rn,
            registers,
            thumb32,
        } => {
            assert!(rn == Reg::R2);
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R0, Reg::R1], elems);
            assert!(!thumb32);
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
        Instruction::LDM {
            rn,
            registers,
            thumb32,
        } => {
            assert!(rn == Reg::R1);
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R3], elems);
            assert!(!thumb32);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_ldm3() {
    // LDM R4!, {R0-R2}
    match decode_16(0xcc07) {
        Instruction::LDM {
            rn,
            registers,
            thumb32,
        } => {
            assert!(rn == Reg::R4);
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R0, Reg::R1, Reg::R2], elems);
            assert!(!thumb32);
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
        Instruction::STM {
            rn,
            registers,
            wback,
            thumb32,
        } => {
            assert!(rn == Reg::R2);
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R0, Reg::R1], elems);
            assert!(wback);
            assert!(!thumb32);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_stm2() {
    // STM R3!, {R0-R2}
    match decode_16(0xc307) {
        Instruction::STM {
            rn,
            registers,
            wback,
            thumb32,
        } => {
            assert!(rn == Reg::R3);
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R0, Reg::R1, Reg::R2], elems);
            assert!(wback);
            assert!(!thumb32);
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
        Instruction::LDRH_imm {
            rn,
            rt,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            assert!(rn == Reg::R0);
            assert!(rt == Reg::R0);
            assert!(imm32 == 0x38);
            assert!(index);
            assert!(add);
            assert!(!wback);
            assert!(!thumb32);
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
        Instruction::AND_reg { params, thumb32 } => {
            assert_eq!(params.rd, Reg::R2);
            assert_eq!(params.rn, Reg::R2);
            assert_eq!(params.rm, Reg::R3);
            assert_eq!(thumb32, false);
            assert_eq!(params.shift_t, SRType::LSL);
            assert_eq!(params.shift_n, 0);
            assert!(params.setflags == SetFlags::NotInITBlock);
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
        Instruction::CMN_reg { params, thumb32 } => {
            assert!(params.rn == Reg::R4);
            assert!(params.rm == Reg::R5);
            assert!(params.shift_t == SRType::LSL);
            assert!(params.shift_n == 0);
            assert!(!thumb32);
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
        Instruction::SBC_reg { params, thumb32 } => {
            assert!(params.rd == Reg::R5);
            assert!(params.rn == Reg::R5);
            assert!(params.rm == Reg::R3);
            assert!(params.setflags == SetFlags::NotInITBlock);
            assert!(params.shift_t == SRType::LSL);
            assert!(params.shift_n == 0);
            assert!(!thumb32);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_strb2() {
    // STRB R2, [R0, R5]
    assert_eq!(
        decode_16(0x5542),
        Instruction::STRB_reg {
            rt: Reg::R2,
            rn: Reg::R0,
            rm: Reg::R5,
            index: true,
            add: true,
            wback: false,
            thumb32: false,
            shift_n: 0,
            shift_t: SRType::LSL,
        }
    );
}

#[test]
fn test_decode_ldrsh() {
    // LDRSH R0, [R6, R0]
    assert_eq!(
        decode_16(0x5e30),
        Instruction::LDRSH_reg {
            rt: Reg::R0,
            rn: Reg::R6,
            rm: Reg::R0,
            shift_t: SRType::LSL,
            shift_n: 0,
            index: true,
            add: true,
            wback: false,
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_strh_reg() {
    // STRH R4, [R6, R1]
    assert_eq!(
        decode_16(0x5274),
        Instruction::STRH_reg {
            rt: Reg::R4,
            rn: Reg::R6,
            rm: Reg::R1,
            index: true,
            add: true,
            wback: false,
            thumb32: false,
            shift_n: 0,
            shift_t: SRType::LSL,
        }
    );
}

#[test]
fn test_decode_eor_reg() {
    // EOR R0, R0, R4
    match decode_16(0x4060) {
        Instruction::EOR_reg { params, thumb32 } => {
            assert_eq!(params.rd, Reg::R0);
            assert_eq!(params.rn, Reg::R0);
            assert_eq!(params.rm, Reg::R4);
            assert_eq!(params.setflags, SetFlags::NotInITBlock);
            assert_eq!(thumb32, false);
            assert_eq!(params.shift_t, SRType::LSL);
            assert_eq!(params.shift_n, 0);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_ldrsb_reg() {
    // LDRSB R4, [R4, R0]
    assert_eq!(
        decode_16(0x5624),
        Instruction::LDRSB_reg {
            rt: Reg::R4,
            rn: Reg::R4,
            rm: Reg::R0,
            shift_t: SRType::LSL,
            shift_n: 0,
            index: true,
            add: true,
            wback: false,
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_sxth_reg() {
    // SXTH R1,R1
    match decode_16(0xb209) {
        Instruction::SXTH { params, thumb32 } => {
            assert_eq!(params.rd, Reg::R1);
            assert_eq!(params.rm, Reg::R1);
            assert_eq!(thumb32, false);
            assert_eq!(params.rotation, 0);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_rsb_imm() {
    // RSB R2, R0, #0
    assert_eq!(
        decode_16(0x4242),
        Instruction::RSB_imm {
            params: Reg2ImmParams {
                rd: Reg::R2,
                rn: Reg::R0,
                imm32: 0,
                setflags: SetFlags::NotInITBlock,
            },
            thumb32: false
        }
    );
}

/*#[test]
fn test_decode_mrs() {
    // MRS R0, ipsr
    assert_eq!(
        decode_32(0xf3ef8005),
        Instruction::MRS {
            rd: Reg::R0,
            spec_reg: SpecialReg::IPSR,
        }
    );
}*/

#[cfg(any(armv6m))]
#[test]
fn test_decode_cpsid() {
    // CPSID i
    assert_eq!(decode_16(0xB672), Instruction::CPS { im: true });
}

#[cfg(any(armv7m, armv7em))]
#[test]
fn test_decode_cpsid() {
    // CPSID i
    assert_eq!(
        decode_16(0xB672),
        Instruction::CPS {
            im: true,
            affect_pri: true,
            affect_fault: false
        }
    );
}

#[test]
fn test_decode_lsl_2() {
    // LSL r1, r1, #31
    assert_eq!(
        decode_16(0x07c9),
        Instruction::LSL_imm {
            params: Reg2ShiftNParams {
                rd: Reg::R1,
                rm: Reg::R1,
                shift_n: 31,
                setflags: SetFlags::NotInITBlock,
            },
            thumb32: false,
        }
    );
}

#[test]
fn test_decode_bl_t1() {
    // BL -130
    assert_eq!(decode_32(0xf7ffffbf), Instruction::BL { imm32: -130 });

    // BL -5694
    assert_eq!(decode_32(0xf7fefce1), Instruction::BL { imm32: -5694 });
}

#[test]
fn test_decode_ldrw_imm() {
    // LDR.W R1, [R0], #0x4
    assert_eq!(
        decode_32(0xf8501b04),
        Instruction::LDR_imm {
            rt: Reg::R1,
            rn: Reg::R0,
            imm32: 0x4,
            index: false,
            add: true,
            wback: true,
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_strw_imm() {
    // STR.W R4, [R3], #0x4
    assert_eq!(
        decode_32(0xf8434b04),
        Instruction::STR_imm {
            rt: Reg::R4,
            rn: Reg::R3,
            imm32: 4,
            index: false,
            add: true,
            wback: true,
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_cbz() {
    // CBZ R1, 0x3be4 (executed on addr 0x3bc2)
    assert_eq!(
        decode_16(0xb179),
        Instruction::CBZ {
            rn: Reg::R1,
            imm32: 30,
            nonzero: false,
        }
    );
}

#[test]
fn test_decode_it() {
    // ITT MI
    assert_eq!(
        decode_16(0xbf44),
        Instruction::IT {
            x: Some(ITCondition::Then),
            y: None,
            z: None,
            firstcond: Condition::MI,
            mask: 0x4,
        }
    );
}

#[test]
fn test_decode_itttt_cc() {
    // 0xbf3f ITTTT CC
    assert_eq!(
        decode_16(0xbf3f),
        Instruction::IT {
            x: Some(ITCondition::Then),
            y: Some(ITCondition::Then),
            z: Some(ITCondition::Then),
            firstcond: Condition::CC,
            mask: 0b1111,
        }
    );
}

#[test]
fn test_decode_itt_cc() {
    // 0xbf3c ITTCC
    assert_eq!(
        decode_16(0xbf3c),
        Instruction::IT {
            x: Some(ITCondition::Then),
            y: None,
            z: None,
            firstcond: Condition::CC,
            mask: 0b1100,
        }
    );
}

#[test]
fn test_decode_pushw() {
    // PUSH.W {R4-R11, LR}
    // PUSH  {R4, LR}
    match decode_32(0xe92d4ff0) {
        Instruction::PUSH { registers, thumb32 } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(
                vec![
                    Reg::R4,
                    Reg::R5,
                    Reg::R6,
                    Reg::R7,
                    Reg::R8,
                    Reg::R9,
                    Reg::R10,
                    Reg::R11,
                    Reg::LR,
                ],
                elems
            );

            assert_eq!(thumb32, true);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_subw_imm() {
    // SUBW SP,SP,#2084
    assert_eq!(
        decode_32(0xf6ad0d24),
        Instruction::SUB_imm {
            params: Reg2ImmParams {
                rd: Reg::SP,
                rn: Reg::SP,
                imm32: 2084,
                setflags: SetFlags::False,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_tbb() {
    // TBB [PC, R0]
    assert_eq!(
        decode_32(0xe8dff000),
        Instruction::TBB {
            rn: Reg::PC,
            rm: Reg::R0,
        }
    );
}

#[test]
fn test_decode_strh_w() {
    // STRH.W R0, [SP, #0x10]
    assert_eq!(
        decode_32(0xf8ad0010),
        Instruction::STRH_imm {
            rt: Reg::R0,
            rn: Reg::SP,
            imm32: 0x10,
            thumb32: true,
            index: true,
            add: true,
            wback: false,
        }
    );
}

#[test]
fn test_decode_strh_w_2() {
    // 0xf8a87000 -> STRH.W R7, [R8]
    assert_eq!(
        decode_32(0xf8a87000),
        Instruction::STRH_imm {
            rt: Reg::R7,
            rn: Reg::R8,
            imm32: 0x0,
            thumb32: true,
            index: true,
            add: true,
            wback: false,
        }
    );
}

#[test]
fn test_decode_mov_w() {
    // MOV.W R8, #-1
    assert_eq!(
        decode_32(0xf04f38ff),
        Instruction::MOV_imm {
            params: RegImmCarryParams {
                rd: Reg::R8,
                imm32: Imm32Carry::Carry {
                    imm32_c0: (0xffffffff, false),
                    imm32_c1: (0xffffffff, true),
                },
                setflags: SetFlags::False,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ldrsh_reg_w() {
    // LDRSH.W R0, [R0, R0, LSL #0]
    assert_eq!(
        decode_32(0xf9300000),
        Instruction::LDRSH_reg {
            rt: Reg::R0,
            rn: Reg::R0,
            rm: Reg::R0,
            shift_t: SRType::LSL,
            shift_n: 0,
            index: true,
            add: true,
            wback: false,
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ldrsh_imm_w() {
    // LDRSH.W R0, [SP, #0x10]
    assert_eq!(
        decode_32(0xf9bd0010),
        Instruction::LDRSH_imm {
            rt: Reg::R0,
            rn: Reg::SP,
            imm32: 0x10,
            index: true,
            add: true,
            wback: false,
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ubfx() {
    // UBFX R1, R0, #1, #1
    assert_eq!(
        decode_32(0xf3c00140),
        Instruction::UBFX {
            rd: Reg::R1,
            rn: Reg::R0,
            lsb: 1,
            widthminus1: 0,
        }
    );
}

#[test]
fn test_decode_udiv() {
    // UDIV R0, R0, R1
    assert_eq!(
        decode_32(0xfbb0f0f1),
        Instruction::UDIV {
            params: Reg3NoSetFlagsParams {
                rd: Reg::R0,
                rn: Reg::R0,
                rm: Reg::R1,
            }
        }
    );
}

#[test]
fn test_decode_mla() {
    // MLA R1, R7, R2, R1
    assert_eq!(
        decode_32(0xfb071102),
        Instruction::MLA {
            params: Reg4NoSetFlagsParams {
                rd: Reg::R1,
                rn: Reg::R7,
                rm: Reg::R2,
                ra: Reg::R1,
            }
        }
    );
}

#[test]
fn test_decode_ldrb_w() {
    // 0xf8960020 LDRB.W R0 [R6, #0x20]
    assert_eq!(
        decode_32(0xf8960020),
        Instruction::LDRB_imm {
            rt: Reg::R0,
            rn: Reg::R6,
            imm32: 0x20,
            index: true,
            add: true,
            wback: false,
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_add_reg_w() {
    // 0xeb0103ca ADD.W R3, R1, R10, LSL #3
    assert_eq!(
        decode_32(0xeb0103ca),
        Instruction::ADD_reg {
            params: Reg3ShiftParams {
                rd: Reg::R3,
                rn: Reg::R1,
                rm: Reg::R10,
                setflags: SetFlags::False,
                shift_t: SRType::LSL,
                shift_n: 3,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_cmp_imm_w() {
    // 0xf1ba0f00 CMP.W R10, #0
    assert_eq!(
        decode_32(0xf1ba0f00),
        Instruction::CMP_imm {
            params: RegImmParams {
                r: Reg::R10,
                imm32: 0,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_and_imm_w() {
    // 0xf01a0c03 ANDS.W R12, R10, 3
    assert_eq!(
        decode_32(0xf01a0c03),
        Instruction::AND_imm {
            params: Reg2ImmCarryParams {
                rd: Reg::R12,
                rn: Reg::R10,
                imm32: Imm32Carry::Carry {
                    imm32_c0: (3, false),
                    imm32_c1: (3, true),
                },
                setflags: true,
            }
        }
    );
}

#[test]
fn test_decode_eor_reg_w() {
    // 0xea8e0402 EOR.W R4, LR, R2
    assert_eq!(
        decode_32(0xea8e0402),
        Instruction::EOR_reg {
            params: Reg3ShiftParams {
                rd: Reg::R4,
                rn: Reg::LR,
                rm: Reg::R2,
                setflags: SetFlags::False,
                shift_t: SRType::LSL,
                shift_n: 0,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_orr_reg_w() {
    // 0xea4404c8  ORR.W R4, R4, R8, LSL #3
    assert_eq!(
        decode_32(0xea4404c8),
        Instruction::ORR_reg {
            params: Reg3ShiftParams {
                rd: Reg::R4,
                rn: Reg::R4,
                rm: Reg::R8,
                setflags: SetFlags::False,
                shift_t: SRType::LSL,
                shift_n: 3,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_lsl_w_imm() {
    // LSL.W R8,R8,1
    assert_eq!(
        decode_32(0xea4f0848),
        Instruction::LSL_imm {
            params: Reg2ShiftNParams {
                rd: Reg::R8,
                rm: Reg::R8,
                shift_n: 1,
                setflags: SetFlags::False,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_lsr_w_imm() {
    // LSRS.W R12,R10,2
    assert_eq!(
        decode_32(0xea5f0c9a),
        Instruction::LSR_imm {
            params: Reg2ShiftNParams {
                rd: Reg::R12,
                rm: Reg::R10,
                shift_n: 2,
                setflags: SetFlags::True,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_pop_w() {
    //0xe8bd47f0 POP.W {R4-R10, LR}
    match decode_32(0xe8bd47f0) {
        Instruction::POP { registers, thumb32 } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(
                vec![
                    Reg::R4,
                    Reg::R5,
                    Reg::R6,
                    Reg::R7,
                    Reg::R8,
                    Reg::R9,
                    Reg::R10,
                    Reg::LR
                ],
                elems
            );
            assert_eq!(thumb32, true);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_mul_w() {
    //0xfb04f604 MUL R6, R4, R4
    assert_eq!(
        decode_32(0xfb04f604),
        Instruction::MUL {
            params: Reg3Params {
                rd: Reg::R6,
                rn: Reg::R4,
                rm: Reg::R4,
                setflags: SetFlags::False,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_asr_w() {
    //0xEA4f39e2 ASR.W R9, R2, #15
    assert_eq!(
        decode_32(0xea4f39e2),
        Instruction::ASR_imm {
            params: Reg2ShiftNParams {
                rd: Reg::R9,
                rm: Reg::R2,
                shift_n: 15,
                setflags: SetFlags::False,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ldrh_w() {
    //0xf8349b02 LDRH.W R9, [R4], #0x2
    assert_eq!(
        decode_32(0xf8349b02),
        Instruction::LDRH_imm {
            rt: Reg::R9,
            rn: Reg::R4,
            imm32: 2,
            thumb32: true,
            add: true,
            index: false,
            wback: true,
        }
    );
}

#[test]
fn test_decode_uxtb_w() {
    //0xfa5ff989 UXTB.W R9, R9
    assert_eq!(
        decode_32(0xfa5ff989),
        Instruction::UXTB {
            params: Reg2UsizeParams {
                rd: Reg::R9,
                rm: Reg::R9,
                rotation: 0,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ldr_lit_w() {
    //0xf8df90cc LDR.W R9, [PC, #0xcc]
    assert_eq!(
        decode_32(0xf8df90cc),
        Instruction::LDR_lit {
            rt: Reg::R9,
            imm32: 0xcc,
            add: true,
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ldr_reg_w() {
    //0xf8594024 LDR.W R4, [R9,R4, LSL #2]
    assert_eq!(
        decode_32(0xf8594024),
        Instruction::LDR_reg {
            rt: Reg::R4,
            rn: Reg::R9,
            rm: Reg::R4,
            shift_t: SRType::LSL,
            shift_n: 2,
            index: true,
            add: true,
            wback: false,
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_strb_imm_w() {
    //0xf80eab01 STRB.W R10, [LR], #1
    assert_eq!(
        decode_32(0xf80eab01),
        Instruction::STRB_imm {
            rt: Reg::R10,
            rn: Reg::LR,
            imm32: 1,
            index: false,
            add: true,
            wback: true,
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_strb_reg_w() {
    //0xf80ce007 STRB.W LR, [R12, R7]
    assert_eq!(
        decode_32(0xf80ce007),
        Instruction::STRB_reg {
            rt: Reg::LR,
            rn: Reg::R12,
            rm: Reg::R7,
            index: true,
            add: true,
            wback: false,
            thumb32: true,
            shift_n: 0,
            shift_t: SRType::LSL,
        }
    );
}

#[test]
fn test_decode_mov_reg_w() {
    // MOV.W R8, R3
    assert_eq!(
        decode_32(0xea4f0803),
        Instruction::MOV_reg {
            params: Reg2Params {
                rd: Reg::R8,
                rm: Reg::R3,
                setflags: false,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_sxth_w() {
    // SXTH.W R10, R10
    assert_eq!(
        decode_32(0xfa0ffa8a),
        Instruction::SXTH {
            params: Reg2UsizeParams {
                rd: Reg::R10,
                rm: Reg::R10,
                rotation: 0
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_adds_w() {
    // 0xf1180801 ADDS.W R8, R8, #1
    assert_eq!(
        decode_32(0xf1180801),
        Instruction::ADD_imm {
            params: Reg2ImmParams {
                rn: Reg::R8,
                rd: Reg::R8,
                imm32: 1,
                setflags: SetFlags::True
            },
            thumb32: true
        }
    );
}

#[test]
fn test_decode_bfi_w() {
    // 0xf3630407 BFI R4, R3, #0, #8
    assert_eq!(
        decode_32(0xf3630407),
        Instruction::BFI {
            rd: Reg::R4,
            rn: Reg::R3,
            lsbit: 0,
            width: 8,
        }
    );
}

#[test]
fn test_decode_sdiv() {
    // 0xfb99f2fa SDIV, R2, R9, R10
    assert_eq!(
        decode_32(0xfb99f2fa),
        Instruction::SDIV {
            params: Reg3NoSetFlagsParams {
                rd: Reg::R2,
                rn: Reg::R9,
                rm: Reg::R10,
            }
        }
    );
}

#[test]
fn test_decode_mls() {
    // 0xfb02921a MLS R2, R2, R10, R9
    assert_eq!(
        decode_32(0xfb02921a),
        Instruction::MLS {
            params: Reg4NoSetFlagsParams {
                rd: Reg::R2,
                rn: Reg::R2,
                rm: Reg::R10,
                ra: Reg::R9,
            }
        }
    );
}

#[test]
fn test_decode_strh_reg_w() {
    //  STRH.W  R12, [R6, R9, LSL #1]
    assert_eq!(
        decode_32(0xf826c019),
        Instruction::STRH_reg {
            rt: Reg::R12,
            rn: Reg::R6,
            rm: Reg::R9,
            index: true,
            add: true,
            wback: false,
            thumb32: true,
            shift_n: 1,
            shift_t: SRType::LSL,
        }
    );
}

#[test]
fn test_decode_cmn_w_reg() {
    // CMN.W R12, R1, LSL #1
    assert_eq!(
        decode_32(0xeb1c0f41),
        Instruction::CMN_reg {
            params: Reg2ShiftNoSetFlagsParams {
                rn: Reg::R12,
                rm: Reg::R1,
                shift_n: 1,
                shift_t: SRType::LSL,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_subw_reg() {
    // 0xebb00b09
    // SUBS.W R11, R0, R9
    assert_eq!(
        decode_32(0xebb00b09),
        Instruction::SUB_reg {
            params: Reg3ShiftParams {
                rd: Reg::R11,
                rn: Reg::R0,
                rm: Reg::R9,
                setflags: SetFlags::True,
                shift_t: SRType::LSL,
                shift_n: 0,
            },
            thumb32: true,
        }
    );

    // 0xEBA45613
    // SUB.W R6, R4, R3, LSR #20
    assert_eq!(
        decode_32(0xEBA45613),
        Instruction::SUB_reg {
            params: Reg3ShiftParams {
                rd: Reg::R6,
                rn: Reg::R4,
                rm: Reg::R3,
                setflags: SetFlags::False,
                shift_t: SRType::LSR,
                shift_n: 20,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_str_reg_w() {
    // 0xf841002a
    // STR.W R0, [R1, R10, LSL #2]
    assert_eq!(
        decode_32(0xf841002a),
        Instruction::STR_reg {
            rt: Reg::R0,
            rn: Reg::R1,
            rm: Reg::R10,
            thumb32: true,
            shift_t: SRType::LSL,
            shift_n: 2,
            index: true,
            add: true,
            wback: false,
        }
    );
}

#[test]
fn test_decode_orr_imm_w() {
    // 0xf0400010
    // ORR.W R0, R0, #16
    assert_eq!(
        decode_32(0xf0400010),
        Instruction::ORR_imm {
            params: Reg2ImmCarryParams {
                rd: Reg::R0,
                rn: Reg::R0,
                imm32: Imm32Carry::Carry {
                    imm32_c0: (16, false),
                    imm32_c1: (16, true)
                },
                setflags: false
            }
        }
    );
}

#[test]
fn test_decode_strd_w() {
    // 0xe9cd0100 -> STRD R0, R1, [SP]
    assert_eq!(
        decode_32(0xe9cd0100),
        Instruction::STRD_imm {
            rt: Reg::R0,
            rt2: Reg::R1,
            rn: Reg::SP,
            imm32: 0,
            index: true,
            add: true,
            wback: false,
        }
    );
}

#[test]
fn test_decode_ldrd_w() {
    // 0xe9d50100 -> LDRD R0, R1, [R5]
    assert_eq!(
        decode_32(0xe9d50100),
        Instruction::LDRD_imm {
            rt: Reg::R0,
            rt2: Reg::R1,
            rn: Reg::R5,
            imm32: 0,
            index: true,
            add: true,
            wback: false,
        }
    );
}

#[test]
fn test_decode_ulmull() {
    // 0xfba42300 -> UMULL R2, R3, R4, R0
    assert_eq!(
        decode_32(0xfba42300),
        Instruction::UMULL {
            params: Reg643232Params {
                rdlo: Reg::R2,
                rdhi: Reg::R3,
                rn: Reg::R4,
                rm: Reg::R0,
            }
        }
    );
}

#[test]
fn test_decode_smull() {
    // fb83 320b       smull   r3, r2, r3, fp
    assert_eq!(
        decode_32(0xfb83320b),
        Instruction::SMULL {
            params: Reg643232Params {
                rdlo: Reg::R3,
                rdhi: Reg::R2,
                rn: Reg::R3,
                rm: Reg::R11,
            }
        }
    );
}

#[test]
fn test_decode_lsr_w_reg() {
    // 0xfa30f009 -> LSRS.W R0, R0, R9
    assert_eq!(
        decode_32(0xfa30f009),
        Instruction::LSR_reg {
            params: Reg3Params {
                rd: Reg::R0,
                rn: Reg::R0,
                rm: Reg::R9,
                setflags: SetFlags::True,
            },
            thumb32: true
        }
    );
}

#[test]
fn test_decode_rsb_w_reg() {
    //0xf1c6003c -> RSB.W R0, R6, #60
    assert_eq!(
        decode_32(0xf1c6003c),
        Instruction::RSB_imm {
            params: Reg2ImmParams {
                rd: Reg::R0,
                rn: Reg::R6,
                imm32: 60,
                setflags: SetFlags::False,
            },
            thumb32: true
        }
    );
}

#[test]
fn test_decode_b_pl_w() {
    //0xf57fad69 -> BPL.W -1326
    assert_eq!(
        decode_32(0xf57fad69),
        Instruction::B_t13 {
            cond: Condition::PL,
            imm32: -1326,
            thumb32: true
        }
    );
}

#[test]
fn test_decode_tst_imm_w() {
    //0xf0113f80 -> TST.W R1, 0x80808080
    assert_eq!(
        decode_32(0xf0113f80),
        Instruction::TST_imm {
            params: RegImmCarryNoSetFlagsParams {
                rn: Reg::R1,
                imm32: Imm32Carry::Carry {
                    imm32_c0: (0x80808080, false),
                    imm32_c1: (0x80808080, true),
                },
            }
        }
    );
}

//

#[test]
fn test_decode_sbc_reg_w() {
    //0xeb6a0a4a -> SBC.W R10, R10, R10, LSL #1
    assert_eq!(
        decode_32(0xeb6a0a4a),
        Instruction::SBC_reg {
            params: Reg3ShiftParams {
                rd: Reg::R10,
                rn: Reg::R10,
                rm: Reg::R10,
                shift_n: 1,
                shift_t: SRType::LSL,
                setflags: SetFlags::False,
            },
            thumb32: true
        }
    );
}

#[test]
fn test_decode_stmdb_w() {
    //0xe920003c -> STMDB R0!, {R2-R5}
    match decode_32(0xe920003c) {
        Instruction::STMDB {
            rn,
            registers,
            wback,
        } => {
            assert!(rn == Reg::R0);
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R2, Reg::R3, Reg::R4, Reg::R5], elems);
            assert!(wback);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_bic_imm_w() {
    //0xf02400ff -> BIC.W R0, R4, #255

    assert_eq!(
        decode_32(0xf02400ff),
        Instruction::BIC_imm {
            params: Reg2ImmCarryParams {
                rd: Reg::R0,
                rn: Reg::R4,
                imm32: Imm32Carry::Carry {
                    imm32_c0: (255, false),
                    imm32_c1: (255, true),
                },
                setflags: false,
            }
        }
    );
}

#[test]
fn test_decode_ldrh_reg_w() {
    //0xf838301a -> LDRH.W R3, [R8, R10, LSL #1]

    assert_eq!(
        decode_32(0xf838301a),
        Instruction::LDRH_reg {
            rt: Reg::R3,
            rn: Reg::R8,
            rm: Reg::R10,
            shift_t: SRType::LSL,
            shift_n: 1,
            index: true,
            add: true,
            wback: false,
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_eor_imm_w() {
    //0xf4814120 -> EOR.W R1, R1, #40960 ; 0xa000
    assert_eq!(
        decode_32(0xf4814120),
        Instruction::EOR_imm {
            params: Reg2ImmCarryParams {
                rd: Reg::R1,
                rn: Reg::R1,
                imm32: Imm32Carry::Carry {
                    imm32_c0: (0xa000, false),
                    imm32_c1: (0xa000, false)
                },
                setflags: false
            }
        }
    );
}

#[test]
fn test_decode_clz_w() {
    //0xfab0f180 -> CLZ R1, R0
    assert_eq!(
        decode_32(0xfab0f180),
        Instruction::CLZ {
            rd: Reg::R1,
            rm: Reg::R0,
        }
    );
}

#[test]
fn test_decode_pop_t3_w() {
    //0xf85deb04 -> LDR.W LR, [SP], #4   // POP.W LR  (pop 3)
    match decode_32(0xf85deb04) {
        Instruction::POP { registers, thumb32 } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::LR], elems);
            assert_eq!(thumb32, true);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_and_reg_w() {
    //0xea155411 -> ANDS.W R4, R5, R1, LSR #20
    assert_eq!(
        decode_32(0xea155411),
        Instruction::AND_reg {
            params: Reg3ShiftParams {
                rd: Reg::R4,
                rn: Reg::R5,
                rm: Reg::R1,
                setflags: SetFlags::True,
                shift_t: SRType::LSR,
                shift_n: 20,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_rsb_reg_w() {
    //0xebc01046 -> RSB.W R0, R0, R6, LSL #5
    assert_eq!(
        decode_32(0xebc01046),
        Instruction::RSB_reg {
            params: Reg3ShiftParams {
                rd: Reg::R0,
                rn: Reg::R0,
                rm: Reg::R6,
                setflags: SetFlags::False,
                shift_t: SRType::LSL,
                shift_n: 5,
            },
            thumb32: true,
        }
    );
    //0xebd720c0 -> RSBS.W R0, R7, R0, LSL #11
    assert_eq!(
        decode_32(0xebd720c0),
        Instruction::RSB_reg {
            params: Reg3ShiftParams {
                rd: Reg::R0,
                rn: Reg::R7,
                rm: Reg::R0,
                setflags: SetFlags::True,
                shift_t: SRType::LSL,
                shift_n: 11,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_sbc_imm_w() {
    //0xf1670700 -> SBC.W R7, R7, #0
    assert_eq!(
        decode_32(0xf1670700),
        Instruction::SBC_imm {
            params: Reg2ImmParams {
                rd: Reg::R7,
                rn: Reg::R7,
                setflags: SetFlags::False,
                imm32: 0
            }
        }
    );
}

#[test]
fn test_decode_adc_reg_w() {
    //0xeb50500e -> ADCS.W R0, R0, LR, LSL #20

    assert_eq!(
        decode_32(0xeb50500e),
        Instruction::ADC_reg {
            params: Reg3ShiftParams {
                rd: Reg::R0,
                rn: Reg::R0,
                rm: Reg::LR,
                setflags: SetFlags::True,
                shift_t: SRType::LSL,
                shift_n: 20,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_bic_reg_w() {
    //0xea235345 -> BIC.W R3, R3, R5, LSL #21

    assert_eq!(
        decode_32(0xea235345),
        Instruction::BIC_reg {
            params: Reg3ShiftParams {
                rd: Reg::R3,
                rn: Reg::R3,
                rm: Reg::R5,
                setflags: SetFlags::False,
                shift_t: SRType::LSL,
                shift_n: 21,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_adc_imm_w() {
    // 0xf1540401 -> ADCS.W R4, R4, #1

    assert_eq!(
        decode_32(0xf1540401),
        Instruction::ADC_imm {
            params: Reg2ImmParams {
                rd: Reg::R4,
                rn: Reg::R4,
                setflags: SetFlags::True,
                imm32: 1
            }
        }
    );
}

#[test]
fn test_decode_teq_reg_w() {
    // 0xea910f03 -> TEQ.W R1, R3

    assert_eq!(
        decode_32(0xea910f03),
        Instruction::TEQ_reg {
            params: Reg2ShiftNoSetFlagsParams {
                rn: Reg::R1,
                rm: Reg::R3,
                shift_t: SRType::LSL,
                shift_n: 0
            }
        }
    );
}

#[test]
fn test_decode_ror_imm_w() {
    // 0xea4f74f4 -> ROR.W R4, R4, #31

    assert_eq!(
        decode_32(0xea4f74f4),
        Instruction::ROR_imm {
            params: Reg2ShiftNParams {
                rd: Reg::R4,
                rm: Reg::R4,
                shift_n: 31,
                setflags: SetFlags::False
            }
        }
    );
}

#[test]
fn test_decode_ldm_t2_w() {
    // 0xe8b11008 -> LDM R1!, {R3, R12}

    match decode_32(0xe8b11008) {
        Instruction::LDM {
            rn,
            registers,
            thumb32,
        } => {
            assert!(rn == Reg::R1);
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R3, Reg::R12], elems);
            assert!(thumb32);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_cmp_reg_w() {
    // 0xebb71f46 -> CMP.W R7, R6, LSL #5
    assert_eq!(
        decode_32(0xebb71f46),
        Instruction::CMP_reg {
            params: Reg2ShiftNoSetFlagsParams {
                rn: Reg::R7,
                rm: Reg::R6,
                shift_t: SRType::LSL,
                shift_n: 5,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ldrsb_imm_w() {
    // 0xf9956000 -> LDRSB R6, [R5]
    assert_eq!(
        decode_32(0xf9956000),
        Instruction::LDRSB_imm {
            rt: Reg::R6,
            rn: Reg::R5,
            imm32: 0,
            index: true,
            add: true,
            wback: false,
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_ldrsb_imm_t2() {
    // 0xf9170c09 -> ldrsb.w r0, [r7, #-9]
    assert_eq!(
        decode_32(0xf9170c09),
        Instruction::LDRSB_imm {
            rt: Reg::R0,
            rn: Reg::R7,
            imm32: 9,
            index: true,
            add: false,
            wback: false,
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_smul_bb() {
    // 0xfb1efe08 -> SMULBB LR, LR, R8
    assert_eq!(
        decode_32(0xfb1efe08),
        Instruction::SMUL {
            params: Reg3HighParams {
                rd: Reg::LR,
                rn: Reg::LR,
                rm: Reg::R8,
                n_high: false,
                m_high: false
            }
        }
    );
}

#[test]
fn test_decode_smla_bb() {
    // 0xfb15ee0b -> SMLABB LR, R5, R11, LR
    assert_eq!(
        decode_32(0xfb15ee0b),
        Instruction::SMLA {
            params: Reg4HighParams {
                rd: Reg::LR,
                rn: Reg::R5,
                rm: Reg::R11,
                ra: Reg::LR,
                n_high: false,
                m_high: false
            }
        }
    );
}

#[test]
fn test_decode_ldrb_reg_w() {
    //0xf816c004 -> LDRB.W R12, [R6, R4]

    assert_eq!(
        decode_32(0xf816c004),
        Instruction::LDRB_reg {
            rt: Reg::R12,
            rn: Reg::R6,
            rm: Reg::R4,
            shift_t: SRType::LSL,
            shift_n: 0,
            index: true,
            add: true,
            wback: false,
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_uxtab_() {
    //0xfa54f480 UXTAB.W R4, R4, R0

    assert_eq!(
        decode_32(0xfa54f480),
        Instruction::UXTAB {
            params: Reg3UsizeParams {
                rd: Reg::R4,
                rn: Reg::R4,
                rm: Reg::R0,
                rotation: 0
            }
        }
    );
}

#[test]
fn test_decode_tst_reg_w() {
    // 0xea180f03 tst.w   r8, r3

    assert_eq!(
        decode_32(0xea180f03),
        Instruction::TST_reg {
            params: Reg2ShiftNoSetFlagsParams {
                rn: Reg::R8,
                rm: Reg::R3,
                shift_n: 0,
                shift_t: SRType::LSL,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_pld_reg() {
    // 0xf890f000 pld [r0]

    assert_eq!(
        decode_32(0xf890f000),
        Instruction::PLD_imm {
            rn: Reg::R0,
            imm32: 0,
            add: true
        }
    );
}

#[test]
fn test_decode_lsl_reg_t2() {
    // 0xfa0cf505 ->     lsl.w   r5, ip, r5

    assert_eq!(
        decode_32(0xfa0cf505),
        Instruction::LSL_reg {
            params: Reg3Params {
                rd: Reg::R5,
                rn: Reg::R12,
                rm: Reg::R5,
                setflags: SetFlags::False
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_orn_reg_t2() {
    // 0xea62 0205       orn     r2, r2, r5

    assert_eq!(
        decode_32(0xea620205),
        Instruction::ORN_reg {
            params: Reg3ShiftParams {
                rd: Reg::R2,
                rn: Reg::R2,
                rm: Reg::R5,
                setflags: SetFlags::False,
                shift_t: SRType::LSL,
                shift_n: 0,
            }
        }
    );
}

#[test]
fn test_decode_uadd8() {
    // fa82 f24c       uadd8   r2, r2, ip

    assert_eq!(
        decode_32(0xfa82f24c),
        Instruction::UADD8 {
            params: Reg3NoSetFlagsParams {
                rd: Reg::R2,
                rn: Reg::R2,
                rm: Reg::R12,
            }
        }
    );
}

#[test]
fn test_decode_sel() {
    //0xfaa4f28c       sel     r2, r4, ip

    assert_eq!(
        decode_32(0xfaa4f28c),
        Instruction::SEL {
            rd: Reg::R2,
            rn: Reg::R4,
            rm: Reg::R12,
        }
    );
}

#[test]
fn test_decode_tbh() {
    // e8df f013       tbh     [pc, r3, lsl #1]

    assert_eq!(
        decode_32(0xe8dff013),
        Instruction::TBH {
            rn: Reg::PC,
            rm: Reg::R3,
        }
    );
}

#[test]
fn test_decode_movt() {
    // f2c2 0100       movt    r1, #8192

    assert_eq!(
        decode_32(0xf2c20100),
        Instruction::MOVT {
            rd: Reg::R1,
            imm16: 0x2000
        }
    );
}

#[test]
fn test_decode_mvn_reg_w() {
    // ea6f 5507       mvn.w   r5, r7, lsl #20

    assert_eq!(
        decode_32(0xea6f5507),
        Instruction::MVN_reg {
            params: Reg2ShiftParams {
                rd: Reg::R5,
                rm: Reg::R7,
                setflags: SetFlags::False,
                shift_n: 20,
                shift_t: SRType::LSL,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_teq_w() {
    //f090 0f00       teq     r0, #0
    assert_eq!(
        decode_32(0xf0900f00),
        Instruction::TEQ_imm {
            params: RegImmCarryNoSetFlagsParams {
                rn: Reg::R0,
                imm32: Imm32Carry::Carry {
                    imm32_c0: (0, false),
                    imm32_c1: (0, true),
                },
            }
        }
    );
}

#[test]
fn test_decode_mov_rxx_w() {
    //ea4f 0232       mov.w   r2, r2, rrx
    assert_eq!(
        decode_32(0xea4f0232),
        Instruction::RRX {
            params: Reg2Params {
                rd: Reg::R2,
                rm: Reg::R2,
                setflags: false,
            }
        }
    );
}

#[test]
fn test_decode_str_imm_t4() {
    //f84d cd04       str.w   ip, [sp, #-4]!
    // => same as PUSH r12
    match decode_32(0xf84dcd04) {
        Instruction::PUSH { registers, thumb32 } => {
            let elems: Vec<_> = registers.iter().collect();
            assert_eq!(vec![Reg::R12], elems);
            assert_eq!(thumb32, true);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn test_decode_subw_imm_t4() {
    // f2a4 4333       subw    r3, r4, #1075   ; 0x433
    assert_eq!(
        decode_32(0xf2a44333),
        Instruction::SUB_imm {
            params: Reg2ImmParams {
                rd: Reg::R3,
                rn: Reg::R4,
                imm32: 1075,
                setflags: SetFlags::False,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_asrw_reg_t2() {
    //  fa43 f305       asr.w   r3, r3, r5
    assert_eq!(
        decode_32(0xfa43f305),
        Instruction::ASR_reg {
            params: Reg3Params {
                rd: Reg::R3,
                rn: Reg::R3,
                rm: Reg::R5,
                setflags: SetFlags::False,
            },
            thumb32: true,
        }
    );
}

#[test]
fn test_decode_dmb() {
    //  f3bf 8f5f       dmb sy
    assert_eq!(decode_32(0xf3bf8f5f), Instruction::DMB);
}

#[test]
fn test_decode_ldrex() {
    //  e850 3f00       ldrex   r3, [r0]
    assert_eq!(
        decode_32(0xe8503f00),
        Instruction::LDREX {
            rt: Reg::R3,
            rn: Reg::R0,
            imm32: 0,
        }
    );
}

#[test]
fn test_decode_strex() {
    //  e840 2c00       strex   ip, r2, [r0]
    assert_eq!(
        decode_32(0xe8402c00),
        Instruction::STREX {
            rd: Reg::R12,
            rt: Reg::R2,
            rn: Reg::R0,
            imm32: 0,
        }
    );
}

#[test]
fn test_decode_bfc() {
    //  f36f 011f       bfc     r1, #0, #32
    assert_eq!(
        decode_32(0xf36f011f),
        Instruction::BFC {
            rd: Reg::R1,
            lsbit: 0,
            msbit: 31,
        }
    );
}

#[test]
fn test_decode_vldr() {
    //  ed9f 7b86       vldr    d7, [pc, #536]  ; 448 <_vfprintf_r+0x290>
    assert_eq!(
        decode_32(0xed9f7b86),
        Instruction::VLDR {
            dd: ExtensionReg::Double { reg: DoubleReg::D7 },
            rn: Reg::PC,
            add: true,
            imm32: 0x86 << 2,
            single_reg: false
        }
    );
}

#[test]
fn test_decode_vstr() {
    //250:       ed8d 7b12       vstr    d7, [sp, #72]   ; 0x48
    assert_eq!(
        decode_32(0xed8d7b12),
        Instruction::VSTR {
            dd: ExtensionReg::Double { reg: DoubleReg::D7 },
            rn: Reg::SP,
            add: true,
            imm32: 0x48,
            single_reg: false
        }
    );
}
