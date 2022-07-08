// This file was automatically generated with the `generate_impl_rs.py` script.
// Any bugs in this script should be addressed in the `reg_reference.rs` file.
// 
// The contents of this file is as follows:
// 1. Definition of RegXBits struct
// 2. Implementation of operations
// 3. Implementation of traits
use core::num::Wrapping;

// The next two lines will be replaced with the appropriate base type and size
type BaseType = u32; // [REF_REPLACE]
const NUM_BITS: usize = BaseType::BITS as usize;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(transparent)]
pub struct Reg32Bits<const N: usize>(BaseType);

impl<const N: usize> AsRef<BaseType> for Reg32Bits<N> {
    fn as_ref(&self) -> &BaseType {
        let Self(inner) = self;
        inner
    }
}

impl<const N: usize> core::ops::Deref for Reg32Bits<N> {
    type Target = BaseType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Eq for Reg32Bits<N> {}
impl<const N: usize> Ord for Reg32Bits<N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Reg32Bits<NUM_BITS> {
    #[inline(always)]
    pub fn new(value: BaseType) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<Reg32Bits<N>> for BaseType {
    #[inline(always)]
    fn from(item: Reg32Bits<N>) -> Self {
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
impl<const N: usize> Reg32Bits<N> {
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
    /// let value: Reg32Bits<4> =
    ///     Reg32Bits::new(0b1010).take_low();
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
    /// This will fail if index >= N, where N is the size of the [Reg32Bits].
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// let bits: Reg32Bits<4> =
    ///     Reg32Bits::new(0b1100).take_low();
    ///
    /// assert_eq!(bits.get(0).unwrap(), 0u8);
    /// assert_eq!(bits.get(1).unwrap(), 0u8);
    /// assert_eq!(bits.get(2).unwrap(), 1u8);
    /// assert_eq!(bits.get(3).unwrap(), 1u8);
    /// assert_eq!(bits.get(4), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<Reg32Bits<1>> {
        if index >= N {
            return None;
        }

        let last_bit = if index == 0 {
            self.0 & 0b1
        } else {
            (self.0 >> index) & 0b1
        };

        Some(Reg32Bits(last_bit))
    }
}

impl From<Reg32Bits<1>> for bool {
    #[inline(always)]
    fn from(item: Reg32Bits<1>) -> bool {
        matches!(item, Reg32Bits::<1>(1))
    }
}

impl PartialEq<bool> for Reg32Bits<1> {
    #[inline(always)]
    fn eq(&self, other: &bool) -> bool {
        bool::from(*self) == *other
    }
}

impl PartialEq<u8> for Reg32Bits<1> {
    #[inline(always)]
    fn eq(&self, other: &u8) -> bool {
        bool::from(*self) == (*other > 0)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Add<T> for Reg32Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: T) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.into());

        Self(((lhs + rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Sub<T> for Reg32Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: T) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.into());

        Self(((lhs - rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Div<T> for Reg32Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: T) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.into();

        Self(lhs / rhs)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Rem<T> for Reg32Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: T) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.into();

        Self(lhs % rhs)
    }
}

impl<const N: usize> core::ops::BitAnd for Reg32Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs & rhs)
    }
}

impl<const N: usize> core::ops::BitOr for Reg32Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs | rhs)
    }
}

impl<const N: usize> core::ops::BitXor for Reg32Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs ^ rhs)
    }
}

impl<const N: usize> core::ops::Not for Reg32Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        let lhs = self.0;

        Self((!lhs) & Self::BASE_ONES)
    }
}

impl<const N: usize, T> core::ops::Shl<T> for Reg32Bits<N>
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

impl<const N: usize, T> core::ops::Shr<T> for Reg32Bits<N>
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

pub trait Reg32BitsBitSize {
    const BIT_SIZE: usize;
    const BASE_ONES: BaseType;
}

impl<const N: usize> Reg32BitsBitSize for Reg32Bits<N> {
    const BIT_SIZE: usize = N;
    const BASE_ONES: BaseType = Reg32Bits::<N>::BASE_ONES;
}

