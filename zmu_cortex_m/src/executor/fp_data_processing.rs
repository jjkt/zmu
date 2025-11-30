use crate::Processor;
use crate::core::fpregister::Fpscr;
use crate::core::instruction::{
    VAddSubParamsf32, VAddSubParamsf64, VCVTParams, VCmpParamsf32, VCmpParamsf64, VMovRegParamsf32,
    VMovRegParamsf64,
};

use crate::executor::ExecuteSuccess;

use super::ExecuteResult;
use super::fp_generic::{FloatingPointChecks, FloatingPointPublicOperations};
use crate::core::register::ExtensionRegOperations;
use crate::executor::ExecutorHelper;

pub trait IsaFloatingPointDataProcessing {
    fn exec_vabs_f32(&mut self, params: &VMovRegParamsf32) -> ExecuteResult;
    fn exec_vabs_f64(&mut self, params: &VMovRegParamsf64) -> ExecuteResult;

    fn exec_vcmp_f32(&mut self, params: &VCmpParamsf32) -> ExecuteResult;
    fn exec_vcmp_f64(&mut self, params: &VCmpParamsf64) -> ExecuteResult;

    fn exec_vadd_f32(&mut self, params: &VAddSubParamsf32) -> ExecuteResult;
    fn exec_vadd_f64(&mut self, params: &VAddSubParamsf64) -> ExecuteResult;

    fn exec_vsub_f32(&mut self, params: &VAddSubParamsf32) -> ExecuteResult;
    fn exec_vsub_f64(&mut self, params: &VAddSubParamsf64) -> ExecuteResult;

    fn exec_vcvt(&mut self, params: &VCVTParams) -> ExecuteResult;
}

