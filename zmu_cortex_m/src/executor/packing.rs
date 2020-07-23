use crate::Processor;

use crate::executor::{ExecuteSuccess, ExecutorHelper};

use super::ExecuteResult;
use crate::core::instruction::Reg2UsizeParams;
use crate::core::{
    bits::Bits,
    operation::{ror, sign_extend},
    register::BaseReg,
};

/// Multiply operations
pub trait IsaPacking {
    fn exec_sxtb(&mut self, params: &Reg2UsizeParams) -> ExecuteResult;
}

impl IsaPacking for Processor {
    fn exec_sxtb(&mut self, params: &Reg2UsizeParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm = self.get_r(params.rm);
            let rotated = ror(rm, params.rotation);
            self.set_r(params.rd, sign_extend(rotated.get_bits(0..8), 7, 32) as u32);
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
}
