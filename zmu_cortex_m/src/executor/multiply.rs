use crate::Processor;

use crate::executor::{ExecuteSuccess, ExecutorHelper};

use super::{conditional_setflags, ExecuteResult};
use crate::core::instruction::{Reg3Params, Reg4NoSetFlagsParams};
use crate::core::register::{Apsr, BaseReg};

/// Multiply operations
pub trait IsaMultiply {
    fn exec_mla(&mut self, params: &Reg4NoSetFlagsParams) -> ExecuteResult;
    fn exec_mls(&mut self, params: &Reg4NoSetFlagsParams) -> ExecuteResult;
    fn exec_mul(&mut self, params: &Reg3Params) -> ExecuteResult;
}

impl IsaMultiply for Processor {
    fn exec_mla(&mut self, params: &Reg4NoSetFlagsParams) -> ExecuteResult {
        if self.condition_passed() {
            let rn = self.get_r(params.rn);
            let rm = self.get_r(params.rm);
            let ra = self.get_r(params.ra);
            let result = rn.wrapping_mul(rm).wrapping_add(ra);

            self.set_r(params.rd, result);
            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_mls(&mut self, params: &Reg4NoSetFlagsParams) -> ExecuteResult {
        if self.condition_passed() {
            let rn = self.get_r(params.rn);
            let rm = self.get_r(params.rm);
            let ra = self.get_r(params.ra);
            let result = ra.wrapping_sub(rn.wrapping_mul(rm));

            self.set_r(params.rd, result);
            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_mul(&mut self, params: &Reg3Params) -> ExecuteResult {
        if self.condition_passed() {
            let rn = self.get_r(params.rn);
            let rm = self.get_r(params.rm);

            let result = rn.wrapping_mul(rm);

            self.set_r(params.rd, result);

            if conditional_setflags(params.setflags, self.in_it_block()) {
                self.psr.set_n(result);
                self.psr.set_z(result);
            }
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
}
