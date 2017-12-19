#[inline]
pub fn bits_0_3(n: u16) -> u8 {
    n as u8 & 0b_111
}

#[inline]
pub fn bits_3_6(n: u16) -> u8 {
    ((n & 0b_111_000) >> 3) as u8
}

#[inline]
pub fn bits_6_9(n: u16) -> u8 {
    ((n & 0b_111_000_000) >> 6) as u8
}

#[inline]
pub fn bits_8_11(n: u16) -> u8 {
    ((n & 0b_111_00_000_000) >> 8) as u8
}

#[inline]
pub fn bits_0_8(n: u16) -> u8 {
    (n & 0xff) as u8
}
