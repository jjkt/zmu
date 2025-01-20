use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use crate::{
    core::{
        bits::Bits,
        fpregister::{FPSCRRounding, Fpscr},
    },
    Processor,
};

use num_bigfloat::BigFloat;

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

/// Trait for floating point operations
/// This trait is generic over the underlying integer type (u32 or u64)
/// and provides methods for floating point operations.
pub trait FloatOps {
    // underlying integer type (u32 or u64)
    type Bits: PartialOrd + AddAssign + Div<Output = Self::Bits> + Copy;

    // signed version of the underlying integer type
    type SignedBits: Sized
        + PartialOrd
        + Default
        + Add<Output = Self::SignedBits>
        + AddAssign
        + Sub<Output = Self::SignedBits>
        + Ord
        + Copy;

    // Pseudo-real number type for unbounded precision calculations
    type Real: PartialOrd
        + Default
        + Copy
        + Add<Output = Self::Real>
        + Neg<Output = Self::Real>
        + Div<Output = Self::Real>
        + Sub<Output = Self::Real>
        + Mul<Output = Self::Real>
        + From<Self::SignedBits>
        + From<Self::Bits>;

    /// Number of bits in the underlying integer type
    fn n() -> usize;

    fn is_zero(value: Self::Bits) -> bool;

    /// Returns the default NaN value
    fn fp_default_nan() -> Self::Bits;
    fn fp_infinity(sign: bool) -> Self::Bits;
    fn fp_zero(sign: bool) -> Self::Bits;
    fn zero() -> Self::Bits;
    fn set_bit(value: Self::Bits, bit: usize, set: bool) -> Self::Bits;
    fn get_bit_unsigned(value: Self::Bits, bit: usize) -> bool;

    fn round_down(value: Self::Real) -> Self::Bits;

    fn float_value_zero_point_five() -> Self::Real;
    fn onef() -> Self::Real;
    fn powf2(e: Self::Real) -> Self::Real;

    fn unsigned_pow2(e: usize) -> Self::Bits;
    fn unsigned_pow2_f(e: usize) -> Self::Real;
    fn unsigned_value(value: i32) -> Self::Bits;

    fn signed_pow2(e: usize) -> Self::SignedBits;
    fn signed_value(value: i32) -> Self::SignedBits;

    fn fp_abs(value: Self::Bits) -> Self::Bits;
    fn fp_max_normal(sign: bool) -> Self::Bits;

    fn fp_unpack(fpval: Self::Bits, fpscr_val: u32) -> (FPType, bool, Self::Real, Option<FPExc>);

    fn normalize(mantissa: Self::Real) -> (Self::Real, Self::SignedBits);
    fn concate_bits(
        sign: bool,
        biased_exp: Self::SignedBits,
        int_mantissa: Self::Bits,
    ) -> Self::Bits;
}

pub trait FloatingPointChecks {
    fn execute_fp_check(&mut self);
    fn fp_process_exception(&mut self, exc: FPExc, fpscr_val: u32);
}

pub trait FloatingPointPublicOperations {
    fn fp_add<T: FloatOps>(
        &mut self,
        op1: T::Bits,
        op2: T::Bits,
        fpscr_controlled: bool,
    ) -> T::Bits;

    fn fp_sub<T: FloatOps>(
        &mut self,
        op1: T::Bits,
        op2: T::Bits,
        fpscr_controlled: bool,
    ) -> T::Bits;

    fn fp_compare<T: FloatOps>(
        &mut self,
        op1: T::Bits,
        op2: T::Bits,
        quiet_nan_exc: bool,
        fpscr_controlled: bool,
    ) -> (bool, bool, bool, bool);

    fn fp_abs<T: FloatOps>(&mut self, op: T::Bits) -> T::Bits;
}

trait FloatingPointHiddenOperations {
    fn fp_process_nan<T: FloatOps>(
        &mut self,
        type1: FPType,
        op: T::Bits,
        fpscr_val: u32,
    ) -> T::Bits;

    fn fp_process_nans<T: FloatOps>(
        &mut self,
        type1: FPType,
        type2: FPType,
        op1: T::Bits,
        op2: T::Bits,
        fpscr_val: u32,
    ) -> (bool, T::Bits);

    fn fp_unpack<T: FloatOps>(&mut self, fpval: T::Bits, fpscr_val: u32)
        -> (FPType, bool, T::Real);

