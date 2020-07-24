use crate::Processor;

use crate::executor::{ExecuteSuccess, ExecutorHelper};

use super::ExecuteResult;
use crate::core::instruction::Reg3NoSetFlagsParams;
use crate::core::{fault::Fault, register::BaseReg};

/// Divide operations
pub trait IsaDivide {
    fn exec_udiv(&mut self, params: &Reg3NoSetFlagsParams) -> ExecuteResult;
    fn exec_sdiv(&mut self, params: &Reg3NoSetFlagsParams) -> ExecuteResult;
}

impl IsaDivide for Processor {
    fn exec_sdiv(&mut self, params: &Reg3NoSetFlagsParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm_ = self.get_r(params.rm);
            let result = if rm_ == 0 {
                if self.integer_zero_divide_trapping_enabled() {
                    return Err(Fault::DivByZero);
                }
                0
            } else {
                let rn_ = self.get_r(params.rn);
                (rn_ as i32) / (rm_ as i32)
            };
            self.set_r(params.rd, result as u32);
            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_udiv(&mut self, params: &Reg3NoSetFlagsParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm = self.get_r(params.rm);
            let result = if rm == 0 {
                if self.integer_zero_divide_trapping_enabled() {
                    return Err(Fault::DivByZero);
                }
                0
            } else {
                let rn = self.get_r(params.rn);
                rn / rm
            };
            self.set_r(params.rd, result);
            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
}
