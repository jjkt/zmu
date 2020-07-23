use crate::Processor;

use crate::executor::{ExecuteSuccess, ExecutorHelper};

use super::ExecuteResult;
use crate::core::instruction::{Reg3HighParams, Reg4HighParams, Reg643232Params};
use crate::core::{
    bits::Bits,
    register::{Apsr, BaseReg},
};

/// Multiply operations
pub trait IsaSignedMultiply {
    fn exec_smull(&mut self, params: &Reg643232Params) -> ExecuteResult;
    fn exec_smul(&mut self, params: &Reg3HighParams) -> ExecuteResult;
    fn exec_smla(&mut self, params: &Reg4HighParams) -> ExecuteResult;
}

impl IsaSignedMultiply for Processor {
    fn exec_smull(&mut self, params: &Reg643232Params) -> ExecuteResult {
        if self.condition_passed() {
            let rn = i64::from(self.get_r(params.rn));
            let rm = i64::from(self.get_r(params.rm));
            let result = rn.wrapping_mul(rm) as u64;

            self.set_r(params.rdlo, result.get_bits(0..32) as u32);
            self.set_r(params.rdhi, result.get_bits(32..64) as u32);
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_smul(&mut self, params: &Reg3HighParams) -> ExecuteResult {
        if self.condition_passed() {
            let operand1 = i32::from(if params.n_high {
                let op = self.get_r(params.rn).get_bits(16..32);
                op as i16
            } else {
                let op = self.get_r(params.rn).get_bits(0..16);
                op as i16
            });
            let operand2 = i32::from(if params.m_high {
                let op = self.get_r(params.rm).get_bits(16..32);
                op as i16
            } else {
                let op = self.get_r(params.rm).get_bits(0..16);
                op as i16
            });

            let result = operand1.wrapping_mul(operand2);

            self.set_r(params.rd, result as u32);

            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_smla(&mut self, params: &Reg4HighParams) -> ExecuteResult {
        if self.condition_passed() {
            let operand1 = i32::from(if params.n_high {
                let op: u32 = self.get_r(params.rn).get_bits(16..32);
                op as i16
            } else {
                let op: u32 = self.get_r(params.rn).get_bits(0..16);
                op as i16
            });
            let operand2 = i32::from(if params.m_high {
                let op: u32 = self.get_r(params.rm).get_bits(16..32);
                op as i16
            } else {
                let op: u32 = self.get_r(params.rm).get_bits(0..16);
                op as i16
            });

            let result = operand1
                .wrapping_mul(operand2)
                .wrapping_add(self.get_r(params.ra) as i32);

            self.set_r(params.rd, result as u32);
            if result != result as i32 {
                self.psr.set_q(true);
            }

            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
}