    fn fp_round<T: FloatOps>(&mut self, value: T::Real, fpscr_val: u32) -> T::Bits;
}

impl FloatOps for u32 {
    type Bits = u32;
    type SignedBits = i32;
    type Real = BigFloat;

    fn n() -> usize {
        32
    }

    fn zero() -> Self::Bits {
        0
    }

    fn onef() -> Self::Real {
        num_bigfloat::ONE
    }

    fn float_value_zero_point_five() -> Self::Real {
        BigFloat::from(0.5)
    }

    fn set_bit(value: Self::Bits, bit: usize, set: bool) -> Self::Bits {
        let mut val = value;
        val.set_bit(bit, set);
        val
    }

    fn get_bit_unsigned(value: Self::Bits, bit: usize) -> bool {
        value.get_bit(bit)
    }

    fn powf2(e: Self::Real) -> Self::Real {
        BigFloat::from(2).pow(&e)
    }

    fn round_down(value: Self::Real) -> Self::Bits {
        // largest integer n so that n <= x
        value.floor().to_i64().unwrap().try_into().unwrap()
    }

    fn unsigned_value(value: i32) -> Self::Bits {
        value as Self::Bits
    }

    fn unsigned_pow2_f(e: usize) -> Self::Real {
        BigFloat::from(2).pow(&((e as u32).into()))
    }

    fn unsigned_pow2(e: usize) -> Self::Bits {
        2u32.pow(e as u32)
    }

    fn signed_pow2(e: usize) -> Self::SignedBits {
        2i32.pow(e as u32)
    }

    fn signed_value(value: i32) -> Self::SignedBits {
        value
    }

    fn fp_infinity(sign: bool) -> Self::Bits {
        if sign {
            0xFF800000
        } else {
            0x7F800000
        }
    }

    fn fp_zero(sign: bool) -> Self::Bits {
        if sign {
            0x80000000
        } else {
            0x00000000
        }
    }

    fn fp_default_nan() -> Self::Bits {
        0x7FC00000
    }

    fn fp_abs(value: Self::Bits) -> Self::Bits {
        value & 0x7FFFFFFF
    }

    fn is_zero(value: Self::Bits) -> bool {
        value == 0
    }

    fn fp_unpack(fpval: Self::Bits, fpscr_val: u32) -> (FPType, bool, Self::Real, Option<FPExc>) {
        let sign = fpval.get_bit(31);
        let exp32 = fpval.get_bits(23..31);
        let frac32 = fpval.get_bits(0..23);
        let mut exception = None;

        let (ret_type, mut value) = if Self::is_zero(exp32) {
            if Self::is_zero(frac32) || fpscr_val.get_bit(24) {
                if !Self::is_zero(frac32) {
                    // Denormalized input flushed to zero
                    exception = Some(FPExc::InputDenorm);
                }
                (FPType::Zero, Self::Real::default())
            } else {
                (
                    FPType::Nonzero,
                    BigFloat::from(2.0).pow(&BigFloat::from(-126.0))
                        * (BigFloat::from(frac32)
                            * BigFloat::from(2.0).pow(&BigFloat::from(-23.0))),
                )
            }
        } else if is_ones_32(exp32, 8) {
            if Self::is_zero(frac32) {
                (
                    FPType::Infinity,
                    BigFloat::from(2.0).pow(&BigFloat::from(1000000.0)),
                )
            } else {
                (
                    if frac32.get_bit(22) {
                        FPType::QNaN
                    } else {
                        FPType::SNaN
                    },
                    Self::Real::default(),
                )
            }
        } else {
            (
                FPType::Nonzero,
                BigFloat::from(2.0).pow(&(BigFloat::from(exp32) - BigFloat::from(127.0)))
                    * (BigFloat::from(1.0)
                        + BigFloat::from(frac32) * BigFloat::from(2.0).pow(&BigFloat::from(-23.0))),
            )
        };
        if sign {
            value = -value;
        }
        (ret_type, sign, value, exception)
    }

    fn fp_max_normal(sign: bool) -> Self::Bits {
        let exp: u32 = 0b1111_1110;
        let frac: u32 = 0b111_1111_1111_1111_1111_1111;

        ((sign as u32) << 31) + (exp << 23) + frac
    }

    fn normalize(mant: Self::Real) -> (Self::Real, Self::SignedBits) {
        let mut exponent: i32 = 0;
        let limit = BigFloat::from(1.0);
        let multiplier = BigFloat::from(2.0);
        let mut mantissa = mant;
        while mantissa < limit {
            mantissa *= multiplier;
            exponent -= 1;
        }
        while mantissa >= multiplier {
            mantissa /= multiplier;
            exponent += 1;
        }
        (mantissa, exponent)
    }

    fn concate_bits(
        sign: bool,
        biased_exp: Self::SignedBits,
        int_mantissa: Self::Bits,
    ) -> Self::Bits {
        let e = if Self::n() == 32 { 8 } else { 11 };
        let f = Self::n() - e - 1;

        let low: u32 = int_mantissa.get_bits(0..f as usize);
        let mid: u32 = (biased_exp as u32).get_bits(0..e as usize);
        let s = sign as u32;

        (s << 31) | (mid << f) | low
    }
}

impl FloatOps for u64 {
    type Bits = u64;
    type SignedBits = i64;
    type Real = BigFloat;

    fn fp_infinity(sign: bool) -> Self::Bits {
        if sign {
            0xFFF0000000000000
        } else {
            0x7FF0000000000000
        }
    }

    fn zero() -> Self::Bits {
        0
    }

    fn onef() -> Self::Real {
        num_bigfloat::ONE
    }

    fn float_value_zero_point_five() -> Self::Real {
        BigFloat::from(0.5)
    }

    fn n() -> usize {
        64
    }

    fn set_bit(value: Self::Bits, bit: usize, set: bool) -> Self::Bits {
        let mut val = value;
        val.set_bit(bit, set);
        val
    }

    fn get_bit_unsigned(value: Self::Bits, bit: usize) -> bool {
        value.get_bit(bit)
    }

    fn powf2(e: Self::Real) -> Self::Real {
        BigFloat::from(2).pow(&e)
    }

    fn round_down(value: Self::Real) -> Self::Bits {
        // largest integer n so that n <= x
        value.floor().to_i64().unwrap().try_into().unwrap()
    }

    fn unsigned_value(value: i32) -> Self::Bits {
        value as Self::Bits
    }

    fn unsigned_pow2_f(e: usize) -> Self::Real {
        BigFloat::from(2).pow(&((e as u32).into()))
    }

    fn unsigned_pow2(e: usize) -> Self::Bits {
        2u64.pow(e as u32)
    }

    fn signed_pow2(e: usize) -> Self::SignedBits {
        2i64.pow(e as u32)
    }

    fn signed_value(value: i32) -> Self::SignedBits {
        value as i64
    }

    fn fp_zero(sign: bool) -> Self::Bits {
        if sign {
            0x8000000000000000
        } else {
            0x0000000000000000
        }
    }

    fn fp_default_nan() -> Self::Bits {
        0x7FF8000000000000
    }

    fn is_zero(value: Self::Bits) -> bool {
        value == 0
    }

    fn fp_abs(value: Self::Bits) -> Self::Bits {
        value & 0x7FFFFFFFFFFFFFFF
    }

    fn fp_unpack(fpval: Self::Bits, fpscr_val: u32) -> (FPType, bool, Self::Real, Option<FPExc>) {
        let sign = fpval.get_bit(63);
        let exp64 = fpval.get_bits(52..63);
        let frac64 = fpval.get_bits(0..52);
        let mut exception = None;

        let (ret_type, mut value) = if Self::is_zero(exp64) {
            if Self::is_zero(frac64) || fpscr_val.get_bit(24) {
                if !Self::is_zero(frac64) {
                    // Denormalized input flushed to zero
                    exception = Some(FPExc::InputDenorm);
                }
                (FPType::Zero, Self::Real::default())
            } else {
                (
                    FPType::Nonzero,
                    BigFloat::from(2.0).pow(&BigFloat::from(-1022.0))
                        * (BigFloat::from(frac64)
                            * BigFloat::from(2.0).pow(&BigFloat::from(-52.0))),
                )
            }
        } else if is_ones_64(exp64, 11) {
            if Self::is_zero(frac64) {
                (
                    FPType::Infinity,
                    BigFloat::from(2.0).pow(&BigFloat::from(1000000.0)),
                )
            } else {
                (
                    if frac64.get_bit(51) {
                        FPType::QNaN
                    } else {
                        FPType::SNaN
                    },
                    Self::Real::default(),
                )
            }
        } else {
            (
                FPType::Nonzero,
                BigFloat::from(2.0).pow(&(BigFloat::from(exp64) - BigFloat::from(1023.0)))
                    * (BigFloat::from(1.0)
                        + BigFloat::from(frac64) * BigFloat::from(2.0).pow(&BigFloat::from(-52.0))),
            )
        };
        if sign {
            value = -value;
        }
        (ret_type, sign, value, exception)
    }

