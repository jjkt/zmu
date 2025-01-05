use crate::core::fpregister::Fpscr;
use crate::core::instruction::{
    VAddParamsf32, VAddParamsf64, VCmpParamsf32, VCmpParamsf64, VMovRegParamsf32, VMovRegParamsf64,
};
use crate::Processor;

use crate::executor::ExecuteSuccess;

use super::fp_generic::{fpabs_32, FloatingPointInternalOperations};
use super::ExecuteResult;
use crate::core::register::ExtensionRegOperations;
use crate::executor::ExecutorHelper;

pub trait IsaFloatingPointDataProcessing {
    fn exec_vabs_f32(&mut self, params: &VMovRegParamsf32) -> ExecuteResult;
    fn exec_vabs_f64(&mut self, params: &VMovRegParamsf64) -> ExecuteResult;

    fn exec_vcmp_f32(&mut self, params: &VCmpParamsf32) -> ExecuteResult;
    fn exec_vcmp_f64(&mut self, params: &VCmpParamsf64) -> ExecuteResult;

    fn exec_vadd_f32(&mut self, params: &VAddParamsf32) -> ExecuteResult;
    fn exec_vadd_f64(&mut self, params: &VAddParamsf64) -> ExecuteResult;
}

impl IsaFloatingPointDataProcessing for Processor {
    fn exec_vabs_f32(&mut self, params: &VMovRegParamsf32) -> ExecuteResult {
        if self.condition_passed() {
            //self.execute_fp_check();
            let value = self.get_sr(params.sm);
            let result = fpabs_32(value);
            self.set_sr(params.sd, result);
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_vabs_f64(&mut self, params: &VMovRegParamsf64) -> ExecuteResult {
        if self.condition_passed() {
            //self.execute_fp_check();
            let (upper, lower) = self.get_dr(params.dm);
            let upper_modified = fpabs_32(upper);
            self.set_dr(params.dd, lower, upper_modified);
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_vcmp_f32(&mut self, params: &VCmpParamsf32) -> ExecuteResult {
        if self.condition_passed() {
            //self.execute_fp_check();
            let op32 = if params.with_zero {
                0
            } else {
                self.get_sr(params.sm)
            };
            let op1 = self.get_sr(params.sd);
            let (n, z, c, v) = self.fp_compare_f32(op1, op32, params.quiet_nan_exc, true);
            self.fpscr.set_n(n);
            self.fpscr.set_z(z);
            self.fpscr.set_c(c);
            self.fpscr.set_v(v);
        }
        Ok(ExecuteSuccess::NotTaken)
    }
    fn exec_vcmp_f64(&mut self, params: &VCmpParamsf64) -> ExecuteResult {
        if self.condition_passed() {
            //self.execute_fp_check();
            let op64 = if params.with_zero {
                0u64
            } else {
                let (lower, upper) = self.get_dr(params.dm);
                (upper as u64) << 32 | lower as u64
            };
            let op1_src = self.get_dr(params.dd);
            let op1 = (op1_src.1 as u64) << 32 | op1_src.0 as u64;
            let (n, z, c, v) = self.fp_compare_f64(op1, op64, params.quiet_nan_exc, true);
            self.fpscr.set_n(n);
            self.fpscr.set_z(z);
            self.fpscr.set_c(c);
            self.fpscr.set_v(v);
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_vadd_f32(&mut self, params: &VAddParamsf32) -> ExecuteResult {
        if self.condition_passed() {
            //self.execute_fp_check();
            let op1 = self.get_sr(params.sn);
            let op2 = self.get_sr(params.sm);
            let result = self.fp_add_f32(op1, op2, true);
            self.set_sr(params.sd, result);
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
    fn exec_vadd_f64(&mut self, params: &VAddParamsf64) -> ExecuteResult {
        if self.condition_passed() {
            //self.execute_fp_check();
            let (op1_lower, op1_upper) = self.get_dr(params.dn);
            let (op2_lower, op2_upper) = self.get_dr(params.dm);
            let op1 = (op1_upper as u64) << 32 | op1_lower as u64;
            let op2 = (op2_upper as u64) << 32 | op2_lower as u64;
            let result = self.fp_add_f64(op1, op2, true);
            let result_upper = (result >> 32) as u32;
            let result_lower = result as u32;
            self.set_dr(params.dd, result_lower, result_upper);
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
}
