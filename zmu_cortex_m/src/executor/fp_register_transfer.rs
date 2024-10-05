use crate::core::instruction::{
    VMovCr2DpParams, VMovCrSpParams, VMovImmParams32, VMovImmParams64, VMovRegParamsf32,
    VMovRegParamsf64,
};
use crate::Processor;

use crate::executor::ExecuteSuccess;

use super::ExecuteResult;
use crate::core::register::{BaseReg, ExtensionRegOperations};

pub trait IsaFloatingPointRegisterTransfer {
    fn exec_vmov_cr_sp(&mut self, params: &VMovCrSpParams) -> ExecuteResult;
    fn exec_vmov_cr2_dp(&mut self, params: &VMovCr2DpParams) -> ExecuteResult;

    fn exec_vmov_reg_f32(&mut self, params: VMovRegParamsf32) -> ExecuteResult;
    fn exec_vmov_reg_f64(&mut self, params: VMovRegParamsf64) -> ExecuteResult;

    fn exec_vmov_imm_32(&mut self, params: VMovImmParams32) -> ExecuteResult;
    fn exec_vmov_imm_64(&mut self, params: VMovImmParams64) -> ExecuteResult;
}

impl IsaFloatingPointRegisterTransfer for Processor {
    fn exec_vmov_cr_sp(&mut self, params: &VMovCrSpParams) -> ExecuteResult {
        if params.to_arm_register {
            let value = self.get_sr(params.sn);
            self.set_r(params.rt, value);
        } else {
            self.set_sr(params.sn, self.get_r(params.rt));
        }

        Ok(ExecuteSuccess::Taken { cycles: 1 })
    }

    fn exec_vmov_cr2_dp(&mut self, params: &VMovCr2DpParams) -> ExecuteResult {
        if params.to_arm_registers {
            let (low, high) = self.get_dr(params.dm);
            self.set_r(params.rt, low);
            self.set_r(params.rt2, high);
        } else {
            let low = self.get_r(params.rt);
            let high = self.get_r(params.rt2);
            self.set_dr(params.dm, low, high);
        }

        Ok(ExecuteSuccess::Taken { cycles: 1 })
    }

    fn exec_vmov_reg_f32(&mut self, params: VMovRegParamsf32) -> ExecuteResult {
        let value = self.get_sr(params.sm);
        self.set_sr(params.sd, value);
        Ok(ExecuteSuccess::Taken { cycles: 1 })
    }

    fn exec_vmov_reg_f64(&mut self, params: VMovRegParamsf64) -> ExecuteResult {
        let (low, high) = self.get_dr(params.dm);
        self.set_dr(params.dd, low, high);
        Ok(ExecuteSuccess::Taken { cycles: 1 })
    }

    fn exec_vmov_imm_32(&mut self, params: VMovImmParams32) -> ExecuteResult {
        self.set_sr(params.sd, params.imm32);
        Ok(ExecuteSuccess::Taken { cycles: 1 })
    }

    fn exec_vmov_imm_64(&mut self, params: VMovImmParams64) -> ExecuteResult {
        let (low, high) = (params.imm64 as u32, (params.imm64 >> 32) as u32);
        self.set_dr(params.dd, low, high);
        Ok(ExecuteSuccess::Taken { cycles: 1 })
    }
}
