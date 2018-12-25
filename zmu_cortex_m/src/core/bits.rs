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
    (n & 0b111_1111) as u8
}

#[inline]
pub fn bits_0_8(n: u16) -> u8 {
    (n & 0b1111_1111) as u8
}

#[inline]
pub fn bit_0(n: u16) -> u8 {
    (n & 0b0000_0001) as u8
}

#[inline]
pub fn bit_1(n: u16) -> u8 {
    ((n & 0b0000_0010) >> 1) as u8
}

#[inline]
pub fn bit_2(n: u16) -> u8 {
    ((n & 0b0000_0100) >> 2) as u8
}

#[inline]
pub fn bit_3(n: u16) -> u8 {
    ((n & 0b0000_1000) >> 3) as u8
}

#[inline]
pub fn bit_4(n: u16) -> u8 {
    ((n & 0b0001_0000) >> 4) as u8
}

#[inline]
pub fn bit_5(n: u16) -> u8 {
    ((n & 0b0010_0000) >> 5) as u8
}

#[inline]
pub fn bit_6(n: u16) -> u8 {
    ((n & 0b0100_0000) >> 6) as u8
}

#[inline]
pub fn bit_7(n: u16) -> u8 {
    ((n & 0b1000_0000) >> 7) as u8
}

#[inline]
pub fn bit_8(n: u16) -> u8 {
    ((n & 0b1_0000_0000) >> 8) as u8
}

#[inline]
pub fn bit_31(n: u32) -> u32 {
    ((n & 0b1000_0000_0000_0000) >> 31) as u32
}

#[inline]
pub fn bits_31_28(n: u32) -> u32 {
    ((n & 0b1111_0000_0000_0000_0000_0000_0000) >> 24) as u32
}

#[inline]
pub fn bits_27_0(n: u32) -> u32 {
    (n & 0b0000_1111_1111_1111_1111_1111_1111) as u32
}

#[inline]
pub fn bits_0_11(n: u16) -> u16 {
    n & 0b11_1111_1111
}

#[inline]
pub fn bits_3_6(n: u16) -> u8 {
    ((n & 0b_111_000) >> 3) as u8
}

#[inline]
pub fn bits_3_7(n: u16) -> u8 {
    ((n & 0b111_1000) >> 3) as u8
}

#[inline]
pub fn bits_6_9(n: u16) -> u8 {
    ((n & 0b111_000_000) >> 6) as u8
}

#[inline]
pub fn bits_8_11(n: u16) -> u8 {
    ((n & 0b111_0000_0000) >> 8) as u8
}

#[inline]
pub fn bits32_8_11(n: u32) -> u8 {
    ((n & 0b111_0000_0000) >> 8) as u8
}

#[inline]
pub fn bits_8_12(n: u16) -> u8 {
    ((n & 0b1111_0000_0000) >> 8) as u8
}

#[inline]
pub fn bits_6_11(n: u16) -> u8 {
    ((n & 0b111_1100_0000) >> 6) as u8
}

pub trait Bits<O> {
    fn get_bits(&self, low: u8, high: u8) -> O;
}

impl Bits<u32> for u32 {
    #[inline]
    fn get_bits(&self, low: u8, high: u8) -> u32 {
        assert!(high >= low);
        let mask: u32 = ((1_u64 << (high + 1)) - 1) as u32;
        ((*self & mask) >> low) as u32
    }
}

impl Bits<u8> for u32 {
    #[inline]
    fn get_bits(&self, low: u8, high: u8) -> u8 {
        assert!(high >= low);
        let mask: u32 = ((1_u64 << (high + 1)) - 1) as u32;
        ((*self & mask) >> low) as u8
    }
}

impl Bits<u16> for u16 {
    #[inline]
    fn get_bits(&self, low: u8, high: u8) -> u16 {
        assert!(high >= low);
        let mask: u16 = ((1_u32 << (high + 1)) - 1) as u16;
        ((*self & mask) >> low) as u16
    }
}

impl Bits<u16> for u32 {
    #[inline]
    fn get_bits(&self, low: u8, high: u8) -> u16 {
        assert!(high >= low);
        let mask: u32 = ((1_u64 << (high + 1)) - 1) as u32;
        ((*self & mask) >> low) as u16
    }
}

impl Bits<u8> for u8 {
    #[inline]
    fn get_bits(&self, low: u8, high: u8) -> u8 {
        assert!(high >= low);
        let mask: u8 = ((1_u16 << (high + 1)) - 1) as u8;
        ((*self & mask) >> low) as u8
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_bits_u32_to_u32() {
        {
            // arrange
            let input: u32 = 0b0000_0000_0000_0000_0000_0000_1111_1111_u32;

            // act
            let o1: u32 = input.get_bits(0, 7);
            let o2: u32 = input.get_bits(0, 3);
            let o3: u32 = input.get_bits(0, 0);

            // assert
            assert_eq!(o1, 0b1111_1111_u32);
            assert_eq!(o2, 0b1111_u32);
            assert_eq!(o3, 0b1_u32);
        }
        {
            // arrange
            let input: u32 = 0b0000_0000_0000_0000_1100_0000_0000_0000_u32;

            // act
            let o1: u32 = input.get_bits(14, 15);

            // assert
            assert_eq!(o1, 0b11_u32);
        }
        {
            // arrange
            let input: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111_u32;
            // act
            let o1: u32 = input.get_bits(0, 31);

            // assert
            assert_eq!(o1, 0b1111_1111_1111_1111_1111_1111_1111_1111_u32);
        }
    }

}
