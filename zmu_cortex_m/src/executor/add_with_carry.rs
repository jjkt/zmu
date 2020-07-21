use crate::Processor;

use crate::core::fault::Fault;
use crate::core::operation::{add_with_carry, shift};

use crate::executor::{ExecuteResult, ExecutorHelper};

use crate::core::instruction::{
    Reg2ImmParams, Reg2ShiftNoSetFlagsParams, Reg2ShiftParams, Reg3ShiftParams, RegImmParams,
    SetFlags,
};
use crate::core::register::{Apsr, BaseReg, Reg};

/// Different variants based on add-with-carry basic operation
pub trait InstructionAdc {
    fn exec_adc_reg(&mut self, params: &Reg3ShiftParams) -> Result<ExecuteResult, Fault>;
    fn exec_add_reg(&mut self, params: &Reg3ShiftParams) -> Result<ExecuteResult, Fault>;
    fn exec_sub_reg(&mut self, params: &Reg3ShiftParams) -> Result<ExecuteResult, Fault>;
    fn exec_rsb_reg(&mut self, params: &Reg3ShiftParams) -> Result<ExecuteResult, Fault>;
    fn exec_sbc_reg(&mut self, params: &Reg3ShiftParams) -> Result<ExecuteResult, Fault>;
    fn exec_add_sp_reg(&mut self, params: &Reg2ShiftParams) -> Result<ExecuteResult, Fault>;
    fn exec_cmp_reg(&mut self, params: &Reg2ShiftNoSetFlagsParams) -> Result<ExecuteResult, Fault>;
    fn exec_cmn_reg(&mut self, params: &Reg2ShiftNoSetFlagsParams) -> Result<ExecuteResult, Fault>;

    fn exec_adc_imm(&mut self, params: &Reg2ImmParams) -> Result<ExecuteResult, Fault>;
    fn exec_add_imm(&mut self, params: &Reg2ImmParams) -> Result<ExecuteResult, Fault>;
    fn exec_rsb_imm(&mut self, params: &Reg2ImmParams) -> Result<ExecuteResult, Fault>;
    fn exec_sbc_imm(&mut self, params: &Reg2ImmParams) -> Result<ExecuteResult, Fault>;
    fn exec_sub_imm(&mut self, params: &Reg2ImmParams) -> Result<ExecuteResult, Fault>;

    fn exec_cmp_imm(&mut self, params: RegImmParams) -> Result<ExecuteResult, Fault>;
    fn exec_cmn_imm(&mut self, params: RegImmParams) -> Result<ExecuteResult, Fault>;
}

