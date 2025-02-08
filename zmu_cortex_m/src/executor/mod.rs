//!
//! Functionality for running instructions on a Processor.
//!

use crate::core::bits::Bits;
use crate::core::condition::Condition;
use crate::core::exception::{Exception, ExceptionHandling};
use crate::core::fault::Fault;
use crate::core::instruction::{Imm32Carry, Instruction, SetFlags};

use crate::core::operation::condition_test;
use crate::core::register::{Apsr, BaseReg};
use crate::memory::map::MapMemory;
use crate::peripheral::{dwt::Dwt, systick::SysTick};

use crate::Processor;

mod branch;
mod coproc;
mod divide;
mod exception;
mod load_and_store;
mod load_and_store_multiple;
mod misc;
mod misc_data_processing;
mod multiply;
mod packing;
mod parallel_add;
mod shift;
mod signed_multiply;
mod status_register;
mod std_data_processing;

mod fp_load_and_store;
mod fp_register_transfer;
mod fp_data_processing;
mod fp_generic;

use branch::IsaBranch;
use coproc::IsaCoprocessor;
use divide::IsaDivide;
use exception::IsaException;
use load_and_store::IsaLoadAndStore;
use load_and_store_multiple::IsaLoadAndStoreMultiple;
use misc::IsaMisc;
use misc_data_processing::IsaMiscDataProcessing;
use multiply::IsaMultiply;
use packing::IsaPacking;
use parallel_add::IsaParallelAddSub;
use shift::IsaShift;
use signed_multiply::IsaSignedMultiply;
use status_register::IsaStatusRegister;
use std_data_processing::IsaStandardDataProcessing;

use fp_load_and_store::IsaFloatingPointLoadAndStore;
use fp_register_transfer::IsaFloatingPointRegisterTransfer;
use fp_data_processing::IsaFloatingPointDataProcessing;

///
/// Stepping processor with instructions
///
pub trait Executor {
    ///
    /// Run processor forward. Simulates core + peripherals.
    ///
    fn step(&mut self);

    ///
    /// Run processor forward with core sleeping (peripherals only)
    ///
    fn step_sleep(&mut self);

    ///
    /// Execute given instruction. Returns number of clock cycles burn.
    ///
    fn execute(&mut self, instruction: &Instruction, instruction_size: usize) -> u32;
}

type ExecuteResult = Result<ExecuteSuccess, Fault>;
#[derive(PartialEq, Debug, Copy, Clone)]
/// Succesfull execution  an instruction
pub enum ExecuteSuccess {
    /// The instruction was taken normally
    Taken {
        /// Number of clock cycles taken for the operation
        cycles: u32,
    },
    /// The instruction was not taken as the condition did not pass
    NotTaken,
    /// The execution branched to a new address, pc was set accordingly
    Branched {
        /// Number of clock cycles taken for the operation
        cycles: u32,
    },
}

trait ExecutorHelper {
    fn condition_passed(&self) -> bool;
    fn condition_passed_b(&self, cond: Condition) -> bool;
    fn integer_zero_divide_trapping_enabled(&self) -> bool;
    fn set_itstate(&mut self, state: u8);
    fn it_advance(&mut self);
    fn in_it_block(&self) -> bool;
    #[allow(dead_code)]
    fn last_in_it_block(&self) -> bool;
    fn execute_internal(&mut self, instruction: &Instruction) -> ExecuteResult;
    fn update_flags_check_it_block(
        &mut self,
        setflags: SetFlags,
        result: u32,
        carry: bool,
        overflow: bool,
    );
}

#[inline(always)]
fn resolve_addressing(rn: u32, imm32: u32, add: bool, index: bool) -> (u32, u32) {
    let offset_address = if add { rn + imm32 } else { rn - imm32 };
    let address = if index { offset_address } else { rn };
    (address, offset_address)
}

fn expand_conditional_carry(imm32: &Imm32Carry, carry: bool) -> (u32, bool) {
    match imm32 {
        Imm32Carry::NoCarry { imm32 } => (*imm32, carry),
        Imm32Carry::Carry { imm32_c0, imm32_c1 } => {
            if carry {
                *imm32_c1
            } else {
                *imm32_c0
            }
        }
    }
}

