use crate::Processor;

use crate::executor::{ExecuteSuccess, ExecutorHelper};

use super::ExecuteResult;

use crate::core::{instruction::Reg2RdRmParams, register::BaseReg};

/// Branching operations
pub trait IsaMisc {
    fn exec_clz(&mut self, params: Reg2RdRmParams) -> ExecuteResult;
}

impl IsaMisc for Processor {
    fn exec_clz(&mut self, params: Reg2RdRmParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm = self.get_r(params.rm);

            self.set_r(params.rd, rm.leading_zeros());

            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
}
