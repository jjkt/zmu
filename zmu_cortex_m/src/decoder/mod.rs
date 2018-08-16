use core::bits::*;
use core::instruction::Instruction;
use core::ThumbCode;

#[cfg(test)]
use core::instruction::CpsEffect;
#[cfg(test)]
use core::register::SpecialReg;

#[cfg(test)]
use core::condition::Condition;
#[cfg(test)]
use core::instruction::ITCondition;

mod bfc;
mod bfi;
mod cbz;
mod clrex;
mod dbg;
mod sbfx;
mod ssat;
mod ubfx;
mod usat;
mod wfe;
mod wfi;
mod yield_;

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

mod clz;
mod cmn;
mod cmp;
mod cpd;
mod cps;

mod dmb;
mod dsb;

mod eor;

mod isb;
mod it;

mod ldc;
mod ldm;
mod ldr;
mod ldrb;
mod ldrh;
mod ldrsb;
mod ldrsh;
mod lsl;
mod lsr;

mod mcr;
mod mla;
mod mls;
mod mov;
mod mrs;
mod msr;
mod mul;
mod mvn;

mod nop;
mod orn;
mod orr;

mod pld;
mod pli;
mod pop;
mod push;

mod rbit;
mod rev;
mod ror;
mod rrx;
mod rsb;

mod sbc;
mod sdiv;
mod sev;
mod smlal;
mod smull;
mod stc;
mod stm;
mod str;
mod strex;
mod sub;
mod sxt;

mod tbb;
mod tbh;
mod teq;
mod tst;

mod movt;
mod udiv;
mod umlal;
mod umull;
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

use decoder::cbz::*;
use decoder::clz::*;
use decoder::cmn::*;
use decoder::cmp::*;
use decoder::cpd::*;
use decoder::cps::*;

use decoder::dmb::*;
use decoder::dsb::*;

use decoder::eor::*;

use decoder::isb::*;
use decoder::it::*;

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
use decoder::mla::*;
use decoder::mls::*;
use decoder::mov::*;
use decoder::mrs::*;
use decoder::msr::*;
use decoder::mul::*;
use decoder::mvn::*;

use decoder::nop::*;

use decoder::orn::*;
use decoder::orr::*;

use decoder::pld::*;
use decoder::pli::*;
use decoder::pop::*;
use decoder::push::*;

use decoder::rbit::*;
use decoder::rev::*;
use decoder::ror::*;
use decoder::rrx::*;
use decoder::rsb::*;

use decoder::sbc::*;
use decoder::sdiv::*;
use decoder::smlal::*;
use decoder::smull::*;
use decoder::stc::*;
use decoder::stm::*;
use decoder::str::*;
use decoder::strex::*;
use decoder::sub::*;
use decoder::sxt::*;

use decoder::tbb::*;
use decoder::tbh::*;
use decoder::teq::*;
use decoder::tst::*;

use decoder::udiv::*;
use decoder::umlal::*;
use decoder::umull::*;
use decoder::uxt::*;

use decoder::bfc::*;
use decoder::bfi::*;
use decoder::clrex::*;
use decoder::dbg::*;
use decoder::movt::*;
use decoder::sbfx::*;
use decoder::sev::*;
use decoder::ssat::*;
use decoder::ubfx::*;
use decoder::usat::*;
use decoder::wfe::*;
use decoder::wfi::*;
use decoder::yield_::*;

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
fn decode_UDF_t2(opcode: u32) -> Instruction {
    Instruction::UDF {
        imm32: 0,
        opcode: ThumbCode::from(opcode),
    }
}

