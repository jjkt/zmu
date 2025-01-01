use crate::core::fpregister::Fpscr;
use crate::core::instruction::{VCmpParamsf32, VCmpParamsf64, VMovRegParamsf32, VMovRegParamsf64};
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
            let (lower, upper) = self.get_dr(params.dm);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::instruction::{
        VCmpParamsf32, VCmpParamsf64, VMovRegParamsf32, VMovRegParamsf64,
    };
    use crate::core::register::{DoubleReg, SingleReg};
    use crate::Processor;

    #[test]
    fn test_exec_vabs_f32() {
        let mut processor = Processor::new();
        let params = VMovRegParamsf32 {
            sd: SingleReg::S0,
            sm: SingleReg::S1,
        };
        processor.set_sr(SingleReg::S1, 0x80000000); // -0.0
        processor.exec_vabs_f32(&params).unwrap();
        assert_eq!(processor.get_sr(SingleReg::S0), 0x00000000); // 0.0

        processor.set_sr(SingleReg::S1, 0xFFFFFFFF); // -1.0
        processor.exec_vabs_f32(&params).unwrap();
        assert_eq!(processor.get_sr(SingleReg::S0), 0x7FFFFFFF); // 1.0
    }

    #[test]
    fn test_exec_vabs_f64() {
        let mut processor = Processor::new();
        let params = VMovRegParamsf64 {
            dd: DoubleReg::D0,
            dm: DoubleReg::D1,
        };
        processor.set_dr(DoubleReg::D1, 0x00000000, 0x80000000); // -0.0
        processor.exec_vabs_f64(&params).unwrap();
        let (lower, upper) = processor.get_dr(DoubleReg::D0);
        assert_eq!((lower, upper), (0x00000000, 0x00000000)); // 0.0

        processor.set_dr(DoubleReg::D1, 0xFFFFFFFF, 0xFFFFFFFF); // -1.0
        processor.exec_vabs_f64(&params).unwrap();
        let (lower, upper) = processor.get_dr(DoubleReg::D0);
        assert_eq!((lower, upper), (0xFFFFFFFF, 0x7FFFFFFF)); // 1.0
    }

    #[test]
    fn test_exec_vcmp_f32() {
        let mut processor = Processor::new();
        let params = VCmpParamsf32 {
            sd: SingleReg::S0,
            sm: SingleReg::S1,
            with_zero: false,
            quiet_nan_exc: false,
        };
        processor.set_sr(SingleReg::S0, 0x3F800000); // 1.0
        processor.set_sr(SingleReg::S1, 0x3F800000); // 1.0
        processor.exec_vcmp_f32(&params).unwrap();
        assert!(!processor.fpscr.get_n());
        assert!(processor.fpscr.get_z()); // 1.0 == 1.0
        assert!(processor.fpscr.get_c());
        assert!(!processor.fpscr.get_v());

        processor.set_sr(SingleReg::S1, 0x40000000); // 2.0
        processor.exec_vcmp_f32(&params).unwrap();
        assert!(processor.fpscr.get_n()); // 1.0 < 2.0
        assert!(!processor.fpscr.get_z());
        assert!(!processor.fpscr.get_c());
        assert!(!processor.fpscr.get_v());

        processor.set_sr(SingleReg::S0, 0x40000000); // 2.0
        processor.set_sr(SingleReg::S1, 0x3F800000); // 1.0
        processor.exec_vcmp_f32(&params).unwrap();
        assert!(!processor.fpscr.get_n()); //2.0 > 1.0
        assert!(!processor.fpscr.get_z());
        assert!(processor.fpscr.get_c());
        assert!(!processor.fpscr.get_v());
    }

    #[test]
    fn test_exec_vcmp_f64() {
        let mut processor = Processor::new();
        let params = VCmpParamsf64 {
            dd: DoubleReg::D0,
            dm: DoubleReg::D1,
            with_zero: false,
            quiet_nan_exc: false,
        };
        processor.set_dr(DoubleReg::D0, 0x00000000, 0x3FF00000); // 1.0
        processor.set_dr(DoubleReg::D1, 0x00000000, 0x3FF00000); // 1.0
        processor.exec_vcmp_f64(&params).unwrap();
        assert!(!processor.fpscr.get_n());
        assert!(processor.fpscr.get_z()); // 1.0 == 1.0
        assert!(processor.fpscr.get_c());
        assert!(!processor.fpscr.get_v());

        processor.set_dr(DoubleReg::D1, 0x00000000, 0x40000000); // 2.0
        processor.exec_vcmp_f64(&params).unwrap();
        assert!(processor.fpscr.get_n()); // 1.0 < 2.0
        assert!(!processor.fpscr.get_z());
        assert!(!processor.fpscr.get_c());
        assert!(!processor.fpscr.get_v());

        processor.set_dr(DoubleReg::D0, 0x00000000, 0x40000000); // 2.0
        processor.set_dr(DoubleReg::D1, 0x00000000, 0x3FF00000); // 1.0
        processor.exec_vcmp_f64(&params).unwrap();
        assert!(!processor.fpscr.get_n()); //2.0 > 1.0
        assert!(!processor.fpscr.get_z());
        assert!(processor.fpscr.get_c());
        assert!(!processor.fpscr.get_v());
    }

}
