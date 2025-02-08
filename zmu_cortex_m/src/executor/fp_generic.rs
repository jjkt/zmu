use std::ops::{Add, AddAssign, Div, Sub};

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

// i128 is used to represent the "integer" pseudocode type in the Architecture Reference Manual
fn round_down(value: BigFloat) -> i128 {
    // largest integer n so that n <= x
    let floored = value.floor();

    if floored.is_nan() {
        return 0;
    }

    let result = floored.to_i128().unwrap_or(if value < BigFloat::default() {
        std::i128::MIN + 1
    } else {
        std::i128::MAX
    });
    result
}

/// Trait for floating point operations
/// This trait is generic over the underlying integer type (u32 or u64)
/// and provides methods for floating point operations.
pub trait FloatOps {
    // underlying integer type (u32 or u64)
    type Bits: PartialOrd
        + AddAssign
        + Div<Output = Self::Bits>
        + Sub<Output = Self::Bits>
        + Into<u64>
        + Copy;

    // signed version of the underlying integer type
    type SignedBits: Sized
        + PartialOrd
        + Default
        + Add<Output = Self::SignedBits>
        + AddAssign
        + Sub<Output = Self::SignedBits>
        + Ord
        + Copy
        + Into<BigFloat>;

    /// Number of bits in the underlying integer type
    fn n() -> usize;

    fn is_zero(value: Self::Bits) -> bool;

    /// Returns the default NaN value
    fn fp_default_nan() -> Self::Bits;
    fn fp_infinity(sign: bool) -> Self::Bits;
    fn fp_zero(sign: bool) -> Self::Bits;
    fn zero() -> Self::Bits;
    fn set_bit(value: Self::Bits, bit: usize, set: bool) -> Self::Bits;

    fn powf2(e: BigFloat) -> BigFloat;

    fn unsigned_pow2_f(e: usize) -> BigFloat;

    fn integer_to_bits(value: i128) -> Self::Bits;
    fn signed_pow2(e: usize) -> Self::SignedBits;
    fn signed_value(value: i32) -> Self::SignedBits;

    fn fp_abs(value: Self::Bits) -> Self::Bits;
    fn fp_max_normal(sign: bool) -> Self::Bits;
    fn from_integer(value: u64) -> Self::Bits;

    fn unsigned_to_signed(value: Self::Bits) -> Self::SignedBits;
    fn bits_to_bigfloat(value: Self::Bits) -> BigFloat;
    fn signedbits_to_bigfloat(value: Self::SignedBits) -> BigFloat;

    fn fp_unpack(fpval: Self::Bits, fpscr_val: u32) -> (FPType, bool, BigFloat, Option<FPExc>);

    fn normalize(mantissa: BigFloat) -> (BigFloat, Self::SignedBits);
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

    fn fp_to_fixed<N: FloatOps, M: FloatOps>(
        &mut self,
        op: N::Bits,
        fraction_bits: usize,
        unsigned: bool,
        round_towards_zero: bool,
        fpscr_controlled: bool,
    ) -> M::Bits;

    fn fixed_to_fp<N: FloatOps, M: FloatOps>(
        &mut self,
        op: M::Bits,
        fraction_bits: usize,
        unsigned: bool,
        round_to_nearest: bool,
        fpscr_controlled: bool,
    ) -> N::Bits;
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

    fn fp_unpack<T: FloatOps>(
        &mut self,
        fpval: T::Bits,
        fpscr_val: u32,
    ) -> (FPType, bool, BigFloat);

    fn fp_round<T: FloatOps>(&mut self, value: BigFloat, fpscr_val: u32) -> T::Bits;
}

impl FloatOps for u32 {
    type Bits = u32;
    type SignedBits = i32;

    fn n() -> usize {
        32
    }

    fn zero() -> Self::Bits {
        0
    }

    fn set_bit(value: Self::Bits, bit: usize, set: bool) -> Self::Bits {
        let mut val = value;
        val.set_bit(bit, set);
        val
    }

    fn powf2(e: BigFloat) -> BigFloat {
        BigFloat::from(2).pow(&e)
    }

    fn integer_to_bits(value: i128) -> Self::Bits {
        value as Self::Bits
    }

    fn unsigned_pow2_f(e: usize) -> BigFloat {
        BigFloat::from(2).pow(&((e as u32).into()))
    }

