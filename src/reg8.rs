// This file was automatically generated with the `generate_impl_rs.py` script.
// Any bugs in this script should be addressed in the `reg_reference.rs` file.
// 
// The contents of this file is as follows:
// 1. Definition of RegXBits struct
// 2. Implementation of operations
// 3. Implementation of traits
use core::num::Wrapping;


// The next two lines will be replaced with the appropriate base type and size
type BaseType = u8; // [REF_REPLACE]
const NUM_BITS: usize = BaseType::BITS as usize;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(transparent)]
pub struct Reg8Bits<const N: usize>(BaseType);

impl<const N: usize> AsRef<BaseType> for Reg8Bits<N> {
    fn as_ref(&self) -> &BaseType {
        let Self(inner) = self;
        inner
    }
}

impl<const N: usize> core::ops::Deref for Reg8Bits<N> {
    type Target = BaseType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Eq for Reg8Bits<N> {}
impl<const N: usize> Ord for Reg8Bits<N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Reg8Bits<NUM_BITS> {
    #[inline(always)]
    pub fn new(value: BaseType) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<Reg8Bits<N>> for BaseType {
    #[inline(always)]
    fn from(item: Reg8Bits<N>) -> Self {
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
impl<const N: usize> Reg8Bits<N> {
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
    /// let value: Reg8Bits<4> =
    ///     Reg8Bits::new(0b1010).take_low();
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
    /// This will fail if index >= N, where N is the size of the [Reg8Bits].
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// let bits: Reg8Bits<4> =
    ///     Reg8Bits::new(0b1100).take_low();
    ///
    /// assert_eq!(bits.get(0).unwrap(), 0u8);
    /// assert_eq!(bits.get(1).unwrap(), 0u8);
    /// assert_eq!(bits.get(2).unwrap(), 1u8);
    /// assert_eq!(bits.get(3).unwrap(), 1u8);
    /// assert_eq!(bits.get(4), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<Reg8Bits<1>> {
        if index >= N {
            return None;
        }

        let last_bit = if index == 0 {
            self.0 & 0b1
        } else {
            (self.0 >> index) & 0b1
        };

        Some(Reg8Bits(last_bit))
    }
}

impl From<Reg8Bits<1>> for bool {
    #[inline(always)]
    fn from(item: Reg8Bits<1>) -> bool {
        matches!(item, Reg8Bits::<1>(1))
    }
}

impl PartialEq<bool> for Reg8Bits<1> {
    #[inline(always)]
    fn eq(&self, other: &bool) -> bool {
        bool::from(*self) == *other
    }
}

impl PartialEq<u8> for Reg8Bits<1> {
    #[inline(always)]
    fn eq(&self, other: &u8) -> bool {
        bool::from(*self) == (*other > 0)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Add<T> for Reg8Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: T) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.into());

        Self(((lhs + rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Sub<T> for Reg8Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: T) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.into());

        Self(((lhs - rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Div<T> for Reg8Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: T) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.into();

        Self(lhs / rhs)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Rem<T> for Reg8Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: T) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.into();

        Self(lhs % rhs)
    }
}

impl<const N: usize> core::ops::BitAnd for Reg8Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs & rhs)
    }
}

impl<const N: usize> core::ops::BitOr for Reg8Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs | rhs)
    }
}

impl<const N: usize> core::ops::BitXor for Reg8Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs ^ rhs)
    }
}

impl<const N: usize> core::ops::Not for Reg8Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        let lhs = self.0;

        Self((!lhs) & Self::BASE_ONES)
    }
}

impl<const N: usize, T> core::ops::Shl<T> for Reg8Bits<N>
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

impl<const N: usize, T> core::ops::Shr<T> for Reg8Bits<N>
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

pub trait Reg8BitsBitSize {
    const BIT_SIZE: usize;
    const BASE_ONES: BaseType;
}

impl<const N: usize> Reg8BitsBitSize for Reg8Bits<N> {
    const BIT_SIZE: usize = N;
    const BASE_ONES: BaseType = Reg8Bits::<N>::BASE_ONES;
}

// F > T
pub trait Reg8BitsDownCast<const T: usize>: Reg8BitsBitSize + Copy + Into<BaseType> {
    /// Take a number of the least significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bottom_byte: Reg8Bits<8> = Reg8Bits::new(0x42).take_low();
    /// let bits: Reg8Bits<4> = bottom_byte.take_low();
    ///
    /// assert_eq!(bits, 0x2);
    /// ```
    #[inline(always)]
    fn take_low(self) -> Reg8Bits<T> {
        let value: BaseType = self.into();
        Reg8Bits(Reg8Bits::<T>::BASE_ONES & value)
    }

