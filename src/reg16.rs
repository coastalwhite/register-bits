// This file was automatically generated with the `generate_impl_rs.py` script.
// Any bugs in this script should be addressed in the `reg_reference.rs` file.
// 
// The contents of this file is as follows:
// 1. Definition of RegXBits struct
// 2. Implementation of operations
// 3. Implementation of traits
use core::num::Wrapping;

// The next two lines will be replaced with the appropriate base type and size
type BaseType = u16; // [REF_REPLACE]
const NUM_BITS: usize = BaseType::BITS as usize;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(transparent)]
pub struct Reg16Bits<const N: usize>(BaseType);

impl<const N: usize> AsRef<BaseType> for Reg16Bits<N> {
    fn as_ref(&self) -> &BaseType {
        let Self(inner) = self;
        inner
    }
}

impl<const N: usize> core::ops::Deref for Reg16Bits<N> {
    type Target = BaseType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Eq for Reg16Bits<N> {}
impl<const N: usize> Ord for Reg16Bits<N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Reg16Bits<NUM_BITS> {
    #[inline(always)]
    pub fn new(value: BaseType) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<Reg16Bits<N>> for BaseType {
    #[inline(always)]
    fn from(item: Reg16Bits<N>) -> Self {
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
impl<const N: usize> Reg16Bits<N> {
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
    /// let value: Reg16Bits<4> =
    ///     Reg16Bits::new(0b1010).take_low();
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
    /// This will fail if index >= N, where N is the size of the [Reg16Bits].
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// let bits: Reg16Bits<4> =
    ///     Reg16Bits::new(0b1100).take_low();
    ///
    /// assert_eq!(bits.get(0).unwrap(), 0u8);
    /// assert_eq!(bits.get(1).unwrap(), 0u8);
    /// assert_eq!(bits.get(2).unwrap(), 1u8);
    /// assert_eq!(bits.get(3).unwrap(), 1u8);
    /// assert_eq!(bits.get(4), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<Reg16Bits<1>> {
        if index >= N {
            return None;
        }

        let last_bit = if index == 0 {
            self.0 & 0b1
        } else {
            (self.0 >> index) & 0b1
        };

        Some(Reg16Bits(last_bit))
    }
}

impl From<Reg16Bits<1>> for bool {
    #[inline(always)]
    fn from(item: Reg16Bits<1>) -> bool {
        matches!(item, Reg16Bits::<1>(1))
    }
}

impl PartialEq<bool> for Reg16Bits<1> {
    #[inline(always)]
    fn eq(&self, other: &bool) -> bool {
        bool::from(*self) == *other
    }
}

impl PartialEq<u8> for Reg16Bits<1> {
    #[inline(always)]
    fn eq(&self, other: &u8) -> bool {
        bool::from(*self) == (*other > 0)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Add<T> for Reg16Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: T) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.into());

        Self(((lhs + rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Sub<T> for Reg16Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: T) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.into());

        Self(((lhs - rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Div<T> for Reg16Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: T) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.into();

        Self(lhs / rhs)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Rem<T> for Reg16Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: T) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.into();

        Self(lhs % rhs)
    }
}

impl<const N: usize> core::ops::BitAnd for Reg16Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs & rhs)
    }
}

impl<const N: usize> core::ops::BitOr for Reg16Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs | rhs)
    }
}

impl<const N: usize> core::ops::BitXor for Reg16Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs ^ rhs)
    }
}

impl<const N: usize> core::ops::Not for Reg16Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        let lhs = self.0;

        Self((!lhs) & Self::BASE_ONES)
    }
}

impl<const N: usize, T> core::ops::Shl<T> for Reg16Bits<N>
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

impl<const N: usize, T> core::ops::Shr<T> for Reg16Bits<N>
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

pub trait Reg16BitsBitSize {
    const BIT_SIZE: usize;
    const BASE_ONES: BaseType;
}

impl<const N: usize> Reg16BitsBitSize for Reg16Bits<N> {
    const BIT_SIZE: usize = N;
    const BASE_ONES: BaseType = Reg16Bits::<N>::BASE_ONES;
}

