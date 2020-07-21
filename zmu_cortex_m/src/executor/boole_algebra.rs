use crate::Processor;

use crate::core::fault::Fault;
use crate::core::operation::shift_c;

use crate::executor::{conditional_setflags, ExecuteResult, ExecutorHelper};

use crate::core::instruction::Reg3ShiftParams;
use crate::core::register::{Apsr, BaseReg};

/// Different variants for boole algebra instructions
pub trait InstructionBooleAlgebra {
    fn exec_and_reg(&mut self, params: &Reg3ShiftParams) -> Result<ExecuteResult, Fault>;
}

impl InstructionBooleAlgebra for Processor {
    fn exec_and_reg(&mut self, params: &Reg3ShiftParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let c = self.psr.get_c();
            let r_n = self.get_r(params.rn);
            let r_m = self.get_r(params.rm);

            let (shifted, _) = shift_c(r_m, params.shift_t, params.shift_n as usize, c);

            let result = r_n & shifted;

            self.set_r(params.rd, result);

            if conditional_setflags(params.setflags, self.in_it_block()) {
                self.psr.set_n(result);
                self.psr.set_z(result);
            }
            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }
}
