// This file was automatically generated with the `generate_impl_rs.py` script.
// Any bugs in this script should be addressed in the `reg_reference.rs` file.
// 
// The contents of this file is as follows:
// 1. Definition of RegXBits struct
// 2. Implementation of operations
// 3. Implementation of traits
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

#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(transparent)]
pub struct Reg8Bits<const N: usize>(BaseType);

impl<const N: usize> core::ops::Deref for Reg8Bits<N> {
    type Target = BaseType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Eq for Reg8Bits<N> {}
impl<const N: usize> Ord for Reg8Bits<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
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

const fn truncate(value: BaseType, num_bits: usize) -> BaseType {
    // Needed to circumvent overflow protection
    if num_bits == 0 {
        return 0;
    }

    // Needed to circumvent overflow protection
    if num_bits >= NUM_BITS {
        return value;
    }

    let mask = BaseType::MAX >> (NUM_BITS - num_bits);
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
    
    1 << (NUM_BITS - 1)
}

// Function to ease matching of Bits to a specific sequence of bits
impl<const N: usize> Reg8Bits<N> {
    /// N 0's in the base type
    const BASE_ZEROS: BaseType = 0;
    /// N 1's in the base type
    const BASE_ONES: BaseType = truncate(!0, N);

    const TOP_BIT_MASK: BaseType = top_bit_mask(N);

    /// A guarenteed N sequential 0's
    pub const ZEROS: Self = Self(Self::BASE_ZEROS);
    /// N sequential 1's
    pub const ONES: Self = Self(Self::BASE_ONES);

    pub fn bits(&self) -> [u8; N] {
        let mut bits = [0; N];
        let Self(mut value) = self;

        for i in 0..N {
            bits[N - i - 1] = (value & 0b1) as u8;
            value >>= 1;
        }

        bits
    }

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

impl<const N: usize> core::ops::Add for Reg8Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.0);

        Self(((lhs + rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize> core::ops::Sub for Reg8Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.0);

        Self(((lhs - rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize> core::ops::Div for Reg8Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs / rhs)
    }
}

impl<const N: usize> core::ops::Rem for Reg8Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

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
    BaseType: core::ops::Shl<T, Output = BaseType>
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
    BaseType: core::ops::Shr<T, Output = BaseType>
{
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: T) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs;

        Self((lhs >> rhs) & Self::ONES.0)
    }
}

// F > T
pub trait Reg8BitsDownCast<const T: usize>: Copy + Into<BaseType> {
    #[inline(always)]
    fn take_low(self) -> Reg8Bits<T> {
        let value: BaseType = self.into();
        Reg8Bits(Reg8Bits::<T>::BASE_ONES & value)
    }
    #[inline(always)]
    fn take_high(self) -> Reg8Bits<T> {
        let value: BaseType = self.into();
        Reg8Bits(value >> (NUM_BITS - T))
    }
}

pub trait Reg8BitsUpCast<const T: usize>: Copy + Into<BaseType> {
    #[inline(always)]
    fn zero_extend(self) -> Reg8Bits<T> {
        let value = self.into();
        Reg8Bits(value)
    }