// F > T
pub trait Reg16BitsDownCast<const T: usize>:
    Reg16BitsBitSize + Copy + Into<BaseType>
{
    /// Take a number of the least significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bottom_byte: Reg16Bits<8> = Reg16Bits::new(0x42).take_low();
    /// let bits: Reg16Bits<4> = bottom_byte.take_low();
    ///
    /// assert_eq!(bits, 0x2);
    /// ```
    #[inline(always)]
    fn take_low(self) -> Reg16Bits<T> {
        let value: BaseType = self.into();
        Reg16Bits(Reg16Bits::<T>::BASE_ONES & value)
    }

    /// Take a number of the most significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bottom_byte: Reg16Bits<8> =
    ///     Reg16Bits::new(0x42).take_low();
    /// let bits: Reg16Bits<4> = bottom_byte.take_high();
    ///
    /// assert_eq!(bits, 0x4);
    /// ```
    #[inline(always)]
    fn take_high(self) -> Reg16Bits<T> {
        let value: BaseType = self.into();
        Reg16Bits(value >> (Self::BIT_SIZE - T))
    }
}

pub trait Reg16BitsUpCast<const T: usize>:
    Reg16BitsBitSize + Copy + Into<BaseType>
{
    /// Extend the current register bits with 0s on the most significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bits: Reg16Bits<4> =
    ///     Reg16Bits::new(0b1010).take_low();
    ///
    /// // 1010 => 0000 1010
    /// assert_eq!(bits.zero_extend().bits(), [0, 0, 0, 0, 1, 0, 1, 0]);
    /// ```
    #[inline(always)]
    fn zero_extend(self) -> Reg16Bits<T> {
        let value = self.into();
        Reg16Bits(value)
    }

    /// Extend the current register bits by copied the most significant bit to the new bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// // Most significant bit is 1
    /// let bits: Reg16Bits<4> =
    ///     Reg16Bits::new(0b1010).take_low();
    ///
    /// // 1010 => 1111 1010
    /// assert_eq!(bits.sign_extend().bits(), [1, 1, 1, 1, 1, 0, 1, 0]);
    ///
    /// // Most significant bit is 0
    /// let bits: Reg16Bits<4> =
    ///     Reg16Bits::new(0b0101).take_low();
    ///
    /// // 0101 => 0000 0101
    /// assert_eq!(bits.sign_extend().bits(), [0, 0, 0, 0, 0, 1, 0, 1]);
    /// ```
    #[inline(always)]
    fn sign_extend(self) -> Reg16Bits<T> {
        // NOTE: We are assuming here that no Bits<0> structure can exist
        let value = self.into();

        let top_bit = value & top_bit_mask(Self::BIT_SIZE); // Capture only the top bit
        let top_bits = if top_bit == 0 {
            // Create a set of NUM_BITS-N bits of with the given sign
            0
        } else {
            (!Self::BASE_ONES) & Reg16Bits::<T>::BASE_ONES // !001111 = 110000
        };

        Reg16Bits(top_bits | value)
    }

    /// Add extra zeros for the least significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// let bits: Reg16Bits<4> =
    ///     Reg16Bits::new(0b1010).take_low();
    ///
    /// // 1010 => 1010 0000
    /// assert_eq!(bits.zero_pad().bits(), [1, 0, 1, 0, 0, 0, 0, 0]);
    /// ```
    #[inline(always)]
    fn zero_pad(self) -> Reg16Bits<T> {
        self.zero_extend() << (T - Self::BIT_SIZE)
    }
}

pub trait Reg16BitsConcat<const R: usize, const O: usize>:
    Copy + Into<BaseType>
{
    /// Concatinate two register bit structures forming a new struct sized the sum of their
    /// concated structs
    fn concat(self, rhs: Reg16Bits<R>) -> Reg16Bits<O> {
        let lhs = self.into();
        let rhs: BaseType = rhs.into();

        Reg16Bits((lhs << R) | rhs)
    }
}

impl<const F: usize, const T: usize> Reg16BitsUpCast<T> for Reg16Bits<F> where
    Reg16Bits<T>: Reg16BitsDownCast<F>
{
}



