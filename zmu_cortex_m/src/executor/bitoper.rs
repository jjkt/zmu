use crate::core::fault::Fault;
use crate::executor::{ExecuteResult, ExecutorHelper};
use crate::Processor;

use super::expand_conditional_carry;
use crate::core::instruction::Reg2ImmCarryParams;
use crate::core::register::{Apsr, BaseReg};

/// Different variants for bit manipulations
pub trait InstructionBitOper {
    fn exec_bic_imm(&mut self, params: &Reg2ImmCarryParams) -> Result<ExecuteResult, Fault>;
}

impl InstructionBitOper for Processor {
    fn exec_bic_imm(&mut self, params: &Reg2ImmCarryParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let c = self.psr.get_c();
            let (im, carry) = expand_conditional_carry(&params.imm32, c);

            let result = self.get_r(params.rn) & (im ^ 0xffff_ffff);
            self.set_r(params.rd, result);

            if params.setflags {
                self.psr.set_n(result);
                self.psr.set_z(result);
                self.psr.set_c(carry);
            }
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }
}