    fn sign_extend(self) -> Reg8Bits<T> {
        // NOTE: We are assuming here that no Bits<0> structure can exist
        let value = self.into();

        let top_bit = value & Reg8Bits::<T>::TOP_BIT_MASK; // Capture only the top bit
        let top_bits = if top_bit == 0 { // Create a set of NUM_BITS-N bits of with the given sign
            0
        } else {
            !Reg8Bits::<T>::BASE_ONES // !001111 = 110000
        };

        Reg8Bits(top_bits & value)
    }
}

pub trait Reg8BitsConcat<const R: usize, const O: usize>: Copy + Into<BaseType> {
    fn concat(self, rhs: Reg8Bits<R>) -> Reg8Bits<O> {
        let lhs = self.into();
        let rhs: BaseType = rhs.into();

        Reg8Bits((lhs << R) | rhs)
    }
}

impl<const F: usize, const T: usize> Reg8BitsUpCast<T> for Reg8Bits<F>
where
    Reg8Bits<T>: Reg8BitsDownCast<F>,
{
}



impl Reg8BitsDownCast<1> for Reg8Bits<1> {}
impl Reg8BitsConcat<1, 2> for Reg8Bits<1> {}
impl Reg8BitsDownCast<1> for Reg8Bits<2> {}
impl Reg8BitsConcat<1, 3> for Reg8Bits<2> {}
impl Reg8BitsDownCast<2> for Reg8Bits<2> {}
impl Reg8BitsConcat<2, 4> for Reg8Bits<2> {}
impl Reg8BitsDownCast<1> for Reg8Bits<3> {}
impl Reg8BitsConcat<1, 4> for Reg8Bits<3> {}
impl Reg8BitsDownCast<2> for Reg8Bits<3> {}
impl Reg8BitsConcat<2, 5> for Reg8Bits<3> {}
impl Reg8BitsDownCast<3> for Reg8Bits<3> {}
impl Reg8BitsConcat<3, 6> for Reg8Bits<3> {}
impl Reg8BitsDownCast<1> for Reg8Bits<4> {}
impl Reg8BitsConcat<1, 5> for Reg8Bits<4> {}
impl Reg8BitsDownCast<2> for Reg8Bits<4> {}
impl Reg8BitsConcat<2, 6> for Reg8Bits<4> {}
impl Reg8BitsDownCast<3> for Reg8Bits<4> {}
impl Reg8BitsConcat<3, 7> for Reg8Bits<4> {}
impl Reg8BitsDownCast<4> for Reg8Bits<4> {}
impl Reg8BitsConcat<4, 8> for Reg8Bits<4> {}
impl Reg8BitsDownCast<1> for Reg8Bits<5> {}
impl Reg8BitsConcat<1, 6> for Reg8Bits<5> {}
impl Reg8BitsDownCast<2> for Reg8Bits<5> {}
impl Reg8BitsConcat<2, 7> for Reg8Bits<5> {}
impl Reg8BitsDownCast<3> for Reg8Bits<5> {}
impl Reg8BitsConcat<3, 8> for Reg8Bits<5> {}
impl Reg8BitsDownCast<4> for Reg8Bits<5> {}
impl Reg8BitsDownCast<5> for Reg8Bits<5> {}
impl Reg8BitsDownCast<1> for Reg8Bits<6> {}
impl Reg8BitsConcat<1, 7> for Reg8Bits<6> {}
impl Reg8BitsDownCast<2> for Reg8Bits<6> {}
impl Reg8BitsConcat<2, 8> for Reg8Bits<6> {}
impl Reg8BitsDownCast<3> for Reg8Bits<6> {}
impl Reg8BitsDownCast<4> for Reg8Bits<6> {}
impl Reg8BitsDownCast<5> for Reg8Bits<6> {}
impl Reg8BitsDownCast<6> for Reg8Bits<6> {}
impl Reg8BitsDownCast<1> for Reg8Bits<7> {}
impl Reg8BitsConcat<1, 8> for Reg8Bits<7> {}
impl Reg8BitsDownCast<2> for Reg8Bits<7> {}
impl Reg8BitsDownCast<3> for Reg8Bits<7> {}
impl Reg8BitsDownCast<4> for Reg8Bits<7> {}
impl Reg8BitsDownCast<5> for Reg8Bits<7> {}
impl Reg8BitsDownCast<6> for Reg8Bits<7> {}
impl Reg8BitsDownCast<7> for Reg8Bits<7> {}
impl Reg8BitsDownCast<1> for Reg8Bits<8> {}
impl Reg8BitsDownCast<2> for Reg8Bits<8> {}
impl Reg8BitsDownCast<3> for Reg8Bits<8> {}
impl Reg8BitsDownCast<4> for Reg8Bits<8> {}
impl Reg8BitsDownCast<5> for Reg8Bits<8> {}
impl Reg8BitsDownCast<6> for Reg8Bits<8> {}
impl Reg8BitsDownCast<7> for Reg8Bits<8> {}
impl Reg8BitsDownCast<8> for Reg8Bits<8> {}
