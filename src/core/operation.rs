use core::condition::Condition;
use core::register::Apsr;

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
    let unsigned_sum = x as u64 + y as u64 + (carry_in as u64);
    let signed_sum = (x as i32) + (y as i32) + (carry_in as i32);
    let result = (unsigned_sum & 0xffffffff) as u32; // same value as signed_sum<N-1:0>
    let carry_out = (result as u64) != unsigned_sum;
    let overflow = (result as i32) != signed_sum;

    (result, carry_out, overflow)
}

//
// This function performs the condition test for an instruction, based on:
// • the two Thumb conditional branch encodings, encodings T1 and T3 of the B instruction
// • the current values of the xPSR.IT[7:0] bits for other Thumb instructions.
//
pub fn condition_passed(condition: Condition, aspr: &u32) -> bool {

    match condition {
        Condition::EQ => aspr.get_z(),
        Condition::NE => !aspr.get_z(),
        Condition::CS => aspr.get_c(),
        Condition::CC => !aspr.get_c(),
        Condition::MI => aspr.get_n(),
        Condition::PL => !aspr.get_n(),

        Condition::VS => aspr.get_v(),
        Condition::VC => !aspr.get_v(),

        Condition::HI => aspr.get_c() && aspr.get_z(),
        Condition::LS => !(aspr.get_c() && aspr.get_z()),

        Condition::GE => aspr.get_n() == aspr.get_v(),
        Condition::LT => !(aspr.get_n() == aspr.get_v()),

        Condition::GT => (aspr.get_n() == aspr.get_v()) && !aspr.get_z(),
        Condition::LE => !((aspr.get_n() == aspr.get_v()) && !aspr.get_z()),

        Condition::AL => true,
    }
}
