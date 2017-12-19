#[inline]
pub fn bits_0_3(n: u16) -> u8 {
    n as u8 & 0b_111
}

#[inline]
pub fn bits_3_6(n: u16) -> u8 {
    ((n & 0b_111000) >> 3) as u8
}
