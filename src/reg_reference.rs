use core::num::Wrapping;


// The next two lines will be replaced with the appropriate base type and size
type BaseType = u32; // [REF_REPLACE]
const NUM_BITS: usize = BaseType::BITS as usize;

#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(transparent)]
pub struct PlaceholderStructName<const N: usize>(BaseType);

impl<const N: usize> core::ops::Deref for PlaceholderStructName<N> {
    type Target = BaseType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Eq for PlaceholderStructName<N> {}
impl<const N: usize> Ord for PlaceholderStructName<N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PlaceholderStructName<NUM_BITS> {
    #[inline(always)]
    pub fn new(value: BaseType) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<PlaceholderStructName<N>> for BaseType {
    #[inline(always)]
    fn from(item: PlaceholderStructName<N>) -> Self {
        item.0
    }
}

const fn truncate(value: BaseType, to_num_bits: usize) -> BaseType {
    // Needed to circumvent overflow protection
    if to_num_bits == 0 {
        return 0;
    }

    // Needed to circumvent overflow protection
    if to_num_bits >= NUM_BITS {
        return value;
    }

    let mask = BaseType::MAX >> (NUM_BITS - to_num_bits);
    value & mask
}

const fn top_bit_mask(num_bits: usize) -> BaseType {
    if num_bits == 0 {
        return 0;
    }

    if num_bits == 1 {
        return 1;
    }

    if num_bits > NUM_BITS {
        return 0;
    }

    1 << (num_bits - 1)
}

// Function to ease matching of Bits to a specific sequence of bits
impl<const N: usize> PlaceholderStructName<N> {
    /// N 0's in the base type
    const BASE_ZEROS: BaseType = 0;
    /// N 1's in the base type
    const BASE_ONES: BaseType = truncate(!0, N);

    /// A guarenteed N sequential 0's
    pub const ZEROS: Self = Self(Self::BASE_ZEROS);
    /// N sequential 1's
    pub const ONES: Self = Self(Self::BASE_ONES);

    /// Turn the register bits into a array of 1s and 0s
    ///
    /// Can be used in both a const environment and at runtime. This function is especially useful
    /// when pattern matching.
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let value: PlaceholderStructName<4> =
    ///     PlaceholderStructName::new(0b1010).take_low();
    ///
    /// match value.bits() {
    ///     [1, 0, 1, 0] => {
    ///         // Wow, we matched the individual bits
    ///     }
    ///     _ => unreachable!(),
    /// }
    /// ```
    pub const fn bits(&self) -> [u8; N] {
        let mut bits = [0; N];
        let Self(mut value) = self;

        let mut i = 0;
        loop {
            bits[N - i - 1] = (value & 0b1) as u8;
            value >>= 1;

            i += 1;
            if i == N {
                break;
            }
        }

        bits
    }

    /// Fetch a bit at runtime
    /// 
    /// This will fail if index >= N, where N is the size of the [PlaceholderStructName].
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// let bits: PlaceholderStructName<4> =
    ///     PlaceholderStructName::new(0b1100).take_low();
    ///
    /// assert_eq!(bits.get(0).unwrap(), 0u8);
    /// assert_eq!(bits.get(1).unwrap(), 0u8);
    /// assert_eq!(bits.get(2).unwrap(), 1u8);
    /// assert_eq!(bits.get(3).unwrap(), 1u8);
    /// assert_eq!(bits.get(4), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<PlaceholderStructName<1>> {
        if index >= N {
            return None;
        }

        let last_bit = if index == 0 {
            self.0 & 0b1
        } else {
            (self.0 >> index) & 0b1
        };