fn conditional_setflags(setflags: SetFlags, in_it_block: bool) -> bool {
    match setflags {
        SetFlags::True => true,
        SetFlags::False => false,
        SetFlags::NotInITBlock => !in_it_block,
    }
}

impl ExecutorHelper for Processor {
    fn set_itstate(&mut self, state: u8) {
        self.itstate = state;
    }

    fn it_advance(&mut self) {
        if self.itstate != 0 {
            if self.itstate.get_bits(0..3) == 0 {
                self.itstate = 0;
            } else {
                let it = self.itstate.get_bits(0..5);
                self.itstate.set_bits(0..5, (it << 1) & 0b11111);
            }
        }
    }

    fn in_it_block(&self) -> bool {
        self.itstate.get_bits(0..4) != 0
    }

    fn last_in_it_block(&self) -> bool {
        self.itstate.get_bits(0..4) == 0b1000
    }
    fn integer_zero_divide_trapping_enabled(&self) -> bool {
        true
    }

    #[inline(always)]
    fn condition_passed(&self) -> bool {
        let itstate = self.itstate;

        if itstate == 0 {
            true
        } else {
            let cond = u16::from(itstate.get_bits(4..8));
            condition_test(
                Condition::from_u16(cond).unwrap_or(Condition::AL),
                &self.psr,
            )
        }
    }

    fn condition_passed_b(&self, cond: Condition) -> bool {
        condition_test(cond, &self.psr)
    }

    fn update_flags_check_it_block(
        &mut self,
        setflags: SetFlags,
        result: u32,
        carry: bool,
        overflow: bool,
    ) {
        if conditional_setflags(setflags, self.in_it_block()) {
            self.psr.set_n(result);
            self.psr.set_z(result);
            self.psr.set_c(carry);
            self.psr.set_v(overflow);
        }
    }