// F > T
pub trait Reg32BitsDownCast<const T: usize>:
    Reg32BitsBitSize + Copy + Into<BaseType>
{
    /// Take a number of the least significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bottom_byte: Reg32Bits<8> = Reg32Bits::new(0x42).take_low();
    /// let bits: Reg32Bits<4> = bottom_byte.take_low();
    ///
    /// assert_eq!(bits, 0x2);
    /// ```
    #[inline(always)]
    fn take_low(self) -> Reg32Bits<T> {
        let value: BaseType = self.into();
        Reg32Bits(Reg32Bits::<T>::BASE_ONES & value)
    }

    /// Take a number of the most significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bottom_byte: Reg32Bits<8> =
    ///     Reg32Bits::new(0x42).take_low();
    /// let bits: Reg32Bits<4> = bottom_byte.take_high();
    ///
    /// assert_eq!(bits, 0x4);
    /// ```
    #[inline(always)]
    fn take_high(self) -> Reg32Bits<T> {
        let value: BaseType = self.into();
        Reg32Bits(value >> (Self::BIT_SIZE - T))
    }
}

pub trait Reg32BitsUpCast<const T: usize>:
    Reg32BitsBitSize + Copy + Into<BaseType>
{
    /// Extend the current register bits with 0s on the most significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bits: Reg32Bits<4> =
    ///     Reg32Bits::new(0b1010).take_low();
    ///
    /// // 1010 => 0000 1010
    /// assert_eq!(bits.zero_extend().bits(), [0, 0, 0, 0, 1, 0, 1, 0]);
    /// ```
    #[inline(always)]
    fn zero_extend(self) -> Reg32Bits<T> {
        let value = self.into();
        Reg32Bits(value)
    }

    /// Extend the current register bits by copied the most significant bit to the new bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// // Most significant bit is 1
    /// let bits: Reg32Bits<4> =
    ///     Reg32Bits::new(0b1010).take_low();
    ///
    /// // 1010 => 1111 1010
    /// assert_eq!(bits.sign_extend().bits(), [1, 1, 1, 1, 1, 0, 1, 0]);
    ///
    /// // Most significant bit is 0
    /// let bits: Reg32Bits<4> =
    ///     Reg32Bits::new(0b0101).take_low();
    ///
    /// // 0101 => 0000 0101
    /// assert_eq!(bits.sign_extend().bits(), [0, 0, 0, 0, 0, 1, 0, 1]);
    /// ```
    #[inline(always)]
    fn sign_extend(self) -> Reg32Bits<T> {
        // NOTE: We are assuming here that no Bits<0> structure can exist
        let value = self.into();

        let top_bit = value & top_bit_mask(Self::BIT_SIZE); // Capture only the top bit
        let top_bits = if top_bit == 0 {
            // Create a set of NUM_BITS-N bits of with the given sign
            0
        } else {
            (!Self::BASE_ONES) & Reg32Bits::<T>::BASE_ONES // !001111 = 110000
        };

        Reg32Bits(top_bits | value)
    }

    /// Add extra zeros for the least significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// let bits: Reg32Bits<4> =
    ///     Reg32Bits::new(0b1010).take_low();
    ///
    /// // 1010 => 1010 0000
    /// assert_eq!(bits.zero_pad().bits(), [1, 0, 1, 0, 0, 0, 0, 0]);
    /// ```
    #[inline(always)]
    fn zero_pad(self) -> Reg32Bits<T> {
        self.zero_extend() << (T - Self::BIT_SIZE)
    }
}

pub trait Reg32BitsConcat<const R: usize, const O: usize>:
    Copy + Into<BaseType>
{
    /// Concatinate two register bit structures forming a new struct sized the sum of their
    /// concated structs
    fn concat(self, rhs: Reg32Bits<R>) -> Reg32Bits<O> {
        let lhs = self.into();
        let rhs: BaseType = rhs.into();

        Reg32Bits((lhs << R) | rhs)
    }
}

impl<const F: usize, const T: usize> Reg32BitsUpCast<T> for Reg32Bits<F> where
    Reg32Bits<T>: Reg32BitsDownCast<F>
{
}



