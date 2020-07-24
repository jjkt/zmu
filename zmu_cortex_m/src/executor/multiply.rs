use crate::Processor;

use crate::executor::{ExecuteSuccess, ExecutorHelper};

use super::{conditional_setflags, ExecuteResult};
use crate::core::instruction::{Reg3Params, Reg4NoSetFlagsParams, Reg643232Params};
use crate::core::{
    bits::Bits,
    register::{Apsr, BaseReg},
};

/// Multiply operations
pub trait IsaMultiply {
    fn exec_mla(&mut self, params: &Reg4NoSetFlagsParams) -> ExecuteResult;
    fn exec_mls(&mut self, params: &Reg4NoSetFlagsParams) -> ExecuteResult;
    fn exec_mul(&mut self, params: &Reg3Params) -> ExecuteResult;
    fn exec_umlal(&mut self, params: &Reg643232Params) -> ExecuteResult;
    fn exec_umull(&mut self, params: &Reg643232Params) -> ExecuteResult;
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

    fn exec_umlal(&mut self, params: &Reg643232Params) -> ExecuteResult {
        if self.condition_passed() {
            let rn = u64::from(self.get_r(params.rn));
            let rm = u64::from(self.get_r(params.rm));
            let rdlo = u64::from(self.get_r(params.rdlo));
            let rdhi = u64::from(self.get_r(params.rdhi));

            let rdhilo = (rdhi << 32) + rdlo;

            let result = rn.wrapping_mul(rm).wrapping_add(rdhilo);

            self.set_r(params.rdlo, result.get_bits(0..32) as u32);
            self.set_r(params.rdhi, result.get_bits(32..64) as u32);
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
    fn exec_umull(&mut self, params: &Reg643232Params) -> ExecuteResult {
        if self.condition_passed() {
            let rn = u64::from(self.get_r(params.rn));
            let rm = u64::from(self.get_r(params.rm));
            let result = rn.wrapping_mul(rm);

            self.set_r(params.rdlo, result.get_bits(0..32) as u32);
            self.set_r(params.rdhi, result.get_bits(32..64) as u32);
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
}