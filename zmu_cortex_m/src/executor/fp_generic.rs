use crate::{core::bits::Bits, Processor};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FPType {
    Nonzero,
    Zero,
    Infinity,
    QNaN,
    SNaN,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FPExc {
    InvalidOp,
    DivideByZero,
    Overflow,
    Underflow,
    Inexact,
    InputDenorm,
}

pub fn fpabs_32(value: u32) -> u32 {
    // set upmost bit to 0
    value & 0x7FFFFFFF
}

fn standard_fpscr_value(fpscr: u32) -> u32 {
    fpscr
}
pub trait FloatingPointInternalOperations {
    fn fp_compare_f32(
        &mut self,
        op1: u32,
        op2: u32,
        quiet_nan_exc: bool,
        fpscr_controlled: bool,
    ) -> (bool, bool, bool, bool);

    fn fp_compare_f64(
        &mut self,
        op1: u64,
        op2: u64,
        quiet_nan_exc: bool,
        fpscr_controlled: bool,
    ) -> (bool, bool, bool, bool);

    fn fp_process_exception(&mut self, exc: FPExc, fpscr_val: u32);

    fn fp_unpack_f32(&mut self, fpval: u32, fpscr_val: u32) -> (FPType, bool, f32);
    fn fp_unpack_f64(&mut self, fpval: u64, fpscr_val: u32) -> (FPType, bool, f64);
}

fn is_zero_32(value: u32) -> bool {
    value == 0
}

fn is_zero_64(value: u64) -> bool {
    value == 0
}

fn is_ones_32(value: u32, len: usize) -> bool {
    value.count_ones() == len as u32
}

fn is_ones_64(value: u64, len: usize) -> bool {
    value.count_ones() == len as u32
}

impl FloatingPointInternalOperations for Processor {
    fn fp_compare_f32(
        &mut self,
        op1: u32,
        op2: u32,
        quiet_nan_exc: bool,
        fpscr_controlled: bool,
    ) -> (bool, bool, bool, bool) {
        let fpscr_val = if fpscr_controlled {
            self.fpscr
        } else {
            standard_fpscr_value(self.fpscr)
        };

        let (type1, _sign1, value1) = self.fp_unpack_f32(op1, fpscr_val);
        let (type2, _sign2, value2) = self.fp_unpack_f32(op2, fpscr_val);

        if type1 == FPType::SNaN
            || type1 == FPType::QNaN
            || type2 == FPType::SNaN
            || type2 == FPType::QNaN
        {
            let result = (false, false, true, true);
            if type1 == FPType::SNaN || type2 == FPType::SNaN || quiet_nan_exc {
                self.fp_process_exception(FPExc::InvalidOp, fpscr_val);
            }
            result
        } else {
            if value1 == value2 {
                (false, true, true, false)
            } else if value1 < value2 {
                (true, false, false, false)
            } else {
                (false, false, true, false)
            }
        }
    }

    fn fp_compare_f64(
        &mut self,
        op1: u64,
        op2: u64,
        quiet_nan_exc: bool,
        fpscr_controlled: bool,
    ) -> (bool, bool, bool, bool) {
        let fpscr_val = if fpscr_controlled {
            self.fpscr
        } else {
            standard_fpscr_value(self.fpscr)
        };

        let (type1, _sign1, value1) = self.fp_unpack_f64(op1, fpscr_val);
        let (type2, _sign2, value2) = self.fp_unpack_f64(op2, fpscr_val);

        if type1 == FPType::SNaN
            || type1 == FPType::QNaN
            || type2 == FPType::SNaN
            || type2 == FPType::QNaN
        {
            let result = (false, false, true, true);
            if type1 == FPType::SNaN || type2 == FPType::SNaN || quiet_nan_exc {
                self.fp_process_exception(FPExc::InvalidOp, fpscr_val);
            }
            result
        } else {
            if value1 == value2 {
                (false, true, true, false)
            } else if value1 < value2 {
                (true, false, false, false)
            } else {
                (false, false, true, false)
            }
        }
    }

    fn fp_process_exception(&mut self, exc: FPExc, fpscr_val: u32) {
        let (enable, cumul) = match exc {
            FPExc::InvalidOp => (8, 0),
            FPExc::DivideByZero => (9, 1),
            FPExc::Overflow => (10, 2),
            FPExc::Underflow => (11, 3),
            FPExc::Inexact => (12, 4),
            FPExc::InputDenorm => (15, 7),
        };
        if fpscr_val.get_bit(enable) {
            // implementation defined trap handling
            todo!()
        } else {
            self.fpscr.set_bit(cumul, true);
        }
    }

    fn fp_unpack_f32(&mut self, fpval: u32, fpscr_val: u32) -> (FPType, bool, f32) {
        let sign = fpval.get_bit(31);
        let exp32 = fpval.get_bits(23..31);
        let frac32 = fpval.get_bits(0..23);

        let (ret_type, value) = if is_zero_32(exp32) {
            if is_zero_32(frac32) || fpscr_val.get_bit(24) {
                if !is_zero_32(frac32) {
                    // Denormalized input flushed to zero
                    self.fp_process_exception(FPExc::InputDenorm, fpscr_val);
                }
                (FPType::Zero, 0.0f32)
            } else {
                (
                    FPType::Nonzero,
                    2.0f32.powf(-126.0) * (frac32 as f32 * 2.0f32.powf(-23.0)),
                )
            }
        } else if is_ones_32(exp32, 8) {
            if is_zero_32(frac32) {
                (FPType::Infinity, 2.0f32.powf(1000000.0))
            } else {
                (
                    if frac32.get_bit(22) {
                        FPType::QNaN
                    } else {
                        FPType::SNaN
                    },
                    0.0f32,
                )
            }
        } else {
            (
                FPType::Nonzero,
                2.0f32.powf(exp32 as f32 - 127.0) * (1.0 + frac32 as f32 * 2.0f32.powf(-23.0)),
            )
        };
        (ret_type, sign, value)
    }

    fn fp_unpack_f64(&mut self, fpval: u64, fpscr_val: u32) -> (FPType, bool, f64) {
        let sign = fpval.get_bit(63);
        let exp64 = fpval.get_bits(52..63);
        let frac64 = fpval.get_bits(0..52);

        let (ret_type, value) = if is_zero_64(exp64) {
            if is_zero_64(frac64) || fpscr_val.get_bit(24) {
                if !is_zero_64(frac64) {
                    // Denormalized input flushed to zero
                    self.fp_process_exception(FPExc::InputDenorm, fpscr_val);
                }
                (FPType::Zero, 0.0f64)
            } else {
                (
                    FPType::Nonzero,
                    2.0f64.powf(-1022.0) * (frac64 as f64 * 2.0f64.powf(-52.0)),
                )
            }
        } else if is_ones_64(exp64, 11) {
            if is_zero_64(frac64) {
                (FPType::Infinity, 2.0f64.powf(1000000.0))
            } else {
                (
                    if frac64.get_bit(51) {
                        FPType::QNaN
                    } else {
                        FPType::SNaN
                    },
                    0.0f64,
                )
            }
        } else {
            (
                FPType::Nonzero,
                2.0f64.powf(exp64 as f64 - 1023.0) * (1.0 + frac64 as f64 * 2.0f64.powf(-52.0)),
            )
        };
        (ret_type, sign, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Processor;

    #[test]
    fn test_fp_compare_f32() {
        let mut processor = Processor::new();
        assert_eq!(
            processor.fp_compare_f32(0x3F800000, 0x3F800000, false, false),
            (false, true, true, false)
        ); // 1.0 == 1.0
        assert_eq!(
            processor.fp_compare_f32(0x3F800000, 0x40000000, false, false),
            (true, false, false, false)
        ); // 1.0 < 2.0
        assert_eq!(
            processor.fp_compare_f32(0x40000000, 0x3F800000, false, false),
            (false, false, true, false)
        ); // 2.0 > 1.0
    }

    #[test]
    fn test_fp_compare_f64() {
        let mut processor = Processor::new();
        assert_eq!(
            processor.fp_compare_f64(0x3FF0000000000000, 0x3FF0000000000000, false, false),
            (false, true, true, false)
        ); // 1.0 == 1.0
        assert_eq!(
            processor.fp_compare_f64(0x3FF0000000000000, 0x4000000000000000, false, false),
            (true, false, false, false)
        ); // 1.0 < 2.0
        assert_eq!(
            processor.fp_compare_f64(0x4000000000000000, 0x3FF0000000000000, false, false),
            (false, false, true, false)
        ); // 2.0 > 1.0
    }

    #[test]
    fn test_fpabs_32() {
        assert_eq!(fpabs_32(0x80000000), 0x00000000); // -0.0 -> 0.0
        assert_eq!(fpabs_32(0xFFFFFFFF), 0x7FFFFFFF); // -1.0 -> 1.0
        assert_eq!(fpabs_32(0x7FFFFFFF), 0x7FFFFFFF); // 1.0 -> 1.0
    }

    #[test]
    fn test_fp_unpack_f32() {
        let mut processor = Processor::new();

        // 1.0
        assert_eq!(
            processor.fp_unpack_f32(0x3F800000, 0x00000000),
            (FPType::Nonzero, false, 1.0)
        );

        // 2.0
        assert_eq!(
            processor.fp_unpack_f32(0x40000000, 0x00000000),
            (FPType::Nonzero, false, 2.0)
        );

        // max value:
        assert_eq!(
            processor.fp_unpack_f32(0x7F7FFFFF, 0x00000000),
            (FPType::Nonzero, false, std::f32::MAX)
        );

        // 0.0
        assert_eq!(
            processor.fp_unpack_f32(0x00000000, 0x00000000),
            (FPType::Zero, false, 0.0f32)
        );

        // minimum positive value, non zero:
        assert_eq!(
            processor.fp_unpack_f32(0x00800000, 0x00000000),
            (FPType::Nonzero, false, std::f32::MIN_POSITIVE)
        );

        // Infinity
        assert_eq!(
            processor.fp_unpack_f32(0x7F800000, 0x00000000),
            (FPType::Infinity, false, std::f32::INFINITY)
        );

        // Negative infinity:
        assert_eq!(
            processor.fp_unpack_f32(0xFF800000, 0x00000000),
            (FPType::Infinity, true, std::f32::INFINITY)
        );

        // QNaN
        assert_eq!(
            processor.fp_unpack_f32(0x7FC00000, 0x00000000),
            (FPType::QNaN, false, 0.0)
        );

        // SNaN
        assert_eq!(
            processor.fp_unpack_f32(0x7F800001, 0x00000000),
            (FPType::SNaN, false, 0.0)
        );
    }

    #[test]
    fn test_fp_unpack_f64() {
        let mut processor = Processor::new();

        // 1.0
        assert_eq!(
            processor.fp_unpack_f64(0x3FF0000000000000, 0x00000000),
            (FPType::Nonzero, false, 1.0)
        );

        // 2.0
        assert_eq!(
            processor.fp_unpack_f64(0x4000000000000000, 0x00000000),
            (FPType::Nonzero, false, 2.0)
        );

        // max value:
        assert_eq!(
            processor.fp_unpack_f64(0x7FEFFFFFFFFFFFFF, 0x00000000),
            (FPType::Nonzero, false, std::f64::MAX)
        );

        // 0.0
        assert_eq!(
            processor.fp_unpack_f64(0x0000000000000000, 0x00000000),
            (FPType::Zero, false, 0.0f64)
        );

        // minimum positive value, non zero:
        assert_eq!(
            processor.fp_unpack_f64(0x0010000000000000, 0x00000000),
            (FPType::Nonzero, false, std::f64::MIN_POSITIVE)
        );

        // Infinity
        assert_eq!(
            processor.fp_unpack_f64(0x7FF0000000000000, 0x00000000),
            (FPType::Infinity, false, std::f64::INFINITY)
        );

        // Negative infinity:
        assert_eq!(
            processor.fp_unpack_f64(0xFFF0000000000000, 0x00000000),
            (FPType::Infinity, true, std::f64::INFINITY)
        );

        // QNaN
        assert_eq!(
            processor.fp_unpack_f64(0x7FF8000000000000, 0x00000000),
            (FPType::QNaN, false, 0.0)
        );

        // SNaN
        assert_eq!(
            processor.fp_unpack_f64(0x7FF0000000000001, 0x00000000),
            (FPType::SNaN, false, 0.0)
        );
    }
}
