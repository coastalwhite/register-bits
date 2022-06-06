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
type BaseType = u16; // [REF_REPLACE]
const NUM_BITS: usize = BaseType::BITS as usize;

#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(transparent)]
pub struct Reg16Bits<const N: usize>(BaseType);

impl<const N: usize> core::ops::Deref for Reg16Bits<N> {
    type Target = BaseType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Eq for Reg16Bits<N> {}
impl<const N: usize> Ord for Reg16Bits<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
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
impl<const N: usize> Reg16Bits<N> {
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

impl<const N: usize> core::ops::Add for Reg16Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.0);

        Self(((lhs + rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize> core::ops::Sub for Reg16Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.0);

        Self(((lhs - rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize> core::ops::Div for Reg16Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs / rhs)
    }
}

impl<const N: usize> core::ops::Rem for Reg16Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

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

impl<const N: usize, T> core::ops::Shr<T> for Reg16Bits<N>
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
pub trait Reg16BitsDownCast<const T: usize>: Copy + Into<BaseType> {
    #[inline(always)]
    fn take_low(self) -> Reg16Bits<T> {
        let value: BaseType = self.into();
        Reg16Bits(Reg16Bits::<T>::BASE_ONES & value)
    }
    #[inline(always)]
    fn take_high(self) -> Reg16Bits<T> {
        let value: BaseType = self.into();
        Reg16Bits(value >> (NUM_BITS - T))
    }
}

pub trait Reg16BitsUpCast<const T: usize>: Copy + Into<BaseType> {
    #[inline(always)]
    fn zero_extend(self) -> Reg16Bits<T> {
        let value = self.into();
        Reg16Bits(value)
    }

    fn sign_extend(self) -> Reg16Bits<T> {
        // NOTE: We are assuming here that no Bits<0> structure can exist
        let value = self.into();

        let top_bit = value & Reg16Bits::<T>::TOP_BIT_MASK; // Capture only the top bit
        let top_bits = if top_bit == 0 { // Create a set of NUM_BITS-N bits of with the given sign
            0
        } else {
            !Reg16Bits::<T>::BASE_ONES // !001111 = 110000
        };

        Reg16Bits(top_bits & value)
    }
}

pub trait Reg16BitsConcat<const R: usize, const O: usize>: Copy + Into<BaseType> {
    fn concat(self, rhs: Reg16Bits<R>) -> Reg16Bits<O> {
        let lhs = self.into();
        let rhs: BaseType = rhs.into();

        Reg16Bits((lhs << R) | rhs)
    }
}

impl<const F: usize, const T: usize> Reg16BitsUpCast<T> for Reg16Bits<F>
where
    Reg16Bits<T>: Reg16BitsDownCast<F>,
{
}



impl Reg16BitsDownCast<1> for Reg16Bits<1> {}
impl Reg16BitsConcat<1, 2> for Reg16Bits<1> {}
impl Reg16BitsDownCast<1> for Reg16Bits<2> {}
impl Reg16BitsConcat<1, 3> for Reg16Bits<2> {}
impl Reg16BitsDownCast<2> for Reg16Bits<2> {}
impl Reg16BitsConcat<2, 4> for Reg16Bits<2> {}
impl Reg16BitsDownCast<1> for Reg16Bits<3> {}
impl Reg16BitsConcat<1, 4> for Reg16Bits<3> {}
impl Reg16BitsDownCast<2> for Reg16Bits<3> {}
impl Reg16BitsConcat<2, 5> for Reg16Bits<3> {}
impl Reg16BitsDownCast<3> for Reg16Bits<3> {}
impl Reg16BitsConcat<3, 6> for Reg16Bits<3> {}
impl Reg16BitsDownCast<1> for Reg16Bits<4> {}
impl Reg16BitsConcat<1, 5> for Reg16Bits<4> {}
impl Reg16BitsDownCast<2> for Reg16Bits<4> {}
impl Reg16BitsConcat<2, 6> for Reg16Bits<4> {}
impl Reg16BitsDownCast<3> for Reg16Bits<4> {}
impl Reg16BitsConcat<3, 7> for Reg16Bits<4> {}
impl Reg16BitsDownCast<4> for Reg16Bits<4> {}
impl Reg16BitsConcat<4, 8> for Reg16Bits<4> {}
impl Reg16BitsDownCast<1> for Reg16Bits<5> {}
impl Reg16BitsConcat<1, 6> for Reg16Bits<5> {}
impl Reg16BitsDownCast<2> for Reg16Bits<5> {}
impl Reg16BitsConcat<2, 7> for Reg16Bits<5> {}
impl Reg16BitsDownCast<3> for Reg16Bits<5> {}
impl Reg16BitsConcat<3, 8> for Reg16Bits<5> {}
impl Reg16BitsDownCast<4> for Reg16Bits<5> {}
impl Reg16BitsConcat<4, 9> for Reg16Bits<5> {}
impl Reg16BitsDownCast<5> for Reg16Bits<5> {}
impl Reg16BitsConcat<5, 10> for Reg16Bits<5> {}
impl Reg16BitsDownCast<1> for Reg16Bits<6> {}
impl Reg16BitsConcat<1, 7> for Reg16Bits<6> {}
impl Reg16BitsDownCast<2> for Reg16Bits<6> {}
impl Reg16BitsConcat<2, 8> for Reg16Bits<6> {}
impl Reg16BitsDownCast<3> for Reg16Bits<6> {}
impl Reg16BitsConcat<3, 9> for Reg16Bits<6> {}
impl Reg16BitsDownCast<4> for Reg16Bits<6> {}
impl Reg16BitsConcat<4, 10> for Reg16Bits<6> {}
impl Reg16BitsDownCast<5> for Reg16Bits<6> {}
impl Reg16BitsConcat<5, 11> for Reg16Bits<6> {}
impl Reg16BitsDownCast<6> for Reg16Bits<6> {}
impl Reg16BitsConcat<6, 12> for Reg16Bits<6> {}
impl Reg16BitsDownCast<1> for Reg16Bits<7> {}
impl Reg16BitsConcat<1, 8> for Reg16Bits<7> {}
impl Reg16BitsDownCast<2> for Reg16Bits<7> {}
impl Reg16BitsConcat<2, 9> for Reg16Bits<7> {}
impl Reg16BitsDownCast<3> for Reg16Bits<7> {}
impl Reg16BitsConcat<3, 10> for Reg16Bits<7> {}
impl Reg16BitsDownCast<4> for Reg16Bits<7> {}
impl Reg16BitsConcat<4, 11> for Reg16Bits<7> {}
impl Reg16BitsDownCast<5> for Reg16Bits<7> {}
impl Reg16BitsConcat<5, 12> for Reg16Bits<7> {}
impl Reg16BitsDownCast<6> for Reg16Bits<7> {}
impl Reg16BitsConcat<6, 13> for Reg16Bits<7> {}
impl Reg16BitsDownCast<7> for Reg16Bits<7> {}
impl Reg16BitsConcat<7, 14> for Reg16Bits<7> {}
impl Reg16BitsDownCast<1> for Reg16Bits<8> {}
impl Reg16BitsConcat<1, 9> for Reg16Bits<8> {}
impl Reg16BitsDownCast<2> for Reg16Bits<8> {}
impl Reg16BitsConcat<2, 10> for Reg16Bits<8> {}
impl Reg16BitsDownCast<3> for Reg16Bits<8> {}
impl Reg16BitsConcat<3, 11> for Reg16Bits<8> {}
impl Reg16BitsDownCast<4> for Reg16Bits<8> {}
impl Reg16BitsConcat<4, 12> for Reg16Bits<8> {}
impl Reg16BitsDownCast<5> for Reg16Bits<8> {}
impl Reg16BitsConcat<5, 13> for Reg16Bits<8> {}
impl Reg16BitsDownCast<6> for Reg16Bits<8> {}
impl Reg16BitsConcat<6, 14> for Reg16Bits<8> {}
impl Reg16BitsDownCast<7> for Reg16Bits<8> {}
impl Reg16BitsConcat<7, 15> for Reg16Bits<8> {}
impl Reg16BitsDownCast<8> for Reg16Bits<8> {}
impl Reg16BitsConcat<8, 16> for Reg16Bits<8> {}
impl Reg16BitsDownCast<1> for Reg16Bits<9> {}
impl Reg16BitsConcat<1, 10> for Reg16Bits<9> {}
impl Reg16BitsDownCast<2> for Reg16Bits<9> {}
impl Reg16BitsConcat<2, 11> for Reg16Bits<9> {}
impl Reg16BitsDownCast<3> for Reg16Bits<9> {}
impl Reg16BitsConcat<3, 12> for Reg16Bits<9> {}
impl Reg16BitsDownCast<4> for Reg16Bits<9> {}
impl Reg16BitsConcat<4, 13> for Reg16Bits<9> {}
impl Reg16BitsDownCast<5> for Reg16Bits<9> {}
impl Reg16BitsConcat<5, 14> for Reg16Bits<9> {}
impl Reg16BitsDownCast<6> for Reg16Bits<9> {}
impl Reg16BitsConcat<6, 15> for Reg16Bits<9> {}
impl Reg16BitsDownCast<7> for Reg16Bits<9> {}
impl Reg16BitsConcat<7, 16> for Reg16Bits<9> {}
impl Reg16BitsDownCast<8> for Reg16Bits<9> {}
impl Reg16BitsDownCast<9> for Reg16Bits<9> {}
impl Reg16BitsDownCast<1> for Reg16Bits<10> {}
impl Reg16BitsConcat<1, 11> for Reg16Bits<10> {}
impl Reg16BitsDownCast<2> for Reg16Bits<10> {}
impl Reg16BitsConcat<2, 12> for Reg16Bits<10> {}
impl Reg16BitsDownCast<3> for Reg16Bits<10> {}
impl Reg16BitsConcat<3, 13> for Reg16Bits<10> {}
impl Reg16BitsDownCast<4> for Reg16Bits<10> {}
impl Reg16BitsConcat<4, 14> for Reg16Bits<10> {}
impl Reg16BitsDownCast<5> for Reg16Bits<10> {}
impl Reg16BitsConcat<5, 15> for Reg16Bits<10> {}
impl Reg16BitsDownCast<6> for Reg16Bits<10> {}
impl Reg16BitsConcat<6, 16> for Reg16Bits<10> {}
impl Reg16BitsDownCast<7> for Reg16Bits<10> {}
impl Reg16BitsDownCast<8> for Reg16Bits<10> {}
impl Reg16BitsDownCast<9> for Reg16Bits<10> {}
impl Reg16BitsDownCast<10> for Reg16Bits<10> {}
impl Reg16BitsDownCast<1> for Reg16Bits<11> {}
impl Reg16BitsConcat<1, 12> for Reg16Bits<11> {}
impl Reg16BitsDownCast<2> for Reg16Bits<11> {}
impl Reg16BitsConcat<2, 13> for Reg16Bits<11> {}
impl Reg16BitsDownCast<3> for Reg16Bits<11> {}
impl Reg16BitsConcat<3, 14> for Reg16Bits<11> {}
impl Reg16BitsDownCast<4> for Reg16Bits<11> {}
impl Reg16BitsConcat<4, 15> for Reg16Bits<11> {}
impl Reg16BitsDownCast<5> for Reg16Bits<11> {}
impl Reg16BitsConcat<5, 16> for Reg16Bits<11> {}
impl Reg16BitsDownCast<6> for Reg16Bits<11> {}
impl Reg16BitsDownCast<7> for Reg16Bits<11> {}
impl Reg16BitsDownCast<8> for Reg16Bits<11> {}
impl Reg16BitsDownCast<9> for Reg16Bits<11> {}
impl Reg16BitsDownCast<10> for Reg16Bits<11> {}
impl Reg16BitsDownCast<11> for Reg16Bits<11> {}
impl Reg16BitsDownCast<1> for Reg16Bits<12> {}
impl Reg16BitsConcat<1, 13> for Reg16Bits<12> {}
impl Reg16BitsDownCast<2> for Reg16Bits<12> {}
impl Reg16BitsConcat<2, 14> for Reg16Bits<12> {}
impl Reg16BitsDownCast<3> for Reg16Bits<12> {}
impl Reg16BitsConcat<3, 15> for Reg16Bits<12> {}
impl Reg16BitsDownCast<4> for Reg16Bits<12> {}
impl Reg16BitsConcat<4, 16> for Reg16Bits<12> {}
impl Reg16BitsDownCast<5> for Reg16Bits<12> {}
impl Reg16BitsDownCast<6> for Reg16Bits<12> {}
impl Reg16BitsDownCast<7> for Reg16Bits<12> {}
impl Reg16BitsDownCast<8> for Reg16Bits<12> {}
impl Reg16BitsDownCast<9> for Reg16Bits<12> {}
impl Reg16BitsDownCast<10> for Reg16Bits<12> {}
impl Reg16BitsDownCast<11> for Reg16Bits<12> {}
impl Reg16BitsDownCast<12> for Reg16Bits<12> {}
impl Reg16BitsDownCast<1> for Reg16Bits<13> {}
impl Reg16BitsConcat<1, 14> for Reg16Bits<13> {}
impl Reg16BitsDownCast<2> for Reg16Bits<13> {}
impl Reg16BitsConcat<2, 15> for Reg16Bits<13> {}
impl Reg16BitsDownCast<3> for Reg16Bits<13> {}
impl Reg16BitsConcat<3, 16> for Reg16Bits<13> {}
impl Reg16BitsDownCast<4> for Reg16Bits<13> {}
impl Reg16BitsDownCast<5> for Reg16Bits<13> {}
impl Reg16BitsDownCast<6> for Reg16Bits<13> {}
impl Reg16BitsDownCast<7> for Reg16Bits<13> {}
impl Reg16BitsDownCast<8> for Reg16Bits<13> {}
impl Reg16BitsDownCast<9> for Reg16Bits<13> {}
impl Reg16BitsDownCast<10> for Reg16Bits<13> {}
impl Reg16BitsDownCast<11> for Reg16Bits<13> {}
impl Reg16BitsDownCast<12> for Reg16Bits<13> {}
impl Reg16BitsDownCast<13> for Reg16Bits<13> {}
impl Reg16BitsDownCast<1> for Reg16Bits<14> {}
impl Reg16BitsConcat<1, 15> for Reg16Bits<14> {}
impl Reg16BitsDownCast<2> for Reg16Bits<14> {}
impl Reg16BitsConcat<2, 16> for Reg16Bits<14> {}
impl Reg16BitsDownCast<3> for Reg16Bits<14> {}
impl Reg16BitsDownCast<4> for Reg16Bits<14> {}
impl Reg16BitsDownCast<5> for Reg16Bits<14> {}
impl Reg16BitsDownCast<6> for Reg16Bits<14> {}
impl Reg16BitsDownCast<7> for Reg16Bits<14> {}
impl Reg16BitsDownCast<8> for Reg16Bits<14> {}
impl Reg16BitsDownCast<9> for Reg16Bits<14> {}
impl Reg16BitsDownCast<10> for Reg16Bits<14> {}
impl Reg16BitsDownCast<11> for Reg16Bits<14> {}
impl Reg16BitsDownCast<12> for Reg16Bits<14> {}
impl Reg16BitsDownCast<13> for Reg16Bits<14> {}
impl Reg16BitsDownCast<14> for Reg16Bits<14> {}
impl Reg16BitsDownCast<1> for Reg16Bits<15> {}
impl Reg16BitsConcat<1, 16> for Reg16Bits<15> {}
impl Reg16BitsDownCast<2> for Reg16Bits<15> {}
impl Reg16BitsDownCast<3> for Reg16Bits<15> {}
impl Reg16BitsDownCast<4> for Reg16Bits<15> {}
impl Reg16BitsDownCast<5> for Reg16Bits<15> {}
impl Reg16BitsDownCast<6> for Reg16Bits<15> {}
impl Reg16BitsDownCast<7> for Reg16Bits<15> {}
impl Reg16BitsDownCast<8> for Reg16Bits<15> {}
impl Reg16BitsDownCast<9> for Reg16Bits<15> {}
impl Reg16BitsDownCast<10> for Reg16Bits<15> {}
impl Reg16BitsDownCast<11> for Reg16Bits<15> {}
impl Reg16BitsDownCast<12> for Reg16Bits<15> {}
impl Reg16BitsDownCast<13> for Reg16Bits<15> {}
impl Reg16BitsDownCast<14> for Reg16Bits<15> {}
impl Reg16BitsDownCast<15> for Reg16Bits<15> {}
impl Reg16BitsDownCast<1> for Reg16Bits<16> {}
impl Reg16BitsDownCast<2> for Reg16Bits<16> {}
impl Reg16BitsDownCast<3> for Reg16Bits<16> {}
impl Reg16BitsDownCast<4> for Reg16Bits<16> {}
impl Reg16BitsDownCast<5> for Reg16Bits<16> {}
impl Reg16BitsDownCast<6> for Reg16Bits<16> {}
impl Reg16BitsDownCast<7> for Reg16Bits<16> {}
impl Reg16BitsDownCast<8> for Reg16Bits<16> {}
impl Reg16BitsDownCast<9> for Reg16Bits<16> {}
impl Reg16BitsDownCast<10> for Reg16Bits<16> {}
impl Reg16BitsDownCast<11> for Reg16Bits<16> {}
impl Reg16BitsDownCast<12> for Reg16Bits<16> {}
impl Reg16BitsDownCast<13> for Reg16Bits<16> {}
impl Reg16BitsDownCast<14> for Reg16Bits<16> {}
impl Reg16BitsDownCast<15> for Reg16Bits<16> {}
impl Reg16BitsDownCast<16> for Reg16Bits<16> {}
