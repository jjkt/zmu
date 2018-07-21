use core::bits::*;
//use bit_field::BitField;
use core::instruction::Instruction;
use core::ThumbCode;

#[cfg(test)]
use core::instruction::CpsEffect;
#[cfg(test)]
use core::register::SpecialReg;

use core::register::Reg;

#[cfg(test)]
use core::condition::Condition;

mod adc;
mod add;
mod adr;
mod and;
mod asr;

mod b;
mod bic;
mod bkpt;
mod bl;
mod blx;
mod bx;

mod cmn;
mod cmp;
mod cps;

mod eor;

mod ldc;
mod ldm;
mod ldr;
mod ldrb;
mod ldrh;
mod ldrsb;
mod ldrsh;
mod lsl;
mod lsr;

mod mcr; // ARMv7-M

mod mov;
mod mrs;
mod msr;
mod mul;
mod mvn;

mod nop;
mod orr;

mod pop;
mod push;

mod rev;
mod ror;
mod rsb;

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
use decoder::bkpt::*;
use decoder::bl::*;
use decoder::blx::*;
use decoder::bx::*;

use decoder::cmn::*;
use decoder::cmp::*;
use decoder::cps::*;

use decoder::eor::*;

use decoder::ldc::*;
use decoder::ldm::*;
use decoder::ldr::*;
use decoder::ldrb::*;
use decoder::ldrh::*;
use decoder::ldrsb::*;
use decoder::ldrsh::*;
use decoder::lsl::*;
use decoder::lsr::*;

use decoder::mcr::*;
use decoder::mov::*;
use decoder::mrs::*;
use decoder::msr::*;
use decoder::mul::*;
use decoder::mvn::*;

use decoder::nop::*;
use decoder::orr::*;
use decoder::pop::*;
use decoder::push::*;

use decoder::rev::*;
use decoder::ror::*;
use decoder::rsb::*;

use decoder::sbc::*;
use decoder::stm::*;
use decoder::str::*;
use decoder::sub::*;
use decoder::sxt::*;
use decoder::tst::*;
use decoder::uxt::*;

pub fn is_thumb32(word: u16) -> bool {
    match word.get_bits(11, 15) {
        0b11101 | 0b11110 | 0b11111 => true,
        _ => false,
    }
}

