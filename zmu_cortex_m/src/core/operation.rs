use bit_field::BitField;
use core::bits::bit_31;
use core::condition::Condition;
use core::instruction::SRType;
use core::register::Apsr;
use core::PSR;

pub fn sign_extend(word: u32, topbit: usize, size: usize) -> u64 {
    if word & (1 << topbit) == (1 << topbit) {
        return u64::from(word) | u64::from(((1_u64 << (size - topbit)) - 1) << topbit);
    }
    word as u64
}

//
// Add two numbers and carry
//
// x + y + carry
//
// return tuple of (result, carry, overflow)
//
pub fn add_with_carry(x: u32, y: u32, carry_in: bool) -> (u32, bool, bool) {
    let unsigned_sum = u64::from(x) + u64::from(y) + (carry_in as u64);
    let signed_sum = (x as i32)
        .wrapping_add(y as i32)
        .wrapping_add(carry_in as i32);
    let result = (unsigned_sum & 0xffff_ffff) as u32; // same value as signed_sum<N-1:0>
    let carry_out = u64::from(result) != unsigned_sum;
    let overflow = (result as i32) != signed_sum;

    (result, carry_out, overflow)
}

//
// This function performs the condition test for an instruction, based on:
// • the two Thumb conditional branch encodings, encodings T1 and T3 of the B instruction
// • the current values of the xPSR.IT[7:0] bits for other Thumb instructions.
//
pub fn condition_test(condition: &Condition, psr: &PSR) -> bool {
    match *condition {
        Condition::EQ => psr.get_z(),
        Condition::NE => !psr.get_z(),
        Condition::CS => psr.get_c(),
        Condition::CC => !psr.get_c(),
        Condition::MI => psr.get_n(),
        Condition::PL => !psr.get_n(),

        Condition::VS => psr.get_v(),
        Condition::VC => !psr.get_v(),

        Condition::HI => psr.get_c() && !psr.get_z(),
        Condition::LS => !(psr.get_c() && !psr.get_z()),

        Condition::GE => psr.get_n() == psr.get_v(),
        Condition::LT => !(psr.get_n() == psr.get_v()),

        Condition::GT => (psr.get_n() == psr.get_v()) && !psr.get_z(),
        Condition::LE => !((psr.get_n() == psr.get_v()) && !psr.get_z()),

        Condition::AL => true,
    }
}

// Decode immedate shift type
// input: bits[2], immedate
// output: (shitft type, immedate to use)
//
pub fn decode_imm_shift(typebits: u8, imm5: u8) -> (SRType, u8) {
    match typebits.get_bits(0..3) {
        0b00 => (SRType::LSL, imm5),
        0b01 => (SRType::LSR, if imm5 == 0 { 32 } else { imm5 }),
        0b10 => (SRType::ASR, if imm5 == 0 { 32 } else { imm5 }),
        0b11 => match imm5 {
            0 => (SRType::RRX, 1),
            _ => (SRType::ROR, imm5),
        },
        _ => panic!("invalid typebits"),
    }
}

fn lsl_c(value: u32, shift: usize) -> (u32, bool) {
    assert!(shift > 0);
    let extended = u64::from(value) << shift;

    (extended.get_bits(0..32) as u32, extended.get_bit(32))
}

fn lsl(value: u32, shift: usize) -> u32 {
    assert!(shift > 0);

    if shift == 0 {
        value
    } else {
        let (result, _) = lsl_c(value, shift);
        result
    }
}

fn lsr_c(value: u32, shift: usize) -> (u32, bool) {
    assert!(shift > 0);

    let extended = u64::from(value);

    (
        extended.get_bits(shift..(shift + 32)) as u32,
        extended.get_bit(shift - 1),
    )
}

fn lsr(value: u32, shift: usize) -> u32 {
    assert!(shift > 0);

    if shift == 0 {
        value
    } else {
        let (result, _) = lsr_c(value, shift);
        result
    }
}

fn asr_c(value: u32, shift: usize) -> (u32, bool) {
    assert!(shift > 0);

    let extended = sign_extend(value, 31, 32 + shift);

    (
        extended.get_bits(shift..(shift + 32)) as u32,
        extended.get_bit(shift - 1),
    )
}

fn ror_c(value: u32, shift: usize) -> (u32, bool) {
    assert!(shift > 0);
    let m = shift % 32;
    let result = lsr(value, m) | lsl(value, 32 - m);
    let carry_out = bit_31(result) == 1;
    (result, carry_out)
}

///
/// Do the one of the different shifting operations, with carry in support
///
/// Parameters:
/// - value is the number on which to apply the shifting on
/// - shift_t selects the shift type to use
/// - amount declares number of bits to shift and
/// - carry_in gives the input carry bit. Carry in is only used for RRX type shifting.
///
/// Returns:
/// - shifted value
/// - carry out
pub fn shift_c(value: u32, shift_t: SRType, amount: usize, carry_in: bool) -> (u32, bool) {
    assert!(!((shift_t == SRType::RRX) && (amount != 1)));
    if amount == 0 {
        (value, carry_in)
    } else {
        match shift_t {
            SRType::LSL => lsl_c(value, amount),
            SRType::LSR => lsr_c(value, amount),
            SRType::ASR => asr_c(value, amount),
            SRType::ROR => ror_c(value, amount),
            _ => panic!("not implemented"),
        }
    }
}

