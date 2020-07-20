use crate::Processor;

use crate::core::fault::Fault;
use crate::core::operation::{add_with_carry, shift};

use crate::executor::{conditional_setflags, ExecuteResult, ExecutorHelper};

use crate::core::instruction::{AdcImmParams, AdcRegParams, CmpParams, SetFlags};
use crate::core::register::{Apsr, BaseReg};

/// Different variants based on add-with-carry basic operation
pub trait InstructionAdc {
    fn exec_adc_reg(&mut self, params: &AdcRegParams) -> Result<ExecuteResult, Fault>;
    fn exec_adc_imm(&mut self, params: &AdcImmParams) -> Result<ExecuteResult, Fault>;

    fn exec_add_imm(&mut self, params: &AdcImmParams) -> Result<ExecuteResult, Fault>;
    fn exec_rsb_imm(&mut self, params: &AdcImmParams) -> Result<ExecuteResult, Fault>;
    fn exec_sbc_imm(&mut self, params: &AdcImmParams) -> Result<ExecuteResult, Fault>;
    fn exec_sub_imm(&mut self, params: &AdcImmParams) -> Result<ExecuteResult, Fault>;

    fn exec_cmp_imm(&mut self, params: CmpParams) -> Result<ExecuteResult, Fault>;
    fn exec_cmn_imm(&mut self, params: CmpParams) -> Result<ExecuteResult, Fault>;
}

impl InstructionAdc for Processor {
    fn exec_adc_reg(&mut self, params: &AdcRegParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let c = self.psr.get_c();
            let shifted = shift(
                self.get_r(params.rm),
                params.shift_t,
                params.shift_n as usize,
                c,
            );
            let (result, carry, overflow) = add_with_carry(self.get_r(params.rn), shifted, c);
            self.set_r(params.rd, result);

            if conditional_setflags(params.setflags, self.in_it_block()) {
                self.psr.set_n(result);
                self.psr.set_z(result);
                self.psr.set_c(carry);
                self.psr.set_v(overflow);
            }

            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_adc_imm(&mut self, params: &AdcImmParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let r_n = self.get_r(params.rn);
            let (result, carry, overflow) = add_with_carry(r_n, params.imm32, self.psr.get_c());

            self.set_r(params.rd, result);

            if conditional_setflags(params.setflags, self.in_it_block()) {
                self.psr.set_n(result);
                self.psr.set_z(result);
                self.psr.set_c(carry);
                self.psr.set_v(overflow);
            }

            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_add_imm(&mut self, params: &AdcImmParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let r_n = self.get_r(params.rn);
            let (result, carry, overflow) = add_with_carry(r_n, params.imm32, false);

            if conditional_setflags(params.setflags, self.in_it_block()) {
                self.psr.set_n(result);
                self.psr.set_z(result);
                self.psr.set_c(carry);
                self.psr.set_v(overflow);
            }

            self.set_r(params.rd, result);
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_rsb_imm(&mut self, params: &AdcImmParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let r_n = self.get_r(params.rn);
            let (result, carry, overflow) = add_with_carry(r_n ^ 0xFFFF_FFFF, params.imm32, true);

            if conditional_setflags(params.setflags, self.in_it_block()) {
                self.psr.set_n(result);
                self.psr.set_z(result);
                self.psr.set_c(carry);
                self.psr.set_v(overflow);
            }

            self.set_r(params.rd, result);
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_sbc_imm(&mut self, params: &AdcImmParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let r_n = self.get_r(params.rn);
            let (result, carry, overflow) =
                add_with_carry(r_n, params.imm32 ^ 0xFFFF_FFFF, self.psr.get_c());

            self.set_r(params.rd, result);

            if params.setflags == SetFlags::True {
                self.psr.set_n(result);
                self.psr.set_z(result);
                self.psr.set_c(carry);
                self.psr.set_v(overflow);
            }

            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_sub_imm(&mut self, params: &AdcImmParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let r_n = self.get_r(params.rn);
            let (result, carry, overflow) = add_with_carry(r_n, params.imm32 ^ 0xFFFF_FFFF, true);

            if conditional_setflags(params.setflags, self.in_it_block()) {
                self.psr.set_n(result);
                self.psr.set_z(result);
                self.psr.set_c(carry);
                self.psr.set_v(overflow);
            }

            self.set_r(params.rd, result);
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_cmp_imm(&mut self, params: CmpParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let (result, carry, overflow) =
                add_with_carry(self.get_r(params.rn), params.imm32 ^ 0xFFFF_FFFF, true);
            self.psr.set_n(result);
            self.psr.set_z(result);
            self.psr.set_c(carry);
            self.psr.set_v(overflow);
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_cmn_imm(&mut self, params: CmpParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let (result, carry, overflow) =
                add_with_carry(self.get_r(params.rn), params.imm32, false);
            self.psr.set_n(result);
            self.psr.set_z(result);
            self.psr.set_c(carry);
            self.psr.set_v(overflow);
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }
}