    fn fp_max_normal(sign: bool) -> Self::Bits {
        let exp: u64 = 0b1111_1110;
        let frac: u64 = 0b111_1111_1111_1111_1111_1111_1111_1111_1111_1111;

        ((sign as u64) << 63) + (exp << 52) + frac
    }

    fn normalize(mant: Self::Real) -> (Self::Real, Self::SignedBits) {
        let mut exponent: i64 = 0;
        let limit = BigFloat::from(1.0);
        let multiplier = BigFloat::from(2.0);
        let mut mantissa = mant;
        while mantissa < limit {
            mantissa *= multiplier;
            exponent -= 1;
        }
        while mantissa >= multiplier {
            mantissa /= multiplier;
            exponent += 1;
        }
        (mantissa, exponent)
    }

    fn concate_bits(
        sign: bool,
        biased_exp: Self::SignedBits,
        int_mantissa: Self::Bits,
    ) -> Self::Bits {
        let e = if Self::n() == 32 { 8 } else { 11 };
        let f = Self::n() - e - 1;
        let low: u64 = int_mantissa.get_bits(0..f as usize);
        let mid: u64 = (biased_exp as u64).get_bits(0..e as usize);
        let s = sign as u64;
        (s << 63) | (mid << f) | low
    }
}

fn standard_fpscr_value(fpscr: u32) -> u32 {
    (0b0_0000 << 27) | ((fpscr.get_bit(26) as u32) << 26) | 0b11_0000_0000_0000_0000_0000_0000
}

impl FloatingPointHiddenOperations for Processor {
    fn fp_process_nan<T: FloatOps>(
        &mut self,
        type1: FPType,
        op: T::Bits,
        fpscr_val: u32,
    ) -> T::Bits {
        let topfrac = if T::n() == 32 { 22 } else { 51 };
        let mut result = op;
        if type1 == FPType::SNaN {
            result = T::set_bit(result, topfrac, true);
            self.fp_process_exception(FPExc::InvalidOp, fpscr_val);
        }
        if fpscr_val.get_dn() {
            result = T::fp_default_nan();
        }
        result
    }

    fn fp_process_nans<T: FloatOps>(
        &mut self,
        type1: FPType,
        type2: FPType,
        op1: T::Bits,
        op2: T::Bits,
        fpscr_val: u32,
    ) -> (bool, T::Bits) {
        if type1 == FPType::SNaN {
            let result = self.fp_process_nan::<T>(type1, op1, fpscr_val);
            return (true, result);
        } else if type2 == FPType::SNaN {
            let result = self.fp_process_nan::<T>(type2, op2, fpscr_val);
            return (true, result);
        } else if type1 == FPType::QNaN {
            let result = self.fp_process_nan::<T>(type1, op1, fpscr_val);
            return (true, result);
        } else if type2 == FPType::QNaN {
            let result = self.fp_process_nan::<T>(type2, op2, fpscr_val);
            return (true, result);
        } else {
            return (false, T::zero());
        }
    }

    fn fp_unpack<T: FloatOps>(
        &mut self,
        fpval: T::Bits,
        fpscr_val: u32,
    ) -> (FPType, bool, T::Real) {
        let (fptype, sign, value, exception) = T::fp_unpack(fpval, fpscr_val);
        if let Some(exc) = exception {
            self.fp_process_exception(exc, fpscr_val);
        }
        (fptype, sign, value)
    }