pub fn decode_16(opcode: u16) -> Instruction {
    if (opcode & 0xffff) == 0xbf30 {
        decode_WFI_t1(opcode)
    } else if (opcode & 0xffff) == 0xbf20 {
        decode_WFE_t1(opcode)
    } else if (opcode & 0xffff) == 0xbf40 {
        decode_SEV_t1(opcode)
    } else if (opcode & 0xffff) == 0xbf00 {
        decode_NOP_t1(opcode)
    } else if (opcode & 0xffff) == 0xbf10 {
        decode_YIELD_t1(opcode)
    } else if (opcode & 0xffef) == 0xb662 {
        decode_CPS_t1(opcode)
    } else if (opcode & 0xff87) == 0x4700 {
        decode_BX_t1(opcode)
    } else if (opcode & 0xff87) == 0x4780 {
        decode_BLX_t1(opcode)
    } else if (opcode & 0xffc0) == 0x4140 {
        decode_ADC_reg_t1(opcode)
    } else if (opcode & 0xffc0) == 0xb280 {
        decode_UXTH_t1(opcode)
    } else if (opcode & 0xffc0) == 0x4180 {
        decode_SBC_reg_t1(opcode)
    } else if (opcode & 0xffc0) == 0xbac0 {
        decode_REVSH_t1(opcode)
    } else if (opcode & 0xffc0) == 0x4300 {
        decode_ORR_reg_t1(opcode)
    } else if (opcode & 0xffc0) == 0xb200 {
        decode_SXTH_t1(opcode)
    } else if (opcode & 0xffc0) == 0x4100 {
        decode_ASR_reg_t1(opcode)
    } else if (opcode & 0xffc0) == 0x4380 {
        decode_BIC_reg_t1(opcode)
    } else if (opcode & 0xffc0) == 0xb2c0 {
        decode_UXTB_t1(opcode)
    } else if (opcode & 0xffc0) == 0x4040 {
        decode_EOR_reg_t1(opcode)
    } else if (opcode & 0xffc0) == 0xba00 {
        decode_REV_t1(opcode)
    } else if (opcode & 0xffc0) == 0x4080 {
        decode_LSL_reg_t1(opcode)
    } else if (opcode & 0xffc0) == 0xba40 {
        decode_REV16_t1(opcode)
    } else if (opcode & 0xffc0) == 0x43c0 {
        decode_MVN_reg_t1(opcode)
    } else if (opcode & 0xffc0) == 0xb240 {
        decode_SXTB_t1(opcode)
    } else if (opcode & 0xffc0) == 0x42c0 {
        decode_CMN_reg_t1(opcode)
    } else if (opcode & 0xffc0) == 0x41c0 {
        decode_ROR_reg_t1(opcode)
    } else if (opcode & 0xffc0) == 0x4000 {
        decode_AND_reg_t1(opcode)
    } else if (opcode & 0xffc0) == 0x4200 {
        decode_TST_reg_t1(opcode)
    } else if (opcode & 0xffc0) == 0x4280 {
        decode_CMP_reg_t1(opcode)
    } else if (opcode & 0xffc0) == 0x40c0 {
        decode_LSR_reg_t1(opcode)
    } else if (opcode & 0xffc0) == 0x4340 {
        decode_MUL_t1(opcode)
    } else if (opcode & 0xffc0) == 0x4240 {
        decode_RSB_imm_t1(opcode)
    } else if (opcode & 0xff80) == 0xb000 {
        decode_ADD_SP_imm_t2(opcode)
    } else if (opcode & 0xff80) == 0xb080 {
        decode_SUB_SP_imm_t1(opcode)
    } else if (opcode & 0xff00) == 0x4500 {
        decode_CMP_reg_t2(opcode)
    } else if (opcode & 0xff00) == 0x4600 {
        decode_MOV_reg_t1(opcode)
    } else if (opcode & 0xff00) == 0x4400 {
        decode_ADD_reg_t2_ADD_SP_reg(opcode)
    } else if (opcode & 0xff00) == 0xbe00 {
        decode_BKPT_t1(opcode)
    } else if (opcode & 0xff00) == 0xbf00 {
        decode_IT_t1(opcode)
    } else if (opcode & 0xfe00) == 0x5800 {
        decode_LDR_reg_t1(opcode)
    } else if (opcode & 0xfe00) == 0x1c00 {
        decode_ADD_imm_t1(opcode)
    } else if (opcode & 0xfe00) == 0x5e00 {
        decode_LDRSH_reg_t1(opcode)
    } else if (opcode & 0xfe00) == 0x5000 {
        decode_STR_reg_t1(opcode)
    } else if (opcode & 0xfe00) == 0x1a00 {
        decode_SUB_reg_t1(opcode)
    } else if (opcode & 0xfe00) == 0x5400 {
        decode_STRB_reg_t1(opcode)
    } else if (opcode & 0xfe00) == 0x5c00 {
        decode_LDRB_reg_t1(opcode)
    } else if (opcode & 0xfe00) == 0x5200 {
        decode_STRH_reg_t1(opcode)
    } else if (opcode & 0xfe00) == 0x5600 {
        decode_LDRSB_reg_t1(opcode)
    } else if (opcode & 0xfe00) == 0x1e00 {
        decode_SUB_imm_t1(opcode)
    } else if (opcode & 0xfe00) == 0xb400 {
        decode_PUSH_t1(opcode)
    } else if (opcode & 0xfe00) == 0xbc00 {
        decode_POP_reg_t1(opcode)
    } else if (opcode & 0xfe00) == 0x5a00 {
        decode_LDRH_reg_t1(opcode)
    } else if (opcode & 0xfe00) == 0x1800 {
        decode_ADD_reg_t1(opcode)
    } else if (opcode & 0xf500) == 0xb100 {
        decode_CBZ_t1(opcode)
    } else if (opcode & 0xf800) == 0x7800 {
        decode_LDRB_imm_t1(opcode)
    } else if (opcode & 0xf800) == 0x7000 {
        decode_STRB_imm_t1(opcode)
    } else if (opcode & 0xf800) == 0x4800 {
        decode_LDR_lit_t1(opcode)
    } else if (opcode & 0xf800) == 0x800 {
        decode_LSR_imm_t1(opcode)
    } else if (opcode & 0xf800) == 0x2800 {
        decode_CMP_imm_t1(opcode)
    } else if (opcode & 0xf800) == 0xa800 {
        decode_ADD_SP_imm_t1(opcode)
    } else if (opcode & 0xf800) == 0x3800 {
        decode_SUB_imm_t2(opcode)
    } else if (opcode & 0xf800) == 0xa000 {
        decode_ADR_t1(opcode)
    } else if (opcode & 0xf800) == 0x9800 {
        decode_LDR_imm_t2(opcode)
    } else if (opcode & 0xf800) == 0x1000 {
        decode_ASR_imm_t1(opcode)
    } else if (opcode & 0xf800) == 0xc000 {
        decode_STM_t1(opcode)
    } else if (opcode & 0xf800) == 0xe000 {
        decode_B_t2(opcode)
    } else if (opcode & 0xf800) == 0x6000 {
        decode_STR_imm_t1(opcode)
    } else if (opcode & 0xf800) == 0x3000 {
        decode_ADD_imm_t2(opcode)
    } else if (opcode & 0xf800) == 0x0 {
        decode_MOV_reg_t2_LSL_imm_t1(opcode)
    } else if (opcode & 0xf800) == 0x9000 {
        decode_STR_imm_t2(opcode)
    } else if (opcode & 0xf800) == 0x8000 {
        decode_STRH_imm_t1(opcode)
    } else if (opcode & 0xf800) == 0x8800 {
        decode_LDRH_imm_t1(opcode)
    } else if (opcode & 0xf800) == 0xc800 {
        decode_LDM_t1(opcode)
    } else if (opcode & 0xf800) == 0x6800 {
        decode_LDR_imm_t1(opcode)
    } else if (opcode & 0xf800) == 0x2000 {
        decode_MOV_imm_t1(opcode)
    } else if (opcode & 0xf000) == 0xd000 {
        decode_B_t1_SVC_t1(opcode)
    } else {
        decode_undefined(opcode)
    }
}