    #[allow(clippy::too_many_lines)]
    fn execute_internal(&mut self, instruction: &Instruction) -> ExecuteResult {
        match instruction {
            // --------------------------------------------
            //
            // Group: Branch instructions
            //
            // --------------------------------------------
            Instruction::B_t13 { params, .. } => self.exec_b_t13(*params),
            Instruction::B_t24 { imm32, .. } => self.exec_b_t24(*imm32),

            Instruction::BLX { rm } => self.exec_blx(*rm),
            Instruction::BX { rm } => self.exec_bx(*rm),
            Instruction::BL { imm32 } => self.exec_bl(*imm32),

            Instruction::CBZ { params } => self.exec_cbz(*params),
            Instruction::CBNZ { params } => self.exec_cbnz(*params),

            Instruction::TBB { params } => self.exec_tbb(*params),
            Instruction::TBH { params } => self.exec_tbh(*params),

            // --------------------------------------------
            //
            // Group: Standard data-processing instructions
            //
            // --------------------------------------------
            Instruction::ADD_reg { params, .. } => self.exec_add_reg(params),
            Instruction::ADD_imm { params, .. } => self.exec_add_imm(params),
            Instruction::ADD_sp_reg { params, .. } => self.exec_add_sp_reg(params),

            Instruction::ADC_reg { params, .. } => self.exec_adc_reg(params),
            Instruction::ADC_imm { params } => self.exec_adc_imm(params),

            Instruction::ADR { params, .. } => self.exec_adr(*params),

            Instruction::AND_reg { params, .. } => self.exec_and_reg(params),
            Instruction::AND_imm { params } => self.exec_and_imm(params),

            Instruction::BIC_imm { params } => self.exec_bic_imm(params),
            Instruction::BIC_reg { params, .. } => self.exec_bic_reg(params),

            Instruction::CMN_reg { params, .. } => self.exec_cmn_reg(params),
            Instruction::CMN_imm { params } => self.exec_cmn_imm(*params),

            Instruction::CMP_reg { params, .. } => self.exec_cmp_reg(params),
            Instruction::CMP_imm { params, .. } => self.exec_cmp_imm(*params),

            Instruction::EOR_reg { params, .. } => self.exec_eor_reg(params),
            Instruction::EOR_imm { params } => self.exec_eor_imm(params),

            Instruction::MOV_reg { params, .. } => self.exec_mov_reg(params),
            Instruction::MOV_imm { params, .. } => self.exec_mov_imm(params),

            Instruction::MVN_reg { params, .. } => self.exec_mvn_reg(params),
            Instruction::MVN_imm { params } => self.exec_mvn_imm(params),

            Instruction::ORN_reg { params } => self.exec_orn_reg(params),

            Instruction::ORR_imm { params } => self.exec_orr_imm(params),
            Instruction::ORR_reg { params, .. } => self.exec_orr_reg(params),

            Instruction::RSB_reg { params, .. } => self.exec_rsb_reg(params),
            Instruction::RSB_imm { params, .. } => self.exec_rsb_imm(params),

            Instruction::SBC_reg { params, .. } => self.exec_sbc_reg(params),
            Instruction::SBC_imm { params } => self.exec_sbc_imm(params),

            Instruction::SUB_reg { params, .. } => self.exec_sub_reg(params),
            Instruction::SUB_imm { params, .. } => self.exec_sub_imm(params),

            Instruction::TEQ_reg { params } => self.exec_teq_reg(params),
            Instruction::TEQ_imm { params } => self.exec_teq_imm(params),

            Instruction::TST_reg { params, .. } => self.exec_tst_reg(params),
            Instruction::TST_imm { params } => self.exec_tst_imm(params),

            // --------------------------------------------
            //
            // Group: Shift instructions
            //
            // --------------------------------------------
            Instruction::ASR_imm { params, .. } => self.exec_asr_imm(params),
            Instruction::ASR_reg { params, .. } => self.exec_asr_reg(params),

            Instruction::LSL_imm { params, .. } => self.exec_lsl_imm(params),
            Instruction::LSL_reg { params, .. } => self.exec_lsl_reg(params),

            Instruction::LSR_imm { params, .. } => self.exec_lsr_imm(params),
            Instruction::LSR_reg { params, .. } => self.exec_lsr_reg(params),

            Instruction::ROR_imm { params } => self.exec_ror_imm(params),
            Instruction::ROR_reg { params, .. } => self.exec_ror_reg(params),

            Instruction::RRX { params } => self.exec_rrx(params),
            // --------------------------------------------
            //
            // Group: Multiply instructions
            //
            // --------------------------------------------
            Instruction::MLA { params } => self.exec_mla(params),
            Instruction::MLS { params } => self.exec_mls(params),
            Instruction::MUL { params, .. } => self.exec_mul(params),
            // --------------------------------------------
            //
            // Group: Signed multiply instructions (ArmV7-m)
            //
            // --------------------------------------------
            Instruction::SMLAL { params: _ } => unimplemented!(),
            Instruction::SMULL { params } => self.exec_smull(params),

            // --------------------------------------------
            //
            // Group: Unsigned Multiply instructions (ARMv7-M base architecture)
            //
            // --------------------------------------------
            Instruction::UMLAL { params } => self.exec_umlal(params),
            Instruction::UMULL { params } => self.exec_umull(params),

            // --------------------------------------------
            //
            // Group: Signed Multiply instructions (ARMv7-M DSP extension)
            //
            // --------------------------------------------
            Instruction::SMUL { params } => self.exec_smul(params),
            Instruction::SMLA { params } => self.exec_smla(params),

            // --------------------------------------------
            //
            // Group: Saturating instructions (ARMv7-M base arch)
            //
            // --------------------------------------------

            // --------------------------------------------
            //
            // Group: Saturating instructions (ARMv7-M DSP extensions)
            //
            // --------------------------------------------

            // --------------------------------------------
            //
            // Group: Saturating add/sub (ARMv7-M DSP extensions)
            //
            // --------------------------------------------

            // --------------------------------------------
            //
            // Group: Packing and unpacking instructions
            //
            // --------------------------------------------
            Instruction::SXTB { params, .. } => self.exec_sxtb(params),
            Instruction::SXTH { params, .. } => self.exec_sxth(params),

            Instruction::UXTB { params, .. } => self.exec_uxtb(params),
            Instruction::UXTH { params, .. } => self.exec_uxth(params),

            // --------------------------------------------
            //
            // Group: Packing and unpacking instructions (DSP extensions)
            //
            // --------------------------------------------
            Instruction::UXTAB { params } => self.exec_uxtab(params),

            // --------------------------------------------
            //
            // Group: Divide instructions
            //
            // --------------------------------------------
            // ARMv7-M
            Instruction::SDIV { params } => self.exec_sdiv(params),
            Instruction::UDIV { params } => self.exec_udiv(params),

            // --------------------------------------------
            //
            // Group: Parallel add / sub (DSP extension)
            //
            // --------------------------------------------
            Instruction::UADD8 { params } => self.exec_uadd8(params),

            // --------------------------------------------
            //
            // Group:  Miscellaneous data-processing instructions
            //
            // --------------------------------------------
            Instruction::BFC { params } => self.exec_bfc(params),
            Instruction::BFI { params } => self.exec_bfi(params),

            Instruction::CLZ { params } => self.exec_clz(*params),

            Instruction::MOVT { params } => self.exec_movt(*params),

            Instruction::REV { params, .. } => self.exec_rev(*params),
            Instruction::REV16 { params, .. } => self.exec_rev16(*params),
            Instruction::REVSH { params, .. } => self.exec_revsh(*params),

            Instruction::UBFX { params } => self.exec_ubfx(params),
            Instruction::SBFX { params } => self.exec_sbfx(params),

            // --------------------------------------------
            //
            // Group:  Miscellaneous data-processing instructions (DSP extensions)
            //
            // --------------------------------------------
            Instruction::SEL { params } => self.exec_sel(params),
            // --------------------------------------------
            //
            // Group: Status register access instructions
            //
            // --------------------------------------------
            Instruction::MRS { params } => self.exec_mrs(*params),
            Instruction::MSR_reg { params } => self.exec_msr(*params),

            #[cfg(feature = "armv6m")]
            Instruction::CPS { im } => self.exec_cps(*im),

            #[cfg(any(feature = "armv7m", feature = "armv7em"))]
            Instruction::CPS {
                im,
                affect_pri,
                affect_fault,
            } => self.exec_cps(*im, *affect_pri, *affect_fault),

            // --------------------------------------------
            //
            // Group:  Load and Store instructions
            //
            // --------------------------------------------
            Instruction::LDREX { params } => self.exec_ldrex(*params),
            Instruction::LDREXB { params } => self.exec_ldrexb(*params),
            Instruction::LDREXH { params } => self.exec_ldrexh(*params),

            Instruction::LDR_imm { params, .. } => self.exec_ldr_imm(params),
            Instruction::LDRB_imm { params, .. } => self.exec_ldrb_imm(params),
            Instruction::LDRH_imm { params, .. } => self.exec_ldrh_imm(params),
            Instruction::LDRSB_imm { params, .. } => self.exec_ldrsb_imm(params),
            Instruction::LDRSH_imm { params, .. } => self.exec_ldrsh_imm(params),

            Instruction::LDR_reg { params, .. } => self.exec_ldr_reg(params),
            Instruction::LDRB_reg { params, .. } => self.exec_ldrb_reg(params),
            Instruction::LDRH_reg { params, .. } => self.exec_ldrh_reg(params),
            Instruction::LDRSB_reg { params, .. } => self.exec_ldrsb_reg(params),
            Instruction::LDRSH_reg { params, .. } => self.exec_ldrsh_reg(params),

            Instruction::STR_imm { params, .. } => self.exec_str_imm(params),
            Instruction::STRB_imm { params, .. } => self.exec_strb_imm(params),
            Instruction::STRH_imm { params, .. } => self.exec_strh_imm(params),

            Instruction::STR_reg { params, .. } => self.exec_str_reg(params),
            Instruction::STRB_reg { params, .. } => self.exec_strb_reg(params),
            Instruction::STRH_reg { params, .. } => self.exec_strh_reg(params),

            Instruction::STREX { params } => self.exec_strex(*params),
            Instruction::STREXB { params } => self.exec_strexb(*params),
            Instruction::STREXH { params } => self.exec_strexh(*params),

            Instruction::STRD_imm { params } => self.exec_strd_imm(params),
            Instruction::LDRD_imm { params } => self.exec_ldrd_imm(params),

            Instruction::LDR_lit { params, .. } => self.exec_ldr_lit(params),

            // --------------------------------------------
            //
            // Group:  Load and Store Multiple instructions
            //
            // --------------------------------------------
            Instruction::STM { params, .. } => self.exec_stm(params),
            Instruction::STMDB { params } => self.exec_stmdb(params),
            Instruction::LDM { params, .. } => self.exec_ldm(params),

            Instruction::PUSH { registers, .. } => self.exec_push(*registers),
            Instruction::POP { registers, .. } => self.exec_pop(*registers),

            // --------------------------------------------
            //
            // Group: Miscellaneous
            //
            // --------------------------------------------
            Instruction::DMB => self.exec_dmb(),
            Instruction::DSB => self.exec_dsb(),
            Instruction::ISB => self.exec_isb(),

            Instruction::IT {
                firstcond, mask, ..
            } => self.exec_it(*firstcond, *mask),

            Instruction::NOP { .. } => Ok(ExecuteSuccess::Taken { cycles: 1 }),

            Instruction::PLD_imm { .. } => self.exec_pld_imm(),
            Instruction::PLD_lit { .. } => self.exec_pld_lit(),
            Instruction::PLD_reg { .. } => self.exec_pld_reg(),

            Instruction::SEV { .. } => self.exec_sev(),
            Instruction::WFE { .. } => self.exec_wfe(),
            Instruction::YIELD { .. } => self.exec_yield(),
            Instruction::WFI { .. } => self.exec_wfi(),

            // --------------------------------------------
            //
            // Group: Exception generating instructions
            //
            // --------------------------------------------
            Instruction::SVC { .. } => self.exec_svc(),
            Instruction::BKPT { imm32 } => self.exec_bkpt(*imm32),

            // --------------------------------------------
            //
            // Group: Coprocessor instructions
            //
            // --------------------------------------------
            Instruction::MCR { .. } => self.exec_mcr(),
            Instruction::MCR2 { .. } => self.exec_mcr2(),
            Instruction::LDC_imm { .. } => self.exec_ldc_imm(),
            Instruction::LDC2_imm { .. } => self.exec_ldc2_imm(),
            // --------------------------------------------
            //
            // Group: Floating-point load and store instructions
            //
            // --------------------------------------------
            Instruction::VLDR { params } => self.exec_vldr(params),
            Instruction::VSTR { params } => self.exec_vstr(params),
            Instruction::VSTM_T1 { params } => self.exec_vstm_t1(params),
            Instruction::VSTM_T2 { params } => self.exec_vstm_t2(params),
            Instruction::VPUSH { params } => self.exec_vpush(params),
            Instruction::VPOP { params } => self.exec_vpop(params),
            Instruction::VMOV_imm_32 { params } => self.exec_vmov_imm_32(*params),
            Instruction::VMOV_imm_64 { params } => self.exec_vmov_imm_64(*params),
            Instruction::VMOV_reg_f32 { params } => self.exec_vmov_reg_f32(*params),
            Instruction::VMOV_reg_f64 { params } => self.exec_vmov_reg_f64(*params),
            Instruction::VMOV_cr_scalar { .. } => unimplemented!(),
            Instruction::VMOV_scalar_cr { .. } => unimplemented!(),
            Instruction::VMOV_cr_sp { params } => self.exec_vmov_cr_sp(params),
            Instruction::VMOV_cr2_sp2 { .. } => unimplemented!(),
            Instruction::VMOV_cr2_dp { params } => self.exec_vmov_cr2_dp(params),

            // --------------------------------------------
            //
            // Group: Floating-point register transfer instructions
            //
            // --------------------------------------------

            Instruction::VMRS { rt } => self.exec_vmrs(*rt),

            // --------------------------------------------
            //
            // Group: Floating-point data-processing instructions
            //
            // --------------------------------------------
            Instruction::VABS_f32 { params } => self.exec_vabs_f32(params),
            Instruction::VABS_f64 { params } => self.exec_vabs_f64(params),
            Instruction::VCMP_f32 { params } => self.exec_vcmp_f32(params),
            Instruction::VCMP_f64 { params } => self.exec_vcmp_f64(params),
            Instruction::VADD_f32 { params } => self.exec_vadd_f32(params),
            Instruction::VADD_f64 { params } => self.exec_vadd_f64(params),
            Instruction::VSUB_f32 { params } => self.exec_vsub_f32(params),
            Instruction::VSUB_f64 { params } => self.exec_vsub_f64(params),
            Instruction::VCVT { params } => self.exec_vcvt(params),

            // --------------------------------------------
            //
            // Fallback: unknown instruction
            //
            // --------------------------------------------
            Instruction::UDF { imm32, opcode, .. } => {
                println!("unsupported instruction, opcode {opcode}, imm32 {imm32}");
                todo!("should give undefined instruction fault")
                //Err(Fault::UndefInstr)
            }
        }
    }
}

