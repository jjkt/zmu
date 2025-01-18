use crate::{
    core::{
        bits::Bits,
        fpregister::{FPSCRRounding, Fpscr},
    },
    Processor,
};

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
    (0b0_0000 << 27) | ((fpscr.get_bit(26) as u32) << 26) | 0b11_0000_0000_0000_0000_0000_0000
}
pub trait FloatingPointInternalOperations {

    fn execute_fp_check(&mut self);

    fn fp_process_exception(&mut self, exc: FPExc, fpscr_val: u32);
    fn fp_process_nan_f32(&mut self, type1: FPType, op: u32, fpscr_val: u32) -> u32;
    fn fp_process_nans_f32(
        &mut self,
        type1: FPType,
        type2: FPType,
        op1: u32,
        op2: u32,
        fpscr_val: u32,
    ) -> (bool, u32);

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


    fn fp_unpack_f32(&mut self, fpval: u32, fpscr_val: u32) -> (FPType, bool, f32);
    fn fp_unpack_f64(&mut self, fpval: u64, fpscr_val: u32) -> (FPType, bool, f64);
    fn fp_round_f32(&mut self, value: f32, fpscr_val: u32) -> u32;

    fn fp_add_f32(&mut self, op1: u32, op2: u32, fpscr_controlled: bool) -> u32;
    fn fp_add_f64(&mut self, op1: u64, op2: u64, fpscr_controlled: bool) -> u64;

    fn fp_sub_f32(&mut self, op1: u32, op2: u32, fpscr_controlled: bool) -> u32;
    fn fp_sub_f64(&mut self, op1: u64, op2: u64, fpscr_controlled: bool) -> u64;
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

fn fp_default_nan_f32() -> u32 {
    0x7FC00000
}

fn fp_default_nan_f64() -> u64 {
    0x7FF8000000000000
}

fn fp_infinity_f32(sign: bool) -> u32 {
    if sign {
        0xFF800000
    } else {
        0x7F800000
    }
}

fn fp_infinity_f64(sign: bool) -> u64 {
    if sign {
        0xFFF0000000000000
    } else {
        0x7FF0000000000000
    }
}

fn fp_zero_f32(sign: bool) -> u32 {
    if sign {
        0x80000000
    } else {
        0x00000000
    }
}

fn fp_zero_f64(sign: bool) -> u64 {
    if sign {
        0x8000000000000000
    } else {
        0x0000000000000000
    }
}

fn fp_max_normal_f32(sign: bool) -> u32 {
    let exp: u32 = 0b1111_1110;
    let frac: u32 = 0b111_1111_1111_1111_1111_1111;

    ((sign as u32) << 31) + (exp << 23) + frac
}

fn round_down_f32(value: f32) -> u32 {
    // largest integer n so that n <= x
    value.floor() as u32
}

impl FloatingPointInternalOperations for Processor {

    fn execute_fp_check(&mut self)
    {
        // todo!()
    }

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

    fn fp_process_nan_f32(&mut self, type1: FPType, op: u32, fpscr_val: u32) -> u32 {
        let topfrac = 22;
        let mut result = op;
        if type1 == FPType::SNaN {
            result.set_bit(topfrac, true);
            self.fp_process_exception(FPExc::InvalidOp, fpscr_val);
        }
        if fpscr_val.get_dn() {
            result = fp_default_nan_f32();
        }
        result
    }

    fn fp_process_nans_f32(
        &mut self,
        type1: FPType,
        type2: FPType,
        op1: u32,
        op2: u32,
        fpscr_val: u32,
    ) -> (bool, u32) {
        if type1 == FPType::SNaN {
            let result = self.fp_process_nan_f32(type1, op1, fpscr_val);
            return (true, result);
        } else if type2 == FPType::SNaN {
            let result = self.fp_process_nan_f32(type2, op2, fpscr_val);
            return (true, result);
        } else if type1 == FPType::QNaN {
            let result = self.fp_process_nan_f32(type1, op1, fpscr_val);
            return (true, result);
        } else if type2 == FPType::QNaN {
            let result = self.fp_process_nan_f32(type2, op2, fpscr_val);
            return (true, result);
        } else {
            return (false, 0);
        }
    }