    fn fp_round<T: FloatOps>(&mut self, value: T::Real, fpscr_val: u32) -> T::Bits {
        assert!(value != T::Real::default());

        let e = if T::n() == 32 { 8 } else { 11 };
        let minimum_exp: T::SignedBits = T::signed_value(2) - T::signed_pow2(e - 1);

        let f = T::n() - e - 1;
        let (sign, mantissa) = if value < T::Real::default() {
            (true, -value)
        } else {
            (false, value)
        };

        let (mut mantissa, exponent) = T::normalize(mantissa);

        if fpscr_val.get_fz() && exponent < minimum_exp {
            self.fpscr.set_ufc(true);
            T::fp_zero(sign)
        } else {
            let mut biased_exp = T::SignedBits::max(
                exponent - minimum_exp + T::signed_value(1),
                T::signed_value(0),
            );
            if biased_exp == T::SignedBits::default() {
                mantissa = mantissa / T::powf2((minimum_exp - exponent).into());
            }
            let pow2_f = T::unsigned_pow2_f(f);
            let mut int_mantissa = T::round_down(mantissa * pow2_f);
            let mut error = mantissa * pow2_f - (int_mantissa).into();

            if biased_exp == T::SignedBits::default()
                && (error != T::Real::default() || fpscr_val.get_bit(11))
            {
                self.fp_process_exception(FPExc::Underflow, fpscr_val);
            }

            let (round_up, overflow_to_inf) = match fpscr_val.get_rounding_mode() {
                FPSCRRounding::RoundToNearest => (
                    (error > T::float_value_zero_point_five()
                        || (error == T::float_value_zero_point_five()
                            && T::get_bit_unsigned(int_mantissa, 0))),
                    true,
                ),
                FPSCRRounding::RoundTowardsPlusInfinity => {
                    (error != T::Real::default() && !sign, !sign)
                }
                FPSCRRounding::RoundTowardsMinusInfinity => {
                    (error != T::Real::default() && sign, sign)
                }
                FPSCRRounding::RoundTowardsZero => (false, false),
            };
            if round_up {
                int_mantissa += T::unsigned_value(1);
                if int_mantissa == T::unsigned_pow2(f) {
                    biased_exp = T::signed_value(1);
                }
                if int_mantissa == T::unsigned_pow2(f + 1) {
                    biased_exp += T::signed_value(1);
                    int_mantissa = int_mantissa / T::unsigned_value(2);
                }
            }

            let result = if biased_exp >= T::signed_pow2(e) - T::signed_value(1) {
                let result = if overflow_to_inf {
                    T::fp_infinity(sign)
                } else {
                    T::fp_max_normal(sign)
                };
                self.fp_process_exception(FPExc::Overflow, fpscr_val);
                error = T::onef();
                result
            } else {
                // concatenate following bits:
                // sign: biased_exp(0..=e-1) : int_mant(0..=f-1)
                T::concate_bits(sign, biased_exp, int_mantissa)
            };

            if error != T::Real::default() {
                self.fp_process_exception(FPExc::Inexact, fpscr_val);
            }
            result
        }
    }
}

impl FloatingPointPublicOperations for Processor {
    fn fp_add<T: FloatOps>(
        &mut self,
        op1: T::Bits,
        op2: T::Bits,
        fpscr_controlled: bool,
    ) -> T::Bits {
        let fpscr_val = if fpscr_controlled {
            self.fpscr
        } else {
            standard_fpscr_value(self.fpscr)
        };
        let (type1, sign1, value1) = self.fp_unpack::<T>(op1, fpscr_val);
        let (type2, sign2, value2) = self.fp_unpack::<T>(op2, fpscr_val);
        let (done, result) = self.fp_process_nans::<T>(type1, type2, op1, op2, fpscr_val);
        if !done {
            let inf1 = type1 == FPType::Infinity;
            let inf2 = type2 == FPType::Infinity;
            let zero1 = type1 == FPType::Zero;
            let zero2 = type2 == FPType::Zero;
            let result = if inf1 && inf2 && sign1 != sign2 {
                let res = T::fp_default_nan();
                self.fp_process_exception(FPExc::InvalidOp, fpscr_val);
                res
            } else if (inf1 && sign1 == false) || (inf2 && sign2 == false) {
                T::fp_infinity(false)
            } else if (inf1 && sign1 == true) || (inf2 && sign2 == true) {
                T::fp_infinity(true)
            } else if zero1 && zero2 && sign1 == sign2 {
                T::fp_zero(sign1)
            } else {
                // Calculate in 'real' numbers, with unbounded size and precision.
                // As in real life there must be some limit, we use the defined Real type limits
                let result_value = value1 + value2;
                if result_value == T::Real::default() {
                    // Sign of exact zero result depends on rounding mode
                    T::fp_zero(
                        fpscr_val.get_rounding_mode() == FPSCRRounding::RoundTowardsMinusInfinity,
                    )
                } else {
                    let rounded = self.fp_round::<T>(result_value, fpscr_val);
                    rounded
                }
            };
            result
        } else {
            result
        }
    }