    fn signed_pow2(e: usize) -> Self::SignedBits {
        2i32.pow(e as u32)
    }

    fn signed_value(value: i32) -> Self::SignedBits {
        value
    }

    fn unsigned_to_signed(value: Self::Bits) -> Self::SignedBits {
        value as Self::SignedBits
    }

    fn bits_to_bigfloat(value: Self::Bits) -> BigFloat {
        BigFloat::from(value)
    }

    fn signedbits_to_bigfloat(value: Self::SignedBits) -> BigFloat {
        BigFloat::from(value)
    }

    fn from_integer(value: u64) -> Self::Bits {
        value as Self::Bits
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

    fn fp_unpack(fpval: Self::Bits, fpscr_val: u32) -> (FPType, bool, BigFloat, Option<FPExc>) {
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
                (FPType::Zero, BigFloat::default())
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
                    BigFloat::default(),
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

    fn normalize(mant: BigFloat) -> (BigFloat, Self::SignedBits) {
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

    fn n() -> usize {
        64
    }

    fn set_bit(value: Self::Bits, bit: usize, set: bool) -> Self::Bits {
        let mut val = value;
        val.set_bit(bit, set);
        val
    }

    fn integer_to_bits(value: i128) -> Self::Bits {
        value as Self::Bits
    }

    fn powf2(e: BigFloat) -> BigFloat {
        BigFloat::from(2).pow(&e)
    }

    fn unsigned_pow2_f(e: usize) -> BigFloat {
        BigFloat::from(2).pow(&((e as u32).into()))
    }

    fn signed_pow2(e: usize) -> Self::SignedBits {
        2i64.pow(e as u32)
    }

    fn signed_value(value: i32) -> Self::SignedBits {
        value as i64
    }

    fn unsigned_to_signed(value: Self::Bits) -> Self::SignedBits {
        value as Self::SignedBits
    }

    fn bits_to_bigfloat(value: Self::Bits) -> BigFloat {
        BigFloat::from(value)
    }

    fn signedbits_to_bigfloat(value: Self::SignedBits) -> BigFloat {
        BigFloat::from(value)
    }

    fn from_integer(value: u64) -> Self::Bits {
        value
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

    fn fp_unpack(fpval: Self::Bits, fpscr_val: u32) -> (FPType, bool, BigFloat, Option<FPExc>) {
        let sign = fpval.get_bit(63);
        let exp64 = fpval.get_bits(52..63);
        let frac64 = fpval.get_bits(0..52);
        let mut exception = None;

        let (ret_type, mut value) = if Self::is_zero(exp64) {
            if Self::is_zero(frac64) || fpscr_val.get_bit(24) {
                if !Self::is_zero(frac64) {
                    exception = Some(FPExc::InputDenorm);
                }
                (FPType::Zero, BigFloat::default())
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
                    BigFloat::default(),
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

    fn normalize(mant: BigFloat) -> (BigFloat, Self::SignedBits) {
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

/// saturate i to n bits, return the result and a boolean indicating if saturation occurred (up to 64 bits)
fn satq(i: i128, n: usize, unsigned: bool) -> (u64, bool) {
    if unsigned {
        let limit = 2u128.pow(n as u32) - 1;
        let i_unsigned = i as u128;

        let (result, saturated) = if i_unsigned > limit {
            (limit, true)
        } else {
            (i_unsigned, false)
        };

        return (result.get_bits(0..n) as u64, saturated);
    } else {
        let limit = 2i128.pow(n as u32 - 1) - 1;
        let neg_limit = -(2i128.pow(n as u32 - 1));

        let (result, saturated) = if i > limit {
            (limit, true)
        } else if i < neg_limit {
            (neg_limit, true)
        } else {
            (i, false)
        };

        return ((result as u64).get_bits(0..n), saturated);
    }
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
    ) -> (FPType, bool, BigFloat) {
        let (fptype, sign, value, exception) = T::fp_unpack(fpval, fpscr_val);
        if let Some(exc) = exception {
            self.fp_process_exception(exc, fpscr_val);
        }
        (fptype, sign, value)
    }

    fn fp_round<T: FloatOps>(&mut self, value: BigFloat, fpscr_val: u32) -> T::Bits {
        assert!(value != BigFloat::default());

        let e = if T::n() == 32 { 8 } else { 11 };
        let minimum_exp: T::SignedBits = T::signed_value(2) - T::signed_pow2(e - 1);

        let f = T::n() - e - 1;
        let (sign, mantissa) = if value < BigFloat::default() {
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
            let mut int_mantissa = round_down(mantissa * pow2_f);
            let mut error: BigFloat = mantissa * pow2_f - BigFloat::from(int_mantissa);

            if biased_exp == T::SignedBits::default()
                && (error != BigFloat::default() || fpscr_val.get_bit(11))
            {
                self.fp_process_exception(FPExc::Underflow, fpscr_val);
            }

            let (round_up, overflow_to_inf) = match fpscr_val.get_rounding_mode() {
                FPSCRRounding::RoundToNearest => (
                    (error > BigFloat::from(0.5)
                        || (error == BigFloat::from(0.5) && int_mantissa.get_bit(0))),
                    true,
                ),
                FPSCRRounding::RoundTowardsPlusInfinity => {
                    (error != BigFloat::default() && !sign, !sign)
                }
                FPSCRRounding::RoundTowardsMinusInfinity => {
                    (error != BigFloat::default() && sign, sign)
                }
                FPSCRRounding::RoundTowardsZero => (false, false),
            };
            if round_up {
                int_mantissa += 1;
                if int_mantissa == 2i128.pow(f as u32) {
                    biased_exp = T::signed_value(1);
                }
                if int_mantissa == 2i128.pow(f as u32 + 1) {
                    biased_exp += T::signed_value(1);
                    int_mantissa = int_mantissa / 2;
                }
            }

            let result = if biased_exp >= T::signed_pow2(e) - T::signed_value(1) {
                let result = if overflow_to_inf {
                    T::fp_infinity(sign)
                } else {
                    T::fp_max_normal(sign)
                };
                self.fp_process_exception(FPExc::Overflow, fpscr_val);
                error = num_bigfloat::ONE;
                result
            } else {
                // concatenate following bits:
                // sign: biased_exp(0..=e-1) : int_mant(0..=f-1)
                T::concate_bits(sign, biased_exp, T::integer_to_bits(int_mantissa))
            };

            if error != BigFloat::default() {
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
                if result_value == BigFloat::default() {
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
                if result_value == BigFloat::default() {
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

    fn fixed_to_fp<N: FloatOps, M: FloatOps>(
        &mut self,
        op: M::Bits,
        fraction_bits: usize,
        unsigned: bool,
        round_to_nearest: bool,
        fpscr_controlled: bool,
    ) -> N::Bits {
        let mut fpscr_val = if fpscr_controlled {
            self.fpscr
        } else {
            standard_fpscr_value(self.fpscr)
        };
        if round_to_nearest {
            fpscr_val.set_rounding_mode(FPSCRRounding::RoundToNearest);
        }

        let real_operand = if unsigned {
            let int_operand = M::Bits::from(op);
            BigFloat::div(
                &M::bits_to_bigfloat(int_operand),
                &M::powf2(BigFloat::from(fraction_bits as u32)),
            )
        } else {
            let int_operand = M::unsigned_to_signed(op);
            BigFloat::div(
                &M::signedbits_to_bigfloat(int_operand),
                &M::powf2(BigFloat::from(fraction_bits as u32)),
            )
        };

        if real_operand == BigFloat::default() {
            return N::zero();
        } else {
            return self.fp_round::<N>(real_operand, fpscr_val);
        }
    }

    fn fp_to_fixed<N: FloatOps, M: FloatOps>(
        &mut self,
        op: N::Bits,
        fraction_bits: usize,
        unsigned: bool,
        round_towards_zero: bool,
        fpscr_controlled: bool,
    ) -> M::Bits {
        let mut fpscr_val = if fpscr_controlled {
            self.fpscr
        } else {
            standard_fpscr_value(self.fpscr)
        };

        if round_towards_zero {
            fpscr_val.set_rounding_mode(FPSCRRounding::RoundTowardsZero);
        }
        let (type_t, _sign, mut value) = self.fp_unpack::<N>(op, fpscr_val);
        if type_t == FPType::SNaN || type_t == FPType::QNaN {
            self.fp_process_exception(FPExc::InvalidOp, fpscr_val);
        }

        value = value * N::powf2(BigFloat::from(fraction_bits as u32));
        let mut int_result = round_down(value);
        let error = value - BigFloat::from(int_result);

        let round_up = match fpscr_val.get_rounding_mode() {
            FPSCRRounding::RoundToNearest => {
                error > BigFloat::from(0.5)
                    || (error == BigFloat::from(0.5) && int_result.get_bit(0) == false)
            }
            FPSCRRounding::RoundTowardsPlusInfinity => error != BigFloat::default(),
            FPSCRRounding::RoundTowardsMinusInfinity => false,
            FPSCRRounding::RoundTowardsZero => error != BigFloat::default() && int_result < 0,
        };
        if round_up {
            int_result = int_result.saturating_add(1);
        }

        let (result, overflow) = satq(int_result, M::n(), unsigned);

        if overflow {
            self.fp_process_exception(FPExc::Overflow, fpscr_val);
        } else if error != BigFloat::default() {
            self.fp_process_exception(FPExc::Inexact, fpscr_val);
        }

        M::from_integer(result)
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

    #[test]
    fn test_satq() {
        // 1.0 -> 1 (unsigned, 32 bit)
        assert_eq!(satq(1, 32, true), (1, false));

        // 1.0 -> 1 (signed, 32 bit)
        assert_eq!(satq(1, 32, false), (1, false));

        // 1.0 -> 1 (unsigned, 64 bit)
        assert_eq!(satq(1, 64, true), (1, false));

        // 1.0 -> 1 (signed, 64 bit)
        assert_eq!(satq(1, 64, false), (1, false));

        // -1.0 -> 0 (signed, 32 bit)
        assert_eq!(satq(-1, 32, false), (0xffff_ffff, false));

        // value bigger than max value (unsigned, 32 bit)
        assert_eq!(satq(0x1_0000_0000, 32, true), (0xffff_ffff, true));

        // value bigger than max value (signed, 32 bit)
        assert_eq!(satq(0x8000_0000, 32, false), (0x7fff_ffff, true));

        // value smaller than min value (signed, 32 bit)
        assert_eq!(satq(-0x8000_0001, 32, false), (0x8000_0000, true));

        // value bigger than max value (unsigned, 64 bit)
        assert_eq!(
            satq(0x1_0000_0000_0000_0000, 64, true),
            (0xffff_ffff_ffff_ffff, true)
        );

        // value bigger than max value (signed, 64 bit)
        assert_eq!(
            satq(0x8000_0000_0000_0000, 64, false),
            (0x7fff_ffff_ffff_ffff, true)
        );

        // value smaller than min value (signed, 64 bit)
        assert_eq!(
            satq(-0x8000_0000_0000_0001, 64, false),
            (0x8000_0000_0000_0000, true)
        );

        // value smaller than min value (unsigned, 64 bit)
        assert_eq!(satq(-1, 64, true), (0xffff_ffff_ffff_ffff, true));
    }

    #[test]
    fn test_fp_to_fixed_f32_s32() {
        let mut processor = Processor::new();

        // 1.0 -> 1 (signed)
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0x3F800000, 0, false, false, false),
            0x00000001
        );

        // 0.00001 -> 0 (signed) (rounding towards zero)
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0x3C23D70A, 0, false, false, true),
            0x00000000
        );

        // 42.0 -> 42 (signed)
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0x42280000, 0, false, false, false),
            0x0000002A
        );

        // -1.0 -> -1 (signed)
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0xBF800000, 0, false, false, false),
            0xFFFFFFFF
        );

        // -42.0 -> -42 (signed)
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0xC2280000, 0, false, false, false),
            0xFFFFFFD6
        );

        // positive infinity -> max value
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0x7F800000, 0, false, false, false),
            0x7FFFFFFF
        );

        // negative infinity -> min value
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0xFF800000, 0, false, false, false),
            0x80000000
        );

        // QNAN -> 0
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0x7FC00000, 0, false, false, false),
            0x00000000
        );

        // SNAN -> 0
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0x7F800001, 0, false, false, false),
            0x00000000
        );

    }

    #[test]
    fn test_fp_to_fixed_f32_u32() {
        let mut processor = Processor::new();

        // 1.0 -> 1 (unsigned)
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0x3F800000, 0, true, false, false),
            0x00000001
        );

        // 42.0 -> 42 (unsigned)
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0x42280000, 0, true, false, false),
            0x0000002A
        );

        // positive infinity -> max value
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0x7F800000, 0, true, false, false),
            0xFFFFFFFF
        );

        // negative infinity -> max value
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0xFF800000, 0, true, false, false),
            0xFFFFFFFF
        );

        // QNAN -> 0
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0x7FC00000, 0, true, false, false),
            0x00000000
        );

        //SNAN -> 0
        assert_eq!(
            processor.fp_to_fixed::<u32, u32>(0x7F800001, 0, true, false, false),
            0x00000000
        );
    }

    #[test]
    fn test_fp_to_fixed_f64_s32() {
        let mut processor = Processor::new();

        // 1.0 -> 1 (signed)
        assert_eq!(
            processor.fp_to_fixed::<u64, u32>(0x3FF0000000000000, 0, false, false, false),
            0x00000001
        );

        // 42.0 -> 42 (signed)
        assert_eq!(
            processor.fp_to_fixed::<u64, u32>(0x4045000000000000, 0, false, false, false),
            0x0000002A
        );

        // -1.0 -> -1 (signed)
        assert_eq!(
            processor.fp_to_fixed::<u64, u32>(0xBFF0000000000000, 0, false, false, false),
            0xFFFFFFFF
        );

        // -42.0 -> -42 (signed)
        assert_eq!(
            processor.fp_to_fixed::<u64, u32>(0xC045000000000000, 0, false, false, false),
            0xFFFFFFD6
        );

        // positive infinity -> max value
        assert_eq!(
            processor.fp_to_fixed::<u64, u32>(0x7FF0000000000000, 0, false, false, false),
            0x7FFFFFFF
        );

        // negative infinity -> min value
        assert_eq!(
            processor.fp_to_fixed::<u64, u32>(0xFFF0000000000000, 0, false, false, false),
            0x80000000
        );

        // QNAN -> 0
        assert_eq!(
            processor.fp_to_fixed::<u64, u32>(0x7FF8000000000000, 0, false, false, false),
            0x00000000
        );

        // SNAN -> 0
        assert_eq!(
            processor.fp_to_fixed::<u64, u32>(0x7FF0000000000001, 0, false, false, false),
            0x00000000
        );
    }

    #[test]
    fn test_fp_to_fixed_f64_u32() {
        let mut processor = Processor::new();

        // 1.0 -> 1 (unsigned)
        assert_eq!(
            processor.fp_to_fixed::<u64, u32>(0x3FF0000000000000, 0, true, false, false),
            0x00000001
        );

        // 42.0 -> 42 (unsigned)
        assert_eq!(
            processor.fp_to_fixed::<u64, u32>(0x4045000000000000, 0, true, false, false),
            0x0000002A
        );

        // positive infinity -> max value
        assert_eq!(
            processor.fp_to_fixed::<u64, u32>(0x7FF0000000000000, 0, true, false, false),
            0xFFFFFFFF
        );

        // negative infinity -> max value
        assert_eq!(
            processor.fp_to_fixed::<u64, u32>(0xFFF0000000000000, 0, true, false, false),
            0xFFFFFFFF
        );

        // QNAN -> 0
        assert_eq!(
            processor.fp_to_fixed::<u64, u32>(0x7FF8000000000000, 0, true, false, false),
            0x00000000
        );

        // SNAN -> 0
        assert_eq!(
            processor.fp_to_fixed::<u64, u32>(0x7FF0000000000001, 0, true, false, false),
            0x00000000
        );

    }

    #[test]
    fn test_fixed_to_fp_s32_f32() {
        let mut processor = Processor::new();


        // 1 -> 1.0 (signed)
        assert_eq!(
            processor.fixed_to_fp::<u32, u32>(1, 0, false, false, false),
            0x3F800000
        );

        // 42 -> 42.0 (signed)
        assert_eq!(
            processor.fixed_to_fp::<u32, u32>(42, 0, false, false, false),
            0x42280000
        );

        // -1 -> -1.0 (signed)
        assert_eq!(
            processor.fixed_to_fp::<u32, u32>(0xffff_ffff, 0, false, false, false),
            0xBF800000
        );



    }

}
