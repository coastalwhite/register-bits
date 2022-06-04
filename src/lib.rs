use core::num::Wrapping;

#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(transparent)]
pub struct Bits<const N: usize>(u32);

impl<const N: usize> Eq for Bits<N> {}
impl<const N: usize> Ord for Bits<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Bits<32> {
    #[inline(always)]
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<Bits<N>> for u32 {
    #[inline(always)]
    fn from(item: Bits<N>) -> Self {
        item.0
    }
}

impl<const N: usize> core::ops::Deref for Bits<N> {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

const fn truncate(value: u32, num_bits: usize) -> u32 {
    // Needed to circumvent overflow protection
    if num_bits == 0 {
        return 0;
    }

    // Needed to circumvent overflow protection
    if num_bits >= 32 {
        return value;
    }

    let mask = u32::MAX >> (32 - num_bits);
    value & mask
}

// Function to ease matching of Bits to a specific sequence of bits
impl<const N: usize> Bits<N> {
    pub const ZEROS: Self = Self(0);
    pub const ONES: Self = Self(truncate(!0, N));

    pub fn bits(&self) -> [u8; N] {
        let mut bits = [0; N];
        let Self(mut value) = self;

        for i in 0..N {
            bits[N - i - 1] = (value & 0b1) as u8;
            value >>= 1;
        }

        bits
    }

    pub fn get(&self, index: usize) -> Option<Bits<1>> {
        if index >= N {
            return None;
        }

        let last_bit = if index == 0 {
            self.0 & 0b1
        } else {
            (self.0 >> index) & 0b1
        };

        Some(Bits(last_bit))
    }
}

pub trait BitsSize {
    const BIT_SIZE: usize;
}

impl<const N: usize> BitsSize for Bits<N> {
    const BIT_SIZE: usize = N;
}

impl From<Bits<1>> for bool {
    #[inline(always)]
    fn from(item: Bits<1>) -> bool {
        matches!(item, Bits::<1>(1))
    }
}

impl PartialEq<bool> for Bits<1> {
    #[inline(always)]
    fn eq(&self, other: &bool) -> bool {
        bool::from(*self) == *other
    }
}

impl PartialEq<u8> for Bits<1> {
    #[inline(always)]
    fn eq(&self, other: &u8) -> bool {
        bool::from(*self) == (*other > 0)
    }
}

impl<const N: usize> core::ops::Add for Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let lhs = Wrapping::<u32>(self.0);
        let rhs = Wrapping::<u32>(rhs.0);

        Bits(truncate((lhs + rhs).0, N))
    }
}

impl<const N: usize> core::ops::Sub for Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = Wrapping::<u32>(self.0);
        let rhs = Wrapping::<u32>(rhs.0);

        Bits(truncate((lhs - rhs).0, N))
    }
}

impl<const N: usize> core::ops::Div for Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Bits(lhs / rhs)
    }
}

impl<const N: usize> core::ops::Rem for Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Bits(lhs % rhs)
    }
}

impl<const N: usize> core::ops::BitAnd for Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Bits(lhs & rhs)
    }
}

impl<const N: usize> core::ops::BitOr for Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Bits(lhs | rhs)
    }
}

impl<const N: usize> core::ops::BitXor for Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Bits(lhs ^ rhs)
    }
}

impl<const N: usize> core::ops::Not for Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        let lhs = self.0;

        Bits(truncate(!lhs, N))
    }
}

impl<const N: usize, T> core::ops::Shl<T> for Bits<N>
where
    u32: core::ops::Shl<T, Output = u32>
{
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: T) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs;

        Bits(truncate(lhs << rhs, N))
    }
}

