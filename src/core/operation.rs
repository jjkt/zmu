use core::condition::Condition;
use core::register::Apsr;
use core::PSR;

pub fn sign_extend(word: u32, topbit: u8, size: u8) -> i32 {
    if word & (1 << topbit) == (1 << topbit) {
        return (word | (((1 << (size - topbit)) - 1) << topbit)) as i32;
    }
    word as i32
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
    let signed_sum = (x as i32) + (y as i32) + (carry_in as i32);
    let result = (unsigned_sum & 0xffff_ffff) as u32; // same value as signed_sum<N-1:0>
    let carry_out = u64::from(result) != unsigned_sum;
    let overflow = (result as i32) != signed_sum;

    (result, carry_out, overflow)
}

#[test]
fn test_add_with_carry() {

    let (result, carry, overflow) = add_with_carry(0x410, 4, false);
    assert!(result == 0x414);
    assert!(carry == false);
    assert!(overflow == false);
}


//
// This function performs the condition test for an instruction, based on:
// • the two Thumb conditional branch encodings, encodings T1 and T3 of the B instruction
// • the current values of the xPSR.IT[7:0] bits for other Thumb instructions.
//
pub fn condition_passed(condition: Condition, psr: &PSR) -> bool {

    match condition {
        Condition::EQ => psr.get_z(),
        Condition::NE => !psr.get_z(),
        Condition::CS => psr.get_c(),
        Condition::CC => !psr.get_c(),
        Condition::MI => psr.get_n(),
        Condition::PL => !psr.get_n(),

        Condition::VS => psr.get_v(),
        Condition::VC => !psr.get_v(),

        Condition::HI => psr.get_c() && psr.get_z(),
        Condition::LS => !(psr.get_c() && psr.get_z()),

        Condition::GE => psr.get_n() == psr.get_v(),
        Condition::LT => !(psr.get_n() == psr.get_v()),

        Condition::GT => (psr.get_n() == psr.get_v()) && !psr.get_z(),
        Condition::LE => !((psr.get_n() == psr.get_v()) && !psr.get_z()),

        Condition::AL => true,
    }
}