    fn fp_unpack_f32(&mut self, fpval: u32, fpscr_val: u32) -> (FPType, bool, f32) {
        let sign = fpval.get_bit(31);
        let exp32 = fpval.get_bits(23..31);
        let frac32 = fpval.get_bits(0..23);

        let (ret_type, mut value) = if is_zero_32(exp32) {
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
        if sign {
            value = -value;
        }
        (ret_type, sign, value)
    }

    fn fp_unpack_f64(&mut self, fpval: u64, fpscr_val: u32) -> (FPType, bool, f64) {
        let sign = fpval.get_bit(63);
        let exp64 = fpval.get_bits(52..63);
        let frac64 = fpval.get_bits(0..52);

        let (ret_type, mut value) = if is_zero_64(exp64) {
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
        if sign {
            value = -value;
        }
        (ret_type, sign, value)
    }

    fn fp_add_f32(&mut self, op1: u32, op2: u32, fpscr_controlled: bool) -> u32 {
        let fpscr_val = if fpscr_controlled {
            self.fpscr
        } else {
            standard_fpscr_value(self.fpscr)
        };
        let (type1, sign1, value1) = self.fp_unpack_f32(op1, fpscr_val);
        let (type2, sign2, value2) = self.fp_unpack_f32(op2, fpscr_val);
        let (done, result) = self.fp_process_nans_f32(type1, type2, op1, op2, fpscr_val);
        if !done {
            let inf1 = type1 == FPType::Infinity;
            let inf2 = type2 == FPType::Infinity;
            let zero1 = type1 == FPType::Zero;
            let zero2 = type2 == FPType::Zero;
            let result = if inf1 && inf2 && sign1 != sign2 {
                let res = fp_default_nan_f32();
                self.fp_process_exception(FPExc::InvalidOp, fpscr_val);
                res
            } else if (inf1 && sign1 == false) || (inf2 && sign2 == false) {
                fp_infinity_f32(false)
            } else if (inf1 && sign1 == true) || (inf2 && sign2 == true) {
                fp_infinity_f32(true)
            } else if zero1 && zero2 && sign1 == sign2 {
                fp_zero_f32(sign1)
            } else {
                let result_value = value1 + value2;
                if result_value == 0.0 {
                    // Sign of exact zero result depends on rounding mode
                    fp_zero_f32(
                        fpscr_val.get_rounding_mode() == FPSCRRounding::RoundTowardsMinusInfinity,
                    )
                } else {
                    let rounded = self.fp_round_f32(result_value, fpscr_val);
                    rounded
                }
            };
            result
        } else {
            result
        }
    }

    fn fp_add_f64(&mut self, _op1: u64, _op2: u64, _fpscr_controlled: bool) -> u64 {
        todo!()
    }

    fn fp_sub_f32(&mut self, op1: u32, op2: u32, fpscr_controlled: bool) -> u32 {
        let fpscr_val = if fpscr_controlled {
            self.fpscr
        } else {
            standard_fpscr_value(self.fpscr)
        };
        let (type1, sign1, value1) = self.fp_unpack_f32(op1, fpscr_val);
        let (type2, sign2, value2) = self.fp_unpack_f32(op2, fpscr_val);
        let (done, result) = self.fp_process_nans_f32(type1, type2, op1, op2, fpscr_val);
        if !done {
            let inf1 = type1 == FPType::Infinity;
            let inf2 = type2 == FPType::Infinity;
            let zero1 = type1 == FPType::Zero;
            let zero2 = type2 == FPType::Zero;
            let result = if inf1 && inf2 && sign1 != sign2 {
                let res = fp_default_nan_f32();
                self.fp_process_exception(FPExc::InvalidOp, fpscr_val);
                res
            } else if (inf1 && sign1 == false) || (inf2 && sign2 == false) {
                fp_infinity_f32(false)
            } else if (inf1 && sign1 == true) || (inf2 && sign2 == false) {
                fp_infinity_f32(true)
            } else if zero1 && zero2 && sign1 == sign2 {
                fp_zero_f32(sign1)
            } else {
                let result_value = value1 - value2;
                if result_value == 0.0 {
                    // Sign of exact zero result depends on rounding mode
                    fp_zero_f32(
                        fpscr_val.get_rounding_mode() == FPSCRRounding::RoundTowardsMinusInfinity,
                    )
                } else {
                    let rounded = self.fp_round_f32(result_value, fpscr_val);
                    rounded
                }
            };
            result
        } else {
            result
        }
    }

    fn fp_sub_f64(&mut self, _op1: u64, _op2: u64, _fpscr_controlled: bool) -> u64 {
        todo!()
    }


    fn fp_round_f32(&mut self, value: f32, fpscr_val: u32) -> u32 {
        assert!(value != 0.0f32);

        let e = 8;
        let minimum_exp: i32 = 2 - 2i32.pow(e - 1);
        let f = 32 - e - 1;
        let (sign, mut mantissa) = if value < 0.0 {
            (true, -value)
        } else {
            (false, value)
        };
        let mut exponent: i32 = 0;
        while mantissa < 1.0f32 {
            mantissa *= 2.0f32;
            exponent -= 1;
        }
        while mantissa >= 2.0f32 {
            mantissa /= 2.0f32;
            exponent += 1;
        }

        if fpscr_val.get_fz() && exponent < minimum_exp {
            self.fpscr.set_ufc(true);
            fp_zero_f32(sign)
        } else {
            let mut biased_exp = i32::max(exponent - minimum_exp + 1, 0);
            if biased_exp == 0 {
                mantissa = mantissa / 2.0f32.powf((minimum_exp - exponent) as f32);
            }
            let pow2_f = 2u32.pow(f) as f32;
            let mut int_mantissa = round_down_f32(mantissa * pow2_f);
            let mut error = mantissa * pow2_f - int_mantissa as f32;

            if biased_exp == 0 && (error != 0.0f32 || fpscr_val.get_bit(11)) {
                self.fp_process_exception(FPExc::Underflow, fpscr_val);
            }

            let (round_up, overflow_to_inf) = match self.fpscr.get_rounding_mode() {
                FPSCRRounding::RoundToNearest => (
                    (error > 0.5f32 || (error == 0.5f32 && int_mantissa.get_bit(0))),
                    true,
                ),
                FPSCRRounding::RoundTowardsPlusInfinity => (error != 0.0f32 && !sign, !sign),
                FPSCRRounding::RoundTowardsMinusInfinity => (error != 0.0f32 && sign, sign),
                FPSCRRounding::RoundTowardsZero => (false, false),
            };
            if round_up {
                int_mantissa += 1;
                if int_mantissa == 2u32.pow(f) {
                    biased_exp = 1;
                }
                if int_mantissa == 2u32.pow(f + 1) {
                    biased_exp += 1;
                    int_mantissa = int_mantissa / 2;
                }
            }

            let result = if biased_exp >= 2i32.pow(e) - 1 {
                let result = if overflow_to_inf {
                    fp_infinity_f32(sign)
                } else {
                    fp_max_normal_f32(sign)
                };
                self.fp_process_exception(FPExc::Overflow, fpscr_val);
                error = 1.0f32;
                result
            } else {
                // concatenate following bits:
                // sign: biased_exp(0..=e-1) : int_mant(0..=f-1)
                let low: u32 = int_mantissa.get_bits(0..f as usize);
                let mid: u32 = (biased_exp as u32).get_bits(0..e as usize);
                let s = sign as u32;

                (s << 31) | (mid << f) | low
            };

            if error != 0.0f32 {
                self.fp_process_exception(FPExc::Inexact, fpscr_val);
            }
            result
        }
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

        // -1.0
        assert_eq!(
            processor.fp_unpack_f32(0xBF800000, 0x00000000),
            (FPType::Nonzero, true, -1.0)
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
            (FPType::Infinity, true, std::f32::NEG_INFINITY)
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

        // -1.0
        assert_eq!(
            processor.fp_unpack_f64(0xBFF0000000000000, 0x00000000),
            (FPType::Nonzero, true, -1.0)
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
            (FPType::Infinity, true, std::f64::NEG_INFINITY)
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
    #[test]
    fn test_fp_round_f32() {
        let mut processor = Processor::new();

        // 1.0 -> 0x3F800000 exact
        assert_eq!(processor.fp_round_f32(1.0f32, 0), 0x3F800000);
    }

    #[test]
    fn test_fp_add_f32() {
        let mut processor = Processor::new();

        // 1.0 + 1.0 = 2.0
        assert_eq!(
            processor.fp_add_f32(0x3F800000, 0x3F800000, true),
            0x40000000
        );

        // 1.0 + 2.0 = 3.0
        assert_eq!(
            processor.fp_add_f32(0x3F800000, 0x40000000, true),
            0x40400000
        );

        // -1.0 + 2.0 = 1.0
        assert_eq!(
            processor.fp_add_f32(0xBF800000, 0x40000000, true),
            0x3F800000
        );
    }
}
