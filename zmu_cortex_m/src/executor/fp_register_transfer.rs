use crate::core::instruction::{
    VMRSTarget, VMovCr2DpParams, VMovCrSpParams, VMovImmParams32, VMovImmParams64,
    VMovRegParamsf32, VMovRegParamsf64,
};

use crate::core::fpregister::Fpscr;
use crate::Processor;

use crate::executor::ExecuteSuccess;

use super::ExecuteResult;
use crate::core::register::{BaseReg, ExtensionRegOperations, Apsr};

use crate::executor::ExecutorHelper;

pub trait IsaFloatingPointRegisterTransfer {
    fn exec_vmov_cr_sp(&mut self, params: &VMovCrSpParams) -> ExecuteResult;
    fn exec_vmov_cr2_dp(&mut self, params: &VMovCr2DpParams) -> ExecuteResult;

    fn exec_vmov_reg_f32(&mut self, params: VMovRegParamsf32) -> ExecuteResult;
    fn exec_vmov_reg_f64(&mut self, params: VMovRegParamsf64) -> ExecuteResult;

    fn exec_vmov_imm_32(&mut self, params: VMovImmParams32) -> ExecuteResult;
    fn exec_vmov_imm_64(&mut self, params: VMovImmParams64) -> ExecuteResult;

    fn exec_vmrs(&mut self, params: VMRSTarget) -> ExecuteResult;
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

    fn exec_vmrs(&mut self, params: VMRSTarget) -> ExecuteResult {
        if self.condition_passed() {
            //EncodingSpecificOperations();
            //ExecuteFPCheck();
            //SerializeVFP();
            //VFPExcBarrier();

            match params {
                VMRSTarget::APSRNZCV => {
                    let n = self.fpscr.get_n();
                    let z = self.fpscr.get_z();
                    let c = self.fpscr.get_c();
                    let v = self.fpscr.get_v();

                    self.psr.set_n_bit(n);
                    self.psr.set_z_bit(z);
                    self.psr.set_c(c);
                    self.psr.set_v(v);
                }
                VMRSTarget::Register(reg) => {
                    self.set_r(reg, self.fpscr);
                }
            }
        }
        Ok(ExecuteSuccess::Taken { cycles: 1 })
    }
}
