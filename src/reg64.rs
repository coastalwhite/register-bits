// This file was automatically generated with the `generate_impl_rs.py` script.
// Any bugs in this script should be addressed in the `reg_reference.rs` file.
// 
// The contents of this file is as follows:
// 1. Definition of RegXBits struct
// 2. Implementation of operations
// 3. Implementation of traits
use core::num::Wrapping;


// The next two lines will be replaced with the appropriate base type and size
type BaseType = u64; // [REF_REPLACE]
const NUM_BITS: usize = BaseType::BITS as usize;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(transparent)]
pub struct Reg64Bits<const N: usize>(BaseType);

impl<const N: usize> core::ops::Deref for Reg64Bits<N> {
    type Target = BaseType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Eq for Reg64Bits<N> {}
impl<const N: usize> Ord for Reg64Bits<N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Reg64Bits<NUM_BITS> {
    #[inline(always)]
    pub fn new(value: BaseType) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<Reg64Bits<N>> for BaseType {
    #[inline(always)]
    fn from(item: Reg64Bits<N>) -> Self {
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
impl<const N: usize> Reg64Bits<N> {
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
    /// let value: Reg64Bits<4> =
    ///     Reg64Bits::new(0b1010).take_low();
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
    /// This will fail if index >= N, where N is the size of the [Reg64Bits].
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// let bits: Reg64Bits<4> =
    ///     Reg64Bits::new(0b1100).take_low();
    ///
    /// assert_eq!(bits.get(0).unwrap(), 0u8);
    /// assert_eq!(bits.get(1).unwrap(), 0u8);
    /// assert_eq!(bits.get(2).unwrap(), 1u8);
    /// assert_eq!(bits.get(3).unwrap(), 1u8);
    /// assert_eq!(bits.get(4), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<Reg64Bits<1>> {
        if index >= N {
            return None;
        }

        let last_bit = if index == 0 {
            self.0 & 0b1
        } else {
            (self.0 >> index) & 0b1
        };

        Some(Reg64Bits(last_bit))
    }
}

impl From<Reg64Bits<1>> for bool {
    #[inline(always)]
    fn from(item: Reg64Bits<1>) -> bool {
        matches!(item, Reg64Bits::<1>(1))
    }
}

impl PartialEq<bool> for Reg64Bits<1> {
    #[inline(always)]
    fn eq(&self, other: &bool) -> bool {
        bool::from(*self) == *other
    }
}

impl PartialEq<u8> for Reg64Bits<1> {
    #[inline(always)]
    fn eq(&self, other: &u8) -> bool {
        bool::from(*self) == (*other > 0)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Add<T> for Reg64Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: T) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.into());

        Self(((lhs + rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Sub<T> for Reg64Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: T) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.into());

        Self(((lhs - rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Div<T> for Reg64Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: T) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.into();

        Self(lhs / rhs)
    }
}

impl<const N: usize, T: Into<BaseType>> core::ops::Rem<T> for Reg64Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: T) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.into();

        Self(lhs % rhs)
    }
}

impl<const N: usize> core::ops::BitAnd for Reg64Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs & rhs)
    }
}

impl<const N: usize> core::ops::BitOr for Reg64Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs | rhs)
    }
}

impl<const N: usize> core::ops::BitXor for Reg64Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs ^ rhs)
    }
}

impl<const N: usize> core::ops::Not for Reg64Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        let lhs = self.0;

        Self((!lhs) & Self::BASE_ONES)
    }
}

impl<const N: usize, T> core::ops::Shl<T> for Reg64Bits<N>
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

impl<const N: usize, T> core::ops::Shr<T> for Reg64Bits<N>
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

pub trait Reg64BitsBitSize {
    const BIT_SIZE: usize;
    const BASE_ONES: BaseType;
}

impl<const N: usize> Reg64BitsBitSize for Reg64Bits<N> {
    const BIT_SIZE: usize = N;
    const BASE_ONES: BaseType = Reg64Bits::<N>::BASE_ONES;
}

// F > T
pub trait Reg64BitsDownCast<const T: usize>: Reg64BitsBitSize + Copy + Into<BaseType> {
    /// Take a number of the least significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bottom_byte: Reg64Bits<8> = Reg64Bits::new(0x42).take_low();
    /// let bits: Reg64Bits<4> = bottom_byte.take_low();
    ///
    /// assert_eq!(bits, 0x2);
    /// ```
    #[inline(always)]
    fn take_low(self) -> Reg64Bits<T> {
        let value: BaseType = self.into();
        Reg64Bits(Reg64Bits::<T>::BASE_ONES & value)
    }

    /// Take a number of the most significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bottom_byte: Reg64Bits<8> =
    ///     Reg64Bits::new(0x42).take_low();
    /// let bits: Reg64Bits<4> = bottom_byte.take_high();
    ///
    /// assert_eq!(bits, 0x4);
    /// ```
    #[inline(always)]
    fn take_high(self) -> Reg64Bits<T> {
        let value: BaseType = self.into();
        Reg64Bits(value >> (Self::BIT_SIZE - T))
    }
}

pub trait Reg64BitsUpCast<const T: usize>: Reg64BitsBitSize + Copy + Into<BaseType> {
    /// Extend the current register bits with 0s on the most significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    /// let bits: Reg64Bits<4> =
    ///     Reg64Bits::new(0b1010).take_low();
    ///
    /// // 1010 => 0000 1010
    /// assert_eq!(bits.zero_extend().bits(), [0, 0, 0, 0, 1, 0, 1, 0]);
    /// ```
    #[inline(always)]
    fn zero_extend(self) -> Reg64Bits<T> {
        let value = self.into();
        Reg64Bits(value)
    }

    /// Extend the current register bits by copied the most significant bit to the new bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// // Most significant bit is 1
    /// let bits: Reg64Bits<4> =
    ///     Reg64Bits::new(0b1010).take_low();
    ///
    /// // 1010 => 1111 1010
    /// assert_eq!(bits.sign_extend().bits(), [1, 1, 1, 1, 1, 0, 1, 0]);
    ///
    /// // Most significant bit is 0
    /// let bits: Reg64Bits<4> =
    ///     Reg64Bits::new(0b0101).take_low();
    ///
    /// // 0101 => 0000 0101
    /// assert_eq!(bits.sign_extend().bits(), [0, 0, 0, 0, 0, 1, 0, 1]);
    /// ```
    #[inline(always)]
    fn sign_extend(self) -> Reg64Bits<T> {
        // NOTE: We are assuming here that no Bits<0> structure can exist
        let value = self.into();

        let top_bit = value & top_bit_mask(Self::BIT_SIZE); // Capture only the top bit
        let top_bits = if top_bit == 0 {
            // Create a set of NUM_BITS-N bits of with the given sign
            0
        } else {
            (!Self::BASE_ONES) & Reg64Bits::<T>::BASE_ONES // !001111 = 110000
        };

        Reg64Bits(top_bits | value)
    }

    /// Add extra zeros for the least significant bits
    ///
    /// # Example
    ///
    /// ```
    /// use register_bits::prelude::*;
    ///
    /// let bits: Reg64Bits<4> =
    ///     Reg64Bits::new(0b1010).take_low();
    ///
    /// // 1010 => 1010 0000
    /// assert_eq!(bits.zero_pad().bits(), [1, 0, 1, 0, 0, 0, 0, 0]);
    /// ```
    #[inline(always)]
    fn zero_pad(self) -> Reg64Bits<T> {
        self.zero_extend() << (T - Self::BIT_SIZE)
    }
}

pub trait Reg64BitsConcat<const R: usize, const O: usize>:
    Copy + Into<BaseType>
{
    /// Concatinate two register bit structures forming a new struct sized the sum of their
    /// concated structs
    fn concat(self, rhs: Reg64Bits<R>) -> Reg64Bits<O> {
        let lhs = self.into();
        let rhs: BaseType = rhs.into();

        Reg64Bits((lhs << R) | rhs)
    }
}

impl<const F: usize, const T: usize> Reg64BitsUpCast<T> for Reg64Bits<F> where
    Reg64Bits<T>: Reg64BitsDownCast<F>
{
}



#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 2> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 3> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 4> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 5> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 6> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 7> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 8> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 9> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 10> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 11> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 12> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 13> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 14> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 15> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 16> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 17> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 18> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 19> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 20> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 21> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 22> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 23> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 24> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 25> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 26> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 27> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 28> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 29> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 30> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 31> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 32> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 33> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 34> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 35> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 36> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 37> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 38> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 39> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 40> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 41> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 42> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 43> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 44> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 45> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 46> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 47> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 48> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 49> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 50> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<50, 51> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<51, 52> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<52, 53> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<53, 54> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<54, 55> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<55, 56> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<56, 57> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<57, 58> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<58, 59> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<59, 60> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<60, 61> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<61, 62> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<62, 63> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsConcat<63, 64> for Reg64Bits<1> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 3> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 4> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 5> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 6> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 7> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 8> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 9> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 10> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 11> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 12> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 13> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 14> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 15> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 16> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 17> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 18> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 19> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 20> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 21> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 22> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 23> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 24> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 25> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 26> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 27> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 28> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 29> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 30> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 31> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 32> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 33> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 34> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 35> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 36> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 37> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 38> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 39> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 40> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 41> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 42> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 43> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 44> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 45> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 46> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 47> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 48> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 49> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 50> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 51> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<50, 52> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<51, 53> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<52, 54> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<53, 55> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<54, 56> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<55, 57> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<56, 58> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<57, 59> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<58, 60> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<59, 61> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<60, 62> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<61, 63> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsConcat<62, 64> for Reg64Bits<2> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 4> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 5> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 6> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 7> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 8> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 9> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 10> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 11> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 12> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 13> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 14> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 15> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 16> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 17> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 18> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 19> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 20> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 21> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 22> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 23> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 24> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 25> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 26> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 27> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 28> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 29> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 30> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 31> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 32> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 33> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 34> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 35> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 36> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 37> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 38> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 39> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 40> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 41> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 42> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 43> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 44> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 45> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 46> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 47> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 48> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 49> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 50> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 51> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 52> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<50, 53> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<51, 54> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<52, 55> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<53, 56> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<54, 57> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<55, 58> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<56, 59> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<57, 60> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<58, 61> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<59, 62> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<60, 63> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsConcat<61, 64> for Reg64Bits<3> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 5> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 6> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 7> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 8> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 9> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 10> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 11> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 12> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 13> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 14> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 15> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 16> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 17> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 18> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 19> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 20> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 21> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 22> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 23> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 24> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 25> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 26> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 27> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 28> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 29> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 30> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 31> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 32> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 33> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 34> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 35> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 36> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 37> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 38> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 39> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 40> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 41> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 42> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 43> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 44> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 45> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 46> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 47> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 48> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 49> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 50> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 51> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 52> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 53> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<50, 54> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<51, 55> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<52, 56> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<53, 57> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<54, 58> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<55, 59> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<56, 60> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<57, 61> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<58, 62> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<59, 63> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsConcat<60, 64> for Reg64Bits<4> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 6> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 7> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 8> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 9> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 10> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 11> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 12> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 13> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 14> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 15> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 16> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 17> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 18> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 19> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 20> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 21> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 22> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 23> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 24> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 25> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 26> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 27> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 28> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 29> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 30> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 31> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 32> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 33> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 34> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 35> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 36> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 37> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 38> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 39> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 40> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 41> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 42> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 43> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 44> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 45> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 46> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 47> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 48> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 49> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 50> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 51> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 52> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 53> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 54> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<50, 55> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<51, 56> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<52, 57> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<53, 58> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<54, 59> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<55, 60> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<56, 61> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<57, 62> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<58, 63> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsConcat<59, 64> for Reg64Bits<5> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 7> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 8> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 9> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 10> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 11> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 12> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 13> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 14> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 15> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 16> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 17> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 18> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 19> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 20> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 21> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 22> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 23> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 24> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 25> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 26> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 27> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 28> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 29> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 30> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 31> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 32> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 33> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 34> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 35> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 36> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 37> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 38> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 39> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 40> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 41> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 42> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 43> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 44> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 45> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 46> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 47> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 48> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 49> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 50> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 51> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 52> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 53> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 54> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 55> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<50, 56> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<51, 57> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<52, 58> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<53, 59> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<54, 60> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<55, 61> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<56, 62> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<57, 63> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsConcat<58, 64> for Reg64Bits<6> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 8> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 9> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 10> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 11> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 12> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 13> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 14> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 15> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 16> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 17> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 18> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 19> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 20> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 21> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 22> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 23> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 24> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 25> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 26> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 27> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 28> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 29> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 30> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 31> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 32> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 33> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 34> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 35> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 36> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 37> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 38> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 39> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 40> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 41> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 42> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 43> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 44> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 45> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 46> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 47> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 48> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 49> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 50> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 51> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 52> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 53> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 54> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 55> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 56> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<50, 57> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<51, 58> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<52, 59> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<53, 60> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<54, 61> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<55, 62> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<56, 63> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsConcat<57, 64> for Reg64Bits<7> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 9> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 10> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 11> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 12> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 13> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 14> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 15> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 16> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 17> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 18> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 19> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 20> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 21> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 22> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 23> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 24> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 25> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 26> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 27> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 28> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 29> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 30> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 31> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 32> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 33> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 34> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 35> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 36> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 37> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 38> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 39> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 40> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 41> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 42> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 43> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 44> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 45> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 46> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 47> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 48> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 49> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 50> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 51> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 52> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 53> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 54> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 55> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 56> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 57> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<50, 58> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<51, 59> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<52, 60> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<53, 61> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<54, 62> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<55, 63> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsConcat<56, 64> for Reg64Bits<8> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 10> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 11> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 12> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 13> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 14> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 15> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 16> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 17> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 18> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 19> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 20> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 21> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 22> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 23> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 24> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 25> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 26> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 27> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 28> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 29> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 30> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 31> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 32> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 33> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 34> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 35> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 36> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 37> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 38> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 39> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 40> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 41> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 42> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 43> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 44> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 45> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 46> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 47> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 48> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 49> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 50> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 51> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 52> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 53> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 54> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 55> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 56> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 57> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 58> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<50, 59> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<51, 60> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<52, 61> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<53, 62> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<54, 63> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsConcat<55, 64> for Reg64Bits<9> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 11> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 12> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 13> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 14> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 15> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 16> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 17> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 18> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 19> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 20> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 21> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 22> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 23> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 24> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 25> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 26> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 27> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 28> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 29> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 30> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 31> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 32> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 33> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 34> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 35> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 36> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 37> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 38> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 39> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 40> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 41> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 42> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 43> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 44> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 45> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 46> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 47> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 48> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 49> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 50> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 51> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 52> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 53> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 54> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 55> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 56> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 57> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 58> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 59> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<50, 60> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<51, 61> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<52, 62> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<53, 63> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsConcat<54, 64> for Reg64Bits<10> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 12> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 13> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 14> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 15> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 16> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 17> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 18> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 19> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 20> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 21> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 22> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 23> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 24> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 25> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 26> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 27> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 28> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 29> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 30> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 31> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 32> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 33> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 34> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 35> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 36> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 37> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 38> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 39> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 40> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 41> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 42> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 43> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 44> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 45> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 46> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 47> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 48> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 49> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 50> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 51> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 52> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 53> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 54> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 55> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 56> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 57> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 58> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 59> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 60> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<50, 61> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<51, 62> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<52, 63> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsConcat<53, 64> for Reg64Bits<11> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 13> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 14> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 15> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 16> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 17> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 18> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 19> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 20> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 21> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 22> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 23> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 24> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 25> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 26> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 27> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 28> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 29> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 30> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 31> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 32> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 33> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 34> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 35> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 36> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 37> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 38> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 39> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 40> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 41> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 42> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 43> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 44> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 45> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 46> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 47> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 48> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 49> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 50> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 51> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 52> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 53> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 54> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 55> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 56> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 57> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 58> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 59> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 60> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 61> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<50, 62> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<51, 63> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsConcat<52, 64> for Reg64Bits<12> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 14> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 15> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 16> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 17> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 18> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 19> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 20> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 21> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 22> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 23> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 24> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 25> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 26> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 27> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 28> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 29> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 30> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 31> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 32> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 33> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 34> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 35> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 36> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 37> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 38> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 39> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 40> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 41> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 42> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 43> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 44> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 45> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 46> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 47> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 48> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 49> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 50> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 51> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 52> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 53> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 54> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 55> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 56> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 57> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 58> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 59> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 60> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 61> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 62> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<50, 63> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsConcat<51, 64> for Reg64Bits<13> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 15> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 16> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 17> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 18> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 19> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 20> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 21> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 22> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 23> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 24> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 25> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 26> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 27> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 28> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 29> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 30> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 31> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 32> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 33> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 34> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 35> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 36> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 37> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 38> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 39> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 40> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 41> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 42> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 43> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 44> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 45> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 46> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 47> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 48> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 49> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 50> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 51> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 52> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 53> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 54> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 55> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 56> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 57> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 58> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 59> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 60> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 61> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 62> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 63> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsConcat<50, 64> for Reg64Bits<14> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 16> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 17> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 18> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 19> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 20> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 21> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 22> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 23> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 24> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 25> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 26> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 27> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 28> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 29> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 30> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 31> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 32> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 33> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 34> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 35> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 36> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 37> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 38> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 39> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 40> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 41> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 42> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 43> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 44> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 45> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 46> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 47> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 48> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 49> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 50> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 51> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 52> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 53> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 54> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 55> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 56> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 57> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 58> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 59> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 60> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 61> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 62> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 63> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsConcat<49, 64> for Reg64Bits<15> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 17> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 18> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 19> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 20> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 21> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 22> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 23> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 24> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 25> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 26> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 27> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 28> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 29> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 30> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 31> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 32> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 33> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 34> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 35> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 36> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 37> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 38> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 39> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 40> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 41> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 42> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 43> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 44> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 45> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 46> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 47> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 48> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 49> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 50> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 51> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 52> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 53> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 54> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 55> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 56> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 57> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 58> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 59> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 60> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 61> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 62> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 63> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsConcat<48, 64> for Reg64Bits<16> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 18> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 19> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 20> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 21> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 22> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 23> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 24> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 25> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 26> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 27> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 28> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 29> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 30> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 31> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 32> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 33> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 34> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 35> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 36> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 37> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 38> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 39> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 40> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 41> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 42> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 43> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 44> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 45> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 46> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 47> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 48> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 49> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 50> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 51> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 52> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 53> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 54> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 55> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 56> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 57> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 58> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 59> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 60> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 61> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 62> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 63> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsConcat<47, 64> for Reg64Bits<17> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 19> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 20> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 21> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 22> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 23> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 24> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 25> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 26> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 27> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 28> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 29> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 30> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 31> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 32> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 33> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 34> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 35> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 36> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 37> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 38> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 39> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 40> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 41> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 42> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 43> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 44> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 45> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 46> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 47> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 48> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 49> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 50> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 51> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 52> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 53> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 54> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 55> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 56> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 57> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 58> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 59> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 60> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 61> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 62> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 63> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsConcat<46, 64> for Reg64Bits<18> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 20> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 21> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 22> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 23> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 24> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 25> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 26> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 27> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 28> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 29> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 30> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 31> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 32> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 33> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 34> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 35> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 36> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 37> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 38> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 39> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 40> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 41> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 42> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 43> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 44> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 45> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 46> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 47> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 48> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 49> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 50> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 51> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 52> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 53> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 54> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 55> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 56> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 57> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 58> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 59> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 60> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 61> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 62> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 63> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsConcat<45, 64> for Reg64Bits<19> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 21> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 22> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 23> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 24> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 25> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 26> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 27> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 28> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 29> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 30> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 31> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 32> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 33> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 34> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 35> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 36> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 37> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 38> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 39> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 40> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 41> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 42> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 43> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 44> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 45> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 46> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 47> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 48> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 49> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 50> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 51> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 52> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 53> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 54> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 55> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 56> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 57> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 58> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 59> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 60> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 61> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 62> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 63> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsConcat<44, 64> for Reg64Bits<20> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 22> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 23> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 24> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 25> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 26> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 27> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 28> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 29> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 30> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 31> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 32> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 33> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 34> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 35> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 36> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 37> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 38> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 39> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 40> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 41> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 42> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 43> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 44> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 45> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 46> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 47> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 48> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 49> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 50> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 51> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 52> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 53> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 54> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 55> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 56> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 57> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 58> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 59> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 60> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 61> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 62> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 63> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsConcat<43, 64> for Reg64Bits<21> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 23> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 24> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 25> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 26> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 27> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 28> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 29> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 30> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 31> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 32> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 33> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 34> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 35> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 36> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 37> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 38> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 39> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 40> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 41> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 42> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 43> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 44> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 45> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 46> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 47> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 48> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 49> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 50> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 51> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 52> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 53> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 54> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 55> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 56> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 57> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 58> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 59> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 60> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 61> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 62> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 63> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsConcat<42, 64> for Reg64Bits<22> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 24> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 25> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 26> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 27> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 28> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 29> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 30> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 31> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 32> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 33> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 34> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 35> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 36> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 37> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 38> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 39> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 40> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 41> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 42> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 43> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 44> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 45> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 46> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 47> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 48> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 49> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 50> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 51> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 52> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 53> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 54> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 55> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 56> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 57> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 58> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 59> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 60> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 61> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 62> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 63> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsConcat<41, 64> for Reg64Bits<23> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 25> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 26> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 27> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 28> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 29> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 30> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 31> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 32> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 33> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 34> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 35> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 36> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 37> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 38> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 39> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 40> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 41> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 42> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 43> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 44> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 45> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 46> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 47> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 48> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 49> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 50> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 51> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 52> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 53> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 54> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 55> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 56> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 57> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 58> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 59> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 60> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 61> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 62> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 63> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsConcat<40, 64> for Reg64Bits<24> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 26> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 27> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 28> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 29> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 30> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 31> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 32> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 33> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 34> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 35> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 36> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 37> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 38> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 39> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 40> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 41> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 42> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 43> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 44> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 45> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 46> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 47> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 48> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 49> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 50> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 51> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 52> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 53> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 54> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 55> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 56> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 57> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 58> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 59> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 60> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 61> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 62> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 63> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsConcat<39, 64> for Reg64Bits<25> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 27> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 28> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 29> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 30> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 31> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 32> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 33> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 34> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 35> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 36> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 37> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 38> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 39> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 40> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 41> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 42> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 43> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 44> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 45> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 46> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 47> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 48> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 49> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 50> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 51> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 52> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 53> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 54> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 55> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 56> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 57> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 58> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 59> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 60> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 61> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 62> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 63> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsConcat<38, 64> for Reg64Bits<26> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 28> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 29> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 30> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 31> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 32> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 33> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 34> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 35> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 36> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 37> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 38> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 39> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 40> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 41> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 42> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 43> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 44> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 45> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 46> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 47> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 48> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 49> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 50> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 51> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 52> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 53> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 54> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 55> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 56> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 57> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 58> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 59> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 60> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 61> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 62> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 63> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsConcat<37, 64> for Reg64Bits<27> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 29> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 30> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 31> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 32> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 33> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 34> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 35> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 36> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 37> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 38> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 39> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 40> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 41> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 42> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 43> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 44> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 45> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 46> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 47> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 48> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 49> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 50> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 51> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 52> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 53> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 54> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 55> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 56> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 57> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 58> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 59> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 60> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 61> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 62> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 63> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsConcat<36, 64> for Reg64Bits<28> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 30> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 31> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 32> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 33> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 34> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 35> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 36> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 37> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 38> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 39> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 40> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 41> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 42> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 43> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 44> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 45> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 46> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 47> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 48> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 49> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 50> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 51> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 52> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 53> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 54> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 55> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 56> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 57> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 58> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 59> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 60> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 61> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 62> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 63> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsConcat<35, 64> for Reg64Bits<29> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 31> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 32> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 33> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 34> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 35> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 36> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 37> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 38> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 39> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 40> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 41> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 42> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 43> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 44> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 45> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 46> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 47> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 48> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 49> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 50> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 51> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 52> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 53> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 54> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 55> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 56> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 57> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 58> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 59> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 60> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 61> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 62> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 63> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsConcat<34, 64> for Reg64Bits<30> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 32> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 33> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 34> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 35> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 36> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 37> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 38> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 39> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 40> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 41> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 42> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 43> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 44> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 45> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 46> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 47> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 48> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 49> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 50> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 51> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 52> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 53> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 54> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 55> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 56> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 57> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 58> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 59> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 60> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 61> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 62> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 63> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsConcat<33, 64> for Reg64Bits<31> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 33> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 34> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 35> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 36> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 37> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 38> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 39> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 40> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 41> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 42> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 43> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 44> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 45> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 46> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 47> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 48> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 49> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 50> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 51> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 52> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 53> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 54> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 55> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 56> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 57> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 58> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 59> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 60> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 61> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 62> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 63> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsConcat<32, 64> for Reg64Bits<32> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 34> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 35> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 36> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 37> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 38> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 39> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 40> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 41> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 42> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 43> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 44> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 45> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 46> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 47> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 48> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 49> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 50> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 51> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 52> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 53> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 54> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 55> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 56> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 57> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 58> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 59> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 60> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 61> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 62> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 63> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsConcat<31, 64> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<33> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 35> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 36> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 37> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 38> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 39> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 40> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 41> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 42> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 43> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 44> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 45> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 46> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 47> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 48> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 49> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 50> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 51> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 52> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 53> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 54> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 55> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 56> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 57> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 58> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 59> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 60> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 61> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 62> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 63> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsConcat<30, 64> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<34> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 36> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 37> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 38> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 39> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 40> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 41> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 42> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 43> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 44> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 45> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 46> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 47> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 48> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 49> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 50> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 51> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 52> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 53> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 54> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 55> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 56> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 57> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 58> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 59> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 60> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 61> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 62> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 63> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsConcat<29, 64> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<35> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 37> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 38> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 39> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 40> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 41> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 42> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 43> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 44> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 45> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 46> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 47> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 48> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 49> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 50> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 51> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 52> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 53> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 54> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 55> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 56> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 57> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 58> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 59> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 60> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 61> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 62> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 63> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsConcat<28, 64> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<36> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 38> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 39> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 40> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 41> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 42> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 43> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 44> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 45> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 46> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 47> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 48> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 49> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 50> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 51> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 52> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 53> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 54> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 55> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 56> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 57> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 58> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 59> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 60> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 61> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 62> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 63> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsConcat<27, 64> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<37> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 39> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 40> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 41> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 42> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 43> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 44> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 45> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 46> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 47> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 48> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 49> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 50> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 51> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 52> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 53> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 54> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 55> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 56> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 57> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 58> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 59> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 60> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 61> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 62> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 63> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsConcat<26, 64> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<38> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 40> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 41> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 42> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 43> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 44> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 45> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 46> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 47> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 48> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 49> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 50> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 51> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 52> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 53> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 54> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 55> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 56> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 57> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 58> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 59> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 60> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 61> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 62> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 63> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsConcat<25, 64> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<39> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 41> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 42> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 43> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 44> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 45> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 46> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 47> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 48> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 49> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 50> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 51> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 52> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 53> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 54> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 55> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 56> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 57> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 58> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 59> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 60> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 61> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 62> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 63> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsConcat<24, 64> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<40> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 42> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 43> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 44> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 45> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 46> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 47> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 48> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 49> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 50> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 51> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 52> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 53> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 54> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 55> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 56> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 57> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 58> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 59> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 60> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 61> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 62> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 63> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsConcat<23, 64> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<41> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 43> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 44> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 45> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 46> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 47> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 48> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 49> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 50> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 51> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 52> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 53> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 54> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 55> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 56> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 57> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 58> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 59> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 60> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 61> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 62> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 63> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsConcat<22, 64> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<42> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 44> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 45> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 46> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 47> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 48> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 49> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 50> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 51> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 52> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 53> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 54> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 55> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 56> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 57> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 58> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 59> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 60> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 61> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 62> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 63> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsConcat<21, 64> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<43> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 45> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 46> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 47> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 48> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 49> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 50> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 51> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 52> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 53> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 54> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 55> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 56> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 57> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 58> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 59> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 60> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 61> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 62> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 63> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsConcat<20, 64> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<44> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 46> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 47> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 48> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 49> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 50> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 51> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 52> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 53> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 54> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 55> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 56> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 57> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 58> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 59> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 60> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 61> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 62> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 63> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsConcat<19, 64> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<45> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 47> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 48> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 49> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 50> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 51> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 52> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 53> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 54> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 55> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 56> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 57> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 58> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 59> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 60> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 61> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 62> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 63> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsConcat<18, 64> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<46> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 48> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 49> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 50> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 51> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 52> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 53> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 54> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 55> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 56> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 57> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 58> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 59> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 60> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 61> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 62> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 63> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsConcat<17, 64> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<47> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 49> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 50> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 51> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 52> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 53> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 54> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 55> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 56> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 57> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 58> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 59> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 60> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 61> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 62> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 63> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsConcat<16, 64> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<48> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 50> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 51> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 52> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 53> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 54> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 55> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 56> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 57> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 58> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 59> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 60> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 61> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 62> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 63> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsConcat<15, 64> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<49> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 51> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 52> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 53> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 54> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 55> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 56> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 57> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 58> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 59> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 60> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 61> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 62> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 63> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsConcat<14, 64> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<50> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 52> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 53> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 54> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 55> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 56> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 57> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 58> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 59> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 60> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 61> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 62> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 63> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsConcat<13, 64> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<51> for Reg64Bits<51> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 53> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 54> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 55> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 56> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 57> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 58> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 59> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 60> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 61> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 62> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 63> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsConcat<12, 64> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<51> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<52> for Reg64Bits<52> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 54> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 55> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 56> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 57> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 58> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 59> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 60> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 61> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 62> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 63> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsConcat<11, 64> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<51> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<52> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<53> for Reg64Bits<53> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 55> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 56> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 57> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 58> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 59> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 60> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 61> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 62> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 63> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsConcat<10, 64> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<51> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<52> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<53> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<54> for Reg64Bits<54> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 56> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 57> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 58> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 59> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 60> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 61> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 62> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 63> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsConcat<9, 64> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<51> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<52> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<53> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<54> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<55> for Reg64Bits<55> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 57> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 58> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 59> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 60> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 61> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 62> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 63> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsConcat<8, 64> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<51> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<52> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<53> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<54> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<55> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<56> for Reg64Bits<56> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 58> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 59> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 60> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 61> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 62> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 63> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsConcat<7, 64> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<51> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<52> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<53> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<54> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<55> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<56> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<57> for Reg64Bits<57> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 59> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 60> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 61> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 62> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 63> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsConcat<6, 64> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<51> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<52> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<53> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<54> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<55> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<56> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<57> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<58> for Reg64Bits<58> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 60> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 61> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 62> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 63> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsConcat<5, 64> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<51> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<52> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<53> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<54> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<55> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<56> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<57> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<58> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<59> for Reg64Bits<59> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 61> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 62> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 63> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsConcat<4, 64> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<51> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<52> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<53> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<54> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<55> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<56> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<57> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<58> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<59> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<60> for Reg64Bits<60> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 62> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 63> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsConcat<3, 64> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<51> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<52> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<53> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<54> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<55> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<56> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<57> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<58> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<59> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<60> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<61> for Reg64Bits<61> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 63> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsConcat<2, 64> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<51> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<52> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<53> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<54> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<55> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<56> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<57> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<58> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<59> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<60> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<61> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<62> for Reg64Bits<62> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsConcat<1, 64> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<51> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<52> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<53> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<54> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<55> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<56> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<57> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<58> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<59> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<60> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<61> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<62> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<63> for Reg64Bits<63> {}
#[doc(hidden)]
impl Reg64BitsDownCast<1> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<2> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<3> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<4> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<5> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<6> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<7> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<8> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<9> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<10> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<11> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<12> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<13> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<14> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<15> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<16> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<17> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<18> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<19> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<20> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<21> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<22> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<23> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<24> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<25> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<26> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<27> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<28> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<29> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<30> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<31> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<32> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<33> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<34> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<35> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<36> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<37> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<38> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<39> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<40> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<41> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<42> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<43> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<44> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<45> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<46> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<47> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<48> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<49> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<50> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<51> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<52> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<53> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<54> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<55> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<56> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<57> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<58> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<59> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<60> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<61> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<62> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<63> for Reg64Bits<64> {}
#[doc(hidden)]
impl Reg64BitsDownCast<64> for Reg64Bits<64> {}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<2> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<3> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<4> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<5> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<6> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<7> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<8> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<9> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<10> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<11> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<12> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<13> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<14> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<15> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<16> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<17> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<18> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<19> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<20> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<21> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<22> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<23> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<24> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<25> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<26> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<27> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<28> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<29> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<30> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<31> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<32> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<33> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<34> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<35> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<36> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<37> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<38> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<39> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<40> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<41> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<42> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<43> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<44> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<45> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<46> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<47> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<48> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<49> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<50> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<51> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<52> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<53> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<54> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<55> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<56> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<57> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<58> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<59> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<60> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<61> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<62> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<63> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
impl PartialEq<BaseType> for Reg64Bits<64> {
    fn eq(&self, other: &BaseType) -> bool {
        self.0 == *other
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<1>> for crate::reg64::Reg64Bits<1> {
    fn from(item: crate::reg8::Reg8Bits<1>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<1>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<1>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<1>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<1>> for crate::reg64::Reg64Bits<1> {
    fn from(item: crate::reg16::Reg16Bits<1>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<1>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<1>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<1>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<1>> for crate::reg64::Reg64Bits<1> {
    fn from(item: crate::reg32::Reg32Bits<1>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<1>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<1>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<1>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<2>> for crate::reg64::Reg64Bits<2> {
    fn from(item: crate::reg8::Reg8Bits<2>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<2>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<2>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<2>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<2>> for crate::reg64::Reg64Bits<2> {
    fn from(item: crate::reg16::Reg16Bits<2>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<2>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<2>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<2>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<2>> for crate::reg64::Reg64Bits<2> {
    fn from(item: crate::reg32::Reg32Bits<2>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<2>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<2>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<2>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<3>> for crate::reg64::Reg64Bits<3> {
    fn from(item: crate::reg8::Reg8Bits<3>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<3>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<3>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<3>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<3>> for crate::reg64::Reg64Bits<3> {
    fn from(item: crate::reg16::Reg16Bits<3>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<3>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<3>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<3>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<3>> for crate::reg64::Reg64Bits<3> {
    fn from(item: crate::reg32::Reg32Bits<3>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<3>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<3>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<3>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<4>> for crate::reg64::Reg64Bits<4> {
    fn from(item: crate::reg8::Reg8Bits<4>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<4>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<4>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<4>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<4>> for crate::reg64::Reg64Bits<4> {
    fn from(item: crate::reg16::Reg16Bits<4>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<4>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<4>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<4>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<4>> for crate::reg64::Reg64Bits<4> {
    fn from(item: crate::reg32::Reg32Bits<4>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<4>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<4>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<4>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<5>> for crate::reg64::Reg64Bits<5> {
    fn from(item: crate::reg8::Reg8Bits<5>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<5>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<5>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<5>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<5>> for crate::reg64::Reg64Bits<5> {
    fn from(item: crate::reg16::Reg16Bits<5>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<5>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<5>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<5>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<5>> for crate::reg64::Reg64Bits<5> {
    fn from(item: crate::reg32::Reg32Bits<5>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<5>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<5>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<5>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<6>> for crate::reg64::Reg64Bits<6> {
    fn from(item: crate::reg8::Reg8Bits<6>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<6>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<6>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<6>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<6>> for crate::reg64::Reg64Bits<6> {
    fn from(item: crate::reg16::Reg16Bits<6>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<6>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<6>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<6>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<6>> for crate::reg64::Reg64Bits<6> {
    fn from(item: crate::reg32::Reg32Bits<6>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<6>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<6>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<6>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<7>> for crate::reg64::Reg64Bits<7> {
    fn from(item: crate::reg8::Reg8Bits<7>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<7>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<7>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<7>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<7>> for crate::reg64::Reg64Bits<7> {
    fn from(item: crate::reg16::Reg16Bits<7>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<7>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<7>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<7>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<7>> for crate::reg64::Reg64Bits<7> {
    fn from(item: crate::reg32::Reg32Bits<7>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<7>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<7>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<7>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<8>> for crate::reg64::Reg64Bits<8> {
    fn from(item: crate::reg8::Reg8Bits<8>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<8>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<8>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<8>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<8>> for crate::reg64::Reg64Bits<8> {
    fn from(item: crate::reg16::Reg16Bits<8>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<8>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<8>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<8>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<8>> for crate::reg64::Reg64Bits<8> {
    fn from(item: crate::reg32::Reg32Bits<8>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<8>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<8>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<8>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<9>> for crate::reg64::Reg64Bits<9> {
    fn from(item: crate::reg8::Reg8Bits<9>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<9>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<9>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<9>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<9>> for crate::reg64::Reg64Bits<9> {
    fn from(item: crate::reg16::Reg16Bits<9>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<9>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<9>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<9>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<9>> for crate::reg64::Reg64Bits<9> {
    fn from(item: crate::reg32::Reg32Bits<9>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<9>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<9>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<9>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<10>> for crate::reg64::Reg64Bits<10> {
    fn from(item: crate::reg8::Reg8Bits<10>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<10>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<10>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<10>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<10>> for crate::reg64::Reg64Bits<10> {
    fn from(item: crate::reg16::Reg16Bits<10>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<10>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<10>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<10>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<10>> for crate::reg64::Reg64Bits<10> {
    fn from(item: crate::reg32::Reg32Bits<10>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<10>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<10>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<10>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<11>> for crate::reg64::Reg64Bits<11> {
    fn from(item: crate::reg8::Reg8Bits<11>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<11>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<11>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<11>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<11>> for crate::reg64::Reg64Bits<11> {
    fn from(item: crate::reg16::Reg16Bits<11>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<11>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<11>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<11>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<11>> for crate::reg64::Reg64Bits<11> {
    fn from(item: crate::reg32::Reg32Bits<11>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<11>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<11>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<11>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<12>> for crate::reg64::Reg64Bits<12> {
    fn from(item: crate::reg8::Reg8Bits<12>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<12>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<12>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<12>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<12>> for crate::reg64::Reg64Bits<12> {
    fn from(item: crate::reg16::Reg16Bits<12>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<12>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<12>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<12>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<12>> for crate::reg64::Reg64Bits<12> {
    fn from(item: crate::reg32::Reg32Bits<12>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<12>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<12>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<12>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<13>> for crate::reg64::Reg64Bits<13> {
    fn from(item: crate::reg8::Reg8Bits<13>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<13>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<13>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<13>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<13>> for crate::reg64::Reg64Bits<13> {
    fn from(item: crate::reg16::Reg16Bits<13>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<13>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<13>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<13>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<13>> for crate::reg64::Reg64Bits<13> {
    fn from(item: crate::reg32::Reg32Bits<13>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<13>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<13>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<13>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<14>> for crate::reg64::Reg64Bits<14> {
    fn from(item: crate::reg8::Reg8Bits<14>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<14>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<14>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<14>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<14>> for crate::reg64::Reg64Bits<14> {
    fn from(item: crate::reg16::Reg16Bits<14>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<14>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<14>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<14>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<14>> for crate::reg64::Reg64Bits<14> {
    fn from(item: crate::reg32::Reg32Bits<14>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<14>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<14>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<14>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<15>> for crate::reg64::Reg64Bits<15> {
    fn from(item: crate::reg8::Reg8Bits<15>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<15>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<15>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<15>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<15>> for crate::reg64::Reg64Bits<15> {
    fn from(item: crate::reg16::Reg16Bits<15>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<15>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<15>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<15>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<15>> for crate::reg64::Reg64Bits<15> {
    fn from(item: crate::reg32::Reg32Bits<15>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<15>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<15>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<15>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<16>> for crate::reg64::Reg64Bits<16> {
    fn from(item: crate::reg8::Reg8Bits<16>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<16>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<16>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<16>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<16>> for crate::reg64::Reg64Bits<16> {
    fn from(item: crate::reg16::Reg16Bits<16>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<16>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<16>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<16>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<16>> for crate::reg64::Reg64Bits<16> {
    fn from(item: crate::reg32::Reg32Bits<16>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<16>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<16>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<16>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<17>> for crate::reg64::Reg64Bits<17> {
    fn from(item: crate::reg8::Reg8Bits<17>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<17>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<17>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<17>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<17>> for crate::reg64::Reg64Bits<17> {
    fn from(item: crate::reg16::Reg16Bits<17>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<17>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<17>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<17>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<17>> for crate::reg64::Reg64Bits<17> {
    fn from(item: crate::reg32::Reg32Bits<17>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<17>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<17>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<17>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<18>> for crate::reg64::Reg64Bits<18> {
    fn from(item: crate::reg8::Reg8Bits<18>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<18>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<18>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<18>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<18>> for crate::reg64::Reg64Bits<18> {
    fn from(item: crate::reg16::Reg16Bits<18>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<18>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<18>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<18>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<18>> for crate::reg64::Reg64Bits<18> {
    fn from(item: crate::reg32::Reg32Bits<18>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<18>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<18>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<18>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<19>> for crate::reg64::Reg64Bits<19> {
    fn from(item: crate::reg8::Reg8Bits<19>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<19>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<19>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<19>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<19>> for crate::reg64::Reg64Bits<19> {
    fn from(item: crate::reg16::Reg16Bits<19>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<19>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<19>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<19>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<19>> for crate::reg64::Reg64Bits<19> {
    fn from(item: crate::reg32::Reg32Bits<19>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<19>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<19>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<19>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<20>> for crate::reg64::Reg64Bits<20> {
    fn from(item: crate::reg8::Reg8Bits<20>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<20>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<20>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<20>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<20>> for crate::reg64::Reg64Bits<20> {
    fn from(item: crate::reg16::Reg16Bits<20>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<20>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<20>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<20>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<20>> for crate::reg64::Reg64Bits<20> {
    fn from(item: crate::reg32::Reg32Bits<20>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<20>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<20>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<20>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<21>> for crate::reg64::Reg64Bits<21> {
    fn from(item: crate::reg8::Reg8Bits<21>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<21>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<21>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<21>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<21>> for crate::reg64::Reg64Bits<21> {
    fn from(item: crate::reg16::Reg16Bits<21>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<21>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<21>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<21>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<21>> for crate::reg64::Reg64Bits<21> {
    fn from(item: crate::reg32::Reg32Bits<21>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<21>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<21>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<21>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<22>> for crate::reg64::Reg64Bits<22> {
    fn from(item: crate::reg8::Reg8Bits<22>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<22>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<22>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<22>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<22>> for crate::reg64::Reg64Bits<22> {
    fn from(item: crate::reg16::Reg16Bits<22>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<22>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<22>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<22>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<22>> for crate::reg64::Reg64Bits<22> {
    fn from(item: crate::reg32::Reg32Bits<22>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<22>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<22>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<22>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<23>> for crate::reg64::Reg64Bits<23> {
    fn from(item: crate::reg8::Reg8Bits<23>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<23>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<23>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<23>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<23>> for crate::reg64::Reg64Bits<23> {
    fn from(item: crate::reg16::Reg16Bits<23>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<23>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<23>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<23>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<23>> for crate::reg64::Reg64Bits<23> {
    fn from(item: crate::reg32::Reg32Bits<23>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<23>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<23>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<23>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<24>> for crate::reg64::Reg64Bits<24> {
    fn from(item: crate::reg8::Reg8Bits<24>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<24>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<24>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<24>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<24>> for crate::reg64::Reg64Bits<24> {
    fn from(item: crate::reg16::Reg16Bits<24>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<24>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<24>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<24>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<24>> for crate::reg64::Reg64Bits<24> {
    fn from(item: crate::reg32::Reg32Bits<24>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<24>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<24>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<24>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<25>> for crate::reg64::Reg64Bits<25> {
    fn from(item: crate::reg8::Reg8Bits<25>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<25>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<25>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<25>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<25>> for crate::reg64::Reg64Bits<25> {
    fn from(item: crate::reg16::Reg16Bits<25>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<25>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<25>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<25>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<25>> for crate::reg64::Reg64Bits<25> {
    fn from(item: crate::reg32::Reg32Bits<25>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<25>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<25>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<25>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<26>> for crate::reg64::Reg64Bits<26> {
    fn from(item: crate::reg8::Reg8Bits<26>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<26>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<26>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<26>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<26>> for crate::reg64::Reg64Bits<26> {
    fn from(item: crate::reg16::Reg16Bits<26>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<26>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<26>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<26>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<26>> for crate::reg64::Reg64Bits<26> {
    fn from(item: crate::reg32::Reg32Bits<26>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<26>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<26>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<26>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<27>> for crate::reg64::Reg64Bits<27> {
    fn from(item: crate::reg8::Reg8Bits<27>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<27>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<27>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<27>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<27>> for crate::reg64::Reg64Bits<27> {
    fn from(item: crate::reg16::Reg16Bits<27>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<27>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<27>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<27>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<27>> for crate::reg64::Reg64Bits<27> {
    fn from(item: crate::reg32::Reg32Bits<27>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<27>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<27>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<27>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<28>> for crate::reg64::Reg64Bits<28> {
    fn from(item: crate::reg8::Reg8Bits<28>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<28>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<28>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<28>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<28>> for crate::reg64::Reg64Bits<28> {
    fn from(item: crate::reg16::Reg16Bits<28>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<28>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<28>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<28>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<28>> for crate::reg64::Reg64Bits<28> {
    fn from(item: crate::reg32::Reg32Bits<28>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<28>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<28>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<28>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<29>> for crate::reg64::Reg64Bits<29> {
    fn from(item: crate::reg8::Reg8Bits<29>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<29>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<29>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<29>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<29>> for crate::reg64::Reg64Bits<29> {
    fn from(item: crate::reg16::Reg16Bits<29>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<29>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<29>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<29>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<29>> for crate::reg64::Reg64Bits<29> {
    fn from(item: crate::reg32::Reg32Bits<29>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<29>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<29>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<29>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<30>> for crate::reg64::Reg64Bits<30> {
    fn from(item: crate::reg8::Reg8Bits<30>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<30>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<30>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<30>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<30>> for crate::reg64::Reg64Bits<30> {
    fn from(item: crate::reg16::Reg16Bits<30>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<30>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<30>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<30>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<30>> for crate::reg64::Reg64Bits<30> {
    fn from(item: crate::reg32::Reg32Bits<30>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<30>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<30>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<30>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<31>> for crate::reg64::Reg64Bits<31> {
    fn from(item: crate::reg8::Reg8Bits<31>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<31>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<31>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<31>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<31>> for crate::reg64::Reg64Bits<31> {
    fn from(item: crate::reg16::Reg16Bits<31>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<31>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<31>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<31>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<31>> for crate::reg64::Reg64Bits<31> {
    fn from(item: crate::reg32::Reg32Bits<31>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<31>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<31>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<31>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<32>> for crate::reg64::Reg64Bits<32> {
    fn from(item: crate::reg8::Reg8Bits<32>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<32>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<32>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<32>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<32>> for crate::reg64::Reg64Bits<32> {
    fn from(item: crate::reg16::Reg16Bits<32>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<32>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<32>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<32>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<32>> for crate::reg64::Reg64Bits<32> {
    fn from(item: crate::reg32::Reg32Bits<32>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<32>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<32>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<32>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<33>> for crate::reg64::Reg64Bits<33> {
    fn from(item: crate::reg8::Reg8Bits<33>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<33>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<33>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<33>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<33>> for crate::reg64::Reg64Bits<33> {
    fn from(item: crate::reg16::Reg16Bits<33>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<33>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<33>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<33>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<33>> for crate::reg64::Reg64Bits<33> {
    fn from(item: crate::reg32::Reg32Bits<33>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<33>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<33>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<33>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<34>> for crate::reg64::Reg64Bits<34> {
    fn from(item: crate::reg8::Reg8Bits<34>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<34>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<34>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<34>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<34>> for crate::reg64::Reg64Bits<34> {
    fn from(item: crate::reg16::Reg16Bits<34>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<34>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<34>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<34>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<34>> for crate::reg64::Reg64Bits<34> {
    fn from(item: crate::reg32::Reg32Bits<34>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<34>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<34>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<34>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<35>> for crate::reg64::Reg64Bits<35> {
    fn from(item: crate::reg8::Reg8Bits<35>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<35>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<35>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<35>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<35>> for crate::reg64::Reg64Bits<35> {
    fn from(item: crate::reg16::Reg16Bits<35>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<35>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<35>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<35>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<35>> for crate::reg64::Reg64Bits<35> {
    fn from(item: crate::reg32::Reg32Bits<35>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<35>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<35>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<35>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<36>> for crate::reg64::Reg64Bits<36> {
    fn from(item: crate::reg8::Reg8Bits<36>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<36>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<36>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<36>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<36>> for crate::reg64::Reg64Bits<36> {
    fn from(item: crate::reg16::Reg16Bits<36>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<36>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<36>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<36>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<36>> for crate::reg64::Reg64Bits<36> {
    fn from(item: crate::reg32::Reg32Bits<36>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<36>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<36>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<36>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<37>> for crate::reg64::Reg64Bits<37> {
    fn from(item: crate::reg8::Reg8Bits<37>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<37>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<37>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<37>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<37>> for crate::reg64::Reg64Bits<37> {
    fn from(item: crate::reg16::Reg16Bits<37>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<37>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<37>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<37>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<37>> for crate::reg64::Reg64Bits<37> {
    fn from(item: crate::reg32::Reg32Bits<37>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<37>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<37>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<37>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<38>> for crate::reg64::Reg64Bits<38> {
    fn from(item: crate::reg8::Reg8Bits<38>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<38>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<38>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<38>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<38>> for crate::reg64::Reg64Bits<38> {
    fn from(item: crate::reg16::Reg16Bits<38>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<38>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<38>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<38>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<38>> for crate::reg64::Reg64Bits<38> {
    fn from(item: crate::reg32::Reg32Bits<38>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<38>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<38>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<38>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<39>> for crate::reg64::Reg64Bits<39> {
    fn from(item: crate::reg8::Reg8Bits<39>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<39>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<39>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<39>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<39>> for crate::reg64::Reg64Bits<39> {
    fn from(item: crate::reg16::Reg16Bits<39>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<39>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<39>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<39>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<39>> for crate::reg64::Reg64Bits<39> {
    fn from(item: crate::reg32::Reg32Bits<39>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<39>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<39>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<39>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<40>> for crate::reg64::Reg64Bits<40> {
    fn from(item: crate::reg8::Reg8Bits<40>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<40>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<40>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<40>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<40>> for crate::reg64::Reg64Bits<40> {
    fn from(item: crate::reg16::Reg16Bits<40>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<40>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<40>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<40>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<40>> for crate::reg64::Reg64Bits<40> {
    fn from(item: crate::reg32::Reg32Bits<40>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<40>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<40>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<40>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<41>> for crate::reg64::Reg64Bits<41> {
    fn from(item: crate::reg8::Reg8Bits<41>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<41>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<41>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<41>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<41>> for crate::reg64::Reg64Bits<41> {
    fn from(item: crate::reg16::Reg16Bits<41>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<41>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<41>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<41>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<41>> for crate::reg64::Reg64Bits<41> {
    fn from(item: crate::reg32::Reg32Bits<41>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<41>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<41>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<41>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<42>> for crate::reg64::Reg64Bits<42> {
    fn from(item: crate::reg8::Reg8Bits<42>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<42>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<42>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<42>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<42>> for crate::reg64::Reg64Bits<42> {
    fn from(item: crate::reg16::Reg16Bits<42>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<42>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<42>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<42>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<42>> for crate::reg64::Reg64Bits<42> {
    fn from(item: crate::reg32::Reg32Bits<42>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<42>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<42>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<42>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<43>> for crate::reg64::Reg64Bits<43> {
    fn from(item: crate::reg8::Reg8Bits<43>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<43>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<43>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<43>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<43>> for crate::reg64::Reg64Bits<43> {
    fn from(item: crate::reg16::Reg16Bits<43>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<43>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<43>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<43>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<43>> for crate::reg64::Reg64Bits<43> {
    fn from(item: crate::reg32::Reg32Bits<43>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<43>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<43>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<43>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<44>> for crate::reg64::Reg64Bits<44> {
    fn from(item: crate::reg8::Reg8Bits<44>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<44>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<44>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<44>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<44>> for crate::reg64::Reg64Bits<44> {
    fn from(item: crate::reg16::Reg16Bits<44>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<44>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<44>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<44>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<44>> for crate::reg64::Reg64Bits<44> {
    fn from(item: crate::reg32::Reg32Bits<44>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<44>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<44>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<44>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<45>> for crate::reg64::Reg64Bits<45> {
    fn from(item: crate::reg8::Reg8Bits<45>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<45>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<45>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<45>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<45>> for crate::reg64::Reg64Bits<45> {
    fn from(item: crate::reg16::Reg16Bits<45>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<45>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<45>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<45>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<45>> for crate::reg64::Reg64Bits<45> {
    fn from(item: crate::reg32::Reg32Bits<45>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<45>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<45>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<45>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<46>> for crate::reg64::Reg64Bits<46> {
    fn from(item: crate::reg8::Reg8Bits<46>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<46>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<46>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<46>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<46>> for crate::reg64::Reg64Bits<46> {
    fn from(item: crate::reg16::Reg16Bits<46>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<46>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<46>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<46>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<46>> for crate::reg64::Reg64Bits<46> {
    fn from(item: crate::reg32::Reg32Bits<46>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<46>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<46>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<46>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<47>> for crate::reg64::Reg64Bits<47> {
    fn from(item: crate::reg8::Reg8Bits<47>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<47>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<47>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<47>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<47>> for crate::reg64::Reg64Bits<47> {
    fn from(item: crate::reg16::Reg16Bits<47>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<47>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<47>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<47>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<47>> for crate::reg64::Reg64Bits<47> {
    fn from(item: crate::reg32::Reg32Bits<47>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<47>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<47>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<47>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<48>> for crate::reg64::Reg64Bits<48> {
    fn from(item: crate::reg8::Reg8Bits<48>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<48>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<48>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<48>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<48>> for crate::reg64::Reg64Bits<48> {
    fn from(item: crate::reg16::Reg16Bits<48>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<48>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<48>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<48>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<48>> for crate::reg64::Reg64Bits<48> {
    fn from(item: crate::reg32::Reg32Bits<48>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<48>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<48>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<48>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<49>> for crate::reg64::Reg64Bits<49> {
    fn from(item: crate::reg8::Reg8Bits<49>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<49>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<49>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<49>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<49>> for crate::reg64::Reg64Bits<49> {
    fn from(item: crate::reg16::Reg16Bits<49>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<49>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<49>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<49>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<49>> for crate::reg64::Reg64Bits<49> {
    fn from(item: crate::reg32::Reg32Bits<49>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<49>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<49>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<49>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<50>> for crate::reg64::Reg64Bits<50> {
    fn from(item: crate::reg8::Reg8Bits<50>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<50>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<50>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<50>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<50>> for crate::reg64::Reg64Bits<50> {
    fn from(item: crate::reg16::Reg16Bits<50>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<50>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<50>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<50>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<50>> for crate::reg64::Reg64Bits<50> {
    fn from(item: crate::reg32::Reg32Bits<50>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<50>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<50>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<50>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<51>> for crate::reg64::Reg64Bits<51> {
    fn from(item: crate::reg8::Reg8Bits<51>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<51>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<51>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<51>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<51>> for crate::reg64::Reg64Bits<51> {
    fn from(item: crate::reg16::Reg16Bits<51>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<51>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<51>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<51>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<51>> for crate::reg64::Reg64Bits<51> {
    fn from(item: crate::reg32::Reg32Bits<51>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<51>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<51>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<51>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<52>> for crate::reg64::Reg64Bits<52> {
    fn from(item: crate::reg8::Reg8Bits<52>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<52>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<52>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<52>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<52>> for crate::reg64::Reg64Bits<52> {
    fn from(item: crate::reg16::Reg16Bits<52>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<52>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<52>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<52>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<52>> for crate::reg64::Reg64Bits<52> {
    fn from(item: crate::reg32::Reg32Bits<52>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<52>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<52>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<52>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<53>> for crate::reg64::Reg64Bits<53> {
    fn from(item: crate::reg8::Reg8Bits<53>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<53>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<53>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<53>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<53>> for crate::reg64::Reg64Bits<53> {
    fn from(item: crate::reg16::Reg16Bits<53>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<53>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<53>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<53>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<53>> for crate::reg64::Reg64Bits<53> {
    fn from(item: crate::reg32::Reg32Bits<53>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<53>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<53>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<53>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<54>> for crate::reg64::Reg64Bits<54> {
    fn from(item: crate::reg8::Reg8Bits<54>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<54>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<54>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<54>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<54>> for crate::reg64::Reg64Bits<54> {
    fn from(item: crate::reg16::Reg16Bits<54>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<54>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<54>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<54>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<54>> for crate::reg64::Reg64Bits<54> {
    fn from(item: crate::reg32::Reg32Bits<54>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<54>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<54>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<54>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<55>> for crate::reg64::Reg64Bits<55> {
    fn from(item: crate::reg8::Reg8Bits<55>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<55>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<55>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<55>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<55>> for crate::reg64::Reg64Bits<55> {
    fn from(item: crate::reg16::Reg16Bits<55>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<55>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<55>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<55>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<55>> for crate::reg64::Reg64Bits<55> {
    fn from(item: crate::reg32::Reg32Bits<55>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<55>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<55>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<55>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<56>> for crate::reg64::Reg64Bits<56> {
    fn from(item: crate::reg8::Reg8Bits<56>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<56>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<56>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<56>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<56>> for crate::reg64::Reg64Bits<56> {
    fn from(item: crate::reg16::Reg16Bits<56>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<56>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<56>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<56>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<56>> for crate::reg64::Reg64Bits<56> {
    fn from(item: crate::reg32::Reg32Bits<56>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<56>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<56>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<56>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<57>> for crate::reg64::Reg64Bits<57> {
    fn from(item: crate::reg8::Reg8Bits<57>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<57>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<57>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<57>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<57>> for crate::reg64::Reg64Bits<57> {
    fn from(item: crate::reg16::Reg16Bits<57>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<57>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<57>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<57>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<57>> for crate::reg64::Reg64Bits<57> {
    fn from(item: crate::reg32::Reg32Bits<57>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<57>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<57>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<57>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<58>> for crate::reg64::Reg64Bits<58> {
    fn from(item: crate::reg8::Reg8Bits<58>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<58>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<58>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<58>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<58>> for crate::reg64::Reg64Bits<58> {
    fn from(item: crate::reg16::Reg16Bits<58>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<58>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<58>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<58>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<58>> for crate::reg64::Reg64Bits<58> {
    fn from(item: crate::reg32::Reg32Bits<58>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<58>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<58>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<58>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<59>> for crate::reg64::Reg64Bits<59> {
    fn from(item: crate::reg8::Reg8Bits<59>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<59>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<59>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<59>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<59>> for crate::reg64::Reg64Bits<59> {
    fn from(item: crate::reg16::Reg16Bits<59>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<59>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<59>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<59>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<59>> for crate::reg64::Reg64Bits<59> {
    fn from(item: crate::reg32::Reg32Bits<59>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<59>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<59>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<59>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<60>> for crate::reg64::Reg64Bits<60> {
    fn from(item: crate::reg8::Reg8Bits<60>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<60>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<60>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<60>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<60>> for crate::reg64::Reg64Bits<60> {
    fn from(item: crate::reg16::Reg16Bits<60>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<60>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<60>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<60>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<60>> for crate::reg64::Reg64Bits<60> {
    fn from(item: crate::reg32::Reg32Bits<60>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<60>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<60>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<60>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<61>> for crate::reg64::Reg64Bits<61> {
    fn from(item: crate::reg8::Reg8Bits<61>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<61>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<61>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<61>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<61>> for crate::reg64::Reg64Bits<61> {
    fn from(item: crate::reg16::Reg16Bits<61>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<61>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<61>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<61>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<61>> for crate::reg64::Reg64Bits<61> {
    fn from(item: crate::reg32::Reg32Bits<61>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<61>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<61>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<61>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<62>> for crate::reg64::Reg64Bits<62> {
    fn from(item: crate::reg8::Reg8Bits<62>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<62>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<62>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<62>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<62>> for crate::reg64::Reg64Bits<62> {
    fn from(item: crate::reg16::Reg16Bits<62>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<62>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<62>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<62>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<62>> for crate::reg64::Reg64Bits<62> {
    fn from(item: crate::reg32::Reg32Bits<62>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<62>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<62>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<62>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<63>> for crate::reg64::Reg64Bits<63> {
    fn from(item: crate::reg8::Reg8Bits<63>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<63>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<63>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<63>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<63>> for crate::reg64::Reg64Bits<63> {
    fn from(item: crate::reg16::Reg16Bits<63>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<63>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<63>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<63>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<63>> for crate::reg64::Reg64Bits<63> {
    fn from(item: crate::reg32::Reg32Bits<63>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<63>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<63>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<63>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
#[cfg(feature = "8bit")]
impl From<crate::reg8::Reg8Bits<64>> for crate::reg64::Reg64Bits<64> {
    fn from(item: crate::reg8::Reg8Bits<64>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<64>>::
            take_low(crate::reg64::Reg64Bits::new(u8::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<64>> for u8 {
    fn from(item: crate::reg64::Reg64Bits<64>) -> Self {
        item.0 as u8
    }
}
#[doc(hidden)]
#[cfg(feature = "16bit")]
impl From<crate::reg16::Reg16Bits<64>> for crate::reg64::Reg64Bits<64> {
    fn from(item: crate::reg16::Reg16Bits<64>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<64>>::
            take_low(crate::reg64::Reg64Bits::new(u16::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<64>> for u16 {
    fn from(item: crate::reg64::Reg64Bits<64>) -> Self {
        item.0 as u16
    }
}
#[doc(hidden)]
#[cfg(feature = "32bit")]
impl From<crate::reg32::Reg32Bits<64>> for crate::reg64::Reg64Bits<64> {
    fn from(item: crate::reg32::Reg32Bits<64>) -> Self {
        <crate::reg64::Reg64Bits<64> as crate::reg64::Reg64BitsDownCast<64>>::
            take_low(crate::reg64::Reg64Bits::new(u32::from(item) as u64))
    }
}
#[doc(hidden)]
impl From<crate::reg64::Reg64Bits<64>> for u32 {
    fn from(item: crate::reg64::Reg64Bits<64>) -> Self {
        item.0 as u32
    }
}
#[doc(hidden)]
impl From<u8> for crate::reg64::Reg64Bits<8> {
    fn from(item: u8) -> Self {
        Self(item.into())
    }
}
#[doc(hidden)]
impl From<u16> for crate::reg64::Reg64Bits<16> {
    fn from(item: u16) -> Self {
        Self(item.into())
    }
}
#[doc(hidden)]
impl From<u32> for crate::reg64::Reg64Bits<32> {
    fn from(item: u32) -> Self {
        Self(item.into())
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<1> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<2> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<3> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<4> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<5> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<6> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<7> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<8> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<9> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<10> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<11> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<12> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<13> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<14> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<15> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<16> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<17> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<18> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<19> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<20> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<21> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<22> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<23> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<24> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<25> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<26> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<27> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<28> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<29> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<30> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<31> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<32> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<33> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<34> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<35> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<36> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<37> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<38> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<39> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<40> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<41> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<42> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<43> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<44> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<45> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<46> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<47> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<48> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<49> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<50> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<51> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<52> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<53> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<54> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<55> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<56> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<57> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<58> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<59> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<60> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<61> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<62> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<63> {
    fn default() -> Self {
        Self(0)
    }
}
#[doc(hidden)]
impl Default for crate::reg64::Reg64Bits<64> {
    fn default() -> Self {
        Self(0)
    }
}