impl InstructionAdc for Processor {
    fn exec_adc_reg(&mut self, params: &Reg3ShiftParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let r_n = self.get_r(params.rn);
            let r_m = self.get_r(params.rm);
            let c = self.psr.get_c();
            let shifted = shift(r_m, params.shift_t, params.shift_n as usize, c);

            let (result, carry, overflow) = add_with_carry(r_n, shifted, c);
            self.set_r(params.rd, result);

            self.update_flags_check_it_block(params.setflags, result, carry, overflow);
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_sub_reg(&mut self, params: &Reg3ShiftParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let r_n = self.get_r(params.rn);
            let r_m = self.get_r(params.rm);
            let c = self.psr.get_c();
            let shifted = shift(r_m, params.shift_t, params.shift_n as usize, c);

            let (result, carry, overflow) = add_with_carry(r_n, shifted ^ 0xFFFF_FFFF, true);
            self.set_r(params.rd, result);

            self.update_flags_check_it_block(params.setflags, result, carry, overflow);
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_add_reg(&mut self, params: &Reg3ShiftParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let c = self.psr.get_c();
            let r_n = self.get_r(params.rn);
            let r_m = self.get_r(params.rm);
            let shifted = shift(r_m, params.shift_t, params.shift_n as usize, c);
            let (result, carry, overflow) = add_with_carry(r_n, shifted, false);

            if params.rd == Reg::PC {
                self.branch_write_pc(result);
                Ok(ExecuteResult::Branched { cycles: 3 })
            } else {
                self.update_flags_check_it_block(params.setflags, result, carry, overflow);
                self.set_r(params.rd, result);
                Ok(ExecuteResult::Taken { cycles: 1 })
            }
        } else {
            Ok(ExecuteResult::NotTaken)
        }
    }


    fn exec_rsb_reg(&mut self, params: &Reg3ShiftParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let r_n = self.get_r(params.rn);
            let r_m = self.get_r(params.rm);
            let c = self.psr.get_c();

            let shifted = shift(r_m, params.shift_t, params.shift_n as usize, c);
            let (result, carry, overflow) = add_with_carry(r_n ^ 0xFFFF_FFFF, shifted, true);

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

    fn exec_sbc_reg(&mut self, params: &Reg3ShiftParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let r_n = self.get_r(params.rn);
            let r_m = self.get_r(params.rm);
            let c = self.psr.get_c();

            let shifted = shift(r_m, params.shift_t, params.shift_n as usize, c);

            let (result, carry, overflow) =
                add_with_carry(r_n, shifted ^ 0xffff_ffff, self.psr.get_c());

            self.update_flags_check_it_block(params.setflags, result, carry, overflow);

            self.set_r(params.rd, result);
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_add_sp_reg(&mut self, params: &Reg2ShiftParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let c = self.psr.get_c();
            let shifted = shift(
                self.get_r(params.rm),
                params.shift_t,
                params.shift_n as usize,
                c,
            );
            let (result, carry, overflow) = add_with_carry(self.get_r(Reg::SP), shifted, false);

            if params.rd == Reg::PC {
                self.branch_write_pc(result);
                Ok(ExecuteResult::Branched { cycles: 3 })
            } else {
                if params.setflags {
                    self.psr.set_n(result);
                    self.psr.set_z(result);
                    self.psr.set_c(carry);
                    self.psr.set_v(overflow);
                }
                self.set_r(params.rd, result);
                Ok(ExecuteResult::Taken { cycles: 1 })
            }
        } else {
            Ok(ExecuteResult::NotTaken)
        }
    }

    fn exec_cmp_reg(&mut self, params: &Reg2ShiftNoSetFlagsParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let r_m = self.get_r(params.rm);
            let c = self.psr.get_c();

            let shifted = shift(r_m, params.shift_t, params.shift_n as usize, c);
            let (result, carry, overflow) =
                add_with_carry(self.get_r(params.rn), shifted ^ 0xFFFF_FFFF, true);

            self.psr.set_n(result);
            self.psr.set_z(result);
            self.psr.set_c(carry);
            self.psr.set_v(overflow);
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_cmn_reg(&mut self, params: &Reg2ShiftNoSetFlagsParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let shifted = shift(
                self.get_r(params.rm),
                params.shift_t,
                params.shift_n as usize,
                self.psr.get_c(),
            );
            let (result, carry, overflow) = add_with_carry(self.get_r(params.rn), shifted, false);
            self.psr.set_n(result);
            self.psr.set_z(result);
            self.psr.set_c(carry);
            self.psr.set_v(overflow);
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_adc_imm(&mut self, params: &Reg2ImmParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let r_n = self.get_r(params.rn);
            let (result, carry, overflow) = add_with_carry(r_n, params.imm32, self.psr.get_c());

            self.set_r(params.rd, result);
            self.update_flags_check_it_block(params.setflags, result, carry, overflow);

            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_add_imm(&mut self, params: &Reg2ImmParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let r_n = self.get_r(params.rn);
            let (result, carry, overflow) = add_with_carry(r_n, params.imm32, false);

            self.update_flags_check_it_block(params.setflags, result, carry, overflow);
            self.set_r(params.rd, result);
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_rsb_imm(&mut self, params: &Reg2ImmParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let r_n = self.get_r(params.rn);
            let (result, carry, overflow) = add_with_carry(r_n ^ 0xFFFF_FFFF, params.imm32, true);

            self.update_flags_check_it_block(params.setflags, result, carry, overflow);

            self.set_r(params.rd, result);
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_sbc_imm(&mut self, params: &Reg2ImmParams) -> Result<ExecuteResult, Fault> {
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

    fn exec_sub_imm(&mut self, params: &Reg2ImmParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let r_n = self.get_r(params.rn);
            let (result, carry, overflow) = add_with_carry(r_n, params.imm32 ^ 0xFFFF_FFFF, true);

            self.update_flags_check_it_block(params.setflags, result, carry, overflow);

            self.set_r(params.rd, result);
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }

    fn exec_cmp_imm(&mut self, params: RegImmParams) -> Result<ExecuteResult, Fault> {
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

    fn exec_cmn_imm(&mut self, params: RegImmParams) -> Result<ExecuteResult, Fault> {
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
