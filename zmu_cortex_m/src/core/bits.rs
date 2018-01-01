#[inline]
pub fn bits_0_3(n: u16) -> u8 {
    n as u8 & 0b_111
}

#[inline]
pub fn bits_0_4(n: u16) -> u8 {
    n as u8 & 0b_1111
}

#[inline]
pub fn bits_0_7(n: u16) -> u8 {
    (n & 0b_111_1111) as u8
}

#[inline]
pub fn bits_0_8(n: u16) -> u8 {
    (n & 0b_1111_1111) as u8
}

#[inline]
pub fn bit_0(n: u16) -> u8 {
    ((n & 0b_0000_0001) >> 0) as u8
}

#[inline]
pub fn bit_1(n: u16) -> u8 {
    ((n & 0b_0000_0010) >> 1) as u8
}

#[inline]
pub fn bit_2(n: u16) -> u8 {
    ((n & 0b_0000_0100) >> 2) as u8
}

#[inline]
pub fn bit_3(n: u16) -> u8 {
    ((n & 0b_0000_1000) >> 3) as u8
}

#[inline]
pub fn bit_4(n: u16) -> u8 {
    ((n & 0b_0001_0000) >> 4) as u8
}

#[inline]
pub fn bit_5(n: u16) -> u8 {
    ((n & 0b_0010_0000) >> 5) as u8
}

#[inline]
pub fn bit_6(n: u16) -> u8 {
    ((n & 0b_0100_0000) >> 6) as u8
}

#[inline]
pub fn bit_7(n: u16) -> u8 {
    ((n & 0b_1000_0000) >> 7) as u8
}

#[inline]
pub fn bit_8(n: u16) -> u8 {
    ((n & 0b_1_0000_0000) >> 8) as u8
}

#[inline]
pub fn bits_0_11(n: u16) -> u16 {
    n & 0b_11_1111_1111
}



#[inline]
pub fn bits_3_6(n: u16) -> u8 {
    ((n & 0b_111_000) >> 3) as u8
}

#[inline]
pub fn bits_3_7(n: u16) -> u8 {
    ((n & 0b_1111_000) >> 3) as u8
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
pub fn bits_8_12(n: u16) -> u8 {
    ((n & 0b_1111_00_000_000) >> 8) as u8
}

#[inline]
pub fn bits_6_11(n: u16) -> u8 {
    ((n & 0b_11111_000_000) >> 6) as u8
}