    /// Take a number of the most significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bottom_byte: Reg8Bits<8> =
    ///     Reg8Bits::new(0x42).take_low();
    /// let bits: Reg8Bits<4> = bottom_byte.take_high();
    ///
    /// assert_eq!(bits, 0x4);
    /// ```
    #[inline(always)]
    fn take_high(self) -> Reg8Bits<T> {
        let value: BaseType = self.into();
        Reg8Bits(value >> (Self::BIT_SIZE - T))
    }
}

pub trait Reg8BitsUpCast<const T: usize>: Reg8BitsBitSize + Copy + Into<BaseType> {
    /// Extend the current register bits with 0s on the most significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bits: Reg8Bits<4> =
    ///     Reg8Bits::new(0b1010).take_low();
    ///
    /// // 1010 => 0000 1010
    /// assert_eq!(bits.zero_extend().bits(), [0, 0, 0, 0, 1, 0, 1, 0]);
    /// ```
    #[inline(always)]
    fn zero_extend(self) -> Reg8Bits<T> {
        let value = self.into();
        Reg8Bits(value)
    }

    /// Extend the current register bits by copied the most significant bit to the new bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// // Most significant bit is 1
    /// let bits: Reg8Bits<4> =
    ///     Reg8Bits::new(0b1010).take_low();
    ///
    /// // 1010 => 1111 1010
    /// assert_eq!(bits.sign_extend().bits(), [1, 1, 1, 1, 1, 0, 1, 0]);
    ///
    /// // Most significant bit is 0
    /// let bits: Reg8Bits<4> =
    ///     Reg8Bits::new(0b0101).take_low();
    ///
    /// // 0101 => 0000 0101
    /// assert_eq!(bits.sign_extend().bits(), [0, 0, 0, 0, 0, 1, 0, 1]);
    /// ```
    #[inline(always)]
    fn sign_extend(self) -> Reg8Bits<T> {
        // NOTE: We are assuming here that no Bits<0> structure can exist
        let value = self.into();

        let top_bit = value & top_bit_mask(Self::BIT_SIZE); // Capture only the top bit
        let top_bits = if top_bit == 0 {
            // Create a set of NUM_BITS-N bits of with the given sign
            0
        } else {
            (!Self::BASE_ONES) & Reg8Bits::<T>::BASE_ONES // !001111 = 110000
        };

        Reg8Bits(top_bits | value)
    }

    /// Add extra zeros for the least significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// let bits: Reg8Bits<4> =
    ///     Reg8Bits::new(0b1010).take_low();
    ///
    /// // 1010 => 1010 0000
    /// assert_eq!(bits.zero_pad().bits(), [1, 0, 1, 0, 0, 0, 0, 0]);
    /// ```
    #[inline(always)]
    fn zero_pad(self) -> Reg8Bits<T> {
        self.zero_extend() << (T - Self::BIT_SIZE)
    }
}

pub trait Reg8BitsConcat<const R: usize, const O: usize>:
    Copy + Into<BaseType>
{
    /// Concatinate two register bit structures forming a new struct sized the sum of their
    /// concated structs
    fn concat(self, rhs: Reg8Bits<R>) -> Reg8Bits<O> {
        let lhs = self.into();
        let rhs: BaseType = rhs.into();

        Reg8Bits((lhs << R) | rhs)
    }
}

impl<const F: usize, const T: usize> Reg8BitsUpCast<T> for Reg8Bits<F> where
    Reg8Bits<T>: Reg8BitsDownCast<F>
{
}