impl IsaFloatingPointDataProcessing for Processor {
    fn exec_vabs_f32(&mut self, params: &VMovRegParamsf32) -> ExecuteResult {
        if self.condition_passed() {
            self.execute_fp_check();
            let op = self.get_sr(params.sm);
            let result = self.fp_abs::<u32>(op);
            self.set_sr(params.sd, result);
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_vabs_f64(&mut self, params: &VMovRegParamsf64) -> ExecuteResult {
        if self.condition_passed() {
            self.execute_fp_check();
            let (lower, upper) = self.get_dr(params.dm);
            let op = (upper as u64) << 32 | lower as u64;

            let result = self.fp_abs::<u64>(op);

            let result_upper = (result >> 32) as u32;
            let result_lower = result as u32;

            self.set_dr(params.dd, result_lower, result_upper);
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_vcmp_f32(&mut self, params: &VCmpParamsf32) -> ExecuteResult {
        if self.condition_passed() {
            self.execute_fp_check();
            let op32 = if params.with_zero {
                0
            } else {
                self.get_sr(params.sm)
            };
            let op1 = self.get_sr(params.sd);
            let (n, z, c, v) = self.fp_compare::<u32>(op1, op32, params.quiet_nan_exc, true);
            self.fpscr.set_n(n);
            self.fpscr.set_z(z);
            self.fpscr.set_c(c);
            self.fpscr.set_v(v);
        }
        Ok(ExecuteSuccess::NotTaken)
    }
    fn exec_vcmp_f64(&mut self, params: &VCmpParamsf64) -> ExecuteResult {
        if self.condition_passed() {
            self.execute_fp_check();
            let op64 = if params.with_zero {
                0u64
            } else {
                let (lower, upper) = self.get_dr(params.dm);
                (upper as u64) << 32 | lower as u64
            };
            let op1_src = self.get_dr(params.dd);
            let op1 = (op1_src.1 as u64) << 32 | op1_src.0 as u64;
            let (n, z, c, v) = self.fp_compare::<u64>(op1, op64, params.quiet_nan_exc, true);
            self.fpscr.set_n(n);
            self.fpscr.set_z(z);
            self.fpscr.set_c(c);
            self.fpscr.set_v(v);
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_vadd_f32(&mut self, params: &VAddSubParamsf32) -> ExecuteResult {
        if self.condition_passed() {
            self.execute_fp_check();
            let op1 = self.get_sr(params.sn);
            let op2 = self.get_sr(params.sm);
            let result = self.fp_add::<u32>(op1, op2, true);
            self.set_sr(params.sd, result);
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
    fn exec_vadd_f64(&mut self, params: &VAddSubParamsf64) -> ExecuteResult {
        if self.condition_passed() {
            self.execute_fp_check();
            let (op1_lower, op1_upper) = self.get_dr(params.dn);
            let (op2_lower, op2_upper) = self.get_dr(params.dm);
            let op1 = (op1_upper as u64) << 32 | op1_lower as u64;
            let op2 = (op2_upper as u64) << 32 | op2_lower as u64;
            let result = self.fp_add::<u64>(op1, op2, true);
            let result_upper = (result >> 32) as u32;
            let result_lower = result as u32;
            self.set_dr(params.dd, result_lower, result_upper);
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_vsub_f32(&mut self, params: &VAddSubParamsf32) -> ExecuteResult {
        if self.condition_passed() {
            self.execute_fp_check();
            let op1 = self.get_sr(params.sn);
            let op2 = self.get_sr(params.sm);
            let result = self.fp_sub::<u32>(op1, op2, true);
            self.set_sr(params.sd, result);
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
    fn exec_vsub_f64(&mut self, params: &VAddSubParamsf64) -> ExecuteResult {
        if self.condition_passed() {
            self.execute_fp_check();
            let (op1_lower, op1_upper) = self.get_dr(params.dn);
            let (op2_lower, op2_upper) = self.get_dr(params.dm);
            let op1 = (op1_upper as u64) << 32 | op1_lower as u64;
            let op2 = (op2_upper as u64) << 32 | op2_lower as u64;
            let result = self.fp_sub::<u64>(op1, op2, true);
            let result_upper = (result >> 32) as u32;
            let result_lower = result as u32;
            self.set_dr(params.dd, result_lower, result_upper);
            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_vcvt(&mut self, params: &VCVTParams) -> ExecuteResult {
        if self.condition_passed() {
            self.execute_fp_check();

            if params.to_integer {
                if params.dp_operation {
                    let m_reg = params
                        .m
                        .as_double()
                        .expect("Invalid register for double precision operation");
                    let d_reg = params
                        .d
                        .as_single()
                        .expect("Invalid register for single precision operation");
                    let dm = self.get_dr(*m_reg);
                    let dm_val = (dm.1 as u64) << 32 | dm.0 as u64;
                    let result = self.fp_to_fixed::<u64, u32>(
                        dm_val,
                        0,
                        params.unsigned,
                        params.round_zero,
                        true,
                    );
                    self.set_sr(*d_reg, result);
                } else {
                    let m_reg = params
                        .m
                        .as_single()
                        .expect("Invalid register for single precision operation");
                    let d_reg = params
                        .d
                        .as_single()
                        .expect("Invalid register for single precision operation");
                    let dm = self.get_sr(*m_reg);
                    let result = self.fp_to_fixed::<u32, u32>(
                        dm,
                        0,
                        params.unsigned,
                        params.round_zero,
                        true,
                    );
                    self.set_sr(*d_reg, result);
                }
            } else {
                if params.dp_operation {
                    let m_reg = params
                        .m
                        .as_single()
                        .expect("Invalid register for single precision operation");
                    let d_reg = params
                        .d
                        .as_double()
                        .expect("Invalid register for double precision operation");
                    let dm = self.get_sr(*m_reg);
                    let result = self.fixed_to_fp::<u64, u32>(
                        dm,
                        0,
                        params.unsigned,
                        params.round_zero,
                        true,
                    );
                    let result_upper = (result >> 32) as u32;
                    let result_lower = result as u32;
                    self.set_dr(*d_reg, result_lower, result_upper);
                } else {
                    let m_reg = params
                        .m
                        .as_single()
                        .expect("Invalid register for single precision operation");
                    let d_reg = params
                        .d
                        .as_single()
                        .expect("Invalid register for single precision operation");
                    let dm = self.get_sr(*m_reg);
                    let result = self.fixed_to_fp::<u32, u32>(
                        dm,
                        0,
                        params.unsigned,
                        params.round_zero,
                        true,
                    );
                    self.set_sr(*d_reg, result);
                }
            }

            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::register::{DoubleReg, SingleReg};

    #[test]
    fn test_vabs_f32() {
        let mut processor = Processor::new();

        // -1.0
        processor.set_sr(SingleReg::S0, 0xBF800000);
        processor
            .exec_vabs_f32(&VMovRegParamsf32 {
                sd: SingleReg::S1,
                sm: SingleReg::S0,
            })
            .unwrap();

        let result = processor.get_sr(SingleReg::S1);
        assert_eq!(result, 0x3F800000);
    }

    #[test]
    fn test_vabs_f64() {
        let mut processor = Processor::new();

        // -1.0
        processor.set_dr(DoubleReg::D0, 0x00000000, 0xBFF00000);
        processor
            .exec_vabs_f64(&VMovRegParamsf64 {
                dd: DoubleReg::D1,
                dm: DoubleReg::D0,
            })
            .unwrap();

        let result = processor.get_dr(DoubleReg::D1);
        assert_eq!(result, (0x00000000, 0x3FF00000));
    }
}