impl<const N: usize, T> core::ops::Shr<T> for Bits<N>
where
    u32: core::ops::Shr<T, Output = u32>
{
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: T) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs;

        Bits(truncate(lhs >> rhs, N))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_consts() {
        assert_eq!(Bits::<4>::ONES, Bits(0b1111));
        assert_eq!(Bits::<6>::ONES, Bits(0b111111));
        assert_eq!(Bits::<4>::ZEROS, Bits(0));
    }

    #[test]
    fn priv_truncate() {
        assert_eq!(truncate(0b1111, 2), 0b11);
        assert_eq!(truncate(0b1111, 0), 0b0);
    }

    #[test]
    fn match_bits() {
        let imm4 = Bits::<4>(0b1010);

        assert_eq!(imm4.bits(), [1, 0, 1, 0]);

        let imm1 = Bits::<1>(0b1);
        assert_eq!(imm1.bits(), [1]);

        let imm1 = Bits::<1>(0b0);
        assert_eq!(imm1.bits(), [0]);

        let imm3 = Bits::<3>(0b001);
        assert_eq!(imm3.bits(), [0, 0, 1]);

        assert!(matches!(imm3.bits(), [0, 0, 1]));
        assert!(matches!(imm3.bits(), [_, _, 1]));
    }

    #[test]
    fn get_bits() {
        let imm4 = Bits::<4>(0b1010);

        assert_eq!(imm4.get(4), None);
        assert_eq!(imm4.get(3).unwrap(), 1);
        assert_eq!(imm4.get(2).unwrap(), 0);
        assert_eq!(imm4.get(1).unwrap(), 1);
        assert_eq!(imm4.get(0).unwrap(), 0);

        let imm1 = Bits::<1>(0b1);
        assert_eq!(imm1.get(1), None);
        assert_eq!(imm1.get(0).unwrap(), 1);

        let imm1 = Bits::<1>(0b0);
        assert_eq!(imm1.get(1), None);
        assert_eq!(imm1.get(0).unwrap(), 0);

        let imm3 = Bits::<3>(0b001);
        assert_eq!(imm3.get(3), None);
        assert_eq!(imm3.get(2).unwrap(), 0);
        assert_eq!(imm3.get(1).unwrap(), 0);
        assert_eq!(imm3.get(0).unwrap(), 1);

    }

    #[test]
    fn bit_eq() {
        let bit_0 = Bits::<1>(0);
        let bit_1 = Bits::<1>(1);

        assert_eq!(bit_0, 0);
        assert_eq!(bit_0, false);
        assert_eq!(bool::from(bit_0), false);
        assert_eq!(bit_1, 1);
        assert_eq!(bit_1, true);
        assert_eq!(bool::from(bit_1), true);
    }

    #[test]
    fn bits_add() {
        assert_eq!(Bits::new(0xFFFF_FFFF) + Bits::new(0x0000_0001), Bits(0));
        assert_eq!(Bits::<4>(2) + Bits::<4>(3), Bits::<4>(5));
        assert_eq!(Bits::<4>(15) + Bits::<4>(1), Bits::<4>(0));
    }

    #[test]
    fn bits_sub() {
        assert_eq!(Bits::new(0x0000_0000) - Bits::new(0x0000_0001), Bits(0xFFFF_FFFF));
        assert_eq!(Bits::<4>(2) - Bits::<4>(3), Bits::<4>(15));
        assert_eq!(Bits::<4>(15) - Bits::<4>(1), Bits::<4>(14));
    }

    #[test]
    fn bits_div() {
        assert_eq!(Bits::new(0xFFFF_FFFF) / Bits::new(15), Bits(0x1111_1111));
        assert_eq!(Bits::<4>(5) / Bits::<4>(3), Bits::<4>(1));
    }

    #[test]
    fn bits_rem() {
        assert_eq!(Bits::new(0xFFFF_FFFF) % Bits::new(15), Bits(0xFFFF_FFFF % 15));
        assert_eq!(Bits::<4>(5) % Bits::<4>(3), Bits::<4>(5 % 3));
    }

    #[test]
    fn bits_not() {
        assert_eq!(!Bits::<32>::ZEROS, Bits::ONES);
        assert_eq!(!Bits::<4>(0b1010), Bits::<4>(0b0101));
    }

    #[test]
    fn bits_shl() {
        assert_eq!(Bits::<32>::ONES << 28, Bits(0xF000_0000));
        assert_eq!(Bits::<4>(0b1010) << 1, Bits::<4>(0b0100));
    }
    #[test]
    fn bits_shr() {
        assert_eq!(Bits::<32>::ONES >> 28, Bits(0x0000_000F));
        assert_eq!(Bits::<4>(0b1010) >> 1, Bits::<4>(0b0101));
    }
}

// F > T
pub trait BitsDownCast<const T: usize>: BitsSize + Copy + Into<u32> {
    #[inline(always)]
    fn take_low(self) -> Bits<T> {
        let value: u32 = self.into();
        Bits(truncate(value, T))
    }
    #[inline(always)]
    fn take_high(self) -> Bits<T> {
        let value: u32 = self.into();
        Bits(value >> (Self::BIT_SIZE - T))
    }
}

pub trait BitsUpCast<const T: usize>: BitsSize + Copy + Into<u32> {
    #[inline(always)]
    fn zero_extend(self) -> Bits<T> {
        let value = self.into();
        Bits(value)
    }

    fn sign_extend(self) -> Bits<T> {
        // NOTE: We are assuming here that no Bits<0> structure can exist
        let top_bit = self.into() >> (Self::BIT_SIZE - 1);
        let top_bits = if top_bit == 1 {
            !(!0 << (T - Self::BIT_SIZE))
        } else {
            0
        };
        let value = self.into();
        Bits((top_bits << Self::BIT_SIZE) + value)
    }
}

pub trait BitsConcat<const R: usize, const O: usize>: Copy + Into<u32> {
    fn concat(self, rhs: Bits<R>) -> Bits<O> {
        let lhs = self.into();
        let rhs: u32 = rhs.into();

        Bits((lhs << R) | rhs)
    }
}

impl<const F: usize, const T: usize> BitsUpCast<T> for Bits<F>
where
    Bits<T>: BitsDownCast<F>,
    Bits<F>: BitsSize,
{
}

mod impls_32;