#[allow(non_snake_case)]
fn decode_undefined(opcode: u16) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_undefined32(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

pub fn decode_16(opcode: u16) -> Instruction {
    match opcode {
        0...2047 => decode_MOV_reg_t2_LSL_imm_t1(opcode),
        2048...4095 => decode_LSR_imm_t1(opcode),
        4096...6143 => decode_ASR_imm_t1(opcode),
        6144...6655 => decode_ADD_reg_t1(opcode),
        6656...7167 => decode_SUB_reg_t1(opcode),
        7168...7679 => decode_ADD_imm_t1(opcode),
        7680...8191 => decode_SUB_imm_t1(opcode),
        8192...10239 => decode_MOV_imm_t1(opcode),
        10240...12287 => decode_CMP_imm_t1(opcode),
        12288...14335 => decode_ADD_imm_t2(opcode),
        14336...16383 => decode_SUB_imm_t2(opcode),
        16384...16447 => decode_AND_reg_t1(opcode),
        16448...16511 => decode_EOR_reg_t1(opcode),
        16512...16575 => decode_LSL_reg_t1(opcode),
        16576...16639 => decode_LSR_reg_t1(opcode),
        16640...16703 => decode_ASR_reg_t1(opcode),
        16704...16767 => decode_ADC_reg_t1(opcode),
        16768...16831 => decode_SBC_reg_t1(opcode),
        16832...16895 => decode_ROR_reg_t1(opcode),
        16896...16959 => decode_TST_reg_t1(opcode),
        16960...17023 => decode_RSB_imm_t1(opcode),
        17024...17087 => decode_CMP_reg_t1(opcode),
        17088...17151 => decode_CMN_reg_t1(opcode),
        17152...17215 => decode_ORR_reg_t1(opcode),
        17216...17279 => decode_MUL_t1(opcode),
        17280...17343 => decode_BIC_reg_t1(opcode),
        17344...17407 => decode_MVN_reg_t1(opcode),
        17408...17663 => decode_ADD_reg_t2_ADD_SP_reg(opcode),
        17664...17919 => decode_CMP_reg_t2(opcode),
        17920...18175 => decode_MOV_reg_t1(opcode),
        18176...18296 => decode_BX_t1(opcode),
        18304...18424 => decode_BLX_t1(opcode),
        18432...20479 => decode_LDR_lit_t1(opcode),
        20480...20991 => decode_STR_reg_t1(opcode),
        20992...21503 => decode_STRH_reg_t1(opcode),
        21504...22015 => decode_STRB_reg_t1(opcode),
        22016...22527 => decode_LDRSB_reg_t1(opcode),
        22528...23039 => decode_LDR_reg_t1(opcode),
        23040...23551 => decode_LDRH_reg_t1(opcode),
        23552...24063 => decode_LDRB_reg_t1(opcode),
        24064...24575 => decode_LDRSH_reg_t1(opcode),
        24576...26623 => decode_STR_imm_t1(opcode),
        26624...28671 => decode_LDR_imm_t1(opcode),
        28672...30719 => decode_STRB_imm_t1(opcode),
        30720...32767 => decode_LDRB_imm_t1(opcode),
        32768...34815 => decode_STRH_imm_t1(opcode),
        34816...36863 => decode_LDRH_imm_t1(opcode),
        36864...38911 => decode_STR_imm_t2(opcode),
        38912...40959 => decode_LDR_imm_t2(opcode),
        40960...43007 => decode_ADR_t1(opcode),
        43008...45055 => decode_ADD_SP_imm_t1(opcode),
        45056...45183 => decode_ADD_SP_imm_t2(opcode),
        45184...45311 => decode_SUB_SP_imm_t1(opcode),
        45568...45631 => decode_SXTH_t1(opcode),
        45632...45695 => decode_SXTB_t1(opcode),
        45696...45759 => decode_UXTH_t1(opcode),
        45760...45823 => decode_UXTB_t1(opcode),
        46080...46591 => decode_PUSH_t1(opcode),
        46690...46706 => decode_CPS_t1(opcode),
        47616...47679 => decode_REV_t1(opcode),
        47680...47743 => decode_REV16_t1(opcode),
        47808...47871 => decode_REVSH_t1(opcode),
        48128...48639 => decode_POP_reg_t1(opcode),
        48640...48895 => decode_BKPT_t1(opcode),
        48896...48896 => decode_NOP_t1(opcode),
        48912...48912 => Instruction::YIELD,
        48928...48928 => Instruction::WFE,
        48944...48944 => Instruction::WFI,
        48960...48960 => Instruction::SEV,
        49152...51199 => decode_STM_t1(opcode),
        51200...53247 => decode_LDM_t1(opcode),
        53248...57343 => decode_B_t1_SVC_t1(opcode),
        57344...59391 => decode_B_t2(opcode),
        _ => decode_undefined(opcode),
    }
}
/*
#[allow(non_snake_case)]
fn decode_CDP2_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_CDP_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_CLZ_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_CMN_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_DMB_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_DSB_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_ISB_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_LDMDB_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_LDM_W_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_LDRBT_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_LDRD_imm_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_LDRD_lit_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_LDREXB_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_LDREXH_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_LDREX_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_LDRHT_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_LDRSBT_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_LDRSHT(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_LDRT_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_MCRR2_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_MCRR_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_MLA_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_MLS_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_MRC2_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_MRC_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_ORN_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_PLD_imm_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_PLD_imm_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_PLI_lit_imm_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_PLI_lit_imm_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_PLI_lit_imm_t3(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_PLI_reg_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_RBIT_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_RRX_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_RSB_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_SBC_reg_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_SDIV_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_SMLAL_t1(opcode: u32) -> Instruction {
    let reg_rm: u8 = opcode.get_bits(0, 3);
    let reg_rd_hi: u8 = opcode.get_bits(8, 11);
    let reg_rd_lo: u8 = opcode.get_bits(12, 15);
    let reg_rn: u8 = opcode.get_bits(16, 19);
    Instruction::SMLAL {
        rm: Reg::from(reg_rm),
        rdlo: Reg::from(reg_rd_hi),
        rdhi: Reg::from(reg_rd_lo),
        rn: Reg::from(reg_rn),
    }
}

#[allow(non_snake_case)]
fn decode_SMULL_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_STC2_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_STC_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_STMDB_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_STMX_W_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_STRD_imm_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_STREXB_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_STREXH_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_STREX_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_TBB_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_TBH_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_TEQ_reg_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_UDF_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

#[allow(non_snake_case)]
fn decode_UDIV_t1(opcode: u32) -> Instruction {
    let reg_rm: u8 = opcode.get_bits(0, 3);
    let reg_rd: u8 = opcode.get_bits(8, 11);
    let reg_rn: u8 = opcode.get_bits(16, 19);
    Instruction::UDIV {
        rm: Reg::from(reg_rm),
        rd: Reg::from(reg_rd),
        rn: Reg::from(reg_rn),
    }
}

#[allow(non_snake_case)]
fn decode_UMLAL_t1(opcode: u32) -> Instruction {
    let reg_rm: u8 = opcode.get_bits(0, 3);
    let reg_rd_hi: u8 = opcode.get_bits(8, 11);
    let reg_rd_lo: u8 = opcode.get_bits(12, 15);
    let reg_rn: u8 = opcode.get_bits(16, 19);
    Instruction::UMLAL {
        rm: Reg::from(reg_rm),
        rdlo: Reg::from(reg_rd_hi),
        rdhi: Reg::from(reg_rd_lo),
        rn: Reg::from(reg_rn),
    }
}

#[allow(non_snake_case)]
fn decode_UMULL_t1(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}
*/

fn decode_group2(opcode: u32) -> Instruction {
    let op2: u8 = opcode.get_bits(12, 14);
    let op1: u8 = opcode.get_bits(20, 26);

    match op2 {
        0x7 | 0x5 => decode_BL_t1(opcode),
        0 => match op1 {
            0b011_1000 | 0b011_1001 => decode_MSR_reg_t1(opcode),
            //0b011_1011 => decode_control(t1, t2),
            0b011_1111 | 0b011_1110 => decode_MRS_t1(opcode),
            _ => Instruction::UDF {
                imm32: 0,
                opcode: ThumbCode::from(opcode),
            },
        },
        _ => Instruction::UDF {
            imm32: 0,
            opcode: ThumbCode::from(opcode),
        },
    }
}

// A5.3 check thumb32 encodings
pub fn decode_32(opcode: u32) -> Instruction {
    let op1: u8 = opcode.get_bits(27, 28);

    match op1 {
        0b01 => decode_undefined32(opcode),
        0b10 => decode_group2(opcode),
        0b11 => decode_undefined32(opcode),
        _ => decode_undefined32(opcode),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

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
                assert_eq!(cond, Condition::EQ);
                assert_eq!(imm32, (1 << 1));
            }
            _ => {
                println!(" {}", decode_16(0xd001));
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
    fn test_decode_ldm3() {
        // LDM R4!, {R0-R2}
        match decode_16(0xcc07) {
            Instruction::LDM { rn, registers } => {
                assert!(rn == Reg::R4);
                let elems: Vec<_> = registers.iter().collect();
                assert_eq!(vec![Reg::R0, Reg::R1, Reg::R2], elems);
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
    fn test_decode_stm2() {
        // STM R3!, {R0-R2}
        match decode_16(0xc307) {
            Instruction::STM { rn, registers } => {
                assert!(rn == Reg::R3);
                let elems: Vec<_> = registers.iter().collect();
                assert_eq!(vec![Reg::R0, Reg::R1, Reg::R2], elems);
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
            Instruction::RSB_imm {
                rd,
                rn,
                imm32,
                setflags,
            } => {
                assert_eq!(rd, Reg::R2);
                assert_eq!(rn, Reg::R0);
                assert_eq!(imm32, 0);
                assert_eq!(setflags, true);
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_decode_mrs() {
        // MRS R0, ipsr
        assert_eq!(
            decode_32(0xf3ef8005),
            Instruction::MRS {
                rd: Reg::R0,
                spec_reg: SpecialReg::IPSR
            }
        );
    }

    #[test]
    fn test_decode_cpsid() {
        // CPSID i
        assert_eq!(decode_16(0xB672), Instruction::CPS { im: CpsEffect::ID });
    }

    #[test]
    fn test_decode_lsl_2() {
        // LSL r1, r1, #31
        assert_eq!(
            decode_16(0x07c9),
            Instruction::LSL_imm {
                rd: Reg::R1,
                rm: Reg::R1,
                imm5: 31,
                setflags: true
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

}