    fn fp_sub<T: FloatOps>(
        &mut self,
        op1: T::Bits,
        op2: T::Bits,
        fpscr_controlled: bool,
    ) -> T::Bits {
        let fpscr_val = if fpscr_controlled {
            self.fpscr
        } else {
            standard_fpscr_value(self.fpscr)
        };
        let (type1, sign1, value1) = self.fp_unpack::<T>(op1, fpscr_val);
        let (type2, sign2, value2) = self.fp_unpack::<T>(op2, fpscr_val);
        let (done, result) = self.fp_process_nans::<T>(type1, type2, op1, op2, fpscr_val);
        if !done {
            let inf1 = type1 == FPType::Infinity;
            let inf2 = type2 == FPType::Infinity;
            let zero1 = type1 == FPType::Zero;
            let zero2 = type2 == FPType::Zero;
            let result = if inf1 && inf2 && sign1 != sign2 {
                let res = T::fp_default_nan();
                self.fp_process_exception(FPExc::InvalidOp, fpscr_val);
                res
            } else if (inf1 && sign1 == false) || (inf2 && sign2 == false) {
                T::fp_infinity(false)
            } else if (inf1 && sign1 == true) || (inf2 && sign2 == false) {
                T::fp_infinity(true)
            } else if zero1 && zero2 && sign1 == sign2 {
                T::fp_zero(sign1)
            } else {
                let result_value = value1 - value2;
                if result_value == T::Real::default() {
                    // Sign of exact zero result depends on rounding mode
                    T::fp_zero(
                        fpscr_val.get_rounding_mode() == FPSCRRounding::RoundTowardsMinusInfinity,
                    )
                } else {
                    let rounded = self.fp_round::<T>(result_value, fpscr_val);
                    rounded
                }
            };
            result
        } else {
            result
        }
    }