#[doc(hidden)]
impl Reg8BitsDownCast<1> for Reg8Bits<1> {}
#[doc(hidden)]
impl Reg8BitsConcat<1, 2> for Reg8Bits<1> {}
#[doc(hidden)]
impl Reg8BitsConcat<2, 3> for Reg8Bits<1> {}
#[doc(hidden)]
impl Reg8BitsConcat<3, 4> for Reg8Bits<1> {}
#[doc(hidden)]
impl Reg8BitsConcat<4, 5> for Reg8Bits<1> {}
#[doc(hidden)]
impl Reg8BitsConcat<5, 6> for Reg8Bits<1> {}
#[doc(hidden)]
impl Reg8BitsConcat<6, 7> for Reg8Bits<1> {}
#[doc(hidden)]
impl Reg8BitsConcat<7, 8> for Reg8Bits<1> {}
#[doc(hidden)]
impl Reg8BitsDownCast<1> for Reg8Bits<2> {}
#[doc(hidden)]
impl Reg8BitsConcat<1, 3> for Reg8Bits<2> {}
#[doc(hidden)]
impl Reg8BitsDownCast<2> for Reg8Bits<2> {}
#[doc(hidden)]
impl Reg8BitsConcat<2, 4> for Reg8Bits<2> {}
#[doc(hidden)]
impl Reg8BitsConcat<3, 5> for Reg8Bits<2> {}
#[doc(hidden)]
impl Reg8BitsConcat<4, 6> for Reg8Bits<2> {}
#[doc(hidden)]
impl Reg8BitsConcat<5, 7> for Reg8Bits<2> {}
#[doc(hidden)]
impl Reg8BitsConcat<6, 8> for Reg8Bits<2> {}
#[doc(hidden)]
impl Reg8BitsDownCast<1> for Reg8Bits<3> {}
#[doc(hidden)]
impl Reg8BitsConcat<1, 4> for Reg8Bits<3> {}
#[doc(hidden)]
impl Reg8BitsDownCast<2> for Reg8Bits<3> {}
#[doc(hidden)]
impl Reg8BitsConcat<2, 5> for Reg8Bits<3> {}
#[doc(hidden)]
impl Reg8BitsDownCast<3> for Reg8Bits<3> {}
#[doc(hidden)]
impl Reg8BitsConcat<3, 6> for Reg8Bits<3> {}
#[doc(hidden)]
impl Reg8BitsConcat<4, 7> for Reg8Bits<3> {}
#[doc(hidden)]
impl Reg8BitsConcat<5, 8> for Reg8Bits<3> {}
#[doc(hidden)]
impl Reg8BitsDownCast<1> for Reg8Bits<4> {}
#[doc(hidden)]
impl Reg8BitsConcat<1, 5> for Reg8Bits<4> {}
#[doc(hidden)]
impl Reg8BitsDownCast<2> for Reg8Bits<4> {}
#[doc(hidden)]
impl Reg8BitsConcat<2, 6> for Reg8Bits<4> {}
#[doc(hidden)]
impl Reg8BitsDownCast<3> for Reg8Bits<4> {}
#[doc(hidden)]
impl Reg8BitsConcat<3, 7> for Reg8Bits<4> {}
#[doc(hidden)]
impl Reg8BitsDownCast<4> for Reg8Bits<4> {}
#[doc(hidden)]
impl Reg8BitsConcat<4, 8> for Reg8Bits<4> {}
#[doc(hidden)]
impl Reg8BitsDownCast<1> for Reg8Bits<5> {}
#[doc(hidden)]
impl Reg8BitsConcat<1, 6> for Reg8Bits<5> {}
#[doc(hidden)]
impl Reg8BitsDownCast<2> for Reg8Bits<5> {}
#[doc(hidden)]
impl Reg8BitsConcat<2, 7> for Reg8Bits<5> {}
#[doc(hidden)]
impl Reg8BitsDownCast<3> for Reg8Bits<5> {}
#[doc(hidden)]
impl Reg8BitsConcat<3, 8> for Reg8Bits<5> {}
#[doc(hidden)]
impl Reg8BitsDownCast<4> for Reg8Bits<5> {}
#[doc(hidden)]
impl Reg8BitsDownCast<5> for Reg8Bits<5> {}
#[doc(hidden)]
impl Reg8BitsDownCast<1> for Reg8Bits<6> {}
#[doc(hidden)]
impl Reg8BitsConcat<1, 7> for Reg8Bits<6> {}
#[doc(hidden)]
impl Reg8BitsDownCast<2> for Reg8Bits<6> {}
#[doc(hidden)]
impl Reg8BitsConcat<2, 8> for Reg8Bits<6> {}
#[doc(hidden)]
impl Reg8BitsDownCast<3> for Reg8Bits<6> {}
#[doc(hidden)]
impl Reg8BitsDownCast<4> for Reg8Bits<6> {}
#[doc(hidden)]
impl Reg8BitsDownCast<5> for Reg8Bits<6> {}
#[doc(hidden)]
impl Reg8BitsDownCast<6> for Reg8Bits<6> {}
#[doc(hidden)]
impl Reg8BitsDownCast<1> for Reg8Bits<7> {}
#[doc(hidden)]
impl Reg8BitsConcat<1, 8> for Reg8Bits<7> {}
#[doc(hidden)]
impl Reg8BitsDownCast<2> for Reg8Bits<7> {}
#[doc(hidden)]
impl Reg8BitsDownCast<3> for Reg8Bits<7> {}
#[doc(hidden)]
impl Reg8BitsDownCast<4> for Reg8Bits<7> {}
#[doc(hidden)]
impl Reg8BitsDownCast<5> for Reg8Bits<7> {}
#[doc(hidden)]
impl Reg8BitsDownCast<6> for Reg8Bits<7> {}
#[doc(hidden)]
impl Reg8BitsDownCast<7> for Reg8Bits<7> {}
#[doc(hidden)]
impl Reg8BitsDownCast<1> for Reg8Bits<8> {}
#[doc(hidden)]
impl Reg8BitsDownCast<2> for Reg8Bits<8> {}
#[doc(hidden)]
impl Reg8BitsDownCast<3> for Reg8Bits<8> {}
#[doc(hidden)]
impl Reg8BitsDownCast<4> for Reg8Bits<8> {}
#[doc(hidden)]
impl Reg8BitsDownCast<5> for Reg8Bits<8> {}
#[doc(hidden)]
impl Reg8BitsDownCast<6> for Reg8Bits<8> {}
#[doc(hidden)]
impl Reg8BitsDownCast<7> for Reg8Bits<8> {}
#[doc(hidden)]
impl Reg8BitsDownCast<8> for Reg8Bits<8> {}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg8Bits<2> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg8Bits<3> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg8Bits<4> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg8Bits<5> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg8Bits<6> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg8Bits<7> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg8Bits<8> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<1>> for crate::reg8::Reg8Bits<1> {
    fn from(item: crate::reg16::Reg16Bits<1>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<1>>::
            take_low(crate::reg8::Reg8Bits::new(u16::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<1>> for u16 {
    fn from(item: crate::reg8::Reg8Bits<1>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<1>> for crate::reg8::Reg8Bits<1> {
    fn from(item: crate::reg32::Reg32Bits<1>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<1>>::
            take_low(crate::reg8::Reg8Bits::new(u32::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<1>> for u32 {
    fn from(item: crate::reg8::Reg8Bits<1>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<1>> for crate::reg8::Reg8Bits<1> {
    fn from(item: crate::reg64::Reg64Bits<1>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<1>>::
            take_low(crate::reg8::Reg8Bits::new(u64::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<1>> for u64 {
    fn from(item: crate::reg8::Reg8Bits<1>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<2>> for crate::reg8::Reg8Bits<2> {
    fn from(item: crate::reg16::Reg16Bits<2>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<2>>::
            take_low(crate::reg8::Reg8Bits::new(u16::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<2>> for u16 {
    fn from(item: crate::reg8::Reg8Bits<2>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<2>> for crate::reg8::Reg8Bits<2> {
    fn from(item: crate::reg32::Reg32Bits<2>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<2>>::
            take_low(crate::reg8::Reg8Bits::new(u32::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<2>> for u32 {
    fn from(item: crate::reg8::Reg8Bits<2>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<2>> for crate::reg8::Reg8Bits<2> {
    fn from(item: crate::reg64::Reg64Bits<2>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<2>>::
            take_low(crate::reg8::Reg8Bits::new(u64::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<2>> for u64 {
    fn from(item: crate::reg8::Reg8Bits<2>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<3>> for crate::reg8::Reg8Bits<3> {
    fn from(item: crate::reg16::Reg16Bits<3>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<3>>::
            take_low(crate::reg8::Reg8Bits::new(u16::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<3>> for u16 {
    fn from(item: crate::reg8::Reg8Bits<3>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<3>> for crate::reg8::Reg8Bits<3> {
    fn from(item: crate::reg32::Reg32Bits<3>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<3>>::
            take_low(crate::reg8::Reg8Bits::new(u32::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<3>> for u32 {
    fn from(item: crate::reg8::Reg8Bits<3>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<3>> for crate::reg8::Reg8Bits<3> {
    fn from(item: crate::reg64::Reg64Bits<3>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<3>>::
            take_low(crate::reg8::Reg8Bits::new(u64::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<3>> for u64 {
    fn from(item: crate::reg8::Reg8Bits<3>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<4>> for crate::reg8::Reg8Bits<4> {
    fn from(item: crate::reg16::Reg16Bits<4>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<4>>::
            take_low(crate::reg8::Reg8Bits::new(u16::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<4>> for u16 {
    fn from(item: crate::reg8::Reg8Bits<4>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<4>> for crate::reg8::Reg8Bits<4> {
    fn from(item: crate::reg32::Reg32Bits<4>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<4>>::
            take_low(crate::reg8::Reg8Bits::new(u32::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<4>> for u32 {
    fn from(item: crate::reg8::Reg8Bits<4>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<4>> for crate::reg8::Reg8Bits<4> {
    fn from(item: crate::reg64::Reg64Bits<4>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<4>>::
            take_low(crate::reg8::Reg8Bits::new(u64::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<4>> for u64 {
    fn from(item: crate::reg8::Reg8Bits<4>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<5>> for crate::reg8::Reg8Bits<5> {
    fn from(item: crate::reg16::Reg16Bits<5>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<5>>::
            take_low(crate::reg8::Reg8Bits::new(u16::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<5>> for u16 {
    fn from(item: crate::reg8::Reg8Bits<5>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<5>> for crate::reg8::Reg8Bits<5> {
    fn from(item: crate::reg32::Reg32Bits<5>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<5>>::
            take_low(crate::reg8::Reg8Bits::new(u32::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<5>> for u32 {
    fn from(item: crate::reg8::Reg8Bits<5>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<5>> for crate::reg8::Reg8Bits<5> {
    fn from(item: crate::reg64::Reg64Bits<5>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<5>>::
            take_low(crate::reg8::Reg8Bits::new(u64::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<5>> for u64 {
    fn from(item: crate::reg8::Reg8Bits<5>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<6>> for crate::reg8::Reg8Bits<6> {
    fn from(item: crate::reg16::Reg16Bits<6>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<6>>::
            take_low(crate::reg8::Reg8Bits::new(u16::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<6>> for u16 {
    fn from(item: crate::reg8::Reg8Bits<6>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<6>> for crate::reg8::Reg8Bits<6> {
    fn from(item: crate::reg32::Reg32Bits<6>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<6>>::
            take_low(crate::reg8::Reg8Bits::new(u32::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<6>> for u32 {
    fn from(item: crate::reg8::Reg8Bits<6>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<6>> for crate::reg8::Reg8Bits<6> {
    fn from(item: crate::reg64::Reg64Bits<6>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<6>>::
            take_low(crate::reg8::Reg8Bits::new(u64::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<6>> for u64 {
    fn from(item: crate::reg8::Reg8Bits<6>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<7>> for crate::reg8::Reg8Bits<7> {
    fn from(item: crate::reg16::Reg16Bits<7>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<7>>::
            take_low(crate::reg8::Reg8Bits::new(u16::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<7>> for u16 {
    fn from(item: crate::reg8::Reg8Bits<7>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<7>> for crate::reg8::Reg8Bits<7> {
    fn from(item: crate::reg32::Reg32Bits<7>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<7>>::
            take_low(crate::reg8::Reg8Bits::new(u32::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<7>> for u32 {
    fn from(item: crate::reg8::Reg8Bits<7>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<7>> for crate::reg8::Reg8Bits<7> {
    fn from(item: crate::reg64::Reg64Bits<7>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<7>>::
            take_low(crate::reg8::Reg8Bits::new(u64::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<7>> for u64 {
    fn from(item: crate::reg8::Reg8Bits<7>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<8>> for crate::reg8::Reg8Bits<8> {
    fn from(item: crate::reg16::Reg16Bits<8>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<8>>::
            take_low(crate::reg8::Reg8Bits::new(u16::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<8>> for u16 {
    fn from(item: crate::reg8::Reg8Bits<8>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<8>> for crate::reg8::Reg8Bits<8> {
    fn from(item: crate::reg32::Reg32Bits<8>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<8>>::
            take_low(crate::reg8::Reg8Bits::new(u32::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<8>> for u32 {
    fn from(item: crate::reg8::Reg8Bits<8>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "64bit")]
impl From<crate::reg64::Reg64Bits<8>> for crate::reg8::Reg8Bits<8> {
    fn from(item: crate::reg64::Reg64Bits<8>) -> Self {
        <crate::reg8::Reg8Bits<8> as crate::reg8::Reg8BitsDownCast<8>>::
            take_low(crate::reg8::Reg8Bits::new(u64::from(item) as u8))
    }
}
#[doc(hidden)]
impl From<crate::reg8::Reg8Bits<8>> for u64 {
    fn from(item: crate::reg8::Reg8Bits<8>) -> Self {
        item.0 as u64
    }
}
#[doc(hidden)]
impl Default for crate::reg8::Reg8Bits<1> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg8::Reg8Bits<2> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg8::Reg8Bits<3> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg8::Reg8Bits<4> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg8::Reg8Bits<5> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg8::Reg8Bits<6> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg8::Reg8Bits<7> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg8::Reg8Bits<8> {
    fn default() -> Self {
        Self(0)
    }
}