#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 2> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 3> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 4> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 5> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 6> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 7> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 8> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 9> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 10> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 11> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 12> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 13> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 14> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 15> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 16> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 17> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 18> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<18, 19> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<19, 20> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<20, 21> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<21, 22> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<22, 23> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<23, 24> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<24, 25> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<25, 26> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<26, 27> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<27, 28> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<28, 29> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<29, 30> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<30, 31> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsConcat<31, 32> for Reg32Bits<1> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 3> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 4> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 5> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 6> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 7> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 8> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 9> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 10> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 11> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 12> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 13> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 14> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 15> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 16> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 17> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 18> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 19> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<18, 20> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<19, 21> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<20, 22> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<21, 23> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<22, 24> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<23, 25> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<24, 26> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<25, 27> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<26, 28> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<27, 29> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<28, 30> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<29, 31> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsConcat<30, 32> for Reg32Bits<2> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 4> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 5> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 6> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 7> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 8> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 9> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 10> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 11> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 12> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 13> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 14> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 15> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 16> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 17> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 18> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 19> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 20> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<18, 21> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<19, 22> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<20, 23> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<21, 24> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<22, 25> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<23, 26> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<24, 27> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<25, 28> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<26, 29> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<27, 30> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<28, 31> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsConcat<29, 32> for Reg32Bits<3> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 5> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 6> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 7> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 8> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 9> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 10> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 11> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 12> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 13> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 14> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 15> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 16> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 17> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 18> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 19> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 20> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 21> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<18, 22> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<19, 23> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<20, 24> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<21, 25> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<22, 26> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<23, 27> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<24, 28> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<25, 29> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<26, 30> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<27, 31> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsConcat<28, 32> for Reg32Bits<4> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 6> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 7> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 8> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 9> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 10> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 11> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 12> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 13> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 14> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 15> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 16> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 17> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 18> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 19> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 20> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 21> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 22> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<18, 23> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<19, 24> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<20, 25> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<21, 26> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<22, 27> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<23, 28> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<24, 29> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<25, 30> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<26, 31> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsConcat<27, 32> for Reg32Bits<5> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 7> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 8> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 9> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 10> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 11> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 12> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 13> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 14> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 15> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 16> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 17> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 18> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 19> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 20> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 21> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 22> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 23> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<18, 24> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<19, 25> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<20, 26> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<21, 27> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<22, 28> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<23, 29> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<24, 30> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<25, 31> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsConcat<26, 32> for Reg32Bits<6> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 8> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 9> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 10> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 11> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 12> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 13> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 14> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 15> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 16> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 17> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 18> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 19> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 20> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 21> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 22> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 23> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 24> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<18, 25> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<19, 26> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<20, 27> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<21, 28> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<22, 29> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<23, 30> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<24, 31> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsConcat<25, 32> for Reg32Bits<7> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 9> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 10> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 11> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 12> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 13> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 14> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 15> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 16> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 17> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 18> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 19> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 20> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 21> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 22> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 23> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 24> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 25> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<18, 26> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<19, 27> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<20, 28> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<21, 29> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<22, 30> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<23, 31> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsConcat<24, 32> for Reg32Bits<8> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 10> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 11> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 12> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 13> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 14> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 15> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 16> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 17> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 18> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 19> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 20> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 21> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 22> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 23> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 24> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 25> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 26> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<18, 27> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<19, 28> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<20, 29> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<21, 30> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<22, 31> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsConcat<23, 32> for Reg32Bits<9> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 11> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 12> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 13> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 14> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 15> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 16> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 17> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 18> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 19> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 20> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 21> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 22> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 23> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 24> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 25> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 26> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 27> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<18, 28> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<19, 29> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<20, 30> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<21, 31> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsConcat<22, 32> for Reg32Bits<10> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 12> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 13> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 14> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 15> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 16> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 17> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 18> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 19> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 20> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 21> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 22> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 23> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 24> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 25> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 26> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 27> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 28> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<18, 29> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<19, 30> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<20, 31> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsConcat<21, 32> for Reg32Bits<11> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 13> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 14> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 15> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 16> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 17> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 18> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 19> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 20> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 21> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 22> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 23> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 24> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 25> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 26> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 27> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 28> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 29> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<18, 30> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<19, 31> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsConcat<20, 32> for Reg32Bits<12> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 14> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 15> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 16> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 17> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 18> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 19> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 20> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 21> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 22> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 23> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 24> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 25> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 26> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 27> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 28> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 29> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 30> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<18, 31> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsConcat<19, 32> for Reg32Bits<13> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 15> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 16> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 17> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 18> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 19> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 20> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 21> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 22> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 23> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 24> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 25> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 26> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 27> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 28> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 29> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 30> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 31> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsConcat<18, 32> for Reg32Bits<14> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 16> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 17> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 18> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 19> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 20> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 21> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 22> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 23> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 24> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 25> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 26> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 27> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 28> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 29> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 30> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 31> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsConcat<17, 32> for Reg32Bits<15> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 17> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 18> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 19> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 20> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 21> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 22> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 23> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 24> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 25> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 26> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 27> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 28> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 29> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 30> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 31> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsConcat<16, 32> for Reg32Bits<16> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 18> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 19> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 20> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 21> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 22> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 23> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 24> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 25> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 26> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 27> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 28> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 29> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 30> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 31> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsConcat<15, 32> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<17> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 19> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 20> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 21> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 22> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 23> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 24> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 25> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 26> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 27> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 28> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 29> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 30> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 31> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsConcat<14, 32> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<18> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 20> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 21> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 22> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 23> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 24> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 25> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 26> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 27> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 28> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 29> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 30> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 31> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsConcat<13, 32> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<19> for Reg32Bits<19> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 21> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 22> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 23> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 24> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 25> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 26> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 27> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 28> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 29> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 30> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 31> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsConcat<12, 32> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<19> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<20> for Reg32Bits<20> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 22> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 23> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 24> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 25> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 26> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 27> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 28> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 29> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 30> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 31> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsConcat<11, 32> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<19> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<20> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<21> for Reg32Bits<21> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 23> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 24> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 25> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 26> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 27> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 28> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 29> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 30> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 31> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsConcat<10, 32> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<19> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<20> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<21> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<22> for Reg32Bits<22> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 24> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 25> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 26> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 27> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 28> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 29> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 30> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 31> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsConcat<9, 32> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<19> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<20> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<21> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<22> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<23> for Reg32Bits<23> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 25> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 26> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 27> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 28> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 29> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 30> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 31> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsConcat<8, 32> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<19> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<20> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<21> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<22> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<23> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<24> for Reg32Bits<24> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 26> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 27> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 28> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 29> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 30> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 31> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsConcat<7, 32> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<19> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<20> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<21> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<22> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<23> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<24> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<25> for Reg32Bits<25> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 27> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 28> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 29> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 30> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 31> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsConcat<6, 32> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<19> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<20> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<21> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<22> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<23> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<24> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<25> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<26> for Reg32Bits<26> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 28> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 29> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 30> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 31> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsConcat<5, 32> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<19> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<20> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<21> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<22> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<23> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<24> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<25> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<26> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<27> for Reg32Bits<27> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 29> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 30> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 31> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsConcat<4, 32> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<19> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<20> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<21> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<22> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<23> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<24> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<25> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<26> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<27> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<28> for Reg32Bits<28> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 30> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 31> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsConcat<3, 32> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<19> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<20> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<21> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<22> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<23> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<24> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<25> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<26> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<27> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<28> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<29> for Reg32Bits<29> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 31> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsConcat<2, 32> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<19> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<20> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<21> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<22> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<23> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<24> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<25> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<26> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<27> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<28> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<29> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<30> for Reg32Bits<30> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsConcat<1, 32> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<19> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<20> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<21> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<22> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<23> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<24> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<25> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<26> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<27> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<28> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<29> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<30> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<31> for Reg32Bits<31> {}
#[doc(hidden)]
impl Reg32BitsDownCast<1> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<2> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<3> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<4> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<5> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<6> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<7> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<8> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<9> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<10> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<11> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<12> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<13> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<14> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<15> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<16> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<17> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<18> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<19> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<20> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<21> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<22> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<23> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<24> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<25> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<26> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<27> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<28> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<29> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<30> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<31> for Reg32Bits<32> {}
#[doc(hidden)]
impl Reg32BitsDownCast<32> for Reg32Bits<32> {}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<2> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<3> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<4> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<5> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<6> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<7> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<8> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<9> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<10> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<11> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<12> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<13> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<14> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<15> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<16> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<17> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<18> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<19> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<20> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<21> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<22> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<23> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<24> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<25> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<26> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<27> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<28> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<29> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<30> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<31> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg32Bits<32> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<1>> for crate::reg32::Reg32Bits<1> {
    fn from(item: crate::reg8::Reg8Bits<1>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<1>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<1>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<1>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<1>> for crate::reg32::Reg32Bits<1> {
    fn from(item: crate::reg16::Reg16Bits<1>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<1>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<1>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<1>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<1>> for crate::reg32::Reg32Bits<1> {
    fn from(item: crate::reg64::Reg64Bits<1>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<1>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<1>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<1>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<2>> for crate::reg32::Reg32Bits<2> {
    fn from(item: crate::reg8::Reg8Bits<2>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<2>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<2>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<2>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<2>> for crate::reg32::Reg32Bits<2> {
    fn from(item: crate::reg16::Reg16Bits<2>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<2>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<2>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<2>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<2>> for crate::reg32::Reg32Bits<2> {
    fn from(item: crate::reg64::Reg64Bits<2>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<2>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<2>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<2>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<3>> for crate::reg32::Reg32Bits<3> {
    fn from(item: crate::reg8::Reg8Bits<3>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<3>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<3>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<3>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<3>> for crate::reg32::Reg32Bits<3> {
    fn from(item: crate::reg16::Reg16Bits<3>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<3>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<3>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<3>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<3>> for crate::reg32::Reg32Bits<3> {
    fn from(item: crate::reg64::Reg64Bits<3>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<3>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<3>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<3>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<4>> for crate::reg32::Reg32Bits<4> {
    fn from(item: crate::reg8::Reg8Bits<4>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<4>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<4>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<4>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<4>> for crate::reg32::Reg32Bits<4> {
    fn from(item: crate::reg16::Reg16Bits<4>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<4>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<4>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<4>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<4>> for crate::reg32::Reg32Bits<4> {
    fn from(item: crate::reg64::Reg64Bits<4>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<4>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<4>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<4>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<5>> for crate::reg32::Reg32Bits<5> {
    fn from(item: crate::reg8::Reg8Bits<5>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<5>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<5>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<5>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<5>> for crate::reg32::Reg32Bits<5> {
    fn from(item: crate::reg16::Reg16Bits<5>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<5>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<5>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<5>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<5>> for crate::reg32::Reg32Bits<5> {
    fn from(item: crate::reg64::Reg64Bits<5>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<5>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<5>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<5>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<6>> for crate::reg32::Reg32Bits<6> {
    fn from(item: crate::reg8::Reg8Bits<6>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<6>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<6>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<6>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<6>> for crate::reg32::Reg32Bits<6> {
    fn from(item: crate::reg16::Reg16Bits<6>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<6>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<6>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<6>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<6>> for crate::reg32::Reg32Bits<6> {
    fn from(item: crate::reg64::Reg64Bits<6>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<6>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<6>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<6>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<7>> for crate::reg32::Reg32Bits<7> {
    fn from(item: crate::reg8::Reg8Bits<7>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<7>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<7>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<7>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<7>> for crate::reg32::Reg32Bits<7> {
    fn from(item: crate::reg16::Reg16Bits<7>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<7>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<7>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<7>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<7>> for crate::reg32::Reg32Bits<7> {
    fn from(item: crate::reg64::Reg64Bits<7>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<7>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<7>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<7>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<8>> for crate::reg32::Reg32Bits<8> {
    fn from(item: crate::reg8::Reg8Bits<8>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<8>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<8>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<8>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<8>> for crate::reg32::Reg32Bits<8> {
    fn from(item: crate::reg16::Reg16Bits<8>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<8>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<8>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<8>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<8>> for crate::reg32::Reg32Bits<8> {
    fn from(item: crate::reg64::Reg64Bits<8>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<8>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<8>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<8>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<9>> for crate::reg32::Reg32Bits<9> {
    fn from(item: crate::reg8::Reg8Bits<9>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<9>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<9>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<9>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<9>> for crate::reg32::Reg32Bits<9> {
    fn from(item: crate::reg16::Reg16Bits<9>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<9>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<9>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<9>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<9>> for crate::reg32::Reg32Bits<9> {
    fn from(item: crate::reg64::Reg64Bits<9>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<9>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<9>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<9>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<10>> for crate::reg32::Reg32Bits<10> {
    fn from(item: crate::reg8::Reg8Bits<10>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<10>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<10>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<10>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<10>> for crate::reg32::Reg32Bits<10> {
    fn from(item: crate::reg16::Reg16Bits<10>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<10>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<10>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<10>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<10>> for crate::reg32::Reg32Bits<10> {
    fn from(item: crate::reg64::Reg64Bits<10>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<10>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<10>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<10>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<11>> for crate::reg32::Reg32Bits<11> {
    fn from(item: crate::reg8::Reg8Bits<11>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<11>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<11>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<11>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<11>> for crate::reg32::Reg32Bits<11> {
    fn from(item: crate::reg16::Reg16Bits<11>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<11>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<11>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<11>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<11>> for crate::reg32::Reg32Bits<11> {
    fn from(item: crate::reg64::Reg64Bits<11>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<11>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<11>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<11>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<12>> for crate::reg32::Reg32Bits<12> {
    fn from(item: crate::reg8::Reg8Bits<12>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<12>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<12>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<12>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<12>> for crate::reg32::Reg32Bits<12> {
    fn from(item: crate::reg16::Reg16Bits<12>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<12>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<12>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<12>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<12>> for crate::reg32::Reg32Bits<12> {
    fn from(item: crate::reg64::Reg64Bits<12>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<12>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<12>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<12>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<13>> for crate::reg32::Reg32Bits<13> {
    fn from(item: crate::reg8::Reg8Bits<13>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<13>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<13>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<13>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<13>> for crate::reg32::Reg32Bits<13> {
    fn from(item: crate::reg16::Reg16Bits<13>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<13>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<13>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<13>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<13>> for crate::reg32::Reg32Bits<13> {
    fn from(item: crate::reg64::Reg64Bits<13>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<13>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<13>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<13>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<14>> for crate::reg32::Reg32Bits<14> {
    fn from(item: crate::reg8::Reg8Bits<14>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<14>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<14>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<14>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<14>> for crate::reg32::Reg32Bits<14> {
    fn from(item: crate::reg16::Reg16Bits<14>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<14>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<14>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<14>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<14>> for crate::reg32::Reg32Bits<14> {
    fn from(item: crate::reg64::Reg64Bits<14>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<14>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<14>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<14>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<15>> for crate::reg32::Reg32Bits<15> {
    fn from(item: crate::reg8::Reg8Bits<15>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<15>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<15>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<15>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<15>> for crate::reg32::Reg32Bits<15> {
    fn from(item: crate::reg16::Reg16Bits<15>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<15>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<15>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<15>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<15>> for crate::reg32::Reg32Bits<15> {
    fn from(item: crate::reg64::Reg64Bits<15>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<15>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<15>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<15>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<16>> for crate::reg32::Reg32Bits<16> {
    fn from(item: crate::reg8::Reg8Bits<16>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<16>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<16>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<16>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<16>> for crate::reg32::Reg32Bits<16> {
    fn from(item: crate::reg16::Reg16Bits<16>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<16>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<16>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<16>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<16>> for crate::reg32::Reg32Bits<16> {
    fn from(item: crate::reg64::Reg64Bits<16>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<16>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<16>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<16>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<17>> for crate::reg32::Reg32Bits<17> {
    fn from(item: crate::reg8::Reg8Bits<17>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<17>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<17>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<17>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<17>> for crate::reg32::Reg32Bits<17> {
    fn from(item: crate::reg16::Reg16Bits<17>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<17>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<17>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<17>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<17>> for crate::reg32::Reg32Bits<17> {
    fn from(item: crate::reg64::Reg64Bits<17>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<17>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<17>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<17>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<18>> for crate::reg32::Reg32Bits<18> {
    fn from(item: crate::reg8::Reg8Bits<18>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<18>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<18>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<18>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<18>> for crate::reg32::Reg32Bits<18> {
    fn from(item: crate::reg16::Reg16Bits<18>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<18>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<18>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<18>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<18>> for crate::reg32::Reg32Bits<18> {
    fn from(item: crate::reg64::Reg64Bits<18>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<18>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<18>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<18>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<19>> for crate::reg32::Reg32Bits<19> {
    fn from(item: crate::reg8::Reg8Bits<19>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<19>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<19>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<19>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<19>> for crate::reg32::Reg32Bits<19> {
    fn from(item: crate::reg16::Reg16Bits<19>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<19>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<19>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<19>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<19>> for crate::reg32::Reg32Bits<19> {
    fn from(item: crate::reg64::Reg64Bits<19>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<19>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<19>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<19>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<20>> for crate::reg32::Reg32Bits<20> {
    fn from(item: crate::reg8::Reg8Bits<20>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<20>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<20>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<20>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<20>> for crate::reg32::Reg32Bits<20> {
    fn from(item: crate::reg16::Reg16Bits<20>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<20>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<20>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<20>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<20>> for crate::reg32::Reg32Bits<20> {
    fn from(item: crate::reg64::Reg64Bits<20>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<20>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<20>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<20>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<21>> for crate::reg32::Reg32Bits<21> {
    fn from(item: crate::reg8::Reg8Bits<21>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<21>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<21>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<21>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<21>> for crate::reg32::Reg32Bits<21> {
    fn from(item: crate::reg16::Reg16Bits<21>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<21>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<21>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<21>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<21>> for crate::reg32::Reg32Bits<21> {
    fn from(item: crate::reg64::Reg64Bits<21>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<21>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<21>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<21>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<22>> for crate::reg32::Reg32Bits<22> {
    fn from(item: crate::reg8::Reg8Bits<22>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<22>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<22>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<22>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<22>> for crate::reg32::Reg32Bits<22> {
    fn from(item: crate::reg16::Reg16Bits<22>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<22>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<22>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<22>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<22>> for crate::reg32::Reg32Bits<22> {
    fn from(item: crate::reg64::Reg64Bits<22>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<22>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<22>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<22>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<23>> for crate::reg32::Reg32Bits<23> {
    fn from(item: crate::reg8::Reg8Bits<23>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<23>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<23>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<23>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<23>> for crate::reg32::Reg32Bits<23> {
    fn from(item: crate::reg16::Reg16Bits<23>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<23>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<23>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<23>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<23>> for crate::reg32::Reg32Bits<23> {
    fn from(item: crate::reg64::Reg64Bits<23>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<23>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<23>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<23>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<24>> for crate::reg32::Reg32Bits<24> {
    fn from(item: crate::reg8::Reg8Bits<24>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<24>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<24>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<24>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<24>> for crate::reg32::Reg32Bits<24> {
    fn from(item: crate::reg16::Reg16Bits<24>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<24>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<24>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<24>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<24>> for crate::reg32::Reg32Bits<24> {
    fn from(item: crate::reg64::Reg64Bits<24>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<24>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<24>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<24>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<25>> for crate::reg32::Reg32Bits<25> {
    fn from(item: crate::reg8::Reg8Bits<25>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<25>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<25>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<25>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<25>> for crate::reg32::Reg32Bits<25> {
    fn from(item: crate::reg16::Reg16Bits<25>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<25>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<25>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<25>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<25>> for crate::reg32::Reg32Bits<25> {
    fn from(item: crate::reg64::Reg64Bits<25>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<25>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<25>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<25>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<26>> for crate::reg32::Reg32Bits<26> {
    fn from(item: crate::reg8::Reg8Bits<26>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<26>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<26>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<26>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<26>> for crate::reg32::Reg32Bits<26> {
    fn from(item: crate::reg16::Reg16Bits<26>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<26>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<26>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<26>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<26>> for crate::reg32::Reg32Bits<26> {
    fn from(item: crate::reg64::Reg64Bits<26>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<26>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<26>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<26>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<27>> for crate::reg32::Reg32Bits<27> {
    fn from(item: crate::reg8::Reg8Bits<27>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<27>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<27>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<27>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<27>> for crate::reg32::Reg32Bits<27> {
    fn from(item: crate::reg16::Reg16Bits<27>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<27>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<27>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<27>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<27>> for crate::reg32::Reg32Bits<27> {
    fn from(item: crate::reg64::Reg64Bits<27>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<27>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<27>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<27>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<28>> for crate::reg32::Reg32Bits<28> {
    fn from(item: crate::reg8::Reg8Bits<28>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<28>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<28>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<28>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<28>> for crate::reg32::Reg32Bits<28> {
    fn from(item: crate::reg16::Reg16Bits<28>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<28>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<28>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<28>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<28>> for crate::reg32::Reg32Bits<28> {
    fn from(item: crate::reg64::Reg64Bits<28>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<28>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<28>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<28>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<29>> for crate::reg32::Reg32Bits<29> {
    fn from(item: crate::reg8::Reg8Bits<29>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<29>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<29>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<29>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<29>> for crate::reg32::Reg32Bits<29> {
    fn from(item: crate::reg16::Reg16Bits<29>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<29>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<29>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<29>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<29>> for crate::reg32::Reg32Bits<29> {
    fn from(item: crate::reg64::Reg64Bits<29>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<29>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<29>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<29>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<30>> for crate::reg32::Reg32Bits<30> {
    fn from(item: crate::reg8::Reg8Bits<30>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<30>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<30>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<30>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<30>> for crate::reg32::Reg32Bits<30> {
    fn from(item: crate::reg16::Reg16Bits<30>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<30>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<30>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<30>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<30>> for crate::reg32::Reg32Bits<30> {
    fn from(item: crate::reg64::Reg64Bits<30>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<30>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<30>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<30>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<31>> for crate::reg32::Reg32Bits<31> {
    fn from(item: crate::reg8::Reg8Bits<31>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<31>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<31>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<31>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<31>> for crate::reg32::Reg32Bits<31> {
    fn from(item: crate::reg16::Reg16Bits<31>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<31>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<31>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<31>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<31>> for crate::reg32::Reg32Bits<31> {
    fn from(item: crate::reg64::Reg64Bits<31>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<31>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<31>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<31>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<32>> for crate::reg32::Reg32Bits<32> {
    fn from(item: crate::reg8::Reg8Bits<32>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<32>>::
            take_low(crate::reg32::Reg32Bits::new(u8::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<32>> for u8 {
    fn from(item: crate::reg32::Reg32Bits<32>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<32>> for crate::reg32::Reg32Bits<32> {
    fn from(item: crate::reg16::Reg16Bits<32>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<32>>::
            take_low(crate::reg32::Reg32Bits::new(u16::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<32>> for u16 {
    fn from(item: crate::reg32::Reg32Bits<32>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<32>> for crate::reg32::Reg32Bits<32> {
    fn from(item: crate::reg64::Reg64Bits<32>) -> Self {
        <crate::reg32::Reg32Bits<32> as crate::reg32::Reg32BitsDownCast<32>>::
            take_low(crate::reg32::Reg32Bits::new(u64::from(item) as u32))
    }
}
#[doc(hidden)]
impl From<crate::reg32::Reg32Bits<32>> for u64 {
    fn from(item: crate::reg32::Reg32Bits<32>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
impl From<u8> for crate::reg32::Reg32Bits<8> {
    fn from(item: u8) -> Self {
        Self(item.into())
    }
}
#[doc(hidden)]
impl From<u16> for crate::reg32::Reg32Bits<16> {
    fn from(item: u16) -> Self {
        Self(item.into())
    }
}

#[cfg(feature = "8bit")]
impl<const N: usize, const T: usize> Reg32BitsDownCast<T> for crate::reg8::Reg8Bits<N>
where
    Reg8Bits<N>: Into<Reg32Bits<N>>,
    Reg32Bits<N>: Reg32BitsDownCast<T>,
{
    fn take_low(self) -> Reg32Bits<T> {
        self.into().take_low()
    }
    fn take_high(self) -> Reg32Bits<T> {
        self.into().take_high()
    }
}

#[cfg(feature = "8bit")]
impl<const N: usize, const T: usize> Reg32BitsUpCast<T> for crate::reg8::Reg8Bits<N>
where
    Reg8Bits<N>: Into<Reg32Bits<N>>,
    Reg32Bits<N>: Reg32BitsUpCast<T>,
{
    fn zero_pad(self) -> Reg32Bits<T> {
        self.into().zero_pad()
    }
    fn zero_extend(self) -> Reg32Bits<T> {
        self.into().zero_extend()
    }
    fn sign_extend(self) -> Reg32Bits<T> {
        self.into().sign_extend()
    }
}

#[cfg(feature = "16bit")]
impl<const N: usize, const T: usize> Reg32BitsDownCast<T> for crate::reg16::Reg16Bits<N>
where
    Reg16Bits<N>: Into<Reg32Bits<N>>,
    Reg32Bits<N>: Reg32BitsDownCast<T>,
{
    fn take_low(self) -> Reg32Bits<T> {
        self.into().take_low()
    }
    fn take_high(self) -> Reg32Bits<T> {
        self.into().take_high()
    }
}

#[cfg(feature = "16bit")]
impl<const N: usize, const T: usize> Reg32BitsUpCast<T> for crate::reg16::Reg16Bits<N>
where
    Reg16Bits<N>: Into<Reg32Bits<N>>,
    Reg32Bits<N>: Reg32BitsUpCast<T>,
{
    fn zero_pad(self) -> Reg32Bits<T> {
        self.into().zero_pad()
    }
    fn zero_extend(self) -> Reg32Bits<T> {
        self.into().zero_extend()
    }
    fn sign_extend(self) -> Reg32Bits<T> {
        self.into().sign_extend()
    }
}

#[cfg(feature = "64bit")]
impl<const N: usize, const T: usize> Reg32BitsDownCast<T> for crate::reg64::Reg64Bits<N>
where
    Reg64Bits<N>: Into<Reg32Bits<N>>,
    Reg32Bits<N>: Reg32BitsDownCast<T>,
{
    fn take_low(self) -> Reg32Bits<T> {
        self.into().take_low()
    }
    fn take_high(self) -> Reg32Bits<T> {
        self.into().take_high()
    }
}

#[cfg(feature = "64bit")]
impl<const N: usize, const T: usize> Reg32BitsUpCast<T> for crate::reg64::Reg64Bits<N>
where
    Reg64Bits<N>: Into<Reg32Bits<N>>,
    Reg32Bits<N>: Reg32BitsUpCast<T>,
{
    fn zero_pad(self) -> Reg32Bits<T> {
        self.into().zero_pad()
    }
    fn zero_extend(self) -> Reg32Bits<T> {
        self.into().zero_extend()
    }
    fn sign_extend(self) -> Reg32Bits<T> {
        self.into().sign_extend()
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<1> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<2> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<3> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<4> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<5> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<6> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<7> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<8> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<9> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<10> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<11> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<12> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<13> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<14> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<15> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<16> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<17> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<18> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<19> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<20> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<21> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<22> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<23> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<24> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<25> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<26> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<27> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<28> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<29> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<30> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<31> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg32::Reg32Bits<32> {
    fn default() -> Self {
        Self(0)
    }
}