    fn fp_compare<T: FloatOps>(
        &mut self,
        op1: T::Bits,
        op2: T::Bits,
        quiet_nan_exc: bool,
        fpscr_controlled: bool,
    ) -> (bool, bool, bool, bool) {
        let fpscr_val = if fpscr_controlled {
            self.fpscr
        } else {
            standard_fpscr_value(self.fpscr)
        };

        let (type1, _sign1, value1) = self.fp_unpack::<T>(op1, fpscr_val);
        let (type2, _sign2, value2) = self.fp_unpack::<T>(op2, fpscr_val);

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

    fn fp_abs<T: FloatOps>(&mut self, op: T::Bits) -> T::Bits {
        T::fp_abs(op)
    }
}

fn is_ones_32(value: u32, len: usize) -> bool {
    value.count_ones() == len as u32
}

fn is_ones_64(value: u64, len: usize) -> bool {
    value.count_ones() == len as u32
}

impl FloatingPointChecks for Processor {
    fn execute_fp_check(&mut self) {
        // todo!()
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Processor;

    #[test]
    fn test_fp_compare_f32() {
        let mut processor = Processor::new();
        assert_eq!(
            processor.fp_compare::<u32>(0x3F800000, 0x3F800000, false, false),
            (false, true, true, false)
        ); // 1.0 == 1.0
        assert_eq!(
            processor.fp_compare::<u32>(0x3F800000, 0x40000000, false, false),
            (true, false, false, false)
        ); // 1.0 < 2.0
        assert_eq!(
            processor.fp_compare::<u32>(0x40000000, 0x3F800000, false, false),
            (false, false, true, false)
        ); // 2.0 > 1.0
    }

    #[test]
    fn test_fp_compare_f64() {
        let mut processor = Processor::new();

        // 1.0 == 1.0
        assert_eq!(
            processor.fp_compare::<u64>(0x3FF0000000000000, 0x3FF0000000000000, false, false),
            (false, true, true, false)
        );

        // 1.0 < 2.0
        assert_eq!(
            processor.fp_compare::<u64>(0x3FF0000000000000, 0x4000000000000000, false, false),
            (true, false, false, false)
        );
        // 2.0 > 1.0
        assert_eq!(
            processor.fp_compare::<u64>(0x4000000000000000, 0x3FF0000000000000, false, false),
            (false, false, true, false)
        );
    }

    #[test]
    fn test_fpabs_32() {
        let mut processor = Processor::new();

        // -0.0 -> 0.0
        assert_eq!(processor.fp_abs::<u32>(0x80000000), 0x00000000);

        // -1.0 -> 1.0
        assert_eq!(processor.fp_abs::<u32>(0xFFFFFFFF), 0x7FFFFFFF);

        // 1.0 -> 1.0
        assert_eq!(processor.fp_abs::<u32>(0x7FFFFFFF), 0x7FFFFFFF);

        // most negative value:
        assert_eq!(
            processor.fp_abs::<u32>((-std::f32::MAX).to_bits()),
            std::f32::MAX.to_bits()
        );
    }
    #[test]
    fn test_fpabs_64() {
        let mut processor = Processor::new();
        // -0.0 -> 0.0
        assert_eq!(
            processor.fp_abs::<u64>(0x8000000000000000),
            0x0000000000000000
        );
        // -1.0 -> 1.0
        assert_eq!(
            processor.fp_abs::<u64>(0xFFFFFFFFFFFFFFFF),
            0x7FFFFFFFFFFFFFFF
        );
        // 1.0 -> 1.0
        assert_eq!(
            processor.fp_abs::<u64>(0x7FFFFFFFFFFFFFFF),
            0x7FFFFFFFFFFFFFFF
        );

        // min value
        assert_eq!(
            processor.fp_abs::<u64>((-std::f64::MAX).to_bits()),
            std::f64::MAX.to_bits()
        );
    }

    #[test]
    fn test_fp_unpack_f32() {
        let mut processor = Processor::new();

        // unpack extracts the sign, exponent and mantissa from a 32-bit float u32 represenantation

        // 1.0
        assert_eq!(
            processor.fp_unpack::<u32>(0x3F800000, 0x00000000),
            (FPType::Nonzero, false, BigFloat::from(1.0))
        );

        // -1.0
        assert_eq!(
            processor.fp_unpack::<u32>(0xBF800000, 0x00000000),
            (FPType::Nonzero, true, BigFloat::from(-1.0))
        );

        // 2.0
        assert_eq!(
            processor.fp_unpack::<u32>(0x40000000, 0x00000000),
            (FPType::Nonzero, false, BigFloat::from(2.0))
        );

        // max value
        assert_eq!(
            processor.fp_unpack::<u32>(0x7F7FFFFF, 0x00000000),
            (
                FPType::Nonzero,
                false,
                BigFloat::from(340282346638528859811704183484516925440u128)
            )
        );

        // 0.0
        assert_eq!(
            processor.fp_unpack::<u32>(0x00000000, 0x00000000),
            (FPType::Zero, false, BigFloat::default())
        );

        // minimum positive value, non zero:
        assert_eq!(
            processor.fp_unpack::<u32>(0x00800000, 0x00000000),
            (
                FPType::Nonzero,
                false,
                BigFloat::parse("1.175494350822287507968736537222245677819e-38").unwrap()
            )
        );

        // Infinity
        assert_eq!(
            processor.fp_unpack::<u32>(0x7F800000, 0x00000000),
            (FPType::Infinity, false, num_bigfloat::INF_POS)
        );

        // Negative infinity:
        assert_eq!(
            processor.fp_unpack::<u32>(0xFF800000, 0x00000000),
            (FPType::Infinity, true, num_bigfloat::INF_NEG)
        );

        // QNaN
        assert_eq!(
            processor.fp_unpack::<u32>(0x7FC00000, 0x00000000),
            (FPType::QNaN, false, BigFloat::default())
        );

        // SNaN
        assert_eq!(
            processor.fp_unpack::<u32>(0x7F800001, 0x00000000),
            (FPType::SNaN, false, BigFloat::default())
        );
    }

    #[test]
    fn test_fp_unpack_f64() {
        let mut processor = Processor::new();

        // 1.0
        assert_eq!(
            processor.fp_unpack::<u64>(0x3FF0000000000000, 0x00000000),
            (FPType::Nonzero, false, BigFloat::from(1.0))
        );

        // -1.0
        assert_eq!(
            processor.fp_unpack::<u64>(0xBFF0000000000000, 0x00000000),
            (FPType::Nonzero, true, BigFloat::from(-1.0))
        );

        // 2.0
        assert_eq!(
            processor.fp_unpack::<u64>(0x4000000000000000, 0x00000000),
            (FPType::Nonzero, false, BigFloat::from(2.0))
        );

        // max value:
        assert_eq!(
            processor.fp_unpack::<u64>(0x7FEFFFFFFFFFFFFF, 0x00000000),
            (FPType::Nonzero, false, BigFloat::from(std::f64::MAX))
        );

        // 0.0
        assert_eq!(
            processor.fp_unpack::<u64>(0x0000000000000000, 0x00000000),
            (FPType::Zero, false, BigFloat::default())
        );

        // minimum positive value, non zero:
        assert_eq!(
            processor.fp_unpack::<u64>(0x0010000000000000, 0x00000000),
            (
                FPType::Nonzero,
                false,
                BigFloat::from(std::f64::MIN_POSITIVE)
            )
        );

        // Infinity
        assert_eq!(
            processor.fp_unpack::<u64>(0x7FF0000000000000, 0x00000000),
            (FPType::Infinity, false, num_bigfloat::INF_POS)
        );

        // Negative infinity:
        assert_eq!(
            processor.fp_unpack::<u64>(0xFFF0000000000000, 0x00000000),
            (FPType::Infinity, true, num_bigfloat::INF_NEG)
        );

        // QNaN
        assert_eq!(
            processor.fp_unpack::<u64>(0x7FF8000000000000, 0x00000000),
            (FPType::QNaN, false, BigFloat::default())
        );

        // SNaN
        assert_eq!(
            processor.fp_unpack::<u64>(0x7FF0000000000001, 0x00000000),
            (FPType::SNaN, false, BigFloat::default())
        );
    }
    #[test]
    fn test_fp_round_f32() {
        let mut processor = Processor::new();

        // Test all rounding modes:
        // FPSCRRounding::RoundToNearest
        // FPSCRRounding::RoundTowardsPlusInfinity
        // FPSCRRounding::RoundTowardsMinusInfinity
        // FPSCRRounding::RoundTowardsZero,

        // Default rounding mode
        processor
            .fpscr
            .set_rounding_mode(FPSCRRounding::RoundToNearest);

        // 1.0 -> 0x3F800000 exact
        assert_eq!(
            processor.fp_round::<u32>(BigFloat::from(1.0), 0),
            0x3F800000
        );

        // 0.33333333333333333333 -> 3eaaaaab
        assert_eq!(
            processor.fp_round::<u32>(BigFloat::from(0.33333333333333333333), 0),
            0x3eaaaaab
        );
    }

    #[test]
    fn test_fp_add_f32() {
        let mut processor = Processor::new();

        // 1.0 + 1.0 = 2.0
        assert_eq!(
            processor.fp_add::<u32>(0x3F800000, 0x3F800000, true),
            0x40000000
        );

        // 1.0 + 2.0 = 3.0
        assert_eq!(
            processor.fp_add::<u32>(0x3F800000, 0x40000000, true),
            0x40400000
        );

        // -1.0 + 2.0 = 1.0
        assert_eq!(
            processor.fp_add::<u32>(0xBF800000, 0x40000000, true),
            0x3F800000
        );
    }

    #[test]
    fn test_fp_sub_f32() {
        let mut processor = Processor::new();

        // 1.0 - 1.0 = 0.0
        assert_eq!(
            processor.fp_sub::<u32>(0x3F800000, 0x3F800000, true),
            0x00000000
        );

        // 2.0 - 1.0 = 1.0
        assert_eq!(
            processor.fp_sub::<u32>(0x40000000, 0x3F800000, true),
            0x3F800000
        );

        // 1.0 - 2.0 = -1.0
        assert_eq!(
            processor.fp_sub::<u32>(0x3F800000, 0x40000000, true),
            0xBF800000
        );
    }

    #[test]
    fn test_fp_round_f64() {
        let mut processor = Processor::new();

        // 1.0 -> 0x3FF0000000000000 exact
        assert_eq!(
            processor.fp_round::<u64>(BigFloat::from(1.0f64), 0),
            0x3FF0000000000000
        );
    }
}
