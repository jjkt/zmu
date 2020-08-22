use crate::Processor;

use crate::{
    core::register::{BaseReg, Reg},
    executor::{ExecuteSuccess, ExecutorHelper},
    semihosting::{decode_semihostcmd, semihost_return},
};

use super::ExecuteResult;

/// Branching operations
pub trait IsaException {
    fn exec_svc(&self) -> ExecuteResult;
    fn exec_bkpt(&mut self, imm32: u32) -> ExecuteResult;
}

impl IsaException for Processor {
    fn exec_svc(&self) -> ExecuteResult {
        if self.condition_passed() {
            //TODO
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_bkpt(&mut self, imm32: u32) -> ExecuteResult {
        if imm32 == 0xab {
            let r0 = self.get_r(Reg::R0);
            let r1 = self.get_r(Reg::R1);
            let semihost_cmd = decode_semihostcmd(r0, r1, self)?;

            if let Some(sh_func) = &mut self.semihost_func {
                let semihost_response = (sh_func)(&semihost_cmd);
                semihost_return(self, &semihost_response);
            }
        }
        Ok(ExecuteSuccess::Taken { cycles: 1 })
    }
}