        Some(PlaceholderStructName(last_bit))
    }
}

impl From<PlaceholderStructName<1>> for bool {
    #[inline(always)]
    fn from(item: PlaceholderStructName<1>) -> bool {
        matches!(item, PlaceholderStructName::<1>(1))
    }
}

impl PartialEq<bool> for PlaceholderStructName<1> {
    #[inline(always)]
    fn eq(&self, other: &bool) -> bool {
        bool::from(*self) == *other
    }
}

impl PartialEq<u8> for PlaceholderStructName<1> {
    #[inline(always)]
    fn eq(&self, other: &u8) -> bool {
        bool::from(*self) == (*other > 0)
    }
}

impl<const N: usize> core::ops::Add for PlaceholderStructName<N> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.0);

        Self(((lhs + rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize> core::ops::Sub for PlaceholderStructName<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.0);

        Self(((lhs - rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize> core::ops::Div for PlaceholderStructName<N> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs / rhs)
    }
}

impl<const N: usize> core::ops::Rem for PlaceholderStructName<N> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs % rhs)
    }
}

impl<const N: usize> core::ops::BitAnd for PlaceholderStructName<N> {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs & rhs)
    }
}

impl<const N: usize> core::ops::BitOr for PlaceholderStructName<N> {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs | rhs)
    }
}

impl<const N: usize> core::ops::BitXor for PlaceholderStructName<N> {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs ^ rhs)
    }
}

impl<const N: usize> core::ops::Not for PlaceholderStructName<N> {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        let lhs = self.0;

        Self((!lhs) & Self::BASE_ONES)
    }
}

impl<const N: usize, T> core::ops::Shl<T> for PlaceholderStructName<N>
where
    BaseType: core::ops::Shl<T, Output = BaseType>,
{
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: T) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs;

        Self((lhs << rhs) & Self::ONES.0)
    }
}

impl<const N: usize, T> core::ops::Shr<T> for PlaceholderStructName<N>
where
    BaseType: core::ops::Shr<T, Output = BaseType>,
{
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: T) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs;

        Self((lhs >> rhs) & Self::ONES.0)
    }
}

pub trait PlaceholderStructNameBitSize {
    const BIT_SIZE: usize;
    const BASE_ONES: BaseType;
}

impl<const N: usize> PlaceholderStructNameBitSize for PlaceholderStructName<N> {
    const BIT_SIZE: usize = N;
    const BASE_ONES: BaseType = PlaceholderStructName::<N>::BASE_ONES;
}

// F > T
pub trait PlaceholderStructNameDownCast<const T: usize>: PlaceholderStructNameBitSize + Copy + Into<BaseType> {
    /// Take a number of the least significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bottom_byte: PlaceholderStructName<8> = PlaceholderStructName::new(0x42).take_low();
    /// let bits: PlaceholderStructName<4> = bottom_byte.take_low();
    ///
    /// assert_eq!(bits, 0x2);
    /// ```
    #[inline(always)]
    fn take_low(self) -> PlaceholderStructName<T> {
        let value: BaseType = self.into();
        PlaceholderStructName(PlaceholderStructName::<T>::BASE_ONES & value)
    }

    /// Take a number of the most significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bottom_byte: PlaceholderStructName<8> =
    ///     PlaceholderStructName::new(0x42).take_low();
    /// let bits: PlaceholderStructName<4> = bottom_byte.take_high();
    ///
    /// assert_eq!(bits, 0x4);
    /// ```
    #[inline(always)]
    fn take_high(self) -> PlaceholderStructName<T> {
        let value: BaseType = self.into();
        PlaceholderStructName(value >> (Self::BIT_SIZE - T))
    }
}

pub trait PlaceholderStructNameUpCast<const T: usize>: PlaceholderStructNameBitSize + Copy + Into<BaseType> {
    /// Extend the current register bits with 0s on the most significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bits: PlaceholderStructName<4> =
    ///     PlaceholderStructName::new(0b1010).take_low();
    ///
    /// // 1010 => 0000 1010
    /// assert_eq!(bits.zero_extend().bits(), [0, 0, 0, 0, 1, 0, 1, 0]);
    /// ```
    #[inline(always)]
    fn zero_extend(self) -> PlaceholderStructName<T> {
        let value = self.into();
        PlaceholderStructName(value)
    }