pub fn decode_32(opcode: u32) -> Instruction {
    if (opcode & 0xffffffff) == 0xf3af8000 {
        decode_NOP_t2(opcode)
    } else if (opcode & 0xffffffff) == 0xf3af80f0 {
        decode_DBG_t1(opcode)
    } else if (opcode & 0xffffffff) == 0xf3af8004 {
        decode_SEV_t2(opcode)
    } else if (opcode & 0xffffffff) == 0xf3af8001 {
        decode_YIELD_t2(opcode)
    } else if (opcode & 0xffffffff) == 0xf3af8002 {
        decode_WFE_t2(opcode)
    } else if (opcode & 0xffffffff) == 0xf3af8003 {
        decode_WFI_t2(opcode)
    } else if (opcode & 0xffff0fff) == 0xf84d0d04 {
        decode_PUSH_t3(opcode)
    } else if (opcode & 0xfffffff0) == 0xf3bf8f60 {
        decode_ISB_t1(opcode)
    } else if (opcode & 0xfffffff0) == 0xf3bf8f20 {
        decode_CLREX_t1(opcode)
    } else if (opcode & 0xffff0fff) == 0xf85d0b04 {
        decode_POP_t3(opcode)
    } else if (opcode & 0xfffffff0) == 0xf3bf8f40 {
        decode_DSB_t1(opcode)
    } else if (opcode & 0xfffffff0) == 0xf3bf8f50 {
        decode_DMB_t1(opcode)
    } else if (opcode & 0xfff00fff) == 0xe8d00f4f {
        decode_LDREXB_t1(opcode)
    } else if (opcode & 0xfff0fff0) == 0xe8d0f000 {
        decode_TBB_t1(opcode)
    } else if (opcode & 0xfff00fff) == 0xe8d00f5f {
        decode_LDREXH_t1(opcode)
    } else if (opcode & 0xfff0fff0) == 0xe8d0f010 {
        decode_TBH_t1(opcode)
    } else if (opcode & 0xffeff0f0) == 0xea4f0030 {
        decode_RRX_t1(opcode)
    } else if (opcode & 0xffeff0f0) == 0xea4f0000 {
        decode_MOV_reg_t2(opcode)
    } else if (opcode & 0xfffff0c0) == 0xfa0ff080 {
        decode_SXTH_t2(opcode)
    } else if (opcode & 0xfff0ffc0) == 0xf910f000 {
        decode_PLI_reg_t1(opcode)
    } else if (opcode & 0xfffff0c0) == 0xfa1ff080 {
        decode_UXTH_t2(opcode)
    } else if (opcode & 0xfffff0c0) == 0xfa5ff080 {
        decode_UXTB_t2(opcode)
    } else if (opcode & 0xfffff0c0) == 0xfa4ff080 {
        decode_SXTB_t2(opcode)
    } else if (opcode & 0xfff0ff00) == 0xf910fc00 {
        decode_PLI_lit_imm_t2(opcode)
    } else if (opcode & 0xfff0ff00) == 0xf810fc00 {
        decode_PLD_imm_t2(opcode)
    } else if (opcode & 0xfff0f0f0) == 0xfa90f0a0 {
        decode_RBIT_t1(opcode)
    } else if (opcode & 0xfff0ff00) == 0xf3808800 {
        decode_MSR_reg_t1(opcode)
    } else if (opcode & 0xfff0f0f0) == 0xfa90f0b0 {
        decode_REVSH_t2(opcode)
    } else if (opcode & 0xfff0f0f0) == 0xfbb0f0f0 {
        decode_UDIV_t1(opcode)
    } else if (opcode & 0xfff0f0f0) == 0xfb90f0f0 {
        decode_SDIV_t1(opcode)
    } else if (opcode & 0xfff0f0f0) == 0xfab0f080 {
        decode_CLZ_t1(opcode)
    } else if (opcode & 0xfff00ff0) == 0xe8c00f40 {
        decode_STREXB_t1(opcode)
    } else if (opcode & 0xfff0f0f0) == 0xfa90f080 {
        decode_REV_t2(opcode)
    } else if (opcode & 0xfff0f0f0) == 0xfa90f090 {
        decode_REV16_t2(opcode)
    } else if (opcode & 0xfff00ff0) == 0xe8c00f50 {
        decode_STREXH_t1(opcode)
    } else if (opcode & 0xfffff000) == 0xf3ef8000 {
        decode_MRS_t1(opcode)
    } else if (opcode & 0xfff0f0f0) == 0xfb00f000 {
        decode_MUL_t2(opcode)
    } else if (opcode & 0xffe0f0f0) == 0xfa00f000 {
        decode_LSL_reg_t2(opcode)
    } else if (opcode & 0xffe0f0f0) == 0xfa60f000 {
        decode_ROR_reg_t2(opcode)
    } else if (opcode & 0xff7ff000) == 0xf91ff000 {
        decode_PLI_lit_imm_t3(opcode)
    } else if (opcode & 0xffe0f0f0) == 0xfa40f000 {
        decode_ASR_reg_t2(opcode)
    } else if (opcode & 0xffe0f0f0) == 0xfa20f000 {
        decode_LSR_reg_t2(opcode)
    } else if (opcode & 0xfff00fc0) == 0xf9000000 {
        decode_LDRSB_reg_t2(opcode)
    } else if (opcode & 0xfff00fc0) == 0xf9300000 {
        decode_LDRSH_reg_t2(opcode)
    } else if (opcode & 0xfff00fc0) == 0xf8000000 {
        decode_STRB_reg_t2(opcode)
    } else if (opcode & 0xfff00fc0) == 0xf8500000 {
        decode_LDR_reg_t2(opcode)
    } else if (opcode & 0xffffa000) == 0xe92d0000 {
        decode_PUSH_t2(opcode)
    } else if (opcode & 0xffef8030) == 0xea4f0010 {
        decode_LSR_imm_t2(opcode)
    } else if (opcode & 0xfff00fc0) == 0xf8400000 {
        decode_STR_reg_t2(opcode)
    } else if (opcode & 0xfff00fc0) == 0xf8300000 {
        decode_LDRH_reg_t2(opcode)
    } else if (opcode & 0xffef8030) == 0xea4f0030 {
        decode_ROR_imm_t1(opcode)
    } else if (opcode & 0xffff8020) == 0xf36f0000 {
        decode_BFC_t1(opcode)
    } else if (opcode & 0xffef8030) == 0xea4f0020 {
        decode_ASR_imm_t2(opcode)
    } else if (opcode & 0xffef8030) == 0xea4f0000 {
        decode_LSL_imm_t2(opcode)
    } else if (opcode & 0xffff2000) == 0xe8bd0000 {
        decode_POP_t2(opcode)
    } else if (opcode & 0xfff00f80) == 0xf8100000 {
        decode_LDRB_reg_t2(opcode)
    } else if (opcode & 0xfff08f00) == 0xea100f00 {
        decode_TST_reg_t2(opcode)
    } else if (opcode & 0xfff08f00) == 0xeb100f00 {
        decode_CMN_reg_t2(opcode)
    } else if (opcode & 0xfff08f00) == 0xea900f00 {
        decode_TEQ_reg_t1(opcode)
    } else if (opcode & 0xfbf08f00) == 0xf1100f00 {
        decode_CMN_imm_t1(opcode)
    } else if (opcode & 0xfbff8000) == 0xf20f0000 {
        decode_ADR_t3(opcode)
    } else if (opcode & 0xfbf08f00) == 0xf1b00f00 {
        decode_CMP_imm_t2(opcode)
    } else if (opcode & 0xfff000f0) == 0xfbe00000 {
        decode_UMLAL_t1(opcode)
    } else if (opcode & 0xfff000f0) == 0xfb000000 {
        decode_MLA_t1(opcode)
    } else if (opcode & 0xfff000f0) == 0xfb000010 {
        decode_MLS_t1(opcode)
    } else if (opcode & 0xfbf08f00) == 0xf0900f00 {
        decode_TEQ_imm_t1(opcode)
    } else if (opcode & 0xfff0f000) == 0xf890f000 {
        decode_PLD_imm_t1(opcode)
    } else if (opcode & 0xfbff8000) == 0xf2af0000 {
        decode_ADR_t2(opcode)
    } else if (opcode & 0xfff000f0) == 0xfbc00000 {
        decode_SMLAL_t1(opcode)
    } else if (opcode & 0xfbf08f00) == 0xf0100f00 {
        decode_TST_imm_t1(opcode)
    } else if (opcode & 0xfff00f00) == 0xf9300e00 {
        decode_LDRSHT(opcode)
    } else if (opcode & 0xfff0f000) == 0xf990f000 {
        decode_PLI_lit_imm_t1(opcode)
    } else if (opcode & 0xfff00f00) == 0xf8500e00 {
        decode_LDRT_t1(opcode)
    } else if (opcode & 0xfff00f00) == 0xf8100e00 {
        decode_LDRBT_t1(opcode)
    } else if (opcode & 0xfff00f00) == 0xf8300e00 {
        decode_LDRHT_t1(opcode)
    } else if (opcode & 0xffef8000) == 0xea6f0000 {
        decode_MVN_reg_t2(opcode)
    } else if (opcode & 0xfff000f0) == 0xfba00000 {
        decode_UMULL_t1(opcode)
    } else if (opcode & 0xfff0f000) == 0xf7f0a000 {
        decode_UDF_t2(opcode)
    } else if (opcode & 0xfff000f0) == 0xfb800000 {
        decode_SMULL_t1(opcode)
    } else if (opcode & 0xfff00f00) == 0xe8500f00 {
        decode_LDREX_t1(opcode)
    } else if (opcode & 0xfff00f00) == 0xf9100e00 {
        decode_LDRSBT_t1(opcode)
    } else if (opcode & 0xfbef8000) == 0xf02f0000 {
        decode_MOV_imm_t2(opcode)
    } else if (opcode & 0xfbe08f00) == 0xf1c00f00 {
        decode_RSB_imm_t2(opcode)
    } else if (opcode & 0xff7f0000) == 0xf85f0000 {
        decode_LDR_lit_t2(opcode)
    } else if (opcode & 0xfbef8000) == 0xf06f0000 {
        decode_MVN_imm_t1(opcode)
    } else if (opcode & 0xff7f0000) == 0xf83f0000 {
        decode_LDRH_lit_t1(opcode)
    } else if (opcode & 0xff7f0000) == 0xf81f0000 {
        decode_LDRB_lit_t1(opcode)
    } else if (opcode & 0xff7f0000) == 0xf91f0000 {
        decode_LDRSB_lit_t1(opcode)
    } else if (opcode & 0xff7f0000) == 0xf93f0000 {
        decode_LDRSH_lit_t1(opcode)
    } else if (opcode & 0xfff08020) == 0xf3600000 {
        decode_BFI_t1(opcode)
    } else if (opcode & 0xfff08020) == 0xf3c00000 {
        decode_UBFX_t1(opcode)
    } else if (opcode & 0xfff08020) == 0xf3400000 {
        decode_SBFX_t1(opcode)
    } else if (opcode & 0xfff00800) == 0xf8400800 {
        decode_STR_imm_t4(opcode)
    } else if (opcode & 0xffd08020) == 0xf3000000 {
        decode_SSAT_t1(opcode)
    } else if (opcode & 0xfff00800) == 0xf8200800 {
        decode_STRH_imm_t3(opcode)
    } else if (opcode & 0xfff00800) == 0xf9100800 {
        decode_LDRSB_imm_t2(opcode)
    } else if (opcode & 0xfff08000) == 0xebb00000 {
        decode_CMP_reg_t3(opcode)
    } else if (opcode & 0xfff00800) == 0xf8000800 {
        decode_STRB_imm_t3(opcode)
    } else if (opcode & 0xfe5f0000) == 0xe85f0000 {
        decode_LDRD_lit_t1(opcode)
    } else if (opcode & 0xffd0a000) == 0xe8800000 {
        decode_STM_t2(opcode)
    } else if (opcode & 0xfff00800) == 0xf9300800 {
        decode_LDRSH_imm_t2(opcode)
    } else if (opcode & 0xfff00800) == 0xf8500800 {
        decode_LDR_imm_t4(opcode)
    } else if (opcode & 0xfff00800) == 0xf8100800 {
        decode_LDRB_imm_t3(opcode)
    } else if (opcode & 0xffd08020) == 0xf3800000 {
        decode_USAT_t1(opcode)
    } else if (opcode & 0xfff00800) == 0xf8300800 {
        decode_LDRH_imm_t3(opcode)
    } else if (opcode & 0xffd0a000) == 0xe9000000 {
        decode_STMDB_t1(opcode)
    } else if (opcode & 0xffe08000) == 0xea800000 {
        decode_EOR_reg_t2(opcode)
    } else if (opcode & 0xfbf08000) == 0xf2c00000 {
        decode_MOVT_t1(opcode)
    } else if (opcode & 0xfbf08000) == 0xf1000000 {
        decode_ADD_imm_t3(opcode)
    } else if (opcode & 0xffd02000) == 0xe9100000 {
        decode_LDMDB_t1(opcode)
    } else if (opcode & 0xffe08000) == 0xea600000 {
        decode_ORN_reg_t2(opcode)
    } else if (opcode & 0xffe08000) == 0xeba00000 {
        decode_SUB_reg_t2(opcode)
    } else if (opcode & 0xfff00000) == 0xf8900000 {
        decode_LDRB_imm_t2(opcode)
    } else if (opcode & 0xffe08000) == 0xea400000 {
        decode_ORR_reg_t2(opcode)
    } else if (opcode & 0xfbf08000) == 0xf0100000 {
        decode_BIC_imm_t1(opcode)
    } else if (opcode & 0xfff00000) == 0xe8400000 {
        decode_STREX_t1(opcode)
    } else if (opcode & 0xffe08000) == 0xebc00000 {
        decode_RSB_reg_t2(opcode)
    } else if (opcode & 0xfff00000) == 0xf9900000 {
        decode_LDRSB_imm_t1(opcode)
    } else if (opcode & 0xfbf08000) == 0xf2400000 {
        decode_MOV_imm_t3(opcode)
    } else if (opcode & 0xffe08000) == 0xeb600000 {
        decode_SBC_reg_t2(opcode)
    } else if (opcode & 0xfff00000) == 0xf8c00000 {
        decode_STR_imm_t3(opcode)
    } else if (opcode & 0xfff00000) == 0xfc400000 {
        decode_MCRR2_t2(opcode)
    } else if (opcode & 0xfbf08000) == 0xf2000000 {
        decode_ADD_imm_t4(opcode)
    } else if (opcode & 0xfe1f0000) == 0xec1f0000 {
        decode_LDC_lit_t1(opcode)
    } else if (opcode & 0xfbf08000) == 0xf0200000 {
        decode_ORR_imm_t1(opcode)
    } else if (opcode & 0xfe1f0000) == 0xfc1f0000 {
        decode_LDC2_lit_t2(opcode)
    } else if (opcode & 0xffe08000) == 0xeb000000 {
        decode_ADD_reg_t3(opcode)
    } else if (opcode & 0xfff00000) == 0xf8d00000 {
        decode_LDR_imm_t3(opcode)
    } else if (opcode & 0xfff00000) == 0xec400000 {
        decode_MCRR_t1(opcode)
    } else if (opcode & 0xfff00000) == 0xf9b00000 {
        decode_LDRSH_imm_t1(opcode)
    } else if (opcode & 0xffe08000) == 0xea000000 {
        decode_AND_reg_t2(opcode)
    } else if (opcode & 0xffe08000) == 0xeb400000 {
        decode_ADC_reg_t2(opcode)
    } else if (opcode & 0xffe08000) == 0xea200000 {
        decode_BIC_reg_t2(opcode)
    } else if (opcode & 0xffd02000) == 0xe8900000 {
        decode_LDM_t2(opcode)
    } else if (opcode & 0xfff00000) == 0xf8800000 {
        decode_STRB_imm_t2(opcode)
    } else if (opcode & 0xfff00000) == 0xf8b00000 {
        decode_LDRH_imm_t2(opcode)
    } else if (opcode & 0xfff00000) == 0xf8a00000 {
        decode_STRH_imm_t2(opcode)
    } else if (opcode & 0xfbf08000) == 0xf2a00000 {
        decode_SUB_imm_t4(opcode)
    } else if (opcode & 0xfbe08000) == 0xf0600000 {
        decode_ORN_imm_t1(opcode)
    } else if (opcode & 0xfbe08000) == 0xf1a00000 {
        decode_SUB_imm_t3(opcode)
    } else if (opcode & 0xfbe08000) == 0xf0800000 {
        decode_EOR_imm_t1(opcode)
    } else if (opcode & 0xfbe08000) == 0xf0000000 {
        decode_AND_imm_t1(opcode)
    } else if (opcode & 0xfbe08000) == 0xf1400000 {
        decode_ADC_imm_t1(opcode)
    } else if (opcode & 0xfbe08000) == 0xf1600000 {
        decode_SBC_imm_t1(opcode)
    } else if (opcode & 0xff100010) == 0xfe000000 {
        decode_MCR2_t2(opcode)
    } else if (opcode & 0xff100010) == 0xfe100010 {
        decode_MRC2_t2(opcode)
    } else if (opcode & 0xff100010) == 0xee000000 {
        decode_MCR_t1(opcode)
    } else if (opcode & 0xff100010) == 0xee100010 {
        decode_MRC_t1(opcode)
    } else if (opcode & 0xfe500000) == 0xe8500000 {
        decode_LDRD_imm_t1(opcode)
    } else if (opcode & 0xff000010) == 0xfe000000 {
        decode_CDP2_t2(opcode)
    } else if (opcode & 0xff000010) == 0xee000000 {
        decode_CDP_t1(opcode)
    } else if (opcode & 0xfe500000) == 0xe8400000 {
        decode_STRD_imm_t1(opcode)
    } else if (opcode & 0xf800d000) == 0xf000d000 {
        decode_BL_t1(opcode)
    } else if (opcode & 0xfe100000) == 0xfc100000 {
        decode_LDC2_imm_t2(opcode)
    } else if (opcode & 0xfe100000) == 0xec000000 {
        decode_STC_t1(opcode)
    } else if (opcode & 0xf800d000) == 0xf0009000 {
        decode_B_t4(opcode)
    } else if (opcode & 0xf800d000) == 0xf0008000 {
        decode_B_t3(opcode)
    } else if (opcode & 0xfe100000) == 0xfc000000 {
        decode_STC2_t2(opcode)
    } else if (opcode & 0xfe100000) == 0xec100000 {
        decode_LDC_imm_t1(opcode)
    } else {
        decode_UDF_t2(opcode)
    }
}