impl Executor for Processor {
    #[inline(always)]
    fn step_sleep(&mut self) {
        self.syst_step(1);
        self.check_exceptions();
        self.dwt_tick(1);
    }

    #[inline(always)]
    fn step(&mut self) {
        let pc = self.get_pc();
        let mapped_pc = (self.map_address(pc) >> 1) as usize;
        let (instruction, instruction_size) = self.instruction_cache[mapped_pc];
        let count = self.execute(&instruction, instruction_size);
        self.cycle_count += u64::from(count);
        self.dwt_tick(count);
        self.syst_step(count);
        self.check_exceptions();
        //TODO exception entry also burns cycles that should be accounted for
        //DWT and SYST ticking
    }

    #[inline(always)]
    fn execute(&mut self, instruction: &Instruction, instruction_size: usize) -> u32 {
        self.instruction_count += 1;

        let in_it_block = self.in_it_block();

        match self.execute_internal(instruction) {
            Err(_fault) => {
                // all faults are mapped to hardfaults on armv6m
                let new_pc = self.get_pc();

                //TODO: map to correct exception
                //TODO: cycles not correctly accumulated yet for exception entry
                self.exception_entry(Exception::HardFault, new_pc)
                    .expect("error handling on exception entry not implemented");
                //TODO: proper amount of cycles calculation
                12
            }
            Ok(ExecuteSuccess::NotTaken) => {
                self.add_pc(instruction_size as u32);
                if in_it_block {
                    self.it_advance();
                }
                1
            }
            Ok(ExecuteSuccess::Branched { cycles }) => cycles,
            Ok(ExecuteSuccess::Taken { cycles }) => {
                self.add_pc(instruction_size as u32);

                if in_it_block {
                    self.it_advance();
                }
                cycles
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::condition::Condition;
    use crate::core::instruction::instruction_size;
    use crate::core::{
        instruction::{
            ITCondition, Reg2ShiftNoSetFlagsParams, RegImmCarryParams, SRType, SetFlags,
        },
        register::Reg,
    };

    #[test]
    fn test_it_block() {
        // arrange
        let mut core = Processor::new();
        core.set_r(Reg::R5, 0x49);
        core.set_r(Reg::R4, 0x01);
        core.set_r(Reg::R0, 0x49);
        core.psr.value = 0;

        let i1 = Instruction::CMP_reg {
            params: Reg2ShiftNoSetFlagsParams {
                rn: Reg::R0,
                rm: Reg::R5,
                shift_t: SRType::LSL,
                shift_n: 0,
            },
            thumb32: false,
        };

        let i2 = Instruction::IT {
            x: Some(ITCondition::Then),
            y: None,
            z: None,
            firstcond: Condition::NE,
            mask: 0b1000,
        };
        let i3 = Instruction::MOV_imm {
            params: RegImmCarryParams {
                rd: Reg::R4,
                imm32: Imm32Carry::NoCarry { imm32: 0 },
                setflags: SetFlags::False,
            },
            thumb32: false,
        };

        core.execute(&i1, instruction_size(&i1));
        core.execute(&i2, instruction_size(&i1));
        core.execute(&i3, instruction_size(&i1));

        assert_eq!(core.get_r(Reg::R4), 0x01);
        assert!(!core.in_it_block());
    }
}