pub fn thumb_expand_imm() -> u32 {
    0
}

pub fn thumb_expand_imm_c() -> u32 {
    0
}

pub fn zero_extend(params: &[u8], lengths: &[u8]) -> u32 {
    assert_eq!(params.len(), lengths.len());

    let mut result : u32 = 0;
    let mut shift = 0;
    for (param, length) in params.iter().rev().zip(lengths.iter().rev()) {
        result += (*param as u32) << shift;
        shift += length;
    }

    result
}

pub fn build_imm_10_11(opcode: u32) -> i32 {
    let t1 = opcode >> 16;
    let t2 = opcode & 0xffff;

    let s = ((t1 >> 10) & 1) as u32;
    let imm10 = (t1 & 0x3ff) as u32;

    let j1 = ((t2 >> 13) & 1) as u32;
    let j2 = ((t2 >> 11) & 1) as u32;
    let imm11 = (t2 & 0x7ff) as u32;

    let i1 = ((j1 ^ s) ^ 1) as u32;
    let i2 = ((j2 ^ s) ^ 1) as u32;

    sign_extend(
        (imm11 << 1) + (imm10 << 12) + (i2 << 22) + (i1 << 23) + (s << 24),
        24,
        32,
    ) as i32
}

pub fn build_imm_6_11(opcode: u32) -> i32 {
    let t1 = opcode >> 16;
    let t2 = opcode & 0xffff;

    let s = ((t1 >> 10) & 1) as u32;
    let imm6 = (t1 & 0x3f) as u32;

    let j1 = ((t2 >> 13) & 1) as u32;
    let j2 = ((t2 >> 11) & 1) as u32;
    let imm11 = (t2 & 0x7ff) as u32;

    let i1 = ((j1 ^ s) ^ 1) as u32;
    let i2 = ((j2 ^ s) ^ 1) as u32;

    sign_extend(
        (imm11 << 1) + (imm6 << 12) + (i2 << 17) + (i1 << 18) + (s << 19),
        19,
        32,
    ) as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_shift_c() {
        {
            let (result, carry) = shift_c(0xFFFFFFF8, SRType::ASR, 8, false);
            assert!(result == 0xFFFFFFFF);
            assert!(carry == true);
        }
        {
            let (result, carry) = shift_c(0xef, SRType::ASR, 9, false);
            assert!(result == 0);
            assert!(carry == false);
        }
        {
            let (result, carry) = shift_c(0xFFFFFFC0, SRType::ASR, 1, false);
            assert!(result == 0xFFFFFFE0);
            assert!(carry == false);
        }

        {
            let (result, carry) = shift_c(0, SRType::ROR, 0, false);
            assert!(result == 0x0);
            assert!(carry == false);
        }
        {
            let (result, carry) = shift_c(2, SRType::ROR, 1, false);
            assert!(result == 0x1);
            assert!(carry == false);
        }
        {
            let (result, carry) = shift_c(1, SRType::ROR, 1, false);
            assert!(result == 0x8000_0000);
            assert!(carry == false);
        }
    }

    #[test]
    fn test_add_with_carry() {
        let (result, carry, overflow) = add_with_carry(0x410, 4, false);
        assert_eq!(result, 0x414);
        assert_eq!(carry, false);
        assert_eq!(overflow, false);
    }

    #[test]
    fn test_add_with_carry_basic() {
        let (result, carry, overflow) = add_with_carry(0x0, 0xffff_ffff, false);
        assert_eq!(result, 0xffff_ffff);
        assert_eq!(carry, false);
        assert_eq!(overflow, false);
    }

    #[test]
    fn test_add_with_carry_basic2() {
        let (result, carry, overflow) = add_with_carry(0x0, 0xffff_ffff, true);
        assert_eq!(result, 0);
        assert_eq!(carry, true);
        assert_eq!(overflow, false);
    }

    #[test]
    fn test_add_with_carry_basic3() {
        let (result, carry, overflow) = add_with_carry(0x0, 0, true);
        assert_eq!(result, 1);
        assert_eq!(carry, false);
        assert_eq!(overflow, false);
    }

    #[test]
    fn test_add_with_carry_basic4() {
        let (result, carry, overflow) = add_with_carry(0xffff_ffff, 0, true);
        assert_eq!(result, 0);
        assert_eq!(carry, true);
        assert_eq!(overflow, false);
    }

    #[test]
    fn test_add_with_carry_basic5() {
        let (result, carry, overflow) = add_with_carry(0xffff_ffff, 0xffff_ffff, true);
        assert_eq!(result, 0xffff_ffff);
        assert_eq!(carry, true);
        assert_eq!(overflow, false);
    }

    #[test]
    fn test_add_with_carry_basic6() {
        let (result, carry, overflow) = add_with_carry(0xffff_ffff, 0xffff_ffff, false);
        assert_eq!(result, 0xffff_fffe);
        assert_eq!(carry, true);
        assert_eq!(overflow, false);
    }

}