#[cfg(test)]
mod tests {

    use core::register::Reg;

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
            Instruction::LDR_lit { rt, imm32, thumb32 } => {
                assert!(rt == Reg::R1);
                assert!(imm32 == (7 << 2));
                assert!(thumb32 == false);
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
                thumb32,
            } => {
                assert!(rd == Reg::SP);
                assert!(rn == Reg::SP);
                assert!(imm32 == 0x8);
                assert!(setflags == false);
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
            Instruction::SUB_imm {
                rd,
                rn,
                imm32,
                setflags,
                thumb32,
            } => {
                assert!(rd == Reg::R2);
                assert!(rn == Reg::R2);
                assert!(imm32 == 48);
                assert!(setflags == true);
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
        assert_eq!(decode_16(0xbeab), Instruction::BKPT { imm32: 0xab });
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
            Instruction::ORR_reg {
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
            Instruction::STM {
                rn,
                registers,
                wback,
            } => {
                assert!(rn == Reg::R2);
                let elems: Vec<_> = registers.iter().collect();
                assert_eq!(vec![Reg::R0, Reg::R1], elems);
                assert!(wback);
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
            } => {
                assert!(rn == Reg::R3);
                let elems: Vec<_> = registers.iter().collect();
                assert_eq!(vec![Reg::R0, Reg::R1, Reg::R2], elems);
                assert!(wback);
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
        assert_eq!(
            decode_16(0x4242),
            Instruction::RSB_imm {
                rd: Reg::R2,
                rn: Reg::R0,
                imm32: 0,
                setflags: true,
            }
        );
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
                thumb32: true
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
                thumb32: true
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
                nonzero: false
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
                mask: 0x4
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

}
