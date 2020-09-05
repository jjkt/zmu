use crate::Processor;

use crate::executor::{ExecuteSuccess, ExecutorHelper};

use super::ExecuteResult;
use crate::core::instruction::Reg3NoSetFlagsParams;
use crate::core::{
    bits::Bits,
    register::{Apsr, BaseReg},
};

/// Divide operations
pub trait IsaParallelAddSub {
    fn exec_uadd8(&mut self, params: &Reg3NoSetFlagsParams) -> ExecuteResult;
}

impl IsaParallelAddSub for Processor {
    fn exec_uadd8(&mut self, params: &Reg3NoSetFlagsParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm: u32 = self.get_r(params.rm);
            let rn: u32 = self.get_r(params.rn);

            let sum1: u32 = rn.get_bits(0..8) + rm.get_bits(0..8);
            let sum2: u32 = rn.get_bits(8..16) + rm.get_bits(8..16);
            let sum3: u32 = rn.get_bits(16..24) + rm.get_bits(16..24);
            let sum4: u32 = rn.get_bits(24..32) + rm.get_bits(24..32);

            let mut result: u32 = sum1.get_bits(0..8);
            result.set_bits(8..16, sum2.get_bits(0..8));
            result.set_bits(16..24, sum3.get_bits(0..8));
            result.set_bits(24..32, sum4.get_bits(0..8));
            self.set_r(params.rd, result);

            self.psr.set_ge0(sum1 >= 0x100);
            self.psr.set_ge1(sum2 >= 0x100);
            self.psr.set_ge2(sum3 >= 0x100);
            self.psr.set_ge3(sum4 >= 0x100);

            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
}
