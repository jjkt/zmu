use crate::core::bits::*;
use crate::core::instruction::Instruction;
use crate::core::ThumbCode;

#[cfg(test)]
use crate::core::instruction::CpsEffect;
#[cfg(test)]
use crate::core::register::SpecialReg;

#[cfg(test)]
use crate::core::condition::Condition;
#[cfg(test)]
use crate::core::instruction::ITCondition;

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
mod smla;
mod smlal;
mod smul;
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

use crate::decoder::adc::*;
use crate::decoder::add::*;
use crate::decoder::adr::*;
use crate::decoder::and::*;
use crate::decoder::asr::*;

use crate::decoder::b::*;
use crate::decoder::bic::*;
use crate::decoder::bkpt::*;
use crate::decoder::bl::*;
use crate::decoder::blx::*;
use crate::decoder::bx::*;

use crate::decoder::cbz::*;
use crate::decoder::clz::*;
use crate::decoder::cmn::*;
use crate::decoder::cmp::*;
use crate::decoder::cpd::*;
use crate::decoder::cps::*;

use crate::decoder::dmb::*;
use crate::decoder::dsb::*;

use crate::decoder::eor::*;

use crate::decoder::isb::*;
use crate::decoder::it::*;

use crate::decoder::ldc::*;
use crate::decoder::ldm::*;
use crate::decoder::ldr::*;
use crate::decoder::ldrb::*;
use crate::decoder::ldrh::*;
use crate::decoder::ldrsb::*;
use crate::decoder::ldrsh::*;
use crate::decoder::lsl::*;
use crate::decoder::lsr::*;

use crate::decoder::mcr::*;
use crate::decoder::mla::*;
use crate::decoder::mls::*;
use crate::decoder::mov::*;
use crate::decoder::mrs::*;
use crate::decoder::msr::*;
use crate::decoder::mul::*;
use crate::decoder::mvn::*;

use crate::decoder::nop::*;

use crate::decoder::orn::*;
use crate::decoder::orr::*;

use crate::decoder::pld::*;
use crate::decoder::pli::*;
use crate::decoder::pop::*;
use crate::decoder::push::*;

use crate::decoder::rbit::*;
use crate::decoder::rev::*;
use crate::decoder::ror::*;
use crate::decoder::rrx::*;
use crate::decoder::rsb::*;

use crate::decoder::sbc::*;
use crate::decoder::sdiv::*;
use crate::decoder::smla::*;
use crate::decoder::smlal::*;
use crate::decoder::smul::*;
use crate::decoder::smull::*;
use crate::decoder::stc::*;
use crate::decoder::stm::*;
use crate::decoder::str::*;
use crate::decoder::strex::*;
use crate::decoder::sub::*;
use crate::decoder::sxt::*;

use crate::decoder::tbb::*;
use crate::decoder::tbh::*;
use crate::decoder::teq::*;
use crate::decoder::tst::*;

use crate::decoder::udiv::*;
use crate::decoder::umlal::*;
use crate::decoder::umull::*;
use crate::decoder::uxt::*;

use crate::decoder::bfc::*;
use crate::decoder::bfi::*;
use crate::decoder::clrex::*;
use crate::decoder::dbg::*;
use crate::decoder::movt::*;
use crate::decoder::sbfx::*;
use crate::decoder::sev::*;
use crate::decoder::ssat::*;
use crate::decoder::ubfx::*;
use crate::decoder::usat::*;
use crate::decoder::wfe::*;
use crate::decoder::wfi::*;
use crate::decoder::yield_::*;

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

#[allow(clippy::cyclomatic_complexity)]
pub fn decode_16(opcode: u16) -> Instruction {
    if opcode == 0xbf20 {
        decode_WFE_t1(opcode)
    } else if opcode == 0xbf40 {
        decode_SEV_t1(opcode)
    } else if opcode == 0xbf00 {
        decode_NOP_t1(opcode)
    } else if opcode == 0xbf10 {
        decode_YIELD_t1(opcode)
    } else if opcode == 0xbf30 {
        decode_WFI_t1(opcode)
    } else if (opcode & 0xffef) == 0xb662 {
        decode_CPS_t1(opcode)
    } else if (opcode & 0xff87) == 0x4485 {
        decode_ADD_reg_sp_t2(opcode)
    } else if (opcode & 0xff87) == 0x4700 {
        decode_BX_t1(opcode)
    } else if (opcode & 0xff78) == 0x4468 {
        decode_ADD_reg_sp_t1(opcode)
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
        decode_ADD_reg_t2(opcode)
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

#[allow(clippy::cyclomatic_complexity, clippy::unreadable_literal)]
pub fn decode_32(opcode: u32) -> Instruction {
    if opcode == 0xf3af8000 {
        decode_NOP_t2(opcode)
    } else if opcode == 0xf3af80f0 {
        decode_DBG_t1(opcode)
    } else if opcode == 0xf3af8004 {
        decode_SEV_t2(opcode)
    } else if opcode == 0xf3af8001 {
        decode_YIELD_t2(opcode)
    } else if opcode == 0xf3af8002 {
        decode_WFE_t2(opcode)
    } else if opcode == 0xf3af8003 {
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
        decode_MOV_reg_t3(opcode)
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
    } else if (opcode & 0xfff00fc0) == 0xf9300000 {
        decode_LDRSH_reg_t2(opcode)
    } else if (opcode & 0xfff00fc0) == 0xf8000000 {
        decode_STRB_reg_t2(opcode)
    } else if (opcode & 0xfff00fc0) == 0xf8500000 {
        decode_LDR_reg_t2(opcode)
    } else if (opcode & 0xfff00fc0) == 0xf9100000 {
        decode_LDRSB_reg_t2(opcode)
    } else if (opcode & 0xffffa000) == 0xe92d0000 {
        decode_PUSH_t2(opcode)
    } else if (opcode & 0xfff00fc0) == 0xf8400000 {
        decode_STR_reg_t2(opcode)
    } else if (opcode & 0xfff0f0c0) == 0xfb10f000 {
        decode_SMUL_t1(opcode)
    } else if (opcode & 0xfff00fc0) == 0xf8300000 {
        decode_LDRH_reg_t2(opcode)
    } else if (opcode & 0xffef8030) == 0xea4f0030 {
        decode_ROR_imm_t1(opcode)
    } else if (opcode & 0xffff8020) == 0xf36f0000 {
        decode_BFC_t1(opcode)
    } else if (opcode & 0xfff00fc0) == 0xf8200000 {
        decode_STRH_reg_t2(opcode)
    } else if (opcode & 0xffef8030) == 0xea4f0020 {
        decode_ASR_imm_t2(opcode)
    } else if (opcode & 0xffef8030) == 0xea4f0010 {
        decode_LSR_imm_t2(opcode)
    } else if (opcode & 0xffef8030) == 0xea4f0000 {
        decode_LSL_imm_t2(opcode)
    } else if (opcode & 0xffff2000) == 0xe8bd0000 {
        decode_POP_t2(opcode)
    } else if (opcode & 0xfff08f00) == 0xebb00f00 {
        decode_CMP_reg_t3(opcode)
    } else if (opcode & 0xfff00f80) == 0xf8100000 {
        decode_LDRB_reg_t2(opcode)
    } else if (opcode & 0xfff08f00) == 0xea100f00 {
        decode_TST_reg_t2(opcode)
    } else if (opcode & 0xfff08f00) == 0xeb100f00 {
        decode_CMN_reg_t2(opcode)
    } else if (opcode & 0xfff08f00) == 0xea900f00 {
        decode_TEQ_reg_t1(opcode)
    } else if (opcode & 0xfff00f00) == 0xe8500f00 {
        decode_LDREX_t1(opcode)
    } else if (opcode & 0xfbf08f00) == 0xf1100f00 {
        decode_CMN_imm_t1(opcode)
    } else if (opcode & 0xfbff8000) == 0xf20f0000 {
        decode_ADR_t3(opcode)
    } else if (opcode & 0xfbff8000) == 0xf2ad0000 {
        decode_SUB_SP_imm_t3(opcode)
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
    } else if (opcode & 0xfff00f00) == 0xf9100e00 {
        decode_LDRSBT_t1(opcode)
    } else if (opcode & 0xfbef8000) == 0xf04f0000 {
        decode_MOV_imm_t2(opcode)
    } else if (opcode & 0xff7f0000) == 0xf85f0000 {
        decode_LDR_lit_t2(opcode)
    } else if (opcode & 0xff7f0000) == 0xf83f0000 {
        decode_LDRH_lit_t1(opcode)
    } else if (opcode & 0xfbef8000) == 0xf1ad0000 {
        decode_SUB_SP_imm_t2(opcode)
    } else if (opcode & 0xfbef8000) == 0xf06f0000 {
        decode_MVN_imm_t1(opcode)
    } else if (opcode & 0xff7f0000) == 0xf81f0000 {
        decode_LDRB_lit_t1(opcode)
    } else if (opcode & 0xff7f0000) == 0xf91f0000 {
        decode_LDRSB_lit_t1(opcode)
    } else if (opcode & 0xff7f0000) == 0xf93f0000 {
        decode_LDRSH_lit_t1(opcode)
    } else if (opcode & 0xfff08020) == 0xf3600000 {
        decode_BFI_t1(opcode)
    } else if (opcode & 0xfff000c0) == 0xfb100000 {
        decode_SMLA_t1(opcode)
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
    } else if (opcode & 0xffd0a000) == 0xe8800000 {
        decode_STM_t2(opcode)
    } else if (opcode & 0xfff00800) == 0xf8000800 {
        decode_STRB_imm_t3(opcode)
    } else if (opcode & 0xfe5f0000) == 0xe85f0000 {
        decode_LDRD_lit_t1(opcode)
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
    } else if (opcode & 0xffe08000) == 0xeba00000 {
        decode_SUB_reg_t2(opcode)
    } else if (opcode & 0xffe08000) == 0xebc00000 {
        decode_RSB_reg_t1(opcode)
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
    } else if (opcode & 0xffe08000) == 0xea400000 {
        decode_ORR_reg_t2(opcode)
    } else if (opcode & 0xfff00000) == 0xe8400000 {
        decode_STREX_t1(opcode)
    } else if (opcode & 0xfe1f0000) == 0xfc1f0000 {
        decode_LDC2_lit_t2(opcode)
    } else if (opcode & 0xffe08000) == 0xeb000000 {
        decode_ADD_reg_t3(opcode)
    } else if (opcode & 0xfff00000) == 0xf8d00000 {
        decode_LDR_imm_t3(opcode)
    } else if (opcode & 0xfff00000) == 0xf8900000 {
        decode_LDRB_imm_t2(opcode)
    } else if (opcode & 0xffd02000) == 0xe8900000 {
        decode_LDM_t2(opcode)
    } else if (opcode & 0xfff00000) == 0xec400000 {
        decode_MCRR_t1(opcode)
    } else if (opcode & 0xfff00000) == 0xf9b00000 {
        decode_LDRSH_imm_t1(opcode)
    } else if (opcode & 0xffe08000) == 0xea000000 {
        decode_AND_reg_t2(opcode)
    } else if (opcode & 0xffe08000) == 0xeb400000 {
        decode_ADC_reg_t2(opcode)
    } else if (opcode & 0xffd02000) == 0xe9100000 {
        decode_LDMDB_t1(opcode)
    } else if (opcode & 0xffe08000) == 0xea600000 {
        decode_ORN_reg_t2(opcode)
    } else if (opcode & 0xfff00000) == 0xf8800000 {
        decode_STRB_imm_t2(opcode)
    } else if (opcode & 0xfff00000) == 0xf8b00000 {
        decode_LDRH_imm_t2(opcode)
    } else if (opcode & 0xffe08000) == 0xea200000 {
        decode_BIC_reg_t2(opcode)
    } else if (opcode & 0xfff00000) == 0xf8a00000 {
        decode_STRH_imm_t2(opcode)
    } else if (opcode & 0xfbf08000) == 0xf2a00000 {
        decode_SUB_imm_t4(opcode)
    } else if (opcode & 0xfbe08000) == 0xf0600000 {
        decode_ORN_imm_t1(opcode)
    } else if (opcode & 0xfbe08000) == 0xf1a00000 {
        decode_SUB_imm_t3(opcode)
    } else if (opcode & 0xfbe08000) == 0xf1c00000 {
        decode_RSB_imm_t2(opcode)
    } else if (opcode & 0xfbe08000) == 0xf0800000 {
        decode_EOR_imm_t1(opcode)
    } else if (opcode & 0xfbe08000) == 0xf0200000 {
        decode_BIC_imm_t1(opcode)
    } else if (opcode & 0xfbe08000) == 0xf1000000 {
        decode_ADD_imm_t3(opcode)
    } else if (opcode & 0xfbe08000) == 0xf0400000 {
        decode_ORR_imm_t1(opcode)
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
    } else if (opcode & 0xfe500000) == 0xe8400000 {
        decode_STRD_imm_t1(opcode)
    } else if (opcode & 0xff000010) == 0xee000000 {
        decode_CDP_t1(opcode)
    } else if (opcode & 0xf800d000) == 0xf000d000 {
        decode_BL_t1(opcode)
    } else if (opcode & 0xfe100000) == 0xfc100000 {
        decode_LDC2_imm_t2(opcode)
    } else if (opcode & 0xf800d000) == 0xf0009000 {
        decode_B_t4(opcode)
    } else if (opcode & 0xfe100000) == 0xec000000 {
        decode_STC_t1(opcode)
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
mod tests;