#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 2> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<2, 3> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<3, 4> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<4, 5> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<5, 6> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<6, 7> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<7, 8> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<8, 9> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<9, 10> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<10, 11> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<11, 12> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<12, 13> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<13, 14> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<14, 15> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsConcat<15, 16> for Reg16Bits<1> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 3> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsConcat<2, 4> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsConcat<3, 5> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsConcat<4, 6> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsConcat<5, 7> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsConcat<6, 8> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsConcat<7, 9> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsConcat<8, 10> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsConcat<9, 11> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsConcat<10, 12> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsConcat<11, 13> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsConcat<12, 14> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsConcat<13, 15> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsConcat<14, 16> for Reg16Bits<2> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 4> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsConcat<2, 5> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsDownCast<3> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsConcat<3, 6> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsConcat<4, 7> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsConcat<5, 8> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsConcat<6, 9> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsConcat<7, 10> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsConcat<8, 11> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsConcat<9, 12> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsConcat<10, 13> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsConcat<11, 14> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsConcat<12, 15> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsConcat<13, 16> for Reg16Bits<3> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 5> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsConcat<2, 6> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsDownCast<3> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsConcat<3, 7> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsDownCast<4> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsConcat<4, 8> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsConcat<5, 9> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsConcat<6, 10> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsConcat<7, 11> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsConcat<8, 12> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsConcat<9, 13> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsConcat<10, 14> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsConcat<11, 15> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsConcat<12, 16> for Reg16Bits<4> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 6> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsConcat<2, 7> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsDownCast<3> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsConcat<3, 8> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsDownCast<4> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsConcat<4, 9> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsDownCast<5> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsConcat<5, 10> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsConcat<6, 11> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsConcat<7, 12> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsConcat<8, 13> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsConcat<9, 14> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsConcat<10, 15> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsConcat<11, 16> for Reg16Bits<5> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 7> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsConcat<2, 8> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsDownCast<3> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsConcat<3, 9> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsDownCast<4> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsConcat<4, 10> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsDownCast<5> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsConcat<5, 11> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsDownCast<6> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsConcat<6, 12> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsConcat<7, 13> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsConcat<8, 14> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsConcat<9, 15> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsConcat<10, 16> for Reg16Bits<6> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 8> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsConcat<2, 9> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsDownCast<3> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsConcat<3, 10> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsDownCast<4> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsConcat<4, 11> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsDownCast<5> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsConcat<5, 12> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsDownCast<6> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsConcat<6, 13> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsDownCast<7> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsConcat<7, 14> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsConcat<8, 15> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsConcat<9, 16> for Reg16Bits<7> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 9> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsConcat<2, 10> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsDownCast<3> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsConcat<3, 11> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsDownCast<4> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsConcat<4, 12> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsDownCast<5> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsConcat<5, 13> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsDownCast<6> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsConcat<6, 14> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsDownCast<7> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsConcat<7, 15> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsDownCast<8> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsConcat<8, 16> for Reg16Bits<8> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 10> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsConcat<2, 11> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsDownCast<3> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsConcat<3, 12> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsDownCast<4> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsConcat<4, 13> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsDownCast<5> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsConcat<5, 14> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsDownCast<6> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsConcat<6, 15> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsDownCast<7> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsConcat<7, 16> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsDownCast<8> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsDownCast<9> for Reg16Bits<9> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 11> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsConcat<2, 12> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsDownCast<3> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsConcat<3, 13> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsDownCast<4> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsConcat<4, 14> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsDownCast<5> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsConcat<5, 15> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsDownCast<6> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsConcat<6, 16> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsDownCast<7> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsDownCast<8> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsDownCast<9> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsDownCast<10> for Reg16Bits<10> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 12> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsConcat<2, 13> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsDownCast<3> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsConcat<3, 14> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsDownCast<4> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsConcat<4, 15> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsDownCast<5> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsConcat<5, 16> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsDownCast<6> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsDownCast<7> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsDownCast<8> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsDownCast<9> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsDownCast<10> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsDownCast<11> for Reg16Bits<11> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 13> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsConcat<2, 14> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsDownCast<3> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsConcat<3, 15> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsDownCast<4> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsConcat<4, 16> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsDownCast<5> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsDownCast<6> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsDownCast<7> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsDownCast<8> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsDownCast<9> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsDownCast<10> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsDownCast<11> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsDownCast<12> for Reg16Bits<12> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 14> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsConcat<2, 15> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsDownCast<3> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsConcat<3, 16> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsDownCast<4> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsDownCast<5> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsDownCast<6> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsDownCast<7> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsDownCast<8> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsDownCast<9> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsDownCast<10> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsDownCast<11> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsDownCast<12> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsDownCast<13> for Reg16Bits<13> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 15> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsConcat<2, 16> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsDownCast<3> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsDownCast<4> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsDownCast<5> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsDownCast<6> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsDownCast<7> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsDownCast<8> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsDownCast<9> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsDownCast<10> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsDownCast<11> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsDownCast<12> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsDownCast<13> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsDownCast<14> for Reg16Bits<14> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsConcat<1, 16> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<3> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<4> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<5> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<6> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<7> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<8> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<9> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<10> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<11> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<12> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<13> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<14> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<15> for Reg16Bits<15> {}
#[doc(hidden)]
impl Reg16BitsDownCast<1> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<2> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<3> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<4> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<5> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<6> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<7> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<8> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<9> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<10> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<11> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<12> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<13> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<14> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<15> for Reg16Bits<16> {}
#[doc(hidden)]
impl Reg16BitsDownCast<16> for Reg16Bits<16> {}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<2> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<3> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<4> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<5> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<6> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<7> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<8> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<9> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<10> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<11> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<12> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<13> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<14> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<15> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg16Bits<16> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<1>> for crate::reg16::Reg16Bits<1> {
    fn from(item: crate::reg8::Reg8Bits<1>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<1>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<1>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<1>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<1>> for crate::reg16::Reg16Bits<1> {
    fn from(item: crate::reg32::Reg32Bits<1>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<1>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<1>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<1>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<1>> for crate::reg16::Reg16Bits<1> {
    fn from(item: crate::reg64::Reg64Bits<1>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<1>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<1>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<1>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<2>> for crate::reg16::Reg16Bits<2> {
    fn from(item: crate::reg8::Reg8Bits<2>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<2>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<2>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<2>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<2>> for crate::reg16::Reg16Bits<2> {
    fn from(item: crate::reg32::Reg32Bits<2>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<2>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<2>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<2>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<2>> for crate::reg16::Reg16Bits<2> {
    fn from(item: crate::reg64::Reg64Bits<2>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<2>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<2>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<2>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<3>> for crate::reg16::Reg16Bits<3> {
    fn from(item: crate::reg8::Reg8Bits<3>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<3>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<3>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<3>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<3>> for crate::reg16::Reg16Bits<3> {
    fn from(item: crate::reg32::Reg32Bits<3>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<3>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<3>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<3>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<3>> for crate::reg16::Reg16Bits<3> {
    fn from(item: crate::reg64::Reg64Bits<3>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<3>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<3>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<3>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<4>> for crate::reg16::Reg16Bits<4> {
    fn from(item: crate::reg8::Reg8Bits<4>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<4>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<4>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<4>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<4>> for crate::reg16::Reg16Bits<4> {
    fn from(item: crate::reg32::Reg32Bits<4>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<4>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<4>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<4>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<4>> for crate::reg16::Reg16Bits<4> {
    fn from(item: crate::reg64::Reg64Bits<4>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<4>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<4>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<4>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<5>> for crate::reg16::Reg16Bits<5> {
    fn from(item: crate::reg8::Reg8Bits<5>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<5>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<5>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<5>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<5>> for crate::reg16::Reg16Bits<5> {
    fn from(item: crate::reg32::Reg32Bits<5>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<5>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<5>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<5>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<5>> for crate::reg16::Reg16Bits<5> {
    fn from(item: crate::reg64::Reg64Bits<5>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<5>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<5>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<5>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<6>> for crate::reg16::Reg16Bits<6> {
    fn from(item: crate::reg8::Reg8Bits<6>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<6>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<6>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<6>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<6>> for crate::reg16::Reg16Bits<6> {
    fn from(item: crate::reg32::Reg32Bits<6>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<6>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<6>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<6>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<6>> for crate::reg16::Reg16Bits<6> {
    fn from(item: crate::reg64::Reg64Bits<6>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<6>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<6>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<6>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<7>> for crate::reg16::Reg16Bits<7> {
    fn from(item: crate::reg8::Reg8Bits<7>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<7>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<7>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<7>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<7>> for crate::reg16::Reg16Bits<7> {
    fn from(item: crate::reg32::Reg32Bits<7>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<7>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<7>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<7>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<7>> for crate::reg16::Reg16Bits<7> {
    fn from(item: crate::reg64::Reg64Bits<7>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<7>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<7>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<7>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<8>> for crate::reg16::Reg16Bits<8> {
    fn from(item: crate::reg8::Reg8Bits<8>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<8>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<8>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<8>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<8>> for crate::reg16::Reg16Bits<8> {
    fn from(item: crate::reg32::Reg32Bits<8>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<8>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<8>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<8>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<8>> for crate::reg16::Reg16Bits<8> {
    fn from(item: crate::reg64::Reg64Bits<8>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<8>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<8>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<8>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<9>> for crate::reg16::Reg16Bits<9> {
    fn from(item: crate::reg8::Reg8Bits<9>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<9>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<9>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<9>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<9>> for crate::reg16::Reg16Bits<9> {
    fn from(item: crate::reg32::Reg32Bits<9>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<9>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<9>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<9>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<9>> for crate::reg16::Reg16Bits<9> {
    fn from(item: crate::reg64::Reg64Bits<9>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<9>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<9>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<9>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<10>> for crate::reg16::Reg16Bits<10> {
    fn from(item: crate::reg8::Reg8Bits<10>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<10>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<10>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<10>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<10>> for crate::reg16::Reg16Bits<10> {
    fn from(item: crate::reg32::Reg32Bits<10>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<10>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<10>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<10>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<10>> for crate::reg16::Reg16Bits<10> {
    fn from(item: crate::reg64::Reg64Bits<10>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<10>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<10>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<10>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<11>> for crate::reg16::Reg16Bits<11> {
    fn from(item: crate::reg8::Reg8Bits<11>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<11>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<11>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<11>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<11>> for crate::reg16::Reg16Bits<11> {
    fn from(item: crate::reg32::Reg32Bits<11>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<11>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<11>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<11>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<11>> for crate::reg16::Reg16Bits<11> {
    fn from(item: crate::reg64::Reg64Bits<11>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<11>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<11>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<11>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<12>> for crate::reg16::Reg16Bits<12> {
    fn from(item: crate::reg8::Reg8Bits<12>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<12>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<12>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<12>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<12>> for crate::reg16::Reg16Bits<12> {
    fn from(item: crate::reg32::Reg32Bits<12>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<12>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<12>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<12>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<12>> for crate::reg16::Reg16Bits<12> {
    fn from(item: crate::reg64::Reg64Bits<12>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<12>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<12>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<12>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<13>> for crate::reg16::Reg16Bits<13> {
    fn from(item: crate::reg8::Reg8Bits<13>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<13>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<13>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<13>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<13>> for crate::reg16::Reg16Bits<13> {
    fn from(item: crate::reg32::Reg32Bits<13>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<13>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<13>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<13>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<13>> for crate::reg16::Reg16Bits<13> {
    fn from(item: crate::reg64::Reg64Bits<13>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<13>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<13>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<13>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<14>> for crate::reg16::Reg16Bits<14> {
    fn from(item: crate::reg8::Reg8Bits<14>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<14>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<14>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<14>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<14>> for crate::reg16::Reg16Bits<14> {
    fn from(item: crate::reg32::Reg32Bits<14>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<14>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<14>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<14>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<14>> for crate::reg16::Reg16Bits<14> {
    fn from(item: crate::reg64::Reg64Bits<14>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<14>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<14>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<14>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<15>> for crate::reg16::Reg16Bits<15> {
    fn from(item: crate::reg8::Reg8Bits<15>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<15>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<15>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<15>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<15>> for crate::reg16::Reg16Bits<15> {
    fn from(item: crate::reg32::Reg32Bits<15>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<15>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<15>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<15>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<15>> for crate::reg16::Reg16Bits<15> {
    fn from(item: crate::reg64::Reg64Bits<15>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<15>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<15>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<15>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<16>> for crate::reg16::Reg16Bits<16> {
    fn from(item: crate::reg8::Reg8Bits<16>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<16>>::
            take_low(crate::reg16::Reg16Bits::new(u8::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<16>> for u8 {
    fn from(item: crate::reg16::Reg16Bits<16>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<16>> for crate::reg16::Reg16Bits<16> {
    fn from(item: crate::reg32::Reg32Bits<16>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<16>>::
            take_low(crate::reg16::Reg16Bits::new(u32::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<16>> for u32 {
    fn from(item: crate::reg16::Reg16Bits<16>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<16>> for crate::reg16::Reg16Bits<16> {
    fn from(item: crate::reg64::Reg64Bits<16>) -> Self {
        <crate::reg16::Reg16Bits<16> as crate::reg16::Reg16BitsDownCast<16>>::
            take_low(crate::reg16::Reg16Bits::new(u64::from(item) as u16))
    }
}
#[doc(hidden)]
impl From<crate::reg16::Reg16Bits<16>> for u64 {
    fn from(item: crate::reg16::Reg16Bits<16>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
impl From<u8> for crate::reg16::Reg16Bits<8> {
    fn from(item: u8) -> Self {
        Self(item.into())
    }
}

#[cfg(feature = "8bit")]
impl<const N: usize, const T: usize> Reg16BitsDownCast<T> for crate::reg8::Reg8Bits<N>
where
    Reg8Bits<N>: Into<Reg16Bits<N>>,
    Reg16Bits<N>: Reg16BitsDownCast<T>,
{
    fn take_low(self) -> Reg16Bits<T> {
        self.into().take_low()
    }
    fn take_high(self) -> Reg16Bits<T> {
        self.into().take_high()
    }
}

#[cfg(feature = "8bit")]
impl<const N: usize, const T: usize> Reg16BitsUpCast<T> for crate::reg8::Reg8Bits<N>
where
    Reg8Bits<N>: Into<Reg16Bits<N>>,
    Reg16Bits<N>: Reg16BitsUpCast<T>,
{
    fn zero_pad(self) -> Reg16Bits<T> {
        self.into().zero_pad()
    }
    fn zero_extend(self) -> Reg16Bits<T> {
        self.into().zero_extend()
    }
    fn sign_extend(self) -> Reg16Bits<T> {
        self.into().sign_extend()
    }
}

#[cfg(feature = "32bit")]
impl<const N: usize, const T: usize> Reg16BitsDownCast<T> for crate::reg32::Reg32Bits<N>
where
    Reg32Bits<N>: Into<Reg16Bits<N>>,
    Reg16Bits<N>: Reg16BitsDownCast<T>,
{
    fn take_low(self) -> Reg16Bits<T> {
        self.into().take_low()
    }
    fn take_high(self) -> Reg16Bits<T> {
        self.into().take_high()
    }
}

#[cfg(feature = "32bit")]
impl<const N: usize, const T: usize> Reg16BitsUpCast<T> for crate::reg32::Reg32Bits<N>
where
    Reg32Bits<N>: Into<Reg16Bits<N>>,
    Reg16Bits<N>: Reg16BitsUpCast<T>,
{
    fn zero_pad(self) -> Reg16Bits<T> {
        self.into().zero_pad()
    }
    fn zero_extend(self) -> Reg16Bits<T> {
        self.into().zero_extend()
    }
    fn sign_extend(self) -> Reg16Bits<T> {
        self.into().sign_extend()
    }
}

#[cfg(feature = "64bit")]
impl<const N: usize, const T: usize> Reg16BitsDownCast<T> for crate::reg64::Reg64Bits<N>
where
    Reg64Bits<N>: Into<Reg16Bits<N>>,
    Reg16Bits<N>: Reg16BitsDownCast<T>,
{
    fn take_low(self) -> Reg16Bits<T> {
        self.into().take_low()
    }
    fn take_high(self) -> Reg16Bits<T> {
        self.into().take_high()
    }
}

#[cfg(feature = "64bit")]
impl<const N: usize, const T: usize> Reg16BitsUpCast<T> for crate::reg64::Reg64Bits<N>
where
    Reg64Bits<N>: Into<Reg16Bits<N>>,
    Reg16Bits<N>: Reg16BitsUpCast<T>,
{
    fn zero_pad(self) -> Reg16Bits<T> {
        self.into().zero_pad()
    }
    fn zero_extend(self) -> Reg16Bits<T> {
        self.into().zero_extend()
    }
    fn sign_extend(self) -> Reg16Bits<T> {
        self.into().sign_extend()
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<1> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<2> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<3> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<4> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<5> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<6> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<7> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<8> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<9> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<10> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<11> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<12> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<13> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<14> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<15> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg16::Reg16Bits<16> {
    fn default() -> Self {
        Self(0)
    }
}