    /// Extend the current register bits by copied the most significant bit to the new bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// // Most significant bit is 1
    /// let bits: PlaceholderStructName<4> =
    ///     PlaceholderStructName::new(0b1010).take_low();
    ///
    /// // 1010 => 1111 1010
    /// assert_eq!(bits.sign_extend().bits(), [1, 1, 1, 1, 1, 0, 1, 0]);
    ///
    /// // Most significant bit is 0
    /// let bits: PlaceholderStructName<4> =
    ///     PlaceholderStructName::new(0b0101).take_low();
    ///
    /// // 0101 => 0000 0101
    /// assert_eq!(bits.sign_extend().bits(), [0, 0, 0, 0, 0, 1, 0, 1]);
    /// ```
    #[inline(always)]
    fn sign_extend(self) -> PlaceholderStructName<T> {
        // NOTE: We are assuming here that no Bits<0> structure can exist
        let value = self.into();

        let top_bit = value & top_bit_mask(Self::BIT_SIZE); // Capture only the top bit
        let top_bits = if top_bit == 0 {
            // Create a set of NUM_BITS-N bits of with the given sign
            0
        } else {
            (!Self::BASE_ONES) & PlaceholderStructName::<T>::BASE_ONES // !001111 = 110000
        };

        PlaceholderStructName(top_bits | value)
    }

    /// Add extra zeros for the least significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// let bits: PlaceholderStructName<4> =
    ///     PlaceholderStructName::new(0b1010).take_low();
    ///
    /// // 1010 => 1010 0000
    /// assert_eq!(bits.zero_pad().bits(), [1, 0, 1, 0, 0, 0, 0, 0]);
    /// ```
    #[inline(always)]
    fn zero_pad(self) -> PlaceholderStructName<T> {
        self.zero_extend() << (T - Self::BIT_SIZE)
    }
}

pub trait PlaceholderStructNameConcat<const R: usize, const O: usize>:
    Copy + Into<BaseType>
{
    /// Concatinate two register bit structures forming a new struct sized the sum of their
    /// concated structs
    fn concat(self, rhs: PlaceholderStructName<R>) -> PlaceholderStructName<O> {
        let lhs = self.into();
        let rhs: BaseType = rhs.into();

        PlaceholderStructName((lhs << R) | rhs)
    }
}

impl<const F: usize, const T: usize> PlaceholderStructNameUpCast<T> for PlaceholderStructName<F> where
    PlaceholderStructName<T>: PlaceholderStructNameDownCast<F>
{
}

// [REF_STOP]
#[cfg(test)]
mod tests {
    use super::PlaceholderStructName as Bits;
    use super::*;

    #[test]
    fn top_bit() {
        assert_eq!(top_bit_mask(5), 0b10000);
        assert_eq!(top_bit_mask(1), 0b1);
        assert_eq!(top_bit_mask(3), 0b100);
    }

    #[test]
    fn base_ones() {
        assert_eq!(PlaceholderStructName::<5>::BASE_ONES, 0b11111);
        assert_eq!(PlaceholderStructName::<2>::BASE_ONES, 0b11);
        assert_eq!(PlaceholderStructName::<4>::BASE_ONES, 0b1111);
        assert_eq!(PlaceholderStructName::<1>::BASE_ONES, 0b1);
    }

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
        assert_eq!(
            Bits::new(0x0000_0000) - Bits::new(0x0000_0001),
            Bits(0xFFFF_FFFF)
        );
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
        assert_eq!(
            Bits::<NUM_BITS>::ONES % Bits::new(15),
            Bits(Bits::<NUM_BITS>::BASE_ONES % 15)
        );
        assert_eq!(Bits::<4>(5) % Bits::<4>(3), Bits::<4>(5 % 3));
    }

    #[test]
    fn bits_not() {
        assert_eq!(!Bits::<NUM_BITS>::ZEROS, Bits::ONES);
        assert_eq!(!Bits::<4>(0b1010), Bits::<4>(0b0101));
    }

    #[test]
    fn bits_shl() {
        assert_eq!(Bits::<NUM_BITS>::ONES << 28, Bits(0xF000_0000));
        assert_eq!(Bits::<4>(0b1010) << 1, Bits::<4>(0b0100));
    }
    #[test]
    fn bits_shr() {
        assert_eq!(Bits::<NUM_BITS>::ONES >> 28, Bits(0x0000_000F));
        assert_eq!(Bits::<4>(0b1010) >> 1, Bits::<4>(0b0101));
    }
}
