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
type BaseType = u128; // [REF_REPLACE]
const NUM_BITS: usize = BaseType::BITS as usize;

#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(transparent)]
pub struct Reg128Bits<const N: usize>(BaseType);

impl<const N: usize> core::ops::Deref for Reg128Bits<N> {
    type Target = BaseType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Eq for Reg128Bits<N> {}
impl<const N: usize> Ord for Reg128Bits<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Reg128Bits<NUM_BITS> {
    #[inline(always)]
    pub fn new(value: BaseType) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<Reg128Bits<N>> for BaseType {
    #[inline(always)]
    fn from(item: Reg128Bits<N>) -> Self {
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
impl<const N: usize> Reg128Bits<N> {
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

    pub fn get(&self, index: usize) -> Option<Reg128Bits<1>> {
        if index >= N {
            return None;
        }

        let last_bit = if index == 0 {
            self.0 & 0b1
        } else {
            (self.0 >> index) & 0b1
        };

        Some(Reg128Bits(last_bit))
    }
}

impl From<Reg128Bits<1>> for bool {
    #[inline(always)]
    fn from(item: Reg128Bits<1>) -> bool {
        matches!(item, Reg128Bits::<1>(1))
    }
}

impl PartialEq<bool> for Reg128Bits<1> {
    #[inline(always)]
    fn eq(&self, other: &bool) -> bool {
        bool::from(*self) == *other
    }
}

impl PartialEq<u8> for Reg128Bits<1> {
    #[inline(always)]
    fn eq(&self, other: &u8) -> bool {
        bool::from(*self) == (*other > 0)
    }
}

impl<const N: usize> core::ops::Add for Reg128Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.0);

        Self(((lhs + rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize> core::ops::Sub for Reg128Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.0);

        Self(((lhs - rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize> core::ops::Div for Reg128Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs / rhs)
    }
}

impl<const N: usize> core::ops::Rem for Reg128Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs % rhs)
    }
}

impl<const N: usize> core::ops::BitAnd for Reg128Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs & rhs)
    }
}

impl<const N: usize> core::ops::BitOr for Reg128Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs | rhs)
    }
}

impl<const N: usize> core::ops::BitXor for Reg128Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs ^ rhs)
    }
}

impl<const N: usize> core::ops::Not for Reg128Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        let lhs = self.0;

        Self((!lhs) & Self::BASE_ONES)
    }
}

impl<const N: usize, T> core::ops::Shl<T> for Reg128Bits<N>
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

impl<const N: usize, T> core::ops::Shr<T> for Reg128Bits<N>
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
pub trait Reg128BitsDownCast<const T: usize>: Copy + Into<BaseType> {
    #[inline(always)]
    fn take_low(self) -> Reg128Bits<T> {
        let value: BaseType = self.into();
        Reg128Bits(Reg128Bits::<T>::BASE_ONES & value)
    }
    #[inline(always)]
    fn take_high(self) -> Reg128Bits<T> {
        let value: BaseType = self.into();
        Reg128Bits(value >> (NUM_BITS - T))
    }
}

pub trait Reg128BitsUpCast<const T: usize>: Copy + Into<BaseType> {
    #[inline(always)]
    fn zero_extend(self) -> Reg128Bits<T> {
        let value = self.into();
        Reg128Bits(value)
    }

    fn sign_extend(self) -> Reg128Bits<T> {
        // NOTE: We are assuming here that no Bits<0> structure can exist
        let value = self.into();

        let top_bit = value & Reg128Bits::<T>::TOP_BIT_MASK; // Capture only the top bit
        let top_bits = if top_bit == 0 { // Create a set of NUM_BITS-N bits of with the given sign
            0
        } else {
            !Reg128Bits::<T>::BASE_ONES // !001111 = 110000
        };

        Reg128Bits(top_bits & value)
    }
}

pub trait Reg128BitsConcat<const R: usize, const O: usize>: Copy + Into<BaseType> {
    fn concat(self, rhs: Reg128Bits<R>) -> Reg128Bits<O> {
        let lhs = self.into();
        let rhs: BaseType = rhs.into();

        Reg128Bits((lhs << R) | rhs)
    }
}

impl<const F: usize, const T: usize> Reg128BitsUpCast<T> for Reg128Bits<F>
where
    Reg128Bits<T>: Reg128BitsDownCast<F>,
{
}



impl Reg128BitsDownCast<1> for Reg128Bits<1> {}
impl Reg128BitsConcat<1, 2> for Reg128Bits<1> {}
impl Reg128BitsDownCast<1> for Reg128Bits<2> {}
impl Reg128BitsConcat<1, 3> for Reg128Bits<2> {}
impl Reg128BitsDownCast<2> for Reg128Bits<2> {}
impl Reg128BitsConcat<2, 4> for Reg128Bits<2> {}
impl Reg128BitsDownCast<1> for Reg128Bits<3> {}
impl Reg128BitsConcat<1, 4> for Reg128Bits<3> {}
impl Reg128BitsDownCast<2> for Reg128Bits<3> {}
impl Reg128BitsConcat<2, 5> for Reg128Bits<3> {}
impl Reg128BitsDownCast<3> for Reg128Bits<3> {}
impl Reg128BitsConcat<3, 6> for Reg128Bits<3> {}
impl Reg128BitsDownCast<1> for Reg128Bits<4> {}
impl Reg128BitsConcat<1, 5> for Reg128Bits<4> {}
impl Reg128BitsDownCast<2> for Reg128Bits<4> {}
impl Reg128BitsConcat<2, 6> for Reg128Bits<4> {}
impl Reg128BitsDownCast<3> for Reg128Bits<4> {}
impl Reg128BitsConcat<3, 7> for Reg128Bits<4> {}
impl Reg128BitsDownCast<4> for Reg128Bits<4> {}
impl Reg128BitsConcat<4, 8> for Reg128Bits<4> {}
impl Reg128BitsDownCast<1> for Reg128Bits<5> {}
impl Reg128BitsConcat<1, 6> for Reg128Bits<5> {}
impl Reg128BitsDownCast<2> for Reg128Bits<5> {}
impl Reg128BitsConcat<2, 7> for Reg128Bits<5> {}
impl Reg128BitsDownCast<3> for Reg128Bits<5> {}
impl Reg128BitsConcat<3, 8> for Reg128Bits<5> {}
impl Reg128BitsDownCast<4> for Reg128Bits<5> {}
impl Reg128BitsConcat<4, 9> for Reg128Bits<5> {}
impl Reg128BitsDownCast<5> for Reg128Bits<5> {}
impl Reg128BitsConcat<5, 10> for Reg128Bits<5> {}
impl Reg128BitsDownCast<1> for Reg128Bits<6> {}
impl Reg128BitsConcat<1, 7> for Reg128Bits<6> {}
impl Reg128BitsDownCast<2> for Reg128Bits<6> {}
impl Reg128BitsConcat<2, 8> for Reg128Bits<6> {}
impl Reg128BitsDownCast<3> for Reg128Bits<6> {}
impl Reg128BitsConcat<3, 9> for Reg128Bits<6> {}
impl Reg128BitsDownCast<4> for Reg128Bits<6> {}
impl Reg128BitsConcat<4, 10> for Reg128Bits<6> {}
impl Reg128BitsDownCast<5> for Reg128Bits<6> {}
impl Reg128BitsConcat<5, 11> for Reg128Bits<6> {}
impl Reg128BitsDownCast<6> for Reg128Bits<6> {}
impl Reg128BitsConcat<6, 12> for Reg128Bits<6> {}
impl Reg128BitsDownCast<1> for Reg128Bits<7> {}
impl Reg128BitsConcat<1, 8> for Reg128Bits<7> {}
impl Reg128BitsDownCast<2> for Reg128Bits<7> {}
impl Reg128BitsConcat<2, 9> for Reg128Bits<7> {}
impl Reg128BitsDownCast<3> for Reg128Bits<7> {}
impl Reg128BitsConcat<3, 10> for Reg128Bits<7> {}
impl Reg128BitsDownCast<4> for Reg128Bits<7> {}
impl Reg128BitsConcat<4, 11> for Reg128Bits<7> {}
impl Reg128BitsDownCast<5> for Reg128Bits<7> {}
impl Reg128BitsConcat<5, 12> for Reg128Bits<7> {}
impl Reg128BitsDownCast<6> for Reg128Bits<7> {}
impl Reg128BitsConcat<6, 13> for Reg128Bits<7> {}
impl Reg128BitsDownCast<7> for Reg128Bits<7> {}
impl Reg128BitsConcat<7, 14> for Reg128Bits<7> {}
impl Reg128BitsDownCast<1> for Reg128Bits<8> {}
impl Reg128BitsConcat<1, 9> for Reg128Bits<8> {}
impl Reg128BitsDownCast<2> for Reg128Bits<8> {}
impl Reg128BitsConcat<2, 10> for Reg128Bits<8> {}
impl Reg128BitsDownCast<3> for Reg128Bits<8> {}
impl Reg128BitsConcat<3, 11> for Reg128Bits<8> {}
impl Reg128BitsDownCast<4> for Reg128Bits<8> {}
impl Reg128BitsConcat<4, 12> for Reg128Bits<8> {}
impl Reg128BitsDownCast<5> for Reg128Bits<8> {}
impl Reg128BitsConcat<5, 13> for Reg128Bits<8> {}
impl Reg128BitsDownCast<6> for Reg128Bits<8> {}
impl Reg128BitsConcat<6, 14> for Reg128Bits<8> {}
impl Reg128BitsDownCast<7> for Reg128Bits<8> {}
impl Reg128BitsConcat<7, 15> for Reg128Bits<8> {}
impl Reg128BitsDownCast<8> for Reg128Bits<8> {}
impl Reg128BitsConcat<8, 16> for Reg128Bits<8> {}
impl Reg128BitsDownCast<1> for Reg128Bits<9> {}
impl Reg128BitsConcat<1, 10> for Reg128Bits<9> {}
impl Reg128BitsDownCast<2> for Reg128Bits<9> {}
impl Reg128BitsConcat<2, 11> for Reg128Bits<9> {}
impl Reg128BitsDownCast<3> for Reg128Bits<9> {}
impl Reg128BitsConcat<3, 12> for Reg128Bits<9> {}
impl Reg128BitsDownCast<4> for Reg128Bits<9> {}
impl Reg128BitsConcat<4, 13> for Reg128Bits<9> {}
impl Reg128BitsDownCast<5> for Reg128Bits<9> {}
impl Reg128BitsConcat<5, 14> for Reg128Bits<9> {}
impl Reg128BitsDownCast<6> for Reg128Bits<9> {}
impl Reg128BitsConcat<6, 15> for Reg128Bits<9> {}
impl Reg128BitsDownCast<7> for Reg128Bits<9> {}
impl Reg128BitsConcat<7, 16> for Reg128Bits<9> {}
impl Reg128BitsDownCast<8> for Reg128Bits<9> {}
impl Reg128BitsConcat<8, 17> for Reg128Bits<9> {}
impl Reg128BitsDownCast<9> for Reg128Bits<9> {}
impl Reg128BitsConcat<9, 18> for Reg128Bits<9> {}
impl Reg128BitsDownCast<1> for Reg128Bits<10> {}
impl Reg128BitsConcat<1, 11> for Reg128Bits<10> {}
impl Reg128BitsDownCast<2> for Reg128Bits<10> {}
impl Reg128BitsConcat<2, 12> for Reg128Bits<10> {}
impl Reg128BitsDownCast<3> for Reg128Bits<10> {}
impl Reg128BitsConcat<3, 13> for Reg128Bits<10> {}
impl Reg128BitsDownCast<4> for Reg128Bits<10> {}
impl Reg128BitsConcat<4, 14> for Reg128Bits<10> {}
impl Reg128BitsDownCast<5> for Reg128Bits<10> {}
impl Reg128BitsConcat<5, 15> for Reg128Bits<10> {}
impl Reg128BitsDownCast<6> for Reg128Bits<10> {}
impl Reg128BitsConcat<6, 16> for Reg128Bits<10> {}
impl Reg128BitsDownCast<7> for Reg128Bits<10> {}
impl Reg128BitsConcat<7, 17> for Reg128Bits<10> {}
impl Reg128BitsDownCast<8> for Reg128Bits<10> {}
impl Reg128BitsConcat<8, 18> for Reg128Bits<10> {}
impl Reg128BitsDownCast<9> for Reg128Bits<10> {}
impl Reg128BitsConcat<9, 19> for Reg128Bits<10> {}
impl Reg128BitsDownCast<10> for Reg128Bits<10> {}
impl Reg128BitsConcat<10, 20> for Reg128Bits<10> {}
impl Reg128BitsDownCast<1> for Reg128Bits<11> {}
impl Reg128BitsConcat<1, 12> for Reg128Bits<11> {}
impl Reg128BitsDownCast<2> for Reg128Bits<11> {}
impl Reg128BitsConcat<2, 13> for Reg128Bits<11> {}
impl Reg128BitsDownCast<3> for Reg128Bits<11> {}
impl Reg128BitsConcat<3, 14> for Reg128Bits<11> {}
impl Reg128BitsDownCast<4> for Reg128Bits<11> {}
impl Reg128BitsConcat<4, 15> for Reg128Bits<11> {}
impl Reg128BitsDownCast<5> for Reg128Bits<11> {}
impl Reg128BitsConcat<5, 16> for Reg128Bits<11> {}
impl Reg128BitsDownCast<6> for Reg128Bits<11> {}
impl Reg128BitsConcat<6, 17> for Reg128Bits<11> {}
impl Reg128BitsDownCast<7> for Reg128Bits<11> {}
impl Reg128BitsConcat<7, 18> for Reg128Bits<11> {}
impl Reg128BitsDownCast<8> for Reg128Bits<11> {}
impl Reg128BitsConcat<8, 19> for Reg128Bits<11> {}
impl Reg128BitsDownCast<9> for Reg128Bits<11> {}
impl Reg128BitsConcat<9, 20> for Reg128Bits<11> {}
impl Reg128BitsDownCast<10> for Reg128Bits<11> {}
impl Reg128BitsConcat<10, 21> for Reg128Bits<11> {}
impl Reg128BitsDownCast<11> for Reg128Bits<11> {}
impl Reg128BitsConcat<11, 22> for Reg128Bits<11> {}
impl Reg128BitsDownCast<1> for Reg128Bits<12> {}
impl Reg128BitsConcat<1, 13> for Reg128Bits<12> {}
impl Reg128BitsDownCast<2> for Reg128Bits<12> {}
impl Reg128BitsConcat<2, 14> for Reg128Bits<12> {}
impl Reg128BitsDownCast<3> for Reg128Bits<12> {}
impl Reg128BitsConcat<3, 15> for Reg128Bits<12> {}
impl Reg128BitsDownCast<4> for Reg128Bits<12> {}
impl Reg128BitsConcat<4, 16> for Reg128Bits<12> {}
impl Reg128BitsDownCast<5> for Reg128Bits<12> {}
impl Reg128BitsConcat<5, 17> for Reg128Bits<12> {}
impl Reg128BitsDownCast<6> for Reg128Bits<12> {}
impl Reg128BitsConcat<6, 18> for Reg128Bits<12> {}
impl Reg128BitsDownCast<7> for Reg128Bits<12> {}
impl Reg128BitsConcat<7, 19> for Reg128Bits<12> {}
impl Reg128BitsDownCast<8> for Reg128Bits<12> {}
impl Reg128BitsConcat<8, 20> for Reg128Bits<12> {}
impl Reg128BitsDownCast<9> for Reg128Bits<12> {}
impl Reg128BitsConcat<9, 21> for Reg128Bits<12> {}
impl Reg128BitsDownCast<10> for Reg128Bits<12> {}
impl Reg128BitsConcat<10, 22> for Reg128Bits<12> {}
impl Reg128BitsDownCast<11> for Reg128Bits<12> {}
impl Reg128BitsConcat<11, 23> for Reg128Bits<12> {}
impl Reg128BitsDownCast<12> for Reg128Bits<12> {}
impl Reg128BitsConcat<12, 24> for Reg128Bits<12> {}
impl Reg128BitsDownCast<1> for Reg128Bits<13> {}
impl Reg128BitsConcat<1, 14> for Reg128Bits<13> {}
impl Reg128BitsDownCast<2> for Reg128Bits<13> {}
impl Reg128BitsConcat<2, 15> for Reg128Bits<13> {}
impl Reg128BitsDownCast<3> for Reg128Bits<13> {}
impl Reg128BitsConcat<3, 16> for Reg128Bits<13> {}
impl Reg128BitsDownCast<4> for Reg128Bits<13> {}
impl Reg128BitsConcat<4, 17> for Reg128Bits<13> {}
impl Reg128BitsDownCast<5> for Reg128Bits<13> {}
impl Reg128BitsConcat<5, 18> for Reg128Bits<13> {}
impl Reg128BitsDownCast<6> for Reg128Bits<13> {}
impl Reg128BitsConcat<6, 19> for Reg128Bits<13> {}
impl Reg128BitsDownCast<7> for Reg128Bits<13> {}
impl Reg128BitsConcat<7, 20> for Reg128Bits<13> {}
impl Reg128BitsDownCast<8> for Reg128Bits<13> {}
impl Reg128BitsConcat<8, 21> for Reg128Bits<13> {}
impl Reg128BitsDownCast<9> for Reg128Bits<13> {}
impl Reg128BitsConcat<9, 22> for Reg128Bits<13> {}
impl Reg128BitsDownCast<10> for Reg128Bits<13> {}
impl Reg128BitsConcat<10, 23> for Reg128Bits<13> {}
impl Reg128BitsDownCast<11> for Reg128Bits<13> {}
impl Reg128BitsConcat<11, 24> for Reg128Bits<13> {}
impl Reg128BitsDownCast<12> for Reg128Bits<13> {}
impl Reg128BitsConcat<12, 25> for Reg128Bits<13> {}
impl Reg128BitsDownCast<13> for Reg128Bits<13> {}
impl Reg128BitsConcat<13, 26> for Reg128Bits<13> {}
impl Reg128BitsDownCast<1> for Reg128Bits<14> {}
impl Reg128BitsConcat<1, 15> for Reg128Bits<14> {}
impl Reg128BitsDownCast<2> for Reg128Bits<14> {}
impl Reg128BitsConcat<2, 16> for Reg128Bits<14> {}
impl Reg128BitsDownCast<3> for Reg128Bits<14> {}
impl Reg128BitsConcat<3, 17> for Reg128Bits<14> {}
impl Reg128BitsDownCast<4> for Reg128Bits<14> {}
impl Reg128BitsConcat<4, 18> for Reg128Bits<14> {}
impl Reg128BitsDownCast<5> for Reg128Bits<14> {}
impl Reg128BitsConcat<5, 19> for Reg128Bits<14> {}
impl Reg128BitsDownCast<6> for Reg128Bits<14> {}
impl Reg128BitsConcat<6, 20> for Reg128Bits<14> {}
impl Reg128BitsDownCast<7> for Reg128Bits<14> {}
impl Reg128BitsConcat<7, 21> for Reg128Bits<14> {}
impl Reg128BitsDownCast<8> for Reg128Bits<14> {}
impl Reg128BitsConcat<8, 22> for Reg128Bits<14> {}
impl Reg128BitsDownCast<9> for Reg128Bits<14> {}
impl Reg128BitsConcat<9, 23> for Reg128Bits<14> {}
impl Reg128BitsDownCast<10> for Reg128Bits<14> {}
impl Reg128BitsConcat<10, 24> for Reg128Bits<14> {}
impl Reg128BitsDownCast<11> for Reg128Bits<14> {}
impl Reg128BitsConcat<11, 25> for Reg128Bits<14> {}
impl Reg128BitsDownCast<12> for Reg128Bits<14> {}
impl Reg128BitsConcat<12, 26> for Reg128Bits<14> {}
impl Reg128BitsDownCast<13> for Reg128Bits<14> {}
impl Reg128BitsConcat<13, 27> for Reg128Bits<14> {}
impl Reg128BitsDownCast<14> for Reg128Bits<14> {}
impl Reg128BitsConcat<14, 28> for Reg128Bits<14> {}
impl Reg128BitsDownCast<1> for Reg128Bits<15> {}
impl Reg128BitsConcat<1, 16> for Reg128Bits<15> {}
impl Reg128BitsDownCast<2> for Reg128Bits<15> {}
impl Reg128BitsConcat<2, 17> for Reg128Bits<15> {}
impl Reg128BitsDownCast<3> for Reg128Bits<15> {}
impl Reg128BitsConcat<3, 18> for Reg128Bits<15> {}
impl Reg128BitsDownCast<4> for Reg128Bits<15> {}
impl Reg128BitsConcat<4, 19> for Reg128Bits<15> {}
impl Reg128BitsDownCast<5> for Reg128Bits<15> {}
impl Reg128BitsConcat<5, 20> for Reg128Bits<15> {}
impl Reg128BitsDownCast<6> for Reg128Bits<15> {}
impl Reg128BitsConcat<6, 21> for Reg128Bits<15> {}
impl Reg128BitsDownCast<7> for Reg128Bits<15> {}
impl Reg128BitsConcat<7, 22> for Reg128Bits<15> {}
impl Reg128BitsDownCast<8> for Reg128Bits<15> {}
impl Reg128BitsConcat<8, 23> for Reg128Bits<15> {}
impl Reg128BitsDownCast<9> for Reg128Bits<15> {}
impl Reg128BitsConcat<9, 24> for Reg128Bits<15> {}
impl Reg128BitsDownCast<10> for Reg128Bits<15> {}
impl Reg128BitsConcat<10, 25> for Reg128Bits<15> {}
impl Reg128BitsDownCast<11> for Reg128Bits<15> {}
impl Reg128BitsConcat<11, 26> for Reg128Bits<15> {}
impl Reg128BitsDownCast<12> for Reg128Bits<15> {}
impl Reg128BitsConcat<12, 27> for Reg128Bits<15> {}
impl Reg128BitsDownCast<13> for Reg128Bits<15> {}
impl Reg128BitsConcat<13, 28> for Reg128Bits<15> {}
impl Reg128BitsDownCast<14> for Reg128Bits<15> {}
impl Reg128BitsConcat<14, 29> for Reg128Bits<15> {}
impl Reg128BitsDownCast<15> for Reg128Bits<15> {}
impl Reg128BitsConcat<15, 30> for Reg128Bits<15> {}
impl Reg128BitsDownCast<1> for Reg128Bits<16> {}
impl Reg128BitsConcat<1, 17> for Reg128Bits<16> {}
impl Reg128BitsDownCast<2> for Reg128Bits<16> {}
impl Reg128BitsConcat<2, 18> for Reg128Bits<16> {}
impl Reg128BitsDownCast<3> for Reg128Bits<16> {}
impl Reg128BitsConcat<3, 19> for Reg128Bits<16> {}
impl Reg128BitsDownCast<4> for Reg128Bits<16> {}
impl Reg128BitsConcat<4, 20> for Reg128Bits<16> {}
impl Reg128BitsDownCast<5> for Reg128Bits<16> {}
impl Reg128BitsConcat<5, 21> for Reg128Bits<16> {}
impl Reg128BitsDownCast<6> for Reg128Bits<16> {}
impl Reg128BitsConcat<6, 22> for Reg128Bits<16> {}
impl Reg128BitsDownCast<7> for Reg128Bits<16> {}
impl Reg128BitsConcat<7, 23> for Reg128Bits<16> {}
impl Reg128BitsDownCast<8> for Reg128Bits<16> {}
impl Reg128BitsConcat<8, 24> for Reg128Bits<16> {}
impl Reg128BitsDownCast<9> for Reg128Bits<16> {}
impl Reg128BitsConcat<9, 25> for Reg128Bits<16> {}
impl Reg128BitsDownCast<10> for Reg128Bits<16> {}
impl Reg128BitsConcat<10, 26> for Reg128Bits<16> {}
impl Reg128BitsDownCast<11> for Reg128Bits<16> {}
impl Reg128BitsConcat<11, 27> for Reg128Bits<16> {}
impl Reg128BitsDownCast<12> for Reg128Bits<16> {}
impl Reg128BitsConcat<12, 28> for Reg128Bits<16> {}
impl Reg128BitsDownCast<13> for Reg128Bits<16> {}
impl Reg128BitsConcat<13, 29> for Reg128Bits<16> {}
impl Reg128BitsDownCast<14> for Reg128Bits<16> {}
impl Reg128BitsConcat<14, 30> for Reg128Bits<16> {}
impl Reg128BitsDownCast<15> for Reg128Bits<16> {}
impl Reg128BitsConcat<15, 31> for Reg128Bits<16> {}
impl Reg128BitsDownCast<16> for Reg128Bits<16> {}
impl Reg128BitsConcat<16, 32> for Reg128Bits<16> {}
impl Reg128BitsDownCast<1> for Reg128Bits<17> {}
impl Reg128BitsConcat<1, 18> for Reg128Bits<17> {}
impl Reg128BitsDownCast<2> for Reg128Bits<17> {}
impl Reg128BitsConcat<2, 19> for Reg128Bits<17> {}
impl Reg128BitsDownCast<3> for Reg128Bits<17> {}
impl Reg128BitsConcat<3, 20> for Reg128Bits<17> {}
impl Reg128BitsDownCast<4> for Reg128Bits<17> {}
impl Reg128BitsConcat<4, 21> for Reg128Bits<17> {}
impl Reg128BitsDownCast<5> for Reg128Bits<17> {}
impl Reg128BitsConcat<5, 22> for Reg128Bits<17> {}
impl Reg128BitsDownCast<6> for Reg128Bits<17> {}
impl Reg128BitsConcat<6, 23> for Reg128Bits<17> {}
impl Reg128BitsDownCast<7> for Reg128Bits<17> {}
impl Reg128BitsConcat<7, 24> for Reg128Bits<17> {}
impl Reg128BitsDownCast<8> for Reg128Bits<17> {}
impl Reg128BitsConcat<8, 25> for Reg128Bits<17> {}
impl Reg128BitsDownCast<9> for Reg128Bits<17> {}
impl Reg128BitsConcat<9, 26> for Reg128Bits<17> {}
impl Reg128BitsDownCast<10> for Reg128Bits<17> {}
impl Reg128BitsConcat<10, 27> for Reg128Bits<17> {}
impl Reg128BitsDownCast<11> for Reg128Bits<17> {}
impl Reg128BitsConcat<11, 28> for Reg128Bits<17> {}
impl Reg128BitsDownCast<12> for Reg128Bits<17> {}
impl Reg128BitsConcat<12, 29> for Reg128Bits<17> {}
impl Reg128BitsDownCast<13> for Reg128Bits<17> {}
impl Reg128BitsConcat<13, 30> for Reg128Bits<17> {}
impl Reg128BitsDownCast<14> for Reg128Bits<17> {}
impl Reg128BitsConcat<14, 31> for Reg128Bits<17> {}
impl Reg128BitsDownCast<15> for Reg128Bits<17> {}
impl Reg128BitsConcat<15, 32> for Reg128Bits<17> {}
impl Reg128BitsDownCast<16> for Reg128Bits<17> {}
impl Reg128BitsConcat<16, 33> for Reg128Bits<17> {}
impl Reg128BitsDownCast<17> for Reg128Bits<17> {}
impl Reg128BitsConcat<17, 34> for Reg128Bits<17> {}
impl Reg128BitsDownCast<1> for Reg128Bits<18> {}
impl Reg128BitsConcat<1, 19> for Reg128Bits<18> {}
impl Reg128BitsDownCast<2> for Reg128Bits<18> {}
impl Reg128BitsConcat<2, 20> for Reg128Bits<18> {}
impl Reg128BitsDownCast<3> for Reg128Bits<18> {}
impl Reg128BitsConcat<3, 21> for Reg128Bits<18> {}
impl Reg128BitsDownCast<4> for Reg128Bits<18> {}
impl Reg128BitsConcat<4, 22> for Reg128Bits<18> {}
impl Reg128BitsDownCast<5> for Reg128Bits<18> {}
impl Reg128BitsConcat<5, 23> for Reg128Bits<18> {}
impl Reg128BitsDownCast<6> for Reg128Bits<18> {}
impl Reg128BitsConcat<6, 24> for Reg128Bits<18> {}
impl Reg128BitsDownCast<7> for Reg128Bits<18> {}
impl Reg128BitsConcat<7, 25> for Reg128Bits<18> {}
impl Reg128BitsDownCast<8> for Reg128Bits<18> {}
impl Reg128BitsConcat<8, 26> for Reg128Bits<18> {}
impl Reg128BitsDownCast<9> for Reg128Bits<18> {}
impl Reg128BitsConcat<9, 27> for Reg128Bits<18> {}
impl Reg128BitsDownCast<10> for Reg128Bits<18> {}
impl Reg128BitsConcat<10, 28> for Reg128Bits<18> {}
impl Reg128BitsDownCast<11> for Reg128Bits<18> {}
impl Reg128BitsConcat<11, 29> for Reg128Bits<18> {}
impl Reg128BitsDownCast<12> for Reg128Bits<18> {}
impl Reg128BitsConcat<12, 30> for Reg128Bits<18> {}
impl Reg128BitsDownCast<13> for Reg128Bits<18> {}
impl Reg128BitsConcat<13, 31> for Reg128Bits<18> {}
impl Reg128BitsDownCast<14> for Reg128Bits<18> {}
impl Reg128BitsConcat<14, 32> for Reg128Bits<18> {}
impl Reg128BitsDownCast<15> for Reg128Bits<18> {}
impl Reg128BitsConcat<15, 33> for Reg128Bits<18> {}
impl Reg128BitsDownCast<16> for Reg128Bits<18> {}
impl Reg128BitsConcat<16, 34> for Reg128Bits<18> {}
impl Reg128BitsDownCast<17> for Reg128Bits<18> {}
impl Reg128BitsConcat<17, 35> for Reg128Bits<18> {}
impl Reg128BitsDownCast<18> for Reg128Bits<18> {}
impl Reg128BitsConcat<18, 36> for Reg128Bits<18> {}
impl Reg128BitsDownCast<1> for Reg128Bits<19> {}
impl Reg128BitsConcat<1, 20> for Reg128Bits<19> {}
impl Reg128BitsDownCast<2> for Reg128Bits<19> {}
impl Reg128BitsConcat<2, 21> for Reg128Bits<19> {}
impl Reg128BitsDownCast<3> for Reg128Bits<19> {}
impl Reg128BitsConcat<3, 22> for Reg128Bits<19> {}
impl Reg128BitsDownCast<4> for Reg128Bits<19> {}
impl Reg128BitsConcat<4, 23> for Reg128Bits<19> {}
impl Reg128BitsDownCast<5> for Reg128Bits<19> {}
impl Reg128BitsConcat<5, 24> for Reg128Bits<19> {}
impl Reg128BitsDownCast<6> for Reg128Bits<19> {}
impl Reg128BitsConcat<6, 25> for Reg128Bits<19> {}
impl Reg128BitsDownCast<7> for Reg128Bits<19> {}
impl Reg128BitsConcat<7, 26> for Reg128Bits<19> {}
impl Reg128BitsDownCast<8> for Reg128Bits<19> {}
impl Reg128BitsConcat<8, 27> for Reg128Bits<19> {}
impl Reg128BitsDownCast<9> for Reg128Bits<19> {}
impl Reg128BitsConcat<9, 28> for Reg128Bits<19> {}
impl Reg128BitsDownCast<10> for Reg128Bits<19> {}
impl Reg128BitsConcat<10, 29> for Reg128Bits<19> {}
impl Reg128BitsDownCast<11> for Reg128Bits<19> {}
impl Reg128BitsConcat<11, 30> for Reg128Bits<19> {}
impl Reg128BitsDownCast<12> for Reg128Bits<19> {}
impl Reg128BitsConcat<12, 31> for Reg128Bits<19> {}
impl Reg128BitsDownCast<13> for Reg128Bits<19> {}
impl Reg128BitsConcat<13, 32> for Reg128Bits<19> {}
impl Reg128BitsDownCast<14> for Reg128Bits<19> {}
impl Reg128BitsConcat<14, 33> for Reg128Bits<19> {}
impl Reg128BitsDownCast<15> for Reg128Bits<19> {}
impl Reg128BitsConcat<15, 34> for Reg128Bits<19> {}
impl Reg128BitsDownCast<16> for Reg128Bits<19> {}
impl Reg128BitsConcat<16, 35> for Reg128Bits<19> {}
impl Reg128BitsDownCast<17> for Reg128Bits<19> {}
impl Reg128BitsConcat<17, 36> for Reg128Bits<19> {}
impl Reg128BitsDownCast<18> for Reg128Bits<19> {}
impl Reg128BitsConcat<18, 37> for Reg128Bits<19> {}
impl Reg128BitsDownCast<19> for Reg128Bits<19> {}
impl Reg128BitsConcat<19, 38> for Reg128Bits<19> {}
impl Reg128BitsDownCast<1> for Reg128Bits<20> {}
impl Reg128BitsConcat<1, 21> for Reg128Bits<20> {}
impl Reg128BitsDownCast<2> for Reg128Bits<20> {}
impl Reg128BitsConcat<2, 22> for Reg128Bits<20> {}
impl Reg128BitsDownCast<3> for Reg128Bits<20> {}
impl Reg128BitsConcat<3, 23> for Reg128Bits<20> {}
impl Reg128BitsDownCast<4> for Reg128Bits<20> {}
impl Reg128BitsConcat<4, 24> for Reg128Bits<20> {}
impl Reg128BitsDownCast<5> for Reg128Bits<20> {}
impl Reg128BitsConcat<5, 25> for Reg128Bits<20> {}
impl Reg128BitsDownCast<6> for Reg128Bits<20> {}
impl Reg128BitsConcat<6, 26> for Reg128Bits<20> {}
impl Reg128BitsDownCast<7> for Reg128Bits<20> {}
impl Reg128BitsConcat<7, 27> for Reg128Bits<20> {}
impl Reg128BitsDownCast<8> for Reg128Bits<20> {}
impl Reg128BitsConcat<8, 28> for Reg128Bits<20> {}
impl Reg128BitsDownCast<9> for Reg128Bits<20> {}
impl Reg128BitsConcat<9, 29> for Reg128Bits<20> {}
impl Reg128BitsDownCast<10> for Reg128Bits<20> {}
impl Reg128BitsConcat<10, 30> for Reg128Bits<20> {}
impl Reg128BitsDownCast<11> for Reg128Bits<20> {}
impl Reg128BitsConcat<11, 31> for Reg128Bits<20> {}
impl Reg128BitsDownCast<12> for Reg128Bits<20> {}
impl Reg128BitsConcat<12, 32> for Reg128Bits<20> {}
impl Reg128BitsDownCast<13> for Reg128Bits<20> {}
impl Reg128BitsConcat<13, 33> for Reg128Bits<20> {}
impl Reg128BitsDownCast<14> for Reg128Bits<20> {}
impl Reg128BitsConcat<14, 34> for Reg128Bits<20> {}
impl Reg128BitsDownCast<15> for Reg128Bits<20> {}
impl Reg128BitsConcat<15, 35> for Reg128Bits<20> {}
impl Reg128BitsDownCast<16> for Reg128Bits<20> {}
impl Reg128BitsConcat<16, 36> for Reg128Bits<20> {}
impl Reg128BitsDownCast<17> for Reg128Bits<20> {}
impl Reg128BitsConcat<17, 37> for Reg128Bits<20> {}
impl Reg128BitsDownCast<18> for Reg128Bits<20> {}
impl Reg128BitsConcat<18, 38> for Reg128Bits<20> {}
impl Reg128BitsDownCast<19> for Reg128Bits<20> {}
impl Reg128BitsConcat<19, 39> for Reg128Bits<20> {}
impl Reg128BitsDownCast<20> for Reg128Bits<20> {}
impl Reg128BitsConcat<20, 40> for Reg128Bits<20> {}
impl Reg128BitsDownCast<1> for Reg128Bits<21> {}
impl Reg128BitsConcat<1, 22> for Reg128Bits<21> {}
impl Reg128BitsDownCast<2> for Reg128Bits<21> {}
impl Reg128BitsConcat<2, 23> for Reg128Bits<21> {}
impl Reg128BitsDownCast<3> for Reg128Bits<21> {}
impl Reg128BitsConcat<3, 24> for Reg128Bits<21> {}
impl Reg128BitsDownCast<4> for Reg128Bits<21> {}
impl Reg128BitsConcat<4, 25> for Reg128Bits<21> {}
impl Reg128BitsDownCast<5> for Reg128Bits<21> {}
impl Reg128BitsConcat<5, 26> for Reg128Bits<21> {}
impl Reg128BitsDownCast<6> for Reg128Bits<21> {}
impl Reg128BitsConcat<6, 27> for Reg128Bits<21> {}
impl Reg128BitsDownCast<7> for Reg128Bits<21> {}
impl Reg128BitsConcat<7, 28> for Reg128Bits<21> {}
impl Reg128BitsDownCast<8> for Reg128Bits<21> {}
impl Reg128BitsConcat<8, 29> for Reg128Bits<21> {}
impl Reg128BitsDownCast<9> for Reg128Bits<21> {}
impl Reg128BitsConcat<9, 30> for Reg128Bits<21> {}
impl Reg128BitsDownCast<10> for Reg128Bits<21> {}
impl Reg128BitsConcat<10, 31> for Reg128Bits<21> {}
impl Reg128BitsDownCast<11> for Reg128Bits<21> {}
impl Reg128BitsConcat<11, 32> for Reg128Bits<21> {}
impl Reg128BitsDownCast<12> for Reg128Bits<21> {}
impl Reg128BitsConcat<12, 33> for Reg128Bits<21> {}
impl Reg128BitsDownCast<13> for Reg128Bits<21> {}
impl Reg128BitsConcat<13, 34> for Reg128Bits<21> {}
impl Reg128BitsDownCast<14> for Reg128Bits<21> {}
impl Reg128BitsConcat<14, 35> for Reg128Bits<21> {}
impl Reg128BitsDownCast<15> for Reg128Bits<21> {}
impl Reg128BitsConcat<15, 36> for Reg128Bits<21> {}
impl Reg128BitsDownCast<16> for Reg128Bits<21> {}
impl Reg128BitsConcat<16, 37> for Reg128Bits<21> {}
impl Reg128BitsDownCast<17> for Reg128Bits<21> {}
impl Reg128BitsConcat<17, 38> for Reg128Bits<21> {}
impl Reg128BitsDownCast<18> for Reg128Bits<21> {}
impl Reg128BitsConcat<18, 39> for Reg128Bits<21> {}
impl Reg128BitsDownCast<19> for Reg128Bits<21> {}
impl Reg128BitsConcat<19, 40> for Reg128Bits<21> {}
impl Reg128BitsDownCast<20> for Reg128Bits<21> {}
impl Reg128BitsConcat<20, 41> for Reg128Bits<21> {}
impl Reg128BitsDownCast<21> for Reg128Bits<21> {}
impl Reg128BitsConcat<21, 42> for Reg128Bits<21> {}
impl Reg128BitsDownCast<1> for Reg128Bits<22> {}
impl Reg128BitsConcat<1, 23> for Reg128Bits<22> {}
impl Reg128BitsDownCast<2> for Reg128Bits<22> {}
impl Reg128BitsConcat<2, 24> for Reg128Bits<22> {}
impl Reg128BitsDownCast<3> for Reg128Bits<22> {}
impl Reg128BitsConcat<3, 25> for Reg128Bits<22> {}
impl Reg128BitsDownCast<4> for Reg128Bits<22> {}
impl Reg128BitsConcat<4, 26> for Reg128Bits<22> {}
impl Reg128BitsDownCast<5> for Reg128Bits<22> {}
impl Reg128BitsConcat<5, 27> for Reg128Bits<22> {}
impl Reg128BitsDownCast<6> for Reg128Bits<22> {}
impl Reg128BitsConcat<6, 28> for Reg128Bits<22> {}
impl Reg128BitsDownCast<7> for Reg128Bits<22> {}
impl Reg128BitsConcat<7, 29> for Reg128Bits<22> {}
impl Reg128BitsDownCast<8> for Reg128Bits<22> {}
impl Reg128BitsConcat<8, 30> for Reg128Bits<22> {}
impl Reg128BitsDownCast<9> for Reg128Bits<22> {}
impl Reg128BitsConcat<9, 31> for Reg128Bits<22> {}
impl Reg128BitsDownCast<10> for Reg128Bits<22> {}
impl Reg128BitsConcat<10, 32> for Reg128Bits<22> {}
impl Reg128BitsDownCast<11> for Reg128Bits<22> {}
impl Reg128BitsConcat<11, 33> for Reg128Bits<22> {}
impl Reg128BitsDownCast<12> for Reg128Bits<22> {}
impl Reg128BitsConcat<12, 34> for Reg128Bits<22> {}
impl Reg128BitsDownCast<13> for Reg128Bits<22> {}
impl Reg128BitsConcat<13, 35> for Reg128Bits<22> {}
impl Reg128BitsDownCast<14> for Reg128Bits<22> {}
impl Reg128BitsConcat<14, 36> for Reg128Bits<22> {}
impl Reg128BitsDownCast<15> for Reg128Bits<22> {}
impl Reg128BitsConcat<15, 37> for Reg128Bits<22> {}
impl Reg128BitsDownCast<16> for Reg128Bits<22> {}
impl Reg128BitsConcat<16, 38> for Reg128Bits<22> {}
impl Reg128BitsDownCast<17> for Reg128Bits<22> {}
impl Reg128BitsConcat<17, 39> for Reg128Bits<22> {}
impl Reg128BitsDownCast<18> for Reg128Bits<22> {}
impl Reg128BitsConcat<18, 40> for Reg128Bits<22> {}
impl Reg128BitsDownCast<19> for Reg128Bits<22> {}
impl Reg128BitsConcat<19, 41> for Reg128Bits<22> {}
impl Reg128BitsDownCast<20> for Reg128Bits<22> {}
impl Reg128BitsConcat<20, 42> for Reg128Bits<22> {}
impl Reg128BitsDownCast<21> for Reg128Bits<22> {}
impl Reg128BitsConcat<21, 43> for Reg128Bits<22> {}
impl Reg128BitsDownCast<22> for Reg128Bits<22> {}
impl Reg128BitsConcat<22, 44> for Reg128Bits<22> {}
impl Reg128BitsDownCast<1> for Reg128Bits<23> {}
impl Reg128BitsConcat<1, 24> for Reg128Bits<23> {}
impl Reg128BitsDownCast<2> for Reg128Bits<23> {}
impl Reg128BitsConcat<2, 25> for Reg128Bits<23> {}
impl Reg128BitsDownCast<3> for Reg128Bits<23> {}
impl Reg128BitsConcat<3, 26> for Reg128Bits<23> {}
impl Reg128BitsDownCast<4> for Reg128Bits<23> {}
impl Reg128BitsConcat<4, 27> for Reg128Bits<23> {}
impl Reg128BitsDownCast<5> for Reg128Bits<23> {}
impl Reg128BitsConcat<5, 28> for Reg128Bits<23> {}
impl Reg128BitsDownCast<6> for Reg128Bits<23> {}
impl Reg128BitsConcat<6, 29> for Reg128Bits<23> {}
impl Reg128BitsDownCast<7> for Reg128Bits<23> {}
impl Reg128BitsConcat<7, 30> for Reg128Bits<23> {}
impl Reg128BitsDownCast<8> for Reg128Bits<23> {}
impl Reg128BitsConcat<8, 31> for Reg128Bits<23> {}
impl Reg128BitsDownCast<9> for Reg128Bits<23> {}
impl Reg128BitsConcat<9, 32> for Reg128Bits<23> {}
impl Reg128BitsDownCast<10> for Reg128Bits<23> {}
impl Reg128BitsConcat<10, 33> for Reg128Bits<23> {}
impl Reg128BitsDownCast<11> for Reg128Bits<23> {}
impl Reg128BitsConcat<11, 34> for Reg128Bits<23> {}
impl Reg128BitsDownCast<12> for Reg128Bits<23> {}
impl Reg128BitsConcat<12, 35> for Reg128Bits<23> {}
impl Reg128BitsDownCast<13> for Reg128Bits<23> {}
impl Reg128BitsConcat<13, 36> for Reg128Bits<23> {}
impl Reg128BitsDownCast<14> for Reg128Bits<23> {}
impl Reg128BitsConcat<14, 37> for Reg128Bits<23> {}
impl Reg128BitsDownCast<15> for Reg128Bits<23> {}
impl Reg128BitsConcat<15, 38> for Reg128Bits<23> {}
impl Reg128BitsDownCast<16> for Reg128Bits<23> {}
impl Reg128BitsConcat<16, 39> for Reg128Bits<23> {}
impl Reg128BitsDownCast<17> for Reg128Bits<23> {}
impl Reg128BitsConcat<17, 40> for Reg128Bits<23> {}
impl Reg128BitsDownCast<18> for Reg128Bits<23> {}
impl Reg128BitsConcat<18, 41> for Reg128Bits<23> {}
impl Reg128BitsDownCast<19> for Reg128Bits<23> {}
impl Reg128BitsConcat<19, 42> for Reg128Bits<23> {}
impl Reg128BitsDownCast<20> for Reg128Bits<23> {}
impl Reg128BitsConcat<20, 43> for Reg128Bits<23> {}
impl Reg128BitsDownCast<21> for Reg128Bits<23> {}
impl Reg128BitsConcat<21, 44> for Reg128Bits<23> {}
impl Reg128BitsDownCast<22> for Reg128Bits<23> {}
impl Reg128BitsConcat<22, 45> for Reg128Bits<23> {}
impl Reg128BitsDownCast<23> for Reg128Bits<23> {}
impl Reg128BitsConcat<23, 46> for Reg128Bits<23> {}
impl Reg128BitsDownCast<1> for Reg128Bits<24> {}
impl Reg128BitsConcat<1, 25> for Reg128Bits<24> {}
impl Reg128BitsDownCast<2> for Reg128Bits<24> {}
impl Reg128BitsConcat<2, 26> for Reg128Bits<24> {}
impl Reg128BitsDownCast<3> for Reg128Bits<24> {}
impl Reg128BitsConcat<3, 27> for Reg128Bits<24> {}
impl Reg128BitsDownCast<4> for Reg128Bits<24> {}
impl Reg128BitsConcat<4, 28> for Reg128Bits<24> {}
impl Reg128BitsDownCast<5> for Reg128Bits<24> {}
impl Reg128BitsConcat<5, 29> for Reg128Bits<24> {}
impl Reg128BitsDownCast<6> for Reg128Bits<24> {}
impl Reg128BitsConcat<6, 30> for Reg128Bits<24> {}
impl Reg128BitsDownCast<7> for Reg128Bits<24> {}
impl Reg128BitsConcat<7, 31> for Reg128Bits<24> {}
impl Reg128BitsDownCast<8> for Reg128Bits<24> {}
impl Reg128BitsConcat<8, 32> for Reg128Bits<24> {}
impl Reg128BitsDownCast<9> for Reg128Bits<24> {}
impl Reg128BitsConcat<9, 33> for Reg128Bits<24> {}
impl Reg128BitsDownCast<10> for Reg128Bits<24> {}
impl Reg128BitsConcat<10, 34> for Reg128Bits<24> {}
impl Reg128BitsDownCast<11> for Reg128Bits<24> {}
impl Reg128BitsConcat<11, 35> for Reg128Bits<24> {}
impl Reg128BitsDownCast<12> for Reg128Bits<24> {}
impl Reg128BitsConcat<12, 36> for Reg128Bits<24> {}
impl Reg128BitsDownCast<13> for Reg128Bits<24> {}
impl Reg128BitsConcat<13, 37> for Reg128Bits<24> {}
impl Reg128BitsDownCast<14> for Reg128Bits<24> {}
impl Reg128BitsConcat<14, 38> for Reg128Bits<24> {}
impl Reg128BitsDownCast<15> for Reg128Bits<24> {}
impl Reg128BitsConcat<15, 39> for Reg128Bits<24> {}
impl Reg128BitsDownCast<16> for Reg128Bits<24> {}
impl Reg128BitsConcat<16, 40> for Reg128Bits<24> {}
impl Reg128BitsDownCast<17> for Reg128Bits<24> {}
impl Reg128BitsConcat<17, 41> for Reg128Bits<24> {}
impl Reg128BitsDownCast<18> for Reg128Bits<24> {}
impl Reg128BitsConcat<18, 42> for Reg128Bits<24> {}
impl Reg128BitsDownCast<19> for Reg128Bits<24> {}
impl Reg128BitsConcat<19, 43> for Reg128Bits<24> {}
impl Reg128BitsDownCast<20> for Reg128Bits<24> {}
impl Reg128BitsConcat<20, 44> for Reg128Bits<24> {}
impl Reg128BitsDownCast<21> for Reg128Bits<24> {}
impl Reg128BitsConcat<21, 45> for Reg128Bits<24> {}
impl Reg128BitsDownCast<22> for Reg128Bits<24> {}
impl Reg128BitsConcat<22, 46> for Reg128Bits<24> {}
impl Reg128BitsDownCast<23> for Reg128Bits<24> {}
impl Reg128BitsConcat<23, 47> for Reg128Bits<24> {}
impl Reg128BitsDownCast<24> for Reg128Bits<24> {}
impl Reg128BitsConcat<24, 48> for Reg128Bits<24> {}
impl Reg128BitsDownCast<1> for Reg128Bits<25> {}
impl Reg128BitsConcat<1, 26> for Reg128Bits<25> {}
impl Reg128BitsDownCast<2> for Reg128Bits<25> {}
impl Reg128BitsConcat<2, 27> for Reg128Bits<25> {}
impl Reg128BitsDownCast<3> for Reg128Bits<25> {}
impl Reg128BitsConcat<3, 28> for Reg128Bits<25> {}
impl Reg128BitsDownCast<4> for Reg128Bits<25> {}
impl Reg128BitsConcat<4, 29> for Reg128Bits<25> {}
impl Reg128BitsDownCast<5> for Reg128Bits<25> {}
impl Reg128BitsConcat<5, 30> for Reg128Bits<25> {}
impl Reg128BitsDownCast<6> for Reg128Bits<25> {}
impl Reg128BitsConcat<6, 31> for Reg128Bits<25> {}
impl Reg128BitsDownCast<7> for Reg128Bits<25> {}
impl Reg128BitsConcat<7, 32> for Reg128Bits<25> {}
impl Reg128BitsDownCast<8> for Reg128Bits<25> {}
impl Reg128BitsConcat<8, 33> for Reg128Bits<25> {}
impl Reg128BitsDownCast<9> for Reg128Bits<25> {}
impl Reg128BitsConcat<9, 34> for Reg128Bits<25> {}
impl Reg128BitsDownCast<10> for Reg128Bits<25> {}
impl Reg128BitsConcat<10, 35> for Reg128Bits<25> {}
impl Reg128BitsDownCast<11> for Reg128Bits<25> {}
impl Reg128BitsConcat<11, 36> for Reg128Bits<25> {}
impl Reg128BitsDownCast<12> for Reg128Bits<25> {}
impl Reg128BitsConcat<12, 37> for Reg128Bits<25> {}
impl Reg128BitsDownCast<13> for Reg128Bits<25> {}
impl Reg128BitsConcat<13, 38> for Reg128Bits<25> {}
impl Reg128BitsDownCast<14> for Reg128Bits<25> {}
impl Reg128BitsConcat<14, 39> for Reg128Bits<25> {}
impl Reg128BitsDownCast<15> for Reg128Bits<25> {}
impl Reg128BitsConcat<15, 40> for Reg128Bits<25> {}
impl Reg128BitsDownCast<16> for Reg128Bits<25> {}
impl Reg128BitsConcat<16, 41> for Reg128Bits<25> {}
impl Reg128BitsDownCast<17> for Reg128Bits<25> {}
impl Reg128BitsConcat<17, 42> for Reg128Bits<25> {}
impl Reg128BitsDownCast<18> for Reg128Bits<25> {}
impl Reg128BitsConcat<18, 43> for Reg128Bits<25> {}
impl Reg128BitsDownCast<19> for Reg128Bits<25> {}
impl Reg128BitsConcat<19, 44> for Reg128Bits<25> {}
impl Reg128BitsDownCast<20> for Reg128Bits<25> {}
impl Reg128BitsConcat<20, 45> for Reg128Bits<25> {}
impl Reg128BitsDownCast<21> for Reg128Bits<25> {}
impl Reg128BitsConcat<21, 46> for Reg128Bits<25> {}
impl Reg128BitsDownCast<22> for Reg128Bits<25> {}
impl Reg128BitsConcat<22, 47> for Reg128Bits<25> {}
impl Reg128BitsDownCast<23> for Reg128Bits<25> {}
impl Reg128BitsConcat<23, 48> for Reg128Bits<25> {}
impl Reg128BitsDownCast<24> for Reg128Bits<25> {}
impl Reg128BitsConcat<24, 49> for Reg128Bits<25> {}
impl Reg128BitsDownCast<25> for Reg128Bits<25> {}
impl Reg128BitsConcat<25, 50> for Reg128Bits<25> {}
impl Reg128BitsDownCast<1> for Reg128Bits<26> {}
impl Reg128BitsConcat<1, 27> for Reg128Bits<26> {}
impl Reg128BitsDownCast<2> for Reg128Bits<26> {}
impl Reg128BitsConcat<2, 28> for Reg128Bits<26> {}
impl Reg128BitsDownCast<3> for Reg128Bits<26> {}
impl Reg128BitsConcat<3, 29> for Reg128Bits<26> {}
impl Reg128BitsDownCast<4> for Reg128Bits<26> {}
impl Reg128BitsConcat<4, 30> for Reg128Bits<26> {}
impl Reg128BitsDownCast<5> for Reg128Bits<26> {}
impl Reg128BitsConcat<5, 31> for Reg128Bits<26> {}
impl Reg128BitsDownCast<6> for Reg128Bits<26> {}
impl Reg128BitsConcat<6, 32> for Reg128Bits<26> {}
impl Reg128BitsDownCast<7> for Reg128Bits<26> {}
impl Reg128BitsConcat<7, 33> for Reg128Bits<26> {}
impl Reg128BitsDownCast<8> for Reg128Bits<26> {}
impl Reg128BitsConcat<8, 34> for Reg128Bits<26> {}
impl Reg128BitsDownCast<9> for Reg128Bits<26> {}
impl Reg128BitsConcat<9, 35> for Reg128Bits<26> {}
impl Reg128BitsDownCast<10> for Reg128Bits<26> {}
impl Reg128BitsConcat<10, 36> for Reg128Bits<26> {}
impl Reg128BitsDownCast<11> for Reg128Bits<26> {}
impl Reg128BitsConcat<11, 37> for Reg128Bits<26> {}
impl Reg128BitsDownCast<12> for Reg128Bits<26> {}
impl Reg128BitsConcat<12, 38> for Reg128Bits<26> {}
impl Reg128BitsDownCast<13> for Reg128Bits<26> {}
impl Reg128BitsConcat<13, 39> for Reg128Bits<26> {}
impl Reg128BitsDownCast<14> for Reg128Bits<26> {}
impl Reg128BitsConcat<14, 40> for Reg128Bits<26> {}
impl Reg128BitsDownCast<15> for Reg128Bits<26> {}
impl Reg128BitsConcat<15, 41> for Reg128Bits<26> {}
impl Reg128BitsDownCast<16> for Reg128Bits<26> {}
impl Reg128BitsConcat<16, 42> for Reg128Bits<26> {}
impl Reg128BitsDownCast<17> for Reg128Bits<26> {}
impl Reg128BitsConcat<17, 43> for Reg128Bits<26> {}
impl Reg128BitsDownCast<18> for Reg128Bits<26> {}
impl Reg128BitsConcat<18, 44> for Reg128Bits<26> {}
impl Reg128BitsDownCast<19> for Reg128Bits<26> {}
impl Reg128BitsConcat<19, 45> for Reg128Bits<26> {}
impl Reg128BitsDownCast<20> for Reg128Bits<26> {}
impl Reg128BitsConcat<20, 46> for Reg128Bits<26> {}
impl Reg128BitsDownCast<21> for Reg128Bits<26> {}
impl Reg128BitsConcat<21, 47> for Reg128Bits<26> {}
impl Reg128BitsDownCast<22> for Reg128Bits<26> {}
impl Reg128BitsConcat<22, 48> for Reg128Bits<26> {}
impl Reg128BitsDownCast<23> for Reg128Bits<26> {}
impl Reg128BitsConcat<23, 49> for Reg128Bits<26> {}
impl Reg128BitsDownCast<24> for Reg128Bits<26> {}
impl Reg128BitsConcat<24, 50> for Reg128Bits<26> {}
impl Reg128BitsDownCast<25> for Reg128Bits<26> {}
impl Reg128BitsConcat<25, 51> for Reg128Bits<26> {}
impl Reg128BitsDownCast<26> for Reg128Bits<26> {}
impl Reg128BitsConcat<26, 52> for Reg128Bits<26> {}
impl Reg128BitsDownCast<1> for Reg128Bits<27> {}
impl Reg128BitsConcat<1, 28> for Reg128Bits<27> {}
impl Reg128BitsDownCast<2> for Reg128Bits<27> {}
impl Reg128BitsConcat<2, 29> for Reg128Bits<27> {}
impl Reg128BitsDownCast<3> for Reg128Bits<27> {}
impl Reg128BitsConcat<3, 30> for Reg128Bits<27> {}
impl Reg128BitsDownCast<4> for Reg128Bits<27> {}
impl Reg128BitsConcat<4, 31> for Reg128Bits<27> {}
impl Reg128BitsDownCast<5> for Reg128Bits<27> {}
impl Reg128BitsConcat<5, 32> for Reg128Bits<27> {}
impl Reg128BitsDownCast<6> for Reg128Bits<27> {}
impl Reg128BitsConcat<6, 33> for Reg128Bits<27> {}
impl Reg128BitsDownCast<7> for Reg128Bits<27> {}
impl Reg128BitsConcat<7, 34> for Reg128Bits<27> {}
impl Reg128BitsDownCast<8> for Reg128Bits<27> {}
impl Reg128BitsConcat<8, 35> for Reg128Bits<27> {}
impl Reg128BitsDownCast<9> for Reg128Bits<27> {}
impl Reg128BitsConcat<9, 36> for Reg128Bits<27> {}
impl Reg128BitsDownCast<10> for Reg128Bits<27> {}
impl Reg128BitsConcat<10, 37> for Reg128Bits<27> {}
impl Reg128BitsDownCast<11> for Reg128Bits<27> {}
impl Reg128BitsConcat<11, 38> for Reg128Bits<27> {}
impl Reg128BitsDownCast<12> for Reg128Bits<27> {}
impl Reg128BitsConcat<12, 39> for Reg128Bits<27> {}
impl Reg128BitsDownCast<13> for Reg128Bits<27> {}
impl Reg128BitsConcat<13, 40> for Reg128Bits<27> {}
impl Reg128BitsDownCast<14> for Reg128Bits<27> {}
impl Reg128BitsConcat<14, 41> for Reg128Bits<27> {}
impl Reg128BitsDownCast<15> for Reg128Bits<27> {}
impl Reg128BitsConcat<15, 42> for Reg128Bits<27> {}
impl Reg128BitsDownCast<16> for Reg128Bits<27> {}
impl Reg128BitsConcat<16, 43> for Reg128Bits<27> {}
impl Reg128BitsDownCast<17> for Reg128Bits<27> {}
impl Reg128BitsConcat<17, 44> for Reg128Bits<27> {}
impl Reg128BitsDownCast<18> for Reg128Bits<27> {}
impl Reg128BitsConcat<18, 45> for Reg128Bits<27> {}
impl Reg128BitsDownCast<19> for Reg128Bits<27> {}
impl Reg128BitsConcat<19, 46> for Reg128Bits<27> {}
impl Reg128BitsDownCast<20> for Reg128Bits<27> {}
impl Reg128BitsConcat<20, 47> for Reg128Bits<27> {}
impl Reg128BitsDownCast<21> for Reg128Bits<27> {}
impl Reg128BitsConcat<21, 48> for Reg128Bits<27> {}
impl Reg128BitsDownCast<22> for Reg128Bits<27> {}
impl Reg128BitsConcat<22, 49> for Reg128Bits<27> {}
impl Reg128BitsDownCast<23> for Reg128Bits<27> {}
impl Reg128BitsConcat<23, 50> for Reg128Bits<27> {}
impl Reg128BitsDownCast<24> for Reg128Bits<27> {}
impl Reg128BitsConcat<24, 51> for Reg128Bits<27> {}
impl Reg128BitsDownCast<25> for Reg128Bits<27> {}
impl Reg128BitsConcat<25, 52> for Reg128Bits<27> {}
impl Reg128BitsDownCast<26> for Reg128Bits<27> {}
impl Reg128BitsConcat<26, 53> for Reg128Bits<27> {}
impl Reg128BitsDownCast<27> for Reg128Bits<27> {}
impl Reg128BitsConcat<27, 54> for Reg128Bits<27> {}
impl Reg128BitsDownCast<1> for Reg128Bits<28> {}
impl Reg128BitsConcat<1, 29> for Reg128Bits<28> {}
impl Reg128BitsDownCast<2> for Reg128Bits<28> {}
impl Reg128BitsConcat<2, 30> for Reg128Bits<28> {}
impl Reg128BitsDownCast<3> for Reg128Bits<28> {}
impl Reg128BitsConcat<3, 31> for Reg128Bits<28> {}
impl Reg128BitsDownCast<4> for Reg128Bits<28> {}
impl Reg128BitsConcat<4, 32> for Reg128Bits<28> {}
impl Reg128BitsDownCast<5> for Reg128Bits<28> {}
impl Reg128BitsConcat<5, 33> for Reg128Bits<28> {}
impl Reg128BitsDownCast<6> for Reg128Bits<28> {}
impl Reg128BitsConcat<6, 34> for Reg128Bits<28> {}
impl Reg128BitsDownCast<7> for Reg128Bits<28> {}
impl Reg128BitsConcat<7, 35> for Reg128Bits<28> {}
impl Reg128BitsDownCast<8> for Reg128Bits<28> {}
impl Reg128BitsConcat<8, 36> for Reg128Bits<28> {}
impl Reg128BitsDownCast<9> for Reg128Bits<28> {}
impl Reg128BitsConcat<9, 37> for Reg128Bits<28> {}
impl Reg128BitsDownCast<10> for Reg128Bits<28> {}
impl Reg128BitsConcat<10, 38> for Reg128Bits<28> {}
impl Reg128BitsDownCast<11> for Reg128Bits<28> {}
impl Reg128BitsConcat<11, 39> for Reg128Bits<28> {}
impl Reg128BitsDownCast<12> for Reg128Bits<28> {}
impl Reg128BitsConcat<12, 40> for Reg128Bits<28> {}
impl Reg128BitsDownCast<13> for Reg128Bits<28> {}
impl Reg128BitsConcat<13, 41> for Reg128Bits<28> {}
impl Reg128BitsDownCast<14> for Reg128Bits<28> {}
impl Reg128BitsConcat<14, 42> for Reg128Bits<28> {}
impl Reg128BitsDownCast<15> for Reg128Bits<28> {}
impl Reg128BitsConcat<15, 43> for Reg128Bits<28> {}
impl Reg128BitsDownCast<16> for Reg128Bits<28> {}
impl Reg128BitsConcat<16, 44> for Reg128Bits<28> {}
impl Reg128BitsDownCast<17> for Reg128Bits<28> {}
impl Reg128BitsConcat<17, 45> for Reg128Bits<28> {}
impl Reg128BitsDownCast<18> for Reg128Bits<28> {}
impl Reg128BitsConcat<18, 46> for Reg128Bits<28> {}
impl Reg128BitsDownCast<19> for Reg128Bits<28> {}
impl Reg128BitsConcat<19, 47> for Reg128Bits<28> {}
impl Reg128BitsDownCast<20> for Reg128Bits<28> {}
impl Reg128BitsConcat<20, 48> for Reg128Bits<28> {}
impl Reg128BitsDownCast<21> for Reg128Bits<28> {}
impl Reg128BitsConcat<21, 49> for Reg128Bits<28> {}
impl Reg128BitsDownCast<22> for Reg128Bits<28> {}
impl Reg128BitsConcat<22, 50> for Reg128Bits<28> {}
impl Reg128BitsDownCast<23> for Reg128Bits<28> {}
impl Reg128BitsConcat<23, 51> for Reg128Bits<28> {}
impl Reg128BitsDownCast<24> for Reg128Bits<28> {}
impl Reg128BitsConcat<24, 52> for Reg128Bits<28> {}
impl Reg128BitsDownCast<25> for Reg128Bits<28> {}
impl Reg128BitsConcat<25, 53> for Reg128Bits<28> {}
impl Reg128BitsDownCast<26> for Reg128Bits<28> {}
impl Reg128BitsConcat<26, 54> for Reg128Bits<28> {}
impl Reg128BitsDownCast<27> for Reg128Bits<28> {}
impl Reg128BitsConcat<27, 55> for Reg128Bits<28> {}
impl Reg128BitsDownCast<28> for Reg128Bits<28> {}
impl Reg128BitsConcat<28, 56> for Reg128Bits<28> {}
impl Reg128BitsDownCast<1> for Reg128Bits<29> {}
impl Reg128BitsConcat<1, 30> for Reg128Bits<29> {}
impl Reg128BitsDownCast<2> for Reg128Bits<29> {}
impl Reg128BitsConcat<2, 31> for Reg128Bits<29> {}
impl Reg128BitsDownCast<3> for Reg128Bits<29> {}
impl Reg128BitsConcat<3, 32> for Reg128Bits<29> {}
impl Reg128BitsDownCast<4> for Reg128Bits<29> {}
impl Reg128BitsConcat<4, 33> for Reg128Bits<29> {}
impl Reg128BitsDownCast<5> for Reg128Bits<29> {}
impl Reg128BitsConcat<5, 34> for Reg128Bits<29> {}
impl Reg128BitsDownCast<6> for Reg128Bits<29> {}
impl Reg128BitsConcat<6, 35> for Reg128Bits<29> {}
impl Reg128BitsDownCast<7> for Reg128Bits<29> {}
impl Reg128BitsConcat<7, 36> for Reg128Bits<29> {}
impl Reg128BitsDownCast<8> for Reg128Bits<29> {}
impl Reg128BitsConcat<8, 37> for Reg128Bits<29> {}
impl Reg128BitsDownCast<9> for Reg128Bits<29> {}
impl Reg128BitsConcat<9, 38> for Reg128Bits<29> {}
impl Reg128BitsDownCast<10> for Reg128Bits<29> {}
impl Reg128BitsConcat<10, 39> for Reg128Bits<29> {}
impl Reg128BitsDownCast<11> for Reg128Bits<29> {}
impl Reg128BitsConcat<11, 40> for Reg128Bits<29> {}
impl Reg128BitsDownCast<12> for Reg128Bits<29> {}
impl Reg128BitsConcat<12, 41> for Reg128Bits<29> {}
impl Reg128BitsDownCast<13> for Reg128Bits<29> {}
impl Reg128BitsConcat<13, 42> for Reg128Bits<29> {}
impl Reg128BitsDownCast<14> for Reg128Bits<29> {}
impl Reg128BitsConcat<14, 43> for Reg128Bits<29> {}
impl Reg128BitsDownCast<15> for Reg128Bits<29> {}
impl Reg128BitsConcat<15, 44> for Reg128Bits<29> {}
impl Reg128BitsDownCast<16> for Reg128Bits<29> {}
impl Reg128BitsConcat<16, 45> for Reg128Bits<29> {}
impl Reg128BitsDownCast<17> for Reg128Bits<29> {}
impl Reg128BitsConcat<17, 46> for Reg128Bits<29> {}
impl Reg128BitsDownCast<18> for Reg128Bits<29> {}
impl Reg128BitsConcat<18, 47> for Reg128Bits<29> {}
impl Reg128BitsDownCast<19> for Reg128Bits<29> {}
impl Reg128BitsConcat<19, 48> for Reg128Bits<29> {}
impl Reg128BitsDownCast<20> for Reg128Bits<29> {}
impl Reg128BitsConcat<20, 49> for Reg128Bits<29> {}
impl Reg128BitsDownCast<21> for Reg128Bits<29> {}
impl Reg128BitsConcat<21, 50> for Reg128Bits<29> {}
impl Reg128BitsDownCast<22> for Reg128Bits<29> {}
impl Reg128BitsConcat<22, 51> for Reg128Bits<29> {}
impl Reg128BitsDownCast<23> for Reg128Bits<29> {}
impl Reg128BitsConcat<23, 52> for Reg128Bits<29> {}
impl Reg128BitsDownCast<24> for Reg128Bits<29> {}
impl Reg128BitsConcat<24, 53> for Reg128Bits<29> {}
impl Reg128BitsDownCast<25> for Reg128Bits<29> {}
impl Reg128BitsConcat<25, 54> for Reg128Bits<29> {}
impl Reg128BitsDownCast<26> for Reg128Bits<29> {}
impl Reg128BitsConcat<26, 55> for Reg128Bits<29> {}
impl Reg128BitsDownCast<27> for Reg128Bits<29> {}
impl Reg128BitsConcat<27, 56> for Reg128Bits<29> {}
impl Reg128BitsDownCast<28> for Reg128Bits<29> {}
impl Reg128BitsConcat<28, 57> for Reg128Bits<29> {}
impl Reg128BitsDownCast<29> for Reg128Bits<29> {}
impl Reg128BitsConcat<29, 58> for Reg128Bits<29> {}
impl Reg128BitsDownCast<1> for Reg128Bits<30> {}
impl Reg128BitsConcat<1, 31> for Reg128Bits<30> {}
impl Reg128BitsDownCast<2> for Reg128Bits<30> {}
impl Reg128BitsConcat<2, 32> for Reg128Bits<30> {}
impl Reg128BitsDownCast<3> for Reg128Bits<30> {}
impl Reg128BitsConcat<3, 33> for Reg128Bits<30> {}
impl Reg128BitsDownCast<4> for Reg128Bits<30> {}
impl Reg128BitsConcat<4, 34> for Reg128Bits<30> {}
impl Reg128BitsDownCast<5> for Reg128Bits<30> {}
impl Reg128BitsConcat<5, 35> for Reg128Bits<30> {}
impl Reg128BitsDownCast<6> for Reg128Bits<30> {}
impl Reg128BitsConcat<6, 36> for Reg128Bits<30> {}
impl Reg128BitsDownCast<7> for Reg128Bits<30> {}
impl Reg128BitsConcat<7, 37> for Reg128Bits<30> {}
impl Reg128BitsDownCast<8> for Reg128Bits<30> {}
impl Reg128BitsConcat<8, 38> for Reg128Bits<30> {}
impl Reg128BitsDownCast<9> for Reg128Bits<30> {}
impl Reg128BitsConcat<9, 39> for Reg128Bits<30> {}
impl Reg128BitsDownCast<10> for Reg128Bits<30> {}
impl Reg128BitsConcat<10, 40> for Reg128Bits<30> {}
impl Reg128BitsDownCast<11> for Reg128Bits<30> {}
impl Reg128BitsConcat<11, 41> for Reg128Bits<30> {}
impl Reg128BitsDownCast<12> for Reg128Bits<30> {}
impl Reg128BitsConcat<12, 42> for Reg128Bits<30> {}
impl Reg128BitsDownCast<13> for Reg128Bits<30> {}
impl Reg128BitsConcat<13, 43> for Reg128Bits<30> {}
impl Reg128BitsDownCast<14> for Reg128Bits<30> {}
impl Reg128BitsConcat<14, 44> for Reg128Bits<30> {}
impl Reg128BitsDownCast<15> for Reg128Bits<30> {}
impl Reg128BitsConcat<15, 45> for Reg128Bits<30> {}
impl Reg128BitsDownCast<16> for Reg128Bits<30> {}
impl Reg128BitsConcat<16, 46> for Reg128Bits<30> {}
impl Reg128BitsDownCast<17> for Reg128Bits<30> {}
impl Reg128BitsConcat<17, 47> for Reg128Bits<30> {}
impl Reg128BitsDownCast<18> for Reg128Bits<30> {}
impl Reg128BitsConcat<18, 48> for Reg128Bits<30> {}
impl Reg128BitsDownCast<19> for Reg128Bits<30> {}
impl Reg128BitsConcat<19, 49> for Reg128Bits<30> {}
impl Reg128BitsDownCast<20> for Reg128Bits<30> {}
impl Reg128BitsConcat<20, 50> for Reg128Bits<30> {}
impl Reg128BitsDownCast<21> for Reg128Bits<30> {}
impl Reg128BitsConcat<21, 51> for Reg128Bits<30> {}
impl Reg128BitsDownCast<22> for Reg128Bits<30> {}
impl Reg128BitsConcat<22, 52> for Reg128Bits<30> {}
impl Reg128BitsDownCast<23> for Reg128Bits<30> {}
impl Reg128BitsConcat<23, 53> for Reg128Bits<30> {}
impl Reg128BitsDownCast<24> for Reg128Bits<30> {}
impl Reg128BitsConcat<24, 54> for Reg128Bits<30> {}
impl Reg128BitsDownCast<25> for Reg128Bits<30> {}
impl Reg128BitsConcat<25, 55> for Reg128Bits<30> {}
impl Reg128BitsDownCast<26> for Reg128Bits<30> {}
impl Reg128BitsConcat<26, 56> for Reg128Bits<30> {}
impl Reg128BitsDownCast<27> for Reg128Bits<30> {}
impl Reg128BitsConcat<27, 57> for Reg128Bits<30> {}
impl Reg128BitsDownCast<28> for Reg128Bits<30> {}
impl Reg128BitsConcat<28, 58> for Reg128Bits<30> {}
impl Reg128BitsDownCast<29> for Reg128Bits<30> {}
impl Reg128BitsConcat<29, 59> for Reg128Bits<30> {}
impl Reg128BitsDownCast<30> for Reg128Bits<30> {}
impl Reg128BitsConcat<30, 60> for Reg128Bits<30> {}
impl Reg128BitsDownCast<1> for Reg128Bits<31> {}
impl Reg128BitsConcat<1, 32> for Reg128Bits<31> {}
impl Reg128BitsDownCast<2> for Reg128Bits<31> {}
impl Reg128BitsConcat<2, 33> for Reg128Bits<31> {}
impl Reg128BitsDownCast<3> for Reg128Bits<31> {}
impl Reg128BitsConcat<3, 34> for Reg128Bits<31> {}
impl Reg128BitsDownCast<4> for Reg128Bits<31> {}
impl Reg128BitsConcat<4, 35> for Reg128Bits<31> {}
impl Reg128BitsDownCast<5> for Reg128Bits<31> {}
impl Reg128BitsConcat<5, 36> for Reg128Bits<31> {}
impl Reg128BitsDownCast<6> for Reg128Bits<31> {}
impl Reg128BitsConcat<6, 37> for Reg128Bits<31> {}
impl Reg128BitsDownCast<7> for Reg128Bits<31> {}
impl Reg128BitsConcat<7, 38> for Reg128Bits<31> {}
impl Reg128BitsDownCast<8> for Reg128Bits<31> {}
impl Reg128BitsConcat<8, 39> for Reg128Bits<31> {}
impl Reg128BitsDownCast<9> for Reg128Bits<31> {}
impl Reg128BitsConcat<9, 40> for Reg128Bits<31> {}
impl Reg128BitsDownCast<10> for Reg128Bits<31> {}
impl Reg128BitsConcat<10, 41> for Reg128Bits<31> {}
impl Reg128BitsDownCast<11> for Reg128Bits<31> {}
impl Reg128BitsConcat<11, 42> for Reg128Bits<31> {}
impl Reg128BitsDownCast<12> for Reg128Bits<31> {}
impl Reg128BitsConcat<12, 43> for Reg128Bits<31> {}
impl Reg128BitsDownCast<13> for Reg128Bits<31> {}
impl Reg128BitsConcat<13, 44> for Reg128Bits<31> {}
impl Reg128BitsDownCast<14> for Reg128Bits<31> {}
impl Reg128BitsConcat<14, 45> for Reg128Bits<31> {}
impl Reg128BitsDownCast<15> for Reg128Bits<31> {}
impl Reg128BitsConcat<15, 46> for Reg128Bits<31> {}
impl Reg128BitsDownCast<16> for Reg128Bits<31> {}
impl Reg128BitsConcat<16, 47> for Reg128Bits<31> {}
impl Reg128BitsDownCast<17> for Reg128Bits<31> {}
impl Reg128BitsConcat<17, 48> for Reg128Bits<31> {}
impl Reg128BitsDownCast<18> for Reg128Bits<31> {}
impl Reg128BitsConcat<18, 49> for Reg128Bits<31> {}
impl Reg128BitsDownCast<19> for Reg128Bits<31> {}
impl Reg128BitsConcat<19, 50> for Reg128Bits<31> {}
impl Reg128BitsDownCast<20> for Reg128Bits<31> {}
impl Reg128BitsConcat<20, 51> for Reg128Bits<31> {}
impl Reg128BitsDownCast<21> for Reg128Bits<31> {}
impl Reg128BitsConcat<21, 52> for Reg128Bits<31> {}
impl Reg128BitsDownCast<22> for Reg128Bits<31> {}
impl Reg128BitsConcat<22, 53> for Reg128Bits<31> {}
impl Reg128BitsDownCast<23> for Reg128Bits<31> {}
impl Reg128BitsConcat<23, 54> for Reg128Bits<31> {}
impl Reg128BitsDownCast<24> for Reg128Bits<31> {}
impl Reg128BitsConcat<24, 55> for Reg128Bits<31> {}
impl Reg128BitsDownCast<25> for Reg128Bits<31> {}
impl Reg128BitsConcat<25, 56> for Reg128Bits<31> {}
impl Reg128BitsDownCast<26> for Reg128Bits<31> {}
impl Reg128BitsConcat<26, 57> for Reg128Bits<31> {}
impl Reg128BitsDownCast<27> for Reg128Bits<31> {}
impl Reg128BitsConcat<27, 58> for Reg128Bits<31> {}
impl Reg128BitsDownCast<28> for Reg128Bits<31> {}
impl Reg128BitsConcat<28, 59> for Reg128Bits<31> {}
impl Reg128BitsDownCast<29> for Reg128Bits<31> {}
impl Reg128BitsConcat<29, 60> for Reg128Bits<31> {}
impl Reg128BitsDownCast<30> for Reg128Bits<31> {}
impl Reg128BitsConcat<30, 61> for Reg128Bits<31> {}
impl Reg128BitsDownCast<31> for Reg128Bits<31> {}
impl Reg128BitsConcat<31, 62> for Reg128Bits<31> {}
impl Reg128BitsDownCast<1> for Reg128Bits<32> {}
impl Reg128BitsConcat<1, 33> for Reg128Bits<32> {}
impl Reg128BitsDownCast<2> for Reg128Bits<32> {}
impl Reg128BitsConcat<2, 34> for Reg128Bits<32> {}
impl Reg128BitsDownCast<3> for Reg128Bits<32> {}
impl Reg128BitsConcat<3, 35> for Reg128Bits<32> {}
impl Reg128BitsDownCast<4> for Reg128Bits<32> {}
impl Reg128BitsConcat<4, 36> for Reg128Bits<32> {}
impl Reg128BitsDownCast<5> for Reg128Bits<32> {}
impl Reg128BitsConcat<5, 37> for Reg128Bits<32> {}
impl Reg128BitsDownCast<6> for Reg128Bits<32> {}
impl Reg128BitsConcat<6, 38> for Reg128Bits<32> {}
impl Reg128BitsDownCast<7> for Reg128Bits<32> {}
impl Reg128BitsConcat<7, 39> for Reg128Bits<32> {}
impl Reg128BitsDownCast<8> for Reg128Bits<32> {}
impl Reg128BitsConcat<8, 40> for Reg128Bits<32> {}
impl Reg128BitsDownCast<9> for Reg128Bits<32> {}
impl Reg128BitsConcat<9, 41> for Reg128Bits<32> {}
impl Reg128BitsDownCast<10> for Reg128Bits<32> {}
impl Reg128BitsConcat<10, 42> for Reg128Bits<32> {}
impl Reg128BitsDownCast<11> for Reg128Bits<32> {}
impl Reg128BitsConcat<11, 43> for Reg128Bits<32> {}
impl Reg128BitsDownCast<12> for Reg128Bits<32> {}
impl Reg128BitsConcat<12, 44> for Reg128Bits<32> {}
impl Reg128BitsDownCast<13> for Reg128Bits<32> {}
impl Reg128BitsConcat<13, 45> for Reg128Bits<32> {}
impl Reg128BitsDownCast<14> for Reg128Bits<32> {}
impl Reg128BitsConcat<14, 46> for Reg128Bits<32> {}
impl Reg128BitsDownCast<15> for Reg128Bits<32> {}
impl Reg128BitsConcat<15, 47> for Reg128Bits<32> {}
impl Reg128BitsDownCast<16> for Reg128Bits<32> {}
impl Reg128BitsConcat<16, 48> for Reg128Bits<32> {}
impl Reg128BitsDownCast<17> for Reg128Bits<32> {}
impl Reg128BitsConcat<17, 49> for Reg128Bits<32> {}
impl Reg128BitsDownCast<18> for Reg128Bits<32> {}
impl Reg128BitsConcat<18, 50> for Reg128Bits<32> {}
impl Reg128BitsDownCast<19> for Reg128Bits<32> {}
impl Reg128BitsConcat<19, 51> for Reg128Bits<32> {}
impl Reg128BitsDownCast<20> for Reg128Bits<32> {}
impl Reg128BitsConcat<20, 52> for Reg128Bits<32> {}
impl Reg128BitsDownCast<21> for Reg128Bits<32> {}
impl Reg128BitsConcat<21, 53> for Reg128Bits<32> {}
impl Reg128BitsDownCast<22> for Reg128Bits<32> {}
impl Reg128BitsConcat<22, 54> for Reg128Bits<32> {}
impl Reg128BitsDownCast<23> for Reg128Bits<32> {}
impl Reg128BitsConcat<23, 55> for Reg128Bits<32> {}
impl Reg128BitsDownCast<24> for Reg128Bits<32> {}
impl Reg128BitsConcat<24, 56> for Reg128Bits<32> {}
impl Reg128BitsDownCast<25> for Reg128Bits<32> {}
impl Reg128BitsConcat<25, 57> for Reg128Bits<32> {}
impl Reg128BitsDownCast<26> for Reg128Bits<32> {}
impl Reg128BitsConcat<26, 58> for Reg128Bits<32> {}
impl Reg128BitsDownCast<27> for Reg128Bits<32> {}
impl Reg128BitsConcat<27, 59> for Reg128Bits<32> {}
impl Reg128BitsDownCast<28> for Reg128Bits<32> {}
impl Reg128BitsConcat<28, 60> for Reg128Bits<32> {}
impl Reg128BitsDownCast<29> for Reg128Bits<32> {}
impl Reg128BitsConcat<29, 61> for Reg128Bits<32> {}
impl Reg128BitsDownCast<30> for Reg128Bits<32> {}
impl Reg128BitsConcat<30, 62> for Reg128Bits<32> {}
impl Reg128BitsDownCast<31> for Reg128Bits<32> {}
impl Reg128BitsConcat<31, 63> for Reg128Bits<32> {}
impl Reg128BitsDownCast<32> for Reg128Bits<32> {}
impl Reg128BitsConcat<32, 64> for Reg128Bits<32> {}
impl Reg128BitsDownCast<1> for Reg128Bits<33> {}
impl Reg128BitsConcat<1, 34> for Reg128Bits<33> {}
impl Reg128BitsDownCast<2> for Reg128Bits<33> {}
impl Reg128BitsConcat<2, 35> for Reg128Bits<33> {}
impl Reg128BitsDownCast<3> for Reg128Bits<33> {}
impl Reg128BitsConcat<3, 36> for Reg128Bits<33> {}
impl Reg128BitsDownCast<4> for Reg128Bits<33> {}
impl Reg128BitsConcat<4, 37> for Reg128Bits<33> {}
impl Reg128BitsDownCast<5> for Reg128Bits<33> {}
impl Reg128BitsConcat<5, 38> for Reg128Bits<33> {}
impl Reg128BitsDownCast<6> for Reg128Bits<33> {}
impl Reg128BitsConcat<6, 39> for Reg128Bits<33> {}
impl Reg128BitsDownCast<7> for Reg128Bits<33> {}
impl Reg128BitsConcat<7, 40> for Reg128Bits<33> {}
impl Reg128BitsDownCast<8> for Reg128Bits<33> {}
impl Reg128BitsConcat<8, 41> for Reg128Bits<33> {}
impl Reg128BitsDownCast<9> for Reg128Bits<33> {}
impl Reg128BitsConcat<9, 42> for Reg128Bits<33> {}
impl Reg128BitsDownCast<10> for Reg128Bits<33> {}
impl Reg128BitsConcat<10, 43> for Reg128Bits<33> {}
impl Reg128BitsDownCast<11> for Reg128Bits<33> {}
impl Reg128BitsConcat<11, 44> for Reg128Bits<33> {}
impl Reg128BitsDownCast<12> for Reg128Bits<33> {}
impl Reg128BitsConcat<12, 45> for Reg128Bits<33> {}
impl Reg128BitsDownCast<13> for Reg128Bits<33> {}
impl Reg128BitsConcat<13, 46> for Reg128Bits<33> {}
impl Reg128BitsDownCast<14> for Reg128Bits<33> {}
impl Reg128BitsConcat<14, 47> for Reg128Bits<33> {}
impl Reg128BitsDownCast<15> for Reg128Bits<33> {}
impl Reg128BitsConcat<15, 48> for Reg128Bits<33> {}
impl Reg128BitsDownCast<16> for Reg128Bits<33> {}
impl Reg128BitsConcat<16, 49> for Reg128Bits<33> {}
impl Reg128BitsDownCast<17> for Reg128Bits<33> {}
impl Reg128BitsConcat<17, 50> for Reg128Bits<33> {}
impl Reg128BitsDownCast<18> for Reg128Bits<33> {}
impl Reg128BitsConcat<18, 51> for Reg128Bits<33> {}
impl Reg128BitsDownCast<19> for Reg128Bits<33> {}
impl Reg128BitsConcat<19, 52> for Reg128Bits<33> {}
impl Reg128BitsDownCast<20> for Reg128Bits<33> {}
impl Reg128BitsConcat<20, 53> for Reg128Bits<33> {}
impl Reg128BitsDownCast<21> for Reg128Bits<33> {}
impl Reg128BitsConcat<21, 54> for Reg128Bits<33> {}
impl Reg128BitsDownCast<22> for Reg128Bits<33> {}
impl Reg128BitsConcat<22, 55> for Reg128Bits<33> {}
impl Reg128BitsDownCast<23> for Reg128Bits<33> {}
impl Reg128BitsConcat<23, 56> for Reg128Bits<33> {}
impl Reg128BitsDownCast<24> for Reg128Bits<33> {}
impl Reg128BitsConcat<24, 57> for Reg128Bits<33> {}
impl Reg128BitsDownCast<25> for Reg128Bits<33> {}
impl Reg128BitsConcat<25, 58> for Reg128Bits<33> {}
impl Reg128BitsDownCast<26> for Reg128Bits<33> {}
impl Reg128BitsConcat<26, 59> for Reg128Bits<33> {}
impl Reg128BitsDownCast<27> for Reg128Bits<33> {}
impl Reg128BitsConcat<27, 60> for Reg128Bits<33> {}
impl Reg128BitsDownCast<28> for Reg128Bits<33> {}
impl Reg128BitsConcat<28, 61> for Reg128Bits<33> {}
impl Reg128BitsDownCast<29> for Reg128Bits<33> {}
impl Reg128BitsConcat<29, 62> for Reg128Bits<33> {}
impl Reg128BitsDownCast<30> for Reg128Bits<33> {}
impl Reg128BitsConcat<30, 63> for Reg128Bits<33> {}
impl Reg128BitsDownCast<31> for Reg128Bits<33> {}
impl Reg128BitsConcat<31, 64> for Reg128Bits<33> {}
impl Reg128BitsDownCast<32> for Reg128Bits<33> {}
impl Reg128BitsConcat<32, 65> for Reg128Bits<33> {}
impl Reg128BitsDownCast<33> for Reg128Bits<33> {}
impl Reg128BitsConcat<33, 66> for Reg128Bits<33> {}
impl Reg128BitsDownCast<1> for Reg128Bits<34> {}
impl Reg128BitsConcat<1, 35> for Reg128Bits<34> {}
impl Reg128BitsDownCast<2> for Reg128Bits<34> {}
impl Reg128BitsConcat<2, 36> for Reg128Bits<34> {}
impl Reg128BitsDownCast<3> for Reg128Bits<34> {}
impl Reg128BitsConcat<3, 37> for Reg128Bits<34> {}
impl Reg128BitsDownCast<4> for Reg128Bits<34> {}
impl Reg128BitsConcat<4, 38> for Reg128Bits<34> {}
impl Reg128BitsDownCast<5> for Reg128Bits<34> {}
impl Reg128BitsConcat<5, 39> for Reg128Bits<34> {}
impl Reg128BitsDownCast<6> for Reg128Bits<34> {}
impl Reg128BitsConcat<6, 40> for Reg128Bits<34> {}
impl Reg128BitsDownCast<7> for Reg128Bits<34> {}
impl Reg128BitsConcat<7, 41> for Reg128Bits<34> {}
impl Reg128BitsDownCast<8> for Reg128Bits<34> {}
impl Reg128BitsConcat<8, 42> for Reg128Bits<34> {}
impl Reg128BitsDownCast<9> for Reg128Bits<34> {}
impl Reg128BitsConcat<9, 43> for Reg128Bits<34> {}
impl Reg128BitsDownCast<10> for Reg128Bits<34> {}
impl Reg128BitsConcat<10, 44> for Reg128Bits<34> {}
impl Reg128BitsDownCast<11> for Reg128Bits<34> {}
impl Reg128BitsConcat<11, 45> for Reg128Bits<34> {}
impl Reg128BitsDownCast<12> for Reg128Bits<34> {}
impl Reg128BitsConcat<12, 46> for Reg128Bits<34> {}
impl Reg128BitsDownCast<13> for Reg128Bits<34> {}
impl Reg128BitsConcat<13, 47> for Reg128Bits<34> {}
impl Reg128BitsDownCast<14> for Reg128Bits<34> {}
impl Reg128BitsConcat<14, 48> for Reg128Bits<34> {}
impl Reg128BitsDownCast<15> for Reg128Bits<34> {}
impl Reg128BitsConcat<15, 49> for Reg128Bits<34> {}
impl Reg128BitsDownCast<16> for Reg128Bits<34> {}
impl Reg128BitsConcat<16, 50> for Reg128Bits<34> {}
impl Reg128BitsDownCast<17> for Reg128Bits<34> {}
impl Reg128BitsConcat<17, 51> for Reg128Bits<34> {}
impl Reg128BitsDownCast<18> for Reg128Bits<34> {}
impl Reg128BitsConcat<18, 52> for Reg128Bits<34> {}
impl Reg128BitsDownCast<19> for Reg128Bits<34> {}
impl Reg128BitsConcat<19, 53> for Reg128Bits<34> {}
impl Reg128BitsDownCast<20> for Reg128Bits<34> {}
impl Reg128BitsConcat<20, 54> for Reg128Bits<34> {}
impl Reg128BitsDownCast<21> for Reg128Bits<34> {}
impl Reg128BitsConcat<21, 55> for Reg128Bits<34> {}
impl Reg128BitsDownCast<22> for Reg128Bits<34> {}
impl Reg128BitsConcat<22, 56> for Reg128Bits<34> {}
impl Reg128BitsDownCast<23> for Reg128Bits<34> {}
impl Reg128BitsConcat<23, 57> for Reg128Bits<34> {}
impl Reg128BitsDownCast<24> for Reg128Bits<34> {}
impl Reg128BitsConcat<24, 58> for Reg128Bits<34> {}
impl Reg128BitsDownCast<25> for Reg128Bits<34> {}
impl Reg128BitsConcat<25, 59> for Reg128Bits<34> {}
impl Reg128BitsDownCast<26> for Reg128Bits<34> {}
impl Reg128BitsConcat<26, 60> for Reg128Bits<34> {}
impl Reg128BitsDownCast<27> for Reg128Bits<34> {}
impl Reg128BitsConcat<27, 61> for Reg128Bits<34> {}
impl Reg128BitsDownCast<28> for Reg128Bits<34> {}
impl Reg128BitsConcat<28, 62> for Reg128Bits<34> {}
impl Reg128BitsDownCast<29> for Reg128Bits<34> {}
impl Reg128BitsConcat<29, 63> for Reg128Bits<34> {}
impl Reg128BitsDownCast<30> for Reg128Bits<34> {}
impl Reg128BitsConcat<30, 64> for Reg128Bits<34> {}
impl Reg128BitsDownCast<31> for Reg128Bits<34> {}
impl Reg128BitsConcat<31, 65> for Reg128Bits<34> {}
impl Reg128BitsDownCast<32> for Reg128Bits<34> {}
impl Reg128BitsConcat<32, 66> for Reg128Bits<34> {}
impl Reg128BitsDownCast<33> for Reg128Bits<34> {}
impl Reg128BitsConcat<33, 67> for Reg128Bits<34> {}
impl Reg128BitsDownCast<34> for Reg128Bits<34> {}
impl Reg128BitsConcat<34, 68> for Reg128Bits<34> {}
impl Reg128BitsDownCast<1> for Reg128Bits<35> {}
impl Reg128BitsConcat<1, 36> for Reg128Bits<35> {}
impl Reg128BitsDownCast<2> for Reg128Bits<35> {}
impl Reg128BitsConcat<2, 37> for Reg128Bits<35> {}
impl Reg128BitsDownCast<3> for Reg128Bits<35> {}
impl Reg128BitsConcat<3, 38> for Reg128Bits<35> {}
impl Reg128BitsDownCast<4> for Reg128Bits<35> {}
impl Reg128BitsConcat<4, 39> for Reg128Bits<35> {}
impl Reg128BitsDownCast<5> for Reg128Bits<35> {}
impl Reg128BitsConcat<5, 40> for Reg128Bits<35> {}
impl Reg128BitsDownCast<6> for Reg128Bits<35> {}
impl Reg128BitsConcat<6, 41> for Reg128Bits<35> {}
impl Reg128BitsDownCast<7> for Reg128Bits<35> {}
impl Reg128BitsConcat<7, 42> for Reg128Bits<35> {}
impl Reg128BitsDownCast<8> for Reg128Bits<35> {}
impl Reg128BitsConcat<8, 43> for Reg128Bits<35> {}
impl Reg128BitsDownCast<9> for Reg128Bits<35> {}
impl Reg128BitsConcat<9, 44> for Reg128Bits<35> {}
impl Reg128BitsDownCast<10> for Reg128Bits<35> {}
impl Reg128BitsConcat<10, 45> for Reg128Bits<35> {}
impl Reg128BitsDownCast<11> for Reg128Bits<35> {}
impl Reg128BitsConcat<11, 46> for Reg128Bits<35> {}
impl Reg128BitsDownCast<12> for Reg128Bits<35> {}
impl Reg128BitsConcat<12, 47> for Reg128Bits<35> {}
impl Reg128BitsDownCast<13> for Reg128Bits<35> {}
impl Reg128BitsConcat<13, 48> for Reg128Bits<35> {}
impl Reg128BitsDownCast<14> for Reg128Bits<35> {}
impl Reg128BitsConcat<14, 49> for Reg128Bits<35> {}
impl Reg128BitsDownCast<15> for Reg128Bits<35> {}
impl Reg128BitsConcat<15, 50> for Reg128Bits<35> {}
impl Reg128BitsDownCast<16> for Reg128Bits<35> {}
impl Reg128BitsConcat<16, 51> for Reg128Bits<35> {}
impl Reg128BitsDownCast<17> for Reg128Bits<35> {}
impl Reg128BitsConcat<17, 52> for Reg128Bits<35> {}
impl Reg128BitsDownCast<18> for Reg128Bits<35> {}
impl Reg128BitsConcat<18, 53> for Reg128Bits<35> {}
impl Reg128BitsDownCast<19> for Reg128Bits<35> {}
impl Reg128BitsConcat<19, 54> for Reg128Bits<35> {}
impl Reg128BitsDownCast<20> for Reg128Bits<35> {}
impl Reg128BitsConcat<20, 55> for Reg128Bits<35> {}
impl Reg128BitsDownCast<21> for Reg128Bits<35> {}
impl Reg128BitsConcat<21, 56> for Reg128Bits<35> {}
impl Reg128BitsDownCast<22> for Reg128Bits<35> {}
impl Reg128BitsConcat<22, 57> for Reg128Bits<35> {}
impl Reg128BitsDownCast<23> for Reg128Bits<35> {}
impl Reg128BitsConcat<23, 58> for Reg128Bits<35> {}
impl Reg128BitsDownCast<24> for Reg128Bits<35> {}
impl Reg128BitsConcat<24, 59> for Reg128Bits<35> {}
impl Reg128BitsDownCast<25> for Reg128Bits<35> {}
impl Reg128BitsConcat<25, 60> for Reg128Bits<35> {}
impl Reg128BitsDownCast<26> for Reg128Bits<35> {}
impl Reg128BitsConcat<26, 61> for Reg128Bits<35> {}
impl Reg128BitsDownCast<27> for Reg128Bits<35> {}
impl Reg128BitsConcat<27, 62> for Reg128Bits<35> {}
impl Reg128BitsDownCast<28> for Reg128Bits<35> {}
impl Reg128BitsConcat<28, 63> for Reg128Bits<35> {}
impl Reg128BitsDownCast<29> for Reg128Bits<35> {}
impl Reg128BitsConcat<29, 64> for Reg128Bits<35> {}
impl Reg128BitsDownCast<30> for Reg128Bits<35> {}
impl Reg128BitsConcat<30, 65> for Reg128Bits<35> {}
impl Reg128BitsDownCast<31> for Reg128Bits<35> {}
impl Reg128BitsConcat<31, 66> for Reg128Bits<35> {}
impl Reg128BitsDownCast<32> for Reg128Bits<35> {}
impl Reg128BitsConcat<32, 67> for Reg128Bits<35> {}
impl Reg128BitsDownCast<33> for Reg128Bits<35> {}
impl Reg128BitsConcat<33, 68> for Reg128Bits<35> {}
impl Reg128BitsDownCast<34> for Reg128Bits<35> {}
impl Reg128BitsConcat<34, 69> for Reg128Bits<35> {}
impl Reg128BitsDownCast<35> for Reg128Bits<35> {}
impl Reg128BitsConcat<35, 70> for Reg128Bits<35> {}
impl Reg128BitsDownCast<1> for Reg128Bits<36> {}
impl Reg128BitsConcat<1, 37> for Reg128Bits<36> {}
impl Reg128BitsDownCast<2> for Reg128Bits<36> {}
impl Reg128BitsConcat<2, 38> for Reg128Bits<36> {}
impl Reg128BitsDownCast<3> for Reg128Bits<36> {}
impl Reg128BitsConcat<3, 39> for Reg128Bits<36> {}
impl Reg128BitsDownCast<4> for Reg128Bits<36> {}
impl Reg128BitsConcat<4, 40> for Reg128Bits<36> {}
impl Reg128BitsDownCast<5> for Reg128Bits<36> {}
impl Reg128BitsConcat<5, 41> for Reg128Bits<36> {}
impl Reg128BitsDownCast<6> for Reg128Bits<36> {}
impl Reg128BitsConcat<6, 42> for Reg128Bits<36> {}
impl Reg128BitsDownCast<7> for Reg128Bits<36> {}
impl Reg128BitsConcat<7, 43> for Reg128Bits<36> {}
impl Reg128BitsDownCast<8> for Reg128Bits<36> {}
impl Reg128BitsConcat<8, 44> for Reg128Bits<36> {}
impl Reg128BitsDownCast<9> for Reg128Bits<36> {}
impl Reg128BitsConcat<9, 45> for Reg128Bits<36> {}
impl Reg128BitsDownCast<10> for Reg128Bits<36> {}
impl Reg128BitsConcat<10, 46> for Reg128Bits<36> {}
impl Reg128BitsDownCast<11> for Reg128Bits<36> {}
impl Reg128BitsConcat<11, 47> for Reg128Bits<36> {}
impl Reg128BitsDownCast<12> for Reg128Bits<36> {}
impl Reg128BitsConcat<12, 48> for Reg128Bits<36> {}
impl Reg128BitsDownCast<13> for Reg128Bits<36> {}
impl Reg128BitsConcat<13, 49> for Reg128Bits<36> {}
impl Reg128BitsDownCast<14> for Reg128Bits<36> {}
impl Reg128BitsConcat<14, 50> for Reg128Bits<36> {}
impl Reg128BitsDownCast<15> for Reg128Bits<36> {}
impl Reg128BitsConcat<15, 51> for Reg128Bits<36> {}
impl Reg128BitsDownCast<16> for Reg128Bits<36> {}
impl Reg128BitsConcat<16, 52> for Reg128Bits<36> {}
impl Reg128BitsDownCast<17> for Reg128Bits<36> {}
impl Reg128BitsConcat<17, 53> for Reg128Bits<36> {}
impl Reg128BitsDownCast<18> for Reg128Bits<36> {}
impl Reg128BitsConcat<18, 54> for Reg128Bits<36> {}
impl Reg128BitsDownCast<19> for Reg128Bits<36> {}
impl Reg128BitsConcat<19, 55> for Reg128Bits<36> {}
impl Reg128BitsDownCast<20> for Reg128Bits<36> {}
impl Reg128BitsConcat<20, 56> for Reg128Bits<36> {}
impl Reg128BitsDownCast<21> for Reg128Bits<36> {}
impl Reg128BitsConcat<21, 57> for Reg128Bits<36> {}
impl Reg128BitsDownCast<22> for Reg128Bits<36> {}
impl Reg128BitsConcat<22, 58> for Reg128Bits<36> {}
impl Reg128BitsDownCast<23> for Reg128Bits<36> {}
impl Reg128BitsConcat<23, 59> for Reg128Bits<36> {}
impl Reg128BitsDownCast<24> for Reg128Bits<36> {}
impl Reg128BitsConcat<24, 60> for Reg128Bits<36> {}
impl Reg128BitsDownCast<25> for Reg128Bits<36> {}
impl Reg128BitsConcat<25, 61> for Reg128Bits<36> {}
impl Reg128BitsDownCast<26> for Reg128Bits<36> {}
impl Reg128BitsConcat<26, 62> for Reg128Bits<36> {}
impl Reg128BitsDownCast<27> for Reg128Bits<36> {}
impl Reg128BitsConcat<27, 63> for Reg128Bits<36> {}
impl Reg128BitsDownCast<28> for Reg128Bits<36> {}
impl Reg128BitsConcat<28, 64> for Reg128Bits<36> {}
impl Reg128BitsDownCast<29> for Reg128Bits<36> {}
impl Reg128BitsConcat<29, 65> for Reg128Bits<36> {}
impl Reg128BitsDownCast<30> for Reg128Bits<36> {}
impl Reg128BitsConcat<30, 66> for Reg128Bits<36> {}
impl Reg128BitsDownCast<31> for Reg128Bits<36> {}
impl Reg128BitsConcat<31, 67> for Reg128Bits<36> {}
impl Reg128BitsDownCast<32> for Reg128Bits<36> {}
impl Reg128BitsConcat<32, 68> for Reg128Bits<36> {}
impl Reg128BitsDownCast<33> for Reg128Bits<36> {}
impl Reg128BitsConcat<33, 69> for Reg128Bits<36> {}
impl Reg128BitsDownCast<34> for Reg128Bits<36> {}
impl Reg128BitsConcat<34, 70> for Reg128Bits<36> {}
impl Reg128BitsDownCast<35> for Reg128Bits<36> {}
impl Reg128BitsConcat<35, 71> for Reg128Bits<36> {}
impl Reg128BitsDownCast<36> for Reg128Bits<36> {}
impl Reg128BitsConcat<36, 72> for Reg128Bits<36> {}
impl Reg128BitsDownCast<1> for Reg128Bits<37> {}
impl Reg128BitsConcat<1, 38> for Reg128Bits<37> {}
impl Reg128BitsDownCast<2> for Reg128Bits<37> {}
impl Reg128BitsConcat<2, 39> for Reg128Bits<37> {}
impl Reg128BitsDownCast<3> for Reg128Bits<37> {}
impl Reg128BitsConcat<3, 40> for Reg128Bits<37> {}
impl Reg128BitsDownCast<4> for Reg128Bits<37> {}
impl Reg128BitsConcat<4, 41> for Reg128Bits<37> {}
impl Reg128BitsDownCast<5> for Reg128Bits<37> {}
impl Reg128BitsConcat<5, 42> for Reg128Bits<37> {}
impl Reg128BitsDownCast<6> for Reg128Bits<37> {}
impl Reg128BitsConcat<6, 43> for Reg128Bits<37> {}
impl Reg128BitsDownCast<7> for Reg128Bits<37> {}
impl Reg128BitsConcat<7, 44> for Reg128Bits<37> {}
impl Reg128BitsDownCast<8> for Reg128Bits<37> {}
impl Reg128BitsConcat<8, 45> for Reg128Bits<37> {}
impl Reg128BitsDownCast<9> for Reg128Bits<37> {}
impl Reg128BitsConcat<9, 46> for Reg128Bits<37> {}
impl Reg128BitsDownCast<10> for Reg128Bits<37> {}
impl Reg128BitsConcat<10, 47> for Reg128Bits<37> {}
impl Reg128BitsDownCast<11> for Reg128Bits<37> {}
impl Reg128BitsConcat<11, 48> for Reg128Bits<37> {}
impl Reg128BitsDownCast<12> for Reg128Bits<37> {}
impl Reg128BitsConcat<12, 49> for Reg128Bits<37> {}
impl Reg128BitsDownCast<13> for Reg128Bits<37> {}
impl Reg128BitsConcat<13, 50> for Reg128Bits<37> {}
impl Reg128BitsDownCast<14> for Reg128Bits<37> {}
impl Reg128BitsConcat<14, 51> for Reg128Bits<37> {}
impl Reg128BitsDownCast<15> for Reg128Bits<37> {}
impl Reg128BitsConcat<15, 52> for Reg128Bits<37> {}
impl Reg128BitsDownCast<16> for Reg128Bits<37> {}
impl Reg128BitsConcat<16, 53> for Reg128Bits<37> {}
impl Reg128BitsDownCast<17> for Reg128Bits<37> {}
impl Reg128BitsConcat<17, 54> for Reg128Bits<37> {}
impl Reg128BitsDownCast<18> for Reg128Bits<37> {}
impl Reg128BitsConcat<18, 55> for Reg128Bits<37> {}
impl Reg128BitsDownCast<19> for Reg128Bits<37> {}
impl Reg128BitsConcat<19, 56> for Reg128Bits<37> {}
impl Reg128BitsDownCast<20> for Reg128Bits<37> {}
impl Reg128BitsConcat<20, 57> for Reg128Bits<37> {}
impl Reg128BitsDownCast<21> for Reg128Bits<37> {}
impl Reg128BitsConcat<21, 58> for Reg128Bits<37> {}
impl Reg128BitsDownCast<22> for Reg128Bits<37> {}
impl Reg128BitsConcat<22, 59> for Reg128Bits<37> {}
impl Reg128BitsDownCast<23> for Reg128Bits<37> {}
impl Reg128BitsConcat<23, 60> for Reg128Bits<37> {}
impl Reg128BitsDownCast<24> for Reg128Bits<37> {}
impl Reg128BitsConcat<24, 61> for Reg128Bits<37> {}
impl Reg128BitsDownCast<25> for Reg128Bits<37> {}
impl Reg128BitsConcat<25, 62> for Reg128Bits<37> {}
impl Reg128BitsDownCast<26> for Reg128Bits<37> {}
impl Reg128BitsConcat<26, 63> for Reg128Bits<37> {}
impl Reg128BitsDownCast<27> for Reg128Bits<37> {}
impl Reg128BitsConcat<27, 64> for Reg128Bits<37> {}
impl Reg128BitsDownCast<28> for Reg128Bits<37> {}
impl Reg128BitsConcat<28, 65> for Reg128Bits<37> {}
impl Reg128BitsDownCast<29> for Reg128Bits<37> {}
impl Reg128BitsConcat<29, 66> for Reg128Bits<37> {}
impl Reg128BitsDownCast<30> for Reg128Bits<37> {}
impl Reg128BitsConcat<30, 67> for Reg128Bits<37> {}
impl Reg128BitsDownCast<31> for Reg128Bits<37> {}
impl Reg128BitsConcat<31, 68> for Reg128Bits<37> {}
impl Reg128BitsDownCast<32> for Reg128Bits<37> {}
impl Reg128BitsConcat<32, 69> for Reg128Bits<37> {}
impl Reg128BitsDownCast<33> for Reg128Bits<37> {}
impl Reg128BitsConcat<33, 70> for Reg128Bits<37> {}
impl Reg128BitsDownCast<34> for Reg128Bits<37> {}
impl Reg128BitsConcat<34, 71> for Reg128Bits<37> {}
impl Reg128BitsDownCast<35> for Reg128Bits<37> {}
impl Reg128BitsConcat<35, 72> for Reg128Bits<37> {}
impl Reg128BitsDownCast<36> for Reg128Bits<37> {}
impl Reg128BitsConcat<36, 73> for Reg128Bits<37> {}
impl Reg128BitsDownCast<37> for Reg128Bits<37> {}
impl Reg128BitsConcat<37, 74> for Reg128Bits<37> {}
impl Reg128BitsDownCast<1> for Reg128Bits<38> {}
impl Reg128BitsConcat<1, 39> for Reg128Bits<38> {}
impl Reg128BitsDownCast<2> for Reg128Bits<38> {}
impl Reg128BitsConcat<2, 40> for Reg128Bits<38> {}
impl Reg128BitsDownCast<3> for Reg128Bits<38> {}
impl Reg128BitsConcat<3, 41> for Reg128Bits<38> {}
impl Reg128BitsDownCast<4> for Reg128Bits<38> {}
impl Reg128BitsConcat<4, 42> for Reg128Bits<38> {}
impl Reg128BitsDownCast<5> for Reg128Bits<38> {}
impl Reg128BitsConcat<5, 43> for Reg128Bits<38> {}
impl Reg128BitsDownCast<6> for Reg128Bits<38> {}
impl Reg128BitsConcat<6, 44> for Reg128Bits<38> {}
impl Reg128BitsDownCast<7> for Reg128Bits<38> {}
impl Reg128BitsConcat<7, 45> for Reg128Bits<38> {}
impl Reg128BitsDownCast<8> for Reg128Bits<38> {}
impl Reg128BitsConcat<8, 46> for Reg128Bits<38> {}
impl Reg128BitsDownCast<9> for Reg128Bits<38> {}
impl Reg128BitsConcat<9, 47> for Reg128Bits<38> {}
impl Reg128BitsDownCast<10> for Reg128Bits<38> {}
impl Reg128BitsConcat<10, 48> for Reg128Bits<38> {}
impl Reg128BitsDownCast<11> for Reg128Bits<38> {}
impl Reg128BitsConcat<11, 49> for Reg128Bits<38> {}
impl Reg128BitsDownCast<12> for Reg128Bits<38> {}
impl Reg128BitsConcat<12, 50> for Reg128Bits<38> {}
impl Reg128BitsDownCast<13> for Reg128Bits<38> {}
impl Reg128BitsConcat<13, 51> for Reg128Bits<38> {}
impl Reg128BitsDownCast<14> for Reg128Bits<38> {}
impl Reg128BitsConcat<14, 52> for Reg128Bits<38> {}
impl Reg128BitsDownCast<15> for Reg128Bits<38> {}
impl Reg128BitsConcat<15, 53> for Reg128Bits<38> {}
impl Reg128BitsDownCast<16> for Reg128Bits<38> {}
impl Reg128BitsConcat<16, 54> for Reg128Bits<38> {}
impl Reg128BitsDownCast<17> for Reg128Bits<38> {}
impl Reg128BitsConcat<17, 55> for Reg128Bits<38> {}
impl Reg128BitsDownCast<18> for Reg128Bits<38> {}
impl Reg128BitsConcat<18, 56> for Reg128Bits<38> {}
impl Reg128BitsDownCast<19> for Reg128Bits<38> {}
impl Reg128BitsConcat<19, 57> for Reg128Bits<38> {}
impl Reg128BitsDownCast<20> for Reg128Bits<38> {}
impl Reg128BitsConcat<20, 58> for Reg128Bits<38> {}
impl Reg128BitsDownCast<21> for Reg128Bits<38> {}
impl Reg128BitsConcat<21, 59> for Reg128Bits<38> {}
impl Reg128BitsDownCast<22> for Reg128Bits<38> {}
impl Reg128BitsConcat<22, 60> for Reg128Bits<38> {}
impl Reg128BitsDownCast<23> for Reg128Bits<38> {}
impl Reg128BitsConcat<23, 61> for Reg128Bits<38> {}
impl Reg128BitsDownCast<24> for Reg128Bits<38> {}
impl Reg128BitsConcat<24, 62> for Reg128Bits<38> {}
impl Reg128BitsDownCast<25> for Reg128Bits<38> {}
impl Reg128BitsConcat<25, 63> for Reg128Bits<38> {}
impl Reg128BitsDownCast<26> for Reg128Bits<38> {}
impl Reg128BitsConcat<26, 64> for Reg128Bits<38> {}
impl Reg128BitsDownCast<27> for Reg128Bits<38> {}
impl Reg128BitsConcat<27, 65> for Reg128Bits<38> {}
impl Reg128BitsDownCast<28> for Reg128Bits<38> {}
impl Reg128BitsConcat<28, 66> for Reg128Bits<38> {}
impl Reg128BitsDownCast<29> for Reg128Bits<38> {}
impl Reg128BitsConcat<29, 67> for Reg128Bits<38> {}
impl Reg128BitsDownCast<30> for Reg128Bits<38> {}
impl Reg128BitsConcat<30, 68> for Reg128Bits<38> {}
impl Reg128BitsDownCast<31> for Reg128Bits<38> {}
impl Reg128BitsConcat<31, 69> for Reg128Bits<38> {}
impl Reg128BitsDownCast<32> for Reg128Bits<38> {}
impl Reg128BitsConcat<32, 70> for Reg128Bits<38> {}
impl Reg128BitsDownCast<33> for Reg128Bits<38> {}
impl Reg128BitsConcat<33, 71> for Reg128Bits<38> {}
impl Reg128BitsDownCast<34> for Reg128Bits<38> {}
impl Reg128BitsConcat<34, 72> for Reg128Bits<38> {}
impl Reg128BitsDownCast<35> for Reg128Bits<38> {}
impl Reg128BitsConcat<35, 73> for Reg128Bits<38> {}
impl Reg128BitsDownCast<36> for Reg128Bits<38> {}
impl Reg128BitsConcat<36, 74> for Reg128Bits<38> {}
impl Reg128BitsDownCast<37> for Reg128Bits<38> {}
impl Reg128BitsConcat<37, 75> for Reg128Bits<38> {}
impl Reg128BitsDownCast<38> for Reg128Bits<38> {}
impl Reg128BitsConcat<38, 76> for Reg128Bits<38> {}
impl Reg128BitsDownCast<1> for Reg128Bits<39> {}
impl Reg128BitsConcat<1, 40> for Reg128Bits<39> {}
impl Reg128BitsDownCast<2> for Reg128Bits<39> {}
impl Reg128BitsConcat<2, 41> for Reg128Bits<39> {}
impl Reg128BitsDownCast<3> for Reg128Bits<39> {}
impl Reg128BitsConcat<3, 42> for Reg128Bits<39> {}
impl Reg128BitsDownCast<4> for Reg128Bits<39> {}
impl Reg128BitsConcat<4, 43> for Reg128Bits<39> {}
impl Reg128BitsDownCast<5> for Reg128Bits<39> {}
impl Reg128BitsConcat<5, 44> for Reg128Bits<39> {}
impl Reg128BitsDownCast<6> for Reg128Bits<39> {}
impl Reg128BitsConcat<6, 45> for Reg128Bits<39> {}
impl Reg128BitsDownCast<7> for Reg128Bits<39> {}
impl Reg128BitsConcat<7, 46> for Reg128Bits<39> {}
impl Reg128BitsDownCast<8> for Reg128Bits<39> {}
impl Reg128BitsConcat<8, 47> for Reg128Bits<39> {}
impl Reg128BitsDownCast<9> for Reg128Bits<39> {}
impl Reg128BitsConcat<9, 48> for Reg128Bits<39> {}
impl Reg128BitsDownCast<10> for Reg128Bits<39> {}
impl Reg128BitsConcat<10, 49> for Reg128Bits<39> {}
impl Reg128BitsDownCast<11> for Reg128Bits<39> {}
impl Reg128BitsConcat<11, 50> for Reg128Bits<39> {}
impl Reg128BitsDownCast<12> for Reg128Bits<39> {}
impl Reg128BitsConcat<12, 51> for Reg128Bits<39> {}
impl Reg128BitsDownCast<13> for Reg128Bits<39> {}
impl Reg128BitsConcat<13, 52> for Reg128Bits<39> {}
impl Reg128BitsDownCast<14> for Reg128Bits<39> {}
impl Reg128BitsConcat<14, 53> for Reg128Bits<39> {}
impl Reg128BitsDownCast<15> for Reg128Bits<39> {}
impl Reg128BitsConcat<15, 54> for Reg128Bits<39> {}
impl Reg128BitsDownCast<16> for Reg128Bits<39> {}
impl Reg128BitsConcat<16, 55> for Reg128Bits<39> {}
impl Reg128BitsDownCast<17> for Reg128Bits<39> {}
impl Reg128BitsConcat<17, 56> for Reg128Bits<39> {}
impl Reg128BitsDownCast<18> for Reg128Bits<39> {}
impl Reg128BitsConcat<18, 57> for Reg128Bits<39> {}
impl Reg128BitsDownCast<19> for Reg128Bits<39> {}
impl Reg128BitsConcat<19, 58> for Reg128Bits<39> {}
impl Reg128BitsDownCast<20> for Reg128Bits<39> {}
impl Reg128BitsConcat<20, 59> for Reg128Bits<39> {}
impl Reg128BitsDownCast<21> for Reg128Bits<39> {}
impl Reg128BitsConcat<21, 60> for Reg128Bits<39> {}
impl Reg128BitsDownCast<22> for Reg128Bits<39> {}
impl Reg128BitsConcat<22, 61> for Reg128Bits<39> {}
impl Reg128BitsDownCast<23> for Reg128Bits<39> {}
impl Reg128BitsConcat<23, 62> for Reg128Bits<39> {}
impl Reg128BitsDownCast<24> for Reg128Bits<39> {}
impl Reg128BitsConcat<24, 63> for Reg128Bits<39> {}
impl Reg128BitsDownCast<25> for Reg128Bits<39> {}
impl Reg128BitsConcat<25, 64> for Reg128Bits<39> {}
impl Reg128BitsDownCast<26> for Reg128Bits<39> {}
impl Reg128BitsConcat<26, 65> for Reg128Bits<39> {}
impl Reg128BitsDownCast<27> for Reg128Bits<39> {}
impl Reg128BitsConcat<27, 66> for Reg128Bits<39> {}
impl Reg128BitsDownCast<28> for Reg128Bits<39> {}
impl Reg128BitsConcat<28, 67> for Reg128Bits<39> {}
impl Reg128BitsDownCast<29> for Reg128Bits<39> {}
impl Reg128BitsConcat<29, 68> for Reg128Bits<39> {}
impl Reg128BitsDownCast<30> for Reg128Bits<39> {}
impl Reg128BitsConcat<30, 69> for Reg128Bits<39> {}
impl Reg128BitsDownCast<31> for Reg128Bits<39> {}
impl Reg128BitsConcat<31, 70> for Reg128Bits<39> {}
impl Reg128BitsDownCast<32> for Reg128Bits<39> {}
impl Reg128BitsConcat<32, 71> for Reg128Bits<39> {}
impl Reg128BitsDownCast<33> for Reg128Bits<39> {}
impl Reg128BitsConcat<33, 72> for Reg128Bits<39> {}
impl Reg128BitsDownCast<34> for Reg128Bits<39> {}
impl Reg128BitsConcat<34, 73> for Reg128Bits<39> {}
impl Reg128BitsDownCast<35> for Reg128Bits<39> {}
impl Reg128BitsConcat<35, 74> for Reg128Bits<39> {}
impl Reg128BitsDownCast<36> for Reg128Bits<39> {}
impl Reg128BitsConcat<36, 75> for Reg128Bits<39> {}
impl Reg128BitsDownCast<37> for Reg128Bits<39> {}
impl Reg128BitsConcat<37, 76> for Reg128Bits<39> {}
impl Reg128BitsDownCast<38> for Reg128Bits<39> {}
impl Reg128BitsConcat<38, 77> for Reg128Bits<39> {}
impl Reg128BitsDownCast<39> for Reg128Bits<39> {}
impl Reg128BitsConcat<39, 78> for Reg128Bits<39> {}
impl Reg128BitsDownCast<1> for Reg128Bits<40> {}
impl Reg128BitsConcat<1, 41> for Reg128Bits<40> {}
impl Reg128BitsDownCast<2> for Reg128Bits<40> {}
impl Reg128BitsConcat<2, 42> for Reg128Bits<40> {}
impl Reg128BitsDownCast<3> for Reg128Bits<40> {}
impl Reg128BitsConcat<3, 43> for Reg128Bits<40> {}
impl Reg128BitsDownCast<4> for Reg128Bits<40> {}
impl Reg128BitsConcat<4, 44> for Reg128Bits<40> {}
impl Reg128BitsDownCast<5> for Reg128Bits<40> {}
impl Reg128BitsConcat<5, 45> for Reg128Bits<40> {}
impl Reg128BitsDownCast<6> for Reg128Bits<40> {}
impl Reg128BitsConcat<6, 46> for Reg128Bits<40> {}
impl Reg128BitsDownCast<7> for Reg128Bits<40> {}
impl Reg128BitsConcat<7, 47> for Reg128Bits<40> {}
impl Reg128BitsDownCast<8> for Reg128Bits<40> {}
impl Reg128BitsConcat<8, 48> for Reg128Bits<40> {}
impl Reg128BitsDownCast<9> for Reg128Bits<40> {}
impl Reg128BitsConcat<9, 49> for Reg128Bits<40> {}
impl Reg128BitsDownCast<10> for Reg128Bits<40> {}
impl Reg128BitsConcat<10, 50> for Reg128Bits<40> {}
impl Reg128BitsDownCast<11> for Reg128Bits<40> {}
impl Reg128BitsConcat<11, 51> for Reg128Bits<40> {}
impl Reg128BitsDownCast<12> for Reg128Bits<40> {}
impl Reg128BitsConcat<12, 52> for Reg128Bits<40> {}
impl Reg128BitsDownCast<13> for Reg128Bits<40> {}
impl Reg128BitsConcat<13, 53> for Reg128Bits<40> {}
impl Reg128BitsDownCast<14> for Reg128Bits<40> {}
impl Reg128BitsConcat<14, 54> for Reg128Bits<40> {}
impl Reg128BitsDownCast<15> for Reg128Bits<40> {}
impl Reg128BitsConcat<15, 55> for Reg128Bits<40> {}
impl Reg128BitsDownCast<16> for Reg128Bits<40> {}
impl Reg128BitsConcat<16, 56> for Reg128Bits<40> {}
impl Reg128BitsDownCast<17> for Reg128Bits<40> {}
impl Reg128BitsConcat<17, 57> for Reg128Bits<40> {}
impl Reg128BitsDownCast<18> for Reg128Bits<40> {}
impl Reg128BitsConcat<18, 58> for Reg128Bits<40> {}
impl Reg128BitsDownCast<19> for Reg128Bits<40> {}
impl Reg128BitsConcat<19, 59> for Reg128Bits<40> {}
impl Reg128BitsDownCast<20> for Reg128Bits<40> {}
impl Reg128BitsConcat<20, 60> for Reg128Bits<40> {}
impl Reg128BitsDownCast<21> for Reg128Bits<40> {}
impl Reg128BitsConcat<21, 61> for Reg128Bits<40> {}
impl Reg128BitsDownCast<22> for Reg128Bits<40> {}
impl Reg128BitsConcat<22, 62> for Reg128Bits<40> {}
impl Reg128BitsDownCast<23> for Reg128Bits<40> {}
impl Reg128BitsConcat<23, 63> for Reg128Bits<40> {}
impl Reg128BitsDownCast<24> for Reg128Bits<40> {}
impl Reg128BitsConcat<24, 64> for Reg128Bits<40> {}
impl Reg128BitsDownCast<25> for Reg128Bits<40> {}
impl Reg128BitsConcat<25, 65> for Reg128Bits<40> {}
impl Reg128BitsDownCast<26> for Reg128Bits<40> {}
impl Reg128BitsConcat<26, 66> for Reg128Bits<40> {}
impl Reg128BitsDownCast<27> for Reg128Bits<40> {}
impl Reg128BitsConcat<27, 67> for Reg128Bits<40> {}
impl Reg128BitsDownCast<28> for Reg128Bits<40> {}
impl Reg128BitsConcat<28, 68> for Reg128Bits<40> {}
impl Reg128BitsDownCast<29> for Reg128Bits<40> {}
impl Reg128BitsConcat<29, 69> for Reg128Bits<40> {}
impl Reg128BitsDownCast<30> for Reg128Bits<40> {}
impl Reg128BitsConcat<30, 70> for Reg128Bits<40> {}
impl Reg128BitsDownCast<31> for Reg128Bits<40> {}
impl Reg128BitsConcat<31, 71> for Reg128Bits<40> {}
impl Reg128BitsDownCast<32> for Reg128Bits<40> {}
impl Reg128BitsConcat<32, 72> for Reg128Bits<40> {}
impl Reg128BitsDownCast<33> for Reg128Bits<40> {}
impl Reg128BitsConcat<33, 73> for Reg128Bits<40> {}
impl Reg128BitsDownCast<34> for Reg128Bits<40> {}
impl Reg128BitsConcat<34, 74> for Reg128Bits<40> {}
impl Reg128BitsDownCast<35> for Reg128Bits<40> {}
impl Reg128BitsConcat<35, 75> for Reg128Bits<40> {}
impl Reg128BitsDownCast<36> for Reg128Bits<40> {}
impl Reg128BitsConcat<36, 76> for Reg128Bits<40> {}
impl Reg128BitsDownCast<37> for Reg128Bits<40> {}
impl Reg128BitsConcat<37, 77> for Reg128Bits<40> {}
impl Reg128BitsDownCast<38> for Reg128Bits<40> {}
impl Reg128BitsConcat<38, 78> for Reg128Bits<40> {}
impl Reg128BitsDownCast<39> for Reg128Bits<40> {}
impl Reg128BitsConcat<39, 79> for Reg128Bits<40> {}
impl Reg128BitsDownCast<40> for Reg128Bits<40> {}
impl Reg128BitsConcat<40, 80> for Reg128Bits<40> {}
impl Reg128BitsDownCast<1> for Reg128Bits<41> {}
impl Reg128BitsConcat<1, 42> for Reg128Bits<41> {}
impl Reg128BitsDownCast<2> for Reg128Bits<41> {}
impl Reg128BitsConcat<2, 43> for Reg128Bits<41> {}
impl Reg128BitsDownCast<3> for Reg128Bits<41> {}
impl Reg128BitsConcat<3, 44> for Reg128Bits<41> {}
impl Reg128BitsDownCast<4> for Reg128Bits<41> {}
impl Reg128BitsConcat<4, 45> for Reg128Bits<41> {}
impl Reg128BitsDownCast<5> for Reg128Bits<41> {}
impl Reg128BitsConcat<5, 46> for Reg128Bits<41> {}
impl Reg128BitsDownCast<6> for Reg128Bits<41> {}
impl Reg128BitsConcat<6, 47> for Reg128Bits<41> {}
impl Reg128BitsDownCast<7> for Reg128Bits<41> {}
impl Reg128BitsConcat<7, 48> for Reg128Bits<41> {}
impl Reg128BitsDownCast<8> for Reg128Bits<41> {}
impl Reg128BitsConcat<8, 49> for Reg128Bits<41> {}
impl Reg128BitsDownCast<9> for Reg128Bits<41> {}
impl Reg128BitsConcat<9, 50> for Reg128Bits<41> {}
impl Reg128BitsDownCast<10> for Reg128Bits<41> {}
impl Reg128BitsConcat<10, 51> for Reg128Bits<41> {}
impl Reg128BitsDownCast<11> for Reg128Bits<41> {}
impl Reg128BitsConcat<11, 52> for Reg128Bits<41> {}
impl Reg128BitsDownCast<12> for Reg128Bits<41> {}
impl Reg128BitsConcat<12, 53> for Reg128Bits<41> {}
impl Reg128BitsDownCast<13> for Reg128Bits<41> {}
impl Reg128BitsConcat<13, 54> for Reg128Bits<41> {}
impl Reg128BitsDownCast<14> for Reg128Bits<41> {}
impl Reg128BitsConcat<14, 55> for Reg128Bits<41> {}
impl Reg128BitsDownCast<15> for Reg128Bits<41> {}
impl Reg128BitsConcat<15, 56> for Reg128Bits<41> {}
impl Reg128BitsDownCast<16> for Reg128Bits<41> {}
impl Reg128BitsConcat<16, 57> for Reg128Bits<41> {}
impl Reg128BitsDownCast<17> for Reg128Bits<41> {}
impl Reg128BitsConcat<17, 58> for Reg128Bits<41> {}
impl Reg128BitsDownCast<18> for Reg128Bits<41> {}
impl Reg128BitsConcat<18, 59> for Reg128Bits<41> {}
impl Reg128BitsDownCast<19> for Reg128Bits<41> {}
impl Reg128BitsConcat<19, 60> for Reg128Bits<41> {}
impl Reg128BitsDownCast<20> for Reg128Bits<41> {}
impl Reg128BitsConcat<20, 61> for Reg128Bits<41> {}
impl Reg128BitsDownCast<21> for Reg128Bits<41> {}
impl Reg128BitsConcat<21, 62> for Reg128Bits<41> {}
impl Reg128BitsDownCast<22> for Reg128Bits<41> {}
impl Reg128BitsConcat<22, 63> for Reg128Bits<41> {}
impl Reg128BitsDownCast<23> for Reg128Bits<41> {}
impl Reg128BitsConcat<23, 64> for Reg128Bits<41> {}
impl Reg128BitsDownCast<24> for Reg128Bits<41> {}
impl Reg128BitsConcat<24, 65> for Reg128Bits<41> {}
impl Reg128BitsDownCast<25> for Reg128Bits<41> {}
impl Reg128BitsConcat<25, 66> for Reg128Bits<41> {}
impl Reg128BitsDownCast<26> for Reg128Bits<41> {}
impl Reg128BitsConcat<26, 67> for Reg128Bits<41> {}
impl Reg128BitsDownCast<27> for Reg128Bits<41> {}
impl Reg128BitsConcat<27, 68> for Reg128Bits<41> {}
impl Reg128BitsDownCast<28> for Reg128Bits<41> {}
impl Reg128BitsConcat<28, 69> for Reg128Bits<41> {}
impl Reg128BitsDownCast<29> for Reg128Bits<41> {}
impl Reg128BitsConcat<29, 70> for Reg128Bits<41> {}
impl Reg128BitsDownCast<30> for Reg128Bits<41> {}
impl Reg128BitsConcat<30, 71> for Reg128Bits<41> {}
impl Reg128BitsDownCast<31> for Reg128Bits<41> {}
impl Reg128BitsConcat<31, 72> for Reg128Bits<41> {}
impl Reg128BitsDownCast<32> for Reg128Bits<41> {}
impl Reg128BitsConcat<32, 73> for Reg128Bits<41> {}
impl Reg128BitsDownCast<33> for Reg128Bits<41> {}
impl Reg128BitsConcat<33, 74> for Reg128Bits<41> {}
impl Reg128BitsDownCast<34> for Reg128Bits<41> {}
impl Reg128BitsConcat<34, 75> for Reg128Bits<41> {}
impl Reg128BitsDownCast<35> for Reg128Bits<41> {}
impl Reg128BitsConcat<35, 76> for Reg128Bits<41> {}
impl Reg128BitsDownCast<36> for Reg128Bits<41> {}
impl Reg128BitsConcat<36, 77> for Reg128Bits<41> {}
impl Reg128BitsDownCast<37> for Reg128Bits<41> {}
impl Reg128BitsConcat<37, 78> for Reg128Bits<41> {}
impl Reg128BitsDownCast<38> for Reg128Bits<41> {}
impl Reg128BitsConcat<38, 79> for Reg128Bits<41> {}
impl Reg128BitsDownCast<39> for Reg128Bits<41> {}
impl Reg128BitsConcat<39, 80> for Reg128Bits<41> {}
impl Reg128BitsDownCast<40> for Reg128Bits<41> {}
impl Reg128BitsConcat<40, 81> for Reg128Bits<41> {}
impl Reg128BitsDownCast<41> for Reg128Bits<41> {}
impl Reg128BitsConcat<41, 82> for Reg128Bits<41> {}
impl Reg128BitsDownCast<1> for Reg128Bits<42> {}
impl Reg128BitsConcat<1, 43> for Reg128Bits<42> {}
impl Reg128BitsDownCast<2> for Reg128Bits<42> {}
impl Reg128BitsConcat<2, 44> for Reg128Bits<42> {}
impl Reg128BitsDownCast<3> for Reg128Bits<42> {}
impl Reg128BitsConcat<3, 45> for Reg128Bits<42> {}
impl Reg128BitsDownCast<4> for Reg128Bits<42> {}
impl Reg128BitsConcat<4, 46> for Reg128Bits<42> {}
impl Reg128BitsDownCast<5> for Reg128Bits<42> {}
impl Reg128BitsConcat<5, 47> for Reg128Bits<42> {}
impl Reg128BitsDownCast<6> for Reg128Bits<42> {}
impl Reg128BitsConcat<6, 48> for Reg128Bits<42> {}
impl Reg128BitsDownCast<7> for Reg128Bits<42> {}
impl Reg128BitsConcat<7, 49> for Reg128Bits<42> {}
impl Reg128BitsDownCast<8> for Reg128Bits<42> {}
impl Reg128BitsConcat<8, 50> for Reg128Bits<42> {}
impl Reg128BitsDownCast<9> for Reg128Bits<42> {}
impl Reg128BitsConcat<9, 51> for Reg128Bits<42> {}
impl Reg128BitsDownCast<10> for Reg128Bits<42> {}
impl Reg128BitsConcat<10, 52> for Reg128Bits<42> {}
impl Reg128BitsDownCast<11> for Reg128Bits<42> {}
impl Reg128BitsConcat<11, 53> for Reg128Bits<42> {}
impl Reg128BitsDownCast<12> for Reg128Bits<42> {}
impl Reg128BitsConcat<12, 54> for Reg128Bits<42> {}
impl Reg128BitsDownCast<13> for Reg128Bits<42> {}
impl Reg128BitsConcat<13, 55> for Reg128Bits<42> {}
impl Reg128BitsDownCast<14> for Reg128Bits<42> {}
impl Reg128BitsConcat<14, 56> for Reg128Bits<42> {}
impl Reg128BitsDownCast<15> for Reg128Bits<42> {}
impl Reg128BitsConcat<15, 57> for Reg128Bits<42> {}
impl Reg128BitsDownCast<16> for Reg128Bits<42> {}
impl Reg128BitsConcat<16, 58> for Reg128Bits<42> {}
impl Reg128BitsDownCast<17> for Reg128Bits<42> {}
impl Reg128BitsConcat<17, 59> for Reg128Bits<42> {}
impl Reg128BitsDownCast<18> for Reg128Bits<42> {}
impl Reg128BitsConcat<18, 60> for Reg128Bits<42> {}
impl Reg128BitsDownCast<19> for Reg128Bits<42> {}
impl Reg128BitsConcat<19, 61> for Reg128Bits<42> {}
impl Reg128BitsDownCast<20> for Reg128Bits<42> {}
impl Reg128BitsConcat<20, 62> for Reg128Bits<42> {}
impl Reg128BitsDownCast<21> for Reg128Bits<42> {}
impl Reg128BitsConcat<21, 63> for Reg128Bits<42> {}
impl Reg128BitsDownCast<22> for Reg128Bits<42> {}
impl Reg128BitsConcat<22, 64> for Reg128Bits<42> {}
impl Reg128BitsDownCast<23> for Reg128Bits<42> {}
impl Reg128BitsConcat<23, 65> for Reg128Bits<42> {}
impl Reg128BitsDownCast<24> for Reg128Bits<42> {}
impl Reg128BitsConcat<24, 66> for Reg128Bits<42> {}
impl Reg128BitsDownCast<25> for Reg128Bits<42> {}
impl Reg128BitsConcat<25, 67> for Reg128Bits<42> {}
impl Reg128BitsDownCast<26> for Reg128Bits<42> {}
impl Reg128BitsConcat<26, 68> for Reg128Bits<42> {}
impl Reg128BitsDownCast<27> for Reg128Bits<42> {}
impl Reg128BitsConcat<27, 69> for Reg128Bits<42> {}
impl Reg128BitsDownCast<28> for Reg128Bits<42> {}
impl Reg128BitsConcat<28, 70> for Reg128Bits<42> {}
impl Reg128BitsDownCast<29> for Reg128Bits<42> {}
impl Reg128BitsConcat<29, 71> for Reg128Bits<42> {}
impl Reg128BitsDownCast<30> for Reg128Bits<42> {}
impl Reg128BitsConcat<30, 72> for Reg128Bits<42> {}
impl Reg128BitsDownCast<31> for Reg128Bits<42> {}
impl Reg128BitsConcat<31, 73> for Reg128Bits<42> {}
impl Reg128BitsDownCast<32> for Reg128Bits<42> {}
impl Reg128BitsConcat<32, 74> for Reg128Bits<42> {}
impl Reg128BitsDownCast<33> for Reg128Bits<42> {}
impl Reg128BitsConcat<33, 75> for Reg128Bits<42> {}
impl Reg128BitsDownCast<34> for Reg128Bits<42> {}
impl Reg128BitsConcat<34, 76> for Reg128Bits<42> {}
impl Reg128BitsDownCast<35> for Reg128Bits<42> {}
impl Reg128BitsConcat<35, 77> for Reg128Bits<42> {}
impl Reg128BitsDownCast<36> for Reg128Bits<42> {}
impl Reg128BitsConcat<36, 78> for Reg128Bits<42> {}
impl Reg128BitsDownCast<37> for Reg128Bits<42> {}
impl Reg128BitsConcat<37, 79> for Reg128Bits<42> {}
impl Reg128BitsDownCast<38> for Reg128Bits<42> {}
impl Reg128BitsConcat<38, 80> for Reg128Bits<42> {}
impl Reg128BitsDownCast<39> for Reg128Bits<42> {}
impl Reg128BitsConcat<39, 81> for Reg128Bits<42> {}
impl Reg128BitsDownCast<40> for Reg128Bits<42> {}
impl Reg128BitsConcat<40, 82> for Reg128Bits<42> {}
impl Reg128BitsDownCast<41> for Reg128Bits<42> {}
impl Reg128BitsConcat<41, 83> for Reg128Bits<42> {}
impl Reg128BitsDownCast<42> for Reg128Bits<42> {}
impl Reg128BitsConcat<42, 84> for Reg128Bits<42> {}
impl Reg128BitsDownCast<1> for Reg128Bits<43> {}
impl Reg128BitsConcat<1, 44> for Reg128Bits<43> {}
impl Reg128BitsDownCast<2> for Reg128Bits<43> {}
impl Reg128BitsConcat<2, 45> for Reg128Bits<43> {}
impl Reg128BitsDownCast<3> for Reg128Bits<43> {}
impl Reg128BitsConcat<3, 46> for Reg128Bits<43> {}
impl Reg128BitsDownCast<4> for Reg128Bits<43> {}
impl Reg128BitsConcat<4, 47> for Reg128Bits<43> {}
impl Reg128BitsDownCast<5> for Reg128Bits<43> {}
impl Reg128BitsConcat<5, 48> for Reg128Bits<43> {}
impl Reg128BitsDownCast<6> for Reg128Bits<43> {}
impl Reg128BitsConcat<6, 49> for Reg128Bits<43> {}
impl Reg128BitsDownCast<7> for Reg128Bits<43> {}
impl Reg128BitsConcat<7, 50> for Reg128Bits<43> {}
impl Reg128BitsDownCast<8> for Reg128Bits<43> {}
impl Reg128BitsConcat<8, 51> for Reg128Bits<43> {}
impl Reg128BitsDownCast<9> for Reg128Bits<43> {}
impl Reg128BitsConcat<9, 52> for Reg128Bits<43> {}
impl Reg128BitsDownCast<10> for Reg128Bits<43> {}
impl Reg128BitsConcat<10, 53> for Reg128Bits<43> {}
impl Reg128BitsDownCast<11> for Reg128Bits<43> {}
impl Reg128BitsConcat<11, 54> for Reg128Bits<43> {}
impl Reg128BitsDownCast<12> for Reg128Bits<43> {}
impl Reg128BitsConcat<12, 55> for Reg128Bits<43> {}
impl Reg128BitsDownCast<13> for Reg128Bits<43> {}
impl Reg128BitsConcat<13, 56> for Reg128Bits<43> {}
impl Reg128BitsDownCast<14> for Reg128Bits<43> {}
impl Reg128BitsConcat<14, 57> for Reg128Bits<43> {}
impl Reg128BitsDownCast<15> for Reg128Bits<43> {}
impl Reg128BitsConcat<15, 58> for Reg128Bits<43> {}
impl Reg128BitsDownCast<16> for Reg128Bits<43> {}
impl Reg128BitsConcat<16, 59> for Reg128Bits<43> {}
impl Reg128BitsDownCast<17> for Reg128Bits<43> {}
impl Reg128BitsConcat<17, 60> for Reg128Bits<43> {}
impl Reg128BitsDownCast<18> for Reg128Bits<43> {}
impl Reg128BitsConcat<18, 61> for Reg128Bits<43> {}
impl Reg128BitsDownCast<19> for Reg128Bits<43> {}
impl Reg128BitsConcat<19, 62> for Reg128Bits<43> {}
impl Reg128BitsDownCast<20> for Reg128Bits<43> {}
impl Reg128BitsConcat<20, 63> for Reg128Bits<43> {}
impl Reg128BitsDownCast<21> for Reg128Bits<43> {}
impl Reg128BitsConcat<21, 64> for Reg128Bits<43> {}
impl Reg128BitsDownCast<22> for Reg128Bits<43> {}
impl Reg128BitsConcat<22, 65> for Reg128Bits<43> {}
impl Reg128BitsDownCast<23> for Reg128Bits<43> {}
impl Reg128BitsConcat<23, 66> for Reg128Bits<43> {}
impl Reg128BitsDownCast<24> for Reg128Bits<43> {}
impl Reg128BitsConcat<24, 67> for Reg128Bits<43> {}
impl Reg128BitsDownCast<25> for Reg128Bits<43> {}
impl Reg128BitsConcat<25, 68> for Reg128Bits<43> {}
impl Reg128BitsDownCast<26> for Reg128Bits<43> {}
impl Reg128BitsConcat<26, 69> for Reg128Bits<43> {}
impl Reg128BitsDownCast<27> for Reg128Bits<43> {}
impl Reg128BitsConcat<27, 70> for Reg128Bits<43> {}
impl Reg128BitsDownCast<28> for Reg128Bits<43> {}
impl Reg128BitsConcat<28, 71> for Reg128Bits<43> {}
impl Reg128BitsDownCast<29> for Reg128Bits<43> {}
impl Reg128BitsConcat<29, 72> for Reg128Bits<43> {}
impl Reg128BitsDownCast<30> for Reg128Bits<43> {}
impl Reg128BitsConcat<30, 73> for Reg128Bits<43> {}
impl Reg128BitsDownCast<31> for Reg128Bits<43> {}
impl Reg128BitsConcat<31, 74> for Reg128Bits<43> {}
impl Reg128BitsDownCast<32> for Reg128Bits<43> {}
impl Reg128BitsConcat<32, 75> for Reg128Bits<43> {}
impl Reg128BitsDownCast<33> for Reg128Bits<43> {}
impl Reg128BitsConcat<33, 76> for Reg128Bits<43> {}
impl Reg128BitsDownCast<34> for Reg128Bits<43> {}
impl Reg128BitsConcat<34, 77> for Reg128Bits<43> {}
impl Reg128BitsDownCast<35> for Reg128Bits<43> {}
impl Reg128BitsConcat<35, 78> for Reg128Bits<43> {}
impl Reg128BitsDownCast<36> for Reg128Bits<43> {}
impl Reg128BitsConcat<36, 79> for Reg128Bits<43> {}
impl Reg128BitsDownCast<37> for Reg128Bits<43> {}
impl Reg128BitsConcat<37, 80> for Reg128Bits<43> {}
impl Reg128BitsDownCast<38> for Reg128Bits<43> {}
impl Reg128BitsConcat<38, 81> for Reg128Bits<43> {}
impl Reg128BitsDownCast<39> for Reg128Bits<43> {}
impl Reg128BitsConcat<39, 82> for Reg128Bits<43> {}
impl Reg128BitsDownCast<40> for Reg128Bits<43> {}
impl Reg128BitsConcat<40, 83> for Reg128Bits<43> {}
impl Reg128BitsDownCast<41> for Reg128Bits<43> {}
impl Reg128BitsConcat<41, 84> for Reg128Bits<43> {}
impl Reg128BitsDownCast<42> for Reg128Bits<43> {}
impl Reg128BitsConcat<42, 85> for Reg128Bits<43> {}
impl Reg128BitsDownCast<43> for Reg128Bits<43> {}
impl Reg128BitsConcat<43, 86> for Reg128Bits<43> {}
impl Reg128BitsDownCast<1> for Reg128Bits<44> {}
impl Reg128BitsConcat<1, 45> for Reg128Bits<44> {}
impl Reg128BitsDownCast<2> for Reg128Bits<44> {}
impl Reg128BitsConcat<2, 46> for Reg128Bits<44> {}
impl Reg128BitsDownCast<3> for Reg128Bits<44> {}
impl Reg128BitsConcat<3, 47> for Reg128Bits<44> {}
impl Reg128BitsDownCast<4> for Reg128Bits<44> {}
impl Reg128BitsConcat<4, 48> for Reg128Bits<44> {}
impl Reg128BitsDownCast<5> for Reg128Bits<44> {}
impl Reg128BitsConcat<5, 49> for Reg128Bits<44> {}
impl Reg128BitsDownCast<6> for Reg128Bits<44> {}
impl Reg128BitsConcat<6, 50> for Reg128Bits<44> {}
impl Reg128BitsDownCast<7> for Reg128Bits<44> {}
impl Reg128BitsConcat<7, 51> for Reg128Bits<44> {}
impl Reg128BitsDownCast<8> for Reg128Bits<44> {}
impl Reg128BitsConcat<8, 52> for Reg128Bits<44> {}
impl Reg128BitsDownCast<9> for Reg128Bits<44> {}
impl Reg128BitsConcat<9, 53> for Reg128Bits<44> {}
impl Reg128BitsDownCast<10> for Reg128Bits<44> {}
impl Reg128BitsConcat<10, 54> for Reg128Bits<44> {}
impl Reg128BitsDownCast<11> for Reg128Bits<44> {}
impl Reg128BitsConcat<11, 55> for Reg128Bits<44> {}
impl Reg128BitsDownCast<12> for Reg128Bits<44> {}
impl Reg128BitsConcat<12, 56> for Reg128Bits<44> {}
impl Reg128BitsDownCast<13> for Reg128Bits<44> {}
impl Reg128BitsConcat<13, 57> for Reg128Bits<44> {}
impl Reg128BitsDownCast<14> for Reg128Bits<44> {}
impl Reg128BitsConcat<14, 58> for Reg128Bits<44> {}
impl Reg128BitsDownCast<15> for Reg128Bits<44> {}
impl Reg128BitsConcat<15, 59> for Reg128Bits<44> {}
impl Reg128BitsDownCast<16> for Reg128Bits<44> {}
impl Reg128BitsConcat<16, 60> for Reg128Bits<44> {}
impl Reg128BitsDownCast<17> for Reg128Bits<44> {}
impl Reg128BitsConcat<17, 61> for Reg128Bits<44> {}
impl Reg128BitsDownCast<18> for Reg128Bits<44> {}
impl Reg128BitsConcat<18, 62> for Reg128Bits<44> {}
impl Reg128BitsDownCast<19> for Reg128Bits<44> {}
impl Reg128BitsConcat<19, 63> for Reg128Bits<44> {}
impl Reg128BitsDownCast<20> for Reg128Bits<44> {}
impl Reg128BitsConcat<20, 64> for Reg128Bits<44> {}
impl Reg128BitsDownCast<21> for Reg128Bits<44> {}
impl Reg128BitsConcat<21, 65> for Reg128Bits<44> {}
impl Reg128BitsDownCast<22> for Reg128Bits<44> {}
impl Reg128BitsConcat<22, 66> for Reg128Bits<44> {}
impl Reg128BitsDownCast<23> for Reg128Bits<44> {}
impl Reg128BitsConcat<23, 67> for Reg128Bits<44> {}
impl Reg128BitsDownCast<24> for Reg128Bits<44> {}
impl Reg128BitsConcat<24, 68> for Reg128Bits<44> {}
impl Reg128BitsDownCast<25> for Reg128Bits<44> {}
impl Reg128BitsConcat<25, 69> for Reg128Bits<44> {}
impl Reg128BitsDownCast<26> for Reg128Bits<44> {}
impl Reg128BitsConcat<26, 70> for Reg128Bits<44> {}
impl Reg128BitsDownCast<27> for Reg128Bits<44> {}
impl Reg128BitsConcat<27, 71> for Reg128Bits<44> {}
impl Reg128BitsDownCast<28> for Reg128Bits<44> {}
impl Reg128BitsConcat<28, 72> for Reg128Bits<44> {}
impl Reg128BitsDownCast<29> for Reg128Bits<44> {}
impl Reg128BitsConcat<29, 73> for Reg128Bits<44> {}
impl Reg128BitsDownCast<30> for Reg128Bits<44> {}
impl Reg128BitsConcat<30, 74> for Reg128Bits<44> {}
impl Reg128BitsDownCast<31> for Reg128Bits<44> {}
impl Reg128BitsConcat<31, 75> for Reg128Bits<44> {}
impl Reg128BitsDownCast<32> for Reg128Bits<44> {}
impl Reg128BitsConcat<32, 76> for Reg128Bits<44> {}
impl Reg128BitsDownCast<33> for Reg128Bits<44> {}
impl Reg128BitsConcat<33, 77> for Reg128Bits<44> {}
impl Reg128BitsDownCast<34> for Reg128Bits<44> {}
impl Reg128BitsConcat<34, 78> for Reg128Bits<44> {}
impl Reg128BitsDownCast<35> for Reg128Bits<44> {}
impl Reg128BitsConcat<35, 79> for Reg128Bits<44> {}
impl Reg128BitsDownCast<36> for Reg128Bits<44> {}
impl Reg128BitsConcat<36, 80> for Reg128Bits<44> {}
impl Reg128BitsDownCast<37> for Reg128Bits<44> {}
impl Reg128BitsConcat<37, 81> for Reg128Bits<44> {}
impl Reg128BitsDownCast<38> for Reg128Bits<44> {}
impl Reg128BitsConcat<38, 82> for Reg128Bits<44> {}
impl Reg128BitsDownCast<39> for Reg128Bits<44> {}
impl Reg128BitsConcat<39, 83> for Reg128Bits<44> {}
impl Reg128BitsDownCast<40> for Reg128Bits<44> {}
impl Reg128BitsConcat<40, 84> for Reg128Bits<44> {}
impl Reg128BitsDownCast<41> for Reg128Bits<44> {}
impl Reg128BitsConcat<41, 85> for Reg128Bits<44> {}
impl Reg128BitsDownCast<42> for Reg128Bits<44> {}
impl Reg128BitsConcat<42, 86> for Reg128Bits<44> {}
impl Reg128BitsDownCast<43> for Reg128Bits<44> {}
impl Reg128BitsConcat<43, 87> for Reg128Bits<44> {}
impl Reg128BitsDownCast<44> for Reg128Bits<44> {}
impl Reg128BitsConcat<44, 88> for Reg128Bits<44> {}
impl Reg128BitsDownCast<1> for Reg128Bits<45> {}
impl Reg128BitsConcat<1, 46> for Reg128Bits<45> {}
impl Reg128BitsDownCast<2> for Reg128Bits<45> {}
impl Reg128BitsConcat<2, 47> for Reg128Bits<45> {}
impl Reg128BitsDownCast<3> for Reg128Bits<45> {}
impl Reg128BitsConcat<3, 48> for Reg128Bits<45> {}
impl Reg128BitsDownCast<4> for Reg128Bits<45> {}
impl Reg128BitsConcat<4, 49> for Reg128Bits<45> {}
impl Reg128BitsDownCast<5> for Reg128Bits<45> {}
impl Reg128BitsConcat<5, 50> for Reg128Bits<45> {}
impl Reg128BitsDownCast<6> for Reg128Bits<45> {}
impl Reg128BitsConcat<6, 51> for Reg128Bits<45> {}
impl Reg128BitsDownCast<7> for Reg128Bits<45> {}
impl Reg128BitsConcat<7, 52> for Reg128Bits<45> {}
impl Reg128BitsDownCast<8> for Reg128Bits<45> {}
impl Reg128BitsConcat<8, 53> for Reg128Bits<45> {}
impl Reg128BitsDownCast<9> for Reg128Bits<45> {}
impl Reg128BitsConcat<9, 54> for Reg128Bits<45> {}
impl Reg128BitsDownCast<10> for Reg128Bits<45> {}
impl Reg128BitsConcat<10, 55> for Reg128Bits<45> {}
impl Reg128BitsDownCast<11> for Reg128Bits<45> {}
impl Reg128BitsConcat<11, 56> for Reg128Bits<45> {}
impl Reg128BitsDownCast<12> for Reg128Bits<45> {}
impl Reg128BitsConcat<12, 57> for Reg128Bits<45> {}
impl Reg128BitsDownCast<13> for Reg128Bits<45> {}
impl Reg128BitsConcat<13, 58> for Reg128Bits<45> {}
impl Reg128BitsDownCast<14> for Reg128Bits<45> {}
impl Reg128BitsConcat<14, 59> for Reg128Bits<45> {}
impl Reg128BitsDownCast<15> for Reg128Bits<45> {}
impl Reg128BitsConcat<15, 60> for Reg128Bits<45> {}
impl Reg128BitsDownCast<16> for Reg128Bits<45> {}
impl Reg128BitsConcat<16, 61> for Reg128Bits<45> {}
impl Reg128BitsDownCast<17> for Reg128Bits<45> {}
impl Reg128BitsConcat<17, 62> for Reg128Bits<45> {}
impl Reg128BitsDownCast<18> for Reg128Bits<45> {}
impl Reg128BitsConcat<18, 63> for Reg128Bits<45> {}
impl Reg128BitsDownCast<19> for Reg128Bits<45> {}
impl Reg128BitsConcat<19, 64> for Reg128Bits<45> {}
impl Reg128BitsDownCast<20> for Reg128Bits<45> {}
impl Reg128BitsConcat<20, 65> for Reg128Bits<45> {}
impl Reg128BitsDownCast<21> for Reg128Bits<45> {}
impl Reg128BitsConcat<21, 66> for Reg128Bits<45> {}
impl Reg128BitsDownCast<22> for Reg128Bits<45> {}
impl Reg128BitsConcat<22, 67> for Reg128Bits<45> {}
impl Reg128BitsDownCast<23> for Reg128Bits<45> {}
impl Reg128BitsConcat<23, 68> for Reg128Bits<45> {}
impl Reg128BitsDownCast<24> for Reg128Bits<45> {}
impl Reg128BitsConcat<24, 69> for Reg128Bits<45> {}
impl Reg128BitsDownCast<25> for Reg128Bits<45> {}
impl Reg128BitsConcat<25, 70> for Reg128Bits<45> {}
impl Reg128BitsDownCast<26> for Reg128Bits<45> {}
impl Reg128BitsConcat<26, 71> for Reg128Bits<45> {}
impl Reg128BitsDownCast<27> for Reg128Bits<45> {}
impl Reg128BitsConcat<27, 72> for Reg128Bits<45> {}
impl Reg128BitsDownCast<28> for Reg128Bits<45> {}
impl Reg128BitsConcat<28, 73> for Reg128Bits<45> {}
impl Reg128BitsDownCast<29> for Reg128Bits<45> {}
impl Reg128BitsConcat<29, 74> for Reg128Bits<45> {}
impl Reg128BitsDownCast<30> for Reg128Bits<45> {}
impl Reg128BitsConcat<30, 75> for Reg128Bits<45> {}
impl Reg128BitsDownCast<31> for Reg128Bits<45> {}
impl Reg128BitsConcat<31, 76> for Reg128Bits<45> {}
impl Reg128BitsDownCast<32> for Reg128Bits<45> {}
impl Reg128BitsConcat<32, 77> for Reg128Bits<45> {}
impl Reg128BitsDownCast<33> for Reg128Bits<45> {}
impl Reg128BitsConcat<33, 78> for Reg128Bits<45> {}
impl Reg128BitsDownCast<34> for Reg128Bits<45> {}
impl Reg128BitsConcat<34, 79> for Reg128Bits<45> {}
impl Reg128BitsDownCast<35> for Reg128Bits<45> {}
impl Reg128BitsConcat<35, 80> for Reg128Bits<45> {}
impl Reg128BitsDownCast<36> for Reg128Bits<45> {}
impl Reg128BitsConcat<36, 81> for Reg128Bits<45> {}
impl Reg128BitsDownCast<37> for Reg128Bits<45> {}
impl Reg128BitsConcat<37, 82> for Reg128Bits<45> {}
impl Reg128BitsDownCast<38> for Reg128Bits<45> {}
impl Reg128BitsConcat<38, 83> for Reg128Bits<45> {}
impl Reg128BitsDownCast<39> for Reg128Bits<45> {}
impl Reg128BitsConcat<39, 84> for Reg128Bits<45> {}
impl Reg128BitsDownCast<40> for Reg128Bits<45> {}
impl Reg128BitsConcat<40, 85> for Reg128Bits<45> {}
impl Reg128BitsDownCast<41> for Reg128Bits<45> {}
impl Reg128BitsConcat<41, 86> for Reg128Bits<45> {}
impl Reg128BitsDownCast<42> for Reg128Bits<45> {}
impl Reg128BitsConcat<42, 87> for Reg128Bits<45> {}
impl Reg128BitsDownCast<43> for Reg128Bits<45> {}
impl Reg128BitsConcat<43, 88> for Reg128Bits<45> {}
impl Reg128BitsDownCast<44> for Reg128Bits<45> {}
impl Reg128BitsConcat<44, 89> for Reg128Bits<45> {}
impl Reg128BitsDownCast<45> for Reg128Bits<45> {}
impl Reg128BitsConcat<45, 90> for Reg128Bits<45> {}
impl Reg128BitsDownCast<1> for Reg128Bits<46> {}
impl Reg128BitsConcat<1, 47> for Reg128Bits<46> {}
impl Reg128BitsDownCast<2> for Reg128Bits<46> {}
impl Reg128BitsConcat<2, 48> for Reg128Bits<46> {}
impl Reg128BitsDownCast<3> for Reg128Bits<46> {}
impl Reg128BitsConcat<3, 49> for Reg128Bits<46> {}
impl Reg128BitsDownCast<4> for Reg128Bits<46> {}
impl Reg128BitsConcat<4, 50> for Reg128Bits<46> {}
impl Reg128BitsDownCast<5> for Reg128Bits<46> {}
impl Reg128BitsConcat<5, 51> for Reg128Bits<46> {}
impl Reg128BitsDownCast<6> for Reg128Bits<46> {}
impl Reg128BitsConcat<6, 52> for Reg128Bits<46> {}
impl Reg128BitsDownCast<7> for Reg128Bits<46> {}
impl Reg128BitsConcat<7, 53> for Reg128Bits<46> {}
impl Reg128BitsDownCast<8> for Reg128Bits<46> {}
impl Reg128BitsConcat<8, 54> for Reg128Bits<46> {}
impl Reg128BitsDownCast<9> for Reg128Bits<46> {}
impl Reg128BitsConcat<9, 55> for Reg128Bits<46> {}
impl Reg128BitsDownCast<10> for Reg128Bits<46> {}
impl Reg128BitsConcat<10, 56> for Reg128Bits<46> {}
impl Reg128BitsDownCast<11> for Reg128Bits<46> {}
impl Reg128BitsConcat<11, 57> for Reg128Bits<46> {}
impl Reg128BitsDownCast<12> for Reg128Bits<46> {}
impl Reg128BitsConcat<12, 58> for Reg128Bits<46> {}
impl Reg128BitsDownCast<13> for Reg128Bits<46> {}
impl Reg128BitsConcat<13, 59> for Reg128Bits<46> {}
impl Reg128BitsDownCast<14> for Reg128Bits<46> {}
impl Reg128BitsConcat<14, 60> for Reg128Bits<46> {}
impl Reg128BitsDownCast<15> for Reg128Bits<46> {}
impl Reg128BitsConcat<15, 61> for Reg128Bits<46> {}
impl Reg128BitsDownCast<16> for Reg128Bits<46> {}
impl Reg128BitsConcat<16, 62> for Reg128Bits<46> {}
impl Reg128BitsDownCast<17> for Reg128Bits<46> {}
impl Reg128BitsConcat<17, 63> for Reg128Bits<46> {}
impl Reg128BitsDownCast<18> for Reg128Bits<46> {}
impl Reg128BitsConcat<18, 64> for Reg128Bits<46> {}
impl Reg128BitsDownCast<19> for Reg128Bits<46> {}
impl Reg128BitsConcat<19, 65> for Reg128Bits<46> {}
impl Reg128BitsDownCast<20> for Reg128Bits<46> {}
impl Reg128BitsConcat<20, 66> for Reg128Bits<46> {}
impl Reg128BitsDownCast<21> for Reg128Bits<46> {}
impl Reg128BitsConcat<21, 67> for Reg128Bits<46> {}
impl Reg128BitsDownCast<22> for Reg128Bits<46> {}
impl Reg128BitsConcat<22, 68> for Reg128Bits<46> {}
impl Reg128BitsDownCast<23> for Reg128Bits<46> {}
impl Reg128BitsConcat<23, 69> for Reg128Bits<46> {}
impl Reg128BitsDownCast<24> for Reg128Bits<46> {}
impl Reg128BitsConcat<24, 70> for Reg128Bits<46> {}
impl Reg128BitsDownCast<25> for Reg128Bits<46> {}
impl Reg128BitsConcat<25, 71> for Reg128Bits<46> {}
impl Reg128BitsDownCast<26> for Reg128Bits<46> {}
impl Reg128BitsConcat<26, 72> for Reg128Bits<46> {}
impl Reg128BitsDownCast<27> for Reg128Bits<46> {}
impl Reg128BitsConcat<27, 73> for Reg128Bits<46> {}
impl Reg128BitsDownCast<28> for Reg128Bits<46> {}
impl Reg128BitsConcat<28, 74> for Reg128Bits<46> {}
impl Reg128BitsDownCast<29> for Reg128Bits<46> {}
impl Reg128BitsConcat<29, 75> for Reg128Bits<46> {}
impl Reg128BitsDownCast<30> for Reg128Bits<46> {}
impl Reg128BitsConcat<30, 76> for Reg128Bits<46> {}
impl Reg128BitsDownCast<31> for Reg128Bits<46> {}
impl Reg128BitsConcat<31, 77> for Reg128Bits<46> {}
impl Reg128BitsDownCast<32> for Reg128Bits<46> {}
impl Reg128BitsConcat<32, 78> for Reg128Bits<46> {}
impl Reg128BitsDownCast<33> for Reg128Bits<46> {}
impl Reg128BitsConcat<33, 79> for Reg128Bits<46> {}
impl Reg128BitsDownCast<34> for Reg128Bits<46> {}
impl Reg128BitsConcat<34, 80> for Reg128Bits<46> {}
impl Reg128BitsDownCast<35> for Reg128Bits<46> {}
impl Reg128BitsConcat<35, 81> for Reg128Bits<46> {}
impl Reg128BitsDownCast<36> for Reg128Bits<46> {}
impl Reg128BitsConcat<36, 82> for Reg128Bits<46> {}
impl Reg128BitsDownCast<37> for Reg128Bits<46> {}
impl Reg128BitsConcat<37, 83> for Reg128Bits<46> {}
impl Reg128BitsDownCast<38> for Reg128Bits<46> {}
impl Reg128BitsConcat<38, 84> for Reg128Bits<46> {}
impl Reg128BitsDownCast<39> for Reg128Bits<46> {}
impl Reg128BitsConcat<39, 85> for Reg128Bits<46> {}
impl Reg128BitsDownCast<40> for Reg128Bits<46> {}
impl Reg128BitsConcat<40, 86> for Reg128Bits<46> {}
impl Reg128BitsDownCast<41> for Reg128Bits<46> {}
impl Reg128BitsConcat<41, 87> for Reg128Bits<46> {}
impl Reg128BitsDownCast<42> for Reg128Bits<46> {}
impl Reg128BitsConcat<42, 88> for Reg128Bits<46> {}
impl Reg128BitsDownCast<43> for Reg128Bits<46> {}
impl Reg128BitsConcat<43, 89> for Reg128Bits<46> {}
impl Reg128BitsDownCast<44> for Reg128Bits<46> {}
impl Reg128BitsConcat<44, 90> for Reg128Bits<46> {}
impl Reg128BitsDownCast<45> for Reg128Bits<46> {}
impl Reg128BitsConcat<45, 91> for Reg128Bits<46> {}
impl Reg128BitsDownCast<46> for Reg128Bits<46> {}
impl Reg128BitsConcat<46, 92> for Reg128Bits<46> {}
impl Reg128BitsDownCast<1> for Reg128Bits<47> {}
impl Reg128BitsConcat<1, 48> for Reg128Bits<47> {}
impl Reg128BitsDownCast<2> for Reg128Bits<47> {}
impl Reg128BitsConcat<2, 49> for Reg128Bits<47> {}
impl Reg128BitsDownCast<3> for Reg128Bits<47> {}
impl Reg128BitsConcat<3, 50> for Reg128Bits<47> {}
impl Reg128BitsDownCast<4> for Reg128Bits<47> {}
impl Reg128BitsConcat<4, 51> for Reg128Bits<47> {}
impl Reg128BitsDownCast<5> for Reg128Bits<47> {}
impl Reg128BitsConcat<5, 52> for Reg128Bits<47> {}
impl Reg128BitsDownCast<6> for Reg128Bits<47> {}
impl Reg128BitsConcat<6, 53> for Reg128Bits<47> {}
impl Reg128BitsDownCast<7> for Reg128Bits<47> {}
impl Reg128BitsConcat<7, 54> for Reg128Bits<47> {}
impl Reg128BitsDownCast<8> for Reg128Bits<47> {}
impl Reg128BitsConcat<8, 55> for Reg128Bits<47> {}
impl Reg128BitsDownCast<9> for Reg128Bits<47> {}
impl Reg128BitsConcat<9, 56> for Reg128Bits<47> {}
impl Reg128BitsDownCast<10> for Reg128Bits<47> {}
impl Reg128BitsConcat<10, 57> for Reg128Bits<47> {}
impl Reg128BitsDownCast<11> for Reg128Bits<47> {}
impl Reg128BitsConcat<11, 58> for Reg128Bits<47> {}
impl Reg128BitsDownCast<12> for Reg128Bits<47> {}
impl Reg128BitsConcat<12, 59> for Reg128Bits<47> {}
impl Reg128BitsDownCast<13> for Reg128Bits<47> {}
impl Reg128BitsConcat<13, 60> for Reg128Bits<47> {}
impl Reg128BitsDownCast<14> for Reg128Bits<47> {}
impl Reg128BitsConcat<14, 61> for Reg128Bits<47> {}
impl Reg128BitsDownCast<15> for Reg128Bits<47> {}
impl Reg128BitsConcat<15, 62> for Reg128Bits<47> {}
impl Reg128BitsDownCast<16> for Reg128Bits<47> {}
impl Reg128BitsConcat<16, 63> for Reg128Bits<47> {}
impl Reg128BitsDownCast<17> for Reg128Bits<47> {}
impl Reg128BitsConcat<17, 64> for Reg128Bits<47> {}
impl Reg128BitsDownCast<18> for Reg128Bits<47> {}
impl Reg128BitsConcat<18, 65> for Reg128Bits<47> {}
impl Reg128BitsDownCast<19> for Reg128Bits<47> {}
impl Reg128BitsConcat<19, 66> for Reg128Bits<47> {}
impl Reg128BitsDownCast<20> for Reg128Bits<47> {}
impl Reg128BitsConcat<20, 67> for Reg128Bits<47> {}
impl Reg128BitsDownCast<21> for Reg128Bits<47> {}
impl Reg128BitsConcat<21, 68> for Reg128Bits<47> {}
impl Reg128BitsDownCast<22> for Reg128Bits<47> {}
impl Reg128BitsConcat<22, 69> for Reg128Bits<47> {}
impl Reg128BitsDownCast<23> for Reg128Bits<47> {}
impl Reg128BitsConcat<23, 70> for Reg128Bits<47> {}
impl Reg128BitsDownCast<24> for Reg128Bits<47> {}
impl Reg128BitsConcat<24, 71> for Reg128Bits<47> {}
impl Reg128BitsDownCast<25> for Reg128Bits<47> {}
impl Reg128BitsConcat<25, 72> for Reg128Bits<47> {}
impl Reg128BitsDownCast<26> for Reg128Bits<47> {}
impl Reg128BitsConcat<26, 73> for Reg128Bits<47> {}
impl Reg128BitsDownCast<27> for Reg128Bits<47> {}
impl Reg128BitsConcat<27, 74> for Reg128Bits<47> {}
impl Reg128BitsDownCast<28> for Reg128Bits<47> {}
impl Reg128BitsConcat<28, 75> for Reg128Bits<47> {}
impl Reg128BitsDownCast<29> for Reg128Bits<47> {}
impl Reg128BitsConcat<29, 76> for Reg128Bits<47> {}
impl Reg128BitsDownCast<30> for Reg128Bits<47> {}
impl Reg128BitsConcat<30, 77> for Reg128Bits<47> {}
impl Reg128BitsDownCast<31> for Reg128Bits<47> {}
impl Reg128BitsConcat<31, 78> for Reg128Bits<47> {}
impl Reg128BitsDownCast<32> for Reg128Bits<47> {}
impl Reg128BitsConcat<32, 79> for Reg128Bits<47> {}
impl Reg128BitsDownCast<33> for Reg128Bits<47> {}
impl Reg128BitsConcat<33, 80> for Reg128Bits<47> {}
impl Reg128BitsDownCast<34> for Reg128Bits<47> {}
impl Reg128BitsConcat<34, 81> for Reg128Bits<47> {}
impl Reg128BitsDownCast<35> for Reg128Bits<47> {}
impl Reg128BitsConcat<35, 82> for Reg128Bits<47> {}
impl Reg128BitsDownCast<36> for Reg128Bits<47> {}
impl Reg128BitsConcat<36, 83> for Reg128Bits<47> {}
impl Reg128BitsDownCast<37> for Reg128Bits<47> {}
impl Reg128BitsConcat<37, 84> for Reg128Bits<47> {}
impl Reg128BitsDownCast<38> for Reg128Bits<47> {}
impl Reg128BitsConcat<38, 85> for Reg128Bits<47> {}
impl Reg128BitsDownCast<39> for Reg128Bits<47> {}
impl Reg128BitsConcat<39, 86> for Reg128Bits<47> {}
impl Reg128BitsDownCast<40> for Reg128Bits<47> {}
impl Reg128BitsConcat<40, 87> for Reg128Bits<47> {}
impl Reg128BitsDownCast<41> for Reg128Bits<47> {}
impl Reg128BitsConcat<41, 88> for Reg128Bits<47> {}
impl Reg128BitsDownCast<42> for Reg128Bits<47> {}
impl Reg128BitsConcat<42, 89> for Reg128Bits<47> {}
impl Reg128BitsDownCast<43> for Reg128Bits<47> {}
impl Reg128BitsConcat<43, 90> for Reg128Bits<47> {}
impl Reg128BitsDownCast<44> for Reg128Bits<47> {}
impl Reg128BitsConcat<44, 91> for Reg128Bits<47> {}
impl Reg128BitsDownCast<45> for Reg128Bits<47> {}
impl Reg128BitsConcat<45, 92> for Reg128Bits<47> {}
impl Reg128BitsDownCast<46> for Reg128Bits<47> {}
impl Reg128BitsConcat<46, 93> for Reg128Bits<47> {}
impl Reg128BitsDownCast<47> for Reg128Bits<47> {}
impl Reg128BitsConcat<47, 94> for Reg128Bits<47> {}
impl Reg128BitsDownCast<1> for Reg128Bits<48> {}
impl Reg128BitsConcat<1, 49> for Reg128Bits<48> {}
impl Reg128BitsDownCast<2> for Reg128Bits<48> {}
impl Reg128BitsConcat<2, 50> for Reg128Bits<48> {}
impl Reg128BitsDownCast<3> for Reg128Bits<48> {}
impl Reg128BitsConcat<3, 51> for Reg128Bits<48> {}
impl Reg128BitsDownCast<4> for Reg128Bits<48> {}
impl Reg128BitsConcat<4, 52> for Reg128Bits<48> {}
impl Reg128BitsDownCast<5> for Reg128Bits<48> {}
impl Reg128BitsConcat<5, 53> for Reg128Bits<48> {}
impl Reg128BitsDownCast<6> for Reg128Bits<48> {}
impl Reg128BitsConcat<6, 54> for Reg128Bits<48> {}
impl Reg128BitsDownCast<7> for Reg128Bits<48> {}
impl Reg128BitsConcat<7, 55> for Reg128Bits<48> {}
impl Reg128BitsDownCast<8> for Reg128Bits<48> {}
impl Reg128BitsConcat<8, 56> for Reg128Bits<48> {}
impl Reg128BitsDownCast<9> for Reg128Bits<48> {}
impl Reg128BitsConcat<9, 57> for Reg128Bits<48> {}
impl Reg128BitsDownCast<10> for Reg128Bits<48> {}
impl Reg128BitsConcat<10, 58> for Reg128Bits<48> {}
impl Reg128BitsDownCast<11> for Reg128Bits<48> {}
impl Reg128BitsConcat<11, 59> for Reg128Bits<48> {}
impl Reg128BitsDownCast<12> for Reg128Bits<48> {}
impl Reg128BitsConcat<12, 60> for Reg128Bits<48> {}
impl Reg128BitsDownCast<13> for Reg128Bits<48> {}
impl Reg128BitsConcat<13, 61> for Reg128Bits<48> {}
impl Reg128BitsDownCast<14> for Reg128Bits<48> {}
impl Reg128BitsConcat<14, 62> for Reg128Bits<48> {}
impl Reg128BitsDownCast<15> for Reg128Bits<48> {}
impl Reg128BitsConcat<15, 63> for Reg128Bits<48> {}
impl Reg128BitsDownCast<16> for Reg128Bits<48> {}
impl Reg128BitsConcat<16, 64> for Reg128Bits<48> {}
impl Reg128BitsDownCast<17> for Reg128Bits<48> {}
impl Reg128BitsConcat<17, 65> for Reg128Bits<48> {}
impl Reg128BitsDownCast<18> for Reg128Bits<48> {}
impl Reg128BitsConcat<18, 66> for Reg128Bits<48> {}
impl Reg128BitsDownCast<19> for Reg128Bits<48> {}
impl Reg128BitsConcat<19, 67> for Reg128Bits<48> {}
impl Reg128BitsDownCast<20> for Reg128Bits<48> {}
impl Reg128BitsConcat<20, 68> for Reg128Bits<48> {}
impl Reg128BitsDownCast<21> for Reg128Bits<48> {}
impl Reg128BitsConcat<21, 69> for Reg128Bits<48> {}
impl Reg128BitsDownCast<22> for Reg128Bits<48> {}
impl Reg128BitsConcat<22, 70> for Reg128Bits<48> {}
impl Reg128BitsDownCast<23> for Reg128Bits<48> {}
impl Reg128BitsConcat<23, 71> for Reg128Bits<48> {}
impl Reg128BitsDownCast<24> for Reg128Bits<48> {}
impl Reg128BitsConcat<24, 72> for Reg128Bits<48> {}
impl Reg128BitsDownCast<25> for Reg128Bits<48> {}
impl Reg128BitsConcat<25, 73> for Reg128Bits<48> {}
impl Reg128BitsDownCast<26> for Reg128Bits<48> {}
impl Reg128BitsConcat<26, 74> for Reg128Bits<48> {}
impl Reg128BitsDownCast<27> for Reg128Bits<48> {}
impl Reg128BitsConcat<27, 75> for Reg128Bits<48> {}
impl Reg128BitsDownCast<28> for Reg128Bits<48> {}
impl Reg128BitsConcat<28, 76> for Reg128Bits<48> {}
impl Reg128BitsDownCast<29> for Reg128Bits<48> {}
impl Reg128BitsConcat<29, 77> for Reg128Bits<48> {}
impl Reg128BitsDownCast<30> for Reg128Bits<48> {}
impl Reg128BitsConcat<30, 78> for Reg128Bits<48> {}
impl Reg128BitsDownCast<31> for Reg128Bits<48> {}
impl Reg128BitsConcat<31, 79> for Reg128Bits<48> {}
impl Reg128BitsDownCast<32> for Reg128Bits<48> {}
impl Reg128BitsConcat<32, 80> for Reg128Bits<48> {}
impl Reg128BitsDownCast<33> for Reg128Bits<48> {}
impl Reg128BitsConcat<33, 81> for Reg128Bits<48> {}
impl Reg128BitsDownCast<34> for Reg128Bits<48> {}
impl Reg128BitsConcat<34, 82> for Reg128Bits<48> {}
impl Reg128BitsDownCast<35> for Reg128Bits<48> {}
impl Reg128BitsConcat<35, 83> for Reg128Bits<48> {}
impl Reg128BitsDownCast<36> for Reg128Bits<48> {}
impl Reg128BitsConcat<36, 84> for Reg128Bits<48> {}
impl Reg128BitsDownCast<37> for Reg128Bits<48> {}
impl Reg128BitsConcat<37, 85> for Reg128Bits<48> {}
impl Reg128BitsDownCast<38> for Reg128Bits<48> {}
impl Reg128BitsConcat<38, 86> for Reg128Bits<48> {}
impl Reg128BitsDownCast<39> for Reg128Bits<48> {}
impl Reg128BitsConcat<39, 87> for Reg128Bits<48> {}
impl Reg128BitsDownCast<40> for Reg128Bits<48> {}
impl Reg128BitsConcat<40, 88> for Reg128Bits<48> {}
impl Reg128BitsDownCast<41> for Reg128Bits<48> {}
impl Reg128BitsConcat<41, 89> for Reg128Bits<48> {}
impl Reg128BitsDownCast<42> for Reg128Bits<48> {}
impl Reg128BitsConcat<42, 90> for Reg128Bits<48> {}
impl Reg128BitsDownCast<43> for Reg128Bits<48> {}
impl Reg128BitsConcat<43, 91> for Reg128Bits<48> {}
impl Reg128BitsDownCast<44> for Reg128Bits<48> {}
impl Reg128BitsConcat<44, 92> for Reg128Bits<48> {}
impl Reg128BitsDownCast<45> for Reg128Bits<48> {}
impl Reg128BitsConcat<45, 93> for Reg128Bits<48> {}
impl Reg128BitsDownCast<46> for Reg128Bits<48> {}
impl Reg128BitsConcat<46, 94> for Reg128Bits<48> {}
impl Reg128BitsDownCast<47> for Reg128Bits<48> {}
impl Reg128BitsConcat<47, 95> for Reg128Bits<48> {}
impl Reg128BitsDownCast<48> for Reg128Bits<48> {}
impl Reg128BitsConcat<48, 96> for Reg128Bits<48> {}
impl Reg128BitsDownCast<1> for Reg128Bits<49> {}
impl Reg128BitsConcat<1, 50> for Reg128Bits<49> {}
impl Reg128BitsDownCast<2> for Reg128Bits<49> {}
impl Reg128BitsConcat<2, 51> for Reg128Bits<49> {}
impl Reg128BitsDownCast<3> for Reg128Bits<49> {}
impl Reg128BitsConcat<3, 52> for Reg128Bits<49> {}
impl Reg128BitsDownCast<4> for Reg128Bits<49> {}
impl Reg128BitsConcat<4, 53> for Reg128Bits<49> {}
impl Reg128BitsDownCast<5> for Reg128Bits<49> {}
impl Reg128BitsConcat<5, 54> for Reg128Bits<49> {}
impl Reg128BitsDownCast<6> for Reg128Bits<49> {}
impl Reg128BitsConcat<6, 55> for Reg128Bits<49> {}
impl Reg128BitsDownCast<7> for Reg128Bits<49> {}
impl Reg128BitsConcat<7, 56> for Reg128Bits<49> {}
impl Reg128BitsDownCast<8> for Reg128Bits<49> {}
impl Reg128BitsConcat<8, 57> for Reg128Bits<49> {}
impl Reg128BitsDownCast<9> for Reg128Bits<49> {}
impl Reg128BitsConcat<9, 58> for Reg128Bits<49> {}
impl Reg128BitsDownCast<10> for Reg128Bits<49> {}
impl Reg128BitsConcat<10, 59> for Reg128Bits<49> {}
impl Reg128BitsDownCast<11> for Reg128Bits<49> {}
impl Reg128BitsConcat<11, 60> for Reg128Bits<49> {}
impl Reg128BitsDownCast<12> for Reg128Bits<49> {}
impl Reg128BitsConcat<12, 61> for Reg128Bits<49> {}
impl Reg128BitsDownCast<13> for Reg128Bits<49> {}
impl Reg128BitsConcat<13, 62> for Reg128Bits<49> {}
impl Reg128BitsDownCast<14> for Reg128Bits<49> {}
impl Reg128BitsConcat<14, 63> for Reg128Bits<49> {}
impl Reg128BitsDownCast<15> for Reg128Bits<49> {}
impl Reg128BitsConcat<15, 64> for Reg128Bits<49> {}
impl Reg128BitsDownCast<16> for Reg128Bits<49> {}
impl Reg128BitsConcat<16, 65> for Reg128Bits<49> {}
impl Reg128BitsDownCast<17> for Reg128Bits<49> {}
impl Reg128BitsConcat<17, 66> for Reg128Bits<49> {}
impl Reg128BitsDownCast<18> for Reg128Bits<49> {}
impl Reg128BitsConcat<18, 67> for Reg128Bits<49> {}
impl Reg128BitsDownCast<19> for Reg128Bits<49> {}
impl Reg128BitsConcat<19, 68> for Reg128Bits<49> {}
impl Reg128BitsDownCast<20> for Reg128Bits<49> {}
impl Reg128BitsConcat<20, 69> for Reg128Bits<49> {}
impl Reg128BitsDownCast<21> for Reg128Bits<49> {}
impl Reg128BitsConcat<21, 70> for Reg128Bits<49> {}
impl Reg128BitsDownCast<22> for Reg128Bits<49> {}
impl Reg128BitsConcat<22, 71> for Reg128Bits<49> {}
impl Reg128BitsDownCast<23> for Reg128Bits<49> {}
impl Reg128BitsConcat<23, 72> for Reg128Bits<49> {}
impl Reg128BitsDownCast<24> for Reg128Bits<49> {}
impl Reg128BitsConcat<24, 73> for Reg128Bits<49> {}
impl Reg128BitsDownCast<25> for Reg128Bits<49> {}
impl Reg128BitsConcat<25, 74> for Reg128Bits<49> {}
impl Reg128BitsDownCast<26> for Reg128Bits<49> {}
impl Reg128BitsConcat<26, 75> for Reg128Bits<49> {}
impl Reg128BitsDownCast<27> for Reg128Bits<49> {}
impl Reg128BitsConcat<27, 76> for Reg128Bits<49> {}
impl Reg128BitsDownCast<28> for Reg128Bits<49> {}
impl Reg128BitsConcat<28, 77> for Reg128Bits<49> {}
impl Reg128BitsDownCast<29> for Reg128Bits<49> {}
impl Reg128BitsConcat<29, 78> for Reg128Bits<49> {}
impl Reg128BitsDownCast<30> for Reg128Bits<49> {}
impl Reg128BitsConcat<30, 79> for Reg128Bits<49> {}
impl Reg128BitsDownCast<31> for Reg128Bits<49> {}
impl Reg128BitsConcat<31, 80> for Reg128Bits<49> {}
impl Reg128BitsDownCast<32> for Reg128Bits<49> {}
impl Reg128BitsConcat<32, 81> for Reg128Bits<49> {}
impl Reg128BitsDownCast<33> for Reg128Bits<49> {}
impl Reg128BitsConcat<33, 82> for Reg128Bits<49> {}
impl Reg128BitsDownCast<34> for Reg128Bits<49> {}
impl Reg128BitsConcat<34, 83> for Reg128Bits<49> {}
impl Reg128BitsDownCast<35> for Reg128Bits<49> {}
impl Reg128BitsConcat<35, 84> for Reg128Bits<49> {}
impl Reg128BitsDownCast<36> for Reg128Bits<49> {}
impl Reg128BitsConcat<36, 85> for Reg128Bits<49> {}
impl Reg128BitsDownCast<37> for Reg128Bits<49> {}
impl Reg128BitsConcat<37, 86> for Reg128Bits<49> {}
impl Reg128BitsDownCast<38> for Reg128Bits<49> {}
impl Reg128BitsConcat<38, 87> for Reg128Bits<49> {}
impl Reg128BitsDownCast<39> for Reg128Bits<49> {}
impl Reg128BitsConcat<39, 88> for Reg128Bits<49> {}
impl Reg128BitsDownCast<40> for Reg128Bits<49> {}
impl Reg128BitsConcat<40, 89> for Reg128Bits<49> {}
impl Reg128BitsDownCast<41> for Reg128Bits<49> {}
impl Reg128BitsConcat<41, 90> for Reg128Bits<49> {}
impl Reg128BitsDownCast<42> for Reg128Bits<49> {}
impl Reg128BitsConcat<42, 91> for Reg128Bits<49> {}
impl Reg128BitsDownCast<43> for Reg128Bits<49> {}
impl Reg128BitsConcat<43, 92> for Reg128Bits<49> {}
impl Reg128BitsDownCast<44> for Reg128Bits<49> {}
impl Reg128BitsConcat<44, 93> for Reg128Bits<49> {}
impl Reg128BitsDownCast<45> for Reg128Bits<49> {}
impl Reg128BitsConcat<45, 94> for Reg128Bits<49> {}
impl Reg128BitsDownCast<46> for Reg128Bits<49> {}
impl Reg128BitsConcat<46, 95> for Reg128Bits<49> {}
impl Reg128BitsDownCast<47> for Reg128Bits<49> {}
impl Reg128BitsConcat<47, 96> for Reg128Bits<49> {}
impl Reg128BitsDownCast<48> for Reg128Bits<49> {}
impl Reg128BitsConcat<48, 97> for Reg128Bits<49> {}
impl Reg128BitsDownCast<49> for Reg128Bits<49> {}
impl Reg128BitsConcat<49, 98> for Reg128Bits<49> {}
impl Reg128BitsDownCast<1> for Reg128Bits<50> {}
impl Reg128BitsConcat<1, 51> for Reg128Bits<50> {}
impl Reg128BitsDownCast<2> for Reg128Bits<50> {}
impl Reg128BitsConcat<2, 52> for Reg128Bits<50> {}
impl Reg128BitsDownCast<3> for Reg128Bits<50> {}
impl Reg128BitsConcat<3, 53> for Reg128Bits<50> {}
impl Reg128BitsDownCast<4> for Reg128Bits<50> {}
impl Reg128BitsConcat<4, 54> for Reg128Bits<50> {}
impl Reg128BitsDownCast<5> for Reg128Bits<50> {}
impl Reg128BitsConcat<5, 55> for Reg128Bits<50> {}
impl Reg128BitsDownCast<6> for Reg128Bits<50> {}
impl Reg128BitsConcat<6, 56> for Reg128Bits<50> {}
impl Reg128BitsDownCast<7> for Reg128Bits<50> {}
impl Reg128BitsConcat<7, 57> for Reg128Bits<50> {}
impl Reg128BitsDownCast<8> for Reg128Bits<50> {}
impl Reg128BitsConcat<8, 58> for Reg128Bits<50> {}
impl Reg128BitsDownCast<9> for Reg128Bits<50> {}
impl Reg128BitsConcat<9, 59> for Reg128Bits<50> {}
impl Reg128BitsDownCast<10> for Reg128Bits<50> {}
impl Reg128BitsConcat<10, 60> for Reg128Bits<50> {}
impl Reg128BitsDownCast<11> for Reg128Bits<50> {}
impl Reg128BitsConcat<11, 61> for Reg128Bits<50> {}
impl Reg128BitsDownCast<12> for Reg128Bits<50> {}
impl Reg128BitsConcat<12, 62> for Reg128Bits<50> {}
impl Reg128BitsDownCast<13> for Reg128Bits<50> {}
impl Reg128BitsConcat<13, 63> for Reg128Bits<50> {}
impl Reg128BitsDownCast<14> for Reg128Bits<50> {}
impl Reg128BitsConcat<14, 64> for Reg128Bits<50> {}
impl Reg128BitsDownCast<15> for Reg128Bits<50> {}
impl Reg128BitsConcat<15, 65> for Reg128Bits<50> {}
impl Reg128BitsDownCast<16> for Reg128Bits<50> {}
impl Reg128BitsConcat<16, 66> for Reg128Bits<50> {}
impl Reg128BitsDownCast<17> for Reg128Bits<50> {}
impl Reg128BitsConcat<17, 67> for Reg128Bits<50> {}
impl Reg128BitsDownCast<18> for Reg128Bits<50> {}
impl Reg128BitsConcat<18, 68> for Reg128Bits<50> {}
impl Reg128BitsDownCast<19> for Reg128Bits<50> {}
impl Reg128BitsConcat<19, 69> for Reg128Bits<50> {}
impl Reg128BitsDownCast<20> for Reg128Bits<50> {}
impl Reg128BitsConcat<20, 70> for Reg128Bits<50> {}
impl Reg128BitsDownCast<21> for Reg128Bits<50> {}
impl Reg128BitsConcat<21, 71> for Reg128Bits<50> {}
impl Reg128BitsDownCast<22> for Reg128Bits<50> {}
impl Reg128BitsConcat<22, 72> for Reg128Bits<50> {}
impl Reg128BitsDownCast<23> for Reg128Bits<50> {}
impl Reg128BitsConcat<23, 73> for Reg128Bits<50> {}
impl Reg128BitsDownCast<24> for Reg128Bits<50> {}
impl Reg128BitsConcat<24, 74> for Reg128Bits<50> {}
impl Reg128BitsDownCast<25> for Reg128Bits<50> {}
impl Reg128BitsConcat<25, 75> for Reg128Bits<50> {}
impl Reg128BitsDownCast<26> for Reg128Bits<50> {}
impl Reg128BitsConcat<26, 76> for Reg128Bits<50> {}
impl Reg128BitsDownCast<27> for Reg128Bits<50> {}
impl Reg128BitsConcat<27, 77> for Reg128Bits<50> {}
impl Reg128BitsDownCast<28> for Reg128Bits<50> {}
impl Reg128BitsConcat<28, 78> for Reg128Bits<50> {}
impl Reg128BitsDownCast<29> for Reg128Bits<50> {}
impl Reg128BitsConcat<29, 79> for Reg128Bits<50> {}
impl Reg128BitsDownCast<30> for Reg128Bits<50> {}
impl Reg128BitsConcat<30, 80> for Reg128Bits<50> {}
impl Reg128BitsDownCast<31> for Reg128Bits<50> {}
impl Reg128BitsConcat<31, 81> for Reg128Bits<50> {}
impl Reg128BitsDownCast<32> for Reg128Bits<50> {}
impl Reg128BitsConcat<32, 82> for Reg128Bits<50> {}
impl Reg128BitsDownCast<33> for Reg128Bits<50> {}
impl Reg128BitsConcat<33, 83> for Reg128Bits<50> {}
impl Reg128BitsDownCast<34> for Reg128Bits<50> {}
impl Reg128BitsConcat<34, 84> for Reg128Bits<50> {}
impl Reg128BitsDownCast<35> for Reg128Bits<50> {}
impl Reg128BitsConcat<35, 85> for Reg128Bits<50> {}
impl Reg128BitsDownCast<36> for Reg128Bits<50> {}
impl Reg128BitsConcat<36, 86> for Reg128Bits<50> {}
impl Reg128BitsDownCast<37> for Reg128Bits<50> {}
impl Reg128BitsConcat<37, 87> for Reg128Bits<50> {}
impl Reg128BitsDownCast<38> for Reg128Bits<50> {}
impl Reg128BitsConcat<38, 88> for Reg128Bits<50> {}
impl Reg128BitsDownCast<39> for Reg128Bits<50> {}
impl Reg128BitsConcat<39, 89> for Reg128Bits<50> {}
impl Reg128BitsDownCast<40> for Reg128Bits<50> {}
impl Reg128BitsConcat<40, 90> for Reg128Bits<50> {}
impl Reg128BitsDownCast<41> for Reg128Bits<50> {}
impl Reg128BitsConcat<41, 91> for Reg128Bits<50> {}
impl Reg128BitsDownCast<42> for Reg128Bits<50> {}
impl Reg128BitsConcat<42, 92> for Reg128Bits<50> {}
impl Reg128BitsDownCast<43> for Reg128Bits<50> {}
impl Reg128BitsConcat<43, 93> for Reg128Bits<50> {}
impl Reg128BitsDownCast<44> for Reg128Bits<50> {}
impl Reg128BitsConcat<44, 94> for Reg128Bits<50> {}
impl Reg128BitsDownCast<45> for Reg128Bits<50> {}
impl Reg128BitsConcat<45, 95> for Reg128Bits<50> {}
impl Reg128BitsDownCast<46> for Reg128Bits<50> {}
impl Reg128BitsConcat<46, 96> for Reg128Bits<50> {}
impl Reg128BitsDownCast<47> for Reg128Bits<50> {}
impl Reg128BitsConcat<47, 97> for Reg128Bits<50> {}
impl Reg128BitsDownCast<48> for Reg128Bits<50> {}
impl Reg128BitsConcat<48, 98> for Reg128Bits<50> {}
impl Reg128BitsDownCast<49> for Reg128Bits<50> {}
impl Reg128BitsConcat<49, 99> for Reg128Bits<50> {}
impl Reg128BitsDownCast<50> for Reg128Bits<50> {}
impl Reg128BitsConcat<50, 100> for Reg128Bits<50> {}
impl Reg128BitsDownCast<1> for Reg128Bits<51> {}
impl Reg128BitsConcat<1, 52> for Reg128Bits<51> {}
impl Reg128BitsDownCast<2> for Reg128Bits<51> {}
impl Reg128BitsConcat<2, 53> for Reg128Bits<51> {}
impl Reg128BitsDownCast<3> for Reg128Bits<51> {}
impl Reg128BitsConcat<3, 54> for Reg128Bits<51> {}
impl Reg128BitsDownCast<4> for Reg128Bits<51> {}
impl Reg128BitsConcat<4, 55> for Reg128Bits<51> {}
impl Reg128BitsDownCast<5> for Reg128Bits<51> {}
impl Reg128BitsConcat<5, 56> for Reg128Bits<51> {}
impl Reg128BitsDownCast<6> for Reg128Bits<51> {}
impl Reg128BitsConcat<6, 57> for Reg128Bits<51> {}
impl Reg128BitsDownCast<7> for Reg128Bits<51> {}
impl Reg128BitsConcat<7, 58> for Reg128Bits<51> {}
impl Reg128BitsDownCast<8> for Reg128Bits<51> {}
impl Reg128BitsConcat<8, 59> for Reg128Bits<51> {}
impl Reg128BitsDownCast<9> for Reg128Bits<51> {}
impl Reg128BitsConcat<9, 60> for Reg128Bits<51> {}
impl Reg128BitsDownCast<10> for Reg128Bits<51> {}
impl Reg128BitsConcat<10, 61> for Reg128Bits<51> {}
impl Reg128BitsDownCast<11> for Reg128Bits<51> {}
impl Reg128BitsConcat<11, 62> for Reg128Bits<51> {}
impl Reg128BitsDownCast<12> for Reg128Bits<51> {}
impl Reg128BitsConcat<12, 63> for Reg128Bits<51> {}
impl Reg128BitsDownCast<13> for Reg128Bits<51> {}
impl Reg128BitsConcat<13, 64> for Reg128Bits<51> {}
impl Reg128BitsDownCast<14> for Reg128Bits<51> {}
impl Reg128BitsConcat<14, 65> for Reg128Bits<51> {}
impl Reg128BitsDownCast<15> for Reg128Bits<51> {}
impl Reg128BitsConcat<15, 66> for Reg128Bits<51> {}
impl Reg128BitsDownCast<16> for Reg128Bits<51> {}
impl Reg128BitsConcat<16, 67> for Reg128Bits<51> {}
impl Reg128BitsDownCast<17> for Reg128Bits<51> {}
impl Reg128BitsConcat<17, 68> for Reg128Bits<51> {}
impl Reg128BitsDownCast<18> for Reg128Bits<51> {}
impl Reg128BitsConcat<18, 69> for Reg128Bits<51> {}
impl Reg128BitsDownCast<19> for Reg128Bits<51> {}
impl Reg128BitsConcat<19, 70> for Reg128Bits<51> {}
impl Reg128BitsDownCast<20> for Reg128Bits<51> {}
impl Reg128BitsConcat<20, 71> for Reg128Bits<51> {}
impl Reg128BitsDownCast<21> for Reg128Bits<51> {}
impl Reg128BitsConcat<21, 72> for Reg128Bits<51> {}
impl Reg128BitsDownCast<22> for Reg128Bits<51> {}
impl Reg128BitsConcat<22, 73> for Reg128Bits<51> {}
impl Reg128BitsDownCast<23> for Reg128Bits<51> {}
impl Reg128BitsConcat<23, 74> for Reg128Bits<51> {}
impl Reg128BitsDownCast<24> for Reg128Bits<51> {}
impl Reg128BitsConcat<24, 75> for Reg128Bits<51> {}
impl Reg128BitsDownCast<25> for Reg128Bits<51> {}
impl Reg128BitsConcat<25, 76> for Reg128Bits<51> {}
impl Reg128BitsDownCast<26> for Reg128Bits<51> {}
impl Reg128BitsConcat<26, 77> for Reg128Bits<51> {}
impl Reg128BitsDownCast<27> for Reg128Bits<51> {}
impl Reg128BitsConcat<27, 78> for Reg128Bits<51> {}
impl Reg128BitsDownCast<28> for Reg128Bits<51> {}
impl Reg128BitsConcat<28, 79> for Reg128Bits<51> {}
impl Reg128BitsDownCast<29> for Reg128Bits<51> {}
impl Reg128BitsConcat<29, 80> for Reg128Bits<51> {}
impl Reg128BitsDownCast<30> for Reg128Bits<51> {}
impl Reg128BitsConcat<30, 81> for Reg128Bits<51> {}
impl Reg128BitsDownCast<31> for Reg128Bits<51> {}
impl Reg128BitsConcat<31, 82> for Reg128Bits<51> {}
impl Reg128BitsDownCast<32> for Reg128Bits<51> {}
impl Reg128BitsConcat<32, 83> for Reg128Bits<51> {}
impl Reg128BitsDownCast<33> for Reg128Bits<51> {}
impl Reg128BitsConcat<33, 84> for Reg128Bits<51> {}
impl Reg128BitsDownCast<34> for Reg128Bits<51> {}
impl Reg128BitsConcat<34, 85> for Reg128Bits<51> {}
impl Reg128BitsDownCast<35> for Reg128Bits<51> {}
impl Reg128BitsConcat<35, 86> for Reg128Bits<51> {}
impl Reg128BitsDownCast<36> for Reg128Bits<51> {}
impl Reg128BitsConcat<36, 87> for Reg128Bits<51> {}
impl Reg128BitsDownCast<37> for Reg128Bits<51> {}
impl Reg128BitsConcat<37, 88> for Reg128Bits<51> {}
impl Reg128BitsDownCast<38> for Reg128Bits<51> {}
impl Reg128BitsConcat<38, 89> for Reg128Bits<51> {}
impl Reg128BitsDownCast<39> for Reg128Bits<51> {}
impl Reg128BitsConcat<39, 90> for Reg128Bits<51> {}
impl Reg128BitsDownCast<40> for Reg128Bits<51> {}
impl Reg128BitsConcat<40, 91> for Reg128Bits<51> {}
impl Reg128BitsDownCast<41> for Reg128Bits<51> {}
impl Reg128BitsConcat<41, 92> for Reg128Bits<51> {}
impl Reg128BitsDownCast<42> for Reg128Bits<51> {}
impl Reg128BitsConcat<42, 93> for Reg128Bits<51> {}
impl Reg128BitsDownCast<43> for Reg128Bits<51> {}
impl Reg128BitsConcat<43, 94> for Reg128Bits<51> {}
impl Reg128BitsDownCast<44> for Reg128Bits<51> {}
impl Reg128BitsConcat<44, 95> for Reg128Bits<51> {}
impl Reg128BitsDownCast<45> for Reg128Bits<51> {}
impl Reg128BitsConcat<45, 96> for Reg128Bits<51> {}
impl Reg128BitsDownCast<46> for Reg128Bits<51> {}
impl Reg128BitsConcat<46, 97> for Reg128Bits<51> {}
impl Reg128BitsDownCast<47> for Reg128Bits<51> {}
impl Reg128BitsConcat<47, 98> for Reg128Bits<51> {}
impl Reg128BitsDownCast<48> for Reg128Bits<51> {}
impl Reg128BitsConcat<48, 99> for Reg128Bits<51> {}
impl Reg128BitsDownCast<49> for Reg128Bits<51> {}
impl Reg128BitsConcat<49, 100> for Reg128Bits<51> {}
impl Reg128BitsDownCast<50> for Reg128Bits<51> {}
impl Reg128BitsConcat<50, 101> for Reg128Bits<51> {}
impl Reg128BitsDownCast<51> for Reg128Bits<51> {}
impl Reg128BitsConcat<51, 102> for Reg128Bits<51> {}
impl Reg128BitsDownCast<1> for Reg128Bits<52> {}
impl Reg128BitsConcat<1, 53> for Reg128Bits<52> {}
impl Reg128BitsDownCast<2> for Reg128Bits<52> {}
impl Reg128BitsConcat<2, 54> for Reg128Bits<52> {}
impl Reg128BitsDownCast<3> for Reg128Bits<52> {}
impl Reg128BitsConcat<3, 55> for Reg128Bits<52> {}
impl Reg128BitsDownCast<4> for Reg128Bits<52> {}
impl Reg128BitsConcat<4, 56> for Reg128Bits<52> {}
impl Reg128BitsDownCast<5> for Reg128Bits<52> {}
impl Reg128BitsConcat<5, 57> for Reg128Bits<52> {}
impl Reg128BitsDownCast<6> for Reg128Bits<52> {}
impl Reg128BitsConcat<6, 58> for Reg128Bits<52> {}
impl Reg128BitsDownCast<7> for Reg128Bits<52> {}
impl Reg128BitsConcat<7, 59> for Reg128Bits<52> {}
impl Reg128BitsDownCast<8> for Reg128Bits<52> {}
impl Reg128BitsConcat<8, 60> for Reg128Bits<52> {}
impl Reg128BitsDownCast<9> for Reg128Bits<52> {}
impl Reg128BitsConcat<9, 61> for Reg128Bits<52> {}
impl Reg128BitsDownCast<10> for Reg128Bits<52> {}
impl Reg128BitsConcat<10, 62> for Reg128Bits<52> {}
impl Reg128BitsDownCast<11> for Reg128Bits<52> {}
impl Reg128BitsConcat<11, 63> for Reg128Bits<52> {}
impl Reg128BitsDownCast<12> for Reg128Bits<52> {}
impl Reg128BitsConcat<12, 64> for Reg128Bits<52> {}
impl Reg128BitsDownCast<13> for Reg128Bits<52> {}
impl Reg128BitsConcat<13, 65> for Reg128Bits<52> {}
impl Reg128BitsDownCast<14> for Reg128Bits<52> {}
impl Reg128BitsConcat<14, 66> for Reg128Bits<52> {}
impl Reg128BitsDownCast<15> for Reg128Bits<52> {}
impl Reg128BitsConcat<15, 67> for Reg128Bits<52> {}
impl Reg128BitsDownCast<16> for Reg128Bits<52> {}
impl Reg128BitsConcat<16, 68> for Reg128Bits<52> {}
impl Reg128BitsDownCast<17> for Reg128Bits<52> {}
impl Reg128BitsConcat<17, 69> for Reg128Bits<52> {}
impl Reg128BitsDownCast<18> for Reg128Bits<52> {}
impl Reg128BitsConcat<18, 70> for Reg128Bits<52> {}
impl Reg128BitsDownCast<19> for Reg128Bits<52> {}
impl Reg128BitsConcat<19, 71> for Reg128Bits<52> {}
impl Reg128BitsDownCast<20> for Reg128Bits<52> {}
impl Reg128BitsConcat<20, 72> for Reg128Bits<52> {}
impl Reg128BitsDownCast<21> for Reg128Bits<52> {}
impl Reg128BitsConcat<21, 73> for Reg128Bits<52> {}
impl Reg128BitsDownCast<22> for Reg128Bits<52> {}
impl Reg128BitsConcat<22, 74> for Reg128Bits<52> {}
impl Reg128BitsDownCast<23> for Reg128Bits<52> {}
impl Reg128BitsConcat<23, 75> for Reg128Bits<52> {}
impl Reg128BitsDownCast<24> for Reg128Bits<52> {}
impl Reg128BitsConcat<24, 76> for Reg128Bits<52> {}
impl Reg128BitsDownCast<25> for Reg128Bits<52> {}
impl Reg128BitsConcat<25, 77> for Reg128Bits<52> {}
impl Reg128BitsDownCast<26> for Reg128Bits<52> {}
impl Reg128BitsConcat<26, 78> for Reg128Bits<52> {}
impl Reg128BitsDownCast<27> for Reg128Bits<52> {}
impl Reg128BitsConcat<27, 79> for Reg128Bits<52> {}
impl Reg128BitsDownCast<28> for Reg128Bits<52> {}
impl Reg128BitsConcat<28, 80> for Reg128Bits<52> {}
impl Reg128BitsDownCast<29> for Reg128Bits<52> {}
impl Reg128BitsConcat<29, 81> for Reg128Bits<52> {}
impl Reg128BitsDownCast<30> for Reg128Bits<52> {}
impl Reg128BitsConcat<30, 82> for Reg128Bits<52> {}
impl Reg128BitsDownCast<31> for Reg128Bits<52> {}
impl Reg128BitsConcat<31, 83> for Reg128Bits<52> {}
impl Reg128BitsDownCast<32> for Reg128Bits<52> {}
impl Reg128BitsConcat<32, 84> for Reg128Bits<52> {}
impl Reg128BitsDownCast<33> for Reg128Bits<52> {}
impl Reg128BitsConcat<33, 85> for Reg128Bits<52> {}
impl Reg128BitsDownCast<34> for Reg128Bits<52> {}
impl Reg128BitsConcat<34, 86> for Reg128Bits<52> {}
impl Reg128BitsDownCast<35> for Reg128Bits<52> {}
impl Reg128BitsConcat<35, 87> for Reg128Bits<52> {}
impl Reg128BitsDownCast<36> for Reg128Bits<52> {}
impl Reg128BitsConcat<36, 88> for Reg128Bits<52> {}
impl Reg128BitsDownCast<37> for Reg128Bits<52> {}
impl Reg128BitsConcat<37, 89> for Reg128Bits<52> {}
impl Reg128BitsDownCast<38> for Reg128Bits<52> {}
impl Reg128BitsConcat<38, 90> for Reg128Bits<52> {}
impl Reg128BitsDownCast<39> for Reg128Bits<52> {}
impl Reg128BitsConcat<39, 91> for Reg128Bits<52> {}
impl Reg128BitsDownCast<40> for Reg128Bits<52> {}
impl Reg128BitsConcat<40, 92> for Reg128Bits<52> {}
impl Reg128BitsDownCast<41> for Reg128Bits<52> {}
impl Reg128BitsConcat<41, 93> for Reg128Bits<52> {}
impl Reg128BitsDownCast<42> for Reg128Bits<52> {}
impl Reg128BitsConcat<42, 94> for Reg128Bits<52> {}
impl Reg128BitsDownCast<43> for Reg128Bits<52> {}
impl Reg128BitsConcat<43, 95> for Reg128Bits<52> {}
impl Reg128BitsDownCast<44> for Reg128Bits<52> {}
impl Reg128BitsConcat<44, 96> for Reg128Bits<52> {}
impl Reg128BitsDownCast<45> for Reg128Bits<52> {}
impl Reg128BitsConcat<45, 97> for Reg128Bits<52> {}
impl Reg128BitsDownCast<46> for Reg128Bits<52> {}
impl Reg128BitsConcat<46, 98> for Reg128Bits<52> {}
impl Reg128BitsDownCast<47> for Reg128Bits<52> {}
impl Reg128BitsConcat<47, 99> for Reg128Bits<52> {}
impl Reg128BitsDownCast<48> for Reg128Bits<52> {}
impl Reg128BitsConcat<48, 100> for Reg128Bits<52> {}
impl Reg128BitsDownCast<49> for Reg128Bits<52> {}
impl Reg128BitsConcat<49, 101> for Reg128Bits<52> {}
impl Reg128BitsDownCast<50> for Reg128Bits<52> {}
impl Reg128BitsConcat<50, 102> for Reg128Bits<52> {}
impl Reg128BitsDownCast<51> for Reg128Bits<52> {}
impl Reg128BitsConcat<51, 103> for Reg128Bits<52> {}
impl Reg128BitsDownCast<52> for Reg128Bits<52> {}
impl Reg128BitsConcat<52, 104> for Reg128Bits<52> {}
impl Reg128BitsDownCast<1> for Reg128Bits<53> {}
impl Reg128BitsConcat<1, 54> for Reg128Bits<53> {}
impl Reg128BitsDownCast<2> for Reg128Bits<53> {}
impl Reg128BitsConcat<2, 55> for Reg128Bits<53> {}
impl Reg128BitsDownCast<3> for Reg128Bits<53> {}
impl Reg128BitsConcat<3, 56> for Reg128Bits<53> {}
impl Reg128BitsDownCast<4> for Reg128Bits<53> {}
impl Reg128BitsConcat<4, 57> for Reg128Bits<53> {}
impl Reg128BitsDownCast<5> for Reg128Bits<53> {}
impl Reg128BitsConcat<5, 58> for Reg128Bits<53> {}
impl Reg128BitsDownCast<6> for Reg128Bits<53> {}
impl Reg128BitsConcat<6, 59> for Reg128Bits<53> {}
impl Reg128BitsDownCast<7> for Reg128Bits<53> {}
impl Reg128BitsConcat<7, 60> for Reg128Bits<53> {}
impl Reg128BitsDownCast<8> for Reg128Bits<53> {}
impl Reg128BitsConcat<8, 61> for Reg128Bits<53> {}
impl Reg128BitsDownCast<9> for Reg128Bits<53> {}
impl Reg128BitsConcat<9, 62> for Reg128Bits<53> {}
impl Reg128BitsDownCast<10> for Reg128Bits<53> {}
impl Reg128BitsConcat<10, 63> for Reg128Bits<53> {}
impl Reg128BitsDownCast<11> for Reg128Bits<53> {}
impl Reg128BitsConcat<11, 64> for Reg128Bits<53> {}
impl Reg128BitsDownCast<12> for Reg128Bits<53> {}
impl Reg128BitsConcat<12, 65> for Reg128Bits<53> {}
impl Reg128BitsDownCast<13> for Reg128Bits<53> {}
impl Reg128BitsConcat<13, 66> for Reg128Bits<53> {}
impl Reg128BitsDownCast<14> for Reg128Bits<53> {}
impl Reg128BitsConcat<14, 67> for Reg128Bits<53> {}
impl Reg128BitsDownCast<15> for Reg128Bits<53> {}
impl Reg128BitsConcat<15, 68> for Reg128Bits<53> {}
impl Reg128BitsDownCast<16> for Reg128Bits<53> {}
impl Reg128BitsConcat<16, 69> for Reg128Bits<53> {}
impl Reg128BitsDownCast<17> for Reg128Bits<53> {}
impl Reg128BitsConcat<17, 70> for Reg128Bits<53> {}
impl Reg128BitsDownCast<18> for Reg128Bits<53> {}
impl Reg128BitsConcat<18, 71> for Reg128Bits<53> {}
impl Reg128BitsDownCast<19> for Reg128Bits<53> {}
impl Reg128BitsConcat<19, 72> for Reg128Bits<53> {}
impl Reg128BitsDownCast<20> for Reg128Bits<53> {}
impl Reg128BitsConcat<20, 73> for Reg128Bits<53> {}
impl Reg128BitsDownCast<21> for Reg128Bits<53> {}
impl Reg128BitsConcat<21, 74> for Reg128Bits<53> {}
impl Reg128BitsDownCast<22> for Reg128Bits<53> {}
impl Reg128BitsConcat<22, 75> for Reg128Bits<53> {}
impl Reg128BitsDownCast<23> for Reg128Bits<53> {}
impl Reg128BitsConcat<23, 76> for Reg128Bits<53> {}
impl Reg128BitsDownCast<24> for Reg128Bits<53> {}
impl Reg128BitsConcat<24, 77> for Reg128Bits<53> {}
impl Reg128BitsDownCast<25> for Reg128Bits<53> {}
impl Reg128BitsConcat<25, 78> for Reg128Bits<53> {}
impl Reg128BitsDownCast<26> for Reg128Bits<53> {}
impl Reg128BitsConcat<26, 79> for Reg128Bits<53> {}
impl Reg128BitsDownCast<27> for Reg128Bits<53> {}
impl Reg128BitsConcat<27, 80> for Reg128Bits<53> {}
impl Reg128BitsDownCast<28> for Reg128Bits<53> {}
impl Reg128BitsConcat<28, 81> for Reg128Bits<53> {}
impl Reg128BitsDownCast<29> for Reg128Bits<53> {}
impl Reg128BitsConcat<29, 82> for Reg128Bits<53> {}
impl Reg128BitsDownCast<30> for Reg128Bits<53> {}
impl Reg128BitsConcat<30, 83> for Reg128Bits<53> {}
impl Reg128BitsDownCast<31> for Reg128Bits<53> {}
impl Reg128BitsConcat<31, 84> for Reg128Bits<53> {}
impl Reg128BitsDownCast<32> for Reg128Bits<53> {}
impl Reg128BitsConcat<32, 85> for Reg128Bits<53> {}
impl Reg128BitsDownCast<33> for Reg128Bits<53> {}
impl Reg128BitsConcat<33, 86> for Reg128Bits<53> {}
impl Reg128BitsDownCast<34> for Reg128Bits<53> {}
impl Reg128BitsConcat<34, 87> for Reg128Bits<53> {}
impl Reg128BitsDownCast<35> for Reg128Bits<53> {}
impl Reg128BitsConcat<35, 88> for Reg128Bits<53> {}
impl Reg128BitsDownCast<36> for Reg128Bits<53> {}
impl Reg128BitsConcat<36, 89> for Reg128Bits<53> {}
impl Reg128BitsDownCast<37> for Reg128Bits<53> {}
impl Reg128BitsConcat<37, 90> for Reg128Bits<53> {}
impl Reg128BitsDownCast<38> for Reg128Bits<53> {}
impl Reg128BitsConcat<38, 91> for Reg128Bits<53> {}
impl Reg128BitsDownCast<39> for Reg128Bits<53> {}
impl Reg128BitsConcat<39, 92> for Reg128Bits<53> {}
impl Reg128BitsDownCast<40> for Reg128Bits<53> {}
impl Reg128BitsConcat<40, 93> for Reg128Bits<53> {}
impl Reg128BitsDownCast<41> for Reg128Bits<53> {}
impl Reg128BitsConcat<41, 94> for Reg128Bits<53> {}
impl Reg128BitsDownCast<42> for Reg128Bits<53> {}
impl Reg128BitsConcat<42, 95> for Reg128Bits<53> {}
impl Reg128BitsDownCast<43> for Reg128Bits<53> {}
impl Reg128BitsConcat<43, 96> for Reg128Bits<53> {}
impl Reg128BitsDownCast<44> for Reg128Bits<53> {}
impl Reg128BitsConcat<44, 97> for Reg128Bits<53> {}
impl Reg128BitsDownCast<45> for Reg128Bits<53> {}
impl Reg128BitsConcat<45, 98> for Reg128Bits<53> {}
impl Reg128BitsDownCast<46> for Reg128Bits<53> {}
impl Reg128BitsConcat<46, 99> for Reg128Bits<53> {}
impl Reg128BitsDownCast<47> for Reg128Bits<53> {}
impl Reg128BitsConcat<47, 100> for Reg128Bits<53> {}
impl Reg128BitsDownCast<48> for Reg128Bits<53> {}
impl Reg128BitsConcat<48, 101> for Reg128Bits<53> {}
impl Reg128BitsDownCast<49> for Reg128Bits<53> {}
impl Reg128BitsConcat<49, 102> for Reg128Bits<53> {}
impl Reg128BitsDownCast<50> for Reg128Bits<53> {}
impl Reg128BitsConcat<50, 103> for Reg128Bits<53> {}
impl Reg128BitsDownCast<51> for Reg128Bits<53> {}
impl Reg128BitsConcat<51, 104> for Reg128Bits<53> {}
impl Reg128BitsDownCast<52> for Reg128Bits<53> {}
impl Reg128BitsConcat<52, 105> for Reg128Bits<53> {}
impl Reg128BitsDownCast<53> for Reg128Bits<53> {}
impl Reg128BitsConcat<53, 106> for Reg128Bits<53> {}
impl Reg128BitsDownCast<1> for Reg128Bits<54> {}
impl Reg128BitsConcat<1, 55> for Reg128Bits<54> {}
impl Reg128BitsDownCast<2> for Reg128Bits<54> {}
impl Reg128BitsConcat<2, 56> for Reg128Bits<54> {}
impl Reg128BitsDownCast<3> for Reg128Bits<54> {}
impl Reg128BitsConcat<3, 57> for Reg128Bits<54> {}
impl Reg128BitsDownCast<4> for Reg128Bits<54> {}
impl Reg128BitsConcat<4, 58> for Reg128Bits<54> {}
impl Reg128BitsDownCast<5> for Reg128Bits<54> {}
impl Reg128BitsConcat<5, 59> for Reg128Bits<54> {}
impl Reg128BitsDownCast<6> for Reg128Bits<54> {}
impl Reg128BitsConcat<6, 60> for Reg128Bits<54> {}
impl Reg128BitsDownCast<7> for Reg128Bits<54> {}
impl Reg128BitsConcat<7, 61> for Reg128Bits<54> {}
impl Reg128BitsDownCast<8> for Reg128Bits<54> {}
impl Reg128BitsConcat<8, 62> for Reg128Bits<54> {}
impl Reg128BitsDownCast<9> for Reg128Bits<54> {}
impl Reg128BitsConcat<9, 63> for Reg128Bits<54> {}
impl Reg128BitsDownCast<10> for Reg128Bits<54> {}
impl Reg128BitsConcat<10, 64> for Reg128Bits<54> {}
impl Reg128BitsDownCast<11> for Reg128Bits<54> {}
impl Reg128BitsConcat<11, 65> for Reg128Bits<54> {}
impl Reg128BitsDownCast<12> for Reg128Bits<54> {}
impl Reg128BitsConcat<12, 66> for Reg128Bits<54> {}
impl Reg128BitsDownCast<13> for Reg128Bits<54> {}
impl Reg128BitsConcat<13, 67> for Reg128Bits<54> {}
impl Reg128BitsDownCast<14> for Reg128Bits<54> {}
impl Reg128BitsConcat<14, 68> for Reg128Bits<54> {}
impl Reg128BitsDownCast<15> for Reg128Bits<54> {}
impl Reg128BitsConcat<15, 69> for Reg128Bits<54> {}
impl Reg128BitsDownCast<16> for Reg128Bits<54> {}
impl Reg128BitsConcat<16, 70> for Reg128Bits<54> {}
impl Reg128BitsDownCast<17> for Reg128Bits<54> {}
impl Reg128BitsConcat<17, 71> for Reg128Bits<54> {}
impl Reg128BitsDownCast<18> for Reg128Bits<54> {}
impl Reg128BitsConcat<18, 72> for Reg128Bits<54> {}
impl Reg128BitsDownCast<19> for Reg128Bits<54> {}
impl Reg128BitsConcat<19, 73> for Reg128Bits<54> {}
impl Reg128BitsDownCast<20> for Reg128Bits<54> {}
impl Reg128BitsConcat<20, 74> for Reg128Bits<54> {}
impl Reg128BitsDownCast<21> for Reg128Bits<54> {}
impl Reg128BitsConcat<21, 75> for Reg128Bits<54> {}
impl Reg128BitsDownCast<22> for Reg128Bits<54> {}
impl Reg128BitsConcat<22, 76> for Reg128Bits<54> {}
impl Reg128BitsDownCast<23> for Reg128Bits<54> {}
impl Reg128BitsConcat<23, 77> for Reg128Bits<54> {}
impl Reg128BitsDownCast<24> for Reg128Bits<54> {}
impl Reg128BitsConcat<24, 78> for Reg128Bits<54> {}
impl Reg128BitsDownCast<25> for Reg128Bits<54> {}
impl Reg128BitsConcat<25, 79> for Reg128Bits<54> {}
impl Reg128BitsDownCast<26> for Reg128Bits<54> {}
impl Reg128BitsConcat<26, 80> for Reg128Bits<54> {}
impl Reg128BitsDownCast<27> for Reg128Bits<54> {}
impl Reg128BitsConcat<27, 81> for Reg128Bits<54> {}
impl Reg128BitsDownCast<28> for Reg128Bits<54> {}
impl Reg128BitsConcat<28, 82> for Reg128Bits<54> {}
impl Reg128BitsDownCast<29> for Reg128Bits<54> {}
impl Reg128BitsConcat<29, 83> for Reg128Bits<54> {}
impl Reg128BitsDownCast<30> for Reg128Bits<54> {}
impl Reg128BitsConcat<30, 84> for Reg128Bits<54> {}
impl Reg128BitsDownCast<31> for Reg128Bits<54> {}
impl Reg128BitsConcat<31, 85> for Reg128Bits<54> {}
impl Reg128BitsDownCast<32> for Reg128Bits<54> {}
impl Reg128BitsConcat<32, 86> for Reg128Bits<54> {}
impl Reg128BitsDownCast<33> for Reg128Bits<54> {}
impl Reg128BitsConcat<33, 87> for Reg128Bits<54> {}
impl Reg128BitsDownCast<34> for Reg128Bits<54> {}
impl Reg128BitsConcat<34, 88> for Reg128Bits<54> {}
impl Reg128BitsDownCast<35> for Reg128Bits<54> {}
impl Reg128BitsConcat<35, 89> for Reg128Bits<54> {}
impl Reg128BitsDownCast<36> for Reg128Bits<54> {}
impl Reg128BitsConcat<36, 90> for Reg128Bits<54> {}
impl Reg128BitsDownCast<37> for Reg128Bits<54> {}
impl Reg128BitsConcat<37, 91> for Reg128Bits<54> {}
impl Reg128BitsDownCast<38> for Reg128Bits<54> {}
impl Reg128BitsConcat<38, 92> for Reg128Bits<54> {}
impl Reg128BitsDownCast<39> for Reg128Bits<54> {}
impl Reg128BitsConcat<39, 93> for Reg128Bits<54> {}
impl Reg128BitsDownCast<40> for Reg128Bits<54> {}
impl Reg128BitsConcat<40, 94> for Reg128Bits<54> {}
impl Reg128BitsDownCast<41> for Reg128Bits<54> {}
impl Reg128BitsConcat<41, 95> for Reg128Bits<54> {}
impl Reg128BitsDownCast<42> for Reg128Bits<54> {}
impl Reg128BitsConcat<42, 96> for Reg128Bits<54> {}
impl Reg128BitsDownCast<43> for Reg128Bits<54> {}
impl Reg128BitsConcat<43, 97> for Reg128Bits<54> {}
impl Reg128BitsDownCast<44> for Reg128Bits<54> {}
impl Reg128BitsConcat<44, 98> for Reg128Bits<54> {}
impl Reg128BitsDownCast<45> for Reg128Bits<54> {}
impl Reg128BitsConcat<45, 99> for Reg128Bits<54> {}
impl Reg128BitsDownCast<46> for Reg128Bits<54> {}
impl Reg128BitsConcat<46, 100> for Reg128Bits<54> {}
impl Reg128BitsDownCast<47> for Reg128Bits<54> {}
impl Reg128BitsConcat<47, 101> for Reg128Bits<54> {}
impl Reg128BitsDownCast<48> for Reg128Bits<54> {}
impl Reg128BitsConcat<48, 102> for Reg128Bits<54> {}
impl Reg128BitsDownCast<49> for Reg128Bits<54> {}
impl Reg128BitsConcat<49, 103> for Reg128Bits<54> {}
impl Reg128BitsDownCast<50> for Reg128Bits<54> {}
impl Reg128BitsConcat<50, 104> for Reg128Bits<54> {}
impl Reg128BitsDownCast<51> for Reg128Bits<54> {}
impl Reg128BitsConcat<51, 105> for Reg128Bits<54> {}
impl Reg128BitsDownCast<52> for Reg128Bits<54> {}
impl Reg128BitsConcat<52, 106> for Reg128Bits<54> {}
impl Reg128BitsDownCast<53> for Reg128Bits<54> {}
impl Reg128BitsConcat<53, 107> for Reg128Bits<54> {}
impl Reg128BitsDownCast<54> for Reg128Bits<54> {}
impl Reg128BitsConcat<54, 108> for Reg128Bits<54> {}
impl Reg128BitsDownCast<1> for Reg128Bits<55> {}
impl Reg128BitsConcat<1, 56> for Reg128Bits<55> {}
impl Reg128BitsDownCast<2> for Reg128Bits<55> {}
impl Reg128BitsConcat<2, 57> for Reg128Bits<55> {}
impl Reg128BitsDownCast<3> for Reg128Bits<55> {}
impl Reg128BitsConcat<3, 58> for Reg128Bits<55> {}
impl Reg128BitsDownCast<4> for Reg128Bits<55> {}
impl Reg128BitsConcat<4, 59> for Reg128Bits<55> {}
impl Reg128BitsDownCast<5> for Reg128Bits<55> {}
impl Reg128BitsConcat<5, 60> for Reg128Bits<55> {}
impl Reg128BitsDownCast<6> for Reg128Bits<55> {}
impl Reg128BitsConcat<6, 61> for Reg128Bits<55> {}
impl Reg128BitsDownCast<7> for Reg128Bits<55> {}
impl Reg128BitsConcat<7, 62> for Reg128Bits<55> {}
impl Reg128BitsDownCast<8> for Reg128Bits<55> {}
impl Reg128BitsConcat<8, 63> for Reg128Bits<55> {}
impl Reg128BitsDownCast<9> for Reg128Bits<55> {}
impl Reg128BitsConcat<9, 64> for Reg128Bits<55> {}
impl Reg128BitsDownCast<10> for Reg128Bits<55> {}
impl Reg128BitsConcat<10, 65> for Reg128Bits<55> {}
impl Reg128BitsDownCast<11> for Reg128Bits<55> {}
impl Reg128BitsConcat<11, 66> for Reg128Bits<55> {}
impl Reg128BitsDownCast<12> for Reg128Bits<55> {}
impl Reg128BitsConcat<12, 67> for Reg128Bits<55> {}
impl Reg128BitsDownCast<13> for Reg128Bits<55> {}
impl Reg128BitsConcat<13, 68> for Reg128Bits<55> {}
impl Reg128BitsDownCast<14> for Reg128Bits<55> {}
impl Reg128BitsConcat<14, 69> for Reg128Bits<55> {}
impl Reg128BitsDownCast<15> for Reg128Bits<55> {}
impl Reg128BitsConcat<15, 70> for Reg128Bits<55> {}
impl Reg128BitsDownCast<16> for Reg128Bits<55> {}
impl Reg128BitsConcat<16, 71> for Reg128Bits<55> {}
impl Reg128BitsDownCast<17> for Reg128Bits<55> {}
impl Reg128BitsConcat<17, 72> for Reg128Bits<55> {}
impl Reg128BitsDownCast<18> for Reg128Bits<55> {}
impl Reg128BitsConcat<18, 73> for Reg128Bits<55> {}
impl Reg128BitsDownCast<19> for Reg128Bits<55> {}
impl Reg128BitsConcat<19, 74> for Reg128Bits<55> {}
impl Reg128BitsDownCast<20> for Reg128Bits<55> {}
impl Reg128BitsConcat<20, 75> for Reg128Bits<55> {}
impl Reg128BitsDownCast<21> for Reg128Bits<55> {}
impl Reg128BitsConcat<21, 76> for Reg128Bits<55> {}
impl Reg128BitsDownCast<22> for Reg128Bits<55> {}
impl Reg128BitsConcat<22, 77> for Reg128Bits<55> {}
impl Reg128BitsDownCast<23> for Reg128Bits<55> {}
impl Reg128BitsConcat<23, 78> for Reg128Bits<55> {}
impl Reg128BitsDownCast<24> for Reg128Bits<55> {}
impl Reg128BitsConcat<24, 79> for Reg128Bits<55> {}
impl Reg128BitsDownCast<25> for Reg128Bits<55> {}
impl Reg128BitsConcat<25, 80> for Reg128Bits<55> {}
impl Reg128BitsDownCast<26> for Reg128Bits<55> {}
impl Reg128BitsConcat<26, 81> for Reg128Bits<55> {}
impl Reg128BitsDownCast<27> for Reg128Bits<55> {}
impl Reg128BitsConcat<27, 82> for Reg128Bits<55> {}
impl Reg128BitsDownCast<28> for Reg128Bits<55> {}
impl Reg128BitsConcat<28, 83> for Reg128Bits<55> {}
impl Reg128BitsDownCast<29> for Reg128Bits<55> {}
impl Reg128BitsConcat<29, 84> for Reg128Bits<55> {}
impl Reg128BitsDownCast<30> for Reg128Bits<55> {}
impl Reg128BitsConcat<30, 85> for Reg128Bits<55> {}
impl Reg128BitsDownCast<31> for Reg128Bits<55> {}
impl Reg128BitsConcat<31, 86> for Reg128Bits<55> {}
impl Reg128BitsDownCast<32> for Reg128Bits<55> {}
impl Reg128BitsConcat<32, 87> for Reg128Bits<55> {}
impl Reg128BitsDownCast<33> for Reg128Bits<55> {}
impl Reg128BitsConcat<33, 88> for Reg128Bits<55> {}
impl Reg128BitsDownCast<34> for Reg128Bits<55> {}
impl Reg128BitsConcat<34, 89> for Reg128Bits<55> {}
impl Reg128BitsDownCast<35> for Reg128Bits<55> {}
impl Reg128BitsConcat<35, 90> for Reg128Bits<55> {}
impl Reg128BitsDownCast<36> for Reg128Bits<55> {}
impl Reg128BitsConcat<36, 91> for Reg128Bits<55> {}
impl Reg128BitsDownCast<37> for Reg128Bits<55> {}
impl Reg128BitsConcat<37, 92> for Reg128Bits<55> {}
impl Reg128BitsDownCast<38> for Reg128Bits<55> {}
impl Reg128BitsConcat<38, 93> for Reg128Bits<55> {}
impl Reg128BitsDownCast<39> for Reg128Bits<55> {}
impl Reg128BitsConcat<39, 94> for Reg128Bits<55> {}
impl Reg128BitsDownCast<40> for Reg128Bits<55> {}
impl Reg128BitsConcat<40, 95> for Reg128Bits<55> {}
impl Reg128BitsDownCast<41> for Reg128Bits<55> {}
impl Reg128BitsConcat<41, 96> for Reg128Bits<55> {}
impl Reg128BitsDownCast<42> for Reg128Bits<55> {}
impl Reg128BitsConcat<42, 97> for Reg128Bits<55> {}
impl Reg128BitsDownCast<43> for Reg128Bits<55> {}
impl Reg128BitsConcat<43, 98> for Reg128Bits<55> {}
impl Reg128BitsDownCast<44> for Reg128Bits<55> {}
impl Reg128BitsConcat<44, 99> for Reg128Bits<55> {}
impl Reg128BitsDownCast<45> for Reg128Bits<55> {}
impl Reg128BitsConcat<45, 100> for Reg128Bits<55> {}
impl Reg128BitsDownCast<46> for Reg128Bits<55> {}
impl Reg128BitsConcat<46, 101> for Reg128Bits<55> {}
impl Reg128BitsDownCast<47> for Reg128Bits<55> {}
impl Reg128BitsConcat<47, 102> for Reg128Bits<55> {}
impl Reg128BitsDownCast<48> for Reg128Bits<55> {}
impl Reg128BitsConcat<48, 103> for Reg128Bits<55> {}
impl Reg128BitsDownCast<49> for Reg128Bits<55> {}
impl Reg128BitsConcat<49, 104> for Reg128Bits<55> {}
impl Reg128BitsDownCast<50> for Reg128Bits<55> {}
impl Reg128BitsConcat<50, 105> for Reg128Bits<55> {}
impl Reg128BitsDownCast<51> for Reg128Bits<55> {}
impl Reg128BitsConcat<51, 106> for Reg128Bits<55> {}
impl Reg128BitsDownCast<52> for Reg128Bits<55> {}
impl Reg128BitsConcat<52, 107> for Reg128Bits<55> {}
impl Reg128BitsDownCast<53> for Reg128Bits<55> {}
impl Reg128BitsConcat<53, 108> for Reg128Bits<55> {}
impl Reg128BitsDownCast<54> for Reg128Bits<55> {}
impl Reg128BitsConcat<54, 109> for Reg128Bits<55> {}
impl Reg128BitsDownCast<55> for Reg128Bits<55> {}
impl Reg128BitsConcat<55, 110> for Reg128Bits<55> {}
impl Reg128BitsDownCast<1> for Reg128Bits<56> {}
impl Reg128BitsConcat<1, 57> for Reg128Bits<56> {}
impl Reg128BitsDownCast<2> for Reg128Bits<56> {}
impl Reg128BitsConcat<2, 58> for Reg128Bits<56> {}
impl Reg128BitsDownCast<3> for Reg128Bits<56> {}
impl Reg128BitsConcat<3, 59> for Reg128Bits<56> {}
impl Reg128BitsDownCast<4> for Reg128Bits<56> {}
impl Reg128BitsConcat<4, 60> for Reg128Bits<56> {}
impl Reg128BitsDownCast<5> for Reg128Bits<56> {}
impl Reg128BitsConcat<5, 61> for Reg128Bits<56> {}
impl Reg128BitsDownCast<6> for Reg128Bits<56> {}
impl Reg128BitsConcat<6, 62> for Reg128Bits<56> {}
impl Reg128BitsDownCast<7> for Reg128Bits<56> {}
impl Reg128BitsConcat<7, 63> for Reg128Bits<56> {}
impl Reg128BitsDownCast<8> for Reg128Bits<56> {}
impl Reg128BitsConcat<8, 64> for Reg128Bits<56> {}
impl Reg128BitsDownCast<9> for Reg128Bits<56> {}
impl Reg128BitsConcat<9, 65> for Reg128Bits<56> {}
impl Reg128BitsDownCast<10> for Reg128Bits<56> {}
impl Reg128BitsConcat<10, 66> for Reg128Bits<56> {}
impl Reg128BitsDownCast<11> for Reg128Bits<56> {}
impl Reg128BitsConcat<11, 67> for Reg128Bits<56> {}
impl Reg128BitsDownCast<12> for Reg128Bits<56> {}
impl Reg128BitsConcat<12, 68> for Reg128Bits<56> {}
impl Reg128BitsDownCast<13> for Reg128Bits<56> {}
impl Reg128BitsConcat<13, 69> for Reg128Bits<56> {}
impl Reg128BitsDownCast<14> for Reg128Bits<56> {}
impl Reg128BitsConcat<14, 70> for Reg128Bits<56> {}
impl Reg128BitsDownCast<15> for Reg128Bits<56> {}
impl Reg128BitsConcat<15, 71> for Reg128Bits<56> {}
impl Reg128BitsDownCast<16> for Reg128Bits<56> {}
impl Reg128BitsConcat<16, 72> for Reg128Bits<56> {}
impl Reg128BitsDownCast<17> for Reg128Bits<56> {}
impl Reg128BitsConcat<17, 73> for Reg128Bits<56> {}
impl Reg128BitsDownCast<18> for Reg128Bits<56> {}
impl Reg128BitsConcat<18, 74> for Reg128Bits<56> {}
impl Reg128BitsDownCast<19> for Reg128Bits<56> {}
impl Reg128BitsConcat<19, 75> for Reg128Bits<56> {}
impl Reg128BitsDownCast<20> for Reg128Bits<56> {}
impl Reg128BitsConcat<20, 76> for Reg128Bits<56> {}
impl Reg128BitsDownCast<21> for Reg128Bits<56> {}
impl Reg128BitsConcat<21, 77> for Reg128Bits<56> {}
impl Reg128BitsDownCast<22> for Reg128Bits<56> {}
impl Reg128BitsConcat<22, 78> for Reg128Bits<56> {}
impl Reg128BitsDownCast<23> for Reg128Bits<56> {}
impl Reg128BitsConcat<23, 79> for Reg128Bits<56> {}
impl Reg128BitsDownCast<24> for Reg128Bits<56> {}
impl Reg128BitsConcat<24, 80> for Reg128Bits<56> {}
impl Reg128BitsDownCast<25> for Reg128Bits<56> {}
impl Reg128BitsConcat<25, 81> for Reg128Bits<56> {}
impl Reg128BitsDownCast<26> for Reg128Bits<56> {}
impl Reg128BitsConcat<26, 82> for Reg128Bits<56> {}
impl Reg128BitsDownCast<27> for Reg128Bits<56> {}
impl Reg128BitsConcat<27, 83> for Reg128Bits<56> {}
impl Reg128BitsDownCast<28> for Reg128Bits<56> {}
impl Reg128BitsConcat<28, 84> for Reg128Bits<56> {}
impl Reg128BitsDownCast<29> for Reg128Bits<56> {}
impl Reg128BitsConcat<29, 85> for Reg128Bits<56> {}
impl Reg128BitsDownCast<30> for Reg128Bits<56> {}
impl Reg128BitsConcat<30, 86> for Reg128Bits<56> {}
impl Reg128BitsDownCast<31> for Reg128Bits<56> {}
impl Reg128BitsConcat<31, 87> for Reg128Bits<56> {}
impl Reg128BitsDownCast<32> for Reg128Bits<56> {}
impl Reg128BitsConcat<32, 88> for Reg128Bits<56> {}
impl Reg128BitsDownCast<33> for Reg128Bits<56> {}
impl Reg128BitsConcat<33, 89> for Reg128Bits<56> {}
impl Reg128BitsDownCast<34> for Reg128Bits<56> {}
impl Reg128BitsConcat<34, 90> for Reg128Bits<56> {}
impl Reg128BitsDownCast<35> for Reg128Bits<56> {}
impl Reg128BitsConcat<35, 91> for Reg128Bits<56> {}
impl Reg128BitsDownCast<36> for Reg128Bits<56> {}
impl Reg128BitsConcat<36, 92> for Reg128Bits<56> {}
impl Reg128BitsDownCast<37> for Reg128Bits<56> {}
impl Reg128BitsConcat<37, 93> for Reg128Bits<56> {}
impl Reg128BitsDownCast<38> for Reg128Bits<56> {}
impl Reg128BitsConcat<38, 94> for Reg128Bits<56> {}
impl Reg128BitsDownCast<39> for Reg128Bits<56> {}
impl Reg128BitsConcat<39, 95> for Reg128Bits<56> {}
impl Reg128BitsDownCast<40> for Reg128Bits<56> {}
impl Reg128BitsConcat<40, 96> for Reg128Bits<56> {}
impl Reg128BitsDownCast<41> for Reg128Bits<56> {}
impl Reg128BitsConcat<41, 97> for Reg128Bits<56> {}
impl Reg128BitsDownCast<42> for Reg128Bits<56> {}
impl Reg128BitsConcat<42, 98> for Reg128Bits<56> {}
impl Reg128BitsDownCast<43> for Reg128Bits<56> {}
impl Reg128BitsConcat<43, 99> for Reg128Bits<56> {}
impl Reg128BitsDownCast<44> for Reg128Bits<56> {}
impl Reg128BitsConcat<44, 100> for Reg128Bits<56> {}
impl Reg128BitsDownCast<45> for Reg128Bits<56> {}
impl Reg128BitsConcat<45, 101> for Reg128Bits<56> {}
impl Reg128BitsDownCast<46> for Reg128Bits<56> {}
impl Reg128BitsConcat<46, 102> for Reg128Bits<56> {}
impl Reg128BitsDownCast<47> for Reg128Bits<56> {}
impl Reg128BitsConcat<47, 103> for Reg128Bits<56> {}
impl Reg128BitsDownCast<48> for Reg128Bits<56> {}
impl Reg128BitsConcat<48, 104> for Reg128Bits<56> {}
impl Reg128BitsDownCast<49> for Reg128Bits<56> {}
impl Reg128BitsConcat<49, 105> for Reg128Bits<56> {}
impl Reg128BitsDownCast<50> for Reg128Bits<56> {}
impl Reg128BitsConcat<50, 106> for Reg128Bits<56> {}
impl Reg128BitsDownCast<51> for Reg128Bits<56> {}
impl Reg128BitsConcat<51, 107> for Reg128Bits<56> {}
impl Reg128BitsDownCast<52> for Reg128Bits<56> {}
impl Reg128BitsConcat<52, 108> for Reg128Bits<56> {}
impl Reg128BitsDownCast<53> for Reg128Bits<56> {}
impl Reg128BitsConcat<53, 109> for Reg128Bits<56> {}
impl Reg128BitsDownCast<54> for Reg128Bits<56> {}
impl Reg128BitsConcat<54, 110> for Reg128Bits<56> {}
impl Reg128BitsDownCast<55> for Reg128Bits<56> {}
impl Reg128BitsConcat<55, 111> for Reg128Bits<56> {}
impl Reg128BitsDownCast<56> for Reg128Bits<56> {}
impl Reg128BitsConcat<56, 112> for Reg128Bits<56> {}
impl Reg128BitsDownCast<1> for Reg128Bits<57> {}
impl Reg128BitsConcat<1, 58> for Reg128Bits<57> {}
impl Reg128BitsDownCast<2> for Reg128Bits<57> {}
impl Reg128BitsConcat<2, 59> for Reg128Bits<57> {}
impl Reg128BitsDownCast<3> for Reg128Bits<57> {}
impl Reg128BitsConcat<3, 60> for Reg128Bits<57> {}
impl Reg128BitsDownCast<4> for Reg128Bits<57> {}
impl Reg128BitsConcat<4, 61> for Reg128Bits<57> {}
impl Reg128BitsDownCast<5> for Reg128Bits<57> {}
impl Reg128BitsConcat<5, 62> for Reg128Bits<57> {}
impl Reg128BitsDownCast<6> for Reg128Bits<57> {}
impl Reg128BitsConcat<6, 63> for Reg128Bits<57> {}
impl Reg128BitsDownCast<7> for Reg128Bits<57> {}
impl Reg128BitsConcat<7, 64> for Reg128Bits<57> {}
impl Reg128BitsDownCast<8> for Reg128Bits<57> {}
impl Reg128BitsConcat<8, 65> for Reg128Bits<57> {}
impl Reg128BitsDownCast<9> for Reg128Bits<57> {}
impl Reg128BitsConcat<9, 66> for Reg128Bits<57> {}
impl Reg128BitsDownCast<10> for Reg128Bits<57> {}
impl Reg128BitsConcat<10, 67> for Reg128Bits<57> {}
impl Reg128BitsDownCast<11> for Reg128Bits<57> {}
impl Reg128BitsConcat<11, 68> for Reg128Bits<57> {}
impl Reg128BitsDownCast<12> for Reg128Bits<57> {}
impl Reg128BitsConcat<12, 69> for Reg128Bits<57> {}
impl Reg128BitsDownCast<13> for Reg128Bits<57> {}
impl Reg128BitsConcat<13, 70> for Reg128Bits<57> {}
impl Reg128BitsDownCast<14> for Reg128Bits<57> {}
impl Reg128BitsConcat<14, 71> for Reg128Bits<57> {}
impl Reg128BitsDownCast<15> for Reg128Bits<57> {}
impl Reg128BitsConcat<15, 72> for Reg128Bits<57> {}
impl Reg128BitsDownCast<16> for Reg128Bits<57> {}
impl Reg128BitsConcat<16, 73> for Reg128Bits<57> {}
impl Reg128BitsDownCast<17> for Reg128Bits<57> {}
impl Reg128BitsConcat<17, 74> for Reg128Bits<57> {}
impl Reg128BitsDownCast<18> for Reg128Bits<57> {}
impl Reg128BitsConcat<18, 75> for Reg128Bits<57> {}
impl Reg128BitsDownCast<19> for Reg128Bits<57> {}
impl Reg128BitsConcat<19, 76> for Reg128Bits<57> {}
impl Reg128BitsDownCast<20> for Reg128Bits<57> {}
impl Reg128BitsConcat<20, 77> for Reg128Bits<57> {}
impl Reg128BitsDownCast<21> for Reg128Bits<57> {}
impl Reg128BitsConcat<21, 78> for Reg128Bits<57> {}
impl Reg128BitsDownCast<22> for Reg128Bits<57> {}
impl Reg128BitsConcat<22, 79> for Reg128Bits<57> {}
impl Reg128BitsDownCast<23> for Reg128Bits<57> {}
impl Reg128BitsConcat<23, 80> for Reg128Bits<57> {}
impl Reg128BitsDownCast<24> for Reg128Bits<57> {}
impl Reg128BitsConcat<24, 81> for Reg128Bits<57> {}
impl Reg128BitsDownCast<25> for Reg128Bits<57> {}
impl Reg128BitsConcat<25, 82> for Reg128Bits<57> {}
impl Reg128BitsDownCast<26> for Reg128Bits<57> {}
impl Reg128BitsConcat<26, 83> for Reg128Bits<57> {}
impl Reg128BitsDownCast<27> for Reg128Bits<57> {}
impl Reg128BitsConcat<27, 84> for Reg128Bits<57> {}
impl Reg128BitsDownCast<28> for Reg128Bits<57> {}
impl Reg128BitsConcat<28, 85> for Reg128Bits<57> {}
impl Reg128BitsDownCast<29> for Reg128Bits<57> {}
impl Reg128BitsConcat<29, 86> for Reg128Bits<57> {}
impl Reg128BitsDownCast<30> for Reg128Bits<57> {}
impl Reg128BitsConcat<30, 87> for Reg128Bits<57> {}
impl Reg128BitsDownCast<31> for Reg128Bits<57> {}
impl Reg128BitsConcat<31, 88> for Reg128Bits<57> {}
impl Reg128BitsDownCast<32> for Reg128Bits<57> {}
impl Reg128BitsConcat<32, 89> for Reg128Bits<57> {}
impl Reg128BitsDownCast<33> for Reg128Bits<57> {}
impl Reg128BitsConcat<33, 90> for Reg128Bits<57> {}
impl Reg128BitsDownCast<34> for Reg128Bits<57> {}
impl Reg128BitsConcat<34, 91> for Reg128Bits<57> {}
impl Reg128BitsDownCast<35> for Reg128Bits<57> {}
impl Reg128BitsConcat<35, 92> for Reg128Bits<57> {}
impl Reg128BitsDownCast<36> for Reg128Bits<57> {}
impl Reg128BitsConcat<36, 93> for Reg128Bits<57> {}
impl Reg128BitsDownCast<37> for Reg128Bits<57> {}
impl Reg128BitsConcat<37, 94> for Reg128Bits<57> {}
impl Reg128BitsDownCast<38> for Reg128Bits<57> {}
impl Reg128BitsConcat<38, 95> for Reg128Bits<57> {}
impl Reg128BitsDownCast<39> for Reg128Bits<57> {}
impl Reg128BitsConcat<39, 96> for Reg128Bits<57> {}
impl Reg128BitsDownCast<40> for Reg128Bits<57> {}
impl Reg128BitsConcat<40, 97> for Reg128Bits<57> {}
impl Reg128BitsDownCast<41> for Reg128Bits<57> {}
impl Reg128BitsConcat<41, 98> for Reg128Bits<57> {}
impl Reg128BitsDownCast<42> for Reg128Bits<57> {}
impl Reg128BitsConcat<42, 99> for Reg128Bits<57> {}
impl Reg128BitsDownCast<43> for Reg128Bits<57> {}
impl Reg128BitsConcat<43, 100> for Reg128Bits<57> {}
impl Reg128BitsDownCast<44> for Reg128Bits<57> {}
impl Reg128BitsConcat<44, 101> for Reg128Bits<57> {}
impl Reg128BitsDownCast<45> for Reg128Bits<57> {}
impl Reg128BitsConcat<45, 102> for Reg128Bits<57> {}
impl Reg128BitsDownCast<46> for Reg128Bits<57> {}
impl Reg128BitsConcat<46, 103> for Reg128Bits<57> {}
impl Reg128BitsDownCast<47> for Reg128Bits<57> {}
impl Reg128BitsConcat<47, 104> for Reg128Bits<57> {}
impl Reg128BitsDownCast<48> for Reg128Bits<57> {}
impl Reg128BitsConcat<48, 105> for Reg128Bits<57> {}
impl Reg128BitsDownCast<49> for Reg128Bits<57> {}
impl Reg128BitsConcat<49, 106> for Reg128Bits<57> {}
impl Reg128BitsDownCast<50> for Reg128Bits<57> {}
impl Reg128BitsConcat<50, 107> for Reg128Bits<57> {}
impl Reg128BitsDownCast<51> for Reg128Bits<57> {}
impl Reg128BitsConcat<51, 108> for Reg128Bits<57> {}
impl Reg128BitsDownCast<52> for Reg128Bits<57> {}
impl Reg128BitsConcat<52, 109> for Reg128Bits<57> {}
impl Reg128BitsDownCast<53> for Reg128Bits<57> {}
impl Reg128BitsConcat<53, 110> for Reg128Bits<57> {}
impl Reg128BitsDownCast<54> for Reg128Bits<57> {}
impl Reg128BitsConcat<54, 111> for Reg128Bits<57> {}
impl Reg128BitsDownCast<55> for Reg128Bits<57> {}
impl Reg128BitsConcat<55, 112> for Reg128Bits<57> {}
impl Reg128BitsDownCast<56> for Reg128Bits<57> {}
impl Reg128BitsConcat<56, 113> for Reg128Bits<57> {}
impl Reg128BitsDownCast<57> for Reg128Bits<57> {}
impl Reg128BitsConcat<57, 114> for Reg128Bits<57> {}
impl Reg128BitsDownCast<1> for Reg128Bits<58> {}
impl Reg128BitsConcat<1, 59> for Reg128Bits<58> {}
impl Reg128BitsDownCast<2> for Reg128Bits<58> {}
impl Reg128BitsConcat<2, 60> for Reg128Bits<58> {}
impl Reg128BitsDownCast<3> for Reg128Bits<58> {}
impl Reg128BitsConcat<3, 61> for Reg128Bits<58> {}
impl Reg128BitsDownCast<4> for Reg128Bits<58> {}
impl Reg128BitsConcat<4, 62> for Reg128Bits<58> {}
impl Reg128BitsDownCast<5> for Reg128Bits<58> {}
impl Reg128BitsConcat<5, 63> for Reg128Bits<58> {}
impl Reg128BitsDownCast<6> for Reg128Bits<58> {}
impl Reg128BitsConcat<6, 64> for Reg128Bits<58> {}
impl Reg128BitsDownCast<7> for Reg128Bits<58> {}
impl Reg128BitsConcat<7, 65> for Reg128Bits<58> {}
impl Reg128BitsDownCast<8> for Reg128Bits<58> {}
impl Reg128BitsConcat<8, 66> for Reg128Bits<58> {}
impl Reg128BitsDownCast<9> for Reg128Bits<58> {}
impl Reg128BitsConcat<9, 67> for Reg128Bits<58> {}
impl Reg128BitsDownCast<10> for Reg128Bits<58> {}
impl Reg128BitsConcat<10, 68> for Reg128Bits<58> {}
impl Reg128BitsDownCast<11> for Reg128Bits<58> {}
impl Reg128BitsConcat<11, 69> for Reg128Bits<58> {}
impl Reg128BitsDownCast<12> for Reg128Bits<58> {}
impl Reg128BitsConcat<12, 70> for Reg128Bits<58> {}
impl Reg128BitsDownCast<13> for Reg128Bits<58> {}
impl Reg128BitsConcat<13, 71> for Reg128Bits<58> {}
impl Reg128BitsDownCast<14> for Reg128Bits<58> {}
impl Reg128BitsConcat<14, 72> for Reg128Bits<58> {}
impl Reg128BitsDownCast<15> for Reg128Bits<58> {}
impl Reg128BitsConcat<15, 73> for Reg128Bits<58> {}
impl Reg128BitsDownCast<16> for Reg128Bits<58> {}
impl Reg128BitsConcat<16, 74> for Reg128Bits<58> {}
impl Reg128BitsDownCast<17> for Reg128Bits<58> {}
impl Reg128BitsConcat<17, 75> for Reg128Bits<58> {}
impl Reg128BitsDownCast<18> for Reg128Bits<58> {}
impl Reg128BitsConcat<18, 76> for Reg128Bits<58> {}
impl Reg128BitsDownCast<19> for Reg128Bits<58> {}
impl Reg128BitsConcat<19, 77> for Reg128Bits<58> {}
impl Reg128BitsDownCast<20> for Reg128Bits<58> {}
impl Reg128BitsConcat<20, 78> for Reg128Bits<58> {}
impl Reg128BitsDownCast<21> for Reg128Bits<58> {}
impl Reg128BitsConcat<21, 79> for Reg128Bits<58> {}
impl Reg128BitsDownCast<22> for Reg128Bits<58> {}
impl Reg128BitsConcat<22, 80> for Reg128Bits<58> {}
impl Reg128BitsDownCast<23> for Reg128Bits<58> {}
impl Reg128BitsConcat<23, 81> for Reg128Bits<58> {}
impl Reg128BitsDownCast<24> for Reg128Bits<58> {}
impl Reg128BitsConcat<24, 82> for Reg128Bits<58> {}
impl Reg128BitsDownCast<25> for Reg128Bits<58> {}
impl Reg128BitsConcat<25, 83> for Reg128Bits<58> {}
impl Reg128BitsDownCast<26> for Reg128Bits<58> {}
impl Reg128BitsConcat<26, 84> for Reg128Bits<58> {}
impl Reg128BitsDownCast<27> for Reg128Bits<58> {}
impl Reg128BitsConcat<27, 85> for Reg128Bits<58> {}
impl Reg128BitsDownCast<28> for Reg128Bits<58> {}
impl Reg128BitsConcat<28, 86> for Reg128Bits<58> {}
impl Reg128BitsDownCast<29> for Reg128Bits<58> {}
impl Reg128BitsConcat<29, 87> for Reg128Bits<58> {}
impl Reg128BitsDownCast<30> for Reg128Bits<58> {}
impl Reg128BitsConcat<30, 88> for Reg128Bits<58> {}
impl Reg128BitsDownCast<31> for Reg128Bits<58> {}
impl Reg128BitsConcat<31, 89> for Reg128Bits<58> {}
impl Reg128BitsDownCast<32> for Reg128Bits<58> {}
impl Reg128BitsConcat<32, 90> for Reg128Bits<58> {}
impl Reg128BitsDownCast<33> for Reg128Bits<58> {}
impl Reg128BitsConcat<33, 91> for Reg128Bits<58> {}
impl Reg128BitsDownCast<34> for Reg128Bits<58> {}
impl Reg128BitsConcat<34, 92> for Reg128Bits<58> {}
impl Reg128BitsDownCast<35> for Reg128Bits<58> {}
impl Reg128BitsConcat<35, 93> for Reg128Bits<58> {}
impl Reg128BitsDownCast<36> for Reg128Bits<58> {}
impl Reg128BitsConcat<36, 94> for Reg128Bits<58> {}
impl Reg128BitsDownCast<37> for Reg128Bits<58> {}
impl Reg128BitsConcat<37, 95> for Reg128Bits<58> {}
impl Reg128BitsDownCast<38> for Reg128Bits<58> {}
impl Reg128BitsConcat<38, 96> for Reg128Bits<58> {}
impl Reg128BitsDownCast<39> for Reg128Bits<58> {}
impl Reg128BitsConcat<39, 97> for Reg128Bits<58> {}
impl Reg128BitsDownCast<40> for Reg128Bits<58> {}
impl Reg128BitsConcat<40, 98> for Reg128Bits<58> {}
impl Reg128BitsDownCast<41> for Reg128Bits<58> {}
impl Reg128BitsConcat<41, 99> for Reg128Bits<58> {}
impl Reg128BitsDownCast<42> for Reg128Bits<58> {}
impl Reg128BitsConcat<42, 100> for Reg128Bits<58> {}
impl Reg128BitsDownCast<43> for Reg128Bits<58> {}
impl Reg128BitsConcat<43, 101> for Reg128Bits<58> {}
impl Reg128BitsDownCast<44> for Reg128Bits<58> {}
impl Reg128BitsConcat<44, 102> for Reg128Bits<58> {}
impl Reg128BitsDownCast<45> for Reg128Bits<58> {}
impl Reg128BitsConcat<45, 103> for Reg128Bits<58> {}
impl Reg128BitsDownCast<46> for Reg128Bits<58> {}
impl Reg128BitsConcat<46, 104> for Reg128Bits<58> {}
impl Reg128BitsDownCast<47> for Reg128Bits<58> {}
impl Reg128BitsConcat<47, 105> for Reg128Bits<58> {}
impl Reg128BitsDownCast<48> for Reg128Bits<58> {}
impl Reg128BitsConcat<48, 106> for Reg128Bits<58> {}
impl Reg128BitsDownCast<49> for Reg128Bits<58> {}
impl Reg128BitsConcat<49, 107> for Reg128Bits<58> {}
impl Reg128BitsDownCast<50> for Reg128Bits<58> {}
impl Reg128BitsConcat<50, 108> for Reg128Bits<58> {}
impl Reg128BitsDownCast<51> for Reg128Bits<58> {}
impl Reg128BitsConcat<51, 109> for Reg128Bits<58> {}
impl Reg128BitsDownCast<52> for Reg128Bits<58> {}
impl Reg128BitsConcat<52, 110> for Reg128Bits<58> {}
impl Reg128BitsDownCast<53> for Reg128Bits<58> {}
impl Reg128BitsConcat<53, 111> for Reg128Bits<58> {}
impl Reg128BitsDownCast<54> for Reg128Bits<58> {}
impl Reg128BitsConcat<54, 112> for Reg128Bits<58> {}
impl Reg128BitsDownCast<55> for Reg128Bits<58> {}
impl Reg128BitsConcat<55, 113> for Reg128Bits<58> {}
impl Reg128BitsDownCast<56> for Reg128Bits<58> {}
impl Reg128BitsConcat<56, 114> for Reg128Bits<58> {}
impl Reg128BitsDownCast<57> for Reg128Bits<58> {}
impl Reg128BitsConcat<57, 115> for Reg128Bits<58> {}
impl Reg128BitsDownCast<58> for Reg128Bits<58> {}
impl Reg128BitsConcat<58, 116> for Reg128Bits<58> {}
impl Reg128BitsDownCast<1> for Reg128Bits<59> {}
impl Reg128BitsConcat<1, 60> for Reg128Bits<59> {}
impl Reg128BitsDownCast<2> for Reg128Bits<59> {}
impl Reg128BitsConcat<2, 61> for Reg128Bits<59> {}
impl Reg128BitsDownCast<3> for Reg128Bits<59> {}
impl Reg128BitsConcat<3, 62> for Reg128Bits<59> {}
impl Reg128BitsDownCast<4> for Reg128Bits<59> {}
impl Reg128BitsConcat<4, 63> for Reg128Bits<59> {}
impl Reg128BitsDownCast<5> for Reg128Bits<59> {}
impl Reg128BitsConcat<5, 64> for Reg128Bits<59> {}
impl Reg128BitsDownCast<6> for Reg128Bits<59> {}
impl Reg128BitsConcat<6, 65> for Reg128Bits<59> {}
impl Reg128BitsDownCast<7> for Reg128Bits<59> {}
impl Reg128BitsConcat<7, 66> for Reg128Bits<59> {}
impl Reg128BitsDownCast<8> for Reg128Bits<59> {}
impl Reg128BitsConcat<8, 67> for Reg128Bits<59> {}
impl Reg128BitsDownCast<9> for Reg128Bits<59> {}
impl Reg128BitsConcat<9, 68> for Reg128Bits<59> {}
impl Reg128BitsDownCast<10> for Reg128Bits<59> {}
impl Reg128BitsConcat<10, 69> for Reg128Bits<59> {}
impl Reg128BitsDownCast<11> for Reg128Bits<59> {}
impl Reg128BitsConcat<11, 70> for Reg128Bits<59> {}
impl Reg128BitsDownCast<12> for Reg128Bits<59> {}
impl Reg128BitsConcat<12, 71> for Reg128Bits<59> {}
impl Reg128BitsDownCast<13> for Reg128Bits<59> {}
impl Reg128BitsConcat<13, 72> for Reg128Bits<59> {}
impl Reg128BitsDownCast<14> for Reg128Bits<59> {}
impl Reg128BitsConcat<14, 73> for Reg128Bits<59> {}
impl Reg128BitsDownCast<15> for Reg128Bits<59> {}
impl Reg128BitsConcat<15, 74> for Reg128Bits<59> {}
impl Reg128BitsDownCast<16> for Reg128Bits<59> {}
impl Reg128BitsConcat<16, 75> for Reg128Bits<59> {}
impl Reg128BitsDownCast<17> for Reg128Bits<59> {}
impl Reg128BitsConcat<17, 76> for Reg128Bits<59> {}
impl Reg128BitsDownCast<18> for Reg128Bits<59> {}
impl Reg128BitsConcat<18, 77> for Reg128Bits<59> {}
impl Reg128BitsDownCast<19> for Reg128Bits<59> {}
impl Reg128BitsConcat<19, 78> for Reg128Bits<59> {}
impl Reg128BitsDownCast<20> for Reg128Bits<59> {}
impl Reg128BitsConcat<20, 79> for Reg128Bits<59> {}
impl Reg128BitsDownCast<21> for Reg128Bits<59> {}
impl Reg128BitsConcat<21, 80> for Reg128Bits<59> {}
impl Reg128BitsDownCast<22> for Reg128Bits<59> {}
impl Reg128BitsConcat<22, 81> for Reg128Bits<59> {}
impl Reg128BitsDownCast<23> for Reg128Bits<59> {}
impl Reg128BitsConcat<23, 82> for Reg128Bits<59> {}
impl Reg128BitsDownCast<24> for Reg128Bits<59> {}
impl Reg128BitsConcat<24, 83> for Reg128Bits<59> {}
impl Reg128BitsDownCast<25> for Reg128Bits<59> {}
impl Reg128BitsConcat<25, 84> for Reg128Bits<59> {}
impl Reg128BitsDownCast<26> for Reg128Bits<59> {}
impl Reg128BitsConcat<26, 85> for Reg128Bits<59> {}
impl Reg128BitsDownCast<27> for Reg128Bits<59> {}
impl Reg128BitsConcat<27, 86> for Reg128Bits<59> {}
impl Reg128BitsDownCast<28> for Reg128Bits<59> {}
impl Reg128BitsConcat<28, 87> for Reg128Bits<59> {}
impl Reg128BitsDownCast<29> for Reg128Bits<59> {}
impl Reg128BitsConcat<29, 88> for Reg128Bits<59> {}
impl Reg128BitsDownCast<30> for Reg128Bits<59> {}
impl Reg128BitsConcat<30, 89> for Reg128Bits<59> {}
impl Reg128BitsDownCast<31> for Reg128Bits<59> {}
impl Reg128BitsConcat<31, 90> for Reg128Bits<59> {}
impl Reg128BitsDownCast<32> for Reg128Bits<59> {}
impl Reg128BitsConcat<32, 91> for Reg128Bits<59> {}
impl Reg128BitsDownCast<33> for Reg128Bits<59> {}
impl Reg128BitsConcat<33, 92> for Reg128Bits<59> {}
impl Reg128BitsDownCast<34> for Reg128Bits<59> {}
impl Reg128BitsConcat<34, 93> for Reg128Bits<59> {}
impl Reg128BitsDownCast<35> for Reg128Bits<59> {}
impl Reg128BitsConcat<35, 94> for Reg128Bits<59> {}
impl Reg128BitsDownCast<36> for Reg128Bits<59> {}
impl Reg128BitsConcat<36, 95> for Reg128Bits<59> {}
impl Reg128BitsDownCast<37> for Reg128Bits<59> {}
impl Reg128BitsConcat<37, 96> for Reg128Bits<59> {}
impl Reg128BitsDownCast<38> for Reg128Bits<59> {}
impl Reg128BitsConcat<38, 97> for Reg128Bits<59> {}
impl Reg128BitsDownCast<39> for Reg128Bits<59> {}
impl Reg128BitsConcat<39, 98> for Reg128Bits<59> {}
impl Reg128BitsDownCast<40> for Reg128Bits<59> {}
impl Reg128BitsConcat<40, 99> for Reg128Bits<59> {}
impl Reg128BitsDownCast<41> for Reg128Bits<59> {}
impl Reg128BitsConcat<41, 100> for Reg128Bits<59> {}
impl Reg128BitsDownCast<42> for Reg128Bits<59> {}
impl Reg128BitsConcat<42, 101> for Reg128Bits<59> {}
impl Reg128BitsDownCast<43> for Reg128Bits<59> {}
impl Reg128BitsConcat<43, 102> for Reg128Bits<59> {}
impl Reg128BitsDownCast<44> for Reg128Bits<59> {}
impl Reg128BitsConcat<44, 103> for Reg128Bits<59> {}
impl Reg128BitsDownCast<45> for Reg128Bits<59> {}
impl Reg128BitsConcat<45, 104> for Reg128Bits<59> {}
impl Reg128BitsDownCast<46> for Reg128Bits<59> {}
impl Reg128BitsConcat<46, 105> for Reg128Bits<59> {}
impl Reg128BitsDownCast<47> for Reg128Bits<59> {}
impl Reg128BitsConcat<47, 106> for Reg128Bits<59> {}
impl Reg128BitsDownCast<48> for Reg128Bits<59> {}
impl Reg128BitsConcat<48, 107> for Reg128Bits<59> {}
impl Reg128BitsDownCast<49> for Reg128Bits<59> {}
impl Reg128BitsConcat<49, 108> for Reg128Bits<59> {}
impl Reg128BitsDownCast<50> for Reg128Bits<59> {}
impl Reg128BitsConcat<50, 109> for Reg128Bits<59> {}
impl Reg128BitsDownCast<51> for Reg128Bits<59> {}
impl Reg128BitsConcat<51, 110> for Reg128Bits<59> {}
impl Reg128BitsDownCast<52> for Reg128Bits<59> {}
impl Reg128BitsConcat<52, 111> for Reg128Bits<59> {}
impl Reg128BitsDownCast<53> for Reg128Bits<59> {}
impl Reg128BitsConcat<53, 112> for Reg128Bits<59> {}
impl Reg128BitsDownCast<54> for Reg128Bits<59> {}
impl Reg128BitsConcat<54, 113> for Reg128Bits<59> {}
impl Reg128BitsDownCast<55> for Reg128Bits<59> {}
impl Reg128BitsConcat<55, 114> for Reg128Bits<59> {}
impl Reg128BitsDownCast<56> for Reg128Bits<59> {}
impl Reg128BitsConcat<56, 115> for Reg128Bits<59> {}
impl Reg128BitsDownCast<57> for Reg128Bits<59> {}
impl Reg128BitsConcat<57, 116> for Reg128Bits<59> {}
impl Reg128BitsDownCast<58> for Reg128Bits<59> {}
impl Reg128BitsConcat<58, 117> for Reg128Bits<59> {}
impl Reg128BitsDownCast<59> for Reg128Bits<59> {}
impl Reg128BitsConcat<59, 118> for Reg128Bits<59> {}
impl Reg128BitsDownCast<1> for Reg128Bits<60> {}
impl Reg128BitsConcat<1, 61> for Reg128Bits<60> {}
impl Reg128BitsDownCast<2> for Reg128Bits<60> {}
impl Reg128BitsConcat<2, 62> for Reg128Bits<60> {}
impl Reg128BitsDownCast<3> for Reg128Bits<60> {}
impl Reg128BitsConcat<3, 63> for Reg128Bits<60> {}
impl Reg128BitsDownCast<4> for Reg128Bits<60> {}
impl Reg128BitsConcat<4, 64> for Reg128Bits<60> {}
impl Reg128BitsDownCast<5> for Reg128Bits<60> {}
impl Reg128BitsConcat<5, 65> for Reg128Bits<60> {}
impl Reg128BitsDownCast<6> for Reg128Bits<60> {}
impl Reg128BitsConcat<6, 66> for Reg128Bits<60> {}
impl Reg128BitsDownCast<7> for Reg128Bits<60> {}
impl Reg128BitsConcat<7, 67> for Reg128Bits<60> {}
impl Reg128BitsDownCast<8> for Reg128Bits<60> {}
impl Reg128BitsConcat<8, 68> for Reg128Bits<60> {}
impl Reg128BitsDownCast<9> for Reg128Bits<60> {}
impl Reg128BitsConcat<9, 69> for Reg128Bits<60> {}
impl Reg128BitsDownCast<10> for Reg128Bits<60> {}
impl Reg128BitsConcat<10, 70> for Reg128Bits<60> {}
impl Reg128BitsDownCast<11> for Reg128Bits<60> {}
impl Reg128BitsConcat<11, 71> for Reg128Bits<60> {}
impl Reg128BitsDownCast<12> for Reg128Bits<60> {}
impl Reg128BitsConcat<12, 72> for Reg128Bits<60> {}
impl Reg128BitsDownCast<13> for Reg128Bits<60> {}
impl Reg128BitsConcat<13, 73> for Reg128Bits<60> {}
impl Reg128BitsDownCast<14> for Reg128Bits<60> {}
impl Reg128BitsConcat<14, 74> for Reg128Bits<60> {}
impl Reg128BitsDownCast<15> for Reg128Bits<60> {}
impl Reg128BitsConcat<15, 75> for Reg128Bits<60> {}
impl Reg128BitsDownCast<16> for Reg128Bits<60> {}
impl Reg128BitsConcat<16, 76> for Reg128Bits<60> {}
impl Reg128BitsDownCast<17> for Reg128Bits<60> {}
impl Reg128BitsConcat<17, 77> for Reg128Bits<60> {}
impl Reg128BitsDownCast<18> for Reg128Bits<60> {}
impl Reg128BitsConcat<18, 78> for Reg128Bits<60> {}
impl Reg128BitsDownCast<19> for Reg128Bits<60> {}
impl Reg128BitsConcat<19, 79> for Reg128Bits<60> {}
impl Reg128BitsDownCast<20> for Reg128Bits<60> {}
impl Reg128BitsConcat<20, 80> for Reg128Bits<60> {}
impl Reg128BitsDownCast<21> for Reg128Bits<60> {}
impl Reg128BitsConcat<21, 81> for Reg128Bits<60> {}
impl Reg128BitsDownCast<22> for Reg128Bits<60> {}
impl Reg128BitsConcat<22, 82> for Reg128Bits<60> {}
impl Reg128BitsDownCast<23> for Reg128Bits<60> {}
impl Reg128BitsConcat<23, 83> for Reg128Bits<60> {}
impl Reg128BitsDownCast<24> for Reg128Bits<60> {}
impl Reg128BitsConcat<24, 84> for Reg128Bits<60> {}
impl Reg128BitsDownCast<25> for Reg128Bits<60> {}
impl Reg128BitsConcat<25, 85> for Reg128Bits<60> {}
impl Reg128BitsDownCast<26> for Reg128Bits<60> {}
impl Reg128BitsConcat<26, 86> for Reg128Bits<60> {}
impl Reg128BitsDownCast<27> for Reg128Bits<60> {}
impl Reg128BitsConcat<27, 87> for Reg128Bits<60> {}
impl Reg128BitsDownCast<28> for Reg128Bits<60> {}
impl Reg128BitsConcat<28, 88> for Reg128Bits<60> {}
impl Reg128BitsDownCast<29> for Reg128Bits<60> {}
impl Reg128BitsConcat<29, 89> for Reg128Bits<60> {}
impl Reg128BitsDownCast<30> for Reg128Bits<60> {}
impl Reg128BitsConcat<30, 90> for Reg128Bits<60> {}
impl Reg128BitsDownCast<31> for Reg128Bits<60> {}
impl Reg128BitsConcat<31, 91> for Reg128Bits<60> {}
impl Reg128BitsDownCast<32> for Reg128Bits<60> {}
impl Reg128BitsConcat<32, 92> for Reg128Bits<60> {}
impl Reg128BitsDownCast<33> for Reg128Bits<60> {}
impl Reg128BitsConcat<33, 93> for Reg128Bits<60> {}
impl Reg128BitsDownCast<34> for Reg128Bits<60> {}
impl Reg128BitsConcat<34, 94> for Reg128Bits<60> {}
impl Reg128BitsDownCast<35> for Reg128Bits<60> {}
impl Reg128BitsConcat<35, 95> for Reg128Bits<60> {}
impl Reg128BitsDownCast<36> for Reg128Bits<60> {}
impl Reg128BitsConcat<36, 96> for Reg128Bits<60> {}
impl Reg128BitsDownCast<37> for Reg128Bits<60> {}
impl Reg128BitsConcat<37, 97> for Reg128Bits<60> {}
impl Reg128BitsDownCast<38> for Reg128Bits<60> {}
impl Reg128BitsConcat<38, 98> for Reg128Bits<60> {}
impl Reg128BitsDownCast<39> for Reg128Bits<60> {}
impl Reg128BitsConcat<39, 99> for Reg128Bits<60> {}
impl Reg128BitsDownCast<40> for Reg128Bits<60> {}
impl Reg128BitsConcat<40, 100> for Reg128Bits<60> {}
impl Reg128BitsDownCast<41> for Reg128Bits<60> {}
impl Reg128BitsConcat<41, 101> for Reg128Bits<60> {}
impl Reg128BitsDownCast<42> for Reg128Bits<60> {}
impl Reg128BitsConcat<42, 102> for Reg128Bits<60> {}
impl Reg128BitsDownCast<43> for Reg128Bits<60> {}
impl Reg128BitsConcat<43, 103> for Reg128Bits<60> {}
impl Reg128BitsDownCast<44> for Reg128Bits<60> {}
impl Reg128BitsConcat<44, 104> for Reg128Bits<60> {}
impl Reg128BitsDownCast<45> for Reg128Bits<60> {}
impl Reg128BitsConcat<45, 105> for Reg128Bits<60> {}
impl Reg128BitsDownCast<46> for Reg128Bits<60> {}
impl Reg128BitsConcat<46, 106> for Reg128Bits<60> {}
impl Reg128BitsDownCast<47> for Reg128Bits<60> {}
impl Reg128BitsConcat<47, 107> for Reg128Bits<60> {}
impl Reg128BitsDownCast<48> for Reg128Bits<60> {}
impl Reg128BitsConcat<48, 108> for Reg128Bits<60> {}
impl Reg128BitsDownCast<49> for Reg128Bits<60> {}
impl Reg128BitsConcat<49, 109> for Reg128Bits<60> {}
impl Reg128BitsDownCast<50> for Reg128Bits<60> {}
impl Reg128BitsConcat<50, 110> for Reg128Bits<60> {}
impl Reg128BitsDownCast<51> for Reg128Bits<60> {}
impl Reg128BitsConcat<51, 111> for Reg128Bits<60> {}
impl Reg128BitsDownCast<52> for Reg128Bits<60> {}
impl Reg128BitsConcat<52, 112> for Reg128Bits<60> {}
impl Reg128BitsDownCast<53> for Reg128Bits<60> {}
impl Reg128BitsConcat<53, 113> for Reg128Bits<60> {}
impl Reg128BitsDownCast<54> for Reg128Bits<60> {}
impl Reg128BitsConcat<54, 114> for Reg128Bits<60> {}
impl Reg128BitsDownCast<55> for Reg128Bits<60> {}
impl Reg128BitsConcat<55, 115> for Reg128Bits<60> {}
impl Reg128BitsDownCast<56> for Reg128Bits<60> {}
impl Reg128BitsConcat<56, 116> for Reg128Bits<60> {}
impl Reg128BitsDownCast<57> for Reg128Bits<60> {}
impl Reg128BitsConcat<57, 117> for Reg128Bits<60> {}
impl Reg128BitsDownCast<58> for Reg128Bits<60> {}
impl Reg128BitsConcat<58, 118> for Reg128Bits<60> {}
impl Reg128BitsDownCast<59> for Reg128Bits<60> {}
impl Reg128BitsConcat<59, 119> for Reg128Bits<60> {}
impl Reg128BitsDownCast<60> for Reg128Bits<60> {}
impl Reg128BitsConcat<60, 120> for Reg128Bits<60> {}
impl Reg128BitsDownCast<1> for Reg128Bits<61> {}
impl Reg128BitsConcat<1, 62> for Reg128Bits<61> {}
impl Reg128BitsDownCast<2> for Reg128Bits<61> {}
impl Reg128BitsConcat<2, 63> for Reg128Bits<61> {}
impl Reg128BitsDownCast<3> for Reg128Bits<61> {}
impl Reg128BitsConcat<3, 64> for Reg128Bits<61> {}
impl Reg128BitsDownCast<4> for Reg128Bits<61> {}
impl Reg128BitsConcat<4, 65> for Reg128Bits<61> {}
impl Reg128BitsDownCast<5> for Reg128Bits<61> {}
impl Reg128BitsConcat<5, 66> for Reg128Bits<61> {}
impl Reg128BitsDownCast<6> for Reg128Bits<61> {}
impl Reg128BitsConcat<6, 67> for Reg128Bits<61> {}
impl Reg128BitsDownCast<7> for Reg128Bits<61> {}
impl Reg128BitsConcat<7, 68> for Reg128Bits<61> {}
impl Reg128BitsDownCast<8> for Reg128Bits<61> {}
impl Reg128BitsConcat<8, 69> for Reg128Bits<61> {}
impl Reg128BitsDownCast<9> for Reg128Bits<61> {}
impl Reg128BitsConcat<9, 70> for Reg128Bits<61> {}
impl Reg128BitsDownCast<10> for Reg128Bits<61> {}
impl Reg128BitsConcat<10, 71> for Reg128Bits<61> {}
impl Reg128BitsDownCast<11> for Reg128Bits<61> {}
impl Reg128BitsConcat<11, 72> for Reg128Bits<61> {}
impl Reg128BitsDownCast<12> for Reg128Bits<61> {}
impl Reg128BitsConcat<12, 73> for Reg128Bits<61> {}
impl Reg128BitsDownCast<13> for Reg128Bits<61> {}
impl Reg128BitsConcat<13, 74> for Reg128Bits<61> {}
impl Reg128BitsDownCast<14> for Reg128Bits<61> {}
impl Reg128BitsConcat<14, 75> for Reg128Bits<61> {}
impl Reg128BitsDownCast<15> for Reg128Bits<61> {}
impl Reg128BitsConcat<15, 76> for Reg128Bits<61> {}
impl Reg128BitsDownCast<16> for Reg128Bits<61> {}
impl Reg128BitsConcat<16, 77> for Reg128Bits<61> {}
impl Reg128BitsDownCast<17> for Reg128Bits<61> {}
impl Reg128BitsConcat<17, 78> for Reg128Bits<61> {}
impl Reg128BitsDownCast<18> for Reg128Bits<61> {}
impl Reg128BitsConcat<18, 79> for Reg128Bits<61> {}
impl Reg128BitsDownCast<19> for Reg128Bits<61> {}
impl Reg128BitsConcat<19, 80> for Reg128Bits<61> {}
impl Reg128BitsDownCast<20> for Reg128Bits<61> {}
impl Reg128BitsConcat<20, 81> for Reg128Bits<61> {}
impl Reg128BitsDownCast<21> for Reg128Bits<61> {}
impl Reg128BitsConcat<21, 82> for Reg128Bits<61> {}
impl Reg128BitsDownCast<22> for Reg128Bits<61> {}
impl Reg128BitsConcat<22, 83> for Reg128Bits<61> {}
impl Reg128BitsDownCast<23> for Reg128Bits<61> {}
impl Reg128BitsConcat<23, 84> for Reg128Bits<61> {}
impl Reg128BitsDownCast<24> for Reg128Bits<61> {}
impl Reg128BitsConcat<24, 85> for Reg128Bits<61> {}
impl Reg128BitsDownCast<25> for Reg128Bits<61> {}
impl Reg128BitsConcat<25, 86> for Reg128Bits<61> {}
impl Reg128BitsDownCast<26> for Reg128Bits<61> {}
impl Reg128BitsConcat<26, 87> for Reg128Bits<61> {}
impl Reg128BitsDownCast<27> for Reg128Bits<61> {}
impl Reg128BitsConcat<27, 88> for Reg128Bits<61> {}
impl Reg128BitsDownCast<28> for Reg128Bits<61> {}
impl Reg128BitsConcat<28, 89> for Reg128Bits<61> {}
impl Reg128BitsDownCast<29> for Reg128Bits<61> {}
impl Reg128BitsConcat<29, 90> for Reg128Bits<61> {}
impl Reg128BitsDownCast<30> for Reg128Bits<61> {}
impl Reg128BitsConcat<30, 91> for Reg128Bits<61> {}
impl Reg128BitsDownCast<31> for Reg128Bits<61> {}
impl Reg128BitsConcat<31, 92> for Reg128Bits<61> {}
impl Reg128BitsDownCast<32> for Reg128Bits<61> {}
impl Reg128BitsConcat<32, 93> for Reg128Bits<61> {}
impl Reg128BitsDownCast<33> for Reg128Bits<61> {}
impl Reg128BitsConcat<33, 94> for Reg128Bits<61> {}
impl Reg128BitsDownCast<34> for Reg128Bits<61> {}
impl Reg128BitsConcat<34, 95> for Reg128Bits<61> {}
impl Reg128BitsDownCast<35> for Reg128Bits<61> {}
impl Reg128BitsConcat<35, 96> for Reg128Bits<61> {}
impl Reg128BitsDownCast<36> for Reg128Bits<61> {}
impl Reg128BitsConcat<36, 97> for Reg128Bits<61> {}
impl Reg128BitsDownCast<37> for Reg128Bits<61> {}
impl Reg128BitsConcat<37, 98> for Reg128Bits<61> {}
impl Reg128BitsDownCast<38> for Reg128Bits<61> {}
impl Reg128BitsConcat<38, 99> for Reg128Bits<61> {}
impl Reg128BitsDownCast<39> for Reg128Bits<61> {}
impl Reg128BitsConcat<39, 100> for Reg128Bits<61> {}
impl Reg128BitsDownCast<40> for Reg128Bits<61> {}
impl Reg128BitsConcat<40, 101> for Reg128Bits<61> {}
impl Reg128BitsDownCast<41> for Reg128Bits<61> {}
impl Reg128BitsConcat<41, 102> for Reg128Bits<61> {}
impl Reg128BitsDownCast<42> for Reg128Bits<61> {}
impl Reg128BitsConcat<42, 103> for Reg128Bits<61> {}
impl Reg128BitsDownCast<43> for Reg128Bits<61> {}
impl Reg128BitsConcat<43, 104> for Reg128Bits<61> {}
impl Reg128BitsDownCast<44> for Reg128Bits<61> {}
impl Reg128BitsConcat<44, 105> for Reg128Bits<61> {}
impl Reg128BitsDownCast<45> for Reg128Bits<61> {}
impl Reg128BitsConcat<45, 106> for Reg128Bits<61> {}
impl Reg128BitsDownCast<46> for Reg128Bits<61> {}
impl Reg128BitsConcat<46, 107> for Reg128Bits<61> {}
impl Reg128BitsDownCast<47> for Reg128Bits<61> {}
impl Reg128BitsConcat<47, 108> for Reg128Bits<61> {}
impl Reg128BitsDownCast<48> for Reg128Bits<61> {}
impl Reg128BitsConcat<48, 109> for Reg128Bits<61> {}
impl Reg128BitsDownCast<49> for Reg128Bits<61> {}
impl Reg128BitsConcat<49, 110> for Reg128Bits<61> {}
impl Reg128BitsDownCast<50> for Reg128Bits<61> {}
impl Reg128BitsConcat<50, 111> for Reg128Bits<61> {}
impl Reg128BitsDownCast<51> for Reg128Bits<61> {}
impl Reg128BitsConcat<51, 112> for Reg128Bits<61> {}
impl Reg128BitsDownCast<52> for Reg128Bits<61> {}
impl Reg128BitsConcat<52, 113> for Reg128Bits<61> {}
impl Reg128BitsDownCast<53> for Reg128Bits<61> {}
impl Reg128BitsConcat<53, 114> for Reg128Bits<61> {}
impl Reg128BitsDownCast<54> for Reg128Bits<61> {}
impl Reg128BitsConcat<54, 115> for Reg128Bits<61> {}
impl Reg128BitsDownCast<55> for Reg128Bits<61> {}
impl Reg128BitsConcat<55, 116> for Reg128Bits<61> {}
impl Reg128BitsDownCast<56> for Reg128Bits<61> {}
impl Reg128BitsConcat<56, 117> for Reg128Bits<61> {}
impl Reg128BitsDownCast<57> for Reg128Bits<61> {}
impl Reg128BitsConcat<57, 118> for Reg128Bits<61> {}
impl Reg128BitsDownCast<58> for Reg128Bits<61> {}
impl Reg128BitsConcat<58, 119> for Reg128Bits<61> {}
impl Reg128BitsDownCast<59> for Reg128Bits<61> {}
impl Reg128BitsConcat<59, 120> for Reg128Bits<61> {}
impl Reg128BitsDownCast<60> for Reg128Bits<61> {}
impl Reg128BitsConcat<60, 121> for Reg128Bits<61> {}
impl Reg128BitsDownCast<61> for Reg128Bits<61> {}
impl Reg128BitsConcat<61, 122> for Reg128Bits<61> {}
impl Reg128BitsDownCast<1> for Reg128Bits<62> {}
impl Reg128BitsConcat<1, 63> for Reg128Bits<62> {}
impl Reg128BitsDownCast<2> for Reg128Bits<62> {}
impl Reg128BitsConcat<2, 64> for Reg128Bits<62> {}
impl Reg128BitsDownCast<3> for Reg128Bits<62> {}
impl Reg128BitsConcat<3, 65> for Reg128Bits<62> {}
impl Reg128BitsDownCast<4> for Reg128Bits<62> {}
impl Reg128BitsConcat<4, 66> for Reg128Bits<62> {}
impl Reg128BitsDownCast<5> for Reg128Bits<62> {}
impl Reg128BitsConcat<5, 67> for Reg128Bits<62> {}
impl Reg128BitsDownCast<6> for Reg128Bits<62> {}
impl Reg128BitsConcat<6, 68> for Reg128Bits<62> {}
impl Reg128BitsDownCast<7> for Reg128Bits<62> {}
impl Reg128BitsConcat<7, 69> for Reg128Bits<62> {}
impl Reg128BitsDownCast<8> for Reg128Bits<62> {}
impl Reg128BitsConcat<8, 70> for Reg128Bits<62> {}
impl Reg128BitsDownCast<9> for Reg128Bits<62> {}
impl Reg128BitsConcat<9, 71> for Reg128Bits<62> {}
impl Reg128BitsDownCast<10> for Reg128Bits<62> {}
impl Reg128BitsConcat<10, 72> for Reg128Bits<62> {}
impl Reg128BitsDownCast<11> for Reg128Bits<62> {}
impl Reg128BitsConcat<11, 73> for Reg128Bits<62> {}
impl Reg128BitsDownCast<12> for Reg128Bits<62> {}
impl Reg128BitsConcat<12, 74> for Reg128Bits<62> {}
impl Reg128BitsDownCast<13> for Reg128Bits<62> {}
impl Reg128BitsConcat<13, 75> for Reg128Bits<62> {}
impl Reg128BitsDownCast<14> for Reg128Bits<62> {}
impl Reg128BitsConcat<14, 76> for Reg128Bits<62> {}
impl Reg128BitsDownCast<15> for Reg128Bits<62> {}
impl Reg128BitsConcat<15, 77> for Reg128Bits<62> {}
impl Reg128BitsDownCast<16> for Reg128Bits<62> {}
impl Reg128BitsConcat<16, 78> for Reg128Bits<62> {}
impl Reg128BitsDownCast<17> for Reg128Bits<62> {}
impl Reg128BitsConcat<17, 79> for Reg128Bits<62> {}
impl Reg128BitsDownCast<18> for Reg128Bits<62> {}
impl Reg128BitsConcat<18, 80> for Reg128Bits<62> {}
impl Reg128BitsDownCast<19> for Reg128Bits<62> {}
impl Reg128BitsConcat<19, 81> for Reg128Bits<62> {}
impl Reg128BitsDownCast<20> for Reg128Bits<62> {}
impl Reg128BitsConcat<20, 82> for Reg128Bits<62> {}
impl Reg128BitsDownCast<21> for Reg128Bits<62> {}
impl Reg128BitsConcat<21, 83> for Reg128Bits<62> {}
impl Reg128BitsDownCast<22> for Reg128Bits<62> {}
impl Reg128BitsConcat<22, 84> for Reg128Bits<62> {}
impl Reg128BitsDownCast<23> for Reg128Bits<62> {}
impl Reg128BitsConcat<23, 85> for Reg128Bits<62> {}
impl Reg128BitsDownCast<24> for Reg128Bits<62> {}
impl Reg128BitsConcat<24, 86> for Reg128Bits<62> {}
impl Reg128BitsDownCast<25> for Reg128Bits<62> {}
impl Reg128BitsConcat<25, 87> for Reg128Bits<62> {}
impl Reg128BitsDownCast<26> for Reg128Bits<62> {}
impl Reg128BitsConcat<26, 88> for Reg128Bits<62> {}
impl Reg128BitsDownCast<27> for Reg128Bits<62> {}
impl Reg128BitsConcat<27, 89> for Reg128Bits<62> {}
impl Reg128BitsDownCast<28> for Reg128Bits<62> {}
impl Reg128BitsConcat<28, 90> for Reg128Bits<62> {}
impl Reg128BitsDownCast<29> for Reg128Bits<62> {}
impl Reg128BitsConcat<29, 91> for Reg128Bits<62> {}
impl Reg128BitsDownCast<30> for Reg128Bits<62> {}
impl Reg128BitsConcat<30, 92> for Reg128Bits<62> {}
impl Reg128BitsDownCast<31> for Reg128Bits<62> {}
impl Reg128BitsConcat<31, 93> for Reg128Bits<62> {}
impl Reg128BitsDownCast<32> for Reg128Bits<62> {}
impl Reg128BitsConcat<32, 94> for Reg128Bits<62> {}
impl Reg128BitsDownCast<33> for Reg128Bits<62> {}
impl Reg128BitsConcat<33, 95> for Reg128Bits<62> {}
impl Reg128BitsDownCast<34> for Reg128Bits<62> {}
impl Reg128BitsConcat<34, 96> for Reg128Bits<62> {}
impl Reg128BitsDownCast<35> for Reg128Bits<62> {}
impl Reg128BitsConcat<35, 97> for Reg128Bits<62> {}
impl Reg128BitsDownCast<36> for Reg128Bits<62> {}
impl Reg128BitsConcat<36, 98> for Reg128Bits<62> {}
impl Reg128BitsDownCast<37> for Reg128Bits<62> {}
impl Reg128BitsConcat<37, 99> for Reg128Bits<62> {}
impl Reg128BitsDownCast<38> for Reg128Bits<62> {}
impl Reg128BitsConcat<38, 100> for Reg128Bits<62> {}
impl Reg128BitsDownCast<39> for Reg128Bits<62> {}
impl Reg128BitsConcat<39, 101> for Reg128Bits<62> {}
impl Reg128BitsDownCast<40> for Reg128Bits<62> {}
impl Reg128BitsConcat<40, 102> for Reg128Bits<62> {}
impl Reg128BitsDownCast<41> for Reg128Bits<62> {}
impl Reg128BitsConcat<41, 103> for Reg128Bits<62> {}
impl Reg128BitsDownCast<42> for Reg128Bits<62> {}
impl Reg128BitsConcat<42, 104> for Reg128Bits<62> {}
impl Reg128BitsDownCast<43> for Reg128Bits<62> {}
impl Reg128BitsConcat<43, 105> for Reg128Bits<62> {}
impl Reg128BitsDownCast<44> for Reg128Bits<62> {}
impl Reg128BitsConcat<44, 106> for Reg128Bits<62> {}
impl Reg128BitsDownCast<45> for Reg128Bits<62> {}
impl Reg128BitsConcat<45, 107> for Reg128Bits<62> {}
impl Reg128BitsDownCast<46> for Reg128Bits<62> {}
impl Reg128BitsConcat<46, 108> for Reg128Bits<62> {}
impl Reg128BitsDownCast<47> for Reg128Bits<62> {}
impl Reg128BitsConcat<47, 109> for Reg128Bits<62> {}
impl Reg128BitsDownCast<48> for Reg128Bits<62> {}
impl Reg128BitsConcat<48, 110> for Reg128Bits<62> {}
impl Reg128BitsDownCast<49> for Reg128Bits<62> {}
impl Reg128BitsConcat<49, 111> for Reg128Bits<62> {}
impl Reg128BitsDownCast<50> for Reg128Bits<62> {}
impl Reg128BitsConcat<50, 112> for Reg128Bits<62> {}
impl Reg128BitsDownCast<51> for Reg128Bits<62> {}
impl Reg128BitsConcat<51, 113> for Reg128Bits<62> {}
impl Reg128BitsDownCast<52> for Reg128Bits<62> {}
impl Reg128BitsConcat<52, 114> for Reg128Bits<62> {}
impl Reg128BitsDownCast<53> for Reg128Bits<62> {}
impl Reg128BitsConcat<53, 115> for Reg128Bits<62> {}
impl Reg128BitsDownCast<54> for Reg128Bits<62> {}
impl Reg128BitsConcat<54, 116> for Reg128Bits<62> {}
impl Reg128BitsDownCast<55> for Reg128Bits<62> {}
impl Reg128BitsConcat<55, 117> for Reg128Bits<62> {}
impl Reg128BitsDownCast<56> for Reg128Bits<62> {}
impl Reg128BitsConcat<56, 118> for Reg128Bits<62> {}
impl Reg128BitsDownCast<57> for Reg128Bits<62> {}
impl Reg128BitsConcat<57, 119> for Reg128Bits<62> {}
impl Reg128BitsDownCast<58> for Reg128Bits<62> {}
impl Reg128BitsConcat<58, 120> for Reg128Bits<62> {}
impl Reg128BitsDownCast<59> for Reg128Bits<62> {}
impl Reg128BitsConcat<59, 121> for Reg128Bits<62> {}
impl Reg128BitsDownCast<60> for Reg128Bits<62> {}
impl Reg128BitsConcat<60, 122> for Reg128Bits<62> {}
impl Reg128BitsDownCast<61> for Reg128Bits<62> {}
impl Reg128BitsConcat<61, 123> for Reg128Bits<62> {}
impl Reg128BitsDownCast<62> for Reg128Bits<62> {}
impl Reg128BitsConcat<62, 124> for Reg128Bits<62> {}
impl Reg128BitsDownCast<1> for Reg128Bits<63> {}
impl Reg128BitsConcat<1, 64> for Reg128Bits<63> {}
impl Reg128BitsDownCast<2> for Reg128Bits<63> {}
impl Reg128BitsConcat<2, 65> for Reg128Bits<63> {}
impl Reg128BitsDownCast<3> for Reg128Bits<63> {}
impl Reg128BitsConcat<3, 66> for Reg128Bits<63> {}
impl Reg128BitsDownCast<4> for Reg128Bits<63> {}
impl Reg128BitsConcat<4, 67> for Reg128Bits<63> {}
impl Reg128BitsDownCast<5> for Reg128Bits<63> {}
impl Reg128BitsConcat<5, 68> for Reg128Bits<63> {}
impl Reg128BitsDownCast<6> for Reg128Bits<63> {}
impl Reg128BitsConcat<6, 69> for Reg128Bits<63> {}
impl Reg128BitsDownCast<7> for Reg128Bits<63> {}
impl Reg128BitsConcat<7, 70> for Reg128Bits<63> {}
impl Reg128BitsDownCast<8> for Reg128Bits<63> {}
impl Reg128BitsConcat<8, 71> for Reg128Bits<63> {}
impl Reg128BitsDownCast<9> for Reg128Bits<63> {}
impl Reg128BitsConcat<9, 72> for Reg128Bits<63> {}
impl Reg128BitsDownCast<10> for Reg128Bits<63> {}
impl Reg128BitsConcat<10, 73> for Reg128Bits<63> {}
impl Reg128BitsDownCast<11> for Reg128Bits<63> {}
impl Reg128BitsConcat<11, 74> for Reg128Bits<63> {}
impl Reg128BitsDownCast<12> for Reg128Bits<63> {}
impl Reg128BitsConcat<12, 75> for Reg128Bits<63> {}
impl Reg128BitsDownCast<13> for Reg128Bits<63> {}
impl Reg128BitsConcat<13, 76> for Reg128Bits<63> {}
impl Reg128BitsDownCast<14> for Reg128Bits<63> {}
impl Reg128BitsConcat<14, 77> for Reg128Bits<63> {}
impl Reg128BitsDownCast<15> for Reg128Bits<63> {}
impl Reg128BitsConcat<15, 78> for Reg128Bits<63> {}
impl Reg128BitsDownCast<16> for Reg128Bits<63> {}
impl Reg128BitsConcat<16, 79> for Reg128Bits<63> {}
impl Reg128BitsDownCast<17> for Reg128Bits<63> {}
impl Reg128BitsConcat<17, 80> for Reg128Bits<63> {}
impl Reg128BitsDownCast<18> for Reg128Bits<63> {}
impl Reg128BitsConcat<18, 81> for Reg128Bits<63> {}
impl Reg128BitsDownCast<19> for Reg128Bits<63> {}
impl Reg128BitsConcat<19, 82> for Reg128Bits<63> {}
impl Reg128BitsDownCast<20> for Reg128Bits<63> {}
impl Reg128BitsConcat<20, 83> for Reg128Bits<63> {}
impl Reg128BitsDownCast<21> for Reg128Bits<63> {}
impl Reg128BitsConcat<21, 84> for Reg128Bits<63> {}
impl Reg128BitsDownCast<22> for Reg128Bits<63> {}
impl Reg128BitsConcat<22, 85> for Reg128Bits<63> {}
impl Reg128BitsDownCast<23> for Reg128Bits<63> {}
impl Reg128BitsConcat<23, 86> for Reg128Bits<63> {}
impl Reg128BitsDownCast<24> for Reg128Bits<63> {}
impl Reg128BitsConcat<24, 87> for Reg128Bits<63> {}
impl Reg128BitsDownCast<25> for Reg128Bits<63> {}
impl Reg128BitsConcat<25, 88> for Reg128Bits<63> {}
impl Reg128BitsDownCast<26> for Reg128Bits<63> {}
impl Reg128BitsConcat<26, 89> for Reg128Bits<63> {}
impl Reg128BitsDownCast<27> for Reg128Bits<63> {}
impl Reg128BitsConcat<27, 90> for Reg128Bits<63> {}
impl Reg128BitsDownCast<28> for Reg128Bits<63> {}
impl Reg128BitsConcat<28, 91> for Reg128Bits<63> {}
impl Reg128BitsDownCast<29> for Reg128Bits<63> {}
impl Reg128BitsConcat<29, 92> for Reg128Bits<63> {}
impl Reg128BitsDownCast<30> for Reg128Bits<63> {}
impl Reg128BitsConcat<30, 93> for Reg128Bits<63> {}
impl Reg128BitsDownCast<31> for Reg128Bits<63> {}
impl Reg128BitsConcat<31, 94> for Reg128Bits<63> {}
impl Reg128BitsDownCast<32> for Reg128Bits<63> {}
impl Reg128BitsConcat<32, 95> for Reg128Bits<63> {}
impl Reg128BitsDownCast<33> for Reg128Bits<63> {}
impl Reg128BitsConcat<33, 96> for Reg128Bits<63> {}
impl Reg128BitsDownCast<34> for Reg128Bits<63> {}
impl Reg128BitsConcat<34, 97> for Reg128Bits<63> {}
impl Reg128BitsDownCast<35> for Reg128Bits<63> {}
impl Reg128BitsConcat<35, 98> for Reg128Bits<63> {}
impl Reg128BitsDownCast<36> for Reg128Bits<63> {}
impl Reg128BitsConcat<36, 99> for Reg128Bits<63> {}
impl Reg128BitsDownCast<37> for Reg128Bits<63> {}
impl Reg128BitsConcat<37, 100> for Reg128Bits<63> {}
impl Reg128BitsDownCast<38> for Reg128Bits<63> {}
impl Reg128BitsConcat<38, 101> for Reg128Bits<63> {}
impl Reg128BitsDownCast<39> for Reg128Bits<63> {}
impl Reg128BitsConcat<39, 102> for Reg128Bits<63> {}
impl Reg128BitsDownCast<40> for Reg128Bits<63> {}
impl Reg128BitsConcat<40, 103> for Reg128Bits<63> {}
impl Reg128BitsDownCast<41> for Reg128Bits<63> {}
impl Reg128BitsConcat<41, 104> for Reg128Bits<63> {}
impl Reg128BitsDownCast<42> for Reg128Bits<63> {}
impl Reg128BitsConcat<42, 105> for Reg128Bits<63> {}
impl Reg128BitsDownCast<43> for Reg128Bits<63> {}
impl Reg128BitsConcat<43, 106> for Reg128Bits<63> {}
impl Reg128BitsDownCast<44> for Reg128Bits<63> {}
impl Reg128BitsConcat<44, 107> for Reg128Bits<63> {}
impl Reg128BitsDownCast<45> for Reg128Bits<63> {}
impl Reg128BitsConcat<45, 108> for Reg128Bits<63> {}
impl Reg128BitsDownCast<46> for Reg128Bits<63> {}
impl Reg128BitsConcat<46, 109> for Reg128Bits<63> {}
impl Reg128BitsDownCast<47> for Reg128Bits<63> {}
impl Reg128BitsConcat<47, 110> for Reg128Bits<63> {}
impl Reg128BitsDownCast<48> for Reg128Bits<63> {}
impl Reg128BitsConcat<48, 111> for Reg128Bits<63> {}
impl Reg128BitsDownCast<49> for Reg128Bits<63> {}
impl Reg128BitsConcat<49, 112> for Reg128Bits<63> {}
impl Reg128BitsDownCast<50> for Reg128Bits<63> {}
impl Reg128BitsConcat<50, 113> for Reg128Bits<63> {}
impl Reg128BitsDownCast<51> for Reg128Bits<63> {}
impl Reg128BitsConcat<51, 114> for Reg128Bits<63> {}
impl Reg128BitsDownCast<52> for Reg128Bits<63> {}
impl Reg128BitsConcat<52, 115> for Reg128Bits<63> {}
impl Reg128BitsDownCast<53> for Reg128Bits<63> {}
impl Reg128BitsConcat<53, 116> for Reg128Bits<63> {}
impl Reg128BitsDownCast<54> for Reg128Bits<63> {}
impl Reg128BitsConcat<54, 117> for Reg128Bits<63> {}
impl Reg128BitsDownCast<55> for Reg128Bits<63> {}
impl Reg128BitsConcat<55, 118> for Reg128Bits<63> {}
impl Reg128BitsDownCast<56> for Reg128Bits<63> {}
impl Reg128BitsConcat<56, 119> for Reg128Bits<63> {}
impl Reg128BitsDownCast<57> for Reg128Bits<63> {}
impl Reg128BitsConcat<57, 120> for Reg128Bits<63> {}
impl Reg128BitsDownCast<58> for Reg128Bits<63> {}
impl Reg128BitsConcat<58, 121> for Reg128Bits<63> {}
impl Reg128BitsDownCast<59> for Reg128Bits<63> {}
impl Reg128BitsConcat<59, 122> for Reg128Bits<63> {}
impl Reg128BitsDownCast<60> for Reg128Bits<63> {}
impl Reg128BitsConcat<60, 123> for Reg128Bits<63> {}
impl Reg128BitsDownCast<61> for Reg128Bits<63> {}
impl Reg128BitsConcat<61, 124> for Reg128Bits<63> {}
impl Reg128BitsDownCast<62> for Reg128Bits<63> {}
impl Reg128BitsConcat<62, 125> for Reg128Bits<63> {}
impl Reg128BitsDownCast<63> for Reg128Bits<63> {}
impl Reg128BitsConcat<63, 126> for Reg128Bits<63> {}
impl Reg128BitsDownCast<1> for Reg128Bits<64> {}
impl Reg128BitsConcat<1, 65> for Reg128Bits<64> {}
impl Reg128BitsDownCast<2> for Reg128Bits<64> {}
impl Reg128BitsConcat<2, 66> for Reg128Bits<64> {}
impl Reg128BitsDownCast<3> for Reg128Bits<64> {}
impl Reg128BitsConcat<3, 67> for Reg128Bits<64> {}
impl Reg128BitsDownCast<4> for Reg128Bits<64> {}
impl Reg128BitsConcat<4, 68> for Reg128Bits<64> {}
impl Reg128BitsDownCast<5> for Reg128Bits<64> {}
impl Reg128BitsConcat<5, 69> for Reg128Bits<64> {}
impl Reg128BitsDownCast<6> for Reg128Bits<64> {}
impl Reg128BitsConcat<6, 70> for Reg128Bits<64> {}
impl Reg128BitsDownCast<7> for Reg128Bits<64> {}
impl Reg128BitsConcat<7, 71> for Reg128Bits<64> {}
impl Reg128BitsDownCast<8> for Reg128Bits<64> {}
impl Reg128BitsConcat<8, 72> for Reg128Bits<64> {}
impl Reg128BitsDownCast<9> for Reg128Bits<64> {}
impl Reg128BitsConcat<9, 73> for Reg128Bits<64> {}
impl Reg128BitsDownCast<10> for Reg128Bits<64> {}
impl Reg128BitsConcat<10, 74> for Reg128Bits<64> {}
impl Reg128BitsDownCast<11> for Reg128Bits<64> {}
impl Reg128BitsConcat<11, 75> for Reg128Bits<64> {}
impl Reg128BitsDownCast<12> for Reg128Bits<64> {}
impl Reg128BitsConcat<12, 76> for Reg128Bits<64> {}
impl Reg128BitsDownCast<13> for Reg128Bits<64> {}
impl Reg128BitsConcat<13, 77> for Reg128Bits<64> {}
impl Reg128BitsDownCast<14> for Reg128Bits<64> {}
impl Reg128BitsConcat<14, 78> for Reg128Bits<64> {}
impl Reg128BitsDownCast<15> for Reg128Bits<64> {}
impl Reg128BitsConcat<15, 79> for Reg128Bits<64> {}
impl Reg128BitsDownCast<16> for Reg128Bits<64> {}
impl Reg128BitsConcat<16, 80> for Reg128Bits<64> {}
impl Reg128BitsDownCast<17> for Reg128Bits<64> {}
impl Reg128BitsConcat<17, 81> for Reg128Bits<64> {}
impl Reg128BitsDownCast<18> for Reg128Bits<64> {}
impl Reg128BitsConcat<18, 82> for Reg128Bits<64> {}
impl Reg128BitsDownCast<19> for Reg128Bits<64> {}
impl Reg128BitsConcat<19, 83> for Reg128Bits<64> {}
impl Reg128BitsDownCast<20> for Reg128Bits<64> {}
impl Reg128BitsConcat<20, 84> for Reg128Bits<64> {}
impl Reg128BitsDownCast<21> for Reg128Bits<64> {}
impl Reg128BitsConcat<21, 85> for Reg128Bits<64> {}
impl Reg128BitsDownCast<22> for Reg128Bits<64> {}
impl Reg128BitsConcat<22, 86> for Reg128Bits<64> {}
impl Reg128BitsDownCast<23> for Reg128Bits<64> {}
impl Reg128BitsConcat<23, 87> for Reg128Bits<64> {}
impl Reg128BitsDownCast<24> for Reg128Bits<64> {}
impl Reg128BitsConcat<24, 88> for Reg128Bits<64> {}
impl Reg128BitsDownCast<25> for Reg128Bits<64> {}
impl Reg128BitsConcat<25, 89> for Reg128Bits<64> {}
impl Reg128BitsDownCast<26> for Reg128Bits<64> {}
impl Reg128BitsConcat<26, 90> for Reg128Bits<64> {}
impl Reg128BitsDownCast<27> for Reg128Bits<64> {}
impl Reg128BitsConcat<27, 91> for Reg128Bits<64> {}
impl Reg128BitsDownCast<28> for Reg128Bits<64> {}
impl Reg128BitsConcat<28, 92> for Reg128Bits<64> {}
impl Reg128BitsDownCast<29> for Reg128Bits<64> {}
impl Reg128BitsConcat<29, 93> for Reg128Bits<64> {}
impl Reg128BitsDownCast<30> for Reg128Bits<64> {}
impl Reg128BitsConcat<30, 94> for Reg128Bits<64> {}
impl Reg128BitsDownCast<31> for Reg128Bits<64> {}
impl Reg128BitsConcat<31, 95> for Reg128Bits<64> {}
impl Reg128BitsDownCast<32> for Reg128Bits<64> {}
impl Reg128BitsConcat<32, 96> for Reg128Bits<64> {}
impl Reg128BitsDownCast<33> for Reg128Bits<64> {}
impl Reg128BitsConcat<33, 97> for Reg128Bits<64> {}
impl Reg128BitsDownCast<34> for Reg128Bits<64> {}
impl Reg128BitsConcat<34, 98> for Reg128Bits<64> {}
impl Reg128BitsDownCast<35> for Reg128Bits<64> {}
impl Reg128BitsConcat<35, 99> for Reg128Bits<64> {}
impl Reg128BitsDownCast<36> for Reg128Bits<64> {}
impl Reg128BitsConcat<36, 100> for Reg128Bits<64> {}
impl Reg128BitsDownCast<37> for Reg128Bits<64> {}
impl Reg128BitsConcat<37, 101> for Reg128Bits<64> {}
impl Reg128BitsDownCast<38> for Reg128Bits<64> {}
impl Reg128BitsConcat<38, 102> for Reg128Bits<64> {}
impl Reg128BitsDownCast<39> for Reg128Bits<64> {}
impl Reg128BitsConcat<39, 103> for Reg128Bits<64> {}
impl Reg128BitsDownCast<40> for Reg128Bits<64> {}
impl Reg128BitsConcat<40, 104> for Reg128Bits<64> {}
impl Reg128BitsDownCast<41> for Reg128Bits<64> {}
impl Reg128BitsConcat<41, 105> for Reg128Bits<64> {}
impl Reg128BitsDownCast<42> for Reg128Bits<64> {}
impl Reg128BitsConcat<42, 106> for Reg128Bits<64> {}
impl Reg128BitsDownCast<43> for Reg128Bits<64> {}
impl Reg128BitsConcat<43, 107> for Reg128Bits<64> {}
impl Reg128BitsDownCast<44> for Reg128Bits<64> {}
impl Reg128BitsConcat<44, 108> for Reg128Bits<64> {}
impl Reg128BitsDownCast<45> for Reg128Bits<64> {}
impl Reg128BitsConcat<45, 109> for Reg128Bits<64> {}
impl Reg128BitsDownCast<46> for Reg128Bits<64> {}
impl Reg128BitsConcat<46, 110> for Reg128Bits<64> {}
impl Reg128BitsDownCast<47> for Reg128Bits<64> {}
impl Reg128BitsConcat<47, 111> for Reg128Bits<64> {}
impl Reg128BitsDownCast<48> for Reg128Bits<64> {}
impl Reg128BitsConcat<48, 112> for Reg128Bits<64> {}
impl Reg128BitsDownCast<49> for Reg128Bits<64> {}
impl Reg128BitsConcat<49, 113> for Reg128Bits<64> {}
impl Reg128BitsDownCast<50> for Reg128Bits<64> {}
impl Reg128BitsConcat<50, 114> for Reg128Bits<64> {}
impl Reg128BitsDownCast<51> for Reg128Bits<64> {}
impl Reg128BitsConcat<51, 115> for Reg128Bits<64> {}
impl Reg128BitsDownCast<52> for Reg128Bits<64> {}
impl Reg128BitsConcat<52, 116> for Reg128Bits<64> {}
impl Reg128BitsDownCast<53> for Reg128Bits<64> {}
impl Reg128BitsConcat<53, 117> for Reg128Bits<64> {}
impl Reg128BitsDownCast<54> for Reg128Bits<64> {}
impl Reg128BitsConcat<54, 118> for Reg128Bits<64> {}
impl Reg128BitsDownCast<55> for Reg128Bits<64> {}
impl Reg128BitsConcat<55, 119> for Reg128Bits<64> {}
impl Reg128BitsDownCast<56> for Reg128Bits<64> {}
impl Reg128BitsConcat<56, 120> for Reg128Bits<64> {}
impl Reg128BitsDownCast<57> for Reg128Bits<64> {}
impl Reg128BitsConcat<57, 121> for Reg128Bits<64> {}
impl Reg128BitsDownCast<58> for Reg128Bits<64> {}
impl Reg128BitsConcat<58, 122> for Reg128Bits<64> {}
impl Reg128BitsDownCast<59> for Reg128Bits<64> {}
impl Reg128BitsConcat<59, 123> for Reg128Bits<64> {}
impl Reg128BitsDownCast<60> for Reg128Bits<64> {}
impl Reg128BitsConcat<60, 124> for Reg128Bits<64> {}
impl Reg128BitsDownCast<61> for Reg128Bits<64> {}
impl Reg128BitsConcat<61, 125> for Reg128Bits<64> {}
impl Reg128BitsDownCast<62> for Reg128Bits<64> {}
impl Reg128BitsConcat<62, 126> for Reg128Bits<64> {}
impl Reg128BitsDownCast<63> for Reg128Bits<64> {}
impl Reg128BitsConcat<63, 127> for Reg128Bits<64> {}
impl Reg128BitsDownCast<64> for Reg128Bits<64> {}
impl Reg128BitsConcat<64, 128> for Reg128Bits<64> {}
impl Reg128BitsDownCast<1> for Reg128Bits<65> {}
impl Reg128BitsConcat<1, 66> for Reg128Bits<65> {}
impl Reg128BitsDownCast<2> for Reg128Bits<65> {}
impl Reg128BitsConcat<2, 67> for Reg128Bits<65> {}
impl Reg128BitsDownCast<3> for Reg128Bits<65> {}
impl Reg128BitsConcat<3, 68> for Reg128Bits<65> {}
impl Reg128BitsDownCast<4> for Reg128Bits<65> {}
impl Reg128BitsConcat<4, 69> for Reg128Bits<65> {}
impl Reg128BitsDownCast<5> for Reg128Bits<65> {}
impl Reg128BitsConcat<5, 70> for Reg128Bits<65> {}
impl Reg128BitsDownCast<6> for Reg128Bits<65> {}
impl Reg128BitsConcat<6, 71> for Reg128Bits<65> {}
impl Reg128BitsDownCast<7> for Reg128Bits<65> {}
impl Reg128BitsConcat<7, 72> for Reg128Bits<65> {}
impl Reg128BitsDownCast<8> for Reg128Bits<65> {}
impl Reg128BitsConcat<8, 73> for Reg128Bits<65> {}
impl Reg128BitsDownCast<9> for Reg128Bits<65> {}
impl Reg128BitsConcat<9, 74> for Reg128Bits<65> {}
impl Reg128BitsDownCast<10> for Reg128Bits<65> {}
impl Reg128BitsConcat<10, 75> for Reg128Bits<65> {}
impl Reg128BitsDownCast<11> for Reg128Bits<65> {}
impl Reg128BitsConcat<11, 76> for Reg128Bits<65> {}
impl Reg128BitsDownCast<12> for Reg128Bits<65> {}
impl Reg128BitsConcat<12, 77> for Reg128Bits<65> {}
impl Reg128BitsDownCast<13> for Reg128Bits<65> {}
impl Reg128BitsConcat<13, 78> for Reg128Bits<65> {}
impl Reg128BitsDownCast<14> for Reg128Bits<65> {}
impl Reg128BitsConcat<14, 79> for Reg128Bits<65> {}
impl Reg128BitsDownCast<15> for Reg128Bits<65> {}
impl Reg128BitsConcat<15, 80> for Reg128Bits<65> {}
impl Reg128BitsDownCast<16> for Reg128Bits<65> {}
impl Reg128BitsConcat<16, 81> for Reg128Bits<65> {}
impl Reg128BitsDownCast<17> for Reg128Bits<65> {}
impl Reg128BitsConcat<17, 82> for Reg128Bits<65> {}
impl Reg128BitsDownCast<18> for Reg128Bits<65> {}
impl Reg128BitsConcat<18, 83> for Reg128Bits<65> {}
impl Reg128BitsDownCast<19> for Reg128Bits<65> {}
impl Reg128BitsConcat<19, 84> for Reg128Bits<65> {}
impl Reg128BitsDownCast<20> for Reg128Bits<65> {}
impl Reg128BitsConcat<20, 85> for Reg128Bits<65> {}
impl Reg128BitsDownCast<21> for Reg128Bits<65> {}
impl Reg128BitsConcat<21, 86> for Reg128Bits<65> {}
impl Reg128BitsDownCast<22> for Reg128Bits<65> {}
impl Reg128BitsConcat<22, 87> for Reg128Bits<65> {}
impl Reg128BitsDownCast<23> for Reg128Bits<65> {}
impl Reg128BitsConcat<23, 88> for Reg128Bits<65> {}
impl Reg128BitsDownCast<24> for Reg128Bits<65> {}
impl Reg128BitsConcat<24, 89> for Reg128Bits<65> {}
impl Reg128BitsDownCast<25> for Reg128Bits<65> {}
impl Reg128BitsConcat<25, 90> for Reg128Bits<65> {}
impl Reg128BitsDownCast<26> for Reg128Bits<65> {}
impl Reg128BitsConcat<26, 91> for Reg128Bits<65> {}
impl Reg128BitsDownCast<27> for Reg128Bits<65> {}
impl Reg128BitsConcat<27, 92> for Reg128Bits<65> {}
impl Reg128BitsDownCast<28> for Reg128Bits<65> {}
impl Reg128BitsConcat<28, 93> for Reg128Bits<65> {}
impl Reg128BitsDownCast<29> for Reg128Bits<65> {}
impl Reg128BitsConcat<29, 94> for Reg128Bits<65> {}
impl Reg128BitsDownCast<30> for Reg128Bits<65> {}
impl Reg128BitsConcat<30, 95> for Reg128Bits<65> {}
impl Reg128BitsDownCast<31> for Reg128Bits<65> {}
impl Reg128BitsConcat<31, 96> for Reg128Bits<65> {}
impl Reg128BitsDownCast<32> for Reg128Bits<65> {}
impl Reg128BitsConcat<32, 97> for Reg128Bits<65> {}
impl Reg128BitsDownCast<33> for Reg128Bits<65> {}
impl Reg128BitsConcat<33, 98> for Reg128Bits<65> {}
impl Reg128BitsDownCast<34> for Reg128Bits<65> {}
impl Reg128BitsConcat<34, 99> for Reg128Bits<65> {}
impl Reg128BitsDownCast<35> for Reg128Bits<65> {}
impl Reg128BitsConcat<35, 100> for Reg128Bits<65> {}
impl Reg128BitsDownCast<36> for Reg128Bits<65> {}
impl Reg128BitsConcat<36, 101> for Reg128Bits<65> {}
impl Reg128BitsDownCast<37> for Reg128Bits<65> {}
impl Reg128BitsConcat<37, 102> for Reg128Bits<65> {}
impl Reg128BitsDownCast<38> for Reg128Bits<65> {}
impl Reg128BitsConcat<38, 103> for Reg128Bits<65> {}
impl Reg128BitsDownCast<39> for Reg128Bits<65> {}
impl Reg128BitsConcat<39, 104> for Reg128Bits<65> {}
impl Reg128BitsDownCast<40> for Reg128Bits<65> {}
impl Reg128BitsConcat<40, 105> for Reg128Bits<65> {}
impl Reg128BitsDownCast<41> for Reg128Bits<65> {}
impl Reg128BitsConcat<41, 106> for Reg128Bits<65> {}
impl Reg128BitsDownCast<42> for Reg128Bits<65> {}
impl Reg128BitsConcat<42, 107> for Reg128Bits<65> {}
impl Reg128BitsDownCast<43> for Reg128Bits<65> {}
impl Reg128BitsConcat<43, 108> for Reg128Bits<65> {}
impl Reg128BitsDownCast<44> for Reg128Bits<65> {}
impl Reg128BitsConcat<44, 109> for Reg128Bits<65> {}
impl Reg128BitsDownCast<45> for Reg128Bits<65> {}
impl Reg128BitsConcat<45, 110> for Reg128Bits<65> {}
impl Reg128BitsDownCast<46> for Reg128Bits<65> {}
impl Reg128BitsConcat<46, 111> for Reg128Bits<65> {}
impl Reg128BitsDownCast<47> for Reg128Bits<65> {}
impl Reg128BitsConcat<47, 112> for Reg128Bits<65> {}
impl Reg128BitsDownCast<48> for Reg128Bits<65> {}
impl Reg128BitsConcat<48, 113> for Reg128Bits<65> {}
impl Reg128BitsDownCast<49> for Reg128Bits<65> {}
impl Reg128BitsConcat<49, 114> for Reg128Bits<65> {}
impl Reg128BitsDownCast<50> for Reg128Bits<65> {}
impl Reg128BitsConcat<50, 115> for Reg128Bits<65> {}
impl Reg128BitsDownCast<51> for Reg128Bits<65> {}
impl Reg128BitsConcat<51, 116> for Reg128Bits<65> {}
impl Reg128BitsDownCast<52> for Reg128Bits<65> {}
impl Reg128BitsConcat<52, 117> for Reg128Bits<65> {}
impl Reg128BitsDownCast<53> for Reg128Bits<65> {}
impl Reg128BitsConcat<53, 118> for Reg128Bits<65> {}
impl Reg128BitsDownCast<54> for Reg128Bits<65> {}
impl Reg128BitsConcat<54, 119> for Reg128Bits<65> {}
impl Reg128BitsDownCast<55> for Reg128Bits<65> {}
impl Reg128BitsConcat<55, 120> for Reg128Bits<65> {}
impl Reg128BitsDownCast<56> for Reg128Bits<65> {}
impl Reg128BitsConcat<56, 121> for Reg128Bits<65> {}
impl Reg128BitsDownCast<57> for Reg128Bits<65> {}
impl Reg128BitsConcat<57, 122> for Reg128Bits<65> {}
impl Reg128BitsDownCast<58> for Reg128Bits<65> {}
impl Reg128BitsConcat<58, 123> for Reg128Bits<65> {}
impl Reg128BitsDownCast<59> for Reg128Bits<65> {}
impl Reg128BitsConcat<59, 124> for Reg128Bits<65> {}
impl Reg128BitsDownCast<60> for Reg128Bits<65> {}
impl Reg128BitsConcat<60, 125> for Reg128Bits<65> {}
impl Reg128BitsDownCast<61> for Reg128Bits<65> {}
impl Reg128BitsConcat<61, 126> for Reg128Bits<65> {}
impl Reg128BitsDownCast<62> for Reg128Bits<65> {}
impl Reg128BitsConcat<62, 127> for Reg128Bits<65> {}
impl Reg128BitsDownCast<63> for Reg128Bits<65> {}
impl Reg128BitsConcat<63, 128> for Reg128Bits<65> {}
impl Reg128BitsDownCast<64> for Reg128Bits<65> {}
impl Reg128BitsDownCast<65> for Reg128Bits<65> {}
impl Reg128BitsDownCast<1> for Reg128Bits<66> {}
impl Reg128BitsConcat<1, 67> for Reg128Bits<66> {}
impl Reg128BitsDownCast<2> for Reg128Bits<66> {}
impl Reg128BitsConcat<2, 68> for Reg128Bits<66> {}
impl Reg128BitsDownCast<3> for Reg128Bits<66> {}
impl Reg128BitsConcat<3, 69> for Reg128Bits<66> {}
impl Reg128BitsDownCast<4> for Reg128Bits<66> {}
impl Reg128BitsConcat<4, 70> for Reg128Bits<66> {}
impl Reg128BitsDownCast<5> for Reg128Bits<66> {}
impl Reg128BitsConcat<5, 71> for Reg128Bits<66> {}
impl Reg128BitsDownCast<6> for Reg128Bits<66> {}
impl Reg128BitsConcat<6, 72> for Reg128Bits<66> {}
impl Reg128BitsDownCast<7> for Reg128Bits<66> {}
impl Reg128BitsConcat<7, 73> for Reg128Bits<66> {}
impl Reg128BitsDownCast<8> for Reg128Bits<66> {}
impl Reg128BitsConcat<8, 74> for Reg128Bits<66> {}
impl Reg128BitsDownCast<9> for Reg128Bits<66> {}
impl Reg128BitsConcat<9, 75> for Reg128Bits<66> {}
impl Reg128BitsDownCast<10> for Reg128Bits<66> {}
impl Reg128BitsConcat<10, 76> for Reg128Bits<66> {}
impl Reg128BitsDownCast<11> for Reg128Bits<66> {}
impl Reg128BitsConcat<11, 77> for Reg128Bits<66> {}
impl Reg128BitsDownCast<12> for Reg128Bits<66> {}
impl Reg128BitsConcat<12, 78> for Reg128Bits<66> {}
impl Reg128BitsDownCast<13> for Reg128Bits<66> {}
impl Reg128BitsConcat<13, 79> for Reg128Bits<66> {}
impl Reg128BitsDownCast<14> for Reg128Bits<66> {}
impl Reg128BitsConcat<14, 80> for Reg128Bits<66> {}
impl Reg128BitsDownCast<15> for Reg128Bits<66> {}
impl Reg128BitsConcat<15, 81> for Reg128Bits<66> {}
impl Reg128BitsDownCast<16> for Reg128Bits<66> {}
impl Reg128BitsConcat<16, 82> for Reg128Bits<66> {}
impl Reg128BitsDownCast<17> for Reg128Bits<66> {}
impl Reg128BitsConcat<17, 83> for Reg128Bits<66> {}
impl Reg128BitsDownCast<18> for Reg128Bits<66> {}
impl Reg128BitsConcat<18, 84> for Reg128Bits<66> {}
impl Reg128BitsDownCast<19> for Reg128Bits<66> {}
impl Reg128BitsConcat<19, 85> for Reg128Bits<66> {}
impl Reg128BitsDownCast<20> for Reg128Bits<66> {}
impl Reg128BitsConcat<20, 86> for Reg128Bits<66> {}
impl Reg128BitsDownCast<21> for Reg128Bits<66> {}
impl Reg128BitsConcat<21, 87> for Reg128Bits<66> {}
impl Reg128BitsDownCast<22> for Reg128Bits<66> {}
impl Reg128BitsConcat<22, 88> for Reg128Bits<66> {}
impl Reg128BitsDownCast<23> for Reg128Bits<66> {}
impl Reg128BitsConcat<23, 89> for Reg128Bits<66> {}
impl Reg128BitsDownCast<24> for Reg128Bits<66> {}
impl Reg128BitsConcat<24, 90> for Reg128Bits<66> {}
impl Reg128BitsDownCast<25> for Reg128Bits<66> {}
impl Reg128BitsConcat<25, 91> for Reg128Bits<66> {}
impl Reg128BitsDownCast<26> for Reg128Bits<66> {}
impl Reg128BitsConcat<26, 92> for Reg128Bits<66> {}
impl Reg128BitsDownCast<27> for Reg128Bits<66> {}
impl Reg128BitsConcat<27, 93> for Reg128Bits<66> {}
impl Reg128BitsDownCast<28> for Reg128Bits<66> {}
impl Reg128BitsConcat<28, 94> for Reg128Bits<66> {}
impl Reg128BitsDownCast<29> for Reg128Bits<66> {}
impl Reg128BitsConcat<29, 95> for Reg128Bits<66> {}
impl Reg128BitsDownCast<30> for Reg128Bits<66> {}
impl Reg128BitsConcat<30, 96> for Reg128Bits<66> {}
impl Reg128BitsDownCast<31> for Reg128Bits<66> {}
impl Reg128BitsConcat<31, 97> for Reg128Bits<66> {}
impl Reg128BitsDownCast<32> for Reg128Bits<66> {}
impl Reg128BitsConcat<32, 98> for Reg128Bits<66> {}
impl Reg128BitsDownCast<33> for Reg128Bits<66> {}
impl Reg128BitsConcat<33, 99> for Reg128Bits<66> {}
impl Reg128BitsDownCast<34> for Reg128Bits<66> {}
impl Reg128BitsConcat<34, 100> for Reg128Bits<66> {}
impl Reg128BitsDownCast<35> for Reg128Bits<66> {}
impl Reg128BitsConcat<35, 101> for Reg128Bits<66> {}
impl Reg128BitsDownCast<36> for Reg128Bits<66> {}
impl Reg128BitsConcat<36, 102> for Reg128Bits<66> {}
impl Reg128BitsDownCast<37> for Reg128Bits<66> {}
impl Reg128BitsConcat<37, 103> for Reg128Bits<66> {}
impl Reg128BitsDownCast<38> for Reg128Bits<66> {}
impl Reg128BitsConcat<38, 104> for Reg128Bits<66> {}
impl Reg128BitsDownCast<39> for Reg128Bits<66> {}
impl Reg128BitsConcat<39, 105> for Reg128Bits<66> {}
impl Reg128BitsDownCast<40> for Reg128Bits<66> {}
impl Reg128BitsConcat<40, 106> for Reg128Bits<66> {}
impl Reg128BitsDownCast<41> for Reg128Bits<66> {}
impl Reg128BitsConcat<41, 107> for Reg128Bits<66> {}
impl Reg128BitsDownCast<42> for Reg128Bits<66> {}
impl Reg128BitsConcat<42, 108> for Reg128Bits<66> {}
impl Reg128BitsDownCast<43> for Reg128Bits<66> {}
impl Reg128BitsConcat<43, 109> for Reg128Bits<66> {}
impl Reg128BitsDownCast<44> for Reg128Bits<66> {}
impl Reg128BitsConcat<44, 110> for Reg128Bits<66> {}
impl Reg128BitsDownCast<45> for Reg128Bits<66> {}
impl Reg128BitsConcat<45, 111> for Reg128Bits<66> {}
impl Reg128BitsDownCast<46> for Reg128Bits<66> {}
impl Reg128BitsConcat<46, 112> for Reg128Bits<66> {}
impl Reg128BitsDownCast<47> for Reg128Bits<66> {}
impl Reg128BitsConcat<47, 113> for Reg128Bits<66> {}
impl Reg128BitsDownCast<48> for Reg128Bits<66> {}
impl Reg128BitsConcat<48, 114> for Reg128Bits<66> {}
impl Reg128BitsDownCast<49> for Reg128Bits<66> {}
impl Reg128BitsConcat<49, 115> for Reg128Bits<66> {}
impl Reg128BitsDownCast<50> for Reg128Bits<66> {}
impl Reg128BitsConcat<50, 116> for Reg128Bits<66> {}
impl Reg128BitsDownCast<51> for Reg128Bits<66> {}
impl Reg128BitsConcat<51, 117> for Reg128Bits<66> {}
impl Reg128BitsDownCast<52> for Reg128Bits<66> {}
impl Reg128BitsConcat<52, 118> for Reg128Bits<66> {}
impl Reg128BitsDownCast<53> for Reg128Bits<66> {}
impl Reg128BitsConcat<53, 119> for Reg128Bits<66> {}
impl Reg128BitsDownCast<54> for Reg128Bits<66> {}
impl Reg128BitsConcat<54, 120> for Reg128Bits<66> {}
impl Reg128BitsDownCast<55> for Reg128Bits<66> {}
impl Reg128BitsConcat<55, 121> for Reg128Bits<66> {}
impl Reg128BitsDownCast<56> for Reg128Bits<66> {}
impl Reg128BitsConcat<56, 122> for Reg128Bits<66> {}
impl Reg128BitsDownCast<57> for Reg128Bits<66> {}
impl Reg128BitsConcat<57, 123> for Reg128Bits<66> {}
impl Reg128BitsDownCast<58> for Reg128Bits<66> {}
impl Reg128BitsConcat<58, 124> for Reg128Bits<66> {}
impl Reg128BitsDownCast<59> for Reg128Bits<66> {}
impl Reg128BitsConcat<59, 125> for Reg128Bits<66> {}
impl Reg128BitsDownCast<60> for Reg128Bits<66> {}
impl Reg128BitsConcat<60, 126> for Reg128Bits<66> {}
impl Reg128BitsDownCast<61> for Reg128Bits<66> {}
impl Reg128BitsConcat<61, 127> for Reg128Bits<66> {}
impl Reg128BitsDownCast<62> for Reg128Bits<66> {}
impl Reg128BitsConcat<62, 128> for Reg128Bits<66> {}
impl Reg128BitsDownCast<63> for Reg128Bits<66> {}
impl Reg128BitsDownCast<64> for Reg128Bits<66> {}
impl Reg128BitsDownCast<65> for Reg128Bits<66> {}
impl Reg128BitsDownCast<66> for Reg128Bits<66> {}
impl Reg128BitsDownCast<1> for Reg128Bits<67> {}
impl Reg128BitsConcat<1, 68> for Reg128Bits<67> {}
impl Reg128BitsDownCast<2> for Reg128Bits<67> {}
impl Reg128BitsConcat<2, 69> for Reg128Bits<67> {}
impl Reg128BitsDownCast<3> for Reg128Bits<67> {}
impl Reg128BitsConcat<3, 70> for Reg128Bits<67> {}
impl Reg128BitsDownCast<4> for Reg128Bits<67> {}
impl Reg128BitsConcat<4, 71> for Reg128Bits<67> {}
impl Reg128BitsDownCast<5> for Reg128Bits<67> {}
impl Reg128BitsConcat<5, 72> for Reg128Bits<67> {}
impl Reg128BitsDownCast<6> for Reg128Bits<67> {}
impl Reg128BitsConcat<6, 73> for Reg128Bits<67> {}
impl Reg128BitsDownCast<7> for Reg128Bits<67> {}
impl Reg128BitsConcat<7, 74> for Reg128Bits<67> {}
impl Reg128BitsDownCast<8> for Reg128Bits<67> {}
impl Reg128BitsConcat<8, 75> for Reg128Bits<67> {}
impl Reg128BitsDownCast<9> for Reg128Bits<67> {}
impl Reg128BitsConcat<9, 76> for Reg128Bits<67> {}
impl Reg128BitsDownCast<10> for Reg128Bits<67> {}
impl Reg128BitsConcat<10, 77> for Reg128Bits<67> {}
impl Reg128BitsDownCast<11> for Reg128Bits<67> {}
impl Reg128BitsConcat<11, 78> for Reg128Bits<67> {}
impl Reg128BitsDownCast<12> for Reg128Bits<67> {}
impl Reg128BitsConcat<12, 79> for Reg128Bits<67> {}
impl Reg128BitsDownCast<13> for Reg128Bits<67> {}
impl Reg128BitsConcat<13, 80> for Reg128Bits<67> {}
impl Reg128BitsDownCast<14> for Reg128Bits<67> {}
impl Reg128BitsConcat<14, 81> for Reg128Bits<67> {}
impl Reg128BitsDownCast<15> for Reg128Bits<67> {}
impl Reg128BitsConcat<15, 82> for Reg128Bits<67> {}
impl Reg128BitsDownCast<16> for Reg128Bits<67> {}
impl Reg128BitsConcat<16, 83> for Reg128Bits<67> {}
impl Reg128BitsDownCast<17> for Reg128Bits<67> {}
impl Reg128BitsConcat<17, 84> for Reg128Bits<67> {}
impl Reg128BitsDownCast<18> for Reg128Bits<67> {}
impl Reg128BitsConcat<18, 85> for Reg128Bits<67> {}
impl Reg128BitsDownCast<19> for Reg128Bits<67> {}
impl Reg128BitsConcat<19, 86> for Reg128Bits<67> {}
impl Reg128BitsDownCast<20> for Reg128Bits<67> {}
impl Reg128BitsConcat<20, 87> for Reg128Bits<67> {}
impl Reg128BitsDownCast<21> for Reg128Bits<67> {}
impl Reg128BitsConcat<21, 88> for Reg128Bits<67> {}
impl Reg128BitsDownCast<22> for Reg128Bits<67> {}
impl Reg128BitsConcat<22, 89> for Reg128Bits<67> {}
impl Reg128BitsDownCast<23> for Reg128Bits<67> {}
impl Reg128BitsConcat<23, 90> for Reg128Bits<67> {}
impl Reg128BitsDownCast<24> for Reg128Bits<67> {}
impl Reg128BitsConcat<24, 91> for Reg128Bits<67> {}
impl Reg128BitsDownCast<25> for Reg128Bits<67> {}
impl Reg128BitsConcat<25, 92> for Reg128Bits<67> {}
impl Reg128BitsDownCast<26> for Reg128Bits<67> {}
impl Reg128BitsConcat<26, 93> for Reg128Bits<67> {}
impl Reg128BitsDownCast<27> for Reg128Bits<67> {}
impl Reg128BitsConcat<27, 94> for Reg128Bits<67> {}
impl Reg128BitsDownCast<28> for Reg128Bits<67> {}
impl Reg128BitsConcat<28, 95> for Reg128Bits<67> {}
impl Reg128BitsDownCast<29> for Reg128Bits<67> {}
impl Reg128BitsConcat<29, 96> for Reg128Bits<67> {}
impl Reg128BitsDownCast<30> for Reg128Bits<67> {}
impl Reg128BitsConcat<30, 97> for Reg128Bits<67> {}
impl Reg128BitsDownCast<31> for Reg128Bits<67> {}
impl Reg128BitsConcat<31, 98> for Reg128Bits<67> {}
impl Reg128BitsDownCast<32> for Reg128Bits<67> {}
impl Reg128BitsConcat<32, 99> for Reg128Bits<67> {}
impl Reg128BitsDownCast<33> for Reg128Bits<67> {}
impl Reg128BitsConcat<33, 100> for Reg128Bits<67> {}
impl Reg128BitsDownCast<34> for Reg128Bits<67> {}
impl Reg128BitsConcat<34, 101> for Reg128Bits<67> {}
impl Reg128BitsDownCast<35> for Reg128Bits<67> {}
impl Reg128BitsConcat<35, 102> for Reg128Bits<67> {}
impl Reg128BitsDownCast<36> for Reg128Bits<67> {}
impl Reg128BitsConcat<36, 103> for Reg128Bits<67> {}
impl Reg128BitsDownCast<37> for Reg128Bits<67> {}
impl Reg128BitsConcat<37, 104> for Reg128Bits<67> {}
impl Reg128BitsDownCast<38> for Reg128Bits<67> {}
impl Reg128BitsConcat<38, 105> for Reg128Bits<67> {}
impl Reg128BitsDownCast<39> for Reg128Bits<67> {}
impl Reg128BitsConcat<39, 106> for Reg128Bits<67> {}
impl Reg128BitsDownCast<40> for Reg128Bits<67> {}
impl Reg128BitsConcat<40, 107> for Reg128Bits<67> {}
impl Reg128BitsDownCast<41> for Reg128Bits<67> {}
impl Reg128BitsConcat<41, 108> for Reg128Bits<67> {}
impl Reg128BitsDownCast<42> for Reg128Bits<67> {}
impl Reg128BitsConcat<42, 109> for Reg128Bits<67> {}
impl Reg128BitsDownCast<43> for Reg128Bits<67> {}
impl Reg128BitsConcat<43, 110> for Reg128Bits<67> {}
impl Reg128BitsDownCast<44> for Reg128Bits<67> {}
impl Reg128BitsConcat<44, 111> for Reg128Bits<67> {}
impl Reg128BitsDownCast<45> for Reg128Bits<67> {}
impl Reg128BitsConcat<45, 112> for Reg128Bits<67> {}
impl Reg128BitsDownCast<46> for Reg128Bits<67> {}
impl Reg128BitsConcat<46, 113> for Reg128Bits<67> {}
impl Reg128BitsDownCast<47> for Reg128Bits<67> {}
impl Reg128BitsConcat<47, 114> for Reg128Bits<67> {}
impl Reg128BitsDownCast<48> for Reg128Bits<67> {}
impl Reg128BitsConcat<48, 115> for Reg128Bits<67> {}
impl Reg128BitsDownCast<49> for Reg128Bits<67> {}
impl Reg128BitsConcat<49, 116> for Reg128Bits<67> {}
impl Reg128BitsDownCast<50> for Reg128Bits<67> {}
impl Reg128BitsConcat<50, 117> for Reg128Bits<67> {}
impl Reg128BitsDownCast<51> for Reg128Bits<67> {}
impl Reg128BitsConcat<51, 118> for Reg128Bits<67> {}
impl Reg128BitsDownCast<52> for Reg128Bits<67> {}
impl Reg128BitsConcat<52, 119> for Reg128Bits<67> {}
impl Reg128BitsDownCast<53> for Reg128Bits<67> {}
impl Reg128BitsConcat<53, 120> for Reg128Bits<67> {}
impl Reg128BitsDownCast<54> for Reg128Bits<67> {}
impl Reg128BitsConcat<54, 121> for Reg128Bits<67> {}
impl Reg128BitsDownCast<55> for Reg128Bits<67> {}
impl Reg128BitsConcat<55, 122> for Reg128Bits<67> {}
impl Reg128BitsDownCast<56> for Reg128Bits<67> {}
impl Reg128BitsConcat<56, 123> for Reg128Bits<67> {}
impl Reg128BitsDownCast<57> for Reg128Bits<67> {}
impl Reg128BitsConcat<57, 124> for Reg128Bits<67> {}
impl Reg128BitsDownCast<58> for Reg128Bits<67> {}
impl Reg128BitsConcat<58, 125> for Reg128Bits<67> {}
impl Reg128BitsDownCast<59> for Reg128Bits<67> {}
impl Reg128BitsConcat<59, 126> for Reg128Bits<67> {}
impl Reg128BitsDownCast<60> for Reg128Bits<67> {}
impl Reg128BitsConcat<60, 127> for Reg128Bits<67> {}
impl Reg128BitsDownCast<61> for Reg128Bits<67> {}
impl Reg128BitsConcat<61, 128> for Reg128Bits<67> {}
impl Reg128BitsDownCast<62> for Reg128Bits<67> {}
impl Reg128BitsDownCast<63> for Reg128Bits<67> {}
impl Reg128BitsDownCast<64> for Reg128Bits<67> {}
impl Reg128BitsDownCast<65> for Reg128Bits<67> {}
impl Reg128BitsDownCast<66> for Reg128Bits<67> {}
impl Reg128BitsDownCast<67> for Reg128Bits<67> {}
impl Reg128BitsDownCast<1> for Reg128Bits<68> {}
impl Reg128BitsConcat<1, 69> for Reg128Bits<68> {}
impl Reg128BitsDownCast<2> for Reg128Bits<68> {}
impl Reg128BitsConcat<2, 70> for Reg128Bits<68> {}
impl Reg128BitsDownCast<3> for Reg128Bits<68> {}
impl Reg128BitsConcat<3, 71> for Reg128Bits<68> {}
impl Reg128BitsDownCast<4> for Reg128Bits<68> {}
impl Reg128BitsConcat<4, 72> for Reg128Bits<68> {}
impl Reg128BitsDownCast<5> for Reg128Bits<68> {}
impl Reg128BitsConcat<5, 73> for Reg128Bits<68> {}
impl Reg128BitsDownCast<6> for Reg128Bits<68> {}
impl Reg128BitsConcat<6, 74> for Reg128Bits<68> {}
impl Reg128BitsDownCast<7> for Reg128Bits<68> {}
impl Reg128BitsConcat<7, 75> for Reg128Bits<68> {}
impl Reg128BitsDownCast<8> for Reg128Bits<68> {}
impl Reg128BitsConcat<8, 76> for Reg128Bits<68> {}
impl Reg128BitsDownCast<9> for Reg128Bits<68> {}
impl Reg128BitsConcat<9, 77> for Reg128Bits<68> {}
impl Reg128BitsDownCast<10> for Reg128Bits<68> {}
impl Reg128BitsConcat<10, 78> for Reg128Bits<68> {}
impl Reg128BitsDownCast<11> for Reg128Bits<68> {}
impl Reg128BitsConcat<11, 79> for Reg128Bits<68> {}
impl Reg128BitsDownCast<12> for Reg128Bits<68> {}
impl Reg128BitsConcat<12, 80> for Reg128Bits<68> {}
impl Reg128BitsDownCast<13> for Reg128Bits<68> {}
impl Reg128BitsConcat<13, 81> for Reg128Bits<68> {}
impl Reg128BitsDownCast<14> for Reg128Bits<68> {}
impl Reg128BitsConcat<14, 82> for Reg128Bits<68> {}
impl Reg128BitsDownCast<15> for Reg128Bits<68> {}
impl Reg128BitsConcat<15, 83> for Reg128Bits<68> {}
impl Reg128BitsDownCast<16> for Reg128Bits<68> {}
impl Reg128BitsConcat<16, 84> for Reg128Bits<68> {}
impl Reg128BitsDownCast<17> for Reg128Bits<68> {}
impl Reg128BitsConcat<17, 85> for Reg128Bits<68> {}
impl Reg128BitsDownCast<18> for Reg128Bits<68> {}
impl Reg128BitsConcat<18, 86> for Reg128Bits<68> {}
impl Reg128BitsDownCast<19> for Reg128Bits<68> {}
impl Reg128BitsConcat<19, 87> for Reg128Bits<68> {}
impl Reg128BitsDownCast<20> for Reg128Bits<68> {}
impl Reg128BitsConcat<20, 88> for Reg128Bits<68> {}
impl Reg128BitsDownCast<21> for Reg128Bits<68> {}
impl Reg128BitsConcat<21, 89> for Reg128Bits<68> {}
impl Reg128BitsDownCast<22> for Reg128Bits<68> {}
impl Reg128BitsConcat<22, 90> for Reg128Bits<68> {}
impl Reg128BitsDownCast<23> for Reg128Bits<68> {}
impl Reg128BitsConcat<23, 91> for Reg128Bits<68> {}
impl Reg128BitsDownCast<24> for Reg128Bits<68> {}
impl Reg128BitsConcat<24, 92> for Reg128Bits<68> {}
impl Reg128BitsDownCast<25> for Reg128Bits<68> {}
impl Reg128BitsConcat<25, 93> for Reg128Bits<68> {}
impl Reg128BitsDownCast<26> for Reg128Bits<68> {}
impl Reg128BitsConcat<26, 94> for Reg128Bits<68> {}
impl Reg128BitsDownCast<27> for Reg128Bits<68> {}
impl Reg128BitsConcat<27, 95> for Reg128Bits<68> {}
impl Reg128BitsDownCast<28> for Reg128Bits<68> {}
impl Reg128BitsConcat<28, 96> for Reg128Bits<68> {}
impl Reg128BitsDownCast<29> for Reg128Bits<68> {}
impl Reg128BitsConcat<29, 97> for Reg128Bits<68> {}
impl Reg128BitsDownCast<30> for Reg128Bits<68> {}
impl Reg128BitsConcat<30, 98> for Reg128Bits<68> {}
impl Reg128BitsDownCast<31> for Reg128Bits<68> {}
impl Reg128BitsConcat<31, 99> for Reg128Bits<68> {}
impl Reg128BitsDownCast<32> for Reg128Bits<68> {}
impl Reg128BitsConcat<32, 100> for Reg128Bits<68> {}
impl Reg128BitsDownCast<33> for Reg128Bits<68> {}
impl Reg128BitsConcat<33, 101> for Reg128Bits<68> {}
impl Reg128BitsDownCast<34> for Reg128Bits<68> {}
impl Reg128BitsConcat<34, 102> for Reg128Bits<68> {}
impl Reg128BitsDownCast<35> for Reg128Bits<68> {}
impl Reg128BitsConcat<35, 103> for Reg128Bits<68> {}
impl Reg128BitsDownCast<36> for Reg128Bits<68> {}
impl Reg128BitsConcat<36, 104> for Reg128Bits<68> {}
impl Reg128BitsDownCast<37> for Reg128Bits<68> {}
impl Reg128BitsConcat<37, 105> for Reg128Bits<68> {}
impl Reg128BitsDownCast<38> for Reg128Bits<68> {}
impl Reg128BitsConcat<38, 106> for Reg128Bits<68> {}
impl Reg128BitsDownCast<39> for Reg128Bits<68> {}
impl Reg128BitsConcat<39, 107> for Reg128Bits<68> {}
impl Reg128BitsDownCast<40> for Reg128Bits<68> {}
impl Reg128BitsConcat<40, 108> for Reg128Bits<68> {}
impl Reg128BitsDownCast<41> for Reg128Bits<68> {}
impl Reg128BitsConcat<41, 109> for Reg128Bits<68> {}
impl Reg128BitsDownCast<42> for Reg128Bits<68> {}
impl Reg128BitsConcat<42, 110> for Reg128Bits<68> {}
impl Reg128BitsDownCast<43> for Reg128Bits<68> {}
impl Reg128BitsConcat<43, 111> for Reg128Bits<68> {}
impl Reg128BitsDownCast<44> for Reg128Bits<68> {}
impl Reg128BitsConcat<44, 112> for Reg128Bits<68> {}
impl Reg128BitsDownCast<45> for Reg128Bits<68> {}
impl Reg128BitsConcat<45, 113> for Reg128Bits<68> {}
impl Reg128BitsDownCast<46> for Reg128Bits<68> {}
impl Reg128BitsConcat<46, 114> for Reg128Bits<68> {}
impl Reg128BitsDownCast<47> for Reg128Bits<68> {}
impl Reg128BitsConcat<47, 115> for Reg128Bits<68> {}
impl Reg128BitsDownCast<48> for Reg128Bits<68> {}
impl Reg128BitsConcat<48, 116> for Reg128Bits<68> {}
impl Reg128BitsDownCast<49> for Reg128Bits<68> {}
impl Reg128BitsConcat<49, 117> for Reg128Bits<68> {}
impl Reg128BitsDownCast<50> for Reg128Bits<68> {}
impl Reg128BitsConcat<50, 118> for Reg128Bits<68> {}
impl Reg128BitsDownCast<51> for Reg128Bits<68> {}
impl Reg128BitsConcat<51, 119> for Reg128Bits<68> {}
impl Reg128BitsDownCast<52> for Reg128Bits<68> {}
impl Reg128BitsConcat<52, 120> for Reg128Bits<68> {}
impl Reg128BitsDownCast<53> for Reg128Bits<68> {}
impl Reg128BitsConcat<53, 121> for Reg128Bits<68> {}
impl Reg128BitsDownCast<54> for Reg128Bits<68> {}
impl Reg128BitsConcat<54, 122> for Reg128Bits<68> {}
impl Reg128BitsDownCast<55> for Reg128Bits<68> {}
impl Reg128BitsConcat<55, 123> for Reg128Bits<68> {}
impl Reg128BitsDownCast<56> for Reg128Bits<68> {}
impl Reg128BitsConcat<56, 124> for Reg128Bits<68> {}
impl Reg128BitsDownCast<57> for Reg128Bits<68> {}
impl Reg128BitsConcat<57, 125> for Reg128Bits<68> {}
impl Reg128BitsDownCast<58> for Reg128Bits<68> {}
impl Reg128BitsConcat<58, 126> for Reg128Bits<68> {}
impl Reg128BitsDownCast<59> for Reg128Bits<68> {}
impl Reg128BitsConcat<59, 127> for Reg128Bits<68> {}
impl Reg128BitsDownCast<60> for Reg128Bits<68> {}
impl Reg128BitsConcat<60, 128> for Reg128Bits<68> {}
impl Reg128BitsDownCast<61> for Reg128Bits<68> {}
impl Reg128BitsDownCast<62> for Reg128Bits<68> {}
impl Reg128BitsDownCast<63> for Reg128Bits<68> {}
impl Reg128BitsDownCast<64> for Reg128Bits<68> {}
impl Reg128BitsDownCast<65> for Reg128Bits<68> {}
impl Reg128BitsDownCast<66> for Reg128Bits<68> {}
impl Reg128BitsDownCast<67> for Reg128Bits<68> {}
impl Reg128BitsDownCast<68> for Reg128Bits<68> {}
impl Reg128BitsDownCast<1> for Reg128Bits<69> {}
impl Reg128BitsConcat<1, 70> for Reg128Bits<69> {}
impl Reg128BitsDownCast<2> for Reg128Bits<69> {}
impl Reg128BitsConcat<2, 71> for Reg128Bits<69> {}
impl Reg128BitsDownCast<3> for Reg128Bits<69> {}
impl Reg128BitsConcat<3, 72> for Reg128Bits<69> {}
impl Reg128BitsDownCast<4> for Reg128Bits<69> {}
impl Reg128BitsConcat<4, 73> for Reg128Bits<69> {}
impl Reg128BitsDownCast<5> for Reg128Bits<69> {}
impl Reg128BitsConcat<5, 74> for Reg128Bits<69> {}
impl Reg128BitsDownCast<6> for Reg128Bits<69> {}
impl Reg128BitsConcat<6, 75> for Reg128Bits<69> {}
impl Reg128BitsDownCast<7> for Reg128Bits<69> {}
impl Reg128BitsConcat<7, 76> for Reg128Bits<69> {}
impl Reg128BitsDownCast<8> for Reg128Bits<69> {}
impl Reg128BitsConcat<8, 77> for Reg128Bits<69> {}
impl Reg128BitsDownCast<9> for Reg128Bits<69> {}
impl Reg128BitsConcat<9, 78> for Reg128Bits<69> {}
impl Reg128BitsDownCast<10> for Reg128Bits<69> {}
impl Reg128BitsConcat<10, 79> for Reg128Bits<69> {}
impl Reg128BitsDownCast<11> for Reg128Bits<69> {}
impl Reg128BitsConcat<11, 80> for Reg128Bits<69> {}
impl Reg128BitsDownCast<12> for Reg128Bits<69> {}
impl Reg128BitsConcat<12, 81> for Reg128Bits<69> {}
impl Reg128BitsDownCast<13> for Reg128Bits<69> {}
impl Reg128BitsConcat<13, 82> for Reg128Bits<69> {}
impl Reg128BitsDownCast<14> for Reg128Bits<69> {}
impl Reg128BitsConcat<14, 83> for Reg128Bits<69> {}
impl Reg128BitsDownCast<15> for Reg128Bits<69> {}
impl Reg128BitsConcat<15, 84> for Reg128Bits<69> {}
impl Reg128BitsDownCast<16> for Reg128Bits<69> {}
impl Reg128BitsConcat<16, 85> for Reg128Bits<69> {}
impl Reg128BitsDownCast<17> for Reg128Bits<69> {}
impl Reg128BitsConcat<17, 86> for Reg128Bits<69> {}
impl Reg128BitsDownCast<18> for Reg128Bits<69> {}
impl Reg128BitsConcat<18, 87> for Reg128Bits<69> {}
impl Reg128BitsDownCast<19> for Reg128Bits<69> {}
impl Reg128BitsConcat<19, 88> for Reg128Bits<69> {}
impl Reg128BitsDownCast<20> for Reg128Bits<69> {}
impl Reg128BitsConcat<20, 89> for Reg128Bits<69> {}
impl Reg128BitsDownCast<21> for Reg128Bits<69> {}
impl Reg128BitsConcat<21, 90> for Reg128Bits<69> {}
impl Reg128BitsDownCast<22> for Reg128Bits<69> {}
impl Reg128BitsConcat<22, 91> for Reg128Bits<69> {}
impl Reg128BitsDownCast<23> for Reg128Bits<69> {}
impl Reg128BitsConcat<23, 92> for Reg128Bits<69> {}
impl Reg128BitsDownCast<24> for Reg128Bits<69> {}
impl Reg128BitsConcat<24, 93> for Reg128Bits<69> {}
impl Reg128BitsDownCast<25> for Reg128Bits<69> {}
impl Reg128BitsConcat<25, 94> for Reg128Bits<69> {}
impl Reg128BitsDownCast<26> for Reg128Bits<69> {}
impl Reg128BitsConcat<26, 95> for Reg128Bits<69> {}
impl Reg128BitsDownCast<27> for Reg128Bits<69> {}
impl Reg128BitsConcat<27, 96> for Reg128Bits<69> {}
impl Reg128BitsDownCast<28> for Reg128Bits<69> {}
impl Reg128BitsConcat<28, 97> for Reg128Bits<69> {}
impl Reg128BitsDownCast<29> for Reg128Bits<69> {}
impl Reg128BitsConcat<29, 98> for Reg128Bits<69> {}
impl Reg128BitsDownCast<30> for Reg128Bits<69> {}
impl Reg128BitsConcat<30, 99> for Reg128Bits<69> {}
impl Reg128BitsDownCast<31> for Reg128Bits<69> {}
impl Reg128BitsConcat<31, 100> for Reg128Bits<69> {}
impl Reg128BitsDownCast<32> for Reg128Bits<69> {}
impl Reg128BitsConcat<32, 101> for Reg128Bits<69> {}
impl Reg128BitsDownCast<33> for Reg128Bits<69> {}
impl Reg128BitsConcat<33, 102> for Reg128Bits<69> {}
impl Reg128BitsDownCast<34> for Reg128Bits<69> {}
impl Reg128BitsConcat<34, 103> for Reg128Bits<69> {}
impl Reg128BitsDownCast<35> for Reg128Bits<69> {}
impl Reg128BitsConcat<35, 104> for Reg128Bits<69> {}
impl Reg128BitsDownCast<36> for Reg128Bits<69> {}
impl Reg128BitsConcat<36, 105> for Reg128Bits<69> {}
impl Reg128BitsDownCast<37> for Reg128Bits<69> {}
impl Reg128BitsConcat<37, 106> for Reg128Bits<69> {}
impl Reg128BitsDownCast<38> for Reg128Bits<69> {}
impl Reg128BitsConcat<38, 107> for Reg128Bits<69> {}
impl Reg128BitsDownCast<39> for Reg128Bits<69> {}
impl Reg128BitsConcat<39, 108> for Reg128Bits<69> {}
impl Reg128BitsDownCast<40> for Reg128Bits<69> {}
impl Reg128BitsConcat<40, 109> for Reg128Bits<69> {}
impl Reg128BitsDownCast<41> for Reg128Bits<69> {}
impl Reg128BitsConcat<41, 110> for Reg128Bits<69> {}
impl Reg128BitsDownCast<42> for Reg128Bits<69> {}
impl Reg128BitsConcat<42, 111> for Reg128Bits<69> {}
impl Reg128BitsDownCast<43> for Reg128Bits<69> {}
impl Reg128BitsConcat<43, 112> for Reg128Bits<69> {}
impl Reg128BitsDownCast<44> for Reg128Bits<69> {}
impl Reg128BitsConcat<44, 113> for Reg128Bits<69> {}
impl Reg128BitsDownCast<45> for Reg128Bits<69> {}
impl Reg128BitsConcat<45, 114> for Reg128Bits<69> {}
impl Reg128BitsDownCast<46> for Reg128Bits<69> {}
impl Reg128BitsConcat<46, 115> for Reg128Bits<69> {}
impl Reg128BitsDownCast<47> for Reg128Bits<69> {}
impl Reg128BitsConcat<47, 116> for Reg128Bits<69> {}
impl Reg128BitsDownCast<48> for Reg128Bits<69> {}
impl Reg128BitsConcat<48, 117> for Reg128Bits<69> {}
impl Reg128BitsDownCast<49> for Reg128Bits<69> {}
impl Reg128BitsConcat<49, 118> for Reg128Bits<69> {}
impl Reg128BitsDownCast<50> for Reg128Bits<69> {}
impl Reg128BitsConcat<50, 119> for Reg128Bits<69> {}
impl Reg128BitsDownCast<51> for Reg128Bits<69> {}
impl Reg128BitsConcat<51, 120> for Reg128Bits<69> {}
impl Reg128BitsDownCast<52> for Reg128Bits<69> {}
impl Reg128BitsConcat<52, 121> for Reg128Bits<69> {}
impl Reg128BitsDownCast<53> for Reg128Bits<69> {}
impl Reg128BitsConcat<53, 122> for Reg128Bits<69> {}
impl Reg128BitsDownCast<54> for Reg128Bits<69> {}
impl Reg128BitsConcat<54, 123> for Reg128Bits<69> {}
impl Reg128BitsDownCast<55> for Reg128Bits<69> {}
impl Reg128BitsConcat<55, 124> for Reg128Bits<69> {}
impl Reg128BitsDownCast<56> for Reg128Bits<69> {}
impl Reg128BitsConcat<56, 125> for Reg128Bits<69> {}
impl Reg128BitsDownCast<57> for Reg128Bits<69> {}
impl Reg128BitsConcat<57, 126> for Reg128Bits<69> {}
impl Reg128BitsDownCast<58> for Reg128Bits<69> {}
impl Reg128BitsConcat<58, 127> for Reg128Bits<69> {}
impl Reg128BitsDownCast<59> for Reg128Bits<69> {}
impl Reg128BitsConcat<59, 128> for Reg128Bits<69> {}
impl Reg128BitsDownCast<60> for Reg128Bits<69> {}
impl Reg128BitsDownCast<61> for Reg128Bits<69> {}
impl Reg128BitsDownCast<62> for Reg128Bits<69> {}
impl Reg128BitsDownCast<63> for Reg128Bits<69> {}
impl Reg128BitsDownCast<64> for Reg128Bits<69> {}
impl Reg128BitsDownCast<65> for Reg128Bits<69> {}
impl Reg128BitsDownCast<66> for Reg128Bits<69> {}
impl Reg128BitsDownCast<67> for Reg128Bits<69> {}
impl Reg128BitsDownCast<68> for Reg128Bits<69> {}
impl Reg128BitsDownCast<69> for Reg128Bits<69> {}
impl Reg128BitsDownCast<1> for Reg128Bits<70> {}
impl Reg128BitsConcat<1, 71> for Reg128Bits<70> {}
impl Reg128BitsDownCast<2> for Reg128Bits<70> {}
impl Reg128BitsConcat<2, 72> for Reg128Bits<70> {}
impl Reg128BitsDownCast<3> for Reg128Bits<70> {}
impl Reg128BitsConcat<3, 73> for Reg128Bits<70> {}
impl Reg128BitsDownCast<4> for Reg128Bits<70> {}
impl Reg128BitsConcat<4, 74> for Reg128Bits<70> {}
impl Reg128BitsDownCast<5> for Reg128Bits<70> {}
impl Reg128BitsConcat<5, 75> for Reg128Bits<70> {}
impl Reg128BitsDownCast<6> for Reg128Bits<70> {}
impl Reg128BitsConcat<6, 76> for Reg128Bits<70> {}
impl Reg128BitsDownCast<7> for Reg128Bits<70> {}
impl Reg128BitsConcat<7, 77> for Reg128Bits<70> {}
impl Reg128BitsDownCast<8> for Reg128Bits<70> {}
impl Reg128BitsConcat<8, 78> for Reg128Bits<70> {}
impl Reg128BitsDownCast<9> for Reg128Bits<70> {}
impl Reg128BitsConcat<9, 79> for Reg128Bits<70> {}
impl Reg128BitsDownCast<10> for Reg128Bits<70> {}
impl Reg128BitsConcat<10, 80> for Reg128Bits<70> {}
impl Reg128BitsDownCast<11> for Reg128Bits<70> {}
impl Reg128BitsConcat<11, 81> for Reg128Bits<70> {}
impl Reg128BitsDownCast<12> for Reg128Bits<70> {}
impl Reg128BitsConcat<12, 82> for Reg128Bits<70> {}
impl Reg128BitsDownCast<13> for Reg128Bits<70> {}
impl Reg128BitsConcat<13, 83> for Reg128Bits<70> {}
impl Reg128BitsDownCast<14> for Reg128Bits<70> {}
impl Reg128BitsConcat<14, 84> for Reg128Bits<70> {}
impl Reg128BitsDownCast<15> for Reg128Bits<70> {}
impl Reg128BitsConcat<15, 85> for Reg128Bits<70> {}
impl Reg128BitsDownCast<16> for Reg128Bits<70> {}
impl Reg128BitsConcat<16, 86> for Reg128Bits<70> {}
impl Reg128BitsDownCast<17> for Reg128Bits<70> {}
impl Reg128BitsConcat<17, 87> for Reg128Bits<70> {}
impl Reg128BitsDownCast<18> for Reg128Bits<70> {}
impl Reg128BitsConcat<18, 88> for Reg128Bits<70> {}
impl Reg128BitsDownCast<19> for Reg128Bits<70> {}
impl Reg128BitsConcat<19, 89> for Reg128Bits<70> {}
impl Reg128BitsDownCast<20> for Reg128Bits<70> {}
impl Reg128BitsConcat<20, 90> for Reg128Bits<70> {}
impl Reg128BitsDownCast<21> for Reg128Bits<70> {}
impl Reg128BitsConcat<21, 91> for Reg128Bits<70> {}
impl Reg128BitsDownCast<22> for Reg128Bits<70> {}
impl Reg128BitsConcat<22, 92> for Reg128Bits<70> {}
impl Reg128BitsDownCast<23> for Reg128Bits<70> {}
impl Reg128BitsConcat<23, 93> for Reg128Bits<70> {}
impl Reg128BitsDownCast<24> for Reg128Bits<70> {}
impl Reg128BitsConcat<24, 94> for Reg128Bits<70> {}
impl Reg128BitsDownCast<25> for Reg128Bits<70> {}
impl Reg128BitsConcat<25, 95> for Reg128Bits<70> {}
impl Reg128BitsDownCast<26> for Reg128Bits<70> {}
impl Reg128BitsConcat<26, 96> for Reg128Bits<70> {}
impl Reg128BitsDownCast<27> for Reg128Bits<70> {}
impl Reg128BitsConcat<27, 97> for Reg128Bits<70> {}
impl Reg128BitsDownCast<28> for Reg128Bits<70> {}
impl Reg128BitsConcat<28, 98> for Reg128Bits<70> {}
impl Reg128BitsDownCast<29> for Reg128Bits<70> {}
impl Reg128BitsConcat<29, 99> for Reg128Bits<70> {}
impl Reg128BitsDownCast<30> for Reg128Bits<70> {}
impl Reg128BitsConcat<30, 100> for Reg128Bits<70> {}
impl Reg128BitsDownCast<31> for Reg128Bits<70> {}
impl Reg128BitsConcat<31, 101> for Reg128Bits<70> {}
impl Reg128BitsDownCast<32> for Reg128Bits<70> {}
impl Reg128BitsConcat<32, 102> for Reg128Bits<70> {}
impl Reg128BitsDownCast<33> for Reg128Bits<70> {}
impl Reg128BitsConcat<33, 103> for Reg128Bits<70> {}
impl Reg128BitsDownCast<34> for Reg128Bits<70> {}
impl Reg128BitsConcat<34, 104> for Reg128Bits<70> {}
impl Reg128BitsDownCast<35> for Reg128Bits<70> {}
impl Reg128BitsConcat<35, 105> for Reg128Bits<70> {}
impl Reg128BitsDownCast<36> for Reg128Bits<70> {}
impl Reg128BitsConcat<36, 106> for Reg128Bits<70> {}
impl Reg128BitsDownCast<37> for Reg128Bits<70> {}
impl Reg128BitsConcat<37, 107> for Reg128Bits<70> {}
impl Reg128BitsDownCast<38> for Reg128Bits<70> {}
impl Reg128BitsConcat<38, 108> for Reg128Bits<70> {}
impl Reg128BitsDownCast<39> for Reg128Bits<70> {}
impl Reg128BitsConcat<39, 109> for Reg128Bits<70> {}
impl Reg128BitsDownCast<40> for Reg128Bits<70> {}
impl Reg128BitsConcat<40, 110> for Reg128Bits<70> {}
impl Reg128BitsDownCast<41> for Reg128Bits<70> {}
impl Reg128BitsConcat<41, 111> for Reg128Bits<70> {}
impl Reg128BitsDownCast<42> for Reg128Bits<70> {}
impl Reg128BitsConcat<42, 112> for Reg128Bits<70> {}
impl Reg128BitsDownCast<43> for Reg128Bits<70> {}
impl Reg128BitsConcat<43, 113> for Reg128Bits<70> {}
impl Reg128BitsDownCast<44> for Reg128Bits<70> {}
impl Reg128BitsConcat<44, 114> for Reg128Bits<70> {}
impl Reg128BitsDownCast<45> for Reg128Bits<70> {}
impl Reg128BitsConcat<45, 115> for Reg128Bits<70> {}
impl Reg128BitsDownCast<46> for Reg128Bits<70> {}
impl Reg128BitsConcat<46, 116> for Reg128Bits<70> {}
impl Reg128BitsDownCast<47> for Reg128Bits<70> {}
impl Reg128BitsConcat<47, 117> for Reg128Bits<70> {}
impl Reg128BitsDownCast<48> for Reg128Bits<70> {}
impl Reg128BitsConcat<48, 118> for Reg128Bits<70> {}
impl Reg128BitsDownCast<49> for Reg128Bits<70> {}
impl Reg128BitsConcat<49, 119> for Reg128Bits<70> {}
impl Reg128BitsDownCast<50> for Reg128Bits<70> {}
impl Reg128BitsConcat<50, 120> for Reg128Bits<70> {}
impl Reg128BitsDownCast<51> for Reg128Bits<70> {}
impl Reg128BitsConcat<51, 121> for Reg128Bits<70> {}
impl Reg128BitsDownCast<52> for Reg128Bits<70> {}
impl Reg128BitsConcat<52, 122> for Reg128Bits<70> {}
impl Reg128BitsDownCast<53> for Reg128Bits<70> {}
impl Reg128BitsConcat<53, 123> for Reg128Bits<70> {}
impl Reg128BitsDownCast<54> for Reg128Bits<70> {}
impl Reg128BitsConcat<54, 124> for Reg128Bits<70> {}
impl Reg128BitsDownCast<55> for Reg128Bits<70> {}
impl Reg128BitsConcat<55, 125> for Reg128Bits<70> {}
impl Reg128BitsDownCast<56> for Reg128Bits<70> {}
impl Reg128BitsConcat<56, 126> for Reg128Bits<70> {}
impl Reg128BitsDownCast<57> for Reg128Bits<70> {}
impl Reg128BitsConcat<57, 127> for Reg128Bits<70> {}
impl Reg128BitsDownCast<58> for Reg128Bits<70> {}
impl Reg128BitsConcat<58, 128> for Reg128Bits<70> {}
impl Reg128BitsDownCast<59> for Reg128Bits<70> {}
impl Reg128BitsDownCast<60> for Reg128Bits<70> {}
impl Reg128BitsDownCast<61> for Reg128Bits<70> {}
impl Reg128BitsDownCast<62> for Reg128Bits<70> {}
impl Reg128BitsDownCast<63> for Reg128Bits<70> {}
impl Reg128BitsDownCast<64> for Reg128Bits<70> {}
impl Reg128BitsDownCast<65> for Reg128Bits<70> {}
impl Reg128BitsDownCast<66> for Reg128Bits<70> {}
impl Reg128BitsDownCast<67> for Reg128Bits<70> {}
impl Reg128BitsDownCast<68> for Reg128Bits<70> {}
impl Reg128BitsDownCast<69> for Reg128Bits<70> {}
impl Reg128BitsDownCast<70> for Reg128Bits<70> {}
impl Reg128BitsDownCast<1> for Reg128Bits<71> {}
impl Reg128BitsConcat<1, 72> for Reg128Bits<71> {}
impl Reg128BitsDownCast<2> for Reg128Bits<71> {}
impl Reg128BitsConcat<2, 73> for Reg128Bits<71> {}
impl Reg128BitsDownCast<3> for Reg128Bits<71> {}
impl Reg128BitsConcat<3, 74> for Reg128Bits<71> {}
impl Reg128BitsDownCast<4> for Reg128Bits<71> {}
impl Reg128BitsConcat<4, 75> for Reg128Bits<71> {}
impl Reg128BitsDownCast<5> for Reg128Bits<71> {}
impl Reg128BitsConcat<5, 76> for Reg128Bits<71> {}
impl Reg128BitsDownCast<6> for Reg128Bits<71> {}
impl Reg128BitsConcat<6, 77> for Reg128Bits<71> {}
impl Reg128BitsDownCast<7> for Reg128Bits<71> {}
impl Reg128BitsConcat<7, 78> for Reg128Bits<71> {}
impl Reg128BitsDownCast<8> for Reg128Bits<71> {}
impl Reg128BitsConcat<8, 79> for Reg128Bits<71> {}
impl Reg128BitsDownCast<9> for Reg128Bits<71> {}
impl Reg128BitsConcat<9, 80> for Reg128Bits<71> {}
impl Reg128BitsDownCast<10> for Reg128Bits<71> {}
impl Reg128BitsConcat<10, 81> for Reg128Bits<71> {}
impl Reg128BitsDownCast<11> for Reg128Bits<71> {}
impl Reg128BitsConcat<11, 82> for Reg128Bits<71> {}
impl Reg128BitsDownCast<12> for Reg128Bits<71> {}
impl Reg128BitsConcat<12, 83> for Reg128Bits<71> {}
impl Reg128BitsDownCast<13> for Reg128Bits<71> {}
impl Reg128BitsConcat<13, 84> for Reg128Bits<71> {}
impl Reg128BitsDownCast<14> for Reg128Bits<71> {}
impl Reg128BitsConcat<14, 85> for Reg128Bits<71> {}
impl Reg128BitsDownCast<15> for Reg128Bits<71> {}
impl Reg128BitsConcat<15, 86> for Reg128Bits<71> {}
impl Reg128BitsDownCast<16> for Reg128Bits<71> {}
impl Reg128BitsConcat<16, 87> for Reg128Bits<71> {}
impl Reg128BitsDownCast<17> for Reg128Bits<71> {}
impl Reg128BitsConcat<17, 88> for Reg128Bits<71> {}
impl Reg128BitsDownCast<18> for Reg128Bits<71> {}
impl Reg128BitsConcat<18, 89> for Reg128Bits<71> {}
impl Reg128BitsDownCast<19> for Reg128Bits<71> {}
impl Reg128BitsConcat<19, 90> for Reg128Bits<71> {}
impl Reg128BitsDownCast<20> for Reg128Bits<71> {}
impl Reg128BitsConcat<20, 91> for Reg128Bits<71> {}
impl Reg128BitsDownCast<21> for Reg128Bits<71> {}
impl Reg128BitsConcat<21, 92> for Reg128Bits<71> {}
impl Reg128BitsDownCast<22> for Reg128Bits<71> {}
impl Reg128BitsConcat<22, 93> for Reg128Bits<71> {}
impl Reg128BitsDownCast<23> for Reg128Bits<71> {}
impl Reg128BitsConcat<23, 94> for Reg128Bits<71> {}
impl Reg128BitsDownCast<24> for Reg128Bits<71> {}
impl Reg128BitsConcat<24, 95> for Reg128Bits<71> {}
impl Reg128BitsDownCast<25> for Reg128Bits<71> {}
impl Reg128BitsConcat<25, 96> for Reg128Bits<71> {}
impl Reg128BitsDownCast<26> for Reg128Bits<71> {}
impl Reg128BitsConcat<26, 97> for Reg128Bits<71> {}
impl Reg128BitsDownCast<27> for Reg128Bits<71> {}
impl Reg128BitsConcat<27, 98> for Reg128Bits<71> {}
impl Reg128BitsDownCast<28> for Reg128Bits<71> {}
impl Reg128BitsConcat<28, 99> for Reg128Bits<71> {}
impl Reg128BitsDownCast<29> for Reg128Bits<71> {}
impl Reg128BitsConcat<29, 100> for Reg128Bits<71> {}
impl Reg128BitsDownCast<30> for Reg128Bits<71> {}
impl Reg128BitsConcat<30, 101> for Reg128Bits<71> {}
impl Reg128BitsDownCast<31> for Reg128Bits<71> {}
impl Reg128BitsConcat<31, 102> for Reg128Bits<71> {}
impl Reg128BitsDownCast<32> for Reg128Bits<71> {}
impl Reg128BitsConcat<32, 103> for Reg128Bits<71> {}
impl Reg128BitsDownCast<33> for Reg128Bits<71> {}
impl Reg128BitsConcat<33, 104> for Reg128Bits<71> {}
impl Reg128BitsDownCast<34> for Reg128Bits<71> {}
impl Reg128BitsConcat<34, 105> for Reg128Bits<71> {}
impl Reg128BitsDownCast<35> for Reg128Bits<71> {}
impl Reg128BitsConcat<35, 106> for Reg128Bits<71> {}
impl Reg128BitsDownCast<36> for Reg128Bits<71> {}
impl Reg128BitsConcat<36, 107> for Reg128Bits<71> {}
impl Reg128BitsDownCast<37> for Reg128Bits<71> {}
impl Reg128BitsConcat<37, 108> for Reg128Bits<71> {}
impl Reg128BitsDownCast<38> for Reg128Bits<71> {}
impl Reg128BitsConcat<38, 109> for Reg128Bits<71> {}
impl Reg128BitsDownCast<39> for Reg128Bits<71> {}
impl Reg128BitsConcat<39, 110> for Reg128Bits<71> {}
impl Reg128BitsDownCast<40> for Reg128Bits<71> {}
impl Reg128BitsConcat<40, 111> for Reg128Bits<71> {}
impl Reg128BitsDownCast<41> for Reg128Bits<71> {}
impl Reg128BitsConcat<41, 112> for Reg128Bits<71> {}
impl Reg128BitsDownCast<42> for Reg128Bits<71> {}
impl Reg128BitsConcat<42, 113> for Reg128Bits<71> {}
impl Reg128BitsDownCast<43> for Reg128Bits<71> {}
impl Reg128BitsConcat<43, 114> for Reg128Bits<71> {}
impl Reg128BitsDownCast<44> for Reg128Bits<71> {}
impl Reg128BitsConcat<44, 115> for Reg128Bits<71> {}
impl Reg128BitsDownCast<45> for Reg128Bits<71> {}
impl Reg128BitsConcat<45, 116> for Reg128Bits<71> {}
impl Reg128BitsDownCast<46> for Reg128Bits<71> {}
impl Reg128BitsConcat<46, 117> for Reg128Bits<71> {}
impl Reg128BitsDownCast<47> for Reg128Bits<71> {}
impl Reg128BitsConcat<47, 118> for Reg128Bits<71> {}
impl Reg128BitsDownCast<48> for Reg128Bits<71> {}
impl Reg128BitsConcat<48, 119> for Reg128Bits<71> {}
impl Reg128BitsDownCast<49> for Reg128Bits<71> {}
impl Reg128BitsConcat<49, 120> for Reg128Bits<71> {}
impl Reg128BitsDownCast<50> for Reg128Bits<71> {}
impl Reg128BitsConcat<50, 121> for Reg128Bits<71> {}
impl Reg128BitsDownCast<51> for Reg128Bits<71> {}
impl Reg128BitsConcat<51, 122> for Reg128Bits<71> {}
impl Reg128BitsDownCast<52> for Reg128Bits<71> {}
impl Reg128BitsConcat<52, 123> for Reg128Bits<71> {}
impl Reg128BitsDownCast<53> for Reg128Bits<71> {}
impl Reg128BitsConcat<53, 124> for Reg128Bits<71> {}
impl Reg128BitsDownCast<54> for Reg128Bits<71> {}
impl Reg128BitsConcat<54, 125> for Reg128Bits<71> {}
impl Reg128BitsDownCast<55> for Reg128Bits<71> {}
impl Reg128BitsConcat<55, 126> for Reg128Bits<71> {}
impl Reg128BitsDownCast<56> for Reg128Bits<71> {}
impl Reg128BitsConcat<56, 127> for Reg128Bits<71> {}
impl Reg128BitsDownCast<57> for Reg128Bits<71> {}
impl Reg128BitsConcat<57, 128> for Reg128Bits<71> {}
impl Reg128BitsDownCast<58> for Reg128Bits<71> {}
impl Reg128BitsDownCast<59> for Reg128Bits<71> {}
impl Reg128BitsDownCast<60> for Reg128Bits<71> {}
impl Reg128BitsDownCast<61> for Reg128Bits<71> {}
impl Reg128BitsDownCast<62> for Reg128Bits<71> {}
impl Reg128BitsDownCast<63> for Reg128Bits<71> {}
impl Reg128BitsDownCast<64> for Reg128Bits<71> {}
impl Reg128BitsDownCast<65> for Reg128Bits<71> {}
impl Reg128BitsDownCast<66> for Reg128Bits<71> {}
impl Reg128BitsDownCast<67> for Reg128Bits<71> {}
impl Reg128BitsDownCast<68> for Reg128Bits<71> {}
impl Reg128BitsDownCast<69> for Reg128Bits<71> {}
impl Reg128BitsDownCast<70> for Reg128Bits<71> {}
impl Reg128BitsDownCast<71> for Reg128Bits<71> {}
impl Reg128BitsDownCast<1> for Reg128Bits<72> {}
impl Reg128BitsConcat<1, 73> for Reg128Bits<72> {}
impl Reg128BitsDownCast<2> for Reg128Bits<72> {}
impl Reg128BitsConcat<2, 74> for Reg128Bits<72> {}
impl Reg128BitsDownCast<3> for Reg128Bits<72> {}
impl Reg128BitsConcat<3, 75> for Reg128Bits<72> {}
impl Reg128BitsDownCast<4> for Reg128Bits<72> {}
impl Reg128BitsConcat<4, 76> for Reg128Bits<72> {}
impl Reg128BitsDownCast<5> for Reg128Bits<72> {}
impl Reg128BitsConcat<5, 77> for Reg128Bits<72> {}
impl Reg128BitsDownCast<6> for Reg128Bits<72> {}
impl Reg128BitsConcat<6, 78> for Reg128Bits<72> {}
impl Reg128BitsDownCast<7> for Reg128Bits<72> {}
impl Reg128BitsConcat<7, 79> for Reg128Bits<72> {}
impl Reg128BitsDownCast<8> for Reg128Bits<72> {}
impl Reg128BitsConcat<8, 80> for Reg128Bits<72> {}
impl Reg128BitsDownCast<9> for Reg128Bits<72> {}
impl Reg128BitsConcat<9, 81> for Reg128Bits<72> {}
impl Reg128BitsDownCast<10> for Reg128Bits<72> {}
impl Reg128BitsConcat<10, 82> for Reg128Bits<72> {}
impl Reg128BitsDownCast<11> for Reg128Bits<72> {}
impl Reg128BitsConcat<11, 83> for Reg128Bits<72> {}
impl Reg128BitsDownCast<12> for Reg128Bits<72> {}
impl Reg128BitsConcat<12, 84> for Reg128Bits<72> {}
impl Reg128BitsDownCast<13> for Reg128Bits<72> {}
impl Reg128BitsConcat<13, 85> for Reg128Bits<72> {}
impl Reg128BitsDownCast<14> for Reg128Bits<72> {}
impl Reg128BitsConcat<14, 86> for Reg128Bits<72> {}
impl Reg128BitsDownCast<15> for Reg128Bits<72> {}
impl Reg128BitsConcat<15, 87> for Reg128Bits<72> {}
impl Reg128BitsDownCast<16> for Reg128Bits<72> {}
impl Reg128BitsConcat<16, 88> for Reg128Bits<72> {}
impl Reg128BitsDownCast<17> for Reg128Bits<72> {}
impl Reg128BitsConcat<17, 89> for Reg128Bits<72> {}
impl Reg128BitsDownCast<18> for Reg128Bits<72> {}
impl Reg128BitsConcat<18, 90> for Reg128Bits<72> {}
impl Reg128BitsDownCast<19> for Reg128Bits<72> {}
impl Reg128BitsConcat<19, 91> for Reg128Bits<72> {}
impl Reg128BitsDownCast<20> for Reg128Bits<72> {}
impl Reg128BitsConcat<20, 92> for Reg128Bits<72> {}
impl Reg128BitsDownCast<21> for Reg128Bits<72> {}
impl Reg128BitsConcat<21, 93> for Reg128Bits<72> {}
impl Reg128BitsDownCast<22> for Reg128Bits<72> {}
impl Reg128BitsConcat<22, 94> for Reg128Bits<72> {}
impl Reg128BitsDownCast<23> for Reg128Bits<72> {}
impl Reg128BitsConcat<23, 95> for Reg128Bits<72> {}
impl Reg128BitsDownCast<24> for Reg128Bits<72> {}
impl Reg128BitsConcat<24, 96> for Reg128Bits<72> {}
impl Reg128BitsDownCast<25> for Reg128Bits<72> {}
impl Reg128BitsConcat<25, 97> for Reg128Bits<72> {}
impl Reg128BitsDownCast<26> for Reg128Bits<72> {}
impl Reg128BitsConcat<26, 98> for Reg128Bits<72> {}
impl Reg128BitsDownCast<27> for Reg128Bits<72> {}
impl Reg128BitsConcat<27, 99> for Reg128Bits<72> {}
impl Reg128BitsDownCast<28> for Reg128Bits<72> {}
impl Reg128BitsConcat<28, 100> for Reg128Bits<72> {}
impl Reg128BitsDownCast<29> for Reg128Bits<72> {}
impl Reg128BitsConcat<29, 101> for Reg128Bits<72> {}
impl Reg128BitsDownCast<30> for Reg128Bits<72> {}
impl Reg128BitsConcat<30, 102> for Reg128Bits<72> {}
impl Reg128BitsDownCast<31> for Reg128Bits<72> {}
impl Reg128BitsConcat<31, 103> for Reg128Bits<72> {}
impl Reg128BitsDownCast<32> for Reg128Bits<72> {}
impl Reg128BitsConcat<32, 104> for Reg128Bits<72> {}
impl Reg128BitsDownCast<33> for Reg128Bits<72> {}
impl Reg128BitsConcat<33, 105> for Reg128Bits<72> {}
impl Reg128BitsDownCast<34> for Reg128Bits<72> {}
impl Reg128BitsConcat<34, 106> for Reg128Bits<72> {}
impl Reg128BitsDownCast<35> for Reg128Bits<72> {}
impl Reg128BitsConcat<35, 107> for Reg128Bits<72> {}
impl Reg128BitsDownCast<36> for Reg128Bits<72> {}
impl Reg128BitsConcat<36, 108> for Reg128Bits<72> {}
impl Reg128BitsDownCast<37> for Reg128Bits<72> {}
impl Reg128BitsConcat<37, 109> for Reg128Bits<72> {}
impl Reg128BitsDownCast<38> for Reg128Bits<72> {}
impl Reg128BitsConcat<38, 110> for Reg128Bits<72> {}
impl Reg128BitsDownCast<39> for Reg128Bits<72> {}
impl Reg128BitsConcat<39, 111> for Reg128Bits<72> {}
impl Reg128BitsDownCast<40> for Reg128Bits<72> {}
impl Reg128BitsConcat<40, 112> for Reg128Bits<72> {}
impl Reg128BitsDownCast<41> for Reg128Bits<72> {}
impl Reg128BitsConcat<41, 113> for Reg128Bits<72> {}
impl Reg128BitsDownCast<42> for Reg128Bits<72> {}
impl Reg128BitsConcat<42, 114> for Reg128Bits<72> {}
impl Reg128BitsDownCast<43> for Reg128Bits<72> {}
impl Reg128BitsConcat<43, 115> for Reg128Bits<72> {}
impl Reg128BitsDownCast<44> for Reg128Bits<72> {}
impl Reg128BitsConcat<44, 116> for Reg128Bits<72> {}
impl Reg128BitsDownCast<45> for Reg128Bits<72> {}
impl Reg128BitsConcat<45, 117> for Reg128Bits<72> {}
impl Reg128BitsDownCast<46> for Reg128Bits<72> {}
impl Reg128BitsConcat<46, 118> for Reg128Bits<72> {}
impl Reg128BitsDownCast<47> for Reg128Bits<72> {}
impl Reg128BitsConcat<47, 119> for Reg128Bits<72> {}
impl Reg128BitsDownCast<48> for Reg128Bits<72> {}
impl Reg128BitsConcat<48, 120> for Reg128Bits<72> {}
impl Reg128BitsDownCast<49> for Reg128Bits<72> {}
impl Reg128BitsConcat<49, 121> for Reg128Bits<72> {}
impl Reg128BitsDownCast<50> for Reg128Bits<72> {}
impl Reg128BitsConcat<50, 122> for Reg128Bits<72> {}
impl Reg128BitsDownCast<51> for Reg128Bits<72> {}
impl Reg128BitsConcat<51, 123> for Reg128Bits<72> {}
impl Reg128BitsDownCast<52> for Reg128Bits<72> {}
impl Reg128BitsConcat<52, 124> for Reg128Bits<72> {}
impl Reg128BitsDownCast<53> for Reg128Bits<72> {}
impl Reg128BitsConcat<53, 125> for Reg128Bits<72> {}
impl Reg128BitsDownCast<54> for Reg128Bits<72> {}
impl Reg128BitsConcat<54, 126> for Reg128Bits<72> {}
impl Reg128BitsDownCast<55> for Reg128Bits<72> {}
impl Reg128BitsConcat<55, 127> for Reg128Bits<72> {}
impl Reg128BitsDownCast<56> for Reg128Bits<72> {}
impl Reg128BitsConcat<56, 128> for Reg128Bits<72> {}
impl Reg128BitsDownCast<57> for Reg128Bits<72> {}
impl Reg128BitsDownCast<58> for Reg128Bits<72> {}
impl Reg128BitsDownCast<59> for Reg128Bits<72> {}
impl Reg128BitsDownCast<60> for Reg128Bits<72> {}
impl Reg128BitsDownCast<61> for Reg128Bits<72> {}
impl Reg128BitsDownCast<62> for Reg128Bits<72> {}
impl Reg128BitsDownCast<63> for Reg128Bits<72> {}
impl Reg128BitsDownCast<64> for Reg128Bits<72> {}
impl Reg128BitsDownCast<65> for Reg128Bits<72> {}
impl Reg128BitsDownCast<66> for Reg128Bits<72> {}
impl Reg128BitsDownCast<67> for Reg128Bits<72> {}
impl Reg128BitsDownCast<68> for Reg128Bits<72> {}
impl Reg128BitsDownCast<69> for Reg128Bits<72> {}
impl Reg128BitsDownCast<70> for Reg128Bits<72> {}
impl Reg128BitsDownCast<71> for Reg128Bits<72> {}
impl Reg128BitsDownCast<72> for Reg128Bits<72> {}
impl Reg128BitsDownCast<1> for Reg128Bits<73> {}
impl Reg128BitsConcat<1, 74> for Reg128Bits<73> {}
impl Reg128BitsDownCast<2> for Reg128Bits<73> {}
impl Reg128BitsConcat<2, 75> for Reg128Bits<73> {}
impl Reg128BitsDownCast<3> for Reg128Bits<73> {}
impl Reg128BitsConcat<3, 76> for Reg128Bits<73> {}
impl Reg128BitsDownCast<4> for Reg128Bits<73> {}
impl Reg128BitsConcat<4, 77> for Reg128Bits<73> {}
impl Reg128BitsDownCast<5> for Reg128Bits<73> {}
impl Reg128BitsConcat<5, 78> for Reg128Bits<73> {}
impl Reg128BitsDownCast<6> for Reg128Bits<73> {}
impl Reg128BitsConcat<6, 79> for Reg128Bits<73> {}
impl Reg128BitsDownCast<7> for Reg128Bits<73> {}
impl Reg128BitsConcat<7, 80> for Reg128Bits<73> {}
impl Reg128BitsDownCast<8> for Reg128Bits<73> {}
impl Reg128BitsConcat<8, 81> for Reg128Bits<73> {}
impl Reg128BitsDownCast<9> for Reg128Bits<73> {}
impl Reg128BitsConcat<9, 82> for Reg128Bits<73> {}
impl Reg128BitsDownCast<10> for Reg128Bits<73> {}
impl Reg128BitsConcat<10, 83> for Reg128Bits<73> {}
impl Reg128BitsDownCast<11> for Reg128Bits<73> {}
impl Reg128BitsConcat<11, 84> for Reg128Bits<73> {}
impl Reg128BitsDownCast<12> for Reg128Bits<73> {}
impl Reg128BitsConcat<12, 85> for Reg128Bits<73> {}
impl Reg128BitsDownCast<13> for Reg128Bits<73> {}
impl Reg128BitsConcat<13, 86> for Reg128Bits<73> {}
impl Reg128BitsDownCast<14> for Reg128Bits<73> {}
impl Reg128BitsConcat<14, 87> for Reg128Bits<73> {}
impl Reg128BitsDownCast<15> for Reg128Bits<73> {}
impl Reg128BitsConcat<15, 88> for Reg128Bits<73> {}
impl Reg128BitsDownCast<16> for Reg128Bits<73> {}
impl Reg128BitsConcat<16, 89> for Reg128Bits<73> {}
impl Reg128BitsDownCast<17> for Reg128Bits<73> {}
impl Reg128BitsConcat<17, 90> for Reg128Bits<73> {}
impl Reg128BitsDownCast<18> for Reg128Bits<73> {}
impl Reg128BitsConcat<18, 91> for Reg128Bits<73> {}
impl Reg128BitsDownCast<19> for Reg128Bits<73> {}
impl Reg128BitsConcat<19, 92> for Reg128Bits<73> {}
impl Reg128BitsDownCast<20> for Reg128Bits<73> {}
impl Reg128BitsConcat<20, 93> for Reg128Bits<73> {}
impl Reg128BitsDownCast<21> for Reg128Bits<73> {}
impl Reg128BitsConcat<21, 94> for Reg128Bits<73> {}
impl Reg128BitsDownCast<22> for Reg128Bits<73> {}
impl Reg128BitsConcat<22, 95> for Reg128Bits<73> {}
impl Reg128BitsDownCast<23> for Reg128Bits<73> {}
impl Reg128BitsConcat<23, 96> for Reg128Bits<73> {}
impl Reg128BitsDownCast<24> for Reg128Bits<73> {}
impl Reg128BitsConcat<24, 97> for Reg128Bits<73> {}
impl Reg128BitsDownCast<25> for Reg128Bits<73> {}
impl Reg128BitsConcat<25, 98> for Reg128Bits<73> {}
impl Reg128BitsDownCast<26> for Reg128Bits<73> {}
impl Reg128BitsConcat<26, 99> for Reg128Bits<73> {}
impl Reg128BitsDownCast<27> for Reg128Bits<73> {}
impl Reg128BitsConcat<27, 100> for Reg128Bits<73> {}
impl Reg128BitsDownCast<28> for Reg128Bits<73> {}
impl Reg128BitsConcat<28, 101> for Reg128Bits<73> {}
impl Reg128BitsDownCast<29> for Reg128Bits<73> {}
impl Reg128BitsConcat<29, 102> for Reg128Bits<73> {}
impl Reg128BitsDownCast<30> for Reg128Bits<73> {}
impl Reg128BitsConcat<30, 103> for Reg128Bits<73> {}
impl Reg128BitsDownCast<31> for Reg128Bits<73> {}
impl Reg128BitsConcat<31, 104> for Reg128Bits<73> {}
impl Reg128BitsDownCast<32> for Reg128Bits<73> {}
impl Reg128BitsConcat<32, 105> for Reg128Bits<73> {}
impl Reg128BitsDownCast<33> for Reg128Bits<73> {}
impl Reg128BitsConcat<33, 106> for Reg128Bits<73> {}
impl Reg128BitsDownCast<34> for Reg128Bits<73> {}
impl Reg128BitsConcat<34, 107> for Reg128Bits<73> {}
impl Reg128BitsDownCast<35> for Reg128Bits<73> {}
impl Reg128BitsConcat<35, 108> for Reg128Bits<73> {}
impl Reg128BitsDownCast<36> for Reg128Bits<73> {}
impl Reg128BitsConcat<36, 109> for Reg128Bits<73> {}
impl Reg128BitsDownCast<37> for Reg128Bits<73> {}
impl Reg128BitsConcat<37, 110> for Reg128Bits<73> {}
impl Reg128BitsDownCast<38> for Reg128Bits<73> {}
impl Reg128BitsConcat<38, 111> for Reg128Bits<73> {}
impl Reg128BitsDownCast<39> for Reg128Bits<73> {}
impl Reg128BitsConcat<39, 112> for Reg128Bits<73> {}
impl Reg128BitsDownCast<40> for Reg128Bits<73> {}
impl Reg128BitsConcat<40, 113> for Reg128Bits<73> {}
impl Reg128BitsDownCast<41> for Reg128Bits<73> {}
impl Reg128BitsConcat<41, 114> for Reg128Bits<73> {}
impl Reg128BitsDownCast<42> for Reg128Bits<73> {}
impl Reg128BitsConcat<42, 115> for Reg128Bits<73> {}
impl Reg128BitsDownCast<43> for Reg128Bits<73> {}
impl Reg128BitsConcat<43, 116> for Reg128Bits<73> {}
impl Reg128BitsDownCast<44> for Reg128Bits<73> {}
impl Reg128BitsConcat<44, 117> for Reg128Bits<73> {}
impl Reg128BitsDownCast<45> for Reg128Bits<73> {}
impl Reg128BitsConcat<45, 118> for Reg128Bits<73> {}
impl Reg128BitsDownCast<46> for Reg128Bits<73> {}
impl Reg128BitsConcat<46, 119> for Reg128Bits<73> {}
impl Reg128BitsDownCast<47> for Reg128Bits<73> {}
impl Reg128BitsConcat<47, 120> for Reg128Bits<73> {}
impl Reg128BitsDownCast<48> for Reg128Bits<73> {}
impl Reg128BitsConcat<48, 121> for Reg128Bits<73> {}
impl Reg128BitsDownCast<49> for Reg128Bits<73> {}
impl Reg128BitsConcat<49, 122> for Reg128Bits<73> {}
impl Reg128BitsDownCast<50> for Reg128Bits<73> {}
impl Reg128BitsConcat<50, 123> for Reg128Bits<73> {}
impl Reg128BitsDownCast<51> for Reg128Bits<73> {}
impl Reg128BitsConcat<51, 124> for Reg128Bits<73> {}
impl Reg128BitsDownCast<52> for Reg128Bits<73> {}
impl Reg128BitsConcat<52, 125> for Reg128Bits<73> {}
impl Reg128BitsDownCast<53> for Reg128Bits<73> {}
impl Reg128BitsConcat<53, 126> for Reg128Bits<73> {}
impl Reg128BitsDownCast<54> for Reg128Bits<73> {}
impl Reg128BitsConcat<54, 127> for Reg128Bits<73> {}
impl Reg128BitsDownCast<55> for Reg128Bits<73> {}
impl Reg128BitsConcat<55, 128> for Reg128Bits<73> {}
impl Reg128BitsDownCast<56> for Reg128Bits<73> {}
impl Reg128BitsDownCast<57> for Reg128Bits<73> {}
impl Reg128BitsDownCast<58> for Reg128Bits<73> {}
impl Reg128BitsDownCast<59> for Reg128Bits<73> {}
impl Reg128BitsDownCast<60> for Reg128Bits<73> {}
impl Reg128BitsDownCast<61> for Reg128Bits<73> {}
impl Reg128BitsDownCast<62> for Reg128Bits<73> {}
impl Reg128BitsDownCast<63> for Reg128Bits<73> {}
impl Reg128BitsDownCast<64> for Reg128Bits<73> {}
impl Reg128BitsDownCast<65> for Reg128Bits<73> {}
impl Reg128BitsDownCast<66> for Reg128Bits<73> {}
impl Reg128BitsDownCast<67> for Reg128Bits<73> {}
impl Reg128BitsDownCast<68> for Reg128Bits<73> {}
impl Reg128BitsDownCast<69> for Reg128Bits<73> {}
impl Reg128BitsDownCast<70> for Reg128Bits<73> {}
impl Reg128BitsDownCast<71> for Reg128Bits<73> {}
impl Reg128BitsDownCast<72> for Reg128Bits<73> {}
impl Reg128BitsDownCast<73> for Reg128Bits<73> {}
impl Reg128BitsDownCast<1> for Reg128Bits<74> {}
impl Reg128BitsConcat<1, 75> for Reg128Bits<74> {}
impl Reg128BitsDownCast<2> for Reg128Bits<74> {}
impl Reg128BitsConcat<2, 76> for Reg128Bits<74> {}
impl Reg128BitsDownCast<3> for Reg128Bits<74> {}
impl Reg128BitsConcat<3, 77> for Reg128Bits<74> {}
impl Reg128BitsDownCast<4> for Reg128Bits<74> {}
impl Reg128BitsConcat<4, 78> for Reg128Bits<74> {}
impl Reg128BitsDownCast<5> for Reg128Bits<74> {}
impl Reg128BitsConcat<5, 79> for Reg128Bits<74> {}
impl Reg128BitsDownCast<6> for Reg128Bits<74> {}
impl Reg128BitsConcat<6, 80> for Reg128Bits<74> {}
impl Reg128BitsDownCast<7> for Reg128Bits<74> {}
impl Reg128BitsConcat<7, 81> for Reg128Bits<74> {}
impl Reg128BitsDownCast<8> for Reg128Bits<74> {}
impl Reg128BitsConcat<8, 82> for Reg128Bits<74> {}
impl Reg128BitsDownCast<9> for Reg128Bits<74> {}
impl Reg128BitsConcat<9, 83> for Reg128Bits<74> {}
impl Reg128BitsDownCast<10> for Reg128Bits<74> {}
impl Reg128BitsConcat<10, 84> for Reg128Bits<74> {}
impl Reg128BitsDownCast<11> for Reg128Bits<74> {}
impl Reg128BitsConcat<11, 85> for Reg128Bits<74> {}
impl Reg128BitsDownCast<12> for Reg128Bits<74> {}
impl Reg128BitsConcat<12, 86> for Reg128Bits<74> {}
impl Reg128BitsDownCast<13> for Reg128Bits<74> {}
impl Reg128BitsConcat<13, 87> for Reg128Bits<74> {}
impl Reg128BitsDownCast<14> for Reg128Bits<74> {}
impl Reg128BitsConcat<14, 88> for Reg128Bits<74> {}
impl Reg128BitsDownCast<15> for Reg128Bits<74> {}
impl Reg128BitsConcat<15, 89> for Reg128Bits<74> {}
impl Reg128BitsDownCast<16> for Reg128Bits<74> {}
impl Reg128BitsConcat<16, 90> for Reg128Bits<74> {}
impl Reg128BitsDownCast<17> for Reg128Bits<74> {}
impl Reg128BitsConcat<17, 91> for Reg128Bits<74> {}
impl Reg128BitsDownCast<18> for Reg128Bits<74> {}
impl Reg128BitsConcat<18, 92> for Reg128Bits<74> {}
impl Reg128BitsDownCast<19> for Reg128Bits<74> {}
impl Reg128BitsConcat<19, 93> for Reg128Bits<74> {}
impl Reg128BitsDownCast<20> for Reg128Bits<74> {}
impl Reg128BitsConcat<20, 94> for Reg128Bits<74> {}
impl Reg128BitsDownCast<21> for Reg128Bits<74> {}
impl Reg128BitsConcat<21, 95> for Reg128Bits<74> {}
impl Reg128BitsDownCast<22> for Reg128Bits<74> {}
impl Reg128BitsConcat<22, 96> for Reg128Bits<74> {}
impl Reg128BitsDownCast<23> for Reg128Bits<74> {}
impl Reg128BitsConcat<23, 97> for Reg128Bits<74> {}
impl Reg128BitsDownCast<24> for Reg128Bits<74> {}
impl Reg128BitsConcat<24, 98> for Reg128Bits<74> {}
impl Reg128BitsDownCast<25> for Reg128Bits<74> {}
impl Reg128BitsConcat<25, 99> for Reg128Bits<74> {}
impl Reg128BitsDownCast<26> for Reg128Bits<74> {}
impl Reg128BitsConcat<26, 100> for Reg128Bits<74> {}
impl Reg128BitsDownCast<27> for Reg128Bits<74> {}
impl Reg128BitsConcat<27, 101> for Reg128Bits<74> {}
impl Reg128BitsDownCast<28> for Reg128Bits<74> {}
impl Reg128BitsConcat<28, 102> for Reg128Bits<74> {}
impl Reg128BitsDownCast<29> for Reg128Bits<74> {}
impl Reg128BitsConcat<29, 103> for Reg128Bits<74> {}
impl Reg128BitsDownCast<30> for Reg128Bits<74> {}
impl Reg128BitsConcat<30, 104> for Reg128Bits<74> {}
impl Reg128BitsDownCast<31> for Reg128Bits<74> {}
impl Reg128BitsConcat<31, 105> for Reg128Bits<74> {}
impl Reg128BitsDownCast<32> for Reg128Bits<74> {}
impl Reg128BitsConcat<32, 106> for Reg128Bits<74> {}
impl Reg128BitsDownCast<33> for Reg128Bits<74> {}
impl Reg128BitsConcat<33, 107> for Reg128Bits<74> {}
impl Reg128BitsDownCast<34> for Reg128Bits<74> {}
impl Reg128BitsConcat<34, 108> for Reg128Bits<74> {}
impl Reg128BitsDownCast<35> for Reg128Bits<74> {}
impl Reg128BitsConcat<35, 109> for Reg128Bits<74> {}
impl Reg128BitsDownCast<36> for Reg128Bits<74> {}
impl Reg128BitsConcat<36, 110> for Reg128Bits<74> {}
impl Reg128BitsDownCast<37> for Reg128Bits<74> {}
impl Reg128BitsConcat<37, 111> for Reg128Bits<74> {}
impl Reg128BitsDownCast<38> for Reg128Bits<74> {}
impl Reg128BitsConcat<38, 112> for Reg128Bits<74> {}
impl Reg128BitsDownCast<39> for Reg128Bits<74> {}
impl Reg128BitsConcat<39, 113> for Reg128Bits<74> {}
impl Reg128BitsDownCast<40> for Reg128Bits<74> {}
impl Reg128BitsConcat<40, 114> for Reg128Bits<74> {}
impl Reg128BitsDownCast<41> for Reg128Bits<74> {}
impl Reg128BitsConcat<41, 115> for Reg128Bits<74> {}
impl Reg128BitsDownCast<42> for Reg128Bits<74> {}
impl Reg128BitsConcat<42, 116> for Reg128Bits<74> {}
impl Reg128BitsDownCast<43> for Reg128Bits<74> {}
impl Reg128BitsConcat<43, 117> for Reg128Bits<74> {}
impl Reg128BitsDownCast<44> for Reg128Bits<74> {}
impl Reg128BitsConcat<44, 118> for Reg128Bits<74> {}
impl Reg128BitsDownCast<45> for Reg128Bits<74> {}
impl Reg128BitsConcat<45, 119> for Reg128Bits<74> {}
impl Reg128BitsDownCast<46> for Reg128Bits<74> {}
impl Reg128BitsConcat<46, 120> for Reg128Bits<74> {}
impl Reg128BitsDownCast<47> for Reg128Bits<74> {}
impl Reg128BitsConcat<47, 121> for Reg128Bits<74> {}
impl Reg128BitsDownCast<48> for Reg128Bits<74> {}
impl Reg128BitsConcat<48, 122> for Reg128Bits<74> {}
impl Reg128BitsDownCast<49> for Reg128Bits<74> {}
impl Reg128BitsConcat<49, 123> for Reg128Bits<74> {}
impl Reg128BitsDownCast<50> for Reg128Bits<74> {}
impl Reg128BitsConcat<50, 124> for Reg128Bits<74> {}
impl Reg128BitsDownCast<51> for Reg128Bits<74> {}
impl Reg128BitsConcat<51, 125> for Reg128Bits<74> {}
impl Reg128BitsDownCast<52> for Reg128Bits<74> {}
impl Reg128BitsConcat<52, 126> for Reg128Bits<74> {}
impl Reg128BitsDownCast<53> for Reg128Bits<74> {}
impl Reg128BitsConcat<53, 127> for Reg128Bits<74> {}
impl Reg128BitsDownCast<54> for Reg128Bits<74> {}
impl Reg128BitsConcat<54, 128> for Reg128Bits<74> {}
impl Reg128BitsDownCast<55> for Reg128Bits<74> {}
impl Reg128BitsDownCast<56> for Reg128Bits<74> {}
impl Reg128BitsDownCast<57> for Reg128Bits<74> {}
impl Reg128BitsDownCast<58> for Reg128Bits<74> {}
impl Reg128BitsDownCast<59> for Reg128Bits<74> {}
impl Reg128BitsDownCast<60> for Reg128Bits<74> {}
impl Reg128BitsDownCast<61> for Reg128Bits<74> {}
impl Reg128BitsDownCast<62> for Reg128Bits<74> {}
impl Reg128BitsDownCast<63> for Reg128Bits<74> {}
impl Reg128BitsDownCast<64> for Reg128Bits<74> {}
impl Reg128BitsDownCast<65> for Reg128Bits<74> {}
impl Reg128BitsDownCast<66> for Reg128Bits<74> {}
impl Reg128BitsDownCast<67> for Reg128Bits<74> {}
impl Reg128BitsDownCast<68> for Reg128Bits<74> {}
impl Reg128BitsDownCast<69> for Reg128Bits<74> {}
impl Reg128BitsDownCast<70> for Reg128Bits<74> {}
impl Reg128BitsDownCast<71> for Reg128Bits<74> {}
impl Reg128BitsDownCast<72> for Reg128Bits<74> {}
impl Reg128BitsDownCast<73> for Reg128Bits<74> {}
impl Reg128BitsDownCast<74> for Reg128Bits<74> {}
impl Reg128BitsDownCast<1> for Reg128Bits<75> {}
impl Reg128BitsConcat<1, 76> for Reg128Bits<75> {}
impl Reg128BitsDownCast<2> for Reg128Bits<75> {}
impl Reg128BitsConcat<2, 77> for Reg128Bits<75> {}
impl Reg128BitsDownCast<3> for Reg128Bits<75> {}
impl Reg128BitsConcat<3, 78> for Reg128Bits<75> {}
impl Reg128BitsDownCast<4> for Reg128Bits<75> {}
impl Reg128BitsConcat<4, 79> for Reg128Bits<75> {}
impl Reg128BitsDownCast<5> for Reg128Bits<75> {}
impl Reg128BitsConcat<5, 80> for Reg128Bits<75> {}
impl Reg128BitsDownCast<6> for Reg128Bits<75> {}
impl Reg128BitsConcat<6, 81> for Reg128Bits<75> {}
impl Reg128BitsDownCast<7> for Reg128Bits<75> {}
impl Reg128BitsConcat<7, 82> for Reg128Bits<75> {}
impl Reg128BitsDownCast<8> for Reg128Bits<75> {}
impl Reg128BitsConcat<8, 83> for Reg128Bits<75> {}
impl Reg128BitsDownCast<9> for Reg128Bits<75> {}
impl Reg128BitsConcat<9, 84> for Reg128Bits<75> {}
impl Reg128BitsDownCast<10> for Reg128Bits<75> {}
impl Reg128BitsConcat<10, 85> for Reg128Bits<75> {}
impl Reg128BitsDownCast<11> for Reg128Bits<75> {}
impl Reg128BitsConcat<11, 86> for Reg128Bits<75> {}
impl Reg128BitsDownCast<12> for Reg128Bits<75> {}
impl Reg128BitsConcat<12, 87> for Reg128Bits<75> {}
impl Reg128BitsDownCast<13> for Reg128Bits<75> {}
impl Reg128BitsConcat<13, 88> for Reg128Bits<75> {}
impl Reg128BitsDownCast<14> for Reg128Bits<75> {}
impl Reg128BitsConcat<14, 89> for Reg128Bits<75> {}
impl Reg128BitsDownCast<15> for Reg128Bits<75> {}
impl Reg128BitsConcat<15, 90> for Reg128Bits<75> {}
impl Reg128BitsDownCast<16> for Reg128Bits<75> {}
impl Reg128BitsConcat<16, 91> for Reg128Bits<75> {}
impl Reg128BitsDownCast<17> for Reg128Bits<75> {}
impl Reg128BitsConcat<17, 92> for Reg128Bits<75> {}
impl Reg128BitsDownCast<18> for Reg128Bits<75> {}
impl Reg128BitsConcat<18, 93> for Reg128Bits<75> {}
impl Reg128BitsDownCast<19> for Reg128Bits<75> {}
impl Reg128BitsConcat<19, 94> for Reg128Bits<75> {}
impl Reg128BitsDownCast<20> for Reg128Bits<75> {}
impl Reg128BitsConcat<20, 95> for Reg128Bits<75> {}
impl Reg128BitsDownCast<21> for Reg128Bits<75> {}
impl Reg128BitsConcat<21, 96> for Reg128Bits<75> {}
impl Reg128BitsDownCast<22> for Reg128Bits<75> {}
impl Reg128BitsConcat<22, 97> for Reg128Bits<75> {}
impl Reg128BitsDownCast<23> for Reg128Bits<75> {}
impl Reg128BitsConcat<23, 98> for Reg128Bits<75> {}
impl Reg128BitsDownCast<24> for Reg128Bits<75> {}
impl Reg128BitsConcat<24, 99> for Reg128Bits<75> {}
impl Reg128BitsDownCast<25> for Reg128Bits<75> {}
impl Reg128BitsConcat<25, 100> for Reg128Bits<75> {}
impl Reg128BitsDownCast<26> for Reg128Bits<75> {}
impl Reg128BitsConcat<26, 101> for Reg128Bits<75> {}
impl Reg128BitsDownCast<27> for Reg128Bits<75> {}
impl Reg128BitsConcat<27, 102> for Reg128Bits<75> {}
impl Reg128BitsDownCast<28> for Reg128Bits<75> {}
impl Reg128BitsConcat<28, 103> for Reg128Bits<75> {}
impl Reg128BitsDownCast<29> for Reg128Bits<75> {}
impl Reg128BitsConcat<29, 104> for Reg128Bits<75> {}
impl Reg128BitsDownCast<30> for Reg128Bits<75> {}
impl Reg128BitsConcat<30, 105> for Reg128Bits<75> {}
impl Reg128BitsDownCast<31> for Reg128Bits<75> {}
impl Reg128BitsConcat<31, 106> for Reg128Bits<75> {}
impl Reg128BitsDownCast<32> for Reg128Bits<75> {}
impl Reg128BitsConcat<32, 107> for Reg128Bits<75> {}
impl Reg128BitsDownCast<33> for Reg128Bits<75> {}
impl Reg128BitsConcat<33, 108> for Reg128Bits<75> {}
impl Reg128BitsDownCast<34> for Reg128Bits<75> {}
impl Reg128BitsConcat<34, 109> for Reg128Bits<75> {}
impl Reg128BitsDownCast<35> for Reg128Bits<75> {}
impl Reg128BitsConcat<35, 110> for Reg128Bits<75> {}
impl Reg128BitsDownCast<36> for Reg128Bits<75> {}
impl Reg128BitsConcat<36, 111> for Reg128Bits<75> {}
impl Reg128BitsDownCast<37> for Reg128Bits<75> {}
impl Reg128BitsConcat<37, 112> for Reg128Bits<75> {}
impl Reg128BitsDownCast<38> for Reg128Bits<75> {}
impl Reg128BitsConcat<38, 113> for Reg128Bits<75> {}
impl Reg128BitsDownCast<39> for Reg128Bits<75> {}
impl Reg128BitsConcat<39, 114> for Reg128Bits<75> {}
impl Reg128BitsDownCast<40> for Reg128Bits<75> {}
impl Reg128BitsConcat<40, 115> for Reg128Bits<75> {}
impl Reg128BitsDownCast<41> for Reg128Bits<75> {}
impl Reg128BitsConcat<41, 116> for Reg128Bits<75> {}
impl Reg128BitsDownCast<42> for Reg128Bits<75> {}
impl Reg128BitsConcat<42, 117> for Reg128Bits<75> {}
impl Reg128BitsDownCast<43> for Reg128Bits<75> {}
impl Reg128BitsConcat<43, 118> for Reg128Bits<75> {}
impl Reg128BitsDownCast<44> for Reg128Bits<75> {}
impl Reg128BitsConcat<44, 119> for Reg128Bits<75> {}
impl Reg128BitsDownCast<45> for Reg128Bits<75> {}
impl Reg128BitsConcat<45, 120> for Reg128Bits<75> {}
impl Reg128BitsDownCast<46> for Reg128Bits<75> {}
impl Reg128BitsConcat<46, 121> for Reg128Bits<75> {}
impl Reg128BitsDownCast<47> for Reg128Bits<75> {}
impl Reg128BitsConcat<47, 122> for Reg128Bits<75> {}
impl Reg128BitsDownCast<48> for Reg128Bits<75> {}
impl Reg128BitsConcat<48, 123> for Reg128Bits<75> {}
impl Reg128BitsDownCast<49> for Reg128Bits<75> {}
impl Reg128BitsConcat<49, 124> for Reg128Bits<75> {}
impl Reg128BitsDownCast<50> for Reg128Bits<75> {}
impl Reg128BitsConcat<50, 125> for Reg128Bits<75> {}
impl Reg128BitsDownCast<51> for Reg128Bits<75> {}
impl Reg128BitsConcat<51, 126> for Reg128Bits<75> {}
impl Reg128BitsDownCast<52> for Reg128Bits<75> {}
impl Reg128BitsConcat<52, 127> for Reg128Bits<75> {}
impl Reg128BitsDownCast<53> for Reg128Bits<75> {}
impl Reg128BitsConcat<53, 128> for Reg128Bits<75> {}
impl Reg128BitsDownCast<54> for Reg128Bits<75> {}
impl Reg128BitsDownCast<55> for Reg128Bits<75> {}
impl Reg128BitsDownCast<56> for Reg128Bits<75> {}
impl Reg128BitsDownCast<57> for Reg128Bits<75> {}
impl Reg128BitsDownCast<58> for Reg128Bits<75> {}
impl Reg128BitsDownCast<59> for Reg128Bits<75> {}
impl Reg128BitsDownCast<60> for Reg128Bits<75> {}
impl Reg128BitsDownCast<61> for Reg128Bits<75> {}
impl Reg128BitsDownCast<62> for Reg128Bits<75> {}
impl Reg128BitsDownCast<63> for Reg128Bits<75> {}
impl Reg128BitsDownCast<64> for Reg128Bits<75> {}
impl Reg128BitsDownCast<65> for Reg128Bits<75> {}
impl Reg128BitsDownCast<66> for Reg128Bits<75> {}
impl Reg128BitsDownCast<67> for Reg128Bits<75> {}
impl Reg128BitsDownCast<68> for Reg128Bits<75> {}
impl Reg128BitsDownCast<69> for Reg128Bits<75> {}
impl Reg128BitsDownCast<70> for Reg128Bits<75> {}
impl Reg128BitsDownCast<71> for Reg128Bits<75> {}
impl Reg128BitsDownCast<72> for Reg128Bits<75> {}
impl Reg128BitsDownCast<73> for Reg128Bits<75> {}
impl Reg128BitsDownCast<74> for Reg128Bits<75> {}
impl Reg128BitsDownCast<75> for Reg128Bits<75> {}
impl Reg128BitsDownCast<1> for Reg128Bits<76> {}
impl Reg128BitsConcat<1, 77> for Reg128Bits<76> {}
impl Reg128BitsDownCast<2> for Reg128Bits<76> {}
impl Reg128BitsConcat<2, 78> for Reg128Bits<76> {}
impl Reg128BitsDownCast<3> for Reg128Bits<76> {}
impl Reg128BitsConcat<3, 79> for Reg128Bits<76> {}
impl Reg128BitsDownCast<4> for Reg128Bits<76> {}
impl Reg128BitsConcat<4, 80> for Reg128Bits<76> {}
impl Reg128BitsDownCast<5> for Reg128Bits<76> {}
impl Reg128BitsConcat<5, 81> for Reg128Bits<76> {}
impl Reg128BitsDownCast<6> for Reg128Bits<76> {}
impl Reg128BitsConcat<6, 82> for Reg128Bits<76> {}
impl Reg128BitsDownCast<7> for Reg128Bits<76> {}
impl Reg128BitsConcat<7, 83> for Reg128Bits<76> {}
impl Reg128BitsDownCast<8> for Reg128Bits<76> {}
impl Reg128BitsConcat<8, 84> for Reg128Bits<76> {}
impl Reg128BitsDownCast<9> for Reg128Bits<76> {}
impl Reg128BitsConcat<9, 85> for Reg128Bits<76> {}
impl Reg128BitsDownCast<10> for Reg128Bits<76> {}
impl Reg128BitsConcat<10, 86> for Reg128Bits<76> {}
impl Reg128BitsDownCast<11> for Reg128Bits<76> {}
impl Reg128BitsConcat<11, 87> for Reg128Bits<76> {}
impl Reg128BitsDownCast<12> for Reg128Bits<76> {}
impl Reg128BitsConcat<12, 88> for Reg128Bits<76> {}
impl Reg128BitsDownCast<13> for Reg128Bits<76> {}
impl Reg128BitsConcat<13, 89> for Reg128Bits<76> {}
impl Reg128BitsDownCast<14> for Reg128Bits<76> {}
impl Reg128BitsConcat<14, 90> for Reg128Bits<76> {}
impl Reg128BitsDownCast<15> for Reg128Bits<76> {}
impl Reg128BitsConcat<15, 91> for Reg128Bits<76> {}
impl Reg128BitsDownCast<16> for Reg128Bits<76> {}
impl Reg128BitsConcat<16, 92> for Reg128Bits<76> {}
impl Reg128BitsDownCast<17> for Reg128Bits<76> {}
impl Reg128BitsConcat<17, 93> for Reg128Bits<76> {}
impl Reg128BitsDownCast<18> for Reg128Bits<76> {}
impl Reg128BitsConcat<18, 94> for Reg128Bits<76> {}
impl Reg128BitsDownCast<19> for Reg128Bits<76> {}
impl Reg128BitsConcat<19, 95> for Reg128Bits<76> {}
impl Reg128BitsDownCast<20> for Reg128Bits<76> {}
impl Reg128BitsConcat<20, 96> for Reg128Bits<76> {}
impl Reg128BitsDownCast<21> for Reg128Bits<76> {}
impl Reg128BitsConcat<21, 97> for Reg128Bits<76> {}
impl Reg128BitsDownCast<22> for Reg128Bits<76> {}
impl Reg128BitsConcat<22, 98> for Reg128Bits<76> {}
impl Reg128BitsDownCast<23> for Reg128Bits<76> {}
impl Reg128BitsConcat<23, 99> for Reg128Bits<76> {}
impl Reg128BitsDownCast<24> for Reg128Bits<76> {}
impl Reg128BitsConcat<24, 100> for Reg128Bits<76> {}
impl Reg128BitsDownCast<25> for Reg128Bits<76> {}
impl Reg128BitsConcat<25, 101> for Reg128Bits<76> {}
impl Reg128BitsDownCast<26> for Reg128Bits<76> {}
impl Reg128BitsConcat<26, 102> for Reg128Bits<76> {}
impl Reg128BitsDownCast<27> for Reg128Bits<76> {}
impl Reg128BitsConcat<27, 103> for Reg128Bits<76> {}
impl Reg128BitsDownCast<28> for Reg128Bits<76> {}
impl Reg128BitsConcat<28, 104> for Reg128Bits<76> {}
impl Reg128BitsDownCast<29> for Reg128Bits<76> {}
impl Reg128BitsConcat<29, 105> for Reg128Bits<76> {}
impl Reg128BitsDownCast<30> for Reg128Bits<76> {}
impl Reg128BitsConcat<30, 106> for Reg128Bits<76> {}
impl Reg128BitsDownCast<31> for Reg128Bits<76> {}
impl Reg128BitsConcat<31, 107> for Reg128Bits<76> {}
impl Reg128BitsDownCast<32> for Reg128Bits<76> {}
impl Reg128BitsConcat<32, 108> for Reg128Bits<76> {}
impl Reg128BitsDownCast<33> for Reg128Bits<76> {}
impl Reg128BitsConcat<33, 109> for Reg128Bits<76> {}
impl Reg128BitsDownCast<34> for Reg128Bits<76> {}
impl Reg128BitsConcat<34, 110> for Reg128Bits<76> {}
impl Reg128BitsDownCast<35> for Reg128Bits<76> {}
impl Reg128BitsConcat<35, 111> for Reg128Bits<76> {}
impl Reg128BitsDownCast<36> for Reg128Bits<76> {}
impl Reg128BitsConcat<36, 112> for Reg128Bits<76> {}
impl Reg128BitsDownCast<37> for Reg128Bits<76> {}
impl Reg128BitsConcat<37, 113> for Reg128Bits<76> {}
impl Reg128BitsDownCast<38> for Reg128Bits<76> {}
impl Reg128BitsConcat<38, 114> for Reg128Bits<76> {}
impl Reg128BitsDownCast<39> for Reg128Bits<76> {}
impl Reg128BitsConcat<39, 115> for Reg128Bits<76> {}
impl Reg128BitsDownCast<40> for Reg128Bits<76> {}
impl Reg128BitsConcat<40, 116> for Reg128Bits<76> {}
impl Reg128BitsDownCast<41> for Reg128Bits<76> {}
impl Reg128BitsConcat<41, 117> for Reg128Bits<76> {}
impl Reg128BitsDownCast<42> for Reg128Bits<76> {}
impl Reg128BitsConcat<42, 118> for Reg128Bits<76> {}
impl Reg128BitsDownCast<43> for Reg128Bits<76> {}
impl Reg128BitsConcat<43, 119> for Reg128Bits<76> {}
impl Reg128BitsDownCast<44> for Reg128Bits<76> {}
impl Reg128BitsConcat<44, 120> for Reg128Bits<76> {}
impl Reg128BitsDownCast<45> for Reg128Bits<76> {}
impl Reg128BitsConcat<45, 121> for Reg128Bits<76> {}
impl Reg128BitsDownCast<46> for Reg128Bits<76> {}
impl Reg128BitsConcat<46, 122> for Reg128Bits<76> {}
impl Reg128BitsDownCast<47> for Reg128Bits<76> {}
impl Reg128BitsConcat<47, 123> for Reg128Bits<76> {}
impl Reg128BitsDownCast<48> for Reg128Bits<76> {}
impl Reg128BitsConcat<48, 124> for Reg128Bits<76> {}
impl Reg128BitsDownCast<49> for Reg128Bits<76> {}
impl Reg128BitsConcat<49, 125> for Reg128Bits<76> {}
impl Reg128BitsDownCast<50> for Reg128Bits<76> {}
impl Reg128BitsConcat<50, 126> for Reg128Bits<76> {}
impl Reg128BitsDownCast<51> for Reg128Bits<76> {}
impl Reg128BitsConcat<51, 127> for Reg128Bits<76> {}
impl Reg128BitsDownCast<52> for Reg128Bits<76> {}
impl Reg128BitsConcat<52, 128> for Reg128Bits<76> {}
impl Reg128BitsDownCast<53> for Reg128Bits<76> {}
impl Reg128BitsDownCast<54> for Reg128Bits<76> {}
impl Reg128BitsDownCast<55> for Reg128Bits<76> {}
impl Reg128BitsDownCast<56> for Reg128Bits<76> {}
impl Reg128BitsDownCast<57> for Reg128Bits<76> {}
impl Reg128BitsDownCast<58> for Reg128Bits<76> {}
impl Reg128BitsDownCast<59> for Reg128Bits<76> {}
impl Reg128BitsDownCast<60> for Reg128Bits<76> {}
impl Reg128BitsDownCast<61> for Reg128Bits<76> {}
impl Reg128BitsDownCast<62> for Reg128Bits<76> {}
impl Reg128BitsDownCast<63> for Reg128Bits<76> {}
impl Reg128BitsDownCast<64> for Reg128Bits<76> {}
impl Reg128BitsDownCast<65> for Reg128Bits<76> {}
impl Reg128BitsDownCast<66> for Reg128Bits<76> {}
impl Reg128BitsDownCast<67> for Reg128Bits<76> {}
impl Reg128BitsDownCast<68> for Reg128Bits<76> {}
impl Reg128BitsDownCast<69> for Reg128Bits<76> {}
impl Reg128BitsDownCast<70> for Reg128Bits<76> {}
impl Reg128BitsDownCast<71> for Reg128Bits<76> {}
impl Reg128BitsDownCast<72> for Reg128Bits<76> {}
impl Reg128BitsDownCast<73> for Reg128Bits<76> {}
impl Reg128BitsDownCast<74> for Reg128Bits<76> {}
impl Reg128BitsDownCast<75> for Reg128Bits<76> {}
impl Reg128BitsDownCast<76> for Reg128Bits<76> {}
impl Reg128BitsDownCast<1> for Reg128Bits<77> {}
impl Reg128BitsConcat<1, 78> for Reg128Bits<77> {}
impl Reg128BitsDownCast<2> for Reg128Bits<77> {}
impl Reg128BitsConcat<2, 79> for Reg128Bits<77> {}
impl Reg128BitsDownCast<3> for Reg128Bits<77> {}
impl Reg128BitsConcat<3, 80> for Reg128Bits<77> {}
impl Reg128BitsDownCast<4> for Reg128Bits<77> {}
impl Reg128BitsConcat<4, 81> for Reg128Bits<77> {}
impl Reg128BitsDownCast<5> for Reg128Bits<77> {}
impl Reg128BitsConcat<5, 82> for Reg128Bits<77> {}
impl Reg128BitsDownCast<6> for Reg128Bits<77> {}
impl Reg128BitsConcat<6, 83> for Reg128Bits<77> {}
impl Reg128BitsDownCast<7> for Reg128Bits<77> {}
impl Reg128BitsConcat<7, 84> for Reg128Bits<77> {}
impl Reg128BitsDownCast<8> for Reg128Bits<77> {}
impl Reg128BitsConcat<8, 85> for Reg128Bits<77> {}
impl Reg128BitsDownCast<9> for Reg128Bits<77> {}
impl Reg128BitsConcat<9, 86> for Reg128Bits<77> {}
impl Reg128BitsDownCast<10> for Reg128Bits<77> {}
impl Reg128BitsConcat<10, 87> for Reg128Bits<77> {}
impl Reg128BitsDownCast<11> for Reg128Bits<77> {}
impl Reg128BitsConcat<11, 88> for Reg128Bits<77> {}
impl Reg128BitsDownCast<12> for Reg128Bits<77> {}
impl Reg128BitsConcat<12, 89> for Reg128Bits<77> {}
impl Reg128BitsDownCast<13> for Reg128Bits<77> {}
impl Reg128BitsConcat<13, 90> for Reg128Bits<77> {}
impl Reg128BitsDownCast<14> for Reg128Bits<77> {}
impl Reg128BitsConcat<14, 91> for Reg128Bits<77> {}
impl Reg128BitsDownCast<15> for Reg128Bits<77> {}
impl Reg128BitsConcat<15, 92> for Reg128Bits<77> {}
impl Reg128BitsDownCast<16> for Reg128Bits<77> {}
impl Reg128BitsConcat<16, 93> for Reg128Bits<77> {}
impl Reg128BitsDownCast<17> for Reg128Bits<77> {}
impl Reg128BitsConcat<17, 94> for Reg128Bits<77> {}
impl Reg128BitsDownCast<18> for Reg128Bits<77> {}
impl Reg128BitsConcat<18, 95> for Reg128Bits<77> {}
impl Reg128BitsDownCast<19> for Reg128Bits<77> {}
impl Reg128BitsConcat<19, 96> for Reg128Bits<77> {}
impl Reg128BitsDownCast<20> for Reg128Bits<77> {}
impl Reg128BitsConcat<20, 97> for Reg128Bits<77> {}
impl Reg128BitsDownCast<21> for Reg128Bits<77> {}
impl Reg128BitsConcat<21, 98> for Reg128Bits<77> {}
impl Reg128BitsDownCast<22> for Reg128Bits<77> {}
impl Reg128BitsConcat<22, 99> for Reg128Bits<77> {}
impl Reg128BitsDownCast<23> for Reg128Bits<77> {}
impl Reg128BitsConcat<23, 100> for Reg128Bits<77> {}
impl Reg128BitsDownCast<24> for Reg128Bits<77> {}
impl Reg128BitsConcat<24, 101> for Reg128Bits<77> {}
impl Reg128BitsDownCast<25> for Reg128Bits<77> {}
impl Reg128BitsConcat<25, 102> for Reg128Bits<77> {}
impl Reg128BitsDownCast<26> for Reg128Bits<77> {}
impl Reg128BitsConcat<26, 103> for Reg128Bits<77> {}
impl Reg128BitsDownCast<27> for Reg128Bits<77> {}
impl Reg128BitsConcat<27, 104> for Reg128Bits<77> {}
impl Reg128BitsDownCast<28> for Reg128Bits<77> {}
impl Reg128BitsConcat<28, 105> for Reg128Bits<77> {}
impl Reg128BitsDownCast<29> for Reg128Bits<77> {}
impl Reg128BitsConcat<29, 106> for Reg128Bits<77> {}
impl Reg128BitsDownCast<30> for Reg128Bits<77> {}
impl Reg128BitsConcat<30, 107> for Reg128Bits<77> {}
impl Reg128BitsDownCast<31> for Reg128Bits<77> {}
impl Reg128BitsConcat<31, 108> for Reg128Bits<77> {}
impl Reg128BitsDownCast<32> for Reg128Bits<77> {}
impl Reg128BitsConcat<32, 109> for Reg128Bits<77> {}
impl Reg128BitsDownCast<33> for Reg128Bits<77> {}
impl Reg128BitsConcat<33, 110> for Reg128Bits<77> {}
impl Reg128BitsDownCast<34> for Reg128Bits<77> {}
impl Reg128BitsConcat<34, 111> for Reg128Bits<77> {}
impl Reg128BitsDownCast<35> for Reg128Bits<77> {}
impl Reg128BitsConcat<35, 112> for Reg128Bits<77> {}
impl Reg128BitsDownCast<36> for Reg128Bits<77> {}
impl Reg128BitsConcat<36, 113> for Reg128Bits<77> {}
impl Reg128BitsDownCast<37> for Reg128Bits<77> {}
impl Reg128BitsConcat<37, 114> for Reg128Bits<77> {}
impl Reg128BitsDownCast<38> for Reg128Bits<77> {}
impl Reg128BitsConcat<38, 115> for Reg128Bits<77> {}
impl Reg128BitsDownCast<39> for Reg128Bits<77> {}
impl Reg128BitsConcat<39, 116> for Reg128Bits<77> {}
impl Reg128BitsDownCast<40> for Reg128Bits<77> {}
impl Reg128BitsConcat<40, 117> for Reg128Bits<77> {}
impl Reg128BitsDownCast<41> for Reg128Bits<77> {}
impl Reg128BitsConcat<41, 118> for Reg128Bits<77> {}
impl Reg128BitsDownCast<42> for Reg128Bits<77> {}
impl Reg128BitsConcat<42, 119> for Reg128Bits<77> {}
impl Reg128BitsDownCast<43> for Reg128Bits<77> {}
impl Reg128BitsConcat<43, 120> for Reg128Bits<77> {}
impl Reg128BitsDownCast<44> for Reg128Bits<77> {}
impl Reg128BitsConcat<44, 121> for Reg128Bits<77> {}
impl Reg128BitsDownCast<45> for Reg128Bits<77> {}
impl Reg128BitsConcat<45, 122> for Reg128Bits<77> {}
impl Reg128BitsDownCast<46> for Reg128Bits<77> {}
impl Reg128BitsConcat<46, 123> for Reg128Bits<77> {}
impl Reg128BitsDownCast<47> for Reg128Bits<77> {}
impl Reg128BitsConcat<47, 124> for Reg128Bits<77> {}
impl Reg128BitsDownCast<48> for Reg128Bits<77> {}
impl Reg128BitsConcat<48, 125> for Reg128Bits<77> {}
impl Reg128BitsDownCast<49> for Reg128Bits<77> {}
impl Reg128BitsConcat<49, 126> for Reg128Bits<77> {}
impl Reg128BitsDownCast<50> for Reg128Bits<77> {}
impl Reg128BitsConcat<50, 127> for Reg128Bits<77> {}
impl Reg128BitsDownCast<51> for Reg128Bits<77> {}
impl Reg128BitsConcat<51, 128> for Reg128Bits<77> {}
impl Reg128BitsDownCast<52> for Reg128Bits<77> {}
impl Reg128BitsDownCast<53> for Reg128Bits<77> {}
impl Reg128BitsDownCast<54> for Reg128Bits<77> {}
impl Reg128BitsDownCast<55> for Reg128Bits<77> {}
impl Reg128BitsDownCast<56> for Reg128Bits<77> {}
impl Reg128BitsDownCast<57> for Reg128Bits<77> {}
impl Reg128BitsDownCast<58> for Reg128Bits<77> {}
impl Reg128BitsDownCast<59> for Reg128Bits<77> {}
impl Reg128BitsDownCast<60> for Reg128Bits<77> {}
impl Reg128BitsDownCast<61> for Reg128Bits<77> {}
impl Reg128BitsDownCast<62> for Reg128Bits<77> {}
impl Reg128BitsDownCast<63> for Reg128Bits<77> {}
impl Reg128BitsDownCast<64> for Reg128Bits<77> {}
impl Reg128BitsDownCast<65> for Reg128Bits<77> {}
impl Reg128BitsDownCast<66> for Reg128Bits<77> {}
impl Reg128BitsDownCast<67> for Reg128Bits<77> {}
impl Reg128BitsDownCast<68> for Reg128Bits<77> {}
impl Reg128BitsDownCast<69> for Reg128Bits<77> {}
impl Reg128BitsDownCast<70> for Reg128Bits<77> {}
impl Reg128BitsDownCast<71> for Reg128Bits<77> {}
impl Reg128BitsDownCast<72> for Reg128Bits<77> {}
impl Reg128BitsDownCast<73> for Reg128Bits<77> {}
impl Reg128BitsDownCast<74> for Reg128Bits<77> {}
impl Reg128BitsDownCast<75> for Reg128Bits<77> {}
impl Reg128BitsDownCast<76> for Reg128Bits<77> {}
impl Reg128BitsDownCast<77> for Reg128Bits<77> {}
impl Reg128BitsDownCast<1> for Reg128Bits<78> {}
impl Reg128BitsConcat<1, 79> for Reg128Bits<78> {}
impl Reg128BitsDownCast<2> for Reg128Bits<78> {}
impl Reg128BitsConcat<2, 80> for Reg128Bits<78> {}
impl Reg128BitsDownCast<3> for Reg128Bits<78> {}
impl Reg128BitsConcat<3, 81> for Reg128Bits<78> {}
impl Reg128BitsDownCast<4> for Reg128Bits<78> {}
impl Reg128BitsConcat<4, 82> for Reg128Bits<78> {}
impl Reg128BitsDownCast<5> for Reg128Bits<78> {}
impl Reg128BitsConcat<5, 83> for Reg128Bits<78> {}
impl Reg128BitsDownCast<6> for Reg128Bits<78> {}
impl Reg128BitsConcat<6, 84> for Reg128Bits<78> {}
impl Reg128BitsDownCast<7> for Reg128Bits<78> {}
impl Reg128BitsConcat<7, 85> for Reg128Bits<78> {}
impl Reg128BitsDownCast<8> for Reg128Bits<78> {}
impl Reg128BitsConcat<8, 86> for Reg128Bits<78> {}
impl Reg128BitsDownCast<9> for Reg128Bits<78> {}
impl Reg128BitsConcat<9, 87> for Reg128Bits<78> {}
impl Reg128BitsDownCast<10> for Reg128Bits<78> {}
impl Reg128BitsConcat<10, 88> for Reg128Bits<78> {}
impl Reg128BitsDownCast<11> for Reg128Bits<78> {}
impl Reg128BitsConcat<11, 89> for Reg128Bits<78> {}
impl Reg128BitsDownCast<12> for Reg128Bits<78> {}
impl Reg128BitsConcat<12, 90> for Reg128Bits<78> {}
impl Reg128BitsDownCast<13> for Reg128Bits<78> {}
impl Reg128BitsConcat<13, 91> for Reg128Bits<78> {}
impl Reg128BitsDownCast<14> for Reg128Bits<78> {}
impl Reg128BitsConcat<14, 92> for Reg128Bits<78> {}
impl Reg128BitsDownCast<15> for Reg128Bits<78> {}
impl Reg128BitsConcat<15, 93> for Reg128Bits<78> {}
impl Reg128BitsDownCast<16> for Reg128Bits<78> {}
impl Reg128BitsConcat<16, 94> for Reg128Bits<78> {}
impl Reg128BitsDownCast<17> for Reg128Bits<78> {}
impl Reg128BitsConcat<17, 95> for Reg128Bits<78> {}
impl Reg128BitsDownCast<18> for Reg128Bits<78> {}
impl Reg128BitsConcat<18, 96> for Reg128Bits<78> {}
impl Reg128BitsDownCast<19> for Reg128Bits<78> {}
impl Reg128BitsConcat<19, 97> for Reg128Bits<78> {}
impl Reg128BitsDownCast<20> for Reg128Bits<78> {}
impl Reg128BitsConcat<20, 98> for Reg128Bits<78> {}
impl Reg128BitsDownCast<21> for Reg128Bits<78> {}
impl Reg128BitsConcat<21, 99> for Reg128Bits<78> {}
impl Reg128BitsDownCast<22> for Reg128Bits<78> {}
impl Reg128BitsConcat<22, 100> for Reg128Bits<78> {}
impl Reg128BitsDownCast<23> for Reg128Bits<78> {}
impl Reg128BitsConcat<23, 101> for Reg128Bits<78> {}
impl Reg128BitsDownCast<24> for Reg128Bits<78> {}
impl Reg128BitsConcat<24, 102> for Reg128Bits<78> {}
impl Reg128BitsDownCast<25> for Reg128Bits<78> {}
impl Reg128BitsConcat<25, 103> for Reg128Bits<78> {}
impl Reg128BitsDownCast<26> for Reg128Bits<78> {}
impl Reg128BitsConcat<26, 104> for Reg128Bits<78> {}
impl Reg128BitsDownCast<27> for Reg128Bits<78> {}
impl Reg128BitsConcat<27, 105> for Reg128Bits<78> {}
impl Reg128BitsDownCast<28> for Reg128Bits<78> {}
impl Reg128BitsConcat<28, 106> for Reg128Bits<78> {}
impl Reg128BitsDownCast<29> for Reg128Bits<78> {}
impl Reg128BitsConcat<29, 107> for Reg128Bits<78> {}
impl Reg128BitsDownCast<30> for Reg128Bits<78> {}
impl Reg128BitsConcat<30, 108> for Reg128Bits<78> {}
impl Reg128BitsDownCast<31> for Reg128Bits<78> {}
impl Reg128BitsConcat<31, 109> for Reg128Bits<78> {}
impl Reg128BitsDownCast<32> for Reg128Bits<78> {}
impl Reg128BitsConcat<32, 110> for Reg128Bits<78> {}
impl Reg128BitsDownCast<33> for Reg128Bits<78> {}
impl Reg128BitsConcat<33, 111> for Reg128Bits<78> {}
impl Reg128BitsDownCast<34> for Reg128Bits<78> {}
impl Reg128BitsConcat<34, 112> for Reg128Bits<78> {}
impl Reg128BitsDownCast<35> for Reg128Bits<78> {}
impl Reg128BitsConcat<35, 113> for Reg128Bits<78> {}
impl Reg128BitsDownCast<36> for Reg128Bits<78> {}
impl Reg128BitsConcat<36, 114> for Reg128Bits<78> {}
impl Reg128BitsDownCast<37> for Reg128Bits<78> {}
impl Reg128BitsConcat<37, 115> for Reg128Bits<78> {}
impl Reg128BitsDownCast<38> for Reg128Bits<78> {}
impl Reg128BitsConcat<38, 116> for Reg128Bits<78> {}
impl Reg128BitsDownCast<39> for Reg128Bits<78> {}
impl Reg128BitsConcat<39, 117> for Reg128Bits<78> {}
impl Reg128BitsDownCast<40> for Reg128Bits<78> {}
impl Reg128BitsConcat<40, 118> for Reg128Bits<78> {}
impl Reg128BitsDownCast<41> for Reg128Bits<78> {}
impl Reg128BitsConcat<41, 119> for Reg128Bits<78> {}
impl Reg128BitsDownCast<42> for Reg128Bits<78> {}
impl Reg128BitsConcat<42, 120> for Reg128Bits<78> {}
impl Reg128BitsDownCast<43> for Reg128Bits<78> {}
impl Reg128BitsConcat<43, 121> for Reg128Bits<78> {}
impl Reg128BitsDownCast<44> for Reg128Bits<78> {}
impl Reg128BitsConcat<44, 122> for Reg128Bits<78> {}
impl Reg128BitsDownCast<45> for Reg128Bits<78> {}
impl Reg128BitsConcat<45, 123> for Reg128Bits<78> {}
impl Reg128BitsDownCast<46> for Reg128Bits<78> {}
impl Reg128BitsConcat<46, 124> for Reg128Bits<78> {}
impl Reg128BitsDownCast<47> for Reg128Bits<78> {}
impl Reg128BitsConcat<47, 125> for Reg128Bits<78> {}
impl Reg128BitsDownCast<48> for Reg128Bits<78> {}
impl Reg128BitsConcat<48, 126> for Reg128Bits<78> {}
impl Reg128BitsDownCast<49> for Reg128Bits<78> {}
impl Reg128BitsConcat<49, 127> for Reg128Bits<78> {}
impl Reg128BitsDownCast<50> for Reg128Bits<78> {}
impl Reg128BitsConcat<50, 128> for Reg128Bits<78> {}
impl Reg128BitsDownCast<51> for Reg128Bits<78> {}
impl Reg128BitsDownCast<52> for Reg128Bits<78> {}
impl Reg128BitsDownCast<53> for Reg128Bits<78> {}
impl Reg128BitsDownCast<54> for Reg128Bits<78> {}
impl Reg128BitsDownCast<55> for Reg128Bits<78> {}
impl Reg128BitsDownCast<56> for Reg128Bits<78> {}
impl Reg128BitsDownCast<57> for Reg128Bits<78> {}
impl Reg128BitsDownCast<58> for Reg128Bits<78> {}
impl Reg128BitsDownCast<59> for Reg128Bits<78> {}
impl Reg128BitsDownCast<60> for Reg128Bits<78> {}
impl Reg128BitsDownCast<61> for Reg128Bits<78> {}
impl Reg128BitsDownCast<62> for Reg128Bits<78> {}
impl Reg128BitsDownCast<63> for Reg128Bits<78> {}
impl Reg128BitsDownCast<64> for Reg128Bits<78> {}
impl Reg128BitsDownCast<65> for Reg128Bits<78> {}
impl Reg128BitsDownCast<66> for Reg128Bits<78> {}
impl Reg128BitsDownCast<67> for Reg128Bits<78> {}
impl Reg128BitsDownCast<68> for Reg128Bits<78> {}
impl Reg128BitsDownCast<69> for Reg128Bits<78> {}
impl Reg128BitsDownCast<70> for Reg128Bits<78> {}
impl Reg128BitsDownCast<71> for Reg128Bits<78> {}
impl Reg128BitsDownCast<72> for Reg128Bits<78> {}
impl Reg128BitsDownCast<73> for Reg128Bits<78> {}
impl Reg128BitsDownCast<74> for Reg128Bits<78> {}
impl Reg128BitsDownCast<75> for Reg128Bits<78> {}
impl Reg128BitsDownCast<76> for Reg128Bits<78> {}
impl Reg128BitsDownCast<77> for Reg128Bits<78> {}
impl Reg128BitsDownCast<78> for Reg128Bits<78> {}
impl Reg128BitsDownCast<1> for Reg128Bits<79> {}
impl Reg128BitsConcat<1, 80> for Reg128Bits<79> {}
impl Reg128BitsDownCast<2> for Reg128Bits<79> {}
impl Reg128BitsConcat<2, 81> for Reg128Bits<79> {}
impl Reg128BitsDownCast<3> for Reg128Bits<79> {}
impl Reg128BitsConcat<3, 82> for Reg128Bits<79> {}
impl Reg128BitsDownCast<4> for Reg128Bits<79> {}
impl Reg128BitsConcat<4, 83> for Reg128Bits<79> {}
impl Reg128BitsDownCast<5> for Reg128Bits<79> {}
impl Reg128BitsConcat<5, 84> for Reg128Bits<79> {}
impl Reg128BitsDownCast<6> for Reg128Bits<79> {}
impl Reg128BitsConcat<6, 85> for Reg128Bits<79> {}
impl Reg128BitsDownCast<7> for Reg128Bits<79> {}
impl Reg128BitsConcat<7, 86> for Reg128Bits<79> {}
impl Reg128BitsDownCast<8> for Reg128Bits<79> {}
impl Reg128BitsConcat<8, 87> for Reg128Bits<79> {}
impl Reg128BitsDownCast<9> for Reg128Bits<79> {}
impl Reg128BitsConcat<9, 88> for Reg128Bits<79> {}
impl Reg128BitsDownCast<10> for Reg128Bits<79> {}
impl Reg128BitsConcat<10, 89> for Reg128Bits<79> {}
impl Reg128BitsDownCast<11> for Reg128Bits<79> {}
impl Reg128BitsConcat<11, 90> for Reg128Bits<79> {}
impl Reg128BitsDownCast<12> for Reg128Bits<79> {}
impl Reg128BitsConcat<12, 91> for Reg128Bits<79> {}
impl Reg128BitsDownCast<13> for Reg128Bits<79> {}
impl Reg128BitsConcat<13, 92> for Reg128Bits<79> {}
impl Reg128BitsDownCast<14> for Reg128Bits<79> {}
impl Reg128BitsConcat<14, 93> for Reg128Bits<79> {}
impl Reg128BitsDownCast<15> for Reg128Bits<79> {}
impl Reg128BitsConcat<15, 94> for Reg128Bits<79> {}
impl Reg128BitsDownCast<16> for Reg128Bits<79> {}
impl Reg128BitsConcat<16, 95> for Reg128Bits<79> {}
impl Reg128BitsDownCast<17> for Reg128Bits<79> {}
impl Reg128BitsConcat<17, 96> for Reg128Bits<79> {}
impl Reg128BitsDownCast<18> for Reg128Bits<79> {}
impl Reg128BitsConcat<18, 97> for Reg128Bits<79> {}
impl Reg128BitsDownCast<19> for Reg128Bits<79> {}
impl Reg128BitsConcat<19, 98> for Reg128Bits<79> {}
impl Reg128BitsDownCast<20> for Reg128Bits<79> {}
impl Reg128BitsConcat<20, 99> for Reg128Bits<79> {}
impl Reg128BitsDownCast<21> for Reg128Bits<79> {}
impl Reg128BitsConcat<21, 100> for Reg128Bits<79> {}
impl Reg128BitsDownCast<22> for Reg128Bits<79> {}
impl Reg128BitsConcat<22, 101> for Reg128Bits<79> {}
impl Reg128BitsDownCast<23> for Reg128Bits<79> {}
impl Reg128BitsConcat<23, 102> for Reg128Bits<79> {}
impl Reg128BitsDownCast<24> for Reg128Bits<79> {}
impl Reg128BitsConcat<24, 103> for Reg128Bits<79> {}
impl Reg128BitsDownCast<25> for Reg128Bits<79> {}
impl Reg128BitsConcat<25, 104> for Reg128Bits<79> {}
impl Reg128BitsDownCast<26> for Reg128Bits<79> {}
impl Reg128BitsConcat<26, 105> for Reg128Bits<79> {}
impl Reg128BitsDownCast<27> for Reg128Bits<79> {}
impl Reg128BitsConcat<27, 106> for Reg128Bits<79> {}
impl Reg128BitsDownCast<28> for Reg128Bits<79> {}
impl Reg128BitsConcat<28, 107> for Reg128Bits<79> {}
impl Reg128BitsDownCast<29> for Reg128Bits<79> {}
impl Reg128BitsConcat<29, 108> for Reg128Bits<79> {}
impl Reg128BitsDownCast<30> for Reg128Bits<79> {}
impl Reg128BitsConcat<30, 109> for Reg128Bits<79> {}
impl Reg128BitsDownCast<31> for Reg128Bits<79> {}
impl Reg128BitsConcat<31, 110> for Reg128Bits<79> {}
impl Reg128BitsDownCast<32> for Reg128Bits<79> {}
impl Reg128BitsConcat<32, 111> for Reg128Bits<79> {}
impl Reg128BitsDownCast<33> for Reg128Bits<79> {}
impl Reg128BitsConcat<33, 112> for Reg128Bits<79> {}
impl Reg128BitsDownCast<34> for Reg128Bits<79> {}
impl Reg128BitsConcat<34, 113> for Reg128Bits<79> {}
impl Reg128BitsDownCast<35> for Reg128Bits<79> {}
impl Reg128BitsConcat<35, 114> for Reg128Bits<79> {}
impl Reg128BitsDownCast<36> for Reg128Bits<79> {}
impl Reg128BitsConcat<36, 115> for Reg128Bits<79> {}
impl Reg128BitsDownCast<37> for Reg128Bits<79> {}
impl Reg128BitsConcat<37, 116> for Reg128Bits<79> {}
impl Reg128BitsDownCast<38> for Reg128Bits<79> {}
impl Reg128BitsConcat<38, 117> for Reg128Bits<79> {}
impl Reg128BitsDownCast<39> for Reg128Bits<79> {}
impl Reg128BitsConcat<39, 118> for Reg128Bits<79> {}
impl Reg128BitsDownCast<40> for Reg128Bits<79> {}
impl Reg128BitsConcat<40, 119> for Reg128Bits<79> {}
impl Reg128BitsDownCast<41> for Reg128Bits<79> {}
impl Reg128BitsConcat<41, 120> for Reg128Bits<79> {}
impl Reg128BitsDownCast<42> for Reg128Bits<79> {}
impl Reg128BitsConcat<42, 121> for Reg128Bits<79> {}
impl Reg128BitsDownCast<43> for Reg128Bits<79> {}
impl Reg128BitsConcat<43, 122> for Reg128Bits<79> {}
impl Reg128BitsDownCast<44> for Reg128Bits<79> {}
impl Reg128BitsConcat<44, 123> for Reg128Bits<79> {}
impl Reg128BitsDownCast<45> for Reg128Bits<79> {}
impl Reg128BitsConcat<45, 124> for Reg128Bits<79> {}
impl Reg128BitsDownCast<46> for Reg128Bits<79> {}
impl Reg128BitsConcat<46, 125> for Reg128Bits<79> {}
impl Reg128BitsDownCast<47> for Reg128Bits<79> {}
impl Reg128BitsConcat<47, 126> for Reg128Bits<79> {}
impl Reg128BitsDownCast<48> for Reg128Bits<79> {}
impl Reg128BitsConcat<48, 127> for Reg128Bits<79> {}
impl Reg128BitsDownCast<49> for Reg128Bits<79> {}
impl Reg128BitsConcat<49, 128> for Reg128Bits<79> {}
impl Reg128BitsDownCast<50> for Reg128Bits<79> {}
impl Reg128BitsDownCast<51> for Reg128Bits<79> {}
impl Reg128BitsDownCast<52> for Reg128Bits<79> {}
impl Reg128BitsDownCast<53> for Reg128Bits<79> {}
impl Reg128BitsDownCast<54> for Reg128Bits<79> {}
impl Reg128BitsDownCast<55> for Reg128Bits<79> {}
impl Reg128BitsDownCast<56> for Reg128Bits<79> {}
impl Reg128BitsDownCast<57> for Reg128Bits<79> {}
impl Reg128BitsDownCast<58> for Reg128Bits<79> {}
impl Reg128BitsDownCast<59> for Reg128Bits<79> {}
impl Reg128BitsDownCast<60> for Reg128Bits<79> {}
impl Reg128BitsDownCast<61> for Reg128Bits<79> {}
impl Reg128BitsDownCast<62> for Reg128Bits<79> {}
impl Reg128BitsDownCast<63> for Reg128Bits<79> {}
impl Reg128BitsDownCast<64> for Reg128Bits<79> {}
impl Reg128BitsDownCast<65> for Reg128Bits<79> {}
impl Reg128BitsDownCast<66> for Reg128Bits<79> {}
impl Reg128BitsDownCast<67> for Reg128Bits<79> {}
impl Reg128BitsDownCast<68> for Reg128Bits<79> {}
impl Reg128BitsDownCast<69> for Reg128Bits<79> {}
impl Reg128BitsDownCast<70> for Reg128Bits<79> {}
impl Reg128BitsDownCast<71> for Reg128Bits<79> {}
impl Reg128BitsDownCast<72> for Reg128Bits<79> {}
impl Reg128BitsDownCast<73> for Reg128Bits<79> {}
impl Reg128BitsDownCast<74> for Reg128Bits<79> {}
impl Reg128BitsDownCast<75> for Reg128Bits<79> {}
impl Reg128BitsDownCast<76> for Reg128Bits<79> {}
impl Reg128BitsDownCast<77> for Reg128Bits<79> {}
impl Reg128BitsDownCast<78> for Reg128Bits<79> {}
impl Reg128BitsDownCast<79> for Reg128Bits<79> {}
impl Reg128BitsDownCast<1> for Reg128Bits<80> {}
impl Reg128BitsConcat<1, 81> for Reg128Bits<80> {}
impl Reg128BitsDownCast<2> for Reg128Bits<80> {}
impl Reg128BitsConcat<2, 82> for Reg128Bits<80> {}
impl Reg128BitsDownCast<3> for Reg128Bits<80> {}
impl Reg128BitsConcat<3, 83> for Reg128Bits<80> {}
impl Reg128BitsDownCast<4> for Reg128Bits<80> {}
impl Reg128BitsConcat<4, 84> for Reg128Bits<80> {}
impl Reg128BitsDownCast<5> for Reg128Bits<80> {}
impl Reg128BitsConcat<5, 85> for Reg128Bits<80> {}
impl Reg128BitsDownCast<6> for Reg128Bits<80> {}
impl Reg128BitsConcat<6, 86> for Reg128Bits<80> {}
impl Reg128BitsDownCast<7> for Reg128Bits<80> {}
impl Reg128BitsConcat<7, 87> for Reg128Bits<80> {}
impl Reg128BitsDownCast<8> for Reg128Bits<80> {}
impl Reg128BitsConcat<8, 88> for Reg128Bits<80> {}
impl Reg128BitsDownCast<9> for Reg128Bits<80> {}
impl Reg128BitsConcat<9, 89> for Reg128Bits<80> {}
impl Reg128BitsDownCast<10> for Reg128Bits<80> {}
impl Reg128BitsConcat<10, 90> for Reg128Bits<80> {}
impl Reg128BitsDownCast<11> for Reg128Bits<80> {}
impl Reg128BitsConcat<11, 91> for Reg128Bits<80> {}
impl Reg128BitsDownCast<12> for Reg128Bits<80> {}
impl Reg128BitsConcat<12, 92> for Reg128Bits<80> {}
impl Reg128BitsDownCast<13> for Reg128Bits<80> {}
impl Reg128BitsConcat<13, 93> for Reg128Bits<80> {}
impl Reg128BitsDownCast<14> for Reg128Bits<80> {}
impl Reg128BitsConcat<14, 94> for Reg128Bits<80> {}
impl Reg128BitsDownCast<15> for Reg128Bits<80> {}
impl Reg128BitsConcat<15, 95> for Reg128Bits<80> {}
impl Reg128BitsDownCast<16> for Reg128Bits<80> {}
impl Reg128BitsConcat<16, 96> for Reg128Bits<80> {}
impl Reg128BitsDownCast<17> for Reg128Bits<80> {}
impl Reg128BitsConcat<17, 97> for Reg128Bits<80> {}
impl Reg128BitsDownCast<18> for Reg128Bits<80> {}
impl Reg128BitsConcat<18, 98> for Reg128Bits<80> {}
impl Reg128BitsDownCast<19> for Reg128Bits<80> {}
impl Reg128BitsConcat<19, 99> for Reg128Bits<80> {}
impl Reg128BitsDownCast<20> for Reg128Bits<80> {}
impl Reg128BitsConcat<20, 100> for Reg128Bits<80> {}
impl Reg128BitsDownCast<21> for Reg128Bits<80> {}
impl Reg128BitsConcat<21, 101> for Reg128Bits<80> {}
impl Reg128BitsDownCast<22> for Reg128Bits<80> {}
impl Reg128BitsConcat<22, 102> for Reg128Bits<80> {}
impl Reg128BitsDownCast<23> for Reg128Bits<80> {}
impl Reg128BitsConcat<23, 103> for Reg128Bits<80> {}
impl Reg128BitsDownCast<24> for Reg128Bits<80> {}
impl Reg128BitsConcat<24, 104> for Reg128Bits<80> {}
impl Reg128BitsDownCast<25> for Reg128Bits<80> {}
impl Reg128BitsConcat<25, 105> for Reg128Bits<80> {}
impl Reg128BitsDownCast<26> for Reg128Bits<80> {}
impl Reg128BitsConcat<26, 106> for Reg128Bits<80> {}
impl Reg128BitsDownCast<27> for Reg128Bits<80> {}
impl Reg128BitsConcat<27, 107> for Reg128Bits<80> {}
impl Reg128BitsDownCast<28> for Reg128Bits<80> {}
impl Reg128BitsConcat<28, 108> for Reg128Bits<80> {}
impl Reg128BitsDownCast<29> for Reg128Bits<80> {}
impl Reg128BitsConcat<29, 109> for Reg128Bits<80> {}
impl Reg128BitsDownCast<30> for Reg128Bits<80> {}
impl Reg128BitsConcat<30, 110> for Reg128Bits<80> {}
impl Reg128BitsDownCast<31> for Reg128Bits<80> {}
impl Reg128BitsConcat<31, 111> for Reg128Bits<80> {}
impl Reg128BitsDownCast<32> for Reg128Bits<80> {}
impl Reg128BitsConcat<32, 112> for Reg128Bits<80> {}
impl Reg128BitsDownCast<33> for Reg128Bits<80> {}
impl Reg128BitsConcat<33, 113> for Reg128Bits<80> {}
impl Reg128BitsDownCast<34> for Reg128Bits<80> {}
impl Reg128BitsConcat<34, 114> for Reg128Bits<80> {}
impl Reg128BitsDownCast<35> for Reg128Bits<80> {}
impl Reg128BitsConcat<35, 115> for Reg128Bits<80> {}
impl Reg128BitsDownCast<36> for Reg128Bits<80> {}
impl Reg128BitsConcat<36, 116> for Reg128Bits<80> {}
impl Reg128BitsDownCast<37> for Reg128Bits<80> {}
impl Reg128BitsConcat<37, 117> for Reg128Bits<80> {}
impl Reg128BitsDownCast<38> for Reg128Bits<80> {}
impl Reg128BitsConcat<38, 118> for Reg128Bits<80> {}
impl Reg128BitsDownCast<39> for Reg128Bits<80> {}
impl Reg128BitsConcat<39, 119> for Reg128Bits<80> {}
impl Reg128BitsDownCast<40> for Reg128Bits<80> {}
impl Reg128BitsConcat<40, 120> for Reg128Bits<80> {}
impl Reg128BitsDownCast<41> for Reg128Bits<80> {}
impl Reg128BitsConcat<41, 121> for Reg128Bits<80> {}
impl Reg128BitsDownCast<42> for Reg128Bits<80> {}
impl Reg128BitsConcat<42, 122> for Reg128Bits<80> {}
impl Reg128BitsDownCast<43> for Reg128Bits<80> {}
impl Reg128BitsConcat<43, 123> for Reg128Bits<80> {}
impl Reg128BitsDownCast<44> for Reg128Bits<80> {}
impl Reg128BitsConcat<44, 124> for Reg128Bits<80> {}
impl Reg128BitsDownCast<45> for Reg128Bits<80> {}
impl Reg128BitsConcat<45, 125> for Reg128Bits<80> {}
impl Reg128BitsDownCast<46> for Reg128Bits<80> {}
impl Reg128BitsConcat<46, 126> for Reg128Bits<80> {}
impl Reg128BitsDownCast<47> for Reg128Bits<80> {}
impl Reg128BitsConcat<47, 127> for Reg128Bits<80> {}
impl Reg128BitsDownCast<48> for Reg128Bits<80> {}
impl Reg128BitsConcat<48, 128> for Reg128Bits<80> {}
impl Reg128BitsDownCast<49> for Reg128Bits<80> {}
impl Reg128BitsDownCast<50> for Reg128Bits<80> {}
impl Reg128BitsDownCast<51> for Reg128Bits<80> {}
impl Reg128BitsDownCast<52> for Reg128Bits<80> {}
impl Reg128BitsDownCast<53> for Reg128Bits<80> {}
impl Reg128BitsDownCast<54> for Reg128Bits<80> {}
impl Reg128BitsDownCast<55> for Reg128Bits<80> {}
impl Reg128BitsDownCast<56> for Reg128Bits<80> {}
impl Reg128BitsDownCast<57> for Reg128Bits<80> {}
impl Reg128BitsDownCast<58> for Reg128Bits<80> {}
impl Reg128BitsDownCast<59> for Reg128Bits<80> {}
impl Reg128BitsDownCast<60> for Reg128Bits<80> {}
impl Reg128BitsDownCast<61> for Reg128Bits<80> {}
impl Reg128BitsDownCast<62> for Reg128Bits<80> {}
impl Reg128BitsDownCast<63> for Reg128Bits<80> {}
impl Reg128BitsDownCast<64> for Reg128Bits<80> {}
impl Reg128BitsDownCast<65> for Reg128Bits<80> {}
impl Reg128BitsDownCast<66> for Reg128Bits<80> {}
impl Reg128BitsDownCast<67> for Reg128Bits<80> {}
impl Reg128BitsDownCast<68> for Reg128Bits<80> {}
impl Reg128BitsDownCast<69> for Reg128Bits<80> {}
impl Reg128BitsDownCast<70> for Reg128Bits<80> {}
impl Reg128BitsDownCast<71> for Reg128Bits<80> {}
impl Reg128BitsDownCast<72> for Reg128Bits<80> {}
impl Reg128BitsDownCast<73> for Reg128Bits<80> {}
impl Reg128BitsDownCast<74> for Reg128Bits<80> {}
impl Reg128BitsDownCast<75> for Reg128Bits<80> {}
impl Reg128BitsDownCast<76> for Reg128Bits<80> {}
impl Reg128BitsDownCast<77> for Reg128Bits<80> {}
impl Reg128BitsDownCast<78> for Reg128Bits<80> {}
impl Reg128BitsDownCast<79> for Reg128Bits<80> {}
impl Reg128BitsDownCast<80> for Reg128Bits<80> {}
impl Reg128BitsDownCast<1> for Reg128Bits<81> {}
impl Reg128BitsConcat<1, 82> for Reg128Bits<81> {}
impl Reg128BitsDownCast<2> for Reg128Bits<81> {}
impl Reg128BitsConcat<2, 83> for Reg128Bits<81> {}
impl Reg128BitsDownCast<3> for Reg128Bits<81> {}
impl Reg128BitsConcat<3, 84> for Reg128Bits<81> {}
impl Reg128BitsDownCast<4> for Reg128Bits<81> {}
impl Reg128BitsConcat<4, 85> for Reg128Bits<81> {}
impl Reg128BitsDownCast<5> for Reg128Bits<81> {}
impl Reg128BitsConcat<5, 86> for Reg128Bits<81> {}
impl Reg128BitsDownCast<6> for Reg128Bits<81> {}
impl Reg128BitsConcat<6, 87> for Reg128Bits<81> {}
impl Reg128BitsDownCast<7> for Reg128Bits<81> {}
impl Reg128BitsConcat<7, 88> for Reg128Bits<81> {}
impl Reg128BitsDownCast<8> for Reg128Bits<81> {}
impl Reg128BitsConcat<8, 89> for Reg128Bits<81> {}
impl Reg128BitsDownCast<9> for Reg128Bits<81> {}
impl Reg128BitsConcat<9, 90> for Reg128Bits<81> {}
impl Reg128BitsDownCast<10> for Reg128Bits<81> {}
impl Reg128BitsConcat<10, 91> for Reg128Bits<81> {}
impl Reg128BitsDownCast<11> for Reg128Bits<81> {}
impl Reg128BitsConcat<11, 92> for Reg128Bits<81> {}
impl Reg128BitsDownCast<12> for Reg128Bits<81> {}
impl Reg128BitsConcat<12, 93> for Reg128Bits<81> {}
impl Reg128BitsDownCast<13> for Reg128Bits<81> {}
impl Reg128BitsConcat<13, 94> for Reg128Bits<81> {}
impl Reg128BitsDownCast<14> for Reg128Bits<81> {}
impl Reg128BitsConcat<14, 95> for Reg128Bits<81> {}
impl Reg128BitsDownCast<15> for Reg128Bits<81> {}
impl Reg128BitsConcat<15, 96> for Reg128Bits<81> {}
impl Reg128BitsDownCast<16> for Reg128Bits<81> {}
impl Reg128BitsConcat<16, 97> for Reg128Bits<81> {}
impl Reg128BitsDownCast<17> for Reg128Bits<81> {}
impl Reg128BitsConcat<17, 98> for Reg128Bits<81> {}
impl Reg128BitsDownCast<18> for Reg128Bits<81> {}
impl Reg128BitsConcat<18, 99> for Reg128Bits<81> {}
impl Reg128BitsDownCast<19> for Reg128Bits<81> {}
impl Reg128BitsConcat<19, 100> for Reg128Bits<81> {}
impl Reg128BitsDownCast<20> for Reg128Bits<81> {}
impl Reg128BitsConcat<20, 101> for Reg128Bits<81> {}
impl Reg128BitsDownCast<21> for Reg128Bits<81> {}
impl Reg128BitsConcat<21, 102> for Reg128Bits<81> {}
impl Reg128BitsDownCast<22> for Reg128Bits<81> {}
impl Reg128BitsConcat<22, 103> for Reg128Bits<81> {}
impl Reg128BitsDownCast<23> for Reg128Bits<81> {}
impl Reg128BitsConcat<23, 104> for Reg128Bits<81> {}
impl Reg128BitsDownCast<24> for Reg128Bits<81> {}
impl Reg128BitsConcat<24, 105> for Reg128Bits<81> {}
impl Reg128BitsDownCast<25> for Reg128Bits<81> {}
impl Reg128BitsConcat<25, 106> for Reg128Bits<81> {}
impl Reg128BitsDownCast<26> for Reg128Bits<81> {}
impl Reg128BitsConcat<26, 107> for Reg128Bits<81> {}
impl Reg128BitsDownCast<27> for Reg128Bits<81> {}
impl Reg128BitsConcat<27, 108> for Reg128Bits<81> {}
impl Reg128BitsDownCast<28> for Reg128Bits<81> {}
impl Reg128BitsConcat<28, 109> for Reg128Bits<81> {}
impl Reg128BitsDownCast<29> for Reg128Bits<81> {}
impl Reg128BitsConcat<29, 110> for Reg128Bits<81> {}
impl Reg128BitsDownCast<30> for Reg128Bits<81> {}
impl Reg128BitsConcat<30, 111> for Reg128Bits<81> {}
impl Reg128BitsDownCast<31> for Reg128Bits<81> {}
impl Reg128BitsConcat<31, 112> for Reg128Bits<81> {}
impl Reg128BitsDownCast<32> for Reg128Bits<81> {}
impl Reg128BitsConcat<32, 113> for Reg128Bits<81> {}
impl Reg128BitsDownCast<33> for Reg128Bits<81> {}
impl Reg128BitsConcat<33, 114> for Reg128Bits<81> {}
impl Reg128BitsDownCast<34> for Reg128Bits<81> {}
impl Reg128BitsConcat<34, 115> for Reg128Bits<81> {}
impl Reg128BitsDownCast<35> for Reg128Bits<81> {}
impl Reg128BitsConcat<35, 116> for Reg128Bits<81> {}
impl Reg128BitsDownCast<36> for Reg128Bits<81> {}
impl Reg128BitsConcat<36, 117> for Reg128Bits<81> {}
impl Reg128BitsDownCast<37> for Reg128Bits<81> {}
impl Reg128BitsConcat<37, 118> for Reg128Bits<81> {}
impl Reg128BitsDownCast<38> for Reg128Bits<81> {}
impl Reg128BitsConcat<38, 119> for Reg128Bits<81> {}
impl Reg128BitsDownCast<39> for Reg128Bits<81> {}
impl Reg128BitsConcat<39, 120> for Reg128Bits<81> {}
impl Reg128BitsDownCast<40> for Reg128Bits<81> {}
impl Reg128BitsConcat<40, 121> for Reg128Bits<81> {}
impl Reg128BitsDownCast<41> for Reg128Bits<81> {}
impl Reg128BitsConcat<41, 122> for Reg128Bits<81> {}
impl Reg128BitsDownCast<42> for Reg128Bits<81> {}
impl Reg128BitsConcat<42, 123> for Reg128Bits<81> {}
impl Reg128BitsDownCast<43> for Reg128Bits<81> {}
impl Reg128BitsConcat<43, 124> for Reg128Bits<81> {}
impl Reg128BitsDownCast<44> for Reg128Bits<81> {}
impl Reg128BitsConcat<44, 125> for Reg128Bits<81> {}
impl Reg128BitsDownCast<45> for Reg128Bits<81> {}
impl Reg128BitsConcat<45, 126> for Reg128Bits<81> {}
impl Reg128BitsDownCast<46> for Reg128Bits<81> {}
impl Reg128BitsConcat<46, 127> for Reg128Bits<81> {}
impl Reg128BitsDownCast<47> for Reg128Bits<81> {}
impl Reg128BitsConcat<47, 128> for Reg128Bits<81> {}
impl Reg128BitsDownCast<48> for Reg128Bits<81> {}
impl Reg128BitsDownCast<49> for Reg128Bits<81> {}
impl Reg128BitsDownCast<50> for Reg128Bits<81> {}
impl Reg128BitsDownCast<51> for Reg128Bits<81> {}
impl Reg128BitsDownCast<52> for Reg128Bits<81> {}
impl Reg128BitsDownCast<53> for Reg128Bits<81> {}
impl Reg128BitsDownCast<54> for Reg128Bits<81> {}
impl Reg128BitsDownCast<55> for Reg128Bits<81> {}
impl Reg128BitsDownCast<56> for Reg128Bits<81> {}
impl Reg128BitsDownCast<57> for Reg128Bits<81> {}
impl Reg128BitsDownCast<58> for Reg128Bits<81> {}
impl Reg128BitsDownCast<59> for Reg128Bits<81> {}
impl Reg128BitsDownCast<60> for Reg128Bits<81> {}
impl Reg128BitsDownCast<61> for Reg128Bits<81> {}
impl Reg128BitsDownCast<62> for Reg128Bits<81> {}
impl Reg128BitsDownCast<63> for Reg128Bits<81> {}
impl Reg128BitsDownCast<64> for Reg128Bits<81> {}
impl Reg128BitsDownCast<65> for Reg128Bits<81> {}
impl Reg128BitsDownCast<66> for Reg128Bits<81> {}
impl Reg128BitsDownCast<67> for Reg128Bits<81> {}
impl Reg128BitsDownCast<68> for Reg128Bits<81> {}
impl Reg128BitsDownCast<69> for Reg128Bits<81> {}
impl Reg128BitsDownCast<70> for Reg128Bits<81> {}
impl Reg128BitsDownCast<71> for Reg128Bits<81> {}
impl Reg128BitsDownCast<72> for Reg128Bits<81> {}
impl Reg128BitsDownCast<73> for Reg128Bits<81> {}
impl Reg128BitsDownCast<74> for Reg128Bits<81> {}
impl Reg128BitsDownCast<75> for Reg128Bits<81> {}
impl Reg128BitsDownCast<76> for Reg128Bits<81> {}
impl Reg128BitsDownCast<77> for Reg128Bits<81> {}
impl Reg128BitsDownCast<78> for Reg128Bits<81> {}
impl Reg128BitsDownCast<79> for Reg128Bits<81> {}
impl Reg128BitsDownCast<80> for Reg128Bits<81> {}
impl Reg128BitsDownCast<81> for Reg128Bits<81> {}
impl Reg128BitsDownCast<1> for Reg128Bits<82> {}
impl Reg128BitsConcat<1, 83> for Reg128Bits<82> {}
impl Reg128BitsDownCast<2> for Reg128Bits<82> {}
impl Reg128BitsConcat<2, 84> for Reg128Bits<82> {}
impl Reg128BitsDownCast<3> for Reg128Bits<82> {}
impl Reg128BitsConcat<3, 85> for Reg128Bits<82> {}
impl Reg128BitsDownCast<4> for Reg128Bits<82> {}
impl Reg128BitsConcat<4, 86> for Reg128Bits<82> {}
impl Reg128BitsDownCast<5> for Reg128Bits<82> {}
impl Reg128BitsConcat<5, 87> for Reg128Bits<82> {}
impl Reg128BitsDownCast<6> for Reg128Bits<82> {}
impl Reg128BitsConcat<6, 88> for Reg128Bits<82> {}
impl Reg128BitsDownCast<7> for Reg128Bits<82> {}
impl Reg128BitsConcat<7, 89> for Reg128Bits<82> {}
impl Reg128BitsDownCast<8> for Reg128Bits<82> {}
impl Reg128BitsConcat<8, 90> for Reg128Bits<82> {}
impl Reg128BitsDownCast<9> for Reg128Bits<82> {}
impl Reg128BitsConcat<9, 91> for Reg128Bits<82> {}
impl Reg128BitsDownCast<10> for Reg128Bits<82> {}
impl Reg128BitsConcat<10, 92> for Reg128Bits<82> {}
impl Reg128BitsDownCast<11> for Reg128Bits<82> {}
impl Reg128BitsConcat<11, 93> for Reg128Bits<82> {}
impl Reg128BitsDownCast<12> for Reg128Bits<82> {}
impl Reg128BitsConcat<12, 94> for Reg128Bits<82> {}
impl Reg128BitsDownCast<13> for Reg128Bits<82> {}
impl Reg128BitsConcat<13, 95> for Reg128Bits<82> {}
impl Reg128BitsDownCast<14> for Reg128Bits<82> {}
impl Reg128BitsConcat<14, 96> for Reg128Bits<82> {}
impl Reg128BitsDownCast<15> for Reg128Bits<82> {}
impl Reg128BitsConcat<15, 97> for Reg128Bits<82> {}
impl Reg128BitsDownCast<16> for Reg128Bits<82> {}
impl Reg128BitsConcat<16, 98> for Reg128Bits<82> {}
impl Reg128BitsDownCast<17> for Reg128Bits<82> {}
impl Reg128BitsConcat<17, 99> for Reg128Bits<82> {}
impl Reg128BitsDownCast<18> for Reg128Bits<82> {}
impl Reg128BitsConcat<18, 100> for Reg128Bits<82> {}
impl Reg128BitsDownCast<19> for Reg128Bits<82> {}
impl Reg128BitsConcat<19, 101> for Reg128Bits<82> {}
impl Reg128BitsDownCast<20> for Reg128Bits<82> {}
impl Reg128BitsConcat<20, 102> for Reg128Bits<82> {}
impl Reg128BitsDownCast<21> for Reg128Bits<82> {}
impl Reg128BitsConcat<21, 103> for Reg128Bits<82> {}
impl Reg128BitsDownCast<22> for Reg128Bits<82> {}
impl Reg128BitsConcat<22, 104> for Reg128Bits<82> {}
impl Reg128BitsDownCast<23> for Reg128Bits<82> {}
impl Reg128BitsConcat<23, 105> for Reg128Bits<82> {}
impl Reg128BitsDownCast<24> for Reg128Bits<82> {}
impl Reg128BitsConcat<24, 106> for Reg128Bits<82> {}
impl Reg128BitsDownCast<25> for Reg128Bits<82> {}
impl Reg128BitsConcat<25, 107> for Reg128Bits<82> {}
impl Reg128BitsDownCast<26> for Reg128Bits<82> {}
impl Reg128BitsConcat<26, 108> for Reg128Bits<82> {}
impl Reg128BitsDownCast<27> for Reg128Bits<82> {}
impl Reg128BitsConcat<27, 109> for Reg128Bits<82> {}
impl Reg128BitsDownCast<28> for Reg128Bits<82> {}
impl Reg128BitsConcat<28, 110> for Reg128Bits<82> {}
impl Reg128BitsDownCast<29> for Reg128Bits<82> {}
impl Reg128BitsConcat<29, 111> for Reg128Bits<82> {}
impl Reg128BitsDownCast<30> for Reg128Bits<82> {}
impl Reg128BitsConcat<30, 112> for Reg128Bits<82> {}
impl Reg128BitsDownCast<31> for Reg128Bits<82> {}
impl Reg128BitsConcat<31, 113> for Reg128Bits<82> {}
impl Reg128BitsDownCast<32> for Reg128Bits<82> {}
impl Reg128BitsConcat<32, 114> for Reg128Bits<82> {}
impl Reg128BitsDownCast<33> for Reg128Bits<82> {}
impl Reg128BitsConcat<33, 115> for Reg128Bits<82> {}
impl Reg128BitsDownCast<34> for Reg128Bits<82> {}
impl Reg128BitsConcat<34, 116> for Reg128Bits<82> {}
impl Reg128BitsDownCast<35> for Reg128Bits<82> {}
impl Reg128BitsConcat<35, 117> for Reg128Bits<82> {}
impl Reg128BitsDownCast<36> for Reg128Bits<82> {}
impl Reg128BitsConcat<36, 118> for Reg128Bits<82> {}
impl Reg128BitsDownCast<37> for Reg128Bits<82> {}
impl Reg128BitsConcat<37, 119> for Reg128Bits<82> {}
impl Reg128BitsDownCast<38> for Reg128Bits<82> {}
impl Reg128BitsConcat<38, 120> for Reg128Bits<82> {}
impl Reg128BitsDownCast<39> for Reg128Bits<82> {}
impl Reg128BitsConcat<39, 121> for Reg128Bits<82> {}
impl Reg128BitsDownCast<40> for Reg128Bits<82> {}
impl Reg128BitsConcat<40, 122> for Reg128Bits<82> {}
impl Reg128BitsDownCast<41> for Reg128Bits<82> {}
impl Reg128BitsConcat<41, 123> for Reg128Bits<82> {}
impl Reg128BitsDownCast<42> for Reg128Bits<82> {}
impl Reg128BitsConcat<42, 124> for Reg128Bits<82> {}
impl Reg128BitsDownCast<43> for Reg128Bits<82> {}
impl Reg128BitsConcat<43, 125> for Reg128Bits<82> {}
impl Reg128BitsDownCast<44> for Reg128Bits<82> {}
impl Reg128BitsConcat<44, 126> for Reg128Bits<82> {}
impl Reg128BitsDownCast<45> for Reg128Bits<82> {}
impl Reg128BitsConcat<45, 127> for Reg128Bits<82> {}
impl Reg128BitsDownCast<46> for Reg128Bits<82> {}
impl Reg128BitsConcat<46, 128> for Reg128Bits<82> {}
impl Reg128BitsDownCast<47> for Reg128Bits<82> {}
impl Reg128BitsDownCast<48> for Reg128Bits<82> {}
impl Reg128BitsDownCast<49> for Reg128Bits<82> {}
impl Reg128BitsDownCast<50> for Reg128Bits<82> {}
impl Reg128BitsDownCast<51> for Reg128Bits<82> {}
impl Reg128BitsDownCast<52> for Reg128Bits<82> {}
impl Reg128BitsDownCast<53> for Reg128Bits<82> {}
impl Reg128BitsDownCast<54> for Reg128Bits<82> {}
impl Reg128BitsDownCast<55> for Reg128Bits<82> {}
impl Reg128BitsDownCast<56> for Reg128Bits<82> {}
impl Reg128BitsDownCast<57> for Reg128Bits<82> {}
impl Reg128BitsDownCast<58> for Reg128Bits<82> {}
impl Reg128BitsDownCast<59> for Reg128Bits<82> {}
impl Reg128BitsDownCast<60> for Reg128Bits<82> {}
impl Reg128BitsDownCast<61> for Reg128Bits<82> {}
impl Reg128BitsDownCast<62> for Reg128Bits<82> {}
impl Reg128BitsDownCast<63> for Reg128Bits<82> {}
impl Reg128BitsDownCast<64> for Reg128Bits<82> {}
impl Reg128BitsDownCast<65> for Reg128Bits<82> {}
impl Reg128BitsDownCast<66> for Reg128Bits<82> {}
impl Reg128BitsDownCast<67> for Reg128Bits<82> {}
impl Reg128BitsDownCast<68> for Reg128Bits<82> {}
impl Reg128BitsDownCast<69> for Reg128Bits<82> {}
impl Reg128BitsDownCast<70> for Reg128Bits<82> {}
impl Reg128BitsDownCast<71> for Reg128Bits<82> {}
impl Reg128BitsDownCast<72> for Reg128Bits<82> {}
impl Reg128BitsDownCast<73> for Reg128Bits<82> {}
impl Reg128BitsDownCast<74> for Reg128Bits<82> {}
impl Reg128BitsDownCast<75> for Reg128Bits<82> {}
impl Reg128BitsDownCast<76> for Reg128Bits<82> {}
impl Reg128BitsDownCast<77> for Reg128Bits<82> {}
impl Reg128BitsDownCast<78> for Reg128Bits<82> {}
impl Reg128BitsDownCast<79> for Reg128Bits<82> {}
impl Reg128BitsDownCast<80> for Reg128Bits<82> {}
impl Reg128BitsDownCast<81> for Reg128Bits<82> {}
impl Reg128BitsDownCast<82> for Reg128Bits<82> {}
impl Reg128BitsDownCast<1> for Reg128Bits<83> {}
impl Reg128BitsConcat<1, 84> for Reg128Bits<83> {}
impl Reg128BitsDownCast<2> for Reg128Bits<83> {}
impl Reg128BitsConcat<2, 85> for Reg128Bits<83> {}
impl Reg128BitsDownCast<3> for Reg128Bits<83> {}
impl Reg128BitsConcat<3, 86> for Reg128Bits<83> {}
impl Reg128BitsDownCast<4> for Reg128Bits<83> {}
impl Reg128BitsConcat<4, 87> for Reg128Bits<83> {}
impl Reg128BitsDownCast<5> for Reg128Bits<83> {}
impl Reg128BitsConcat<5, 88> for Reg128Bits<83> {}
impl Reg128BitsDownCast<6> for Reg128Bits<83> {}
impl Reg128BitsConcat<6, 89> for Reg128Bits<83> {}
impl Reg128BitsDownCast<7> for Reg128Bits<83> {}
impl Reg128BitsConcat<7, 90> for Reg128Bits<83> {}
impl Reg128BitsDownCast<8> for Reg128Bits<83> {}
impl Reg128BitsConcat<8, 91> for Reg128Bits<83> {}
impl Reg128BitsDownCast<9> for Reg128Bits<83> {}
impl Reg128BitsConcat<9, 92> for Reg128Bits<83> {}
impl Reg128BitsDownCast<10> for Reg128Bits<83> {}
impl Reg128BitsConcat<10, 93> for Reg128Bits<83> {}
impl Reg128BitsDownCast<11> for Reg128Bits<83> {}
impl Reg128BitsConcat<11, 94> for Reg128Bits<83> {}
impl Reg128BitsDownCast<12> for Reg128Bits<83> {}
impl Reg128BitsConcat<12, 95> for Reg128Bits<83> {}
impl Reg128BitsDownCast<13> for Reg128Bits<83> {}
impl Reg128BitsConcat<13, 96> for Reg128Bits<83> {}
impl Reg128BitsDownCast<14> for Reg128Bits<83> {}
impl Reg128BitsConcat<14, 97> for Reg128Bits<83> {}
impl Reg128BitsDownCast<15> for Reg128Bits<83> {}
impl Reg128BitsConcat<15, 98> for Reg128Bits<83> {}
impl Reg128BitsDownCast<16> for Reg128Bits<83> {}
impl Reg128BitsConcat<16, 99> for Reg128Bits<83> {}
impl Reg128BitsDownCast<17> for Reg128Bits<83> {}
impl Reg128BitsConcat<17, 100> for Reg128Bits<83> {}
impl Reg128BitsDownCast<18> for Reg128Bits<83> {}
impl Reg128BitsConcat<18, 101> for Reg128Bits<83> {}
impl Reg128BitsDownCast<19> for Reg128Bits<83> {}
impl Reg128BitsConcat<19, 102> for Reg128Bits<83> {}
impl Reg128BitsDownCast<20> for Reg128Bits<83> {}
impl Reg128BitsConcat<20, 103> for Reg128Bits<83> {}
impl Reg128BitsDownCast<21> for Reg128Bits<83> {}
impl Reg128BitsConcat<21, 104> for Reg128Bits<83> {}
impl Reg128BitsDownCast<22> for Reg128Bits<83> {}
impl Reg128BitsConcat<22, 105> for Reg128Bits<83> {}
impl Reg128BitsDownCast<23> for Reg128Bits<83> {}
impl Reg128BitsConcat<23, 106> for Reg128Bits<83> {}
impl Reg128BitsDownCast<24> for Reg128Bits<83> {}
impl Reg128BitsConcat<24, 107> for Reg128Bits<83> {}
impl Reg128BitsDownCast<25> for Reg128Bits<83> {}
impl Reg128BitsConcat<25, 108> for Reg128Bits<83> {}
impl Reg128BitsDownCast<26> for Reg128Bits<83> {}
impl Reg128BitsConcat<26, 109> for Reg128Bits<83> {}
impl Reg128BitsDownCast<27> for Reg128Bits<83> {}
impl Reg128BitsConcat<27, 110> for Reg128Bits<83> {}
impl Reg128BitsDownCast<28> for Reg128Bits<83> {}
impl Reg128BitsConcat<28, 111> for Reg128Bits<83> {}
impl Reg128BitsDownCast<29> for Reg128Bits<83> {}
impl Reg128BitsConcat<29, 112> for Reg128Bits<83> {}
impl Reg128BitsDownCast<30> for Reg128Bits<83> {}
impl Reg128BitsConcat<30, 113> for Reg128Bits<83> {}
impl Reg128BitsDownCast<31> for Reg128Bits<83> {}
impl Reg128BitsConcat<31, 114> for Reg128Bits<83> {}
impl Reg128BitsDownCast<32> for Reg128Bits<83> {}
impl Reg128BitsConcat<32, 115> for Reg128Bits<83> {}
impl Reg128BitsDownCast<33> for Reg128Bits<83> {}
impl Reg128BitsConcat<33, 116> for Reg128Bits<83> {}
impl Reg128BitsDownCast<34> for Reg128Bits<83> {}
impl Reg128BitsConcat<34, 117> for Reg128Bits<83> {}
impl Reg128BitsDownCast<35> for Reg128Bits<83> {}
impl Reg128BitsConcat<35, 118> for Reg128Bits<83> {}
impl Reg128BitsDownCast<36> for Reg128Bits<83> {}
impl Reg128BitsConcat<36, 119> for Reg128Bits<83> {}
impl Reg128BitsDownCast<37> for Reg128Bits<83> {}
impl Reg128BitsConcat<37, 120> for Reg128Bits<83> {}
impl Reg128BitsDownCast<38> for Reg128Bits<83> {}
impl Reg128BitsConcat<38, 121> for Reg128Bits<83> {}
impl Reg128BitsDownCast<39> for Reg128Bits<83> {}
impl Reg128BitsConcat<39, 122> for Reg128Bits<83> {}
impl Reg128BitsDownCast<40> for Reg128Bits<83> {}
impl Reg128BitsConcat<40, 123> for Reg128Bits<83> {}
impl Reg128BitsDownCast<41> for Reg128Bits<83> {}
impl Reg128BitsConcat<41, 124> for Reg128Bits<83> {}
impl Reg128BitsDownCast<42> for Reg128Bits<83> {}
impl Reg128BitsConcat<42, 125> for Reg128Bits<83> {}
impl Reg128BitsDownCast<43> for Reg128Bits<83> {}
impl Reg128BitsConcat<43, 126> for Reg128Bits<83> {}
impl Reg128BitsDownCast<44> for Reg128Bits<83> {}
impl Reg128BitsConcat<44, 127> for Reg128Bits<83> {}
impl Reg128BitsDownCast<45> for Reg128Bits<83> {}
impl Reg128BitsConcat<45, 128> for Reg128Bits<83> {}
impl Reg128BitsDownCast<46> for Reg128Bits<83> {}
impl Reg128BitsDownCast<47> for Reg128Bits<83> {}
impl Reg128BitsDownCast<48> for Reg128Bits<83> {}
impl Reg128BitsDownCast<49> for Reg128Bits<83> {}
impl Reg128BitsDownCast<50> for Reg128Bits<83> {}
impl Reg128BitsDownCast<51> for Reg128Bits<83> {}
impl Reg128BitsDownCast<52> for Reg128Bits<83> {}
impl Reg128BitsDownCast<53> for Reg128Bits<83> {}
impl Reg128BitsDownCast<54> for Reg128Bits<83> {}
impl Reg128BitsDownCast<55> for Reg128Bits<83> {}
impl Reg128BitsDownCast<56> for Reg128Bits<83> {}
impl Reg128BitsDownCast<57> for Reg128Bits<83> {}
impl Reg128BitsDownCast<58> for Reg128Bits<83> {}
impl Reg128BitsDownCast<59> for Reg128Bits<83> {}
impl Reg128BitsDownCast<60> for Reg128Bits<83> {}
impl Reg128BitsDownCast<61> for Reg128Bits<83> {}
impl Reg128BitsDownCast<62> for Reg128Bits<83> {}
impl Reg128BitsDownCast<63> for Reg128Bits<83> {}
impl Reg128BitsDownCast<64> for Reg128Bits<83> {}
impl Reg128BitsDownCast<65> for Reg128Bits<83> {}
impl Reg128BitsDownCast<66> for Reg128Bits<83> {}
impl Reg128BitsDownCast<67> for Reg128Bits<83> {}
impl Reg128BitsDownCast<68> for Reg128Bits<83> {}
impl Reg128BitsDownCast<69> for Reg128Bits<83> {}
impl Reg128BitsDownCast<70> for Reg128Bits<83> {}
impl Reg128BitsDownCast<71> for Reg128Bits<83> {}
impl Reg128BitsDownCast<72> for Reg128Bits<83> {}
impl Reg128BitsDownCast<73> for Reg128Bits<83> {}
impl Reg128BitsDownCast<74> for Reg128Bits<83> {}
impl Reg128BitsDownCast<75> for Reg128Bits<83> {}
impl Reg128BitsDownCast<76> for Reg128Bits<83> {}
impl Reg128BitsDownCast<77> for Reg128Bits<83> {}
impl Reg128BitsDownCast<78> for Reg128Bits<83> {}
impl Reg128BitsDownCast<79> for Reg128Bits<83> {}
impl Reg128BitsDownCast<80> for Reg128Bits<83> {}
impl Reg128BitsDownCast<81> for Reg128Bits<83> {}
impl Reg128BitsDownCast<82> for Reg128Bits<83> {}
impl Reg128BitsDownCast<83> for Reg128Bits<83> {}
impl Reg128BitsDownCast<1> for Reg128Bits<84> {}
impl Reg128BitsConcat<1, 85> for Reg128Bits<84> {}
impl Reg128BitsDownCast<2> for Reg128Bits<84> {}
impl Reg128BitsConcat<2, 86> for Reg128Bits<84> {}
impl Reg128BitsDownCast<3> for Reg128Bits<84> {}
impl Reg128BitsConcat<3, 87> for Reg128Bits<84> {}
impl Reg128BitsDownCast<4> for Reg128Bits<84> {}
impl Reg128BitsConcat<4, 88> for Reg128Bits<84> {}
impl Reg128BitsDownCast<5> for Reg128Bits<84> {}
impl Reg128BitsConcat<5, 89> for Reg128Bits<84> {}
impl Reg128BitsDownCast<6> for Reg128Bits<84> {}
impl Reg128BitsConcat<6, 90> for Reg128Bits<84> {}
impl Reg128BitsDownCast<7> for Reg128Bits<84> {}
impl Reg128BitsConcat<7, 91> for Reg128Bits<84> {}
impl Reg128BitsDownCast<8> for Reg128Bits<84> {}
impl Reg128BitsConcat<8, 92> for Reg128Bits<84> {}
impl Reg128BitsDownCast<9> for Reg128Bits<84> {}
impl Reg128BitsConcat<9, 93> for Reg128Bits<84> {}
impl Reg128BitsDownCast<10> for Reg128Bits<84> {}
impl Reg128BitsConcat<10, 94> for Reg128Bits<84> {}
impl Reg128BitsDownCast<11> for Reg128Bits<84> {}
impl Reg128BitsConcat<11, 95> for Reg128Bits<84> {}
impl Reg128BitsDownCast<12> for Reg128Bits<84> {}
impl Reg128BitsConcat<12, 96> for Reg128Bits<84> {}
impl Reg128BitsDownCast<13> for Reg128Bits<84> {}
impl Reg128BitsConcat<13, 97> for Reg128Bits<84> {}
impl Reg128BitsDownCast<14> for Reg128Bits<84> {}
impl Reg128BitsConcat<14, 98> for Reg128Bits<84> {}
impl Reg128BitsDownCast<15> for Reg128Bits<84> {}
impl Reg128BitsConcat<15, 99> for Reg128Bits<84> {}
impl Reg128BitsDownCast<16> for Reg128Bits<84> {}
impl Reg128BitsConcat<16, 100> for Reg128Bits<84> {}
impl Reg128BitsDownCast<17> for Reg128Bits<84> {}
impl Reg128BitsConcat<17, 101> for Reg128Bits<84> {}
impl Reg128BitsDownCast<18> for Reg128Bits<84> {}
impl Reg128BitsConcat<18, 102> for Reg128Bits<84> {}
impl Reg128BitsDownCast<19> for Reg128Bits<84> {}
impl Reg128BitsConcat<19, 103> for Reg128Bits<84> {}
impl Reg128BitsDownCast<20> for Reg128Bits<84> {}
impl Reg128BitsConcat<20, 104> for Reg128Bits<84> {}
impl Reg128BitsDownCast<21> for Reg128Bits<84> {}
impl Reg128BitsConcat<21, 105> for Reg128Bits<84> {}
impl Reg128BitsDownCast<22> for Reg128Bits<84> {}
impl Reg128BitsConcat<22, 106> for Reg128Bits<84> {}
impl Reg128BitsDownCast<23> for Reg128Bits<84> {}
impl Reg128BitsConcat<23, 107> for Reg128Bits<84> {}
impl Reg128BitsDownCast<24> for Reg128Bits<84> {}
impl Reg128BitsConcat<24, 108> for Reg128Bits<84> {}
impl Reg128BitsDownCast<25> for Reg128Bits<84> {}
impl Reg128BitsConcat<25, 109> for Reg128Bits<84> {}
impl Reg128BitsDownCast<26> for Reg128Bits<84> {}
impl Reg128BitsConcat<26, 110> for Reg128Bits<84> {}
impl Reg128BitsDownCast<27> for Reg128Bits<84> {}
impl Reg128BitsConcat<27, 111> for Reg128Bits<84> {}
impl Reg128BitsDownCast<28> for Reg128Bits<84> {}
impl Reg128BitsConcat<28, 112> for Reg128Bits<84> {}
impl Reg128BitsDownCast<29> for Reg128Bits<84> {}
impl Reg128BitsConcat<29, 113> for Reg128Bits<84> {}
impl Reg128BitsDownCast<30> for Reg128Bits<84> {}
impl Reg128BitsConcat<30, 114> for Reg128Bits<84> {}
impl Reg128BitsDownCast<31> for Reg128Bits<84> {}
impl Reg128BitsConcat<31, 115> for Reg128Bits<84> {}
impl Reg128BitsDownCast<32> for Reg128Bits<84> {}
impl Reg128BitsConcat<32, 116> for Reg128Bits<84> {}
impl Reg128BitsDownCast<33> for Reg128Bits<84> {}
impl Reg128BitsConcat<33, 117> for Reg128Bits<84> {}
impl Reg128BitsDownCast<34> for Reg128Bits<84> {}
impl Reg128BitsConcat<34, 118> for Reg128Bits<84> {}
impl Reg128BitsDownCast<35> for Reg128Bits<84> {}
impl Reg128BitsConcat<35, 119> for Reg128Bits<84> {}
impl Reg128BitsDownCast<36> for Reg128Bits<84> {}
impl Reg128BitsConcat<36, 120> for Reg128Bits<84> {}
impl Reg128BitsDownCast<37> for Reg128Bits<84> {}
impl Reg128BitsConcat<37, 121> for Reg128Bits<84> {}
impl Reg128BitsDownCast<38> for Reg128Bits<84> {}
impl Reg128BitsConcat<38, 122> for Reg128Bits<84> {}
impl Reg128BitsDownCast<39> for Reg128Bits<84> {}
impl Reg128BitsConcat<39, 123> for Reg128Bits<84> {}
impl Reg128BitsDownCast<40> for Reg128Bits<84> {}
impl Reg128BitsConcat<40, 124> for Reg128Bits<84> {}
impl Reg128BitsDownCast<41> for Reg128Bits<84> {}
impl Reg128BitsConcat<41, 125> for Reg128Bits<84> {}
impl Reg128BitsDownCast<42> for Reg128Bits<84> {}
impl Reg128BitsConcat<42, 126> for Reg128Bits<84> {}
impl Reg128BitsDownCast<43> for Reg128Bits<84> {}
impl Reg128BitsConcat<43, 127> for Reg128Bits<84> {}
impl Reg128BitsDownCast<44> for Reg128Bits<84> {}
impl Reg128BitsConcat<44, 128> for Reg128Bits<84> {}
impl Reg128BitsDownCast<45> for Reg128Bits<84> {}
impl Reg128BitsDownCast<46> for Reg128Bits<84> {}
impl Reg128BitsDownCast<47> for Reg128Bits<84> {}
impl Reg128BitsDownCast<48> for Reg128Bits<84> {}
impl Reg128BitsDownCast<49> for Reg128Bits<84> {}
impl Reg128BitsDownCast<50> for Reg128Bits<84> {}
impl Reg128BitsDownCast<51> for Reg128Bits<84> {}
impl Reg128BitsDownCast<52> for Reg128Bits<84> {}
impl Reg128BitsDownCast<53> for Reg128Bits<84> {}
impl Reg128BitsDownCast<54> for Reg128Bits<84> {}
impl Reg128BitsDownCast<55> for Reg128Bits<84> {}
impl Reg128BitsDownCast<56> for Reg128Bits<84> {}
impl Reg128BitsDownCast<57> for Reg128Bits<84> {}
impl Reg128BitsDownCast<58> for Reg128Bits<84> {}
impl Reg128BitsDownCast<59> for Reg128Bits<84> {}
impl Reg128BitsDownCast<60> for Reg128Bits<84> {}
impl Reg128BitsDownCast<61> for Reg128Bits<84> {}
impl Reg128BitsDownCast<62> for Reg128Bits<84> {}
impl Reg128BitsDownCast<63> for Reg128Bits<84> {}
impl Reg128BitsDownCast<64> for Reg128Bits<84> {}
impl Reg128BitsDownCast<65> for Reg128Bits<84> {}
impl Reg128BitsDownCast<66> for Reg128Bits<84> {}
impl Reg128BitsDownCast<67> for Reg128Bits<84> {}
impl Reg128BitsDownCast<68> for Reg128Bits<84> {}
impl Reg128BitsDownCast<69> for Reg128Bits<84> {}
impl Reg128BitsDownCast<70> for Reg128Bits<84> {}
impl Reg128BitsDownCast<71> for Reg128Bits<84> {}
impl Reg128BitsDownCast<72> for Reg128Bits<84> {}
impl Reg128BitsDownCast<73> for Reg128Bits<84> {}
impl Reg128BitsDownCast<74> for Reg128Bits<84> {}
impl Reg128BitsDownCast<75> for Reg128Bits<84> {}
impl Reg128BitsDownCast<76> for Reg128Bits<84> {}
impl Reg128BitsDownCast<77> for Reg128Bits<84> {}
impl Reg128BitsDownCast<78> for Reg128Bits<84> {}
impl Reg128BitsDownCast<79> for Reg128Bits<84> {}
impl Reg128BitsDownCast<80> for Reg128Bits<84> {}
impl Reg128BitsDownCast<81> for Reg128Bits<84> {}
impl Reg128BitsDownCast<82> for Reg128Bits<84> {}
impl Reg128BitsDownCast<83> for Reg128Bits<84> {}
impl Reg128BitsDownCast<84> for Reg128Bits<84> {}
impl Reg128BitsDownCast<1> for Reg128Bits<85> {}
impl Reg128BitsConcat<1, 86> for Reg128Bits<85> {}
impl Reg128BitsDownCast<2> for Reg128Bits<85> {}
impl Reg128BitsConcat<2, 87> for Reg128Bits<85> {}
impl Reg128BitsDownCast<3> for Reg128Bits<85> {}
impl Reg128BitsConcat<3, 88> for Reg128Bits<85> {}
impl Reg128BitsDownCast<4> for Reg128Bits<85> {}
impl Reg128BitsConcat<4, 89> for Reg128Bits<85> {}
impl Reg128BitsDownCast<5> for Reg128Bits<85> {}
impl Reg128BitsConcat<5, 90> for Reg128Bits<85> {}
impl Reg128BitsDownCast<6> for Reg128Bits<85> {}
impl Reg128BitsConcat<6, 91> for Reg128Bits<85> {}
impl Reg128BitsDownCast<7> for Reg128Bits<85> {}
impl Reg128BitsConcat<7, 92> for Reg128Bits<85> {}
impl Reg128BitsDownCast<8> for Reg128Bits<85> {}
impl Reg128BitsConcat<8, 93> for Reg128Bits<85> {}
impl Reg128BitsDownCast<9> for Reg128Bits<85> {}
impl Reg128BitsConcat<9, 94> for Reg128Bits<85> {}
impl Reg128BitsDownCast<10> for Reg128Bits<85> {}
impl Reg128BitsConcat<10, 95> for Reg128Bits<85> {}
impl Reg128BitsDownCast<11> for Reg128Bits<85> {}
impl Reg128BitsConcat<11, 96> for Reg128Bits<85> {}
impl Reg128BitsDownCast<12> for Reg128Bits<85> {}
impl Reg128BitsConcat<12, 97> for Reg128Bits<85> {}
impl Reg128BitsDownCast<13> for Reg128Bits<85> {}
impl Reg128BitsConcat<13, 98> for Reg128Bits<85> {}
impl Reg128BitsDownCast<14> for Reg128Bits<85> {}
impl Reg128BitsConcat<14, 99> for Reg128Bits<85> {}
impl Reg128BitsDownCast<15> for Reg128Bits<85> {}
impl Reg128BitsConcat<15, 100> for Reg128Bits<85> {}
impl Reg128BitsDownCast<16> for Reg128Bits<85> {}
impl Reg128BitsConcat<16, 101> for Reg128Bits<85> {}
impl Reg128BitsDownCast<17> for Reg128Bits<85> {}
impl Reg128BitsConcat<17, 102> for Reg128Bits<85> {}
impl Reg128BitsDownCast<18> for Reg128Bits<85> {}
impl Reg128BitsConcat<18, 103> for Reg128Bits<85> {}
impl Reg128BitsDownCast<19> for Reg128Bits<85> {}
impl Reg128BitsConcat<19, 104> for Reg128Bits<85> {}
impl Reg128BitsDownCast<20> for Reg128Bits<85> {}
impl Reg128BitsConcat<20, 105> for Reg128Bits<85> {}
impl Reg128BitsDownCast<21> for Reg128Bits<85> {}
impl Reg128BitsConcat<21, 106> for Reg128Bits<85> {}
impl Reg128BitsDownCast<22> for Reg128Bits<85> {}
impl Reg128BitsConcat<22, 107> for Reg128Bits<85> {}
impl Reg128BitsDownCast<23> for Reg128Bits<85> {}
impl Reg128BitsConcat<23, 108> for Reg128Bits<85> {}
impl Reg128BitsDownCast<24> for Reg128Bits<85> {}
impl Reg128BitsConcat<24, 109> for Reg128Bits<85> {}
impl Reg128BitsDownCast<25> for Reg128Bits<85> {}
impl Reg128BitsConcat<25, 110> for Reg128Bits<85> {}
impl Reg128BitsDownCast<26> for Reg128Bits<85> {}
impl Reg128BitsConcat<26, 111> for Reg128Bits<85> {}
impl Reg128BitsDownCast<27> for Reg128Bits<85> {}
impl Reg128BitsConcat<27, 112> for Reg128Bits<85> {}
impl Reg128BitsDownCast<28> for Reg128Bits<85> {}
impl Reg128BitsConcat<28, 113> for Reg128Bits<85> {}
impl Reg128BitsDownCast<29> for Reg128Bits<85> {}
impl Reg128BitsConcat<29, 114> for Reg128Bits<85> {}
impl Reg128BitsDownCast<30> for Reg128Bits<85> {}
impl Reg128BitsConcat<30, 115> for Reg128Bits<85> {}
impl Reg128BitsDownCast<31> for Reg128Bits<85> {}
impl Reg128BitsConcat<31, 116> for Reg128Bits<85> {}
impl Reg128BitsDownCast<32> for Reg128Bits<85> {}
impl Reg128BitsConcat<32, 117> for Reg128Bits<85> {}
impl Reg128BitsDownCast<33> for Reg128Bits<85> {}
impl Reg128BitsConcat<33, 118> for Reg128Bits<85> {}
impl Reg128BitsDownCast<34> for Reg128Bits<85> {}
impl Reg128BitsConcat<34, 119> for Reg128Bits<85> {}
impl Reg128BitsDownCast<35> for Reg128Bits<85> {}
impl Reg128BitsConcat<35, 120> for Reg128Bits<85> {}
impl Reg128BitsDownCast<36> for Reg128Bits<85> {}
impl Reg128BitsConcat<36, 121> for Reg128Bits<85> {}
impl Reg128BitsDownCast<37> for Reg128Bits<85> {}
impl Reg128BitsConcat<37, 122> for Reg128Bits<85> {}
impl Reg128BitsDownCast<38> for Reg128Bits<85> {}
impl Reg128BitsConcat<38, 123> for Reg128Bits<85> {}
impl Reg128BitsDownCast<39> for Reg128Bits<85> {}
impl Reg128BitsConcat<39, 124> for Reg128Bits<85> {}
impl Reg128BitsDownCast<40> for Reg128Bits<85> {}
impl Reg128BitsConcat<40, 125> for Reg128Bits<85> {}
impl Reg128BitsDownCast<41> for Reg128Bits<85> {}
impl Reg128BitsConcat<41, 126> for Reg128Bits<85> {}
impl Reg128BitsDownCast<42> for Reg128Bits<85> {}
impl Reg128BitsConcat<42, 127> for Reg128Bits<85> {}
impl Reg128BitsDownCast<43> for Reg128Bits<85> {}
impl Reg128BitsConcat<43, 128> for Reg128Bits<85> {}
impl Reg128BitsDownCast<44> for Reg128Bits<85> {}
impl Reg128BitsDownCast<45> for Reg128Bits<85> {}
impl Reg128BitsDownCast<46> for Reg128Bits<85> {}
impl Reg128BitsDownCast<47> for Reg128Bits<85> {}
impl Reg128BitsDownCast<48> for Reg128Bits<85> {}
impl Reg128BitsDownCast<49> for Reg128Bits<85> {}
impl Reg128BitsDownCast<50> for Reg128Bits<85> {}
impl Reg128BitsDownCast<51> for Reg128Bits<85> {}
impl Reg128BitsDownCast<52> for Reg128Bits<85> {}
impl Reg128BitsDownCast<53> for Reg128Bits<85> {}
impl Reg128BitsDownCast<54> for Reg128Bits<85> {}
impl Reg128BitsDownCast<55> for Reg128Bits<85> {}
impl Reg128BitsDownCast<56> for Reg128Bits<85> {}
impl Reg128BitsDownCast<57> for Reg128Bits<85> {}
impl Reg128BitsDownCast<58> for Reg128Bits<85> {}
impl Reg128BitsDownCast<59> for Reg128Bits<85> {}
impl Reg128BitsDownCast<60> for Reg128Bits<85> {}
impl Reg128BitsDownCast<61> for Reg128Bits<85> {}
impl Reg128BitsDownCast<62> for Reg128Bits<85> {}
impl Reg128BitsDownCast<63> for Reg128Bits<85> {}
impl Reg128BitsDownCast<64> for Reg128Bits<85> {}
impl Reg128BitsDownCast<65> for Reg128Bits<85> {}
impl Reg128BitsDownCast<66> for Reg128Bits<85> {}
impl Reg128BitsDownCast<67> for Reg128Bits<85> {}
impl Reg128BitsDownCast<68> for Reg128Bits<85> {}
impl Reg128BitsDownCast<69> for Reg128Bits<85> {}
impl Reg128BitsDownCast<70> for Reg128Bits<85> {}
impl Reg128BitsDownCast<71> for Reg128Bits<85> {}
impl Reg128BitsDownCast<72> for Reg128Bits<85> {}
impl Reg128BitsDownCast<73> for Reg128Bits<85> {}
impl Reg128BitsDownCast<74> for Reg128Bits<85> {}
impl Reg128BitsDownCast<75> for Reg128Bits<85> {}
impl Reg128BitsDownCast<76> for Reg128Bits<85> {}
impl Reg128BitsDownCast<77> for Reg128Bits<85> {}
impl Reg128BitsDownCast<78> for Reg128Bits<85> {}
impl Reg128BitsDownCast<79> for Reg128Bits<85> {}
impl Reg128BitsDownCast<80> for Reg128Bits<85> {}
impl Reg128BitsDownCast<81> for Reg128Bits<85> {}
impl Reg128BitsDownCast<82> for Reg128Bits<85> {}
impl Reg128BitsDownCast<83> for Reg128Bits<85> {}
impl Reg128BitsDownCast<84> for Reg128Bits<85> {}
impl Reg128BitsDownCast<85> for Reg128Bits<85> {}
impl Reg128BitsDownCast<1> for Reg128Bits<86> {}
impl Reg128BitsConcat<1, 87> for Reg128Bits<86> {}
impl Reg128BitsDownCast<2> for Reg128Bits<86> {}
impl Reg128BitsConcat<2, 88> for Reg128Bits<86> {}
impl Reg128BitsDownCast<3> for Reg128Bits<86> {}
impl Reg128BitsConcat<3, 89> for Reg128Bits<86> {}
impl Reg128BitsDownCast<4> for Reg128Bits<86> {}
impl Reg128BitsConcat<4, 90> for Reg128Bits<86> {}
impl Reg128BitsDownCast<5> for Reg128Bits<86> {}
impl Reg128BitsConcat<5, 91> for Reg128Bits<86> {}
impl Reg128BitsDownCast<6> for Reg128Bits<86> {}
impl Reg128BitsConcat<6, 92> for Reg128Bits<86> {}
impl Reg128BitsDownCast<7> for Reg128Bits<86> {}
impl Reg128BitsConcat<7, 93> for Reg128Bits<86> {}
impl Reg128BitsDownCast<8> for Reg128Bits<86> {}
impl Reg128BitsConcat<8, 94> for Reg128Bits<86> {}
impl Reg128BitsDownCast<9> for Reg128Bits<86> {}
impl Reg128BitsConcat<9, 95> for Reg128Bits<86> {}
impl Reg128BitsDownCast<10> for Reg128Bits<86> {}
impl Reg128BitsConcat<10, 96> for Reg128Bits<86> {}
impl Reg128BitsDownCast<11> for Reg128Bits<86> {}
impl Reg128BitsConcat<11, 97> for Reg128Bits<86> {}
impl Reg128BitsDownCast<12> for Reg128Bits<86> {}
impl Reg128BitsConcat<12, 98> for Reg128Bits<86> {}
impl Reg128BitsDownCast<13> for Reg128Bits<86> {}
impl Reg128BitsConcat<13, 99> for Reg128Bits<86> {}
impl Reg128BitsDownCast<14> for Reg128Bits<86> {}
impl Reg128BitsConcat<14, 100> for Reg128Bits<86> {}
impl Reg128BitsDownCast<15> for Reg128Bits<86> {}
impl Reg128BitsConcat<15, 101> for Reg128Bits<86> {}
impl Reg128BitsDownCast<16> for Reg128Bits<86> {}
impl Reg128BitsConcat<16, 102> for Reg128Bits<86> {}
impl Reg128BitsDownCast<17> for Reg128Bits<86> {}
impl Reg128BitsConcat<17, 103> for Reg128Bits<86> {}
impl Reg128BitsDownCast<18> for Reg128Bits<86> {}
impl Reg128BitsConcat<18, 104> for Reg128Bits<86> {}
impl Reg128BitsDownCast<19> for Reg128Bits<86> {}
impl Reg128BitsConcat<19, 105> for Reg128Bits<86> {}
impl Reg128BitsDownCast<20> for Reg128Bits<86> {}
impl Reg128BitsConcat<20, 106> for Reg128Bits<86> {}
impl Reg128BitsDownCast<21> for Reg128Bits<86> {}
impl Reg128BitsConcat<21, 107> for Reg128Bits<86> {}
impl Reg128BitsDownCast<22> for Reg128Bits<86> {}
impl Reg128BitsConcat<22, 108> for Reg128Bits<86> {}
impl Reg128BitsDownCast<23> for Reg128Bits<86> {}
impl Reg128BitsConcat<23, 109> for Reg128Bits<86> {}
impl Reg128BitsDownCast<24> for Reg128Bits<86> {}
impl Reg128BitsConcat<24, 110> for Reg128Bits<86> {}
impl Reg128BitsDownCast<25> for Reg128Bits<86> {}
impl Reg128BitsConcat<25, 111> for Reg128Bits<86> {}
impl Reg128BitsDownCast<26> for Reg128Bits<86> {}
impl Reg128BitsConcat<26, 112> for Reg128Bits<86> {}
impl Reg128BitsDownCast<27> for Reg128Bits<86> {}
impl Reg128BitsConcat<27, 113> for Reg128Bits<86> {}
impl Reg128BitsDownCast<28> for Reg128Bits<86> {}
impl Reg128BitsConcat<28, 114> for Reg128Bits<86> {}
impl Reg128BitsDownCast<29> for Reg128Bits<86> {}
impl Reg128BitsConcat<29, 115> for Reg128Bits<86> {}
impl Reg128BitsDownCast<30> for Reg128Bits<86> {}
impl Reg128BitsConcat<30, 116> for Reg128Bits<86> {}
impl Reg128BitsDownCast<31> for Reg128Bits<86> {}
impl Reg128BitsConcat<31, 117> for Reg128Bits<86> {}
impl Reg128BitsDownCast<32> for Reg128Bits<86> {}
impl Reg128BitsConcat<32, 118> for Reg128Bits<86> {}
impl Reg128BitsDownCast<33> for Reg128Bits<86> {}
impl Reg128BitsConcat<33, 119> for Reg128Bits<86> {}
impl Reg128BitsDownCast<34> for Reg128Bits<86> {}
impl Reg128BitsConcat<34, 120> for Reg128Bits<86> {}
impl Reg128BitsDownCast<35> for Reg128Bits<86> {}
impl Reg128BitsConcat<35, 121> for Reg128Bits<86> {}
impl Reg128BitsDownCast<36> for Reg128Bits<86> {}
impl Reg128BitsConcat<36, 122> for Reg128Bits<86> {}
impl Reg128BitsDownCast<37> for Reg128Bits<86> {}
impl Reg128BitsConcat<37, 123> for Reg128Bits<86> {}
impl Reg128BitsDownCast<38> for Reg128Bits<86> {}
impl Reg128BitsConcat<38, 124> for Reg128Bits<86> {}
impl Reg128BitsDownCast<39> for Reg128Bits<86> {}
impl Reg128BitsConcat<39, 125> for Reg128Bits<86> {}
impl Reg128BitsDownCast<40> for Reg128Bits<86> {}
impl Reg128BitsConcat<40, 126> for Reg128Bits<86> {}
impl Reg128BitsDownCast<41> for Reg128Bits<86> {}
impl Reg128BitsConcat<41, 127> for Reg128Bits<86> {}
impl Reg128BitsDownCast<42> for Reg128Bits<86> {}
impl Reg128BitsConcat<42, 128> for Reg128Bits<86> {}
impl Reg128BitsDownCast<43> for Reg128Bits<86> {}
impl Reg128BitsDownCast<44> for Reg128Bits<86> {}
impl Reg128BitsDownCast<45> for Reg128Bits<86> {}
impl Reg128BitsDownCast<46> for Reg128Bits<86> {}
impl Reg128BitsDownCast<47> for Reg128Bits<86> {}
impl Reg128BitsDownCast<48> for Reg128Bits<86> {}
impl Reg128BitsDownCast<49> for Reg128Bits<86> {}
impl Reg128BitsDownCast<50> for Reg128Bits<86> {}
impl Reg128BitsDownCast<51> for Reg128Bits<86> {}
impl Reg128BitsDownCast<52> for Reg128Bits<86> {}
impl Reg128BitsDownCast<53> for Reg128Bits<86> {}
impl Reg128BitsDownCast<54> for Reg128Bits<86> {}
impl Reg128BitsDownCast<55> for Reg128Bits<86> {}
impl Reg128BitsDownCast<56> for Reg128Bits<86> {}
impl Reg128BitsDownCast<57> for Reg128Bits<86> {}
impl Reg128BitsDownCast<58> for Reg128Bits<86> {}
impl Reg128BitsDownCast<59> for Reg128Bits<86> {}
impl Reg128BitsDownCast<60> for Reg128Bits<86> {}
impl Reg128BitsDownCast<61> for Reg128Bits<86> {}
impl Reg128BitsDownCast<62> for Reg128Bits<86> {}
impl Reg128BitsDownCast<63> for Reg128Bits<86> {}
impl Reg128BitsDownCast<64> for Reg128Bits<86> {}
impl Reg128BitsDownCast<65> for Reg128Bits<86> {}
impl Reg128BitsDownCast<66> for Reg128Bits<86> {}
impl Reg128BitsDownCast<67> for Reg128Bits<86> {}
impl Reg128BitsDownCast<68> for Reg128Bits<86> {}
impl Reg128BitsDownCast<69> for Reg128Bits<86> {}
impl Reg128BitsDownCast<70> for Reg128Bits<86> {}
impl Reg128BitsDownCast<71> for Reg128Bits<86> {}
impl Reg128BitsDownCast<72> for Reg128Bits<86> {}
impl Reg128BitsDownCast<73> for Reg128Bits<86> {}
impl Reg128BitsDownCast<74> for Reg128Bits<86> {}
impl Reg128BitsDownCast<75> for Reg128Bits<86> {}
impl Reg128BitsDownCast<76> for Reg128Bits<86> {}
impl Reg128BitsDownCast<77> for Reg128Bits<86> {}
impl Reg128BitsDownCast<78> for Reg128Bits<86> {}
impl Reg128BitsDownCast<79> for Reg128Bits<86> {}
impl Reg128BitsDownCast<80> for Reg128Bits<86> {}
impl Reg128BitsDownCast<81> for Reg128Bits<86> {}
impl Reg128BitsDownCast<82> for Reg128Bits<86> {}
impl Reg128BitsDownCast<83> for Reg128Bits<86> {}
impl Reg128BitsDownCast<84> for Reg128Bits<86> {}
impl Reg128BitsDownCast<85> for Reg128Bits<86> {}
impl Reg128BitsDownCast<86> for Reg128Bits<86> {}
impl Reg128BitsDownCast<1> for Reg128Bits<87> {}
impl Reg128BitsConcat<1, 88> for Reg128Bits<87> {}
impl Reg128BitsDownCast<2> for Reg128Bits<87> {}
impl Reg128BitsConcat<2, 89> for Reg128Bits<87> {}
impl Reg128BitsDownCast<3> for Reg128Bits<87> {}
impl Reg128BitsConcat<3, 90> for Reg128Bits<87> {}
impl Reg128BitsDownCast<4> for Reg128Bits<87> {}
impl Reg128BitsConcat<4, 91> for Reg128Bits<87> {}
impl Reg128BitsDownCast<5> for Reg128Bits<87> {}
impl Reg128BitsConcat<5, 92> for Reg128Bits<87> {}
impl Reg128BitsDownCast<6> for Reg128Bits<87> {}
impl Reg128BitsConcat<6, 93> for Reg128Bits<87> {}
impl Reg128BitsDownCast<7> for Reg128Bits<87> {}
impl Reg128BitsConcat<7, 94> for Reg128Bits<87> {}
impl Reg128BitsDownCast<8> for Reg128Bits<87> {}
impl Reg128BitsConcat<8, 95> for Reg128Bits<87> {}
impl Reg128BitsDownCast<9> for Reg128Bits<87> {}
impl Reg128BitsConcat<9, 96> for Reg128Bits<87> {}
impl Reg128BitsDownCast<10> for Reg128Bits<87> {}
impl Reg128BitsConcat<10, 97> for Reg128Bits<87> {}
impl Reg128BitsDownCast<11> for Reg128Bits<87> {}
impl Reg128BitsConcat<11, 98> for Reg128Bits<87> {}
impl Reg128BitsDownCast<12> for Reg128Bits<87> {}
impl Reg128BitsConcat<12, 99> for Reg128Bits<87> {}
impl Reg128BitsDownCast<13> for Reg128Bits<87> {}
impl Reg128BitsConcat<13, 100> for Reg128Bits<87> {}
impl Reg128BitsDownCast<14> for Reg128Bits<87> {}
impl Reg128BitsConcat<14, 101> for Reg128Bits<87> {}
impl Reg128BitsDownCast<15> for Reg128Bits<87> {}
impl Reg128BitsConcat<15, 102> for Reg128Bits<87> {}
impl Reg128BitsDownCast<16> for Reg128Bits<87> {}
impl Reg128BitsConcat<16, 103> for Reg128Bits<87> {}
impl Reg128BitsDownCast<17> for Reg128Bits<87> {}
impl Reg128BitsConcat<17, 104> for Reg128Bits<87> {}
impl Reg128BitsDownCast<18> for Reg128Bits<87> {}
impl Reg128BitsConcat<18, 105> for Reg128Bits<87> {}
impl Reg128BitsDownCast<19> for Reg128Bits<87> {}
impl Reg128BitsConcat<19, 106> for Reg128Bits<87> {}
impl Reg128BitsDownCast<20> for Reg128Bits<87> {}
impl Reg128BitsConcat<20, 107> for Reg128Bits<87> {}
impl Reg128BitsDownCast<21> for Reg128Bits<87> {}
impl Reg128BitsConcat<21, 108> for Reg128Bits<87> {}
impl Reg128BitsDownCast<22> for Reg128Bits<87> {}
impl Reg128BitsConcat<22, 109> for Reg128Bits<87> {}
impl Reg128BitsDownCast<23> for Reg128Bits<87> {}
impl Reg128BitsConcat<23, 110> for Reg128Bits<87> {}
impl Reg128BitsDownCast<24> for Reg128Bits<87> {}
impl Reg128BitsConcat<24, 111> for Reg128Bits<87> {}
impl Reg128BitsDownCast<25> for Reg128Bits<87> {}
impl Reg128BitsConcat<25, 112> for Reg128Bits<87> {}
impl Reg128BitsDownCast<26> for Reg128Bits<87> {}
impl Reg128BitsConcat<26, 113> for Reg128Bits<87> {}
impl Reg128BitsDownCast<27> for Reg128Bits<87> {}
impl Reg128BitsConcat<27, 114> for Reg128Bits<87> {}
impl Reg128BitsDownCast<28> for Reg128Bits<87> {}
impl Reg128BitsConcat<28, 115> for Reg128Bits<87> {}
impl Reg128BitsDownCast<29> for Reg128Bits<87> {}
impl Reg128BitsConcat<29, 116> for Reg128Bits<87> {}
impl Reg128BitsDownCast<30> for Reg128Bits<87> {}
impl Reg128BitsConcat<30, 117> for Reg128Bits<87> {}
impl Reg128BitsDownCast<31> for Reg128Bits<87> {}
impl Reg128BitsConcat<31, 118> for Reg128Bits<87> {}
impl Reg128BitsDownCast<32> for Reg128Bits<87> {}
impl Reg128BitsConcat<32, 119> for Reg128Bits<87> {}
impl Reg128BitsDownCast<33> for Reg128Bits<87> {}
impl Reg128BitsConcat<33, 120> for Reg128Bits<87> {}
impl Reg128BitsDownCast<34> for Reg128Bits<87> {}
impl Reg128BitsConcat<34, 121> for Reg128Bits<87> {}
impl Reg128BitsDownCast<35> for Reg128Bits<87> {}
impl Reg128BitsConcat<35, 122> for Reg128Bits<87> {}
impl Reg128BitsDownCast<36> for Reg128Bits<87> {}
impl Reg128BitsConcat<36, 123> for Reg128Bits<87> {}
impl Reg128BitsDownCast<37> for Reg128Bits<87> {}
impl Reg128BitsConcat<37, 124> for Reg128Bits<87> {}
impl Reg128BitsDownCast<38> for Reg128Bits<87> {}
impl Reg128BitsConcat<38, 125> for Reg128Bits<87> {}
impl Reg128BitsDownCast<39> for Reg128Bits<87> {}
impl Reg128BitsConcat<39, 126> for Reg128Bits<87> {}
impl Reg128BitsDownCast<40> for Reg128Bits<87> {}
impl Reg128BitsConcat<40, 127> for Reg128Bits<87> {}
impl Reg128BitsDownCast<41> for Reg128Bits<87> {}
impl Reg128BitsConcat<41, 128> for Reg128Bits<87> {}
impl Reg128BitsDownCast<42> for Reg128Bits<87> {}
impl Reg128BitsDownCast<43> for Reg128Bits<87> {}
impl Reg128BitsDownCast<44> for Reg128Bits<87> {}
impl Reg128BitsDownCast<45> for Reg128Bits<87> {}
impl Reg128BitsDownCast<46> for Reg128Bits<87> {}
impl Reg128BitsDownCast<47> for Reg128Bits<87> {}
impl Reg128BitsDownCast<48> for Reg128Bits<87> {}
impl Reg128BitsDownCast<49> for Reg128Bits<87> {}
impl Reg128BitsDownCast<50> for Reg128Bits<87> {}
impl Reg128BitsDownCast<51> for Reg128Bits<87> {}
impl Reg128BitsDownCast<52> for Reg128Bits<87> {}
impl Reg128BitsDownCast<53> for Reg128Bits<87> {}
impl Reg128BitsDownCast<54> for Reg128Bits<87> {}
impl Reg128BitsDownCast<55> for Reg128Bits<87> {}
impl Reg128BitsDownCast<56> for Reg128Bits<87> {}
impl Reg128BitsDownCast<57> for Reg128Bits<87> {}
impl Reg128BitsDownCast<58> for Reg128Bits<87> {}
impl Reg128BitsDownCast<59> for Reg128Bits<87> {}
impl Reg128BitsDownCast<60> for Reg128Bits<87> {}
impl Reg128BitsDownCast<61> for Reg128Bits<87> {}
impl Reg128BitsDownCast<62> for Reg128Bits<87> {}
impl Reg128BitsDownCast<63> for Reg128Bits<87> {}
impl Reg128BitsDownCast<64> for Reg128Bits<87> {}
impl Reg128BitsDownCast<65> for Reg128Bits<87> {}
impl Reg128BitsDownCast<66> for Reg128Bits<87> {}
impl Reg128BitsDownCast<67> for Reg128Bits<87> {}
impl Reg128BitsDownCast<68> for Reg128Bits<87> {}
impl Reg128BitsDownCast<69> for Reg128Bits<87> {}
impl Reg128BitsDownCast<70> for Reg128Bits<87> {}
impl Reg128BitsDownCast<71> for Reg128Bits<87> {}
impl Reg128BitsDownCast<72> for Reg128Bits<87> {}
impl Reg128BitsDownCast<73> for Reg128Bits<87> {}
impl Reg128BitsDownCast<74> for Reg128Bits<87> {}
impl Reg128BitsDownCast<75> for Reg128Bits<87> {}
impl Reg128BitsDownCast<76> for Reg128Bits<87> {}
impl Reg128BitsDownCast<77> for Reg128Bits<87> {}
impl Reg128BitsDownCast<78> for Reg128Bits<87> {}
impl Reg128BitsDownCast<79> for Reg128Bits<87> {}
impl Reg128BitsDownCast<80> for Reg128Bits<87> {}
impl Reg128BitsDownCast<81> for Reg128Bits<87> {}
impl Reg128BitsDownCast<82> for Reg128Bits<87> {}
impl Reg128BitsDownCast<83> for Reg128Bits<87> {}
impl Reg128BitsDownCast<84> for Reg128Bits<87> {}
impl Reg128BitsDownCast<85> for Reg128Bits<87> {}
impl Reg128BitsDownCast<86> for Reg128Bits<87> {}
impl Reg128BitsDownCast<87> for Reg128Bits<87> {}
impl Reg128BitsDownCast<1> for Reg128Bits<88> {}
impl Reg128BitsConcat<1, 89> for Reg128Bits<88> {}
impl Reg128BitsDownCast<2> for Reg128Bits<88> {}
impl Reg128BitsConcat<2, 90> for Reg128Bits<88> {}
impl Reg128BitsDownCast<3> for Reg128Bits<88> {}
impl Reg128BitsConcat<3, 91> for Reg128Bits<88> {}
impl Reg128BitsDownCast<4> for Reg128Bits<88> {}
impl Reg128BitsConcat<4, 92> for Reg128Bits<88> {}
impl Reg128BitsDownCast<5> for Reg128Bits<88> {}
impl Reg128BitsConcat<5, 93> for Reg128Bits<88> {}
impl Reg128BitsDownCast<6> for Reg128Bits<88> {}
impl Reg128BitsConcat<6, 94> for Reg128Bits<88> {}
impl Reg128BitsDownCast<7> for Reg128Bits<88> {}
impl Reg128BitsConcat<7, 95> for Reg128Bits<88> {}
impl Reg128BitsDownCast<8> for Reg128Bits<88> {}
impl Reg128BitsConcat<8, 96> for Reg128Bits<88> {}
impl Reg128BitsDownCast<9> for Reg128Bits<88> {}
impl Reg128BitsConcat<9, 97> for Reg128Bits<88> {}
impl Reg128BitsDownCast<10> for Reg128Bits<88> {}
impl Reg128BitsConcat<10, 98> for Reg128Bits<88> {}
impl Reg128BitsDownCast<11> for Reg128Bits<88> {}
impl Reg128BitsConcat<11, 99> for Reg128Bits<88> {}
impl Reg128BitsDownCast<12> for Reg128Bits<88> {}
impl Reg128BitsConcat<12, 100> for Reg128Bits<88> {}
impl Reg128BitsDownCast<13> for Reg128Bits<88> {}
impl Reg128BitsConcat<13, 101> for Reg128Bits<88> {}
impl Reg128BitsDownCast<14> for Reg128Bits<88> {}
impl Reg128BitsConcat<14, 102> for Reg128Bits<88> {}
impl Reg128BitsDownCast<15> for Reg128Bits<88> {}
impl Reg128BitsConcat<15, 103> for Reg128Bits<88> {}
impl Reg128BitsDownCast<16> for Reg128Bits<88> {}
impl Reg128BitsConcat<16, 104> for Reg128Bits<88> {}
impl Reg128BitsDownCast<17> for Reg128Bits<88> {}
impl Reg128BitsConcat<17, 105> for Reg128Bits<88> {}
impl Reg128BitsDownCast<18> for Reg128Bits<88> {}
impl Reg128BitsConcat<18, 106> for Reg128Bits<88> {}
impl Reg128BitsDownCast<19> for Reg128Bits<88> {}
impl Reg128BitsConcat<19, 107> for Reg128Bits<88> {}
impl Reg128BitsDownCast<20> for Reg128Bits<88> {}
impl Reg128BitsConcat<20, 108> for Reg128Bits<88> {}
impl Reg128BitsDownCast<21> for Reg128Bits<88> {}
impl Reg128BitsConcat<21, 109> for Reg128Bits<88> {}
impl Reg128BitsDownCast<22> for Reg128Bits<88> {}
impl Reg128BitsConcat<22, 110> for Reg128Bits<88> {}
impl Reg128BitsDownCast<23> for Reg128Bits<88> {}
impl Reg128BitsConcat<23, 111> for Reg128Bits<88> {}
impl Reg128BitsDownCast<24> for Reg128Bits<88> {}
impl Reg128BitsConcat<24, 112> for Reg128Bits<88> {}
impl Reg128BitsDownCast<25> for Reg128Bits<88> {}
impl Reg128BitsConcat<25, 113> for Reg128Bits<88> {}
impl Reg128BitsDownCast<26> for Reg128Bits<88> {}
impl Reg128BitsConcat<26, 114> for Reg128Bits<88> {}
impl Reg128BitsDownCast<27> for Reg128Bits<88> {}
impl Reg128BitsConcat<27, 115> for Reg128Bits<88> {}
impl Reg128BitsDownCast<28> for Reg128Bits<88> {}
impl Reg128BitsConcat<28, 116> for Reg128Bits<88> {}
impl Reg128BitsDownCast<29> for Reg128Bits<88> {}
impl Reg128BitsConcat<29, 117> for Reg128Bits<88> {}
impl Reg128BitsDownCast<30> for Reg128Bits<88> {}
impl Reg128BitsConcat<30, 118> for Reg128Bits<88> {}
impl Reg128BitsDownCast<31> for Reg128Bits<88> {}
impl Reg128BitsConcat<31, 119> for Reg128Bits<88> {}
impl Reg128BitsDownCast<32> for Reg128Bits<88> {}
impl Reg128BitsConcat<32, 120> for Reg128Bits<88> {}
impl Reg128BitsDownCast<33> for Reg128Bits<88> {}
impl Reg128BitsConcat<33, 121> for Reg128Bits<88> {}
impl Reg128BitsDownCast<34> for Reg128Bits<88> {}
impl Reg128BitsConcat<34, 122> for Reg128Bits<88> {}
impl Reg128BitsDownCast<35> for Reg128Bits<88> {}
impl Reg128BitsConcat<35, 123> for Reg128Bits<88> {}
impl Reg128BitsDownCast<36> for Reg128Bits<88> {}
impl Reg128BitsConcat<36, 124> for Reg128Bits<88> {}
impl Reg128BitsDownCast<37> for Reg128Bits<88> {}
impl Reg128BitsConcat<37, 125> for Reg128Bits<88> {}
impl Reg128BitsDownCast<38> for Reg128Bits<88> {}
impl Reg128BitsConcat<38, 126> for Reg128Bits<88> {}
impl Reg128BitsDownCast<39> for Reg128Bits<88> {}
impl Reg128BitsConcat<39, 127> for Reg128Bits<88> {}
impl Reg128BitsDownCast<40> for Reg128Bits<88> {}
impl Reg128BitsConcat<40, 128> for Reg128Bits<88> {}
impl Reg128BitsDownCast<41> for Reg128Bits<88> {}
impl Reg128BitsDownCast<42> for Reg128Bits<88> {}
impl Reg128BitsDownCast<43> for Reg128Bits<88> {}
impl Reg128BitsDownCast<44> for Reg128Bits<88> {}
impl Reg128BitsDownCast<45> for Reg128Bits<88> {}
impl Reg128BitsDownCast<46> for Reg128Bits<88> {}
impl Reg128BitsDownCast<47> for Reg128Bits<88> {}
impl Reg128BitsDownCast<48> for Reg128Bits<88> {}
impl Reg128BitsDownCast<49> for Reg128Bits<88> {}
impl Reg128BitsDownCast<50> for Reg128Bits<88> {}
impl Reg128BitsDownCast<51> for Reg128Bits<88> {}
impl Reg128BitsDownCast<52> for Reg128Bits<88> {}
impl Reg128BitsDownCast<53> for Reg128Bits<88> {}
impl Reg128BitsDownCast<54> for Reg128Bits<88> {}
impl Reg128BitsDownCast<55> for Reg128Bits<88> {}
impl Reg128BitsDownCast<56> for Reg128Bits<88> {}
impl Reg128BitsDownCast<57> for Reg128Bits<88> {}
impl Reg128BitsDownCast<58> for Reg128Bits<88> {}
impl Reg128BitsDownCast<59> for Reg128Bits<88> {}
impl Reg128BitsDownCast<60> for Reg128Bits<88> {}
impl Reg128BitsDownCast<61> for Reg128Bits<88> {}
impl Reg128BitsDownCast<62> for Reg128Bits<88> {}
impl Reg128BitsDownCast<63> for Reg128Bits<88> {}
impl Reg128BitsDownCast<64> for Reg128Bits<88> {}
impl Reg128BitsDownCast<65> for Reg128Bits<88> {}
impl Reg128BitsDownCast<66> for Reg128Bits<88> {}
impl Reg128BitsDownCast<67> for Reg128Bits<88> {}
impl Reg128BitsDownCast<68> for Reg128Bits<88> {}
impl Reg128BitsDownCast<69> for Reg128Bits<88> {}
impl Reg128BitsDownCast<70> for Reg128Bits<88> {}
impl Reg128BitsDownCast<71> for Reg128Bits<88> {}
impl Reg128BitsDownCast<72> for Reg128Bits<88> {}
impl Reg128BitsDownCast<73> for Reg128Bits<88> {}
impl Reg128BitsDownCast<74> for Reg128Bits<88> {}
impl Reg128BitsDownCast<75> for Reg128Bits<88> {}
impl Reg128BitsDownCast<76> for Reg128Bits<88> {}
impl Reg128BitsDownCast<77> for Reg128Bits<88> {}
impl Reg128BitsDownCast<78> for Reg128Bits<88> {}
impl Reg128BitsDownCast<79> for Reg128Bits<88> {}
impl Reg128BitsDownCast<80> for Reg128Bits<88> {}
impl Reg128BitsDownCast<81> for Reg128Bits<88> {}
impl Reg128BitsDownCast<82> for Reg128Bits<88> {}
impl Reg128BitsDownCast<83> for Reg128Bits<88> {}
impl Reg128BitsDownCast<84> for Reg128Bits<88> {}
impl Reg128BitsDownCast<85> for Reg128Bits<88> {}
impl Reg128BitsDownCast<86> for Reg128Bits<88> {}
impl Reg128BitsDownCast<87> for Reg128Bits<88> {}
impl Reg128BitsDownCast<88> for Reg128Bits<88> {}
impl Reg128BitsDownCast<1> for Reg128Bits<89> {}
impl Reg128BitsConcat<1, 90> for Reg128Bits<89> {}
impl Reg128BitsDownCast<2> for Reg128Bits<89> {}
impl Reg128BitsConcat<2, 91> for Reg128Bits<89> {}
impl Reg128BitsDownCast<3> for Reg128Bits<89> {}
impl Reg128BitsConcat<3, 92> for Reg128Bits<89> {}
impl Reg128BitsDownCast<4> for Reg128Bits<89> {}
impl Reg128BitsConcat<4, 93> for Reg128Bits<89> {}
impl Reg128BitsDownCast<5> for Reg128Bits<89> {}
impl Reg128BitsConcat<5, 94> for Reg128Bits<89> {}
impl Reg128BitsDownCast<6> for Reg128Bits<89> {}
impl Reg128BitsConcat<6, 95> for Reg128Bits<89> {}
impl Reg128BitsDownCast<7> for Reg128Bits<89> {}
impl Reg128BitsConcat<7, 96> for Reg128Bits<89> {}
impl Reg128BitsDownCast<8> for Reg128Bits<89> {}
impl Reg128BitsConcat<8, 97> for Reg128Bits<89> {}
impl Reg128BitsDownCast<9> for Reg128Bits<89> {}
impl Reg128BitsConcat<9, 98> for Reg128Bits<89> {}
impl Reg128BitsDownCast<10> for Reg128Bits<89> {}
impl Reg128BitsConcat<10, 99> for Reg128Bits<89> {}
impl Reg128BitsDownCast<11> for Reg128Bits<89> {}
impl Reg128BitsConcat<11, 100> for Reg128Bits<89> {}
impl Reg128BitsDownCast<12> for Reg128Bits<89> {}
impl Reg128BitsConcat<12, 101> for Reg128Bits<89> {}
impl Reg128BitsDownCast<13> for Reg128Bits<89> {}
impl Reg128BitsConcat<13, 102> for Reg128Bits<89> {}
impl Reg128BitsDownCast<14> for Reg128Bits<89> {}
impl Reg128BitsConcat<14, 103> for Reg128Bits<89> {}
impl Reg128BitsDownCast<15> for Reg128Bits<89> {}
impl Reg128BitsConcat<15, 104> for Reg128Bits<89> {}
impl Reg128BitsDownCast<16> for Reg128Bits<89> {}
impl Reg128BitsConcat<16, 105> for Reg128Bits<89> {}
impl Reg128BitsDownCast<17> for Reg128Bits<89> {}
impl Reg128BitsConcat<17, 106> for Reg128Bits<89> {}
impl Reg128BitsDownCast<18> for Reg128Bits<89> {}
impl Reg128BitsConcat<18, 107> for Reg128Bits<89> {}
impl Reg128BitsDownCast<19> for Reg128Bits<89> {}
impl Reg128BitsConcat<19, 108> for Reg128Bits<89> {}
impl Reg128BitsDownCast<20> for Reg128Bits<89> {}
impl Reg128BitsConcat<20, 109> for Reg128Bits<89> {}
impl Reg128BitsDownCast<21> for Reg128Bits<89> {}
impl Reg128BitsConcat<21, 110> for Reg128Bits<89> {}
impl Reg128BitsDownCast<22> for Reg128Bits<89> {}
impl Reg128BitsConcat<22, 111> for Reg128Bits<89> {}
impl Reg128BitsDownCast<23> for Reg128Bits<89> {}
impl Reg128BitsConcat<23, 112> for Reg128Bits<89> {}
impl Reg128BitsDownCast<24> for Reg128Bits<89> {}
impl Reg128BitsConcat<24, 113> for Reg128Bits<89> {}
impl Reg128BitsDownCast<25> for Reg128Bits<89> {}
impl Reg128BitsConcat<25, 114> for Reg128Bits<89> {}
impl Reg128BitsDownCast<26> for Reg128Bits<89> {}
impl Reg128BitsConcat<26, 115> for Reg128Bits<89> {}
impl Reg128BitsDownCast<27> for Reg128Bits<89> {}
impl Reg128BitsConcat<27, 116> for Reg128Bits<89> {}
impl Reg128BitsDownCast<28> for Reg128Bits<89> {}
impl Reg128BitsConcat<28, 117> for Reg128Bits<89> {}
impl Reg128BitsDownCast<29> for Reg128Bits<89> {}
impl Reg128BitsConcat<29, 118> for Reg128Bits<89> {}
impl Reg128BitsDownCast<30> for Reg128Bits<89> {}
impl Reg128BitsConcat<30, 119> for Reg128Bits<89> {}
impl Reg128BitsDownCast<31> for Reg128Bits<89> {}
impl Reg128BitsConcat<31, 120> for Reg128Bits<89> {}
impl Reg128BitsDownCast<32> for Reg128Bits<89> {}
impl Reg128BitsConcat<32, 121> for Reg128Bits<89> {}
impl Reg128BitsDownCast<33> for Reg128Bits<89> {}
impl Reg128BitsConcat<33, 122> for Reg128Bits<89> {}
impl Reg128BitsDownCast<34> for Reg128Bits<89> {}
impl Reg128BitsConcat<34, 123> for Reg128Bits<89> {}
impl Reg128BitsDownCast<35> for Reg128Bits<89> {}
impl Reg128BitsConcat<35, 124> for Reg128Bits<89> {}
impl Reg128BitsDownCast<36> for Reg128Bits<89> {}
impl Reg128BitsConcat<36, 125> for Reg128Bits<89> {}
impl Reg128BitsDownCast<37> for Reg128Bits<89> {}
impl Reg128BitsConcat<37, 126> for Reg128Bits<89> {}
impl Reg128BitsDownCast<38> for Reg128Bits<89> {}
impl Reg128BitsConcat<38, 127> for Reg128Bits<89> {}
impl Reg128BitsDownCast<39> for Reg128Bits<89> {}
impl Reg128BitsConcat<39, 128> for Reg128Bits<89> {}
impl Reg128BitsDownCast<40> for Reg128Bits<89> {}
impl Reg128BitsDownCast<41> for Reg128Bits<89> {}
impl Reg128BitsDownCast<42> for Reg128Bits<89> {}
impl Reg128BitsDownCast<43> for Reg128Bits<89> {}
impl Reg128BitsDownCast<44> for Reg128Bits<89> {}
impl Reg128BitsDownCast<45> for Reg128Bits<89> {}
impl Reg128BitsDownCast<46> for Reg128Bits<89> {}
impl Reg128BitsDownCast<47> for Reg128Bits<89> {}
impl Reg128BitsDownCast<48> for Reg128Bits<89> {}
impl Reg128BitsDownCast<49> for Reg128Bits<89> {}
impl Reg128BitsDownCast<50> for Reg128Bits<89> {}
impl Reg128BitsDownCast<51> for Reg128Bits<89> {}
impl Reg128BitsDownCast<52> for Reg128Bits<89> {}
impl Reg128BitsDownCast<53> for Reg128Bits<89> {}
impl Reg128BitsDownCast<54> for Reg128Bits<89> {}
impl Reg128BitsDownCast<55> for Reg128Bits<89> {}
impl Reg128BitsDownCast<56> for Reg128Bits<89> {}
impl Reg128BitsDownCast<57> for Reg128Bits<89> {}
impl Reg128BitsDownCast<58> for Reg128Bits<89> {}
impl Reg128BitsDownCast<59> for Reg128Bits<89> {}
impl Reg128BitsDownCast<60> for Reg128Bits<89> {}
impl Reg128BitsDownCast<61> for Reg128Bits<89> {}
impl Reg128BitsDownCast<62> for Reg128Bits<89> {}
impl Reg128BitsDownCast<63> for Reg128Bits<89> {}
impl Reg128BitsDownCast<64> for Reg128Bits<89> {}
impl Reg128BitsDownCast<65> for Reg128Bits<89> {}
impl Reg128BitsDownCast<66> for Reg128Bits<89> {}
impl Reg128BitsDownCast<67> for Reg128Bits<89> {}
impl Reg128BitsDownCast<68> for Reg128Bits<89> {}
impl Reg128BitsDownCast<69> for Reg128Bits<89> {}
impl Reg128BitsDownCast<70> for Reg128Bits<89> {}
impl Reg128BitsDownCast<71> for Reg128Bits<89> {}
impl Reg128BitsDownCast<72> for Reg128Bits<89> {}
impl Reg128BitsDownCast<73> for Reg128Bits<89> {}
impl Reg128BitsDownCast<74> for Reg128Bits<89> {}
impl Reg128BitsDownCast<75> for Reg128Bits<89> {}
impl Reg128BitsDownCast<76> for Reg128Bits<89> {}
impl Reg128BitsDownCast<77> for Reg128Bits<89> {}
impl Reg128BitsDownCast<78> for Reg128Bits<89> {}
impl Reg128BitsDownCast<79> for Reg128Bits<89> {}
impl Reg128BitsDownCast<80> for Reg128Bits<89> {}
impl Reg128BitsDownCast<81> for Reg128Bits<89> {}
impl Reg128BitsDownCast<82> for Reg128Bits<89> {}
impl Reg128BitsDownCast<83> for Reg128Bits<89> {}
impl Reg128BitsDownCast<84> for Reg128Bits<89> {}
impl Reg128BitsDownCast<85> for Reg128Bits<89> {}
impl Reg128BitsDownCast<86> for Reg128Bits<89> {}
impl Reg128BitsDownCast<87> for Reg128Bits<89> {}
impl Reg128BitsDownCast<88> for Reg128Bits<89> {}
impl Reg128BitsDownCast<89> for Reg128Bits<89> {}
impl Reg128BitsDownCast<1> for Reg128Bits<90> {}
impl Reg128BitsConcat<1, 91> for Reg128Bits<90> {}
impl Reg128BitsDownCast<2> for Reg128Bits<90> {}
impl Reg128BitsConcat<2, 92> for Reg128Bits<90> {}
impl Reg128BitsDownCast<3> for Reg128Bits<90> {}
impl Reg128BitsConcat<3, 93> for Reg128Bits<90> {}
impl Reg128BitsDownCast<4> for Reg128Bits<90> {}
impl Reg128BitsConcat<4, 94> for Reg128Bits<90> {}
impl Reg128BitsDownCast<5> for Reg128Bits<90> {}
impl Reg128BitsConcat<5, 95> for Reg128Bits<90> {}
impl Reg128BitsDownCast<6> for Reg128Bits<90> {}
impl Reg128BitsConcat<6, 96> for Reg128Bits<90> {}
impl Reg128BitsDownCast<7> for Reg128Bits<90> {}
impl Reg128BitsConcat<7, 97> for Reg128Bits<90> {}
impl Reg128BitsDownCast<8> for Reg128Bits<90> {}
impl Reg128BitsConcat<8, 98> for Reg128Bits<90> {}
impl Reg128BitsDownCast<9> for Reg128Bits<90> {}
impl Reg128BitsConcat<9, 99> for Reg128Bits<90> {}
impl Reg128BitsDownCast<10> for Reg128Bits<90> {}
impl Reg128BitsConcat<10, 100> for Reg128Bits<90> {}
impl Reg128BitsDownCast<11> for Reg128Bits<90> {}
impl Reg128BitsConcat<11, 101> for Reg128Bits<90> {}
impl Reg128BitsDownCast<12> for Reg128Bits<90> {}
impl Reg128BitsConcat<12, 102> for Reg128Bits<90> {}
impl Reg128BitsDownCast<13> for Reg128Bits<90> {}
impl Reg128BitsConcat<13, 103> for Reg128Bits<90> {}
impl Reg128BitsDownCast<14> for Reg128Bits<90> {}
impl Reg128BitsConcat<14, 104> for Reg128Bits<90> {}
impl Reg128BitsDownCast<15> for Reg128Bits<90> {}
impl Reg128BitsConcat<15, 105> for Reg128Bits<90> {}
impl Reg128BitsDownCast<16> for Reg128Bits<90> {}
impl Reg128BitsConcat<16, 106> for Reg128Bits<90> {}
impl Reg128BitsDownCast<17> for Reg128Bits<90> {}
impl Reg128BitsConcat<17, 107> for Reg128Bits<90> {}
impl Reg128BitsDownCast<18> for Reg128Bits<90> {}
impl Reg128BitsConcat<18, 108> for Reg128Bits<90> {}
impl Reg128BitsDownCast<19> for Reg128Bits<90> {}
impl Reg128BitsConcat<19, 109> for Reg128Bits<90> {}
impl Reg128BitsDownCast<20> for Reg128Bits<90> {}
impl Reg128BitsConcat<20, 110> for Reg128Bits<90> {}
impl Reg128BitsDownCast<21> for Reg128Bits<90> {}
impl Reg128BitsConcat<21, 111> for Reg128Bits<90> {}
impl Reg128BitsDownCast<22> for Reg128Bits<90> {}
impl Reg128BitsConcat<22, 112> for Reg128Bits<90> {}
impl Reg128BitsDownCast<23> for Reg128Bits<90> {}
impl Reg128BitsConcat<23, 113> for Reg128Bits<90> {}
impl Reg128BitsDownCast<24> for Reg128Bits<90> {}
impl Reg128BitsConcat<24, 114> for Reg128Bits<90> {}
impl Reg128BitsDownCast<25> for Reg128Bits<90> {}
impl Reg128BitsConcat<25, 115> for Reg128Bits<90> {}
impl Reg128BitsDownCast<26> for Reg128Bits<90> {}
impl Reg128BitsConcat<26, 116> for Reg128Bits<90> {}
impl Reg128BitsDownCast<27> for Reg128Bits<90> {}
impl Reg128BitsConcat<27, 117> for Reg128Bits<90> {}
impl Reg128BitsDownCast<28> for Reg128Bits<90> {}
impl Reg128BitsConcat<28, 118> for Reg128Bits<90> {}
impl Reg128BitsDownCast<29> for Reg128Bits<90> {}
impl Reg128BitsConcat<29, 119> for Reg128Bits<90> {}
impl Reg128BitsDownCast<30> for Reg128Bits<90> {}
impl Reg128BitsConcat<30, 120> for Reg128Bits<90> {}
impl Reg128BitsDownCast<31> for Reg128Bits<90> {}
impl Reg128BitsConcat<31, 121> for Reg128Bits<90> {}
impl Reg128BitsDownCast<32> for Reg128Bits<90> {}
impl Reg128BitsConcat<32, 122> for Reg128Bits<90> {}
impl Reg128BitsDownCast<33> for Reg128Bits<90> {}
impl Reg128BitsConcat<33, 123> for Reg128Bits<90> {}
impl Reg128BitsDownCast<34> for Reg128Bits<90> {}
impl Reg128BitsConcat<34, 124> for Reg128Bits<90> {}
impl Reg128BitsDownCast<35> for Reg128Bits<90> {}
impl Reg128BitsConcat<35, 125> for Reg128Bits<90> {}
impl Reg128BitsDownCast<36> for Reg128Bits<90> {}
impl Reg128BitsConcat<36, 126> for Reg128Bits<90> {}
impl Reg128BitsDownCast<37> for Reg128Bits<90> {}
impl Reg128BitsConcat<37, 127> for Reg128Bits<90> {}
impl Reg128BitsDownCast<38> for Reg128Bits<90> {}
impl Reg128BitsConcat<38, 128> for Reg128Bits<90> {}
impl Reg128BitsDownCast<39> for Reg128Bits<90> {}
impl Reg128BitsDownCast<40> for Reg128Bits<90> {}
impl Reg128BitsDownCast<41> for Reg128Bits<90> {}
impl Reg128BitsDownCast<42> for Reg128Bits<90> {}
impl Reg128BitsDownCast<43> for Reg128Bits<90> {}
impl Reg128BitsDownCast<44> for Reg128Bits<90> {}
impl Reg128BitsDownCast<45> for Reg128Bits<90> {}
impl Reg128BitsDownCast<46> for Reg128Bits<90> {}
impl Reg128BitsDownCast<47> for Reg128Bits<90> {}
impl Reg128BitsDownCast<48> for Reg128Bits<90> {}
impl Reg128BitsDownCast<49> for Reg128Bits<90> {}
impl Reg128BitsDownCast<50> for Reg128Bits<90> {}
impl Reg128BitsDownCast<51> for Reg128Bits<90> {}
impl Reg128BitsDownCast<52> for Reg128Bits<90> {}
impl Reg128BitsDownCast<53> for Reg128Bits<90> {}
impl Reg128BitsDownCast<54> for Reg128Bits<90> {}
impl Reg128BitsDownCast<55> for Reg128Bits<90> {}
impl Reg128BitsDownCast<56> for Reg128Bits<90> {}
impl Reg128BitsDownCast<57> for Reg128Bits<90> {}
impl Reg128BitsDownCast<58> for Reg128Bits<90> {}
impl Reg128BitsDownCast<59> for Reg128Bits<90> {}
impl Reg128BitsDownCast<60> for Reg128Bits<90> {}
impl Reg128BitsDownCast<61> for Reg128Bits<90> {}
impl Reg128BitsDownCast<62> for Reg128Bits<90> {}
impl Reg128BitsDownCast<63> for Reg128Bits<90> {}
impl Reg128BitsDownCast<64> for Reg128Bits<90> {}
impl Reg128BitsDownCast<65> for Reg128Bits<90> {}
impl Reg128BitsDownCast<66> for Reg128Bits<90> {}
impl Reg128BitsDownCast<67> for Reg128Bits<90> {}
impl Reg128BitsDownCast<68> for Reg128Bits<90> {}
impl Reg128BitsDownCast<69> for Reg128Bits<90> {}
impl Reg128BitsDownCast<70> for Reg128Bits<90> {}
impl Reg128BitsDownCast<71> for Reg128Bits<90> {}
impl Reg128BitsDownCast<72> for Reg128Bits<90> {}
impl Reg128BitsDownCast<73> for Reg128Bits<90> {}
impl Reg128BitsDownCast<74> for Reg128Bits<90> {}
impl Reg128BitsDownCast<75> for Reg128Bits<90> {}
impl Reg128BitsDownCast<76> for Reg128Bits<90> {}
impl Reg128BitsDownCast<77> for Reg128Bits<90> {}
impl Reg128BitsDownCast<78> for Reg128Bits<90> {}
impl Reg128BitsDownCast<79> for Reg128Bits<90> {}
impl Reg128BitsDownCast<80> for Reg128Bits<90> {}
impl Reg128BitsDownCast<81> for Reg128Bits<90> {}
impl Reg128BitsDownCast<82> for Reg128Bits<90> {}
impl Reg128BitsDownCast<83> for Reg128Bits<90> {}
impl Reg128BitsDownCast<84> for Reg128Bits<90> {}
impl Reg128BitsDownCast<85> for Reg128Bits<90> {}
impl Reg128BitsDownCast<86> for Reg128Bits<90> {}
impl Reg128BitsDownCast<87> for Reg128Bits<90> {}
impl Reg128BitsDownCast<88> for Reg128Bits<90> {}
impl Reg128BitsDownCast<89> for Reg128Bits<90> {}
impl Reg128BitsDownCast<90> for Reg128Bits<90> {}
impl Reg128BitsDownCast<1> for Reg128Bits<91> {}
impl Reg128BitsConcat<1, 92> for Reg128Bits<91> {}
impl Reg128BitsDownCast<2> for Reg128Bits<91> {}
impl Reg128BitsConcat<2, 93> for Reg128Bits<91> {}
impl Reg128BitsDownCast<3> for Reg128Bits<91> {}
impl Reg128BitsConcat<3, 94> for Reg128Bits<91> {}
impl Reg128BitsDownCast<4> for Reg128Bits<91> {}
impl Reg128BitsConcat<4, 95> for Reg128Bits<91> {}
impl Reg128BitsDownCast<5> for Reg128Bits<91> {}
impl Reg128BitsConcat<5, 96> for Reg128Bits<91> {}
impl Reg128BitsDownCast<6> for Reg128Bits<91> {}
impl Reg128BitsConcat<6, 97> for Reg128Bits<91> {}
impl Reg128BitsDownCast<7> for Reg128Bits<91> {}
impl Reg128BitsConcat<7, 98> for Reg128Bits<91> {}
impl Reg128BitsDownCast<8> for Reg128Bits<91> {}
impl Reg128BitsConcat<8, 99> for Reg128Bits<91> {}
impl Reg128BitsDownCast<9> for Reg128Bits<91> {}
impl Reg128BitsConcat<9, 100> for Reg128Bits<91> {}
impl Reg128BitsDownCast<10> for Reg128Bits<91> {}
impl Reg128BitsConcat<10, 101> for Reg128Bits<91> {}
impl Reg128BitsDownCast<11> for Reg128Bits<91> {}
impl Reg128BitsConcat<11, 102> for Reg128Bits<91> {}
impl Reg128BitsDownCast<12> for Reg128Bits<91> {}
impl Reg128BitsConcat<12, 103> for Reg128Bits<91> {}
impl Reg128BitsDownCast<13> for Reg128Bits<91> {}
impl Reg128BitsConcat<13, 104> for Reg128Bits<91> {}
impl Reg128BitsDownCast<14> for Reg128Bits<91> {}
impl Reg128BitsConcat<14, 105> for Reg128Bits<91> {}
impl Reg128BitsDownCast<15> for Reg128Bits<91> {}
impl Reg128BitsConcat<15, 106> for Reg128Bits<91> {}
impl Reg128BitsDownCast<16> for Reg128Bits<91> {}
impl Reg128BitsConcat<16, 107> for Reg128Bits<91> {}
impl Reg128BitsDownCast<17> for Reg128Bits<91> {}
impl Reg128BitsConcat<17, 108> for Reg128Bits<91> {}
impl Reg128BitsDownCast<18> for Reg128Bits<91> {}
impl Reg128BitsConcat<18, 109> for Reg128Bits<91> {}
impl Reg128BitsDownCast<19> for Reg128Bits<91> {}
impl Reg128BitsConcat<19, 110> for Reg128Bits<91> {}
impl Reg128BitsDownCast<20> for Reg128Bits<91> {}
impl Reg128BitsConcat<20, 111> for Reg128Bits<91> {}
impl Reg128BitsDownCast<21> for Reg128Bits<91> {}
impl Reg128BitsConcat<21, 112> for Reg128Bits<91> {}
impl Reg128BitsDownCast<22> for Reg128Bits<91> {}
impl Reg128BitsConcat<22, 113> for Reg128Bits<91> {}
impl Reg128BitsDownCast<23> for Reg128Bits<91> {}
impl Reg128BitsConcat<23, 114> for Reg128Bits<91> {}
impl Reg128BitsDownCast<24> for Reg128Bits<91> {}
impl Reg128BitsConcat<24, 115> for Reg128Bits<91> {}
impl Reg128BitsDownCast<25> for Reg128Bits<91> {}
impl Reg128BitsConcat<25, 116> for Reg128Bits<91> {}
impl Reg128BitsDownCast<26> for Reg128Bits<91> {}
impl Reg128BitsConcat<26, 117> for Reg128Bits<91> {}
impl Reg128BitsDownCast<27> for Reg128Bits<91> {}
impl Reg128BitsConcat<27, 118> for Reg128Bits<91> {}
impl Reg128BitsDownCast<28> for Reg128Bits<91> {}
impl Reg128BitsConcat<28, 119> for Reg128Bits<91> {}
impl Reg128BitsDownCast<29> for Reg128Bits<91> {}
impl Reg128BitsConcat<29, 120> for Reg128Bits<91> {}
impl Reg128BitsDownCast<30> for Reg128Bits<91> {}
impl Reg128BitsConcat<30, 121> for Reg128Bits<91> {}
impl Reg128BitsDownCast<31> for Reg128Bits<91> {}
impl Reg128BitsConcat<31, 122> for Reg128Bits<91> {}
impl Reg128BitsDownCast<32> for Reg128Bits<91> {}
impl Reg128BitsConcat<32, 123> for Reg128Bits<91> {}
impl Reg128BitsDownCast<33> for Reg128Bits<91> {}
impl Reg128BitsConcat<33, 124> for Reg128Bits<91> {}
impl Reg128BitsDownCast<34> for Reg128Bits<91> {}
impl Reg128BitsConcat<34, 125> for Reg128Bits<91> {}
impl Reg128BitsDownCast<35> for Reg128Bits<91> {}
impl Reg128BitsConcat<35, 126> for Reg128Bits<91> {}
impl Reg128BitsDownCast<36> for Reg128Bits<91> {}
impl Reg128BitsConcat<36, 127> for Reg128Bits<91> {}
impl Reg128BitsDownCast<37> for Reg128Bits<91> {}
impl Reg128BitsConcat<37, 128> for Reg128Bits<91> {}
impl Reg128BitsDownCast<38> for Reg128Bits<91> {}
impl Reg128BitsDownCast<39> for Reg128Bits<91> {}
impl Reg128BitsDownCast<40> for Reg128Bits<91> {}
impl Reg128BitsDownCast<41> for Reg128Bits<91> {}
impl Reg128BitsDownCast<42> for Reg128Bits<91> {}
impl Reg128BitsDownCast<43> for Reg128Bits<91> {}
impl Reg128BitsDownCast<44> for Reg128Bits<91> {}
impl Reg128BitsDownCast<45> for Reg128Bits<91> {}
impl Reg128BitsDownCast<46> for Reg128Bits<91> {}
impl Reg128BitsDownCast<47> for Reg128Bits<91> {}
impl Reg128BitsDownCast<48> for Reg128Bits<91> {}
impl Reg128BitsDownCast<49> for Reg128Bits<91> {}
impl Reg128BitsDownCast<50> for Reg128Bits<91> {}
impl Reg128BitsDownCast<51> for Reg128Bits<91> {}
impl Reg128BitsDownCast<52> for Reg128Bits<91> {}
impl Reg128BitsDownCast<53> for Reg128Bits<91> {}
impl Reg128BitsDownCast<54> for Reg128Bits<91> {}
impl Reg128BitsDownCast<55> for Reg128Bits<91> {}
impl Reg128BitsDownCast<56> for Reg128Bits<91> {}
impl Reg128BitsDownCast<57> for Reg128Bits<91> {}
impl Reg128BitsDownCast<58> for Reg128Bits<91> {}
impl Reg128BitsDownCast<59> for Reg128Bits<91> {}
impl Reg128BitsDownCast<60> for Reg128Bits<91> {}
impl Reg128BitsDownCast<61> for Reg128Bits<91> {}
impl Reg128BitsDownCast<62> for Reg128Bits<91> {}
impl Reg128BitsDownCast<63> for Reg128Bits<91> {}
impl Reg128BitsDownCast<64> for Reg128Bits<91> {}
impl Reg128BitsDownCast<65> for Reg128Bits<91> {}
impl Reg128BitsDownCast<66> for Reg128Bits<91> {}
impl Reg128BitsDownCast<67> for Reg128Bits<91> {}
impl Reg128BitsDownCast<68> for Reg128Bits<91> {}
impl Reg128BitsDownCast<69> for Reg128Bits<91> {}
impl Reg128BitsDownCast<70> for Reg128Bits<91> {}
impl Reg128BitsDownCast<71> for Reg128Bits<91> {}
impl Reg128BitsDownCast<72> for Reg128Bits<91> {}
impl Reg128BitsDownCast<73> for Reg128Bits<91> {}
impl Reg128BitsDownCast<74> for Reg128Bits<91> {}
impl Reg128BitsDownCast<75> for Reg128Bits<91> {}
impl Reg128BitsDownCast<76> for Reg128Bits<91> {}
impl Reg128BitsDownCast<77> for Reg128Bits<91> {}
impl Reg128BitsDownCast<78> for Reg128Bits<91> {}
impl Reg128BitsDownCast<79> for Reg128Bits<91> {}
impl Reg128BitsDownCast<80> for Reg128Bits<91> {}
impl Reg128BitsDownCast<81> for Reg128Bits<91> {}
impl Reg128BitsDownCast<82> for Reg128Bits<91> {}
impl Reg128BitsDownCast<83> for Reg128Bits<91> {}
impl Reg128BitsDownCast<84> for Reg128Bits<91> {}
impl Reg128BitsDownCast<85> for Reg128Bits<91> {}
impl Reg128BitsDownCast<86> for Reg128Bits<91> {}
impl Reg128BitsDownCast<87> for Reg128Bits<91> {}
impl Reg128BitsDownCast<88> for Reg128Bits<91> {}
impl Reg128BitsDownCast<89> for Reg128Bits<91> {}
impl Reg128BitsDownCast<90> for Reg128Bits<91> {}
impl Reg128BitsDownCast<91> for Reg128Bits<91> {}
impl Reg128BitsDownCast<1> for Reg128Bits<92> {}
impl Reg128BitsConcat<1, 93> for Reg128Bits<92> {}
impl Reg128BitsDownCast<2> for Reg128Bits<92> {}
impl Reg128BitsConcat<2, 94> for Reg128Bits<92> {}
impl Reg128BitsDownCast<3> for Reg128Bits<92> {}
impl Reg128BitsConcat<3, 95> for Reg128Bits<92> {}
impl Reg128BitsDownCast<4> for Reg128Bits<92> {}
impl Reg128BitsConcat<4, 96> for Reg128Bits<92> {}
impl Reg128BitsDownCast<5> for Reg128Bits<92> {}
impl Reg128BitsConcat<5, 97> for Reg128Bits<92> {}
impl Reg128BitsDownCast<6> for Reg128Bits<92> {}
impl Reg128BitsConcat<6, 98> for Reg128Bits<92> {}
impl Reg128BitsDownCast<7> for Reg128Bits<92> {}
impl Reg128BitsConcat<7, 99> for Reg128Bits<92> {}
impl Reg128BitsDownCast<8> for Reg128Bits<92> {}
impl Reg128BitsConcat<8, 100> for Reg128Bits<92> {}
impl Reg128BitsDownCast<9> for Reg128Bits<92> {}
impl Reg128BitsConcat<9, 101> for Reg128Bits<92> {}
impl Reg128BitsDownCast<10> for Reg128Bits<92> {}
impl Reg128BitsConcat<10, 102> for Reg128Bits<92> {}
impl Reg128BitsDownCast<11> for Reg128Bits<92> {}
impl Reg128BitsConcat<11, 103> for Reg128Bits<92> {}
impl Reg128BitsDownCast<12> for Reg128Bits<92> {}
impl Reg128BitsConcat<12, 104> for Reg128Bits<92> {}
impl Reg128BitsDownCast<13> for Reg128Bits<92> {}
impl Reg128BitsConcat<13, 105> for Reg128Bits<92> {}
impl Reg128BitsDownCast<14> for Reg128Bits<92> {}
impl Reg128BitsConcat<14, 106> for Reg128Bits<92> {}
impl Reg128BitsDownCast<15> for Reg128Bits<92> {}
impl Reg128BitsConcat<15, 107> for Reg128Bits<92> {}
impl Reg128BitsDownCast<16> for Reg128Bits<92> {}
impl Reg128BitsConcat<16, 108> for Reg128Bits<92> {}
impl Reg128BitsDownCast<17> for Reg128Bits<92> {}
impl Reg128BitsConcat<17, 109> for Reg128Bits<92> {}
impl Reg128BitsDownCast<18> for Reg128Bits<92> {}
impl Reg128BitsConcat<18, 110> for Reg128Bits<92> {}
impl Reg128BitsDownCast<19> for Reg128Bits<92> {}
impl Reg128BitsConcat<19, 111> for Reg128Bits<92> {}
impl Reg128BitsDownCast<20> for Reg128Bits<92> {}
impl Reg128BitsConcat<20, 112> for Reg128Bits<92> {}
impl Reg128BitsDownCast<21> for Reg128Bits<92> {}
impl Reg128BitsConcat<21, 113> for Reg128Bits<92> {}
impl Reg128BitsDownCast<22> for Reg128Bits<92> {}
impl Reg128BitsConcat<22, 114> for Reg128Bits<92> {}
impl Reg128BitsDownCast<23> for Reg128Bits<92> {}
impl Reg128BitsConcat<23, 115> for Reg128Bits<92> {}
impl Reg128BitsDownCast<24> for Reg128Bits<92> {}
impl Reg128BitsConcat<24, 116> for Reg128Bits<92> {}
impl Reg128BitsDownCast<25> for Reg128Bits<92> {}
impl Reg128BitsConcat<25, 117> for Reg128Bits<92> {}
impl Reg128BitsDownCast<26> for Reg128Bits<92> {}
impl Reg128BitsConcat<26, 118> for Reg128Bits<92> {}
impl Reg128BitsDownCast<27> for Reg128Bits<92> {}
impl Reg128BitsConcat<27, 119> for Reg128Bits<92> {}
impl Reg128BitsDownCast<28> for Reg128Bits<92> {}
impl Reg128BitsConcat<28, 120> for Reg128Bits<92> {}
impl Reg128BitsDownCast<29> for Reg128Bits<92> {}
impl Reg128BitsConcat<29, 121> for Reg128Bits<92> {}
impl Reg128BitsDownCast<30> for Reg128Bits<92> {}
impl Reg128BitsConcat<30, 122> for Reg128Bits<92> {}
impl Reg128BitsDownCast<31> for Reg128Bits<92> {}
impl Reg128BitsConcat<31, 123> for Reg128Bits<92> {}
impl Reg128BitsDownCast<32> for Reg128Bits<92> {}
impl Reg128BitsConcat<32, 124> for Reg128Bits<92> {}
impl Reg128BitsDownCast<33> for Reg128Bits<92> {}
impl Reg128BitsConcat<33, 125> for Reg128Bits<92> {}
impl Reg128BitsDownCast<34> for Reg128Bits<92> {}
impl Reg128BitsConcat<34, 126> for Reg128Bits<92> {}
impl Reg128BitsDownCast<35> for Reg128Bits<92> {}
impl Reg128BitsConcat<35, 127> for Reg128Bits<92> {}
impl Reg128BitsDownCast<36> for Reg128Bits<92> {}
impl Reg128BitsConcat<36, 128> for Reg128Bits<92> {}
impl Reg128BitsDownCast<37> for Reg128Bits<92> {}
impl Reg128BitsDownCast<38> for Reg128Bits<92> {}
impl Reg128BitsDownCast<39> for Reg128Bits<92> {}
impl Reg128BitsDownCast<40> for Reg128Bits<92> {}
impl Reg128BitsDownCast<41> for Reg128Bits<92> {}
impl Reg128BitsDownCast<42> for Reg128Bits<92> {}
impl Reg128BitsDownCast<43> for Reg128Bits<92> {}
impl Reg128BitsDownCast<44> for Reg128Bits<92> {}
impl Reg128BitsDownCast<45> for Reg128Bits<92> {}
impl Reg128BitsDownCast<46> for Reg128Bits<92> {}
impl Reg128BitsDownCast<47> for Reg128Bits<92> {}
impl Reg128BitsDownCast<48> for Reg128Bits<92> {}
impl Reg128BitsDownCast<49> for Reg128Bits<92> {}
impl Reg128BitsDownCast<50> for Reg128Bits<92> {}
impl Reg128BitsDownCast<51> for Reg128Bits<92> {}
impl Reg128BitsDownCast<52> for Reg128Bits<92> {}
impl Reg128BitsDownCast<53> for Reg128Bits<92> {}
impl Reg128BitsDownCast<54> for Reg128Bits<92> {}
impl Reg128BitsDownCast<55> for Reg128Bits<92> {}
impl Reg128BitsDownCast<56> for Reg128Bits<92> {}
impl Reg128BitsDownCast<57> for Reg128Bits<92> {}
impl Reg128BitsDownCast<58> for Reg128Bits<92> {}
impl Reg128BitsDownCast<59> for Reg128Bits<92> {}
impl Reg128BitsDownCast<60> for Reg128Bits<92> {}
impl Reg128BitsDownCast<61> for Reg128Bits<92> {}
impl Reg128BitsDownCast<62> for Reg128Bits<92> {}
impl Reg128BitsDownCast<63> for Reg128Bits<92> {}
impl Reg128BitsDownCast<64> for Reg128Bits<92> {}
impl Reg128BitsDownCast<65> for Reg128Bits<92> {}
impl Reg128BitsDownCast<66> for Reg128Bits<92> {}
impl Reg128BitsDownCast<67> for Reg128Bits<92> {}
impl Reg128BitsDownCast<68> for Reg128Bits<92> {}
impl Reg128BitsDownCast<69> for Reg128Bits<92> {}
impl Reg128BitsDownCast<70> for Reg128Bits<92> {}
impl Reg128BitsDownCast<71> for Reg128Bits<92> {}
impl Reg128BitsDownCast<72> for Reg128Bits<92> {}
impl Reg128BitsDownCast<73> for Reg128Bits<92> {}
impl Reg128BitsDownCast<74> for Reg128Bits<92> {}
impl Reg128BitsDownCast<75> for Reg128Bits<92> {}
impl Reg128BitsDownCast<76> for Reg128Bits<92> {}
impl Reg128BitsDownCast<77> for Reg128Bits<92> {}
impl Reg128BitsDownCast<78> for Reg128Bits<92> {}
impl Reg128BitsDownCast<79> for Reg128Bits<92> {}
impl Reg128BitsDownCast<80> for Reg128Bits<92> {}
impl Reg128BitsDownCast<81> for Reg128Bits<92> {}
impl Reg128BitsDownCast<82> for Reg128Bits<92> {}
impl Reg128BitsDownCast<83> for Reg128Bits<92> {}
impl Reg128BitsDownCast<84> for Reg128Bits<92> {}
impl Reg128BitsDownCast<85> for Reg128Bits<92> {}
impl Reg128BitsDownCast<86> for Reg128Bits<92> {}
impl Reg128BitsDownCast<87> for Reg128Bits<92> {}
impl Reg128BitsDownCast<88> for Reg128Bits<92> {}
impl Reg128BitsDownCast<89> for Reg128Bits<92> {}
impl Reg128BitsDownCast<90> for Reg128Bits<92> {}
impl Reg128BitsDownCast<91> for Reg128Bits<92> {}
impl Reg128BitsDownCast<92> for Reg128Bits<92> {}
impl Reg128BitsDownCast<1> for Reg128Bits<93> {}
impl Reg128BitsConcat<1, 94> for Reg128Bits<93> {}
impl Reg128BitsDownCast<2> for Reg128Bits<93> {}
impl Reg128BitsConcat<2, 95> for Reg128Bits<93> {}
impl Reg128BitsDownCast<3> for Reg128Bits<93> {}
impl Reg128BitsConcat<3, 96> for Reg128Bits<93> {}
impl Reg128BitsDownCast<4> for Reg128Bits<93> {}
impl Reg128BitsConcat<4, 97> for Reg128Bits<93> {}
impl Reg128BitsDownCast<5> for Reg128Bits<93> {}
impl Reg128BitsConcat<5, 98> for Reg128Bits<93> {}
impl Reg128BitsDownCast<6> for Reg128Bits<93> {}
impl Reg128BitsConcat<6, 99> for Reg128Bits<93> {}
impl Reg128BitsDownCast<7> for Reg128Bits<93> {}
impl Reg128BitsConcat<7, 100> for Reg128Bits<93> {}
impl Reg128BitsDownCast<8> for Reg128Bits<93> {}
impl Reg128BitsConcat<8, 101> for Reg128Bits<93> {}
impl Reg128BitsDownCast<9> for Reg128Bits<93> {}
impl Reg128BitsConcat<9, 102> for Reg128Bits<93> {}
impl Reg128BitsDownCast<10> for Reg128Bits<93> {}
impl Reg128BitsConcat<10, 103> for Reg128Bits<93> {}
impl Reg128BitsDownCast<11> for Reg128Bits<93> {}
impl Reg128BitsConcat<11, 104> for Reg128Bits<93> {}
impl Reg128BitsDownCast<12> for Reg128Bits<93> {}
impl Reg128BitsConcat<12, 105> for Reg128Bits<93> {}
impl Reg128BitsDownCast<13> for Reg128Bits<93> {}
impl Reg128BitsConcat<13, 106> for Reg128Bits<93> {}
impl Reg128BitsDownCast<14> for Reg128Bits<93> {}
impl Reg128BitsConcat<14, 107> for Reg128Bits<93> {}
impl Reg128BitsDownCast<15> for Reg128Bits<93> {}
impl Reg128BitsConcat<15, 108> for Reg128Bits<93> {}
impl Reg128BitsDownCast<16> for Reg128Bits<93> {}
impl Reg128BitsConcat<16, 109> for Reg128Bits<93> {}
impl Reg128BitsDownCast<17> for Reg128Bits<93> {}
impl Reg128BitsConcat<17, 110> for Reg128Bits<93> {}
impl Reg128BitsDownCast<18> for Reg128Bits<93> {}
impl Reg128BitsConcat<18, 111> for Reg128Bits<93> {}
impl Reg128BitsDownCast<19> for Reg128Bits<93> {}
impl Reg128BitsConcat<19, 112> for Reg128Bits<93> {}
impl Reg128BitsDownCast<20> for Reg128Bits<93> {}
impl Reg128BitsConcat<20, 113> for Reg128Bits<93> {}
impl Reg128BitsDownCast<21> for Reg128Bits<93> {}
impl Reg128BitsConcat<21, 114> for Reg128Bits<93> {}
impl Reg128BitsDownCast<22> for Reg128Bits<93> {}
impl Reg128BitsConcat<22, 115> for Reg128Bits<93> {}
impl Reg128BitsDownCast<23> for Reg128Bits<93> {}
impl Reg128BitsConcat<23, 116> for Reg128Bits<93> {}
impl Reg128BitsDownCast<24> for Reg128Bits<93> {}
impl Reg128BitsConcat<24, 117> for Reg128Bits<93> {}
impl Reg128BitsDownCast<25> for Reg128Bits<93> {}
impl Reg128BitsConcat<25, 118> for Reg128Bits<93> {}
impl Reg128BitsDownCast<26> for Reg128Bits<93> {}
impl Reg128BitsConcat<26, 119> for Reg128Bits<93> {}
impl Reg128BitsDownCast<27> for Reg128Bits<93> {}
impl Reg128BitsConcat<27, 120> for Reg128Bits<93> {}
impl Reg128BitsDownCast<28> for Reg128Bits<93> {}
impl Reg128BitsConcat<28, 121> for Reg128Bits<93> {}
impl Reg128BitsDownCast<29> for Reg128Bits<93> {}
impl Reg128BitsConcat<29, 122> for Reg128Bits<93> {}
impl Reg128BitsDownCast<30> for Reg128Bits<93> {}
impl Reg128BitsConcat<30, 123> for Reg128Bits<93> {}
impl Reg128BitsDownCast<31> for Reg128Bits<93> {}
impl Reg128BitsConcat<31, 124> for Reg128Bits<93> {}
impl Reg128BitsDownCast<32> for Reg128Bits<93> {}
impl Reg128BitsConcat<32, 125> for Reg128Bits<93> {}
impl Reg128BitsDownCast<33> for Reg128Bits<93> {}
impl Reg128BitsConcat<33, 126> for Reg128Bits<93> {}
impl Reg128BitsDownCast<34> for Reg128Bits<93> {}
impl Reg128BitsConcat<34, 127> for Reg128Bits<93> {}
impl Reg128BitsDownCast<35> for Reg128Bits<93> {}
impl Reg128BitsConcat<35, 128> for Reg128Bits<93> {}
impl Reg128BitsDownCast<36> for Reg128Bits<93> {}
impl Reg128BitsDownCast<37> for Reg128Bits<93> {}
impl Reg128BitsDownCast<38> for Reg128Bits<93> {}
impl Reg128BitsDownCast<39> for Reg128Bits<93> {}
impl Reg128BitsDownCast<40> for Reg128Bits<93> {}
impl Reg128BitsDownCast<41> for Reg128Bits<93> {}
impl Reg128BitsDownCast<42> for Reg128Bits<93> {}
impl Reg128BitsDownCast<43> for Reg128Bits<93> {}
impl Reg128BitsDownCast<44> for Reg128Bits<93> {}
impl Reg128BitsDownCast<45> for Reg128Bits<93> {}
impl Reg128BitsDownCast<46> for Reg128Bits<93> {}
impl Reg128BitsDownCast<47> for Reg128Bits<93> {}
impl Reg128BitsDownCast<48> for Reg128Bits<93> {}
impl Reg128BitsDownCast<49> for Reg128Bits<93> {}
impl Reg128BitsDownCast<50> for Reg128Bits<93> {}
impl Reg128BitsDownCast<51> for Reg128Bits<93> {}
impl Reg128BitsDownCast<52> for Reg128Bits<93> {}
impl Reg128BitsDownCast<53> for Reg128Bits<93> {}
impl Reg128BitsDownCast<54> for Reg128Bits<93> {}
impl Reg128BitsDownCast<55> for Reg128Bits<93> {}
impl Reg128BitsDownCast<56> for Reg128Bits<93> {}
impl Reg128BitsDownCast<57> for Reg128Bits<93> {}
impl Reg128BitsDownCast<58> for Reg128Bits<93> {}
impl Reg128BitsDownCast<59> for Reg128Bits<93> {}
impl Reg128BitsDownCast<60> for Reg128Bits<93> {}
impl Reg128BitsDownCast<61> for Reg128Bits<93> {}
impl Reg128BitsDownCast<62> for Reg128Bits<93> {}
impl Reg128BitsDownCast<63> for Reg128Bits<93> {}
impl Reg128BitsDownCast<64> for Reg128Bits<93> {}
impl Reg128BitsDownCast<65> for Reg128Bits<93> {}
impl Reg128BitsDownCast<66> for Reg128Bits<93> {}
impl Reg128BitsDownCast<67> for Reg128Bits<93> {}
impl Reg128BitsDownCast<68> for Reg128Bits<93> {}
impl Reg128BitsDownCast<69> for Reg128Bits<93> {}
impl Reg128BitsDownCast<70> for Reg128Bits<93> {}
impl Reg128BitsDownCast<71> for Reg128Bits<93> {}
impl Reg128BitsDownCast<72> for Reg128Bits<93> {}
impl Reg128BitsDownCast<73> for Reg128Bits<93> {}
impl Reg128BitsDownCast<74> for Reg128Bits<93> {}
impl Reg128BitsDownCast<75> for Reg128Bits<93> {}
impl Reg128BitsDownCast<76> for Reg128Bits<93> {}
impl Reg128BitsDownCast<77> for Reg128Bits<93> {}
impl Reg128BitsDownCast<78> for Reg128Bits<93> {}
impl Reg128BitsDownCast<79> for Reg128Bits<93> {}
impl Reg128BitsDownCast<80> for Reg128Bits<93> {}
impl Reg128BitsDownCast<81> for Reg128Bits<93> {}
impl Reg128BitsDownCast<82> for Reg128Bits<93> {}
impl Reg128BitsDownCast<83> for Reg128Bits<93> {}
impl Reg128BitsDownCast<84> for Reg128Bits<93> {}
impl Reg128BitsDownCast<85> for Reg128Bits<93> {}
impl Reg128BitsDownCast<86> for Reg128Bits<93> {}
impl Reg128BitsDownCast<87> for Reg128Bits<93> {}
impl Reg128BitsDownCast<88> for Reg128Bits<93> {}
impl Reg128BitsDownCast<89> for Reg128Bits<93> {}
impl Reg128BitsDownCast<90> for Reg128Bits<93> {}
impl Reg128BitsDownCast<91> for Reg128Bits<93> {}
impl Reg128BitsDownCast<92> for Reg128Bits<93> {}
impl Reg128BitsDownCast<93> for Reg128Bits<93> {}
impl Reg128BitsDownCast<1> for Reg128Bits<94> {}
impl Reg128BitsConcat<1, 95> for Reg128Bits<94> {}
impl Reg128BitsDownCast<2> for Reg128Bits<94> {}
impl Reg128BitsConcat<2, 96> for Reg128Bits<94> {}
impl Reg128BitsDownCast<3> for Reg128Bits<94> {}
impl Reg128BitsConcat<3, 97> for Reg128Bits<94> {}
impl Reg128BitsDownCast<4> for Reg128Bits<94> {}
impl Reg128BitsConcat<4, 98> for Reg128Bits<94> {}
impl Reg128BitsDownCast<5> for Reg128Bits<94> {}
impl Reg128BitsConcat<5, 99> for Reg128Bits<94> {}
impl Reg128BitsDownCast<6> for Reg128Bits<94> {}
impl Reg128BitsConcat<6, 100> for Reg128Bits<94> {}
impl Reg128BitsDownCast<7> for Reg128Bits<94> {}
impl Reg128BitsConcat<7, 101> for Reg128Bits<94> {}
impl Reg128BitsDownCast<8> for Reg128Bits<94> {}
impl Reg128BitsConcat<8, 102> for Reg128Bits<94> {}
impl Reg128BitsDownCast<9> for Reg128Bits<94> {}
impl Reg128BitsConcat<9, 103> for Reg128Bits<94> {}
impl Reg128BitsDownCast<10> for Reg128Bits<94> {}
impl Reg128BitsConcat<10, 104> for Reg128Bits<94> {}
impl Reg128BitsDownCast<11> for Reg128Bits<94> {}
impl Reg128BitsConcat<11, 105> for Reg128Bits<94> {}
impl Reg128BitsDownCast<12> for Reg128Bits<94> {}
impl Reg128BitsConcat<12, 106> for Reg128Bits<94> {}
impl Reg128BitsDownCast<13> for Reg128Bits<94> {}
impl Reg128BitsConcat<13, 107> for Reg128Bits<94> {}
impl Reg128BitsDownCast<14> for Reg128Bits<94> {}
impl Reg128BitsConcat<14, 108> for Reg128Bits<94> {}
impl Reg128BitsDownCast<15> for Reg128Bits<94> {}
impl Reg128BitsConcat<15, 109> for Reg128Bits<94> {}
impl Reg128BitsDownCast<16> for Reg128Bits<94> {}
impl Reg128BitsConcat<16, 110> for Reg128Bits<94> {}
impl Reg128BitsDownCast<17> for Reg128Bits<94> {}
impl Reg128BitsConcat<17, 111> for Reg128Bits<94> {}
impl Reg128BitsDownCast<18> for Reg128Bits<94> {}
impl Reg128BitsConcat<18, 112> for Reg128Bits<94> {}
impl Reg128BitsDownCast<19> for Reg128Bits<94> {}
impl Reg128BitsConcat<19, 113> for Reg128Bits<94> {}
impl Reg128BitsDownCast<20> for Reg128Bits<94> {}
impl Reg128BitsConcat<20, 114> for Reg128Bits<94> {}
impl Reg128BitsDownCast<21> for Reg128Bits<94> {}
impl Reg128BitsConcat<21, 115> for Reg128Bits<94> {}
impl Reg128BitsDownCast<22> for Reg128Bits<94> {}
impl Reg128BitsConcat<22, 116> for Reg128Bits<94> {}
impl Reg128BitsDownCast<23> for Reg128Bits<94> {}
impl Reg128BitsConcat<23, 117> for Reg128Bits<94> {}
impl Reg128BitsDownCast<24> for Reg128Bits<94> {}
impl Reg128BitsConcat<24, 118> for Reg128Bits<94> {}
impl Reg128BitsDownCast<25> for Reg128Bits<94> {}
impl Reg128BitsConcat<25, 119> for Reg128Bits<94> {}
impl Reg128BitsDownCast<26> for Reg128Bits<94> {}
impl Reg128BitsConcat<26, 120> for Reg128Bits<94> {}
impl Reg128BitsDownCast<27> for Reg128Bits<94> {}
impl Reg128BitsConcat<27, 121> for Reg128Bits<94> {}
impl Reg128BitsDownCast<28> for Reg128Bits<94> {}
impl Reg128BitsConcat<28, 122> for Reg128Bits<94> {}
impl Reg128BitsDownCast<29> for Reg128Bits<94> {}
impl Reg128BitsConcat<29, 123> for Reg128Bits<94> {}
impl Reg128BitsDownCast<30> for Reg128Bits<94> {}
impl Reg128BitsConcat<30, 124> for Reg128Bits<94> {}
impl Reg128BitsDownCast<31> for Reg128Bits<94> {}
impl Reg128BitsConcat<31, 125> for Reg128Bits<94> {}
impl Reg128BitsDownCast<32> for Reg128Bits<94> {}
impl Reg128BitsConcat<32, 126> for Reg128Bits<94> {}
impl Reg128BitsDownCast<33> for Reg128Bits<94> {}
impl Reg128BitsConcat<33, 127> for Reg128Bits<94> {}
impl Reg128BitsDownCast<34> for Reg128Bits<94> {}
impl Reg128BitsConcat<34, 128> for Reg128Bits<94> {}
impl Reg128BitsDownCast<35> for Reg128Bits<94> {}
impl Reg128BitsDownCast<36> for Reg128Bits<94> {}
impl Reg128BitsDownCast<37> for Reg128Bits<94> {}
impl Reg128BitsDownCast<38> for Reg128Bits<94> {}
impl Reg128BitsDownCast<39> for Reg128Bits<94> {}
impl Reg128BitsDownCast<40> for Reg128Bits<94> {}
impl Reg128BitsDownCast<41> for Reg128Bits<94> {}
impl Reg128BitsDownCast<42> for Reg128Bits<94> {}
impl Reg128BitsDownCast<43> for Reg128Bits<94> {}
impl Reg128BitsDownCast<44> for Reg128Bits<94> {}
impl Reg128BitsDownCast<45> for Reg128Bits<94> {}
impl Reg128BitsDownCast<46> for Reg128Bits<94> {}
impl Reg128BitsDownCast<47> for Reg128Bits<94> {}
impl Reg128BitsDownCast<48> for Reg128Bits<94> {}
impl Reg128BitsDownCast<49> for Reg128Bits<94> {}
impl Reg128BitsDownCast<50> for Reg128Bits<94> {}
impl Reg128BitsDownCast<51> for Reg128Bits<94> {}
impl Reg128BitsDownCast<52> for Reg128Bits<94> {}
impl Reg128BitsDownCast<53> for Reg128Bits<94> {}
impl Reg128BitsDownCast<54> for Reg128Bits<94> {}
impl Reg128BitsDownCast<55> for Reg128Bits<94> {}
impl Reg128BitsDownCast<56> for Reg128Bits<94> {}
impl Reg128BitsDownCast<57> for Reg128Bits<94> {}
impl Reg128BitsDownCast<58> for Reg128Bits<94> {}
impl Reg128BitsDownCast<59> for Reg128Bits<94> {}
impl Reg128BitsDownCast<60> for Reg128Bits<94> {}
impl Reg128BitsDownCast<61> for Reg128Bits<94> {}
impl Reg128BitsDownCast<62> for Reg128Bits<94> {}
impl Reg128BitsDownCast<63> for Reg128Bits<94> {}
impl Reg128BitsDownCast<64> for Reg128Bits<94> {}
impl Reg128BitsDownCast<65> for Reg128Bits<94> {}
impl Reg128BitsDownCast<66> for Reg128Bits<94> {}
impl Reg128BitsDownCast<67> for Reg128Bits<94> {}
impl Reg128BitsDownCast<68> for Reg128Bits<94> {}
impl Reg128BitsDownCast<69> for Reg128Bits<94> {}
impl Reg128BitsDownCast<70> for Reg128Bits<94> {}
impl Reg128BitsDownCast<71> for Reg128Bits<94> {}
impl Reg128BitsDownCast<72> for Reg128Bits<94> {}
impl Reg128BitsDownCast<73> for Reg128Bits<94> {}
impl Reg128BitsDownCast<74> for Reg128Bits<94> {}
impl Reg128BitsDownCast<75> for Reg128Bits<94> {}
impl Reg128BitsDownCast<76> for Reg128Bits<94> {}
impl Reg128BitsDownCast<77> for Reg128Bits<94> {}
impl Reg128BitsDownCast<78> for Reg128Bits<94> {}
impl Reg128BitsDownCast<79> for Reg128Bits<94> {}
impl Reg128BitsDownCast<80> for Reg128Bits<94> {}
impl Reg128BitsDownCast<81> for Reg128Bits<94> {}
impl Reg128BitsDownCast<82> for Reg128Bits<94> {}
impl Reg128BitsDownCast<83> for Reg128Bits<94> {}
impl Reg128BitsDownCast<84> for Reg128Bits<94> {}
impl Reg128BitsDownCast<85> for Reg128Bits<94> {}
impl Reg128BitsDownCast<86> for Reg128Bits<94> {}
impl Reg128BitsDownCast<87> for Reg128Bits<94> {}
impl Reg128BitsDownCast<88> for Reg128Bits<94> {}
impl Reg128BitsDownCast<89> for Reg128Bits<94> {}
impl Reg128BitsDownCast<90> for Reg128Bits<94> {}
impl Reg128BitsDownCast<91> for Reg128Bits<94> {}
impl Reg128BitsDownCast<92> for Reg128Bits<94> {}
impl Reg128BitsDownCast<93> for Reg128Bits<94> {}
impl Reg128BitsDownCast<94> for Reg128Bits<94> {}
impl Reg128BitsDownCast<1> for Reg128Bits<95> {}
impl Reg128BitsConcat<1, 96> for Reg128Bits<95> {}
impl Reg128BitsDownCast<2> for Reg128Bits<95> {}
impl Reg128BitsConcat<2, 97> for Reg128Bits<95> {}
impl Reg128BitsDownCast<3> for Reg128Bits<95> {}
impl Reg128BitsConcat<3, 98> for Reg128Bits<95> {}
impl Reg128BitsDownCast<4> for Reg128Bits<95> {}
impl Reg128BitsConcat<4, 99> for Reg128Bits<95> {}
impl Reg128BitsDownCast<5> for Reg128Bits<95> {}
impl Reg128BitsConcat<5, 100> for Reg128Bits<95> {}
impl Reg128BitsDownCast<6> for Reg128Bits<95> {}
impl Reg128BitsConcat<6, 101> for Reg128Bits<95> {}
impl Reg128BitsDownCast<7> for Reg128Bits<95> {}
impl Reg128BitsConcat<7, 102> for Reg128Bits<95> {}
impl Reg128BitsDownCast<8> for Reg128Bits<95> {}
impl Reg128BitsConcat<8, 103> for Reg128Bits<95> {}
impl Reg128BitsDownCast<9> for Reg128Bits<95> {}
impl Reg128BitsConcat<9, 104> for Reg128Bits<95> {}
impl Reg128BitsDownCast<10> for Reg128Bits<95> {}
impl Reg128BitsConcat<10, 105> for Reg128Bits<95> {}
impl Reg128BitsDownCast<11> for Reg128Bits<95> {}
impl Reg128BitsConcat<11, 106> for Reg128Bits<95> {}
impl Reg128BitsDownCast<12> for Reg128Bits<95> {}
impl Reg128BitsConcat<12, 107> for Reg128Bits<95> {}
impl Reg128BitsDownCast<13> for Reg128Bits<95> {}
impl Reg128BitsConcat<13, 108> for Reg128Bits<95> {}
impl Reg128BitsDownCast<14> for Reg128Bits<95> {}
impl Reg128BitsConcat<14, 109> for Reg128Bits<95> {}
impl Reg128BitsDownCast<15> for Reg128Bits<95> {}
impl Reg128BitsConcat<15, 110> for Reg128Bits<95> {}
impl Reg128BitsDownCast<16> for Reg128Bits<95> {}
impl Reg128BitsConcat<16, 111> for Reg128Bits<95> {}
impl Reg128BitsDownCast<17> for Reg128Bits<95> {}
impl Reg128BitsConcat<17, 112> for Reg128Bits<95> {}
impl Reg128BitsDownCast<18> for Reg128Bits<95> {}
impl Reg128BitsConcat<18, 113> for Reg128Bits<95> {}
impl Reg128BitsDownCast<19> for Reg128Bits<95> {}
impl Reg128BitsConcat<19, 114> for Reg128Bits<95> {}
impl Reg128BitsDownCast<20> for Reg128Bits<95> {}
impl Reg128BitsConcat<20, 115> for Reg128Bits<95> {}
impl Reg128BitsDownCast<21> for Reg128Bits<95> {}
impl Reg128BitsConcat<21, 116> for Reg128Bits<95> {}
impl Reg128BitsDownCast<22> for Reg128Bits<95> {}
impl Reg128BitsConcat<22, 117> for Reg128Bits<95> {}
impl Reg128BitsDownCast<23> for Reg128Bits<95> {}
impl Reg128BitsConcat<23, 118> for Reg128Bits<95> {}
impl Reg128BitsDownCast<24> for Reg128Bits<95> {}
impl Reg128BitsConcat<24, 119> for Reg128Bits<95> {}
impl Reg128BitsDownCast<25> for Reg128Bits<95> {}
impl Reg128BitsConcat<25, 120> for Reg128Bits<95> {}
impl Reg128BitsDownCast<26> for Reg128Bits<95> {}
impl Reg128BitsConcat<26, 121> for Reg128Bits<95> {}
impl Reg128BitsDownCast<27> for Reg128Bits<95> {}
impl Reg128BitsConcat<27, 122> for Reg128Bits<95> {}
impl Reg128BitsDownCast<28> for Reg128Bits<95> {}
impl Reg128BitsConcat<28, 123> for Reg128Bits<95> {}
impl Reg128BitsDownCast<29> for Reg128Bits<95> {}
impl Reg128BitsConcat<29, 124> for Reg128Bits<95> {}
impl Reg128BitsDownCast<30> for Reg128Bits<95> {}
impl Reg128BitsConcat<30, 125> for Reg128Bits<95> {}
impl Reg128BitsDownCast<31> for Reg128Bits<95> {}
impl Reg128BitsConcat<31, 126> for Reg128Bits<95> {}
impl Reg128BitsDownCast<32> for Reg128Bits<95> {}
impl Reg128BitsConcat<32, 127> for Reg128Bits<95> {}
impl Reg128BitsDownCast<33> for Reg128Bits<95> {}
impl Reg128BitsConcat<33, 128> for Reg128Bits<95> {}
impl Reg128BitsDownCast<34> for Reg128Bits<95> {}
impl Reg128BitsDownCast<35> for Reg128Bits<95> {}
impl Reg128BitsDownCast<36> for Reg128Bits<95> {}
impl Reg128BitsDownCast<37> for Reg128Bits<95> {}
impl Reg128BitsDownCast<38> for Reg128Bits<95> {}
impl Reg128BitsDownCast<39> for Reg128Bits<95> {}
impl Reg128BitsDownCast<40> for Reg128Bits<95> {}
impl Reg128BitsDownCast<41> for Reg128Bits<95> {}
impl Reg128BitsDownCast<42> for Reg128Bits<95> {}
impl Reg128BitsDownCast<43> for Reg128Bits<95> {}
impl Reg128BitsDownCast<44> for Reg128Bits<95> {}
impl Reg128BitsDownCast<45> for Reg128Bits<95> {}
impl Reg128BitsDownCast<46> for Reg128Bits<95> {}
impl Reg128BitsDownCast<47> for Reg128Bits<95> {}
impl Reg128BitsDownCast<48> for Reg128Bits<95> {}
impl Reg128BitsDownCast<49> for Reg128Bits<95> {}
impl Reg128BitsDownCast<50> for Reg128Bits<95> {}
impl Reg128BitsDownCast<51> for Reg128Bits<95> {}
impl Reg128BitsDownCast<52> for Reg128Bits<95> {}
impl Reg128BitsDownCast<53> for Reg128Bits<95> {}
impl Reg128BitsDownCast<54> for Reg128Bits<95> {}
impl Reg128BitsDownCast<55> for Reg128Bits<95> {}
impl Reg128BitsDownCast<56> for Reg128Bits<95> {}
impl Reg128BitsDownCast<57> for Reg128Bits<95> {}
impl Reg128BitsDownCast<58> for Reg128Bits<95> {}
impl Reg128BitsDownCast<59> for Reg128Bits<95> {}
impl Reg128BitsDownCast<60> for Reg128Bits<95> {}
impl Reg128BitsDownCast<61> for Reg128Bits<95> {}
impl Reg128BitsDownCast<62> for Reg128Bits<95> {}
impl Reg128BitsDownCast<63> for Reg128Bits<95> {}
impl Reg128BitsDownCast<64> for Reg128Bits<95> {}
impl Reg128BitsDownCast<65> for Reg128Bits<95> {}
impl Reg128BitsDownCast<66> for Reg128Bits<95> {}
impl Reg128BitsDownCast<67> for Reg128Bits<95> {}
impl Reg128BitsDownCast<68> for Reg128Bits<95> {}
impl Reg128BitsDownCast<69> for Reg128Bits<95> {}
impl Reg128BitsDownCast<70> for Reg128Bits<95> {}
impl Reg128BitsDownCast<71> for Reg128Bits<95> {}
impl Reg128BitsDownCast<72> for Reg128Bits<95> {}
impl Reg128BitsDownCast<73> for Reg128Bits<95> {}
impl Reg128BitsDownCast<74> for Reg128Bits<95> {}
impl Reg128BitsDownCast<75> for Reg128Bits<95> {}
impl Reg128BitsDownCast<76> for Reg128Bits<95> {}
impl Reg128BitsDownCast<77> for Reg128Bits<95> {}
impl Reg128BitsDownCast<78> for Reg128Bits<95> {}
impl Reg128BitsDownCast<79> for Reg128Bits<95> {}
impl Reg128BitsDownCast<80> for Reg128Bits<95> {}
impl Reg128BitsDownCast<81> for Reg128Bits<95> {}
impl Reg128BitsDownCast<82> for Reg128Bits<95> {}
impl Reg128BitsDownCast<83> for Reg128Bits<95> {}
impl Reg128BitsDownCast<84> for Reg128Bits<95> {}
impl Reg128BitsDownCast<85> for Reg128Bits<95> {}
impl Reg128BitsDownCast<86> for Reg128Bits<95> {}
impl Reg128BitsDownCast<87> for Reg128Bits<95> {}
impl Reg128BitsDownCast<88> for Reg128Bits<95> {}
impl Reg128BitsDownCast<89> for Reg128Bits<95> {}
impl Reg128BitsDownCast<90> for Reg128Bits<95> {}
impl Reg128BitsDownCast<91> for Reg128Bits<95> {}
impl Reg128BitsDownCast<92> for Reg128Bits<95> {}
impl Reg128BitsDownCast<93> for Reg128Bits<95> {}
impl Reg128BitsDownCast<94> for Reg128Bits<95> {}
impl Reg128BitsDownCast<95> for Reg128Bits<95> {}
impl Reg128BitsDownCast<1> for Reg128Bits<96> {}
impl Reg128BitsConcat<1, 97> for Reg128Bits<96> {}
impl Reg128BitsDownCast<2> for Reg128Bits<96> {}
impl Reg128BitsConcat<2, 98> for Reg128Bits<96> {}
impl Reg128BitsDownCast<3> for Reg128Bits<96> {}
impl Reg128BitsConcat<3, 99> for Reg128Bits<96> {}
impl Reg128BitsDownCast<4> for Reg128Bits<96> {}
impl Reg128BitsConcat<4, 100> for Reg128Bits<96> {}
impl Reg128BitsDownCast<5> for Reg128Bits<96> {}
impl Reg128BitsConcat<5, 101> for Reg128Bits<96> {}
impl Reg128BitsDownCast<6> for Reg128Bits<96> {}
impl Reg128BitsConcat<6, 102> for Reg128Bits<96> {}
impl Reg128BitsDownCast<7> for Reg128Bits<96> {}
impl Reg128BitsConcat<7, 103> for Reg128Bits<96> {}
impl Reg128BitsDownCast<8> for Reg128Bits<96> {}
impl Reg128BitsConcat<8, 104> for Reg128Bits<96> {}
impl Reg128BitsDownCast<9> for Reg128Bits<96> {}
impl Reg128BitsConcat<9, 105> for Reg128Bits<96> {}
impl Reg128BitsDownCast<10> for Reg128Bits<96> {}
impl Reg128BitsConcat<10, 106> for Reg128Bits<96> {}
impl Reg128BitsDownCast<11> for Reg128Bits<96> {}
impl Reg128BitsConcat<11, 107> for Reg128Bits<96> {}
impl Reg128BitsDownCast<12> for Reg128Bits<96> {}
impl Reg128BitsConcat<12, 108> for Reg128Bits<96> {}
impl Reg128BitsDownCast<13> for Reg128Bits<96> {}
impl Reg128BitsConcat<13, 109> for Reg128Bits<96> {}
impl Reg128BitsDownCast<14> for Reg128Bits<96> {}
impl Reg128BitsConcat<14, 110> for Reg128Bits<96> {}
impl Reg128BitsDownCast<15> for Reg128Bits<96> {}
impl Reg128BitsConcat<15, 111> for Reg128Bits<96> {}
impl Reg128BitsDownCast<16> for Reg128Bits<96> {}
impl Reg128BitsConcat<16, 112> for Reg128Bits<96> {}
impl Reg128BitsDownCast<17> for Reg128Bits<96> {}
impl Reg128BitsConcat<17, 113> for Reg128Bits<96> {}
impl Reg128BitsDownCast<18> for Reg128Bits<96> {}
impl Reg128BitsConcat<18, 114> for Reg128Bits<96> {}
impl Reg128BitsDownCast<19> for Reg128Bits<96> {}
impl Reg128BitsConcat<19, 115> for Reg128Bits<96> {}
impl Reg128BitsDownCast<20> for Reg128Bits<96> {}
impl Reg128BitsConcat<20, 116> for Reg128Bits<96> {}
impl Reg128BitsDownCast<21> for Reg128Bits<96> {}
impl Reg128BitsConcat<21, 117> for Reg128Bits<96> {}
impl Reg128BitsDownCast<22> for Reg128Bits<96> {}
impl Reg128BitsConcat<22, 118> for Reg128Bits<96> {}
impl Reg128BitsDownCast<23> for Reg128Bits<96> {}
impl Reg128BitsConcat<23, 119> for Reg128Bits<96> {}
impl Reg128BitsDownCast<24> for Reg128Bits<96> {}
impl Reg128BitsConcat<24, 120> for Reg128Bits<96> {}
impl Reg128BitsDownCast<25> for Reg128Bits<96> {}
impl Reg128BitsConcat<25, 121> for Reg128Bits<96> {}
impl Reg128BitsDownCast<26> for Reg128Bits<96> {}
impl Reg128BitsConcat<26, 122> for Reg128Bits<96> {}
impl Reg128BitsDownCast<27> for Reg128Bits<96> {}
impl Reg128BitsConcat<27, 123> for Reg128Bits<96> {}
impl Reg128BitsDownCast<28> for Reg128Bits<96> {}
impl Reg128BitsConcat<28, 124> for Reg128Bits<96> {}
impl Reg128BitsDownCast<29> for Reg128Bits<96> {}
impl Reg128BitsConcat<29, 125> for Reg128Bits<96> {}
impl Reg128BitsDownCast<30> for Reg128Bits<96> {}
impl Reg128BitsConcat<30, 126> for Reg128Bits<96> {}
impl Reg128BitsDownCast<31> for Reg128Bits<96> {}
impl Reg128BitsConcat<31, 127> for Reg128Bits<96> {}
impl Reg128BitsDownCast<32> for Reg128Bits<96> {}
impl Reg128BitsConcat<32, 128> for Reg128Bits<96> {}
impl Reg128BitsDownCast<33> for Reg128Bits<96> {}
impl Reg128BitsDownCast<34> for Reg128Bits<96> {}
impl Reg128BitsDownCast<35> for Reg128Bits<96> {}
impl Reg128BitsDownCast<36> for Reg128Bits<96> {}
impl Reg128BitsDownCast<37> for Reg128Bits<96> {}
impl Reg128BitsDownCast<38> for Reg128Bits<96> {}
impl Reg128BitsDownCast<39> for Reg128Bits<96> {}
impl Reg128BitsDownCast<40> for Reg128Bits<96> {}
impl Reg128BitsDownCast<41> for Reg128Bits<96> {}
impl Reg128BitsDownCast<42> for Reg128Bits<96> {}
impl Reg128BitsDownCast<43> for Reg128Bits<96> {}
impl Reg128BitsDownCast<44> for Reg128Bits<96> {}
impl Reg128BitsDownCast<45> for Reg128Bits<96> {}
impl Reg128BitsDownCast<46> for Reg128Bits<96> {}
impl Reg128BitsDownCast<47> for Reg128Bits<96> {}
impl Reg128BitsDownCast<48> for Reg128Bits<96> {}
impl Reg128BitsDownCast<49> for Reg128Bits<96> {}
impl Reg128BitsDownCast<50> for Reg128Bits<96> {}
impl Reg128BitsDownCast<51> for Reg128Bits<96> {}
impl Reg128BitsDownCast<52> for Reg128Bits<96> {}
impl Reg128BitsDownCast<53> for Reg128Bits<96> {}
impl Reg128BitsDownCast<54> for Reg128Bits<96> {}
impl Reg128BitsDownCast<55> for Reg128Bits<96> {}
impl Reg128BitsDownCast<56> for Reg128Bits<96> {}
impl Reg128BitsDownCast<57> for Reg128Bits<96> {}
impl Reg128BitsDownCast<58> for Reg128Bits<96> {}
impl Reg128BitsDownCast<59> for Reg128Bits<96> {}
impl Reg128BitsDownCast<60> for Reg128Bits<96> {}
impl Reg128BitsDownCast<61> for Reg128Bits<96> {}
impl Reg128BitsDownCast<62> for Reg128Bits<96> {}
impl Reg128BitsDownCast<63> for Reg128Bits<96> {}
impl Reg128BitsDownCast<64> for Reg128Bits<96> {}
impl Reg128BitsDownCast<65> for Reg128Bits<96> {}
impl Reg128BitsDownCast<66> for Reg128Bits<96> {}
impl Reg128BitsDownCast<67> for Reg128Bits<96> {}
impl Reg128BitsDownCast<68> for Reg128Bits<96> {}
impl Reg128BitsDownCast<69> for Reg128Bits<96> {}
impl Reg128BitsDownCast<70> for Reg128Bits<96> {}
impl Reg128BitsDownCast<71> for Reg128Bits<96> {}
impl Reg128BitsDownCast<72> for Reg128Bits<96> {}
impl Reg128BitsDownCast<73> for Reg128Bits<96> {}
impl Reg128BitsDownCast<74> for Reg128Bits<96> {}
impl Reg128BitsDownCast<75> for Reg128Bits<96> {}
impl Reg128BitsDownCast<76> for Reg128Bits<96> {}
impl Reg128BitsDownCast<77> for Reg128Bits<96> {}
impl Reg128BitsDownCast<78> for Reg128Bits<96> {}
impl Reg128BitsDownCast<79> for Reg128Bits<96> {}
impl Reg128BitsDownCast<80> for Reg128Bits<96> {}
impl Reg128BitsDownCast<81> for Reg128Bits<96> {}
impl Reg128BitsDownCast<82> for Reg128Bits<96> {}
impl Reg128BitsDownCast<83> for Reg128Bits<96> {}
impl Reg128BitsDownCast<84> for Reg128Bits<96> {}
impl Reg128BitsDownCast<85> for Reg128Bits<96> {}
impl Reg128BitsDownCast<86> for Reg128Bits<96> {}
impl Reg128BitsDownCast<87> for Reg128Bits<96> {}
impl Reg128BitsDownCast<88> for Reg128Bits<96> {}
impl Reg128BitsDownCast<89> for Reg128Bits<96> {}
impl Reg128BitsDownCast<90> for Reg128Bits<96> {}
impl Reg128BitsDownCast<91> for Reg128Bits<96> {}
impl Reg128BitsDownCast<92> for Reg128Bits<96> {}
impl Reg128BitsDownCast<93> for Reg128Bits<96> {}
impl Reg128BitsDownCast<94> for Reg128Bits<96> {}
impl Reg128BitsDownCast<95> for Reg128Bits<96> {}
impl Reg128BitsDownCast<96> for Reg128Bits<96> {}
impl Reg128BitsDownCast<1> for Reg128Bits<97> {}
impl Reg128BitsConcat<1, 98> for Reg128Bits<97> {}
impl Reg128BitsDownCast<2> for Reg128Bits<97> {}
impl Reg128BitsConcat<2, 99> for Reg128Bits<97> {}
impl Reg128BitsDownCast<3> for Reg128Bits<97> {}
impl Reg128BitsConcat<3, 100> for Reg128Bits<97> {}
impl Reg128BitsDownCast<4> for Reg128Bits<97> {}
impl Reg128BitsConcat<4, 101> for Reg128Bits<97> {}
impl Reg128BitsDownCast<5> for Reg128Bits<97> {}
impl Reg128BitsConcat<5, 102> for Reg128Bits<97> {}
impl Reg128BitsDownCast<6> for Reg128Bits<97> {}
impl Reg128BitsConcat<6, 103> for Reg128Bits<97> {}
impl Reg128BitsDownCast<7> for Reg128Bits<97> {}
impl Reg128BitsConcat<7, 104> for Reg128Bits<97> {}
impl Reg128BitsDownCast<8> for Reg128Bits<97> {}
impl Reg128BitsConcat<8, 105> for Reg128Bits<97> {}
impl Reg128BitsDownCast<9> for Reg128Bits<97> {}
impl Reg128BitsConcat<9, 106> for Reg128Bits<97> {}
impl Reg128BitsDownCast<10> for Reg128Bits<97> {}
impl Reg128BitsConcat<10, 107> for Reg128Bits<97> {}
impl Reg128BitsDownCast<11> for Reg128Bits<97> {}
impl Reg128BitsConcat<11, 108> for Reg128Bits<97> {}
impl Reg128BitsDownCast<12> for Reg128Bits<97> {}
impl Reg128BitsConcat<12, 109> for Reg128Bits<97> {}
impl Reg128BitsDownCast<13> for Reg128Bits<97> {}
impl Reg128BitsConcat<13, 110> for Reg128Bits<97> {}
impl Reg128BitsDownCast<14> for Reg128Bits<97> {}
impl Reg128BitsConcat<14, 111> for Reg128Bits<97> {}
impl Reg128BitsDownCast<15> for Reg128Bits<97> {}
impl Reg128BitsConcat<15, 112> for Reg128Bits<97> {}
impl Reg128BitsDownCast<16> for Reg128Bits<97> {}
impl Reg128BitsConcat<16, 113> for Reg128Bits<97> {}
impl Reg128BitsDownCast<17> for Reg128Bits<97> {}
impl Reg128BitsConcat<17, 114> for Reg128Bits<97> {}
impl Reg128BitsDownCast<18> for Reg128Bits<97> {}
impl Reg128BitsConcat<18, 115> for Reg128Bits<97> {}
impl Reg128BitsDownCast<19> for Reg128Bits<97> {}
impl Reg128BitsConcat<19, 116> for Reg128Bits<97> {}
impl Reg128BitsDownCast<20> for Reg128Bits<97> {}
impl Reg128BitsConcat<20, 117> for Reg128Bits<97> {}
impl Reg128BitsDownCast<21> for Reg128Bits<97> {}
impl Reg128BitsConcat<21, 118> for Reg128Bits<97> {}
impl Reg128BitsDownCast<22> for Reg128Bits<97> {}
impl Reg128BitsConcat<22, 119> for Reg128Bits<97> {}
impl Reg128BitsDownCast<23> for Reg128Bits<97> {}
impl Reg128BitsConcat<23, 120> for Reg128Bits<97> {}
impl Reg128BitsDownCast<24> for Reg128Bits<97> {}
impl Reg128BitsConcat<24, 121> for Reg128Bits<97> {}
impl Reg128BitsDownCast<25> for Reg128Bits<97> {}
impl Reg128BitsConcat<25, 122> for Reg128Bits<97> {}
impl Reg128BitsDownCast<26> for Reg128Bits<97> {}
impl Reg128BitsConcat<26, 123> for Reg128Bits<97> {}
impl Reg128BitsDownCast<27> for Reg128Bits<97> {}
impl Reg128BitsConcat<27, 124> for Reg128Bits<97> {}
impl Reg128BitsDownCast<28> for Reg128Bits<97> {}
impl Reg128BitsConcat<28, 125> for Reg128Bits<97> {}
impl Reg128BitsDownCast<29> for Reg128Bits<97> {}
impl Reg128BitsConcat<29, 126> for Reg128Bits<97> {}
impl Reg128BitsDownCast<30> for Reg128Bits<97> {}
impl Reg128BitsConcat<30, 127> for Reg128Bits<97> {}
impl Reg128BitsDownCast<31> for Reg128Bits<97> {}
impl Reg128BitsConcat<31, 128> for Reg128Bits<97> {}
impl Reg128BitsDownCast<32> for Reg128Bits<97> {}
impl Reg128BitsDownCast<33> for Reg128Bits<97> {}
impl Reg128BitsDownCast<34> for Reg128Bits<97> {}
impl Reg128BitsDownCast<35> for Reg128Bits<97> {}
impl Reg128BitsDownCast<36> for Reg128Bits<97> {}
impl Reg128BitsDownCast<37> for Reg128Bits<97> {}
impl Reg128BitsDownCast<38> for Reg128Bits<97> {}
impl Reg128BitsDownCast<39> for Reg128Bits<97> {}
impl Reg128BitsDownCast<40> for Reg128Bits<97> {}
impl Reg128BitsDownCast<41> for Reg128Bits<97> {}
impl Reg128BitsDownCast<42> for Reg128Bits<97> {}
impl Reg128BitsDownCast<43> for Reg128Bits<97> {}
impl Reg128BitsDownCast<44> for Reg128Bits<97> {}
impl Reg128BitsDownCast<45> for Reg128Bits<97> {}
impl Reg128BitsDownCast<46> for Reg128Bits<97> {}
impl Reg128BitsDownCast<47> for Reg128Bits<97> {}
impl Reg128BitsDownCast<48> for Reg128Bits<97> {}
impl Reg128BitsDownCast<49> for Reg128Bits<97> {}
impl Reg128BitsDownCast<50> for Reg128Bits<97> {}
impl Reg128BitsDownCast<51> for Reg128Bits<97> {}
impl Reg128BitsDownCast<52> for Reg128Bits<97> {}
impl Reg128BitsDownCast<53> for Reg128Bits<97> {}
impl Reg128BitsDownCast<54> for Reg128Bits<97> {}
impl Reg128BitsDownCast<55> for Reg128Bits<97> {}
impl Reg128BitsDownCast<56> for Reg128Bits<97> {}
impl Reg128BitsDownCast<57> for Reg128Bits<97> {}
impl Reg128BitsDownCast<58> for Reg128Bits<97> {}
impl Reg128BitsDownCast<59> for Reg128Bits<97> {}
impl Reg128BitsDownCast<60> for Reg128Bits<97> {}
impl Reg128BitsDownCast<61> for Reg128Bits<97> {}
impl Reg128BitsDownCast<62> for Reg128Bits<97> {}
impl Reg128BitsDownCast<63> for Reg128Bits<97> {}
impl Reg128BitsDownCast<64> for Reg128Bits<97> {}
impl Reg128BitsDownCast<65> for Reg128Bits<97> {}
impl Reg128BitsDownCast<66> for Reg128Bits<97> {}
impl Reg128BitsDownCast<67> for Reg128Bits<97> {}
impl Reg128BitsDownCast<68> for Reg128Bits<97> {}
impl Reg128BitsDownCast<69> for Reg128Bits<97> {}
impl Reg128BitsDownCast<70> for Reg128Bits<97> {}
impl Reg128BitsDownCast<71> for Reg128Bits<97> {}
impl Reg128BitsDownCast<72> for Reg128Bits<97> {}
impl Reg128BitsDownCast<73> for Reg128Bits<97> {}
impl Reg128BitsDownCast<74> for Reg128Bits<97> {}
impl Reg128BitsDownCast<75> for Reg128Bits<97> {}
impl Reg128BitsDownCast<76> for Reg128Bits<97> {}
impl Reg128BitsDownCast<77> for Reg128Bits<97> {}
impl Reg128BitsDownCast<78> for Reg128Bits<97> {}
impl Reg128BitsDownCast<79> for Reg128Bits<97> {}
impl Reg128BitsDownCast<80> for Reg128Bits<97> {}
impl Reg128BitsDownCast<81> for Reg128Bits<97> {}
impl Reg128BitsDownCast<82> for Reg128Bits<97> {}
impl Reg128BitsDownCast<83> for Reg128Bits<97> {}
impl Reg128BitsDownCast<84> for Reg128Bits<97> {}
impl Reg128BitsDownCast<85> for Reg128Bits<97> {}
impl Reg128BitsDownCast<86> for Reg128Bits<97> {}
impl Reg128BitsDownCast<87> for Reg128Bits<97> {}
impl Reg128BitsDownCast<88> for Reg128Bits<97> {}
impl Reg128BitsDownCast<89> for Reg128Bits<97> {}
impl Reg128BitsDownCast<90> for Reg128Bits<97> {}
impl Reg128BitsDownCast<91> for Reg128Bits<97> {}
impl Reg128BitsDownCast<92> for Reg128Bits<97> {}
impl Reg128BitsDownCast<93> for Reg128Bits<97> {}
impl Reg128BitsDownCast<94> for Reg128Bits<97> {}
impl Reg128BitsDownCast<95> for Reg128Bits<97> {}
impl Reg128BitsDownCast<96> for Reg128Bits<97> {}
impl Reg128BitsDownCast<97> for Reg128Bits<97> {}
impl Reg128BitsDownCast<1> for Reg128Bits<98> {}
impl Reg128BitsConcat<1, 99> for Reg128Bits<98> {}
impl Reg128BitsDownCast<2> for Reg128Bits<98> {}
impl Reg128BitsConcat<2, 100> for Reg128Bits<98> {}
impl Reg128BitsDownCast<3> for Reg128Bits<98> {}
impl Reg128BitsConcat<3, 101> for Reg128Bits<98> {}
impl Reg128BitsDownCast<4> for Reg128Bits<98> {}
impl Reg128BitsConcat<4, 102> for Reg128Bits<98> {}
impl Reg128BitsDownCast<5> for Reg128Bits<98> {}
impl Reg128BitsConcat<5, 103> for Reg128Bits<98> {}
impl Reg128BitsDownCast<6> for Reg128Bits<98> {}
impl Reg128BitsConcat<6, 104> for Reg128Bits<98> {}
impl Reg128BitsDownCast<7> for Reg128Bits<98> {}
impl Reg128BitsConcat<7, 105> for Reg128Bits<98> {}
impl Reg128BitsDownCast<8> for Reg128Bits<98> {}
impl Reg128BitsConcat<8, 106> for Reg128Bits<98> {}
impl Reg128BitsDownCast<9> for Reg128Bits<98> {}
impl Reg128BitsConcat<9, 107> for Reg128Bits<98> {}
impl Reg128BitsDownCast<10> for Reg128Bits<98> {}
impl Reg128BitsConcat<10, 108> for Reg128Bits<98> {}
impl Reg128BitsDownCast<11> for Reg128Bits<98> {}
impl Reg128BitsConcat<11, 109> for Reg128Bits<98> {}
impl Reg128BitsDownCast<12> for Reg128Bits<98> {}
impl Reg128BitsConcat<12, 110> for Reg128Bits<98> {}
impl Reg128BitsDownCast<13> for Reg128Bits<98> {}
impl Reg128BitsConcat<13, 111> for Reg128Bits<98> {}
impl Reg128BitsDownCast<14> for Reg128Bits<98> {}
impl Reg128BitsConcat<14, 112> for Reg128Bits<98> {}
impl Reg128BitsDownCast<15> for Reg128Bits<98> {}
impl Reg128BitsConcat<15, 113> for Reg128Bits<98> {}
impl Reg128BitsDownCast<16> for Reg128Bits<98> {}
impl Reg128BitsConcat<16, 114> for Reg128Bits<98> {}
impl Reg128BitsDownCast<17> for Reg128Bits<98> {}
impl Reg128BitsConcat<17, 115> for Reg128Bits<98> {}
impl Reg128BitsDownCast<18> for Reg128Bits<98> {}
impl Reg128BitsConcat<18, 116> for Reg128Bits<98> {}
impl Reg128BitsDownCast<19> for Reg128Bits<98> {}
impl Reg128BitsConcat<19, 117> for Reg128Bits<98> {}
impl Reg128BitsDownCast<20> for Reg128Bits<98> {}
impl Reg128BitsConcat<20, 118> for Reg128Bits<98> {}
impl Reg128BitsDownCast<21> for Reg128Bits<98> {}
impl Reg128BitsConcat<21, 119> for Reg128Bits<98> {}
impl Reg128BitsDownCast<22> for Reg128Bits<98> {}
impl Reg128BitsConcat<22, 120> for Reg128Bits<98> {}
impl Reg128BitsDownCast<23> for Reg128Bits<98> {}
impl Reg128BitsConcat<23, 121> for Reg128Bits<98> {}
impl Reg128BitsDownCast<24> for Reg128Bits<98> {}
impl Reg128BitsConcat<24, 122> for Reg128Bits<98> {}
impl Reg128BitsDownCast<25> for Reg128Bits<98> {}
impl Reg128BitsConcat<25, 123> for Reg128Bits<98> {}
impl Reg128BitsDownCast<26> for Reg128Bits<98> {}
impl Reg128BitsConcat<26, 124> for Reg128Bits<98> {}
impl Reg128BitsDownCast<27> for Reg128Bits<98> {}
impl Reg128BitsConcat<27, 125> for Reg128Bits<98> {}
impl Reg128BitsDownCast<28> for Reg128Bits<98> {}
impl Reg128BitsConcat<28, 126> for Reg128Bits<98> {}
impl Reg128BitsDownCast<29> for Reg128Bits<98> {}
impl Reg128BitsConcat<29, 127> for Reg128Bits<98> {}
impl Reg128BitsDownCast<30> for Reg128Bits<98> {}
impl Reg128BitsConcat<30, 128> for Reg128Bits<98> {}
impl Reg128BitsDownCast<31> for Reg128Bits<98> {}
impl Reg128BitsDownCast<32> for Reg128Bits<98> {}
impl Reg128BitsDownCast<33> for Reg128Bits<98> {}
impl Reg128BitsDownCast<34> for Reg128Bits<98> {}
impl Reg128BitsDownCast<35> for Reg128Bits<98> {}
impl Reg128BitsDownCast<36> for Reg128Bits<98> {}
impl Reg128BitsDownCast<37> for Reg128Bits<98> {}
impl Reg128BitsDownCast<38> for Reg128Bits<98> {}
impl Reg128BitsDownCast<39> for Reg128Bits<98> {}
impl Reg128BitsDownCast<40> for Reg128Bits<98> {}
impl Reg128BitsDownCast<41> for Reg128Bits<98> {}
impl Reg128BitsDownCast<42> for Reg128Bits<98> {}
impl Reg128BitsDownCast<43> for Reg128Bits<98> {}
impl Reg128BitsDownCast<44> for Reg128Bits<98> {}
impl Reg128BitsDownCast<45> for Reg128Bits<98> {}
impl Reg128BitsDownCast<46> for Reg128Bits<98> {}
impl Reg128BitsDownCast<47> for Reg128Bits<98> {}
impl Reg128BitsDownCast<48> for Reg128Bits<98> {}
impl Reg128BitsDownCast<49> for Reg128Bits<98> {}
impl Reg128BitsDownCast<50> for Reg128Bits<98> {}
impl Reg128BitsDownCast<51> for Reg128Bits<98> {}
impl Reg128BitsDownCast<52> for Reg128Bits<98> {}
impl Reg128BitsDownCast<53> for Reg128Bits<98> {}
impl Reg128BitsDownCast<54> for Reg128Bits<98> {}
impl Reg128BitsDownCast<55> for Reg128Bits<98> {}
impl Reg128BitsDownCast<56> for Reg128Bits<98> {}
impl Reg128BitsDownCast<57> for Reg128Bits<98> {}
impl Reg128BitsDownCast<58> for Reg128Bits<98> {}
impl Reg128BitsDownCast<59> for Reg128Bits<98> {}
impl Reg128BitsDownCast<60> for Reg128Bits<98> {}
impl Reg128BitsDownCast<61> for Reg128Bits<98> {}
impl Reg128BitsDownCast<62> for Reg128Bits<98> {}
impl Reg128BitsDownCast<63> for Reg128Bits<98> {}
impl Reg128BitsDownCast<64> for Reg128Bits<98> {}
impl Reg128BitsDownCast<65> for Reg128Bits<98> {}
impl Reg128BitsDownCast<66> for Reg128Bits<98> {}
impl Reg128BitsDownCast<67> for Reg128Bits<98> {}
impl Reg128BitsDownCast<68> for Reg128Bits<98> {}
impl Reg128BitsDownCast<69> for Reg128Bits<98> {}
impl Reg128BitsDownCast<70> for Reg128Bits<98> {}
impl Reg128BitsDownCast<71> for Reg128Bits<98> {}
impl Reg128BitsDownCast<72> for Reg128Bits<98> {}
impl Reg128BitsDownCast<73> for Reg128Bits<98> {}
impl Reg128BitsDownCast<74> for Reg128Bits<98> {}
impl Reg128BitsDownCast<75> for Reg128Bits<98> {}
impl Reg128BitsDownCast<76> for Reg128Bits<98> {}
impl Reg128BitsDownCast<77> for Reg128Bits<98> {}
impl Reg128BitsDownCast<78> for Reg128Bits<98> {}
impl Reg128BitsDownCast<79> for Reg128Bits<98> {}
impl Reg128BitsDownCast<80> for Reg128Bits<98> {}
impl Reg128BitsDownCast<81> for Reg128Bits<98> {}
impl Reg128BitsDownCast<82> for Reg128Bits<98> {}
impl Reg128BitsDownCast<83> for Reg128Bits<98> {}
impl Reg128BitsDownCast<84> for Reg128Bits<98> {}
impl Reg128BitsDownCast<85> for Reg128Bits<98> {}
impl Reg128BitsDownCast<86> for Reg128Bits<98> {}
impl Reg128BitsDownCast<87> for Reg128Bits<98> {}
impl Reg128BitsDownCast<88> for Reg128Bits<98> {}
impl Reg128BitsDownCast<89> for Reg128Bits<98> {}
impl Reg128BitsDownCast<90> for Reg128Bits<98> {}
impl Reg128BitsDownCast<91> for Reg128Bits<98> {}
impl Reg128BitsDownCast<92> for Reg128Bits<98> {}
impl Reg128BitsDownCast<93> for Reg128Bits<98> {}
impl Reg128BitsDownCast<94> for Reg128Bits<98> {}
impl Reg128BitsDownCast<95> for Reg128Bits<98> {}
impl Reg128BitsDownCast<96> for Reg128Bits<98> {}
impl Reg128BitsDownCast<97> for Reg128Bits<98> {}
impl Reg128BitsDownCast<98> for Reg128Bits<98> {}
impl Reg128BitsDownCast<1> for Reg128Bits<99> {}
impl Reg128BitsConcat<1, 100> for Reg128Bits<99> {}
impl Reg128BitsDownCast<2> for Reg128Bits<99> {}
impl Reg128BitsConcat<2, 101> for Reg128Bits<99> {}
impl Reg128BitsDownCast<3> for Reg128Bits<99> {}
impl Reg128BitsConcat<3, 102> for Reg128Bits<99> {}
impl Reg128BitsDownCast<4> for Reg128Bits<99> {}
impl Reg128BitsConcat<4, 103> for Reg128Bits<99> {}
impl Reg128BitsDownCast<5> for Reg128Bits<99> {}
impl Reg128BitsConcat<5, 104> for Reg128Bits<99> {}
impl Reg128BitsDownCast<6> for Reg128Bits<99> {}
impl Reg128BitsConcat<6, 105> for Reg128Bits<99> {}
impl Reg128BitsDownCast<7> for Reg128Bits<99> {}
impl Reg128BitsConcat<7, 106> for Reg128Bits<99> {}
impl Reg128BitsDownCast<8> for Reg128Bits<99> {}
impl Reg128BitsConcat<8, 107> for Reg128Bits<99> {}
impl Reg128BitsDownCast<9> for Reg128Bits<99> {}
impl Reg128BitsConcat<9, 108> for Reg128Bits<99> {}
impl Reg128BitsDownCast<10> for Reg128Bits<99> {}
impl Reg128BitsConcat<10, 109> for Reg128Bits<99> {}
impl Reg128BitsDownCast<11> for Reg128Bits<99> {}
impl Reg128BitsConcat<11, 110> for Reg128Bits<99> {}
impl Reg128BitsDownCast<12> for Reg128Bits<99> {}
impl Reg128BitsConcat<12, 111> for Reg128Bits<99> {}
impl Reg128BitsDownCast<13> for Reg128Bits<99> {}
impl Reg128BitsConcat<13, 112> for Reg128Bits<99> {}
impl Reg128BitsDownCast<14> for Reg128Bits<99> {}
impl Reg128BitsConcat<14, 113> for Reg128Bits<99> {}
impl Reg128BitsDownCast<15> for Reg128Bits<99> {}
impl Reg128BitsConcat<15, 114> for Reg128Bits<99> {}
impl Reg128BitsDownCast<16> for Reg128Bits<99> {}
impl Reg128BitsConcat<16, 115> for Reg128Bits<99> {}
impl Reg128BitsDownCast<17> for Reg128Bits<99> {}
impl Reg128BitsConcat<17, 116> for Reg128Bits<99> {}
impl Reg128BitsDownCast<18> for Reg128Bits<99> {}
impl Reg128BitsConcat<18, 117> for Reg128Bits<99> {}
impl Reg128BitsDownCast<19> for Reg128Bits<99> {}
impl Reg128BitsConcat<19, 118> for Reg128Bits<99> {}
impl Reg128BitsDownCast<20> for Reg128Bits<99> {}
impl Reg128BitsConcat<20, 119> for Reg128Bits<99> {}
impl Reg128BitsDownCast<21> for Reg128Bits<99> {}
impl Reg128BitsConcat<21, 120> for Reg128Bits<99> {}
impl Reg128BitsDownCast<22> for Reg128Bits<99> {}
impl Reg128BitsConcat<22, 121> for Reg128Bits<99> {}
impl Reg128BitsDownCast<23> for Reg128Bits<99> {}
impl Reg128BitsConcat<23, 122> for Reg128Bits<99> {}
impl Reg128BitsDownCast<24> for Reg128Bits<99> {}
impl Reg128BitsConcat<24, 123> for Reg128Bits<99> {}
impl Reg128BitsDownCast<25> for Reg128Bits<99> {}
impl Reg128BitsConcat<25, 124> for Reg128Bits<99> {}
impl Reg128BitsDownCast<26> for Reg128Bits<99> {}
impl Reg128BitsConcat<26, 125> for Reg128Bits<99> {}
impl Reg128BitsDownCast<27> for Reg128Bits<99> {}
impl Reg128BitsConcat<27, 126> for Reg128Bits<99> {}
impl Reg128BitsDownCast<28> for Reg128Bits<99> {}
impl Reg128BitsConcat<28, 127> for Reg128Bits<99> {}
impl Reg128BitsDownCast<29> for Reg128Bits<99> {}
impl Reg128BitsConcat<29, 128> for Reg128Bits<99> {}
impl Reg128BitsDownCast<30> for Reg128Bits<99> {}
impl Reg128BitsDownCast<31> for Reg128Bits<99> {}
impl Reg128BitsDownCast<32> for Reg128Bits<99> {}
impl Reg128BitsDownCast<33> for Reg128Bits<99> {}
impl Reg128BitsDownCast<34> for Reg128Bits<99> {}
impl Reg128BitsDownCast<35> for Reg128Bits<99> {}
impl Reg128BitsDownCast<36> for Reg128Bits<99> {}
impl Reg128BitsDownCast<37> for Reg128Bits<99> {}
impl Reg128BitsDownCast<38> for Reg128Bits<99> {}
impl Reg128BitsDownCast<39> for Reg128Bits<99> {}
impl Reg128BitsDownCast<40> for Reg128Bits<99> {}
impl Reg128BitsDownCast<41> for Reg128Bits<99> {}
impl Reg128BitsDownCast<42> for Reg128Bits<99> {}
impl Reg128BitsDownCast<43> for Reg128Bits<99> {}
impl Reg128BitsDownCast<44> for Reg128Bits<99> {}
impl Reg128BitsDownCast<45> for Reg128Bits<99> {}
impl Reg128BitsDownCast<46> for Reg128Bits<99> {}
impl Reg128BitsDownCast<47> for Reg128Bits<99> {}
impl Reg128BitsDownCast<48> for Reg128Bits<99> {}
impl Reg128BitsDownCast<49> for Reg128Bits<99> {}
impl Reg128BitsDownCast<50> for Reg128Bits<99> {}
impl Reg128BitsDownCast<51> for Reg128Bits<99> {}
impl Reg128BitsDownCast<52> for Reg128Bits<99> {}
impl Reg128BitsDownCast<53> for Reg128Bits<99> {}
impl Reg128BitsDownCast<54> for Reg128Bits<99> {}
impl Reg128BitsDownCast<55> for Reg128Bits<99> {}
impl Reg128BitsDownCast<56> for Reg128Bits<99> {}
impl Reg128BitsDownCast<57> for Reg128Bits<99> {}
impl Reg128BitsDownCast<58> for Reg128Bits<99> {}
impl Reg128BitsDownCast<59> for Reg128Bits<99> {}
impl Reg128BitsDownCast<60> for Reg128Bits<99> {}
impl Reg128BitsDownCast<61> for Reg128Bits<99> {}
impl Reg128BitsDownCast<62> for Reg128Bits<99> {}
impl Reg128BitsDownCast<63> for Reg128Bits<99> {}
impl Reg128BitsDownCast<64> for Reg128Bits<99> {}
impl Reg128BitsDownCast<65> for Reg128Bits<99> {}
impl Reg128BitsDownCast<66> for Reg128Bits<99> {}
impl Reg128BitsDownCast<67> for Reg128Bits<99> {}
impl Reg128BitsDownCast<68> for Reg128Bits<99> {}
impl Reg128BitsDownCast<69> for Reg128Bits<99> {}
impl Reg128BitsDownCast<70> for Reg128Bits<99> {}
impl Reg128BitsDownCast<71> for Reg128Bits<99> {}
impl Reg128BitsDownCast<72> for Reg128Bits<99> {}
impl Reg128BitsDownCast<73> for Reg128Bits<99> {}
impl Reg128BitsDownCast<74> for Reg128Bits<99> {}
impl Reg128BitsDownCast<75> for Reg128Bits<99> {}
impl Reg128BitsDownCast<76> for Reg128Bits<99> {}
impl Reg128BitsDownCast<77> for Reg128Bits<99> {}
impl Reg128BitsDownCast<78> for Reg128Bits<99> {}
impl Reg128BitsDownCast<79> for Reg128Bits<99> {}
impl Reg128BitsDownCast<80> for Reg128Bits<99> {}
impl Reg128BitsDownCast<81> for Reg128Bits<99> {}
impl Reg128BitsDownCast<82> for Reg128Bits<99> {}
impl Reg128BitsDownCast<83> for Reg128Bits<99> {}
impl Reg128BitsDownCast<84> for Reg128Bits<99> {}
impl Reg128BitsDownCast<85> for Reg128Bits<99> {}
impl Reg128BitsDownCast<86> for Reg128Bits<99> {}
impl Reg128BitsDownCast<87> for Reg128Bits<99> {}
impl Reg128BitsDownCast<88> for Reg128Bits<99> {}
impl Reg128BitsDownCast<89> for Reg128Bits<99> {}
impl Reg128BitsDownCast<90> for Reg128Bits<99> {}
impl Reg128BitsDownCast<91> for Reg128Bits<99> {}
impl Reg128BitsDownCast<92> for Reg128Bits<99> {}
impl Reg128BitsDownCast<93> for Reg128Bits<99> {}
impl Reg128BitsDownCast<94> for Reg128Bits<99> {}
impl Reg128BitsDownCast<95> for Reg128Bits<99> {}
impl Reg128BitsDownCast<96> for Reg128Bits<99> {}
impl Reg128BitsDownCast<97> for Reg128Bits<99> {}
impl Reg128BitsDownCast<98> for Reg128Bits<99> {}
impl Reg128BitsDownCast<99> for Reg128Bits<99> {}
impl Reg128BitsDownCast<1> for Reg128Bits<100> {}
impl Reg128BitsConcat<1, 101> for Reg128Bits<100> {}
impl Reg128BitsDownCast<2> for Reg128Bits<100> {}
impl Reg128BitsConcat<2, 102> for Reg128Bits<100> {}
impl Reg128BitsDownCast<3> for Reg128Bits<100> {}
impl Reg128BitsConcat<3, 103> for Reg128Bits<100> {}
impl Reg128BitsDownCast<4> for Reg128Bits<100> {}
impl Reg128BitsConcat<4, 104> for Reg128Bits<100> {}
impl Reg128BitsDownCast<5> for Reg128Bits<100> {}
impl Reg128BitsConcat<5, 105> for Reg128Bits<100> {}
impl Reg128BitsDownCast<6> for Reg128Bits<100> {}
impl Reg128BitsConcat<6, 106> for Reg128Bits<100> {}
impl Reg128BitsDownCast<7> for Reg128Bits<100> {}
impl Reg128BitsConcat<7, 107> for Reg128Bits<100> {}
impl Reg128BitsDownCast<8> for Reg128Bits<100> {}
impl Reg128BitsConcat<8, 108> for Reg128Bits<100> {}
impl Reg128BitsDownCast<9> for Reg128Bits<100> {}
impl Reg128BitsConcat<9, 109> for Reg128Bits<100> {}
impl Reg128BitsDownCast<10> for Reg128Bits<100> {}
impl Reg128BitsConcat<10, 110> for Reg128Bits<100> {}
impl Reg128BitsDownCast<11> for Reg128Bits<100> {}
impl Reg128BitsConcat<11, 111> for Reg128Bits<100> {}
impl Reg128BitsDownCast<12> for Reg128Bits<100> {}
impl Reg128BitsConcat<12, 112> for Reg128Bits<100> {}
impl Reg128BitsDownCast<13> for Reg128Bits<100> {}
impl Reg128BitsConcat<13, 113> for Reg128Bits<100> {}
impl Reg128BitsDownCast<14> for Reg128Bits<100> {}
impl Reg128BitsConcat<14, 114> for Reg128Bits<100> {}
impl Reg128BitsDownCast<15> for Reg128Bits<100> {}
impl Reg128BitsConcat<15, 115> for Reg128Bits<100> {}
impl Reg128BitsDownCast<16> for Reg128Bits<100> {}
impl Reg128BitsConcat<16, 116> for Reg128Bits<100> {}
impl Reg128BitsDownCast<17> for Reg128Bits<100> {}
impl Reg128BitsConcat<17, 117> for Reg128Bits<100> {}
impl Reg128BitsDownCast<18> for Reg128Bits<100> {}
impl Reg128BitsConcat<18, 118> for Reg128Bits<100> {}
impl Reg128BitsDownCast<19> for Reg128Bits<100> {}
impl Reg128BitsConcat<19, 119> for Reg128Bits<100> {}
impl Reg128BitsDownCast<20> for Reg128Bits<100> {}
impl Reg128BitsConcat<20, 120> for Reg128Bits<100> {}
impl Reg128BitsDownCast<21> for Reg128Bits<100> {}
impl Reg128BitsConcat<21, 121> for Reg128Bits<100> {}
impl Reg128BitsDownCast<22> for Reg128Bits<100> {}
impl Reg128BitsConcat<22, 122> for Reg128Bits<100> {}
impl Reg128BitsDownCast<23> for Reg128Bits<100> {}
impl Reg128BitsConcat<23, 123> for Reg128Bits<100> {}
impl Reg128BitsDownCast<24> for Reg128Bits<100> {}
impl Reg128BitsConcat<24, 124> for Reg128Bits<100> {}
impl Reg128BitsDownCast<25> for Reg128Bits<100> {}
impl Reg128BitsConcat<25, 125> for Reg128Bits<100> {}
impl Reg128BitsDownCast<26> for Reg128Bits<100> {}
impl Reg128BitsConcat<26, 126> for Reg128Bits<100> {}
impl Reg128BitsDownCast<27> for Reg128Bits<100> {}
impl Reg128BitsConcat<27, 127> for Reg128Bits<100> {}
impl Reg128BitsDownCast<28> for Reg128Bits<100> {}
impl Reg128BitsConcat<28, 128> for Reg128Bits<100> {}
impl Reg128BitsDownCast<29> for Reg128Bits<100> {}
impl Reg128BitsDownCast<30> for Reg128Bits<100> {}
impl Reg128BitsDownCast<31> for Reg128Bits<100> {}
impl Reg128BitsDownCast<32> for Reg128Bits<100> {}
impl Reg128BitsDownCast<33> for Reg128Bits<100> {}
impl Reg128BitsDownCast<34> for Reg128Bits<100> {}
impl Reg128BitsDownCast<35> for Reg128Bits<100> {}
impl Reg128BitsDownCast<36> for Reg128Bits<100> {}
impl Reg128BitsDownCast<37> for Reg128Bits<100> {}
impl Reg128BitsDownCast<38> for Reg128Bits<100> {}
impl Reg128BitsDownCast<39> for Reg128Bits<100> {}
impl Reg128BitsDownCast<40> for Reg128Bits<100> {}
impl Reg128BitsDownCast<41> for Reg128Bits<100> {}
impl Reg128BitsDownCast<42> for Reg128Bits<100> {}
impl Reg128BitsDownCast<43> for Reg128Bits<100> {}
impl Reg128BitsDownCast<44> for Reg128Bits<100> {}
impl Reg128BitsDownCast<45> for Reg128Bits<100> {}
impl Reg128BitsDownCast<46> for Reg128Bits<100> {}
impl Reg128BitsDownCast<47> for Reg128Bits<100> {}
impl Reg128BitsDownCast<48> for Reg128Bits<100> {}
impl Reg128BitsDownCast<49> for Reg128Bits<100> {}
impl Reg128BitsDownCast<50> for Reg128Bits<100> {}
impl Reg128BitsDownCast<51> for Reg128Bits<100> {}
impl Reg128BitsDownCast<52> for Reg128Bits<100> {}
impl Reg128BitsDownCast<53> for Reg128Bits<100> {}
impl Reg128BitsDownCast<54> for Reg128Bits<100> {}
impl Reg128BitsDownCast<55> for Reg128Bits<100> {}
impl Reg128BitsDownCast<56> for Reg128Bits<100> {}
impl Reg128BitsDownCast<57> for Reg128Bits<100> {}
impl Reg128BitsDownCast<58> for Reg128Bits<100> {}
impl Reg128BitsDownCast<59> for Reg128Bits<100> {}
impl Reg128BitsDownCast<60> for Reg128Bits<100> {}
impl Reg128BitsDownCast<61> for Reg128Bits<100> {}
impl Reg128BitsDownCast<62> for Reg128Bits<100> {}
impl Reg128BitsDownCast<63> for Reg128Bits<100> {}
impl Reg128BitsDownCast<64> for Reg128Bits<100> {}
impl Reg128BitsDownCast<65> for Reg128Bits<100> {}
impl Reg128BitsDownCast<66> for Reg128Bits<100> {}
impl Reg128BitsDownCast<67> for Reg128Bits<100> {}
impl Reg128BitsDownCast<68> for Reg128Bits<100> {}
impl Reg128BitsDownCast<69> for Reg128Bits<100> {}
impl Reg128BitsDownCast<70> for Reg128Bits<100> {}
impl Reg128BitsDownCast<71> for Reg128Bits<100> {}
impl Reg128BitsDownCast<72> for Reg128Bits<100> {}
impl Reg128BitsDownCast<73> for Reg128Bits<100> {}
impl Reg128BitsDownCast<74> for Reg128Bits<100> {}
impl Reg128BitsDownCast<75> for Reg128Bits<100> {}
impl Reg128BitsDownCast<76> for Reg128Bits<100> {}
impl Reg128BitsDownCast<77> for Reg128Bits<100> {}
impl Reg128BitsDownCast<78> for Reg128Bits<100> {}
impl Reg128BitsDownCast<79> for Reg128Bits<100> {}
impl Reg128BitsDownCast<80> for Reg128Bits<100> {}
impl Reg128BitsDownCast<81> for Reg128Bits<100> {}
impl Reg128BitsDownCast<82> for Reg128Bits<100> {}
impl Reg128BitsDownCast<83> for Reg128Bits<100> {}
impl Reg128BitsDownCast<84> for Reg128Bits<100> {}
impl Reg128BitsDownCast<85> for Reg128Bits<100> {}
impl Reg128BitsDownCast<86> for Reg128Bits<100> {}
impl Reg128BitsDownCast<87> for Reg128Bits<100> {}
impl Reg128BitsDownCast<88> for Reg128Bits<100> {}
impl Reg128BitsDownCast<89> for Reg128Bits<100> {}
impl Reg128BitsDownCast<90> for Reg128Bits<100> {}
impl Reg128BitsDownCast<91> for Reg128Bits<100> {}
impl Reg128BitsDownCast<92> for Reg128Bits<100> {}
impl Reg128BitsDownCast<93> for Reg128Bits<100> {}
impl Reg128BitsDownCast<94> for Reg128Bits<100> {}
impl Reg128BitsDownCast<95> for Reg128Bits<100> {}
impl Reg128BitsDownCast<96> for Reg128Bits<100> {}
impl Reg128BitsDownCast<97> for Reg128Bits<100> {}
impl Reg128BitsDownCast<98> for Reg128Bits<100> {}
impl Reg128BitsDownCast<99> for Reg128Bits<100> {}
impl Reg128BitsDownCast<100> for Reg128Bits<100> {}
impl Reg128BitsDownCast<1> for Reg128Bits<101> {}
impl Reg128BitsConcat<1, 102> for Reg128Bits<101> {}
impl Reg128BitsDownCast<2> for Reg128Bits<101> {}
impl Reg128BitsConcat<2, 103> for Reg128Bits<101> {}
impl Reg128BitsDownCast<3> for Reg128Bits<101> {}
impl Reg128BitsConcat<3, 104> for Reg128Bits<101> {}
impl Reg128BitsDownCast<4> for Reg128Bits<101> {}
impl Reg128BitsConcat<4, 105> for Reg128Bits<101> {}
impl Reg128BitsDownCast<5> for Reg128Bits<101> {}
impl Reg128BitsConcat<5, 106> for Reg128Bits<101> {}
impl Reg128BitsDownCast<6> for Reg128Bits<101> {}
impl Reg128BitsConcat<6, 107> for Reg128Bits<101> {}
impl Reg128BitsDownCast<7> for Reg128Bits<101> {}
impl Reg128BitsConcat<7, 108> for Reg128Bits<101> {}
impl Reg128BitsDownCast<8> for Reg128Bits<101> {}
impl Reg128BitsConcat<8, 109> for Reg128Bits<101> {}
impl Reg128BitsDownCast<9> for Reg128Bits<101> {}
impl Reg128BitsConcat<9, 110> for Reg128Bits<101> {}
impl Reg128BitsDownCast<10> for Reg128Bits<101> {}
impl Reg128BitsConcat<10, 111> for Reg128Bits<101> {}
impl Reg128BitsDownCast<11> for Reg128Bits<101> {}
impl Reg128BitsConcat<11, 112> for Reg128Bits<101> {}
impl Reg128BitsDownCast<12> for Reg128Bits<101> {}
impl Reg128BitsConcat<12, 113> for Reg128Bits<101> {}
impl Reg128BitsDownCast<13> for Reg128Bits<101> {}
impl Reg128BitsConcat<13, 114> for Reg128Bits<101> {}
impl Reg128BitsDownCast<14> for Reg128Bits<101> {}
impl Reg128BitsConcat<14, 115> for Reg128Bits<101> {}
impl Reg128BitsDownCast<15> for Reg128Bits<101> {}
impl Reg128BitsConcat<15, 116> for Reg128Bits<101> {}
impl Reg128BitsDownCast<16> for Reg128Bits<101> {}
impl Reg128BitsConcat<16, 117> for Reg128Bits<101> {}
impl Reg128BitsDownCast<17> for Reg128Bits<101> {}
impl Reg128BitsConcat<17, 118> for Reg128Bits<101> {}
impl Reg128BitsDownCast<18> for Reg128Bits<101> {}
impl Reg128BitsConcat<18, 119> for Reg128Bits<101> {}
impl Reg128BitsDownCast<19> for Reg128Bits<101> {}
impl Reg128BitsConcat<19, 120> for Reg128Bits<101> {}
impl Reg128BitsDownCast<20> for Reg128Bits<101> {}
impl Reg128BitsConcat<20, 121> for Reg128Bits<101> {}
impl Reg128BitsDownCast<21> for Reg128Bits<101> {}
impl Reg128BitsConcat<21, 122> for Reg128Bits<101> {}
impl Reg128BitsDownCast<22> for Reg128Bits<101> {}
impl Reg128BitsConcat<22, 123> for Reg128Bits<101> {}
impl Reg128BitsDownCast<23> for Reg128Bits<101> {}
impl Reg128BitsConcat<23, 124> for Reg128Bits<101> {}
impl Reg128BitsDownCast<24> for Reg128Bits<101> {}
impl Reg128BitsConcat<24, 125> for Reg128Bits<101> {}
impl Reg128BitsDownCast<25> for Reg128Bits<101> {}
impl Reg128BitsConcat<25, 126> for Reg128Bits<101> {}
impl Reg128BitsDownCast<26> for Reg128Bits<101> {}
impl Reg128BitsConcat<26, 127> for Reg128Bits<101> {}
impl Reg128BitsDownCast<27> for Reg128Bits<101> {}
impl Reg128BitsConcat<27, 128> for Reg128Bits<101> {}
impl Reg128BitsDownCast<28> for Reg128Bits<101> {}
impl Reg128BitsDownCast<29> for Reg128Bits<101> {}
impl Reg128BitsDownCast<30> for Reg128Bits<101> {}
impl Reg128BitsDownCast<31> for Reg128Bits<101> {}
impl Reg128BitsDownCast<32> for Reg128Bits<101> {}
impl Reg128BitsDownCast<33> for Reg128Bits<101> {}
impl Reg128BitsDownCast<34> for Reg128Bits<101> {}
impl Reg128BitsDownCast<35> for Reg128Bits<101> {}
impl Reg128BitsDownCast<36> for Reg128Bits<101> {}
impl Reg128BitsDownCast<37> for Reg128Bits<101> {}
impl Reg128BitsDownCast<38> for Reg128Bits<101> {}
impl Reg128BitsDownCast<39> for Reg128Bits<101> {}
impl Reg128BitsDownCast<40> for Reg128Bits<101> {}
impl Reg128BitsDownCast<41> for Reg128Bits<101> {}
impl Reg128BitsDownCast<42> for Reg128Bits<101> {}
impl Reg128BitsDownCast<43> for Reg128Bits<101> {}
impl Reg128BitsDownCast<44> for Reg128Bits<101> {}
impl Reg128BitsDownCast<45> for Reg128Bits<101> {}
impl Reg128BitsDownCast<46> for Reg128Bits<101> {}
impl Reg128BitsDownCast<47> for Reg128Bits<101> {}
impl Reg128BitsDownCast<48> for Reg128Bits<101> {}
impl Reg128BitsDownCast<49> for Reg128Bits<101> {}
impl Reg128BitsDownCast<50> for Reg128Bits<101> {}
impl Reg128BitsDownCast<51> for Reg128Bits<101> {}
impl Reg128BitsDownCast<52> for Reg128Bits<101> {}
impl Reg128BitsDownCast<53> for Reg128Bits<101> {}
impl Reg128BitsDownCast<54> for Reg128Bits<101> {}
impl Reg128BitsDownCast<55> for Reg128Bits<101> {}
impl Reg128BitsDownCast<56> for Reg128Bits<101> {}
impl Reg128BitsDownCast<57> for Reg128Bits<101> {}
impl Reg128BitsDownCast<58> for Reg128Bits<101> {}
impl Reg128BitsDownCast<59> for Reg128Bits<101> {}
impl Reg128BitsDownCast<60> for Reg128Bits<101> {}
impl Reg128BitsDownCast<61> for Reg128Bits<101> {}
impl Reg128BitsDownCast<62> for Reg128Bits<101> {}
impl Reg128BitsDownCast<63> for Reg128Bits<101> {}
impl Reg128BitsDownCast<64> for Reg128Bits<101> {}
impl Reg128BitsDownCast<65> for Reg128Bits<101> {}
impl Reg128BitsDownCast<66> for Reg128Bits<101> {}
impl Reg128BitsDownCast<67> for Reg128Bits<101> {}
impl Reg128BitsDownCast<68> for Reg128Bits<101> {}
impl Reg128BitsDownCast<69> for Reg128Bits<101> {}
impl Reg128BitsDownCast<70> for Reg128Bits<101> {}
impl Reg128BitsDownCast<71> for Reg128Bits<101> {}
impl Reg128BitsDownCast<72> for Reg128Bits<101> {}
impl Reg128BitsDownCast<73> for Reg128Bits<101> {}
impl Reg128BitsDownCast<74> for Reg128Bits<101> {}
impl Reg128BitsDownCast<75> for Reg128Bits<101> {}
impl Reg128BitsDownCast<76> for Reg128Bits<101> {}
impl Reg128BitsDownCast<77> for Reg128Bits<101> {}
impl Reg128BitsDownCast<78> for Reg128Bits<101> {}
impl Reg128BitsDownCast<79> for Reg128Bits<101> {}
impl Reg128BitsDownCast<80> for Reg128Bits<101> {}
impl Reg128BitsDownCast<81> for Reg128Bits<101> {}
impl Reg128BitsDownCast<82> for Reg128Bits<101> {}
impl Reg128BitsDownCast<83> for Reg128Bits<101> {}
impl Reg128BitsDownCast<84> for Reg128Bits<101> {}
impl Reg128BitsDownCast<85> for Reg128Bits<101> {}
impl Reg128BitsDownCast<86> for Reg128Bits<101> {}
impl Reg128BitsDownCast<87> for Reg128Bits<101> {}
impl Reg128BitsDownCast<88> for Reg128Bits<101> {}
impl Reg128BitsDownCast<89> for Reg128Bits<101> {}
impl Reg128BitsDownCast<90> for Reg128Bits<101> {}
impl Reg128BitsDownCast<91> for Reg128Bits<101> {}
impl Reg128BitsDownCast<92> for Reg128Bits<101> {}
impl Reg128BitsDownCast<93> for Reg128Bits<101> {}
impl Reg128BitsDownCast<94> for Reg128Bits<101> {}
impl Reg128BitsDownCast<95> for Reg128Bits<101> {}
impl Reg128BitsDownCast<96> for Reg128Bits<101> {}
impl Reg128BitsDownCast<97> for Reg128Bits<101> {}
impl Reg128BitsDownCast<98> for Reg128Bits<101> {}
impl Reg128BitsDownCast<99> for Reg128Bits<101> {}
impl Reg128BitsDownCast<100> for Reg128Bits<101> {}
impl Reg128BitsDownCast<101> for Reg128Bits<101> {}
impl Reg128BitsDownCast<1> for Reg128Bits<102> {}
impl Reg128BitsConcat<1, 103> for Reg128Bits<102> {}
impl Reg128BitsDownCast<2> for Reg128Bits<102> {}
impl Reg128BitsConcat<2, 104> for Reg128Bits<102> {}
impl Reg128BitsDownCast<3> for Reg128Bits<102> {}
impl Reg128BitsConcat<3, 105> for Reg128Bits<102> {}
impl Reg128BitsDownCast<4> for Reg128Bits<102> {}
impl Reg128BitsConcat<4, 106> for Reg128Bits<102> {}
impl Reg128BitsDownCast<5> for Reg128Bits<102> {}
impl Reg128BitsConcat<5, 107> for Reg128Bits<102> {}
impl Reg128BitsDownCast<6> for Reg128Bits<102> {}
impl Reg128BitsConcat<6, 108> for Reg128Bits<102> {}
impl Reg128BitsDownCast<7> for Reg128Bits<102> {}
impl Reg128BitsConcat<7, 109> for Reg128Bits<102> {}
impl Reg128BitsDownCast<8> for Reg128Bits<102> {}
impl Reg128BitsConcat<8, 110> for Reg128Bits<102> {}
impl Reg128BitsDownCast<9> for Reg128Bits<102> {}
impl Reg128BitsConcat<9, 111> for Reg128Bits<102> {}
impl Reg128BitsDownCast<10> for Reg128Bits<102> {}
impl Reg128BitsConcat<10, 112> for Reg128Bits<102> {}
impl Reg128BitsDownCast<11> for Reg128Bits<102> {}
impl Reg128BitsConcat<11, 113> for Reg128Bits<102> {}
impl Reg128BitsDownCast<12> for Reg128Bits<102> {}
impl Reg128BitsConcat<12, 114> for Reg128Bits<102> {}
impl Reg128BitsDownCast<13> for Reg128Bits<102> {}
impl Reg128BitsConcat<13, 115> for Reg128Bits<102> {}
impl Reg128BitsDownCast<14> for Reg128Bits<102> {}
impl Reg128BitsConcat<14, 116> for Reg128Bits<102> {}
impl Reg128BitsDownCast<15> for Reg128Bits<102> {}
impl Reg128BitsConcat<15, 117> for Reg128Bits<102> {}
impl Reg128BitsDownCast<16> for Reg128Bits<102> {}
impl Reg128BitsConcat<16, 118> for Reg128Bits<102> {}
impl Reg128BitsDownCast<17> for Reg128Bits<102> {}
impl Reg128BitsConcat<17, 119> for Reg128Bits<102> {}
impl Reg128BitsDownCast<18> for Reg128Bits<102> {}
impl Reg128BitsConcat<18, 120> for Reg128Bits<102> {}
impl Reg128BitsDownCast<19> for Reg128Bits<102> {}
impl Reg128BitsConcat<19, 121> for Reg128Bits<102> {}
impl Reg128BitsDownCast<20> for Reg128Bits<102> {}
impl Reg128BitsConcat<20, 122> for Reg128Bits<102> {}
impl Reg128BitsDownCast<21> for Reg128Bits<102> {}
impl Reg128BitsConcat<21, 123> for Reg128Bits<102> {}
impl Reg128BitsDownCast<22> for Reg128Bits<102> {}
impl Reg128BitsConcat<22, 124> for Reg128Bits<102> {}
impl Reg128BitsDownCast<23> for Reg128Bits<102> {}
impl Reg128BitsConcat<23, 125> for Reg128Bits<102> {}
impl Reg128BitsDownCast<24> for Reg128Bits<102> {}
impl Reg128BitsConcat<24, 126> for Reg128Bits<102> {}
impl Reg128BitsDownCast<25> for Reg128Bits<102> {}
impl Reg128BitsConcat<25, 127> for Reg128Bits<102> {}
impl Reg128BitsDownCast<26> for Reg128Bits<102> {}
impl Reg128BitsConcat<26, 128> for Reg128Bits<102> {}
impl Reg128BitsDownCast<27> for Reg128Bits<102> {}
impl Reg128BitsDownCast<28> for Reg128Bits<102> {}
impl Reg128BitsDownCast<29> for Reg128Bits<102> {}
impl Reg128BitsDownCast<30> for Reg128Bits<102> {}
impl Reg128BitsDownCast<31> for Reg128Bits<102> {}
impl Reg128BitsDownCast<32> for Reg128Bits<102> {}
impl Reg128BitsDownCast<33> for Reg128Bits<102> {}
impl Reg128BitsDownCast<34> for Reg128Bits<102> {}
impl Reg128BitsDownCast<35> for Reg128Bits<102> {}
impl Reg128BitsDownCast<36> for Reg128Bits<102> {}
impl Reg128BitsDownCast<37> for Reg128Bits<102> {}
impl Reg128BitsDownCast<38> for Reg128Bits<102> {}
impl Reg128BitsDownCast<39> for Reg128Bits<102> {}
impl Reg128BitsDownCast<40> for Reg128Bits<102> {}
impl Reg128BitsDownCast<41> for Reg128Bits<102> {}
impl Reg128BitsDownCast<42> for Reg128Bits<102> {}
impl Reg128BitsDownCast<43> for Reg128Bits<102> {}
impl Reg128BitsDownCast<44> for Reg128Bits<102> {}
impl Reg128BitsDownCast<45> for Reg128Bits<102> {}
impl Reg128BitsDownCast<46> for Reg128Bits<102> {}
impl Reg128BitsDownCast<47> for Reg128Bits<102> {}
impl Reg128BitsDownCast<48> for Reg128Bits<102> {}
impl Reg128BitsDownCast<49> for Reg128Bits<102> {}
impl Reg128BitsDownCast<50> for Reg128Bits<102> {}
impl Reg128BitsDownCast<51> for Reg128Bits<102> {}
impl Reg128BitsDownCast<52> for Reg128Bits<102> {}
impl Reg128BitsDownCast<53> for Reg128Bits<102> {}
impl Reg128BitsDownCast<54> for Reg128Bits<102> {}
impl Reg128BitsDownCast<55> for Reg128Bits<102> {}
impl Reg128BitsDownCast<56> for Reg128Bits<102> {}
impl Reg128BitsDownCast<57> for Reg128Bits<102> {}
impl Reg128BitsDownCast<58> for Reg128Bits<102> {}
impl Reg128BitsDownCast<59> for Reg128Bits<102> {}
impl Reg128BitsDownCast<60> for Reg128Bits<102> {}
impl Reg128BitsDownCast<61> for Reg128Bits<102> {}
impl Reg128BitsDownCast<62> for Reg128Bits<102> {}
impl Reg128BitsDownCast<63> for Reg128Bits<102> {}
impl Reg128BitsDownCast<64> for Reg128Bits<102> {}
impl Reg128BitsDownCast<65> for Reg128Bits<102> {}
impl Reg128BitsDownCast<66> for Reg128Bits<102> {}
impl Reg128BitsDownCast<67> for Reg128Bits<102> {}
impl Reg128BitsDownCast<68> for Reg128Bits<102> {}
impl Reg128BitsDownCast<69> for Reg128Bits<102> {}
impl Reg128BitsDownCast<70> for Reg128Bits<102> {}
impl Reg128BitsDownCast<71> for Reg128Bits<102> {}
impl Reg128BitsDownCast<72> for Reg128Bits<102> {}
impl Reg128BitsDownCast<73> for Reg128Bits<102> {}
impl Reg128BitsDownCast<74> for Reg128Bits<102> {}
impl Reg128BitsDownCast<75> for Reg128Bits<102> {}
impl Reg128BitsDownCast<76> for Reg128Bits<102> {}
impl Reg128BitsDownCast<77> for Reg128Bits<102> {}
impl Reg128BitsDownCast<78> for Reg128Bits<102> {}
impl Reg128BitsDownCast<79> for Reg128Bits<102> {}
impl Reg128BitsDownCast<80> for Reg128Bits<102> {}
impl Reg128BitsDownCast<81> for Reg128Bits<102> {}
impl Reg128BitsDownCast<82> for Reg128Bits<102> {}
impl Reg128BitsDownCast<83> for Reg128Bits<102> {}
impl Reg128BitsDownCast<84> for Reg128Bits<102> {}
impl Reg128BitsDownCast<85> for Reg128Bits<102> {}
impl Reg128BitsDownCast<86> for Reg128Bits<102> {}
impl Reg128BitsDownCast<87> for Reg128Bits<102> {}
impl Reg128BitsDownCast<88> for Reg128Bits<102> {}
impl Reg128BitsDownCast<89> for Reg128Bits<102> {}
impl Reg128BitsDownCast<90> for Reg128Bits<102> {}
impl Reg128BitsDownCast<91> for Reg128Bits<102> {}
impl Reg128BitsDownCast<92> for Reg128Bits<102> {}
impl Reg128BitsDownCast<93> for Reg128Bits<102> {}
impl Reg128BitsDownCast<94> for Reg128Bits<102> {}
impl Reg128BitsDownCast<95> for Reg128Bits<102> {}
impl Reg128BitsDownCast<96> for Reg128Bits<102> {}
impl Reg128BitsDownCast<97> for Reg128Bits<102> {}
impl Reg128BitsDownCast<98> for Reg128Bits<102> {}
impl Reg128BitsDownCast<99> for Reg128Bits<102> {}
impl Reg128BitsDownCast<100> for Reg128Bits<102> {}
impl Reg128BitsDownCast<101> for Reg128Bits<102> {}
impl Reg128BitsDownCast<102> for Reg128Bits<102> {}
impl Reg128BitsDownCast<1> for Reg128Bits<103> {}
impl Reg128BitsConcat<1, 104> for Reg128Bits<103> {}
impl Reg128BitsDownCast<2> for Reg128Bits<103> {}
impl Reg128BitsConcat<2, 105> for Reg128Bits<103> {}
impl Reg128BitsDownCast<3> for Reg128Bits<103> {}
impl Reg128BitsConcat<3, 106> for Reg128Bits<103> {}
impl Reg128BitsDownCast<4> for Reg128Bits<103> {}
impl Reg128BitsConcat<4, 107> for Reg128Bits<103> {}
impl Reg128BitsDownCast<5> for Reg128Bits<103> {}
impl Reg128BitsConcat<5, 108> for Reg128Bits<103> {}
impl Reg128BitsDownCast<6> for Reg128Bits<103> {}
impl Reg128BitsConcat<6, 109> for Reg128Bits<103> {}
impl Reg128BitsDownCast<7> for Reg128Bits<103> {}
impl Reg128BitsConcat<7, 110> for Reg128Bits<103> {}
impl Reg128BitsDownCast<8> for Reg128Bits<103> {}
impl Reg128BitsConcat<8, 111> for Reg128Bits<103> {}
impl Reg128BitsDownCast<9> for Reg128Bits<103> {}
impl Reg128BitsConcat<9, 112> for Reg128Bits<103> {}
impl Reg128BitsDownCast<10> for Reg128Bits<103> {}
impl Reg128BitsConcat<10, 113> for Reg128Bits<103> {}
impl Reg128BitsDownCast<11> for Reg128Bits<103> {}
impl Reg128BitsConcat<11, 114> for Reg128Bits<103> {}
impl Reg128BitsDownCast<12> for Reg128Bits<103> {}
impl Reg128BitsConcat<12, 115> for Reg128Bits<103> {}
impl Reg128BitsDownCast<13> for Reg128Bits<103> {}
impl Reg128BitsConcat<13, 116> for Reg128Bits<103> {}
impl Reg128BitsDownCast<14> for Reg128Bits<103> {}
impl Reg128BitsConcat<14, 117> for Reg128Bits<103> {}
impl Reg128BitsDownCast<15> for Reg128Bits<103> {}
impl Reg128BitsConcat<15, 118> for Reg128Bits<103> {}
impl Reg128BitsDownCast<16> for Reg128Bits<103> {}
impl Reg128BitsConcat<16, 119> for Reg128Bits<103> {}
impl Reg128BitsDownCast<17> for Reg128Bits<103> {}
impl Reg128BitsConcat<17, 120> for Reg128Bits<103> {}
impl Reg128BitsDownCast<18> for Reg128Bits<103> {}
impl Reg128BitsConcat<18, 121> for Reg128Bits<103> {}
impl Reg128BitsDownCast<19> for Reg128Bits<103> {}
impl Reg128BitsConcat<19, 122> for Reg128Bits<103> {}
impl Reg128BitsDownCast<20> for Reg128Bits<103> {}
impl Reg128BitsConcat<20, 123> for Reg128Bits<103> {}
impl Reg128BitsDownCast<21> for Reg128Bits<103> {}
impl Reg128BitsConcat<21, 124> for Reg128Bits<103> {}
impl Reg128BitsDownCast<22> for Reg128Bits<103> {}
impl Reg128BitsConcat<22, 125> for Reg128Bits<103> {}
impl Reg128BitsDownCast<23> for Reg128Bits<103> {}
impl Reg128BitsConcat<23, 126> for Reg128Bits<103> {}
impl Reg128BitsDownCast<24> for Reg128Bits<103> {}
impl Reg128BitsConcat<24, 127> for Reg128Bits<103> {}
impl Reg128BitsDownCast<25> for Reg128Bits<103> {}
impl Reg128BitsConcat<25, 128> for Reg128Bits<103> {}
impl Reg128BitsDownCast<26> for Reg128Bits<103> {}
impl Reg128BitsDownCast<27> for Reg128Bits<103> {}
impl Reg128BitsDownCast<28> for Reg128Bits<103> {}
impl Reg128BitsDownCast<29> for Reg128Bits<103> {}
impl Reg128BitsDownCast<30> for Reg128Bits<103> {}
impl Reg128BitsDownCast<31> for Reg128Bits<103> {}
impl Reg128BitsDownCast<32> for Reg128Bits<103> {}
impl Reg128BitsDownCast<33> for Reg128Bits<103> {}
impl Reg128BitsDownCast<34> for Reg128Bits<103> {}
impl Reg128BitsDownCast<35> for Reg128Bits<103> {}
impl Reg128BitsDownCast<36> for Reg128Bits<103> {}
impl Reg128BitsDownCast<37> for Reg128Bits<103> {}
impl Reg128BitsDownCast<38> for Reg128Bits<103> {}
impl Reg128BitsDownCast<39> for Reg128Bits<103> {}
impl Reg128BitsDownCast<40> for Reg128Bits<103> {}
impl Reg128BitsDownCast<41> for Reg128Bits<103> {}
impl Reg128BitsDownCast<42> for Reg128Bits<103> {}
impl Reg128BitsDownCast<43> for Reg128Bits<103> {}
impl Reg128BitsDownCast<44> for Reg128Bits<103> {}
impl Reg128BitsDownCast<45> for Reg128Bits<103> {}
impl Reg128BitsDownCast<46> for Reg128Bits<103> {}
impl Reg128BitsDownCast<47> for Reg128Bits<103> {}
impl Reg128BitsDownCast<48> for Reg128Bits<103> {}
impl Reg128BitsDownCast<49> for Reg128Bits<103> {}
impl Reg128BitsDownCast<50> for Reg128Bits<103> {}
impl Reg128BitsDownCast<51> for Reg128Bits<103> {}
impl Reg128BitsDownCast<52> for Reg128Bits<103> {}
impl Reg128BitsDownCast<53> for Reg128Bits<103> {}
impl Reg128BitsDownCast<54> for Reg128Bits<103> {}
impl Reg128BitsDownCast<55> for Reg128Bits<103> {}
impl Reg128BitsDownCast<56> for Reg128Bits<103> {}
impl Reg128BitsDownCast<57> for Reg128Bits<103> {}
impl Reg128BitsDownCast<58> for Reg128Bits<103> {}
impl Reg128BitsDownCast<59> for Reg128Bits<103> {}
impl Reg128BitsDownCast<60> for Reg128Bits<103> {}
impl Reg128BitsDownCast<61> for Reg128Bits<103> {}
impl Reg128BitsDownCast<62> for Reg128Bits<103> {}
impl Reg128BitsDownCast<63> for Reg128Bits<103> {}
impl Reg128BitsDownCast<64> for Reg128Bits<103> {}
impl Reg128BitsDownCast<65> for Reg128Bits<103> {}
impl Reg128BitsDownCast<66> for Reg128Bits<103> {}
impl Reg128BitsDownCast<67> for Reg128Bits<103> {}
impl Reg128BitsDownCast<68> for Reg128Bits<103> {}
impl Reg128BitsDownCast<69> for Reg128Bits<103> {}
impl Reg128BitsDownCast<70> for Reg128Bits<103> {}
impl Reg128BitsDownCast<71> for Reg128Bits<103> {}
impl Reg128BitsDownCast<72> for Reg128Bits<103> {}
impl Reg128BitsDownCast<73> for Reg128Bits<103> {}
impl Reg128BitsDownCast<74> for Reg128Bits<103> {}
impl Reg128BitsDownCast<75> for Reg128Bits<103> {}
impl Reg128BitsDownCast<76> for Reg128Bits<103> {}
impl Reg128BitsDownCast<77> for Reg128Bits<103> {}
impl Reg128BitsDownCast<78> for Reg128Bits<103> {}
impl Reg128BitsDownCast<79> for Reg128Bits<103> {}
impl Reg128BitsDownCast<80> for Reg128Bits<103> {}
impl Reg128BitsDownCast<81> for Reg128Bits<103> {}
impl Reg128BitsDownCast<82> for Reg128Bits<103> {}
impl Reg128BitsDownCast<83> for Reg128Bits<103> {}
impl Reg128BitsDownCast<84> for Reg128Bits<103> {}
impl Reg128BitsDownCast<85> for Reg128Bits<103> {}
impl Reg128BitsDownCast<86> for Reg128Bits<103> {}
impl Reg128BitsDownCast<87> for Reg128Bits<103> {}
impl Reg128BitsDownCast<88> for Reg128Bits<103> {}
impl Reg128BitsDownCast<89> for Reg128Bits<103> {}
impl Reg128BitsDownCast<90> for Reg128Bits<103> {}
impl Reg128BitsDownCast<91> for Reg128Bits<103> {}
impl Reg128BitsDownCast<92> for Reg128Bits<103> {}
impl Reg128BitsDownCast<93> for Reg128Bits<103> {}
impl Reg128BitsDownCast<94> for Reg128Bits<103> {}
impl Reg128BitsDownCast<95> for Reg128Bits<103> {}
impl Reg128BitsDownCast<96> for Reg128Bits<103> {}
impl Reg128BitsDownCast<97> for Reg128Bits<103> {}
impl Reg128BitsDownCast<98> for Reg128Bits<103> {}
impl Reg128BitsDownCast<99> for Reg128Bits<103> {}
impl Reg128BitsDownCast<100> for Reg128Bits<103> {}
impl Reg128BitsDownCast<101> for Reg128Bits<103> {}
impl Reg128BitsDownCast<102> for Reg128Bits<103> {}
impl Reg128BitsDownCast<103> for Reg128Bits<103> {}
impl Reg128BitsDownCast<1> for Reg128Bits<104> {}
impl Reg128BitsConcat<1, 105> for Reg128Bits<104> {}
impl Reg128BitsDownCast<2> for Reg128Bits<104> {}
impl Reg128BitsConcat<2, 106> for Reg128Bits<104> {}
impl Reg128BitsDownCast<3> for Reg128Bits<104> {}
impl Reg128BitsConcat<3, 107> for Reg128Bits<104> {}
impl Reg128BitsDownCast<4> for Reg128Bits<104> {}
impl Reg128BitsConcat<4, 108> for Reg128Bits<104> {}
impl Reg128BitsDownCast<5> for Reg128Bits<104> {}
impl Reg128BitsConcat<5, 109> for Reg128Bits<104> {}
impl Reg128BitsDownCast<6> for Reg128Bits<104> {}
impl Reg128BitsConcat<6, 110> for Reg128Bits<104> {}
impl Reg128BitsDownCast<7> for Reg128Bits<104> {}
impl Reg128BitsConcat<7, 111> for Reg128Bits<104> {}
impl Reg128BitsDownCast<8> for Reg128Bits<104> {}
impl Reg128BitsConcat<8, 112> for Reg128Bits<104> {}
impl Reg128BitsDownCast<9> for Reg128Bits<104> {}
impl Reg128BitsConcat<9, 113> for Reg128Bits<104> {}
impl Reg128BitsDownCast<10> for Reg128Bits<104> {}
impl Reg128BitsConcat<10, 114> for Reg128Bits<104> {}
impl Reg128BitsDownCast<11> for Reg128Bits<104> {}
impl Reg128BitsConcat<11, 115> for Reg128Bits<104> {}
impl Reg128BitsDownCast<12> for Reg128Bits<104> {}
impl Reg128BitsConcat<12, 116> for Reg128Bits<104> {}
impl Reg128BitsDownCast<13> for Reg128Bits<104> {}
impl Reg128BitsConcat<13, 117> for Reg128Bits<104> {}
impl Reg128BitsDownCast<14> for Reg128Bits<104> {}
impl Reg128BitsConcat<14, 118> for Reg128Bits<104> {}
impl Reg128BitsDownCast<15> for Reg128Bits<104> {}
impl Reg128BitsConcat<15, 119> for Reg128Bits<104> {}
impl Reg128BitsDownCast<16> for Reg128Bits<104> {}
impl Reg128BitsConcat<16, 120> for Reg128Bits<104> {}
impl Reg128BitsDownCast<17> for Reg128Bits<104> {}
impl Reg128BitsConcat<17, 121> for Reg128Bits<104> {}
impl Reg128BitsDownCast<18> for Reg128Bits<104> {}
impl Reg128BitsConcat<18, 122> for Reg128Bits<104> {}
impl Reg128BitsDownCast<19> for Reg128Bits<104> {}
impl Reg128BitsConcat<19, 123> for Reg128Bits<104> {}
impl Reg128BitsDownCast<20> for Reg128Bits<104> {}
impl Reg128BitsConcat<20, 124> for Reg128Bits<104> {}
impl Reg128BitsDownCast<21> for Reg128Bits<104> {}
impl Reg128BitsConcat<21, 125> for Reg128Bits<104> {}
impl Reg128BitsDownCast<22> for Reg128Bits<104> {}
impl Reg128BitsConcat<22, 126> for Reg128Bits<104> {}
impl Reg128BitsDownCast<23> for Reg128Bits<104> {}
impl Reg128BitsConcat<23, 127> for Reg128Bits<104> {}
impl Reg128BitsDownCast<24> for Reg128Bits<104> {}
impl Reg128BitsConcat<24, 128> for Reg128Bits<104> {}
impl Reg128BitsDownCast<25> for Reg128Bits<104> {}
impl Reg128BitsDownCast<26> for Reg128Bits<104> {}
impl Reg128BitsDownCast<27> for Reg128Bits<104> {}
impl Reg128BitsDownCast<28> for Reg128Bits<104> {}
impl Reg128BitsDownCast<29> for Reg128Bits<104> {}
impl Reg128BitsDownCast<30> for Reg128Bits<104> {}
impl Reg128BitsDownCast<31> for Reg128Bits<104> {}
impl Reg128BitsDownCast<32> for Reg128Bits<104> {}
impl Reg128BitsDownCast<33> for Reg128Bits<104> {}
impl Reg128BitsDownCast<34> for Reg128Bits<104> {}
impl Reg128BitsDownCast<35> for Reg128Bits<104> {}
impl Reg128BitsDownCast<36> for Reg128Bits<104> {}
impl Reg128BitsDownCast<37> for Reg128Bits<104> {}
impl Reg128BitsDownCast<38> for Reg128Bits<104> {}
impl Reg128BitsDownCast<39> for Reg128Bits<104> {}
impl Reg128BitsDownCast<40> for Reg128Bits<104> {}
impl Reg128BitsDownCast<41> for Reg128Bits<104> {}
impl Reg128BitsDownCast<42> for Reg128Bits<104> {}
impl Reg128BitsDownCast<43> for Reg128Bits<104> {}
impl Reg128BitsDownCast<44> for Reg128Bits<104> {}
impl Reg128BitsDownCast<45> for Reg128Bits<104> {}
impl Reg128BitsDownCast<46> for Reg128Bits<104> {}
impl Reg128BitsDownCast<47> for Reg128Bits<104> {}
impl Reg128BitsDownCast<48> for Reg128Bits<104> {}
impl Reg128BitsDownCast<49> for Reg128Bits<104> {}
impl Reg128BitsDownCast<50> for Reg128Bits<104> {}
impl Reg128BitsDownCast<51> for Reg128Bits<104> {}
impl Reg128BitsDownCast<52> for Reg128Bits<104> {}
impl Reg128BitsDownCast<53> for Reg128Bits<104> {}
impl Reg128BitsDownCast<54> for Reg128Bits<104> {}
impl Reg128BitsDownCast<55> for Reg128Bits<104> {}
impl Reg128BitsDownCast<56> for Reg128Bits<104> {}
impl Reg128BitsDownCast<57> for Reg128Bits<104> {}
impl Reg128BitsDownCast<58> for Reg128Bits<104> {}
impl Reg128BitsDownCast<59> for Reg128Bits<104> {}
impl Reg128BitsDownCast<60> for Reg128Bits<104> {}
impl Reg128BitsDownCast<61> for Reg128Bits<104> {}
impl Reg128BitsDownCast<62> for Reg128Bits<104> {}
impl Reg128BitsDownCast<63> for Reg128Bits<104> {}
impl Reg128BitsDownCast<64> for Reg128Bits<104> {}
impl Reg128BitsDownCast<65> for Reg128Bits<104> {}
impl Reg128BitsDownCast<66> for Reg128Bits<104> {}
impl Reg128BitsDownCast<67> for Reg128Bits<104> {}
impl Reg128BitsDownCast<68> for Reg128Bits<104> {}
impl Reg128BitsDownCast<69> for Reg128Bits<104> {}
impl Reg128BitsDownCast<70> for Reg128Bits<104> {}
impl Reg128BitsDownCast<71> for Reg128Bits<104> {}
impl Reg128BitsDownCast<72> for Reg128Bits<104> {}
impl Reg128BitsDownCast<73> for Reg128Bits<104> {}
impl Reg128BitsDownCast<74> for Reg128Bits<104> {}
impl Reg128BitsDownCast<75> for Reg128Bits<104> {}
impl Reg128BitsDownCast<76> for Reg128Bits<104> {}
impl Reg128BitsDownCast<77> for Reg128Bits<104> {}
impl Reg128BitsDownCast<78> for Reg128Bits<104> {}
impl Reg128BitsDownCast<79> for Reg128Bits<104> {}
impl Reg128BitsDownCast<80> for Reg128Bits<104> {}
impl Reg128BitsDownCast<81> for Reg128Bits<104> {}
impl Reg128BitsDownCast<82> for Reg128Bits<104> {}
impl Reg128BitsDownCast<83> for Reg128Bits<104> {}
impl Reg128BitsDownCast<84> for Reg128Bits<104> {}
impl Reg128BitsDownCast<85> for Reg128Bits<104> {}
impl Reg128BitsDownCast<86> for Reg128Bits<104> {}
impl Reg128BitsDownCast<87> for Reg128Bits<104> {}
impl Reg128BitsDownCast<88> for Reg128Bits<104> {}
impl Reg128BitsDownCast<89> for Reg128Bits<104> {}
impl Reg128BitsDownCast<90> for Reg128Bits<104> {}
impl Reg128BitsDownCast<91> for Reg128Bits<104> {}
impl Reg128BitsDownCast<92> for Reg128Bits<104> {}
impl Reg128BitsDownCast<93> for Reg128Bits<104> {}
impl Reg128BitsDownCast<94> for Reg128Bits<104> {}
impl Reg128BitsDownCast<95> for Reg128Bits<104> {}
impl Reg128BitsDownCast<96> for Reg128Bits<104> {}
impl Reg128BitsDownCast<97> for Reg128Bits<104> {}
impl Reg128BitsDownCast<98> for Reg128Bits<104> {}
impl Reg128BitsDownCast<99> for Reg128Bits<104> {}
impl Reg128BitsDownCast<100> for Reg128Bits<104> {}
impl Reg128BitsDownCast<101> for Reg128Bits<104> {}
impl Reg128BitsDownCast<102> for Reg128Bits<104> {}
impl Reg128BitsDownCast<103> for Reg128Bits<104> {}
impl Reg128BitsDownCast<104> for Reg128Bits<104> {}
impl Reg128BitsDownCast<1> for Reg128Bits<105> {}
impl Reg128BitsConcat<1, 106> for Reg128Bits<105> {}
impl Reg128BitsDownCast<2> for Reg128Bits<105> {}
impl Reg128BitsConcat<2, 107> for Reg128Bits<105> {}
impl Reg128BitsDownCast<3> for Reg128Bits<105> {}
impl Reg128BitsConcat<3, 108> for Reg128Bits<105> {}
impl Reg128BitsDownCast<4> for Reg128Bits<105> {}
impl Reg128BitsConcat<4, 109> for Reg128Bits<105> {}
impl Reg128BitsDownCast<5> for Reg128Bits<105> {}
impl Reg128BitsConcat<5, 110> for Reg128Bits<105> {}
impl Reg128BitsDownCast<6> for Reg128Bits<105> {}
impl Reg128BitsConcat<6, 111> for Reg128Bits<105> {}
impl Reg128BitsDownCast<7> for Reg128Bits<105> {}
impl Reg128BitsConcat<7, 112> for Reg128Bits<105> {}
impl Reg128BitsDownCast<8> for Reg128Bits<105> {}
impl Reg128BitsConcat<8, 113> for Reg128Bits<105> {}
impl Reg128BitsDownCast<9> for Reg128Bits<105> {}
impl Reg128BitsConcat<9, 114> for Reg128Bits<105> {}
impl Reg128BitsDownCast<10> for Reg128Bits<105> {}
impl Reg128BitsConcat<10, 115> for Reg128Bits<105> {}
impl Reg128BitsDownCast<11> for Reg128Bits<105> {}
impl Reg128BitsConcat<11, 116> for Reg128Bits<105> {}
impl Reg128BitsDownCast<12> for Reg128Bits<105> {}
impl Reg128BitsConcat<12, 117> for Reg128Bits<105> {}
impl Reg128BitsDownCast<13> for Reg128Bits<105> {}
impl Reg128BitsConcat<13, 118> for Reg128Bits<105> {}
impl Reg128BitsDownCast<14> for Reg128Bits<105> {}
impl Reg128BitsConcat<14, 119> for Reg128Bits<105> {}
impl Reg128BitsDownCast<15> for Reg128Bits<105> {}
impl Reg128BitsConcat<15, 120> for Reg128Bits<105> {}
impl Reg128BitsDownCast<16> for Reg128Bits<105> {}
impl Reg128BitsConcat<16, 121> for Reg128Bits<105> {}
impl Reg128BitsDownCast<17> for Reg128Bits<105> {}
impl Reg128BitsConcat<17, 122> for Reg128Bits<105> {}
impl Reg128BitsDownCast<18> for Reg128Bits<105> {}
impl Reg128BitsConcat<18, 123> for Reg128Bits<105> {}
impl Reg128BitsDownCast<19> for Reg128Bits<105> {}
impl Reg128BitsConcat<19, 124> for Reg128Bits<105> {}
impl Reg128BitsDownCast<20> for Reg128Bits<105> {}
impl Reg128BitsConcat<20, 125> for Reg128Bits<105> {}
impl Reg128BitsDownCast<21> for Reg128Bits<105> {}
impl Reg128BitsConcat<21, 126> for Reg128Bits<105> {}
impl Reg128BitsDownCast<22> for Reg128Bits<105> {}
impl Reg128BitsConcat<22, 127> for Reg128Bits<105> {}
impl Reg128BitsDownCast<23> for Reg128Bits<105> {}
impl Reg128BitsConcat<23, 128> for Reg128Bits<105> {}
impl Reg128BitsDownCast<24> for Reg128Bits<105> {}
impl Reg128BitsDownCast<25> for Reg128Bits<105> {}
impl Reg128BitsDownCast<26> for Reg128Bits<105> {}
impl Reg128BitsDownCast<27> for Reg128Bits<105> {}
impl Reg128BitsDownCast<28> for Reg128Bits<105> {}
impl Reg128BitsDownCast<29> for Reg128Bits<105> {}
impl Reg128BitsDownCast<30> for Reg128Bits<105> {}
impl Reg128BitsDownCast<31> for Reg128Bits<105> {}
impl Reg128BitsDownCast<32> for Reg128Bits<105> {}
impl Reg128BitsDownCast<33> for Reg128Bits<105> {}
impl Reg128BitsDownCast<34> for Reg128Bits<105> {}
impl Reg128BitsDownCast<35> for Reg128Bits<105> {}
impl Reg128BitsDownCast<36> for Reg128Bits<105> {}
impl Reg128BitsDownCast<37> for Reg128Bits<105> {}
impl Reg128BitsDownCast<38> for Reg128Bits<105> {}
impl Reg128BitsDownCast<39> for Reg128Bits<105> {}
impl Reg128BitsDownCast<40> for Reg128Bits<105> {}
impl Reg128BitsDownCast<41> for Reg128Bits<105> {}
impl Reg128BitsDownCast<42> for Reg128Bits<105> {}
impl Reg128BitsDownCast<43> for Reg128Bits<105> {}
impl Reg128BitsDownCast<44> for Reg128Bits<105> {}
impl Reg128BitsDownCast<45> for Reg128Bits<105> {}
impl Reg128BitsDownCast<46> for Reg128Bits<105> {}
impl Reg128BitsDownCast<47> for Reg128Bits<105> {}
impl Reg128BitsDownCast<48> for Reg128Bits<105> {}
impl Reg128BitsDownCast<49> for Reg128Bits<105> {}
impl Reg128BitsDownCast<50> for Reg128Bits<105> {}
impl Reg128BitsDownCast<51> for Reg128Bits<105> {}
impl Reg128BitsDownCast<52> for Reg128Bits<105> {}
impl Reg128BitsDownCast<53> for Reg128Bits<105> {}
impl Reg128BitsDownCast<54> for Reg128Bits<105> {}
impl Reg128BitsDownCast<55> for Reg128Bits<105> {}
impl Reg128BitsDownCast<56> for Reg128Bits<105> {}
impl Reg128BitsDownCast<57> for Reg128Bits<105> {}
impl Reg128BitsDownCast<58> for Reg128Bits<105> {}
impl Reg128BitsDownCast<59> for Reg128Bits<105> {}
impl Reg128BitsDownCast<60> for Reg128Bits<105> {}
impl Reg128BitsDownCast<61> for Reg128Bits<105> {}
impl Reg128BitsDownCast<62> for Reg128Bits<105> {}
impl Reg128BitsDownCast<63> for Reg128Bits<105> {}
impl Reg128BitsDownCast<64> for Reg128Bits<105> {}
impl Reg128BitsDownCast<65> for Reg128Bits<105> {}
impl Reg128BitsDownCast<66> for Reg128Bits<105> {}
impl Reg128BitsDownCast<67> for Reg128Bits<105> {}
impl Reg128BitsDownCast<68> for Reg128Bits<105> {}
impl Reg128BitsDownCast<69> for Reg128Bits<105> {}
impl Reg128BitsDownCast<70> for Reg128Bits<105> {}
impl Reg128BitsDownCast<71> for Reg128Bits<105> {}
impl Reg128BitsDownCast<72> for Reg128Bits<105> {}
impl Reg128BitsDownCast<73> for Reg128Bits<105> {}
impl Reg128BitsDownCast<74> for Reg128Bits<105> {}
impl Reg128BitsDownCast<75> for Reg128Bits<105> {}
impl Reg128BitsDownCast<76> for Reg128Bits<105> {}
impl Reg128BitsDownCast<77> for Reg128Bits<105> {}
impl Reg128BitsDownCast<78> for Reg128Bits<105> {}
impl Reg128BitsDownCast<79> for Reg128Bits<105> {}
impl Reg128BitsDownCast<80> for Reg128Bits<105> {}
impl Reg128BitsDownCast<81> for Reg128Bits<105> {}
impl Reg128BitsDownCast<82> for Reg128Bits<105> {}
impl Reg128BitsDownCast<83> for Reg128Bits<105> {}
impl Reg128BitsDownCast<84> for Reg128Bits<105> {}
impl Reg128BitsDownCast<85> for Reg128Bits<105> {}
impl Reg128BitsDownCast<86> for Reg128Bits<105> {}
impl Reg128BitsDownCast<87> for Reg128Bits<105> {}
impl Reg128BitsDownCast<88> for Reg128Bits<105> {}
impl Reg128BitsDownCast<89> for Reg128Bits<105> {}
impl Reg128BitsDownCast<90> for Reg128Bits<105> {}
impl Reg128BitsDownCast<91> for Reg128Bits<105> {}
impl Reg128BitsDownCast<92> for Reg128Bits<105> {}
impl Reg128BitsDownCast<93> for Reg128Bits<105> {}
impl Reg128BitsDownCast<94> for Reg128Bits<105> {}
impl Reg128BitsDownCast<95> for Reg128Bits<105> {}
impl Reg128BitsDownCast<96> for Reg128Bits<105> {}
impl Reg128BitsDownCast<97> for Reg128Bits<105> {}
impl Reg128BitsDownCast<98> for Reg128Bits<105> {}
impl Reg128BitsDownCast<99> for Reg128Bits<105> {}
impl Reg128BitsDownCast<100> for Reg128Bits<105> {}
impl Reg128BitsDownCast<101> for Reg128Bits<105> {}
impl Reg128BitsDownCast<102> for Reg128Bits<105> {}
impl Reg128BitsDownCast<103> for Reg128Bits<105> {}
impl Reg128BitsDownCast<104> for Reg128Bits<105> {}
impl Reg128BitsDownCast<105> for Reg128Bits<105> {}
impl Reg128BitsDownCast<1> for Reg128Bits<106> {}
impl Reg128BitsConcat<1, 107> for Reg128Bits<106> {}
impl Reg128BitsDownCast<2> for Reg128Bits<106> {}
impl Reg128BitsConcat<2, 108> for Reg128Bits<106> {}
impl Reg128BitsDownCast<3> for Reg128Bits<106> {}
impl Reg128BitsConcat<3, 109> for Reg128Bits<106> {}
impl Reg128BitsDownCast<4> for Reg128Bits<106> {}
impl Reg128BitsConcat<4, 110> for Reg128Bits<106> {}
impl Reg128BitsDownCast<5> for Reg128Bits<106> {}
impl Reg128BitsConcat<5, 111> for Reg128Bits<106> {}
impl Reg128BitsDownCast<6> for Reg128Bits<106> {}
impl Reg128BitsConcat<6, 112> for Reg128Bits<106> {}
impl Reg128BitsDownCast<7> for Reg128Bits<106> {}
impl Reg128BitsConcat<7, 113> for Reg128Bits<106> {}
impl Reg128BitsDownCast<8> for Reg128Bits<106> {}
impl Reg128BitsConcat<8, 114> for Reg128Bits<106> {}
impl Reg128BitsDownCast<9> for Reg128Bits<106> {}
impl Reg128BitsConcat<9, 115> for Reg128Bits<106> {}
impl Reg128BitsDownCast<10> for Reg128Bits<106> {}
impl Reg128BitsConcat<10, 116> for Reg128Bits<106> {}
impl Reg128BitsDownCast<11> for Reg128Bits<106> {}
impl Reg128BitsConcat<11, 117> for Reg128Bits<106> {}
impl Reg128BitsDownCast<12> for Reg128Bits<106> {}
impl Reg128BitsConcat<12, 118> for Reg128Bits<106> {}
impl Reg128BitsDownCast<13> for Reg128Bits<106> {}
impl Reg128BitsConcat<13, 119> for Reg128Bits<106> {}
impl Reg128BitsDownCast<14> for Reg128Bits<106> {}
impl Reg128BitsConcat<14, 120> for Reg128Bits<106> {}
impl Reg128BitsDownCast<15> for Reg128Bits<106> {}
impl Reg128BitsConcat<15, 121> for Reg128Bits<106> {}
impl Reg128BitsDownCast<16> for Reg128Bits<106> {}
impl Reg128BitsConcat<16, 122> for Reg128Bits<106> {}
impl Reg128BitsDownCast<17> for Reg128Bits<106> {}
impl Reg128BitsConcat<17, 123> for Reg128Bits<106> {}
impl Reg128BitsDownCast<18> for Reg128Bits<106> {}
impl Reg128BitsConcat<18, 124> for Reg128Bits<106> {}
impl Reg128BitsDownCast<19> for Reg128Bits<106> {}
impl Reg128BitsConcat<19, 125> for Reg128Bits<106> {}
impl Reg128BitsDownCast<20> for Reg128Bits<106> {}
impl Reg128BitsConcat<20, 126> for Reg128Bits<106> {}
impl Reg128BitsDownCast<21> for Reg128Bits<106> {}
impl Reg128BitsConcat<21, 127> for Reg128Bits<106> {}
impl Reg128BitsDownCast<22> for Reg128Bits<106> {}
impl Reg128BitsConcat<22, 128> for Reg128Bits<106> {}
impl Reg128BitsDownCast<23> for Reg128Bits<106> {}
impl Reg128BitsDownCast<24> for Reg128Bits<106> {}
impl Reg128BitsDownCast<25> for Reg128Bits<106> {}
impl Reg128BitsDownCast<26> for Reg128Bits<106> {}
impl Reg128BitsDownCast<27> for Reg128Bits<106> {}
impl Reg128BitsDownCast<28> for Reg128Bits<106> {}
impl Reg128BitsDownCast<29> for Reg128Bits<106> {}
impl Reg128BitsDownCast<30> for Reg128Bits<106> {}
impl Reg128BitsDownCast<31> for Reg128Bits<106> {}
impl Reg128BitsDownCast<32> for Reg128Bits<106> {}
impl Reg128BitsDownCast<33> for Reg128Bits<106> {}
impl Reg128BitsDownCast<34> for Reg128Bits<106> {}
impl Reg128BitsDownCast<35> for Reg128Bits<106> {}
impl Reg128BitsDownCast<36> for Reg128Bits<106> {}
impl Reg128BitsDownCast<37> for Reg128Bits<106> {}
impl Reg128BitsDownCast<38> for Reg128Bits<106> {}
impl Reg128BitsDownCast<39> for Reg128Bits<106> {}
impl Reg128BitsDownCast<40> for Reg128Bits<106> {}
impl Reg128BitsDownCast<41> for Reg128Bits<106> {}
impl Reg128BitsDownCast<42> for Reg128Bits<106> {}
impl Reg128BitsDownCast<43> for Reg128Bits<106> {}
impl Reg128BitsDownCast<44> for Reg128Bits<106> {}
impl Reg128BitsDownCast<45> for Reg128Bits<106> {}
impl Reg128BitsDownCast<46> for Reg128Bits<106> {}
impl Reg128BitsDownCast<47> for Reg128Bits<106> {}
impl Reg128BitsDownCast<48> for Reg128Bits<106> {}
impl Reg128BitsDownCast<49> for Reg128Bits<106> {}
impl Reg128BitsDownCast<50> for Reg128Bits<106> {}
impl Reg128BitsDownCast<51> for Reg128Bits<106> {}
impl Reg128BitsDownCast<52> for Reg128Bits<106> {}
impl Reg128BitsDownCast<53> for Reg128Bits<106> {}
impl Reg128BitsDownCast<54> for Reg128Bits<106> {}
impl Reg128BitsDownCast<55> for Reg128Bits<106> {}
impl Reg128BitsDownCast<56> for Reg128Bits<106> {}
impl Reg128BitsDownCast<57> for Reg128Bits<106> {}
impl Reg128BitsDownCast<58> for Reg128Bits<106> {}
impl Reg128BitsDownCast<59> for Reg128Bits<106> {}
impl Reg128BitsDownCast<60> for Reg128Bits<106> {}
impl Reg128BitsDownCast<61> for Reg128Bits<106> {}
impl Reg128BitsDownCast<62> for Reg128Bits<106> {}
impl Reg128BitsDownCast<63> for Reg128Bits<106> {}
impl Reg128BitsDownCast<64> for Reg128Bits<106> {}
impl Reg128BitsDownCast<65> for Reg128Bits<106> {}
impl Reg128BitsDownCast<66> for Reg128Bits<106> {}
impl Reg128BitsDownCast<67> for Reg128Bits<106> {}
impl Reg128BitsDownCast<68> for Reg128Bits<106> {}
impl Reg128BitsDownCast<69> for Reg128Bits<106> {}
impl Reg128BitsDownCast<70> for Reg128Bits<106> {}
impl Reg128BitsDownCast<71> for Reg128Bits<106> {}
impl Reg128BitsDownCast<72> for Reg128Bits<106> {}
impl Reg128BitsDownCast<73> for Reg128Bits<106> {}
impl Reg128BitsDownCast<74> for Reg128Bits<106> {}
impl Reg128BitsDownCast<75> for Reg128Bits<106> {}
impl Reg128BitsDownCast<76> for Reg128Bits<106> {}
impl Reg128BitsDownCast<77> for Reg128Bits<106> {}
impl Reg128BitsDownCast<78> for Reg128Bits<106> {}
impl Reg128BitsDownCast<79> for Reg128Bits<106> {}
impl Reg128BitsDownCast<80> for Reg128Bits<106> {}
impl Reg128BitsDownCast<81> for Reg128Bits<106> {}
impl Reg128BitsDownCast<82> for Reg128Bits<106> {}
impl Reg128BitsDownCast<83> for Reg128Bits<106> {}
impl Reg128BitsDownCast<84> for Reg128Bits<106> {}
impl Reg128BitsDownCast<85> for Reg128Bits<106> {}
impl Reg128BitsDownCast<86> for Reg128Bits<106> {}
impl Reg128BitsDownCast<87> for Reg128Bits<106> {}
impl Reg128BitsDownCast<88> for Reg128Bits<106> {}
impl Reg128BitsDownCast<89> for Reg128Bits<106> {}
impl Reg128BitsDownCast<90> for Reg128Bits<106> {}
impl Reg128BitsDownCast<91> for Reg128Bits<106> {}
impl Reg128BitsDownCast<92> for Reg128Bits<106> {}
impl Reg128BitsDownCast<93> for Reg128Bits<106> {}
impl Reg128BitsDownCast<94> for Reg128Bits<106> {}
impl Reg128BitsDownCast<95> for Reg128Bits<106> {}
impl Reg128BitsDownCast<96> for Reg128Bits<106> {}
impl Reg128BitsDownCast<97> for Reg128Bits<106> {}
impl Reg128BitsDownCast<98> for Reg128Bits<106> {}
impl Reg128BitsDownCast<99> for Reg128Bits<106> {}
impl Reg128BitsDownCast<100> for Reg128Bits<106> {}
impl Reg128BitsDownCast<101> for Reg128Bits<106> {}
impl Reg128BitsDownCast<102> for Reg128Bits<106> {}
impl Reg128BitsDownCast<103> for Reg128Bits<106> {}
impl Reg128BitsDownCast<104> for Reg128Bits<106> {}
impl Reg128BitsDownCast<105> for Reg128Bits<106> {}
impl Reg128BitsDownCast<106> for Reg128Bits<106> {}
impl Reg128BitsDownCast<1> for Reg128Bits<107> {}
impl Reg128BitsConcat<1, 108> for Reg128Bits<107> {}
impl Reg128BitsDownCast<2> for Reg128Bits<107> {}
impl Reg128BitsConcat<2, 109> for Reg128Bits<107> {}
impl Reg128BitsDownCast<3> for Reg128Bits<107> {}
impl Reg128BitsConcat<3, 110> for Reg128Bits<107> {}
impl Reg128BitsDownCast<4> for Reg128Bits<107> {}
impl Reg128BitsConcat<4, 111> for Reg128Bits<107> {}
impl Reg128BitsDownCast<5> for Reg128Bits<107> {}
impl Reg128BitsConcat<5, 112> for Reg128Bits<107> {}
impl Reg128BitsDownCast<6> for Reg128Bits<107> {}
impl Reg128BitsConcat<6, 113> for Reg128Bits<107> {}
impl Reg128BitsDownCast<7> for Reg128Bits<107> {}
impl Reg128BitsConcat<7, 114> for Reg128Bits<107> {}
impl Reg128BitsDownCast<8> for Reg128Bits<107> {}
impl Reg128BitsConcat<8, 115> for Reg128Bits<107> {}
impl Reg128BitsDownCast<9> for Reg128Bits<107> {}
impl Reg128BitsConcat<9, 116> for Reg128Bits<107> {}
impl Reg128BitsDownCast<10> for Reg128Bits<107> {}
impl Reg128BitsConcat<10, 117> for Reg128Bits<107> {}
impl Reg128BitsDownCast<11> for Reg128Bits<107> {}
impl Reg128BitsConcat<11, 118> for Reg128Bits<107> {}
impl Reg128BitsDownCast<12> for Reg128Bits<107> {}
impl Reg128BitsConcat<12, 119> for Reg128Bits<107> {}
impl Reg128BitsDownCast<13> for Reg128Bits<107> {}
impl Reg128BitsConcat<13, 120> for Reg128Bits<107> {}
impl Reg128BitsDownCast<14> for Reg128Bits<107> {}
impl Reg128BitsConcat<14, 121> for Reg128Bits<107> {}
impl Reg128BitsDownCast<15> for Reg128Bits<107> {}
impl Reg128BitsConcat<15, 122> for Reg128Bits<107> {}
impl Reg128BitsDownCast<16> for Reg128Bits<107> {}
impl Reg128BitsConcat<16, 123> for Reg128Bits<107> {}
impl Reg128BitsDownCast<17> for Reg128Bits<107> {}
impl Reg128BitsConcat<17, 124> for Reg128Bits<107> {}
impl Reg128BitsDownCast<18> for Reg128Bits<107> {}
impl Reg128BitsConcat<18, 125> for Reg128Bits<107> {}
impl Reg128BitsDownCast<19> for Reg128Bits<107> {}
impl Reg128BitsConcat<19, 126> for Reg128Bits<107> {}
impl Reg128BitsDownCast<20> for Reg128Bits<107> {}
impl Reg128BitsConcat<20, 127> for Reg128Bits<107> {}
impl Reg128BitsDownCast<21> for Reg128Bits<107> {}
impl Reg128BitsConcat<21, 128> for Reg128Bits<107> {}
impl Reg128BitsDownCast<22> for Reg128Bits<107> {}
impl Reg128BitsDownCast<23> for Reg128Bits<107> {}
impl Reg128BitsDownCast<24> for Reg128Bits<107> {}
impl Reg128BitsDownCast<25> for Reg128Bits<107> {}
impl Reg128BitsDownCast<26> for Reg128Bits<107> {}
impl Reg128BitsDownCast<27> for Reg128Bits<107> {}
impl Reg128BitsDownCast<28> for Reg128Bits<107> {}
impl Reg128BitsDownCast<29> for Reg128Bits<107> {}
impl Reg128BitsDownCast<30> for Reg128Bits<107> {}
impl Reg128BitsDownCast<31> for Reg128Bits<107> {}
impl Reg128BitsDownCast<32> for Reg128Bits<107> {}
impl Reg128BitsDownCast<33> for Reg128Bits<107> {}
impl Reg128BitsDownCast<34> for Reg128Bits<107> {}
impl Reg128BitsDownCast<35> for Reg128Bits<107> {}
impl Reg128BitsDownCast<36> for Reg128Bits<107> {}
impl Reg128BitsDownCast<37> for Reg128Bits<107> {}
impl Reg128BitsDownCast<38> for Reg128Bits<107> {}
impl Reg128BitsDownCast<39> for Reg128Bits<107> {}
impl Reg128BitsDownCast<40> for Reg128Bits<107> {}
impl Reg128BitsDownCast<41> for Reg128Bits<107> {}
impl Reg128BitsDownCast<42> for Reg128Bits<107> {}
impl Reg128BitsDownCast<43> for Reg128Bits<107> {}
impl Reg128BitsDownCast<44> for Reg128Bits<107> {}
impl Reg128BitsDownCast<45> for Reg128Bits<107> {}
impl Reg128BitsDownCast<46> for Reg128Bits<107> {}
impl Reg128BitsDownCast<47> for Reg128Bits<107> {}
impl Reg128BitsDownCast<48> for Reg128Bits<107> {}
impl Reg128BitsDownCast<49> for Reg128Bits<107> {}
impl Reg128BitsDownCast<50> for Reg128Bits<107> {}
impl Reg128BitsDownCast<51> for Reg128Bits<107> {}
impl Reg128BitsDownCast<52> for Reg128Bits<107> {}
impl Reg128BitsDownCast<53> for Reg128Bits<107> {}
impl Reg128BitsDownCast<54> for Reg128Bits<107> {}
impl Reg128BitsDownCast<55> for Reg128Bits<107> {}
impl Reg128BitsDownCast<56> for Reg128Bits<107> {}
impl Reg128BitsDownCast<57> for Reg128Bits<107> {}
impl Reg128BitsDownCast<58> for Reg128Bits<107> {}
impl Reg128BitsDownCast<59> for Reg128Bits<107> {}
impl Reg128BitsDownCast<60> for Reg128Bits<107> {}
impl Reg128BitsDownCast<61> for Reg128Bits<107> {}
impl Reg128BitsDownCast<62> for Reg128Bits<107> {}
impl Reg128BitsDownCast<63> for Reg128Bits<107> {}
impl Reg128BitsDownCast<64> for Reg128Bits<107> {}
impl Reg128BitsDownCast<65> for Reg128Bits<107> {}
impl Reg128BitsDownCast<66> for Reg128Bits<107> {}
impl Reg128BitsDownCast<67> for Reg128Bits<107> {}
impl Reg128BitsDownCast<68> for Reg128Bits<107> {}
impl Reg128BitsDownCast<69> for Reg128Bits<107> {}
impl Reg128BitsDownCast<70> for Reg128Bits<107> {}
impl Reg128BitsDownCast<71> for Reg128Bits<107> {}
impl Reg128BitsDownCast<72> for Reg128Bits<107> {}
impl Reg128BitsDownCast<73> for Reg128Bits<107> {}
impl Reg128BitsDownCast<74> for Reg128Bits<107> {}
impl Reg128BitsDownCast<75> for Reg128Bits<107> {}
impl Reg128BitsDownCast<76> for Reg128Bits<107> {}
impl Reg128BitsDownCast<77> for Reg128Bits<107> {}
impl Reg128BitsDownCast<78> for Reg128Bits<107> {}
impl Reg128BitsDownCast<79> for Reg128Bits<107> {}
impl Reg128BitsDownCast<80> for Reg128Bits<107> {}
impl Reg128BitsDownCast<81> for Reg128Bits<107> {}
impl Reg128BitsDownCast<82> for Reg128Bits<107> {}
impl Reg128BitsDownCast<83> for Reg128Bits<107> {}
impl Reg128BitsDownCast<84> for Reg128Bits<107> {}
impl Reg128BitsDownCast<85> for Reg128Bits<107> {}
impl Reg128BitsDownCast<86> for Reg128Bits<107> {}
impl Reg128BitsDownCast<87> for Reg128Bits<107> {}
impl Reg128BitsDownCast<88> for Reg128Bits<107> {}
impl Reg128BitsDownCast<89> for Reg128Bits<107> {}
impl Reg128BitsDownCast<90> for Reg128Bits<107> {}
impl Reg128BitsDownCast<91> for Reg128Bits<107> {}
impl Reg128BitsDownCast<92> for Reg128Bits<107> {}
impl Reg128BitsDownCast<93> for Reg128Bits<107> {}
impl Reg128BitsDownCast<94> for Reg128Bits<107> {}
impl Reg128BitsDownCast<95> for Reg128Bits<107> {}
impl Reg128BitsDownCast<96> for Reg128Bits<107> {}
impl Reg128BitsDownCast<97> for Reg128Bits<107> {}
impl Reg128BitsDownCast<98> for Reg128Bits<107> {}
impl Reg128BitsDownCast<99> for Reg128Bits<107> {}
impl Reg128BitsDownCast<100> for Reg128Bits<107> {}
impl Reg128BitsDownCast<101> for Reg128Bits<107> {}
impl Reg128BitsDownCast<102> for Reg128Bits<107> {}
impl Reg128BitsDownCast<103> for Reg128Bits<107> {}
impl Reg128BitsDownCast<104> for Reg128Bits<107> {}
impl Reg128BitsDownCast<105> for Reg128Bits<107> {}
impl Reg128BitsDownCast<106> for Reg128Bits<107> {}
impl Reg128BitsDownCast<107> for Reg128Bits<107> {}
impl Reg128BitsDownCast<1> for Reg128Bits<108> {}
impl Reg128BitsConcat<1, 109> for Reg128Bits<108> {}
impl Reg128BitsDownCast<2> for Reg128Bits<108> {}
impl Reg128BitsConcat<2, 110> for Reg128Bits<108> {}
impl Reg128BitsDownCast<3> for Reg128Bits<108> {}
impl Reg128BitsConcat<3, 111> for Reg128Bits<108> {}
impl Reg128BitsDownCast<4> for Reg128Bits<108> {}
impl Reg128BitsConcat<4, 112> for Reg128Bits<108> {}
impl Reg128BitsDownCast<5> for Reg128Bits<108> {}
impl Reg128BitsConcat<5, 113> for Reg128Bits<108> {}
impl Reg128BitsDownCast<6> for Reg128Bits<108> {}
impl Reg128BitsConcat<6, 114> for Reg128Bits<108> {}
impl Reg128BitsDownCast<7> for Reg128Bits<108> {}
impl Reg128BitsConcat<7, 115> for Reg128Bits<108> {}
impl Reg128BitsDownCast<8> for Reg128Bits<108> {}
impl Reg128BitsConcat<8, 116> for Reg128Bits<108> {}
impl Reg128BitsDownCast<9> for Reg128Bits<108> {}
impl Reg128BitsConcat<9, 117> for Reg128Bits<108> {}
impl Reg128BitsDownCast<10> for Reg128Bits<108> {}
impl Reg128BitsConcat<10, 118> for Reg128Bits<108> {}
impl Reg128BitsDownCast<11> for Reg128Bits<108> {}
impl Reg128BitsConcat<11, 119> for Reg128Bits<108> {}
impl Reg128BitsDownCast<12> for Reg128Bits<108> {}
impl Reg128BitsConcat<12, 120> for Reg128Bits<108> {}
impl Reg128BitsDownCast<13> for Reg128Bits<108> {}
impl Reg128BitsConcat<13, 121> for Reg128Bits<108> {}
impl Reg128BitsDownCast<14> for Reg128Bits<108> {}
impl Reg128BitsConcat<14, 122> for Reg128Bits<108> {}
impl Reg128BitsDownCast<15> for Reg128Bits<108> {}
impl Reg128BitsConcat<15, 123> for Reg128Bits<108> {}
impl Reg128BitsDownCast<16> for Reg128Bits<108> {}
impl Reg128BitsConcat<16, 124> for Reg128Bits<108> {}
impl Reg128BitsDownCast<17> for Reg128Bits<108> {}
impl Reg128BitsConcat<17, 125> for Reg128Bits<108> {}
impl Reg128BitsDownCast<18> for Reg128Bits<108> {}
impl Reg128BitsConcat<18, 126> for Reg128Bits<108> {}
impl Reg128BitsDownCast<19> for Reg128Bits<108> {}
impl Reg128BitsConcat<19, 127> for Reg128Bits<108> {}
impl Reg128BitsDownCast<20> for Reg128Bits<108> {}
impl Reg128BitsConcat<20, 128> for Reg128Bits<108> {}
impl Reg128BitsDownCast<21> for Reg128Bits<108> {}
impl Reg128BitsDownCast<22> for Reg128Bits<108> {}
impl Reg128BitsDownCast<23> for Reg128Bits<108> {}
impl Reg128BitsDownCast<24> for Reg128Bits<108> {}
impl Reg128BitsDownCast<25> for Reg128Bits<108> {}
impl Reg128BitsDownCast<26> for Reg128Bits<108> {}
impl Reg128BitsDownCast<27> for Reg128Bits<108> {}
impl Reg128BitsDownCast<28> for Reg128Bits<108> {}
impl Reg128BitsDownCast<29> for Reg128Bits<108> {}
impl Reg128BitsDownCast<30> for Reg128Bits<108> {}
impl Reg128BitsDownCast<31> for Reg128Bits<108> {}
impl Reg128BitsDownCast<32> for Reg128Bits<108> {}
impl Reg128BitsDownCast<33> for Reg128Bits<108> {}
impl Reg128BitsDownCast<34> for Reg128Bits<108> {}
impl Reg128BitsDownCast<35> for Reg128Bits<108> {}
impl Reg128BitsDownCast<36> for Reg128Bits<108> {}
impl Reg128BitsDownCast<37> for Reg128Bits<108> {}
impl Reg128BitsDownCast<38> for Reg128Bits<108> {}
impl Reg128BitsDownCast<39> for Reg128Bits<108> {}
impl Reg128BitsDownCast<40> for Reg128Bits<108> {}
impl Reg128BitsDownCast<41> for Reg128Bits<108> {}
impl Reg128BitsDownCast<42> for Reg128Bits<108> {}
impl Reg128BitsDownCast<43> for Reg128Bits<108> {}
impl Reg128BitsDownCast<44> for Reg128Bits<108> {}
impl Reg128BitsDownCast<45> for Reg128Bits<108> {}
impl Reg128BitsDownCast<46> for Reg128Bits<108> {}
impl Reg128BitsDownCast<47> for Reg128Bits<108> {}
impl Reg128BitsDownCast<48> for Reg128Bits<108> {}
impl Reg128BitsDownCast<49> for Reg128Bits<108> {}
impl Reg128BitsDownCast<50> for Reg128Bits<108> {}
impl Reg128BitsDownCast<51> for Reg128Bits<108> {}
impl Reg128BitsDownCast<52> for Reg128Bits<108> {}
impl Reg128BitsDownCast<53> for Reg128Bits<108> {}
impl Reg128BitsDownCast<54> for Reg128Bits<108> {}
impl Reg128BitsDownCast<55> for Reg128Bits<108> {}
impl Reg128BitsDownCast<56> for Reg128Bits<108> {}
impl Reg128BitsDownCast<57> for Reg128Bits<108> {}
impl Reg128BitsDownCast<58> for Reg128Bits<108> {}
impl Reg128BitsDownCast<59> for Reg128Bits<108> {}
impl Reg128BitsDownCast<60> for Reg128Bits<108> {}
impl Reg128BitsDownCast<61> for Reg128Bits<108> {}
impl Reg128BitsDownCast<62> for Reg128Bits<108> {}
impl Reg128BitsDownCast<63> for Reg128Bits<108> {}
impl Reg128BitsDownCast<64> for Reg128Bits<108> {}
impl Reg128BitsDownCast<65> for Reg128Bits<108> {}
impl Reg128BitsDownCast<66> for Reg128Bits<108> {}
impl Reg128BitsDownCast<67> for Reg128Bits<108> {}
impl Reg128BitsDownCast<68> for Reg128Bits<108> {}
impl Reg128BitsDownCast<69> for Reg128Bits<108> {}
impl Reg128BitsDownCast<70> for Reg128Bits<108> {}
impl Reg128BitsDownCast<71> for Reg128Bits<108> {}
impl Reg128BitsDownCast<72> for Reg128Bits<108> {}
impl Reg128BitsDownCast<73> for Reg128Bits<108> {}
impl Reg128BitsDownCast<74> for Reg128Bits<108> {}
impl Reg128BitsDownCast<75> for Reg128Bits<108> {}
impl Reg128BitsDownCast<76> for Reg128Bits<108> {}
impl Reg128BitsDownCast<77> for Reg128Bits<108> {}
impl Reg128BitsDownCast<78> for Reg128Bits<108> {}
impl Reg128BitsDownCast<79> for Reg128Bits<108> {}
impl Reg128BitsDownCast<80> for Reg128Bits<108> {}
impl Reg128BitsDownCast<81> for Reg128Bits<108> {}
impl Reg128BitsDownCast<82> for Reg128Bits<108> {}
impl Reg128BitsDownCast<83> for Reg128Bits<108> {}
impl Reg128BitsDownCast<84> for Reg128Bits<108> {}
impl Reg128BitsDownCast<85> for Reg128Bits<108> {}
impl Reg128BitsDownCast<86> for Reg128Bits<108> {}
impl Reg128BitsDownCast<87> for Reg128Bits<108> {}
impl Reg128BitsDownCast<88> for Reg128Bits<108> {}
impl Reg128BitsDownCast<89> for Reg128Bits<108> {}
impl Reg128BitsDownCast<90> for Reg128Bits<108> {}
impl Reg128BitsDownCast<91> for Reg128Bits<108> {}
impl Reg128BitsDownCast<92> for Reg128Bits<108> {}
impl Reg128BitsDownCast<93> for Reg128Bits<108> {}
impl Reg128BitsDownCast<94> for Reg128Bits<108> {}
impl Reg128BitsDownCast<95> for Reg128Bits<108> {}
impl Reg128BitsDownCast<96> for Reg128Bits<108> {}
impl Reg128BitsDownCast<97> for Reg128Bits<108> {}
impl Reg128BitsDownCast<98> for Reg128Bits<108> {}
impl Reg128BitsDownCast<99> for Reg128Bits<108> {}
impl Reg128BitsDownCast<100> for Reg128Bits<108> {}
impl Reg128BitsDownCast<101> for Reg128Bits<108> {}
impl Reg128BitsDownCast<102> for Reg128Bits<108> {}
impl Reg128BitsDownCast<103> for Reg128Bits<108> {}
impl Reg128BitsDownCast<104> for Reg128Bits<108> {}
impl Reg128BitsDownCast<105> for Reg128Bits<108> {}
impl Reg128BitsDownCast<106> for Reg128Bits<108> {}
impl Reg128BitsDownCast<107> for Reg128Bits<108> {}
impl Reg128BitsDownCast<108> for Reg128Bits<108> {}
impl Reg128BitsDownCast<1> for Reg128Bits<109> {}
impl Reg128BitsConcat<1, 110> for Reg128Bits<109> {}
impl Reg128BitsDownCast<2> for Reg128Bits<109> {}
impl Reg128BitsConcat<2, 111> for Reg128Bits<109> {}
impl Reg128BitsDownCast<3> for Reg128Bits<109> {}
impl Reg128BitsConcat<3, 112> for Reg128Bits<109> {}
impl Reg128BitsDownCast<4> for Reg128Bits<109> {}
impl Reg128BitsConcat<4, 113> for Reg128Bits<109> {}
impl Reg128BitsDownCast<5> for Reg128Bits<109> {}
impl Reg128BitsConcat<5, 114> for Reg128Bits<109> {}
impl Reg128BitsDownCast<6> for Reg128Bits<109> {}
impl Reg128BitsConcat<6, 115> for Reg128Bits<109> {}
impl Reg128BitsDownCast<7> for Reg128Bits<109> {}
impl Reg128BitsConcat<7, 116> for Reg128Bits<109> {}
impl Reg128BitsDownCast<8> for Reg128Bits<109> {}
impl Reg128BitsConcat<8, 117> for Reg128Bits<109> {}
impl Reg128BitsDownCast<9> for Reg128Bits<109> {}
impl Reg128BitsConcat<9, 118> for Reg128Bits<109> {}
impl Reg128BitsDownCast<10> for Reg128Bits<109> {}
impl Reg128BitsConcat<10, 119> for Reg128Bits<109> {}
impl Reg128BitsDownCast<11> for Reg128Bits<109> {}
impl Reg128BitsConcat<11, 120> for Reg128Bits<109> {}
impl Reg128BitsDownCast<12> for Reg128Bits<109> {}
impl Reg128BitsConcat<12, 121> for Reg128Bits<109> {}
impl Reg128BitsDownCast<13> for Reg128Bits<109> {}
impl Reg128BitsConcat<13, 122> for Reg128Bits<109> {}
impl Reg128BitsDownCast<14> for Reg128Bits<109> {}
impl Reg128BitsConcat<14, 123> for Reg128Bits<109> {}
impl Reg128BitsDownCast<15> for Reg128Bits<109> {}
impl Reg128BitsConcat<15, 124> for Reg128Bits<109> {}
impl Reg128BitsDownCast<16> for Reg128Bits<109> {}
impl Reg128BitsConcat<16, 125> for Reg128Bits<109> {}
impl Reg128BitsDownCast<17> for Reg128Bits<109> {}
impl Reg128BitsConcat<17, 126> for Reg128Bits<109> {}
impl Reg128BitsDownCast<18> for Reg128Bits<109> {}
impl Reg128BitsConcat<18, 127> for Reg128Bits<109> {}
impl Reg128BitsDownCast<19> for Reg128Bits<109> {}
impl Reg128BitsConcat<19, 128> for Reg128Bits<109> {}
impl Reg128BitsDownCast<20> for Reg128Bits<109> {}
impl Reg128BitsDownCast<21> for Reg128Bits<109> {}
impl Reg128BitsDownCast<22> for Reg128Bits<109> {}
impl Reg128BitsDownCast<23> for Reg128Bits<109> {}
impl Reg128BitsDownCast<24> for Reg128Bits<109> {}
impl Reg128BitsDownCast<25> for Reg128Bits<109> {}
impl Reg128BitsDownCast<26> for Reg128Bits<109> {}
impl Reg128BitsDownCast<27> for Reg128Bits<109> {}
impl Reg128BitsDownCast<28> for Reg128Bits<109> {}
impl Reg128BitsDownCast<29> for Reg128Bits<109> {}
impl Reg128BitsDownCast<30> for Reg128Bits<109> {}
impl Reg128BitsDownCast<31> for Reg128Bits<109> {}
impl Reg128BitsDownCast<32> for Reg128Bits<109> {}
impl Reg128BitsDownCast<33> for Reg128Bits<109> {}
impl Reg128BitsDownCast<34> for Reg128Bits<109> {}
impl Reg128BitsDownCast<35> for Reg128Bits<109> {}
impl Reg128BitsDownCast<36> for Reg128Bits<109> {}
impl Reg128BitsDownCast<37> for Reg128Bits<109> {}
impl Reg128BitsDownCast<38> for Reg128Bits<109> {}
impl Reg128BitsDownCast<39> for Reg128Bits<109> {}
impl Reg128BitsDownCast<40> for Reg128Bits<109> {}
impl Reg128BitsDownCast<41> for Reg128Bits<109> {}
impl Reg128BitsDownCast<42> for Reg128Bits<109> {}
impl Reg128BitsDownCast<43> for Reg128Bits<109> {}
impl Reg128BitsDownCast<44> for Reg128Bits<109> {}
impl Reg128BitsDownCast<45> for Reg128Bits<109> {}
impl Reg128BitsDownCast<46> for Reg128Bits<109> {}
impl Reg128BitsDownCast<47> for Reg128Bits<109> {}
impl Reg128BitsDownCast<48> for Reg128Bits<109> {}
impl Reg128BitsDownCast<49> for Reg128Bits<109> {}
impl Reg128BitsDownCast<50> for Reg128Bits<109> {}
impl Reg128BitsDownCast<51> for Reg128Bits<109> {}
impl Reg128BitsDownCast<52> for Reg128Bits<109> {}
impl Reg128BitsDownCast<53> for Reg128Bits<109> {}
impl Reg128BitsDownCast<54> for Reg128Bits<109> {}
impl Reg128BitsDownCast<55> for Reg128Bits<109> {}
impl Reg128BitsDownCast<56> for Reg128Bits<109> {}
impl Reg128BitsDownCast<57> for Reg128Bits<109> {}
impl Reg128BitsDownCast<58> for Reg128Bits<109> {}
impl Reg128BitsDownCast<59> for Reg128Bits<109> {}
impl Reg128BitsDownCast<60> for Reg128Bits<109> {}
impl Reg128BitsDownCast<61> for Reg128Bits<109> {}
impl Reg128BitsDownCast<62> for Reg128Bits<109> {}
impl Reg128BitsDownCast<63> for Reg128Bits<109> {}
impl Reg128BitsDownCast<64> for Reg128Bits<109> {}
impl Reg128BitsDownCast<65> for Reg128Bits<109> {}
impl Reg128BitsDownCast<66> for Reg128Bits<109> {}
impl Reg128BitsDownCast<67> for Reg128Bits<109> {}
impl Reg128BitsDownCast<68> for Reg128Bits<109> {}
impl Reg128BitsDownCast<69> for Reg128Bits<109> {}
impl Reg128BitsDownCast<70> for Reg128Bits<109> {}
impl Reg128BitsDownCast<71> for Reg128Bits<109> {}
impl Reg128BitsDownCast<72> for Reg128Bits<109> {}
impl Reg128BitsDownCast<73> for Reg128Bits<109> {}
impl Reg128BitsDownCast<74> for Reg128Bits<109> {}
impl Reg128BitsDownCast<75> for Reg128Bits<109> {}
impl Reg128BitsDownCast<76> for Reg128Bits<109> {}
impl Reg128BitsDownCast<77> for Reg128Bits<109> {}
impl Reg128BitsDownCast<78> for Reg128Bits<109> {}
impl Reg128BitsDownCast<79> for Reg128Bits<109> {}
impl Reg128BitsDownCast<80> for Reg128Bits<109> {}
impl Reg128BitsDownCast<81> for Reg128Bits<109> {}
impl Reg128BitsDownCast<82> for Reg128Bits<109> {}
impl Reg128BitsDownCast<83> for Reg128Bits<109> {}
impl Reg128BitsDownCast<84> for Reg128Bits<109> {}
impl Reg128BitsDownCast<85> for Reg128Bits<109> {}
impl Reg128BitsDownCast<86> for Reg128Bits<109> {}
impl Reg128BitsDownCast<87> for Reg128Bits<109> {}
impl Reg128BitsDownCast<88> for Reg128Bits<109> {}
impl Reg128BitsDownCast<89> for Reg128Bits<109> {}
impl Reg128BitsDownCast<90> for Reg128Bits<109> {}
impl Reg128BitsDownCast<91> for Reg128Bits<109> {}
impl Reg128BitsDownCast<92> for Reg128Bits<109> {}
impl Reg128BitsDownCast<93> for Reg128Bits<109> {}
impl Reg128BitsDownCast<94> for Reg128Bits<109> {}
impl Reg128BitsDownCast<95> for Reg128Bits<109> {}
impl Reg128BitsDownCast<96> for Reg128Bits<109> {}
impl Reg128BitsDownCast<97> for Reg128Bits<109> {}
impl Reg128BitsDownCast<98> for Reg128Bits<109> {}
impl Reg128BitsDownCast<99> for Reg128Bits<109> {}
impl Reg128BitsDownCast<100> for Reg128Bits<109> {}
impl Reg128BitsDownCast<101> for Reg128Bits<109> {}
impl Reg128BitsDownCast<102> for Reg128Bits<109> {}
impl Reg128BitsDownCast<103> for Reg128Bits<109> {}
impl Reg128BitsDownCast<104> for Reg128Bits<109> {}
impl Reg128BitsDownCast<105> for Reg128Bits<109> {}
impl Reg128BitsDownCast<106> for Reg128Bits<109> {}
impl Reg128BitsDownCast<107> for Reg128Bits<109> {}
impl Reg128BitsDownCast<108> for Reg128Bits<109> {}
impl Reg128BitsDownCast<109> for Reg128Bits<109> {}
impl Reg128BitsDownCast<1> for Reg128Bits<110> {}
impl Reg128BitsConcat<1, 111> for Reg128Bits<110> {}
impl Reg128BitsDownCast<2> for Reg128Bits<110> {}
impl Reg128BitsConcat<2, 112> for Reg128Bits<110> {}
impl Reg128BitsDownCast<3> for Reg128Bits<110> {}
impl Reg128BitsConcat<3, 113> for Reg128Bits<110> {}
impl Reg128BitsDownCast<4> for Reg128Bits<110> {}
impl Reg128BitsConcat<4, 114> for Reg128Bits<110> {}
impl Reg128BitsDownCast<5> for Reg128Bits<110> {}
impl Reg128BitsConcat<5, 115> for Reg128Bits<110> {}
impl Reg128BitsDownCast<6> for Reg128Bits<110> {}
impl Reg128BitsConcat<6, 116> for Reg128Bits<110> {}
impl Reg128BitsDownCast<7> for Reg128Bits<110> {}
impl Reg128BitsConcat<7, 117> for Reg128Bits<110> {}
impl Reg128BitsDownCast<8> for Reg128Bits<110> {}
impl Reg128BitsConcat<8, 118> for Reg128Bits<110> {}
impl Reg128BitsDownCast<9> for Reg128Bits<110> {}
impl Reg128BitsConcat<9, 119> for Reg128Bits<110> {}
impl Reg128BitsDownCast<10> for Reg128Bits<110> {}
impl Reg128BitsConcat<10, 120> for Reg128Bits<110> {}
impl Reg128BitsDownCast<11> for Reg128Bits<110> {}
impl Reg128BitsConcat<11, 121> for Reg128Bits<110> {}
impl Reg128BitsDownCast<12> for Reg128Bits<110> {}
impl Reg128BitsConcat<12, 122> for Reg128Bits<110> {}
impl Reg128BitsDownCast<13> for Reg128Bits<110> {}
impl Reg128BitsConcat<13, 123> for Reg128Bits<110> {}
impl Reg128BitsDownCast<14> for Reg128Bits<110> {}
impl Reg128BitsConcat<14, 124> for Reg128Bits<110> {}
impl Reg128BitsDownCast<15> for Reg128Bits<110> {}
impl Reg128BitsConcat<15, 125> for Reg128Bits<110> {}
impl Reg128BitsDownCast<16> for Reg128Bits<110> {}
impl Reg128BitsConcat<16, 126> for Reg128Bits<110> {}
impl Reg128BitsDownCast<17> for Reg128Bits<110> {}
impl Reg128BitsConcat<17, 127> for Reg128Bits<110> {}
impl Reg128BitsDownCast<18> for Reg128Bits<110> {}
impl Reg128BitsConcat<18, 128> for Reg128Bits<110> {}
impl Reg128BitsDownCast<19> for Reg128Bits<110> {}
impl Reg128BitsDownCast<20> for Reg128Bits<110> {}
impl Reg128BitsDownCast<21> for Reg128Bits<110> {}
impl Reg128BitsDownCast<22> for Reg128Bits<110> {}
impl Reg128BitsDownCast<23> for Reg128Bits<110> {}
impl Reg128BitsDownCast<24> for Reg128Bits<110> {}
impl Reg128BitsDownCast<25> for Reg128Bits<110> {}
impl Reg128BitsDownCast<26> for Reg128Bits<110> {}
impl Reg128BitsDownCast<27> for Reg128Bits<110> {}
impl Reg128BitsDownCast<28> for Reg128Bits<110> {}
impl Reg128BitsDownCast<29> for Reg128Bits<110> {}
impl Reg128BitsDownCast<30> for Reg128Bits<110> {}
impl Reg128BitsDownCast<31> for Reg128Bits<110> {}
impl Reg128BitsDownCast<32> for Reg128Bits<110> {}
impl Reg128BitsDownCast<33> for Reg128Bits<110> {}
impl Reg128BitsDownCast<34> for Reg128Bits<110> {}
impl Reg128BitsDownCast<35> for Reg128Bits<110> {}
impl Reg128BitsDownCast<36> for Reg128Bits<110> {}
impl Reg128BitsDownCast<37> for Reg128Bits<110> {}
impl Reg128BitsDownCast<38> for Reg128Bits<110> {}
impl Reg128BitsDownCast<39> for Reg128Bits<110> {}
impl Reg128BitsDownCast<40> for Reg128Bits<110> {}
impl Reg128BitsDownCast<41> for Reg128Bits<110> {}
impl Reg128BitsDownCast<42> for Reg128Bits<110> {}
impl Reg128BitsDownCast<43> for Reg128Bits<110> {}
impl Reg128BitsDownCast<44> for Reg128Bits<110> {}
impl Reg128BitsDownCast<45> for Reg128Bits<110> {}
impl Reg128BitsDownCast<46> for Reg128Bits<110> {}
impl Reg128BitsDownCast<47> for Reg128Bits<110> {}
impl Reg128BitsDownCast<48> for Reg128Bits<110> {}
impl Reg128BitsDownCast<49> for Reg128Bits<110> {}
impl Reg128BitsDownCast<50> for Reg128Bits<110> {}
impl Reg128BitsDownCast<51> for Reg128Bits<110> {}
impl Reg128BitsDownCast<52> for Reg128Bits<110> {}
impl Reg128BitsDownCast<53> for Reg128Bits<110> {}
impl Reg128BitsDownCast<54> for Reg128Bits<110> {}
impl Reg128BitsDownCast<55> for Reg128Bits<110> {}
impl Reg128BitsDownCast<56> for Reg128Bits<110> {}
impl Reg128BitsDownCast<57> for Reg128Bits<110> {}
impl Reg128BitsDownCast<58> for Reg128Bits<110> {}
impl Reg128BitsDownCast<59> for Reg128Bits<110> {}
impl Reg128BitsDownCast<60> for Reg128Bits<110> {}
impl Reg128BitsDownCast<61> for Reg128Bits<110> {}
impl Reg128BitsDownCast<62> for Reg128Bits<110> {}
impl Reg128BitsDownCast<63> for Reg128Bits<110> {}
impl Reg128BitsDownCast<64> for Reg128Bits<110> {}
impl Reg128BitsDownCast<65> for Reg128Bits<110> {}
impl Reg128BitsDownCast<66> for Reg128Bits<110> {}
impl Reg128BitsDownCast<67> for Reg128Bits<110> {}
impl Reg128BitsDownCast<68> for Reg128Bits<110> {}
impl Reg128BitsDownCast<69> for Reg128Bits<110> {}
impl Reg128BitsDownCast<70> for Reg128Bits<110> {}
impl Reg128BitsDownCast<71> for Reg128Bits<110> {}
impl Reg128BitsDownCast<72> for Reg128Bits<110> {}
impl Reg128BitsDownCast<73> for Reg128Bits<110> {}
impl Reg128BitsDownCast<74> for Reg128Bits<110> {}
impl Reg128BitsDownCast<75> for Reg128Bits<110> {}
impl Reg128BitsDownCast<76> for Reg128Bits<110> {}
impl Reg128BitsDownCast<77> for Reg128Bits<110> {}
impl Reg128BitsDownCast<78> for Reg128Bits<110> {}
impl Reg128BitsDownCast<79> for Reg128Bits<110> {}
impl Reg128BitsDownCast<80> for Reg128Bits<110> {}
impl Reg128BitsDownCast<81> for Reg128Bits<110> {}
impl Reg128BitsDownCast<82> for Reg128Bits<110> {}
impl Reg128BitsDownCast<83> for Reg128Bits<110> {}
impl Reg128BitsDownCast<84> for Reg128Bits<110> {}
impl Reg128BitsDownCast<85> for Reg128Bits<110> {}
impl Reg128BitsDownCast<86> for Reg128Bits<110> {}
impl Reg128BitsDownCast<87> for Reg128Bits<110> {}
impl Reg128BitsDownCast<88> for Reg128Bits<110> {}
impl Reg128BitsDownCast<89> for Reg128Bits<110> {}
impl Reg128BitsDownCast<90> for Reg128Bits<110> {}
impl Reg128BitsDownCast<91> for Reg128Bits<110> {}
impl Reg128BitsDownCast<92> for Reg128Bits<110> {}
impl Reg128BitsDownCast<93> for Reg128Bits<110> {}
impl Reg128BitsDownCast<94> for Reg128Bits<110> {}
impl Reg128BitsDownCast<95> for Reg128Bits<110> {}
impl Reg128BitsDownCast<96> for Reg128Bits<110> {}
impl Reg128BitsDownCast<97> for Reg128Bits<110> {}
impl Reg128BitsDownCast<98> for Reg128Bits<110> {}
impl Reg128BitsDownCast<99> for Reg128Bits<110> {}
impl Reg128BitsDownCast<100> for Reg128Bits<110> {}
impl Reg128BitsDownCast<101> for Reg128Bits<110> {}
impl Reg128BitsDownCast<102> for Reg128Bits<110> {}
impl Reg128BitsDownCast<103> for Reg128Bits<110> {}
impl Reg128BitsDownCast<104> for Reg128Bits<110> {}
impl Reg128BitsDownCast<105> for Reg128Bits<110> {}
impl Reg128BitsDownCast<106> for Reg128Bits<110> {}
impl Reg128BitsDownCast<107> for Reg128Bits<110> {}
impl Reg128BitsDownCast<108> for Reg128Bits<110> {}
impl Reg128BitsDownCast<109> for Reg128Bits<110> {}
impl Reg128BitsDownCast<110> for Reg128Bits<110> {}
impl Reg128BitsDownCast<1> for Reg128Bits<111> {}
impl Reg128BitsConcat<1, 112> for Reg128Bits<111> {}
impl Reg128BitsDownCast<2> for Reg128Bits<111> {}
impl Reg128BitsConcat<2, 113> for Reg128Bits<111> {}
impl Reg128BitsDownCast<3> for Reg128Bits<111> {}
impl Reg128BitsConcat<3, 114> for Reg128Bits<111> {}
impl Reg128BitsDownCast<4> for Reg128Bits<111> {}
impl Reg128BitsConcat<4, 115> for Reg128Bits<111> {}
impl Reg128BitsDownCast<5> for Reg128Bits<111> {}
impl Reg128BitsConcat<5, 116> for Reg128Bits<111> {}
impl Reg128BitsDownCast<6> for Reg128Bits<111> {}
impl Reg128BitsConcat<6, 117> for Reg128Bits<111> {}
impl Reg128BitsDownCast<7> for Reg128Bits<111> {}
impl Reg128BitsConcat<7, 118> for Reg128Bits<111> {}
impl Reg128BitsDownCast<8> for Reg128Bits<111> {}
impl Reg128BitsConcat<8, 119> for Reg128Bits<111> {}
impl Reg128BitsDownCast<9> for Reg128Bits<111> {}
impl Reg128BitsConcat<9, 120> for Reg128Bits<111> {}
impl Reg128BitsDownCast<10> for Reg128Bits<111> {}
impl Reg128BitsConcat<10, 121> for Reg128Bits<111> {}
impl Reg128BitsDownCast<11> for Reg128Bits<111> {}
impl Reg128BitsConcat<11, 122> for Reg128Bits<111> {}
impl Reg128BitsDownCast<12> for Reg128Bits<111> {}
impl Reg128BitsConcat<12, 123> for Reg128Bits<111> {}
impl Reg128BitsDownCast<13> for Reg128Bits<111> {}
impl Reg128BitsConcat<13, 124> for Reg128Bits<111> {}
impl Reg128BitsDownCast<14> for Reg128Bits<111> {}
impl Reg128BitsConcat<14, 125> for Reg128Bits<111> {}
impl Reg128BitsDownCast<15> for Reg128Bits<111> {}
impl Reg128BitsConcat<15, 126> for Reg128Bits<111> {}
impl Reg128BitsDownCast<16> for Reg128Bits<111> {}
impl Reg128BitsConcat<16, 127> for Reg128Bits<111> {}
impl Reg128BitsDownCast<17> for Reg128Bits<111> {}
impl Reg128BitsConcat<17, 128> for Reg128Bits<111> {}
impl Reg128BitsDownCast<18> for Reg128Bits<111> {}
impl Reg128BitsDownCast<19> for Reg128Bits<111> {}
impl Reg128BitsDownCast<20> for Reg128Bits<111> {}
impl Reg128BitsDownCast<21> for Reg128Bits<111> {}
impl Reg128BitsDownCast<22> for Reg128Bits<111> {}
impl Reg128BitsDownCast<23> for Reg128Bits<111> {}
impl Reg128BitsDownCast<24> for Reg128Bits<111> {}
impl Reg128BitsDownCast<25> for Reg128Bits<111> {}
impl Reg128BitsDownCast<26> for Reg128Bits<111> {}
impl Reg128BitsDownCast<27> for Reg128Bits<111> {}
impl Reg128BitsDownCast<28> for Reg128Bits<111> {}
impl Reg128BitsDownCast<29> for Reg128Bits<111> {}
impl Reg128BitsDownCast<30> for Reg128Bits<111> {}
impl Reg128BitsDownCast<31> for Reg128Bits<111> {}
impl Reg128BitsDownCast<32> for Reg128Bits<111> {}
impl Reg128BitsDownCast<33> for Reg128Bits<111> {}
impl Reg128BitsDownCast<34> for Reg128Bits<111> {}
impl Reg128BitsDownCast<35> for Reg128Bits<111> {}
impl Reg128BitsDownCast<36> for Reg128Bits<111> {}
impl Reg128BitsDownCast<37> for Reg128Bits<111> {}
impl Reg128BitsDownCast<38> for Reg128Bits<111> {}
impl Reg128BitsDownCast<39> for Reg128Bits<111> {}
impl Reg128BitsDownCast<40> for Reg128Bits<111> {}
impl Reg128BitsDownCast<41> for Reg128Bits<111> {}
impl Reg128BitsDownCast<42> for Reg128Bits<111> {}
impl Reg128BitsDownCast<43> for Reg128Bits<111> {}
impl Reg128BitsDownCast<44> for Reg128Bits<111> {}
impl Reg128BitsDownCast<45> for Reg128Bits<111> {}
impl Reg128BitsDownCast<46> for Reg128Bits<111> {}
impl Reg128BitsDownCast<47> for Reg128Bits<111> {}
impl Reg128BitsDownCast<48> for Reg128Bits<111> {}
impl Reg128BitsDownCast<49> for Reg128Bits<111> {}
impl Reg128BitsDownCast<50> for Reg128Bits<111> {}
impl Reg128BitsDownCast<51> for Reg128Bits<111> {}
impl Reg128BitsDownCast<52> for Reg128Bits<111> {}
impl Reg128BitsDownCast<53> for Reg128Bits<111> {}
impl Reg128BitsDownCast<54> for Reg128Bits<111> {}
impl Reg128BitsDownCast<55> for Reg128Bits<111> {}
impl Reg128BitsDownCast<56> for Reg128Bits<111> {}
impl Reg128BitsDownCast<57> for Reg128Bits<111> {}
impl Reg128BitsDownCast<58> for Reg128Bits<111> {}
impl Reg128BitsDownCast<59> for Reg128Bits<111> {}
impl Reg128BitsDownCast<60> for Reg128Bits<111> {}
impl Reg128BitsDownCast<61> for Reg128Bits<111> {}
impl Reg128BitsDownCast<62> for Reg128Bits<111> {}
impl Reg128BitsDownCast<63> for Reg128Bits<111> {}
impl Reg128BitsDownCast<64> for Reg128Bits<111> {}
impl Reg128BitsDownCast<65> for Reg128Bits<111> {}
impl Reg128BitsDownCast<66> for Reg128Bits<111> {}
impl Reg128BitsDownCast<67> for Reg128Bits<111> {}
impl Reg128BitsDownCast<68> for Reg128Bits<111> {}
impl Reg128BitsDownCast<69> for Reg128Bits<111> {}
impl Reg128BitsDownCast<70> for Reg128Bits<111> {}
impl Reg128BitsDownCast<71> for Reg128Bits<111> {}
impl Reg128BitsDownCast<72> for Reg128Bits<111> {}
impl Reg128BitsDownCast<73> for Reg128Bits<111> {}
impl Reg128BitsDownCast<74> for Reg128Bits<111> {}
impl Reg128BitsDownCast<75> for Reg128Bits<111> {}
impl Reg128BitsDownCast<76> for Reg128Bits<111> {}
impl Reg128BitsDownCast<77> for Reg128Bits<111> {}
impl Reg128BitsDownCast<78> for Reg128Bits<111> {}
impl Reg128BitsDownCast<79> for Reg128Bits<111> {}
impl Reg128BitsDownCast<80> for Reg128Bits<111> {}
impl Reg128BitsDownCast<81> for Reg128Bits<111> {}
impl Reg128BitsDownCast<82> for Reg128Bits<111> {}
impl Reg128BitsDownCast<83> for Reg128Bits<111> {}
impl Reg128BitsDownCast<84> for Reg128Bits<111> {}
impl Reg128BitsDownCast<85> for Reg128Bits<111> {}
impl Reg128BitsDownCast<86> for Reg128Bits<111> {}
impl Reg128BitsDownCast<87> for Reg128Bits<111> {}
impl Reg128BitsDownCast<88> for Reg128Bits<111> {}
impl Reg128BitsDownCast<89> for Reg128Bits<111> {}
impl Reg128BitsDownCast<90> for Reg128Bits<111> {}
impl Reg128BitsDownCast<91> for Reg128Bits<111> {}
impl Reg128BitsDownCast<92> for Reg128Bits<111> {}
impl Reg128BitsDownCast<93> for Reg128Bits<111> {}
impl Reg128BitsDownCast<94> for Reg128Bits<111> {}
impl Reg128BitsDownCast<95> for Reg128Bits<111> {}
impl Reg128BitsDownCast<96> for Reg128Bits<111> {}
impl Reg128BitsDownCast<97> for Reg128Bits<111> {}
impl Reg128BitsDownCast<98> for Reg128Bits<111> {}
impl Reg128BitsDownCast<99> for Reg128Bits<111> {}
impl Reg128BitsDownCast<100> for Reg128Bits<111> {}
impl Reg128BitsDownCast<101> for Reg128Bits<111> {}
impl Reg128BitsDownCast<102> for Reg128Bits<111> {}
impl Reg128BitsDownCast<103> for Reg128Bits<111> {}
impl Reg128BitsDownCast<104> for Reg128Bits<111> {}
impl Reg128BitsDownCast<105> for Reg128Bits<111> {}
impl Reg128BitsDownCast<106> for Reg128Bits<111> {}
impl Reg128BitsDownCast<107> for Reg128Bits<111> {}
impl Reg128BitsDownCast<108> for Reg128Bits<111> {}
impl Reg128BitsDownCast<109> for Reg128Bits<111> {}
impl Reg128BitsDownCast<110> for Reg128Bits<111> {}
impl Reg128BitsDownCast<111> for Reg128Bits<111> {}
impl Reg128BitsDownCast<1> for Reg128Bits<112> {}
impl Reg128BitsConcat<1, 113> for Reg128Bits<112> {}
impl Reg128BitsDownCast<2> for Reg128Bits<112> {}
impl Reg128BitsConcat<2, 114> for Reg128Bits<112> {}
impl Reg128BitsDownCast<3> for Reg128Bits<112> {}
impl Reg128BitsConcat<3, 115> for Reg128Bits<112> {}
impl Reg128BitsDownCast<4> for Reg128Bits<112> {}
impl Reg128BitsConcat<4, 116> for Reg128Bits<112> {}
impl Reg128BitsDownCast<5> for Reg128Bits<112> {}
impl Reg128BitsConcat<5, 117> for Reg128Bits<112> {}
impl Reg128BitsDownCast<6> for Reg128Bits<112> {}
impl Reg128BitsConcat<6, 118> for Reg128Bits<112> {}
impl Reg128BitsDownCast<7> for Reg128Bits<112> {}
impl Reg128BitsConcat<7, 119> for Reg128Bits<112> {}
impl Reg128BitsDownCast<8> for Reg128Bits<112> {}
impl Reg128BitsConcat<8, 120> for Reg128Bits<112> {}
impl Reg128BitsDownCast<9> for Reg128Bits<112> {}
impl Reg128BitsConcat<9, 121> for Reg128Bits<112> {}
impl Reg128BitsDownCast<10> for Reg128Bits<112> {}
impl Reg128BitsConcat<10, 122> for Reg128Bits<112> {}
impl Reg128BitsDownCast<11> for Reg128Bits<112> {}
impl Reg128BitsConcat<11, 123> for Reg128Bits<112> {}
impl Reg128BitsDownCast<12> for Reg128Bits<112> {}
impl Reg128BitsConcat<12, 124> for Reg128Bits<112> {}
impl Reg128BitsDownCast<13> for Reg128Bits<112> {}
impl Reg128BitsConcat<13, 125> for Reg128Bits<112> {}
impl Reg128BitsDownCast<14> for Reg128Bits<112> {}
impl Reg128BitsConcat<14, 126> for Reg128Bits<112> {}
impl Reg128BitsDownCast<15> for Reg128Bits<112> {}
impl Reg128BitsConcat<15, 127> for Reg128Bits<112> {}
impl Reg128BitsDownCast<16> for Reg128Bits<112> {}
impl Reg128BitsConcat<16, 128> for Reg128Bits<112> {}
impl Reg128BitsDownCast<17> for Reg128Bits<112> {}
impl Reg128BitsDownCast<18> for Reg128Bits<112> {}
impl Reg128BitsDownCast<19> for Reg128Bits<112> {}
impl Reg128BitsDownCast<20> for Reg128Bits<112> {}
impl Reg128BitsDownCast<21> for Reg128Bits<112> {}
impl Reg128BitsDownCast<22> for Reg128Bits<112> {}
impl Reg128BitsDownCast<23> for Reg128Bits<112> {}
impl Reg128BitsDownCast<24> for Reg128Bits<112> {}
impl Reg128BitsDownCast<25> for Reg128Bits<112> {}
impl Reg128BitsDownCast<26> for Reg128Bits<112> {}
impl Reg128BitsDownCast<27> for Reg128Bits<112> {}
impl Reg128BitsDownCast<28> for Reg128Bits<112> {}
impl Reg128BitsDownCast<29> for Reg128Bits<112> {}
impl Reg128BitsDownCast<30> for Reg128Bits<112> {}
impl Reg128BitsDownCast<31> for Reg128Bits<112> {}
impl Reg128BitsDownCast<32> for Reg128Bits<112> {}
impl Reg128BitsDownCast<33> for Reg128Bits<112> {}
impl Reg128BitsDownCast<34> for Reg128Bits<112> {}
impl Reg128BitsDownCast<35> for Reg128Bits<112> {}
impl Reg128BitsDownCast<36> for Reg128Bits<112> {}
impl Reg128BitsDownCast<37> for Reg128Bits<112> {}
impl Reg128BitsDownCast<38> for Reg128Bits<112> {}
impl Reg128BitsDownCast<39> for Reg128Bits<112> {}
impl Reg128BitsDownCast<40> for Reg128Bits<112> {}
impl Reg128BitsDownCast<41> for Reg128Bits<112> {}
impl Reg128BitsDownCast<42> for Reg128Bits<112> {}
impl Reg128BitsDownCast<43> for Reg128Bits<112> {}
impl Reg128BitsDownCast<44> for Reg128Bits<112> {}
impl Reg128BitsDownCast<45> for Reg128Bits<112> {}
impl Reg128BitsDownCast<46> for Reg128Bits<112> {}
impl Reg128BitsDownCast<47> for Reg128Bits<112> {}
impl Reg128BitsDownCast<48> for Reg128Bits<112> {}
impl Reg128BitsDownCast<49> for Reg128Bits<112> {}
impl Reg128BitsDownCast<50> for Reg128Bits<112> {}
impl Reg128BitsDownCast<51> for Reg128Bits<112> {}
impl Reg128BitsDownCast<52> for Reg128Bits<112> {}
impl Reg128BitsDownCast<53> for Reg128Bits<112> {}
impl Reg128BitsDownCast<54> for Reg128Bits<112> {}
impl Reg128BitsDownCast<55> for Reg128Bits<112> {}
impl Reg128BitsDownCast<56> for Reg128Bits<112> {}
impl Reg128BitsDownCast<57> for Reg128Bits<112> {}
impl Reg128BitsDownCast<58> for Reg128Bits<112> {}
impl Reg128BitsDownCast<59> for Reg128Bits<112> {}
impl Reg128BitsDownCast<60> for Reg128Bits<112> {}
impl Reg128BitsDownCast<61> for Reg128Bits<112> {}
impl Reg128BitsDownCast<62> for Reg128Bits<112> {}
impl Reg128BitsDownCast<63> for Reg128Bits<112> {}
impl Reg128BitsDownCast<64> for Reg128Bits<112> {}
impl Reg128BitsDownCast<65> for Reg128Bits<112> {}
impl Reg128BitsDownCast<66> for Reg128Bits<112> {}
impl Reg128BitsDownCast<67> for Reg128Bits<112> {}
impl Reg128BitsDownCast<68> for Reg128Bits<112> {}
impl Reg128BitsDownCast<69> for Reg128Bits<112> {}
impl Reg128BitsDownCast<70> for Reg128Bits<112> {}
impl Reg128BitsDownCast<71> for Reg128Bits<112> {}
impl Reg128BitsDownCast<72> for Reg128Bits<112> {}
impl Reg128BitsDownCast<73> for Reg128Bits<112> {}
impl Reg128BitsDownCast<74> for Reg128Bits<112> {}
impl Reg128BitsDownCast<75> for Reg128Bits<112> {}
impl Reg128BitsDownCast<76> for Reg128Bits<112> {}
impl Reg128BitsDownCast<77> for Reg128Bits<112> {}
impl Reg128BitsDownCast<78> for Reg128Bits<112> {}
impl Reg128BitsDownCast<79> for Reg128Bits<112> {}
impl Reg128BitsDownCast<80> for Reg128Bits<112> {}
impl Reg128BitsDownCast<81> for Reg128Bits<112> {}
impl Reg128BitsDownCast<82> for Reg128Bits<112> {}
impl Reg128BitsDownCast<83> for Reg128Bits<112> {}
impl Reg128BitsDownCast<84> for Reg128Bits<112> {}
impl Reg128BitsDownCast<85> for Reg128Bits<112> {}
impl Reg128BitsDownCast<86> for Reg128Bits<112> {}
impl Reg128BitsDownCast<87> for Reg128Bits<112> {}
impl Reg128BitsDownCast<88> for Reg128Bits<112> {}
impl Reg128BitsDownCast<89> for Reg128Bits<112> {}
impl Reg128BitsDownCast<90> for Reg128Bits<112> {}
impl Reg128BitsDownCast<91> for Reg128Bits<112> {}
impl Reg128BitsDownCast<92> for Reg128Bits<112> {}
impl Reg128BitsDownCast<93> for Reg128Bits<112> {}
impl Reg128BitsDownCast<94> for Reg128Bits<112> {}
impl Reg128BitsDownCast<95> for Reg128Bits<112> {}
impl Reg128BitsDownCast<96> for Reg128Bits<112> {}
impl Reg128BitsDownCast<97> for Reg128Bits<112> {}
impl Reg128BitsDownCast<98> for Reg128Bits<112> {}
impl Reg128BitsDownCast<99> for Reg128Bits<112> {}
impl Reg128BitsDownCast<100> for Reg128Bits<112> {}
impl Reg128BitsDownCast<101> for Reg128Bits<112> {}
impl Reg128BitsDownCast<102> for Reg128Bits<112> {}
impl Reg128BitsDownCast<103> for Reg128Bits<112> {}
impl Reg128BitsDownCast<104> for Reg128Bits<112> {}
impl Reg128BitsDownCast<105> for Reg128Bits<112> {}
impl Reg128BitsDownCast<106> for Reg128Bits<112> {}
impl Reg128BitsDownCast<107> for Reg128Bits<112> {}
impl Reg128BitsDownCast<108> for Reg128Bits<112> {}
impl Reg128BitsDownCast<109> for Reg128Bits<112> {}
impl Reg128BitsDownCast<110> for Reg128Bits<112> {}
impl Reg128BitsDownCast<111> for Reg128Bits<112> {}
impl Reg128BitsDownCast<112> for Reg128Bits<112> {}
impl Reg128BitsDownCast<1> for Reg128Bits<113> {}
impl Reg128BitsConcat<1, 114> for Reg128Bits<113> {}
impl Reg128BitsDownCast<2> for Reg128Bits<113> {}
impl Reg128BitsConcat<2, 115> for Reg128Bits<113> {}
impl Reg128BitsDownCast<3> for Reg128Bits<113> {}
impl Reg128BitsConcat<3, 116> for Reg128Bits<113> {}
impl Reg128BitsDownCast<4> for Reg128Bits<113> {}
impl Reg128BitsConcat<4, 117> for Reg128Bits<113> {}
impl Reg128BitsDownCast<5> for Reg128Bits<113> {}
impl Reg128BitsConcat<5, 118> for Reg128Bits<113> {}
impl Reg128BitsDownCast<6> for Reg128Bits<113> {}
impl Reg128BitsConcat<6, 119> for Reg128Bits<113> {}
impl Reg128BitsDownCast<7> for Reg128Bits<113> {}
impl Reg128BitsConcat<7, 120> for Reg128Bits<113> {}
impl Reg128BitsDownCast<8> for Reg128Bits<113> {}
impl Reg128BitsConcat<8, 121> for Reg128Bits<113> {}
impl Reg128BitsDownCast<9> for Reg128Bits<113> {}
impl Reg128BitsConcat<9, 122> for Reg128Bits<113> {}
impl Reg128BitsDownCast<10> for Reg128Bits<113> {}
impl Reg128BitsConcat<10, 123> for Reg128Bits<113> {}
impl Reg128BitsDownCast<11> for Reg128Bits<113> {}
impl Reg128BitsConcat<11, 124> for Reg128Bits<113> {}
impl Reg128BitsDownCast<12> for Reg128Bits<113> {}
impl Reg128BitsConcat<12, 125> for Reg128Bits<113> {}
impl Reg128BitsDownCast<13> for Reg128Bits<113> {}
impl Reg128BitsConcat<13, 126> for Reg128Bits<113> {}
impl Reg128BitsDownCast<14> for Reg128Bits<113> {}
impl Reg128BitsConcat<14, 127> for Reg128Bits<113> {}
impl Reg128BitsDownCast<15> for Reg128Bits<113> {}
impl Reg128BitsConcat<15, 128> for Reg128Bits<113> {}
impl Reg128BitsDownCast<16> for Reg128Bits<113> {}
impl Reg128BitsDownCast<17> for Reg128Bits<113> {}
impl Reg128BitsDownCast<18> for Reg128Bits<113> {}
impl Reg128BitsDownCast<19> for Reg128Bits<113> {}
impl Reg128BitsDownCast<20> for Reg128Bits<113> {}
impl Reg128BitsDownCast<21> for Reg128Bits<113> {}
impl Reg128BitsDownCast<22> for Reg128Bits<113> {}
impl Reg128BitsDownCast<23> for Reg128Bits<113> {}
impl Reg128BitsDownCast<24> for Reg128Bits<113> {}
impl Reg128BitsDownCast<25> for Reg128Bits<113> {}
impl Reg128BitsDownCast<26> for Reg128Bits<113> {}
impl Reg128BitsDownCast<27> for Reg128Bits<113> {}
impl Reg128BitsDownCast<28> for Reg128Bits<113> {}
impl Reg128BitsDownCast<29> for Reg128Bits<113> {}
impl Reg128BitsDownCast<30> for Reg128Bits<113> {}
impl Reg128BitsDownCast<31> for Reg128Bits<113> {}
impl Reg128BitsDownCast<32> for Reg128Bits<113> {}
impl Reg128BitsDownCast<33> for Reg128Bits<113> {}
impl Reg128BitsDownCast<34> for Reg128Bits<113> {}
impl Reg128BitsDownCast<35> for Reg128Bits<113> {}
impl Reg128BitsDownCast<36> for Reg128Bits<113> {}
impl Reg128BitsDownCast<37> for Reg128Bits<113> {}
impl Reg128BitsDownCast<38> for Reg128Bits<113> {}
impl Reg128BitsDownCast<39> for Reg128Bits<113> {}
impl Reg128BitsDownCast<40> for Reg128Bits<113> {}
impl Reg128BitsDownCast<41> for Reg128Bits<113> {}
impl Reg128BitsDownCast<42> for Reg128Bits<113> {}
impl Reg128BitsDownCast<43> for Reg128Bits<113> {}
impl Reg128BitsDownCast<44> for Reg128Bits<113> {}
impl Reg128BitsDownCast<45> for Reg128Bits<113> {}
impl Reg128BitsDownCast<46> for Reg128Bits<113> {}
impl Reg128BitsDownCast<47> for Reg128Bits<113> {}
impl Reg128BitsDownCast<48> for Reg128Bits<113> {}
impl Reg128BitsDownCast<49> for Reg128Bits<113> {}
impl Reg128BitsDownCast<50> for Reg128Bits<113> {}
impl Reg128BitsDownCast<51> for Reg128Bits<113> {}
impl Reg128BitsDownCast<52> for Reg128Bits<113> {}
impl Reg128BitsDownCast<53> for Reg128Bits<113> {}
impl Reg128BitsDownCast<54> for Reg128Bits<113> {}
impl Reg128BitsDownCast<55> for Reg128Bits<113> {}
impl Reg128BitsDownCast<56> for Reg128Bits<113> {}
impl Reg128BitsDownCast<57> for Reg128Bits<113> {}
impl Reg128BitsDownCast<58> for Reg128Bits<113> {}
impl Reg128BitsDownCast<59> for Reg128Bits<113> {}
impl Reg128BitsDownCast<60> for Reg128Bits<113> {}
impl Reg128BitsDownCast<61> for Reg128Bits<113> {}
impl Reg128BitsDownCast<62> for Reg128Bits<113> {}
impl Reg128BitsDownCast<63> for Reg128Bits<113> {}
impl Reg128BitsDownCast<64> for Reg128Bits<113> {}
impl Reg128BitsDownCast<65> for Reg128Bits<113> {}
impl Reg128BitsDownCast<66> for Reg128Bits<113> {}
impl Reg128BitsDownCast<67> for Reg128Bits<113> {}
impl Reg128BitsDownCast<68> for Reg128Bits<113> {}
impl Reg128BitsDownCast<69> for Reg128Bits<113> {}
impl Reg128BitsDownCast<70> for Reg128Bits<113> {}
impl Reg128BitsDownCast<71> for Reg128Bits<113> {}
impl Reg128BitsDownCast<72> for Reg128Bits<113> {}
impl Reg128BitsDownCast<73> for Reg128Bits<113> {}
impl Reg128BitsDownCast<74> for Reg128Bits<113> {}
impl Reg128BitsDownCast<75> for Reg128Bits<113> {}
impl Reg128BitsDownCast<76> for Reg128Bits<113> {}
impl Reg128BitsDownCast<77> for Reg128Bits<113> {}
impl Reg128BitsDownCast<78> for Reg128Bits<113> {}
impl Reg128BitsDownCast<79> for Reg128Bits<113> {}
impl Reg128BitsDownCast<80> for Reg128Bits<113> {}
impl Reg128BitsDownCast<81> for Reg128Bits<113> {}
impl Reg128BitsDownCast<82> for Reg128Bits<113> {}
impl Reg128BitsDownCast<83> for Reg128Bits<113> {}
impl Reg128BitsDownCast<84> for Reg128Bits<113> {}
impl Reg128BitsDownCast<85> for Reg128Bits<113> {}
impl Reg128BitsDownCast<86> for Reg128Bits<113> {}
impl Reg128BitsDownCast<87> for Reg128Bits<113> {}
impl Reg128BitsDownCast<88> for Reg128Bits<113> {}
impl Reg128BitsDownCast<89> for Reg128Bits<113> {}
impl Reg128BitsDownCast<90> for Reg128Bits<113> {}
impl Reg128BitsDownCast<91> for Reg128Bits<113> {}
impl Reg128BitsDownCast<92> for Reg128Bits<113> {}
impl Reg128BitsDownCast<93> for Reg128Bits<113> {}
impl Reg128BitsDownCast<94> for Reg128Bits<113> {}
impl Reg128BitsDownCast<95> for Reg128Bits<113> {}
impl Reg128BitsDownCast<96> for Reg128Bits<113> {}
impl Reg128BitsDownCast<97> for Reg128Bits<113> {}
impl Reg128BitsDownCast<98> for Reg128Bits<113> {}
impl Reg128BitsDownCast<99> for Reg128Bits<113> {}
impl Reg128BitsDownCast<100> for Reg128Bits<113> {}
impl Reg128BitsDownCast<101> for Reg128Bits<113> {}
impl Reg128BitsDownCast<102> for Reg128Bits<113> {}
impl Reg128BitsDownCast<103> for Reg128Bits<113> {}
impl Reg128BitsDownCast<104> for Reg128Bits<113> {}
impl Reg128BitsDownCast<105> for Reg128Bits<113> {}
impl Reg128BitsDownCast<106> for Reg128Bits<113> {}
impl Reg128BitsDownCast<107> for Reg128Bits<113> {}
impl Reg128BitsDownCast<108> for Reg128Bits<113> {}
impl Reg128BitsDownCast<109> for Reg128Bits<113> {}
impl Reg128BitsDownCast<110> for Reg128Bits<113> {}
impl Reg128BitsDownCast<111> for Reg128Bits<113> {}
impl Reg128BitsDownCast<112> for Reg128Bits<113> {}
impl Reg128BitsDownCast<113> for Reg128Bits<113> {}
impl Reg128BitsDownCast<1> for Reg128Bits<114> {}
impl Reg128BitsConcat<1, 115> for Reg128Bits<114> {}
impl Reg128BitsDownCast<2> for Reg128Bits<114> {}
impl Reg128BitsConcat<2, 116> for Reg128Bits<114> {}
impl Reg128BitsDownCast<3> for Reg128Bits<114> {}
impl Reg128BitsConcat<3, 117> for Reg128Bits<114> {}
impl Reg128BitsDownCast<4> for Reg128Bits<114> {}
impl Reg128BitsConcat<4, 118> for Reg128Bits<114> {}
impl Reg128BitsDownCast<5> for Reg128Bits<114> {}
impl Reg128BitsConcat<5, 119> for Reg128Bits<114> {}
impl Reg128BitsDownCast<6> for Reg128Bits<114> {}
impl Reg128BitsConcat<6, 120> for Reg128Bits<114> {}
impl Reg128BitsDownCast<7> for Reg128Bits<114> {}
impl Reg128BitsConcat<7, 121> for Reg128Bits<114> {}
impl Reg128BitsDownCast<8> for Reg128Bits<114> {}
impl Reg128BitsConcat<8, 122> for Reg128Bits<114> {}
impl Reg128BitsDownCast<9> for Reg128Bits<114> {}
impl Reg128BitsConcat<9, 123> for Reg128Bits<114> {}
impl Reg128BitsDownCast<10> for Reg128Bits<114> {}
impl Reg128BitsConcat<10, 124> for Reg128Bits<114> {}
impl Reg128BitsDownCast<11> for Reg128Bits<114> {}
impl Reg128BitsConcat<11, 125> for Reg128Bits<114> {}
impl Reg128BitsDownCast<12> for Reg128Bits<114> {}
impl Reg128BitsConcat<12, 126> for Reg128Bits<114> {}
impl Reg128BitsDownCast<13> for Reg128Bits<114> {}
impl Reg128BitsConcat<13, 127> for Reg128Bits<114> {}
impl Reg128BitsDownCast<14> for Reg128Bits<114> {}
impl Reg128BitsConcat<14, 128> for Reg128Bits<114> {}
impl Reg128BitsDownCast<15> for Reg128Bits<114> {}
impl Reg128BitsDownCast<16> for Reg128Bits<114> {}
impl Reg128BitsDownCast<17> for Reg128Bits<114> {}
impl Reg128BitsDownCast<18> for Reg128Bits<114> {}
impl Reg128BitsDownCast<19> for Reg128Bits<114> {}
impl Reg128BitsDownCast<20> for Reg128Bits<114> {}
impl Reg128BitsDownCast<21> for Reg128Bits<114> {}
impl Reg128BitsDownCast<22> for Reg128Bits<114> {}
impl Reg128BitsDownCast<23> for Reg128Bits<114> {}
impl Reg128BitsDownCast<24> for Reg128Bits<114> {}
impl Reg128BitsDownCast<25> for Reg128Bits<114> {}
impl Reg128BitsDownCast<26> for Reg128Bits<114> {}
impl Reg128BitsDownCast<27> for Reg128Bits<114> {}
impl Reg128BitsDownCast<28> for Reg128Bits<114> {}
impl Reg128BitsDownCast<29> for Reg128Bits<114> {}
impl Reg128BitsDownCast<30> for Reg128Bits<114> {}
impl Reg128BitsDownCast<31> for Reg128Bits<114> {}
impl Reg128BitsDownCast<32> for Reg128Bits<114> {}
impl Reg128BitsDownCast<33> for Reg128Bits<114> {}
impl Reg128BitsDownCast<34> for Reg128Bits<114> {}
impl Reg128BitsDownCast<35> for Reg128Bits<114> {}
impl Reg128BitsDownCast<36> for Reg128Bits<114> {}
impl Reg128BitsDownCast<37> for Reg128Bits<114> {}
impl Reg128BitsDownCast<38> for Reg128Bits<114> {}
impl Reg128BitsDownCast<39> for Reg128Bits<114> {}
impl Reg128BitsDownCast<40> for Reg128Bits<114> {}
impl Reg128BitsDownCast<41> for Reg128Bits<114> {}
impl Reg128BitsDownCast<42> for Reg128Bits<114> {}
impl Reg128BitsDownCast<43> for Reg128Bits<114> {}
impl Reg128BitsDownCast<44> for Reg128Bits<114> {}
impl Reg128BitsDownCast<45> for Reg128Bits<114> {}
impl Reg128BitsDownCast<46> for Reg128Bits<114> {}
impl Reg128BitsDownCast<47> for Reg128Bits<114> {}
impl Reg128BitsDownCast<48> for Reg128Bits<114> {}
impl Reg128BitsDownCast<49> for Reg128Bits<114> {}
impl Reg128BitsDownCast<50> for Reg128Bits<114> {}
impl Reg128BitsDownCast<51> for Reg128Bits<114> {}
impl Reg128BitsDownCast<52> for Reg128Bits<114> {}
impl Reg128BitsDownCast<53> for Reg128Bits<114> {}
impl Reg128BitsDownCast<54> for Reg128Bits<114> {}
impl Reg128BitsDownCast<55> for Reg128Bits<114> {}
impl Reg128BitsDownCast<56> for Reg128Bits<114> {}
impl Reg128BitsDownCast<57> for Reg128Bits<114> {}
impl Reg128BitsDownCast<58> for Reg128Bits<114> {}
impl Reg128BitsDownCast<59> for Reg128Bits<114> {}
impl Reg128BitsDownCast<60> for Reg128Bits<114> {}
impl Reg128BitsDownCast<61> for Reg128Bits<114> {}
impl Reg128BitsDownCast<62> for Reg128Bits<114> {}
impl Reg128BitsDownCast<63> for Reg128Bits<114> {}
impl Reg128BitsDownCast<64> for Reg128Bits<114> {}
impl Reg128BitsDownCast<65> for Reg128Bits<114> {}
impl Reg128BitsDownCast<66> for Reg128Bits<114> {}
impl Reg128BitsDownCast<67> for Reg128Bits<114> {}
impl Reg128BitsDownCast<68> for Reg128Bits<114> {}
impl Reg128BitsDownCast<69> for Reg128Bits<114> {}
impl Reg128BitsDownCast<70> for Reg128Bits<114> {}
impl Reg128BitsDownCast<71> for Reg128Bits<114> {}
impl Reg128BitsDownCast<72> for Reg128Bits<114> {}
impl Reg128BitsDownCast<73> for Reg128Bits<114> {}
impl Reg128BitsDownCast<74> for Reg128Bits<114> {}
impl Reg128BitsDownCast<75> for Reg128Bits<114> {}
impl Reg128BitsDownCast<76> for Reg128Bits<114> {}
impl Reg128BitsDownCast<77> for Reg128Bits<114> {}
impl Reg128BitsDownCast<78> for Reg128Bits<114> {}
impl Reg128BitsDownCast<79> for Reg128Bits<114> {}
impl Reg128BitsDownCast<80> for Reg128Bits<114> {}
impl Reg128BitsDownCast<81> for Reg128Bits<114> {}
impl Reg128BitsDownCast<82> for Reg128Bits<114> {}
impl Reg128BitsDownCast<83> for Reg128Bits<114> {}
impl Reg128BitsDownCast<84> for Reg128Bits<114> {}
impl Reg128BitsDownCast<85> for Reg128Bits<114> {}
impl Reg128BitsDownCast<86> for Reg128Bits<114> {}
impl Reg128BitsDownCast<87> for Reg128Bits<114> {}
impl Reg128BitsDownCast<88> for Reg128Bits<114> {}
impl Reg128BitsDownCast<89> for Reg128Bits<114> {}
impl Reg128BitsDownCast<90> for Reg128Bits<114> {}
impl Reg128BitsDownCast<91> for Reg128Bits<114> {}
impl Reg128BitsDownCast<92> for Reg128Bits<114> {}
impl Reg128BitsDownCast<93> for Reg128Bits<114> {}
impl Reg128BitsDownCast<94> for Reg128Bits<114> {}
impl Reg128BitsDownCast<95> for Reg128Bits<114> {}
impl Reg128BitsDownCast<96> for Reg128Bits<114> {}
impl Reg128BitsDownCast<97> for Reg128Bits<114> {}
impl Reg128BitsDownCast<98> for Reg128Bits<114> {}
impl Reg128BitsDownCast<99> for Reg128Bits<114> {}
impl Reg128BitsDownCast<100> for Reg128Bits<114> {}
impl Reg128BitsDownCast<101> for Reg128Bits<114> {}
impl Reg128BitsDownCast<102> for Reg128Bits<114> {}
impl Reg128BitsDownCast<103> for Reg128Bits<114> {}
impl Reg128BitsDownCast<104> for Reg128Bits<114> {}
impl Reg128BitsDownCast<105> for Reg128Bits<114> {}
impl Reg128BitsDownCast<106> for Reg128Bits<114> {}
impl Reg128BitsDownCast<107> for Reg128Bits<114> {}
impl Reg128BitsDownCast<108> for Reg128Bits<114> {}
impl Reg128BitsDownCast<109> for Reg128Bits<114> {}
impl Reg128BitsDownCast<110> for Reg128Bits<114> {}
impl Reg128BitsDownCast<111> for Reg128Bits<114> {}
impl Reg128BitsDownCast<112> for Reg128Bits<114> {}
impl Reg128BitsDownCast<113> for Reg128Bits<114> {}
impl Reg128BitsDownCast<114> for Reg128Bits<114> {}
impl Reg128BitsDownCast<1> for Reg128Bits<115> {}
impl Reg128BitsConcat<1, 116> for Reg128Bits<115> {}
impl Reg128BitsDownCast<2> for Reg128Bits<115> {}
impl Reg128BitsConcat<2, 117> for Reg128Bits<115> {}
impl Reg128BitsDownCast<3> for Reg128Bits<115> {}
impl Reg128BitsConcat<3, 118> for Reg128Bits<115> {}
impl Reg128BitsDownCast<4> for Reg128Bits<115> {}
impl Reg128BitsConcat<4, 119> for Reg128Bits<115> {}
impl Reg128BitsDownCast<5> for Reg128Bits<115> {}
impl Reg128BitsConcat<5, 120> for Reg128Bits<115> {}
impl Reg128BitsDownCast<6> for Reg128Bits<115> {}
impl Reg128BitsConcat<6, 121> for Reg128Bits<115> {}
impl Reg128BitsDownCast<7> for Reg128Bits<115> {}
impl Reg128BitsConcat<7, 122> for Reg128Bits<115> {}
impl Reg128BitsDownCast<8> for Reg128Bits<115> {}
impl Reg128BitsConcat<8, 123> for Reg128Bits<115> {}
impl Reg128BitsDownCast<9> for Reg128Bits<115> {}
impl Reg128BitsConcat<9, 124> for Reg128Bits<115> {}
impl Reg128BitsDownCast<10> for Reg128Bits<115> {}
impl Reg128BitsConcat<10, 125> for Reg128Bits<115> {}
impl Reg128BitsDownCast<11> for Reg128Bits<115> {}
impl Reg128BitsConcat<11, 126> for Reg128Bits<115> {}
impl Reg128BitsDownCast<12> for Reg128Bits<115> {}
impl Reg128BitsConcat<12, 127> for Reg128Bits<115> {}
impl Reg128BitsDownCast<13> for Reg128Bits<115> {}
impl Reg128BitsConcat<13, 128> for Reg128Bits<115> {}
impl Reg128BitsDownCast<14> for Reg128Bits<115> {}
impl Reg128BitsDownCast<15> for Reg128Bits<115> {}
impl Reg128BitsDownCast<16> for Reg128Bits<115> {}
impl Reg128BitsDownCast<17> for Reg128Bits<115> {}
impl Reg128BitsDownCast<18> for Reg128Bits<115> {}
impl Reg128BitsDownCast<19> for Reg128Bits<115> {}
impl Reg128BitsDownCast<20> for Reg128Bits<115> {}
impl Reg128BitsDownCast<21> for Reg128Bits<115> {}
impl Reg128BitsDownCast<22> for Reg128Bits<115> {}
impl Reg128BitsDownCast<23> for Reg128Bits<115> {}
impl Reg128BitsDownCast<24> for Reg128Bits<115> {}
impl Reg128BitsDownCast<25> for Reg128Bits<115> {}
impl Reg128BitsDownCast<26> for Reg128Bits<115> {}
impl Reg128BitsDownCast<27> for Reg128Bits<115> {}
impl Reg128BitsDownCast<28> for Reg128Bits<115> {}
impl Reg128BitsDownCast<29> for Reg128Bits<115> {}
impl Reg128BitsDownCast<30> for Reg128Bits<115> {}
impl Reg128BitsDownCast<31> for Reg128Bits<115> {}
impl Reg128BitsDownCast<32> for Reg128Bits<115> {}
impl Reg128BitsDownCast<33> for Reg128Bits<115> {}
impl Reg128BitsDownCast<34> for Reg128Bits<115> {}
impl Reg128BitsDownCast<35> for Reg128Bits<115> {}
impl Reg128BitsDownCast<36> for Reg128Bits<115> {}
impl Reg128BitsDownCast<37> for Reg128Bits<115> {}
impl Reg128BitsDownCast<38> for Reg128Bits<115> {}
impl Reg128BitsDownCast<39> for Reg128Bits<115> {}
impl Reg128BitsDownCast<40> for Reg128Bits<115> {}
impl Reg128BitsDownCast<41> for Reg128Bits<115> {}
impl Reg128BitsDownCast<42> for Reg128Bits<115> {}
impl Reg128BitsDownCast<43> for Reg128Bits<115> {}
impl Reg128BitsDownCast<44> for Reg128Bits<115> {}
impl Reg128BitsDownCast<45> for Reg128Bits<115> {}
impl Reg128BitsDownCast<46> for Reg128Bits<115> {}
impl Reg128BitsDownCast<47> for Reg128Bits<115> {}
impl Reg128BitsDownCast<48> for Reg128Bits<115> {}
impl Reg128BitsDownCast<49> for Reg128Bits<115> {}
impl Reg128BitsDownCast<50> for Reg128Bits<115> {}
impl Reg128BitsDownCast<51> for Reg128Bits<115> {}
impl Reg128BitsDownCast<52> for Reg128Bits<115> {}
impl Reg128BitsDownCast<53> for Reg128Bits<115> {}
impl Reg128BitsDownCast<54> for Reg128Bits<115> {}
impl Reg128BitsDownCast<55> for Reg128Bits<115> {}
impl Reg128BitsDownCast<56> for Reg128Bits<115> {}
impl Reg128BitsDownCast<57> for Reg128Bits<115> {}
impl Reg128BitsDownCast<58> for Reg128Bits<115> {}
impl Reg128BitsDownCast<59> for Reg128Bits<115> {}
impl Reg128BitsDownCast<60> for Reg128Bits<115> {}
impl Reg128BitsDownCast<61> for Reg128Bits<115> {}
impl Reg128BitsDownCast<62> for Reg128Bits<115> {}
impl Reg128BitsDownCast<63> for Reg128Bits<115> {}
impl Reg128BitsDownCast<64> for Reg128Bits<115> {}
impl Reg128BitsDownCast<65> for Reg128Bits<115> {}
impl Reg128BitsDownCast<66> for Reg128Bits<115> {}
impl Reg128BitsDownCast<67> for Reg128Bits<115> {}
impl Reg128BitsDownCast<68> for Reg128Bits<115> {}
impl Reg128BitsDownCast<69> for Reg128Bits<115> {}
impl Reg128BitsDownCast<70> for Reg128Bits<115> {}
impl Reg128BitsDownCast<71> for Reg128Bits<115> {}
impl Reg128BitsDownCast<72> for Reg128Bits<115> {}
impl Reg128BitsDownCast<73> for Reg128Bits<115> {}
impl Reg128BitsDownCast<74> for Reg128Bits<115> {}
impl Reg128BitsDownCast<75> for Reg128Bits<115> {}
impl Reg128BitsDownCast<76> for Reg128Bits<115> {}
impl Reg128BitsDownCast<77> for Reg128Bits<115> {}
impl Reg128BitsDownCast<78> for Reg128Bits<115> {}
impl Reg128BitsDownCast<79> for Reg128Bits<115> {}
impl Reg128BitsDownCast<80> for Reg128Bits<115> {}
impl Reg128BitsDownCast<81> for Reg128Bits<115> {}
impl Reg128BitsDownCast<82> for Reg128Bits<115> {}
impl Reg128BitsDownCast<83> for Reg128Bits<115> {}
impl Reg128BitsDownCast<84> for Reg128Bits<115> {}
impl Reg128BitsDownCast<85> for Reg128Bits<115> {}
impl Reg128BitsDownCast<86> for Reg128Bits<115> {}
impl Reg128BitsDownCast<87> for Reg128Bits<115> {}
impl Reg128BitsDownCast<88> for Reg128Bits<115> {}
impl Reg128BitsDownCast<89> for Reg128Bits<115> {}
impl Reg128BitsDownCast<90> for Reg128Bits<115> {}
impl Reg128BitsDownCast<91> for Reg128Bits<115> {}
impl Reg128BitsDownCast<92> for Reg128Bits<115> {}
impl Reg128BitsDownCast<93> for Reg128Bits<115> {}
impl Reg128BitsDownCast<94> for Reg128Bits<115> {}
impl Reg128BitsDownCast<95> for Reg128Bits<115> {}
impl Reg128BitsDownCast<96> for Reg128Bits<115> {}
impl Reg128BitsDownCast<97> for Reg128Bits<115> {}
impl Reg128BitsDownCast<98> for Reg128Bits<115> {}
impl Reg128BitsDownCast<99> for Reg128Bits<115> {}
impl Reg128BitsDownCast<100> for Reg128Bits<115> {}
impl Reg128BitsDownCast<101> for Reg128Bits<115> {}
impl Reg128BitsDownCast<102> for Reg128Bits<115> {}
impl Reg128BitsDownCast<103> for Reg128Bits<115> {}
impl Reg128BitsDownCast<104> for Reg128Bits<115> {}
impl Reg128BitsDownCast<105> for Reg128Bits<115> {}
impl Reg128BitsDownCast<106> for Reg128Bits<115> {}
impl Reg128BitsDownCast<107> for Reg128Bits<115> {}
impl Reg128BitsDownCast<108> for Reg128Bits<115> {}
impl Reg128BitsDownCast<109> for Reg128Bits<115> {}
impl Reg128BitsDownCast<110> for Reg128Bits<115> {}
impl Reg128BitsDownCast<111> for Reg128Bits<115> {}
impl Reg128BitsDownCast<112> for Reg128Bits<115> {}
impl Reg128BitsDownCast<113> for Reg128Bits<115> {}
impl Reg128BitsDownCast<114> for Reg128Bits<115> {}
impl Reg128BitsDownCast<115> for Reg128Bits<115> {}
impl Reg128BitsDownCast<1> for Reg128Bits<116> {}
impl Reg128BitsConcat<1, 117> for Reg128Bits<116> {}
impl Reg128BitsDownCast<2> for Reg128Bits<116> {}
impl Reg128BitsConcat<2, 118> for Reg128Bits<116> {}
impl Reg128BitsDownCast<3> for Reg128Bits<116> {}
impl Reg128BitsConcat<3, 119> for Reg128Bits<116> {}
impl Reg128BitsDownCast<4> for Reg128Bits<116> {}
impl Reg128BitsConcat<4, 120> for Reg128Bits<116> {}
impl Reg128BitsDownCast<5> for Reg128Bits<116> {}
impl Reg128BitsConcat<5, 121> for Reg128Bits<116> {}
impl Reg128BitsDownCast<6> for Reg128Bits<116> {}
impl Reg128BitsConcat<6, 122> for Reg128Bits<116> {}
impl Reg128BitsDownCast<7> for Reg128Bits<116> {}
impl Reg128BitsConcat<7, 123> for Reg128Bits<116> {}
impl Reg128BitsDownCast<8> for Reg128Bits<116> {}
impl Reg128BitsConcat<8, 124> for Reg128Bits<116> {}
impl Reg128BitsDownCast<9> for Reg128Bits<116> {}
impl Reg128BitsConcat<9, 125> for Reg128Bits<116> {}
impl Reg128BitsDownCast<10> for Reg128Bits<116> {}
impl Reg128BitsConcat<10, 126> for Reg128Bits<116> {}
impl Reg128BitsDownCast<11> for Reg128Bits<116> {}
impl Reg128BitsConcat<11, 127> for Reg128Bits<116> {}
impl Reg128BitsDownCast<12> for Reg128Bits<116> {}
impl Reg128BitsConcat<12, 128> for Reg128Bits<116> {}
impl Reg128BitsDownCast<13> for Reg128Bits<116> {}
impl Reg128BitsDownCast<14> for Reg128Bits<116> {}
impl Reg128BitsDownCast<15> for Reg128Bits<116> {}
impl Reg128BitsDownCast<16> for Reg128Bits<116> {}
impl Reg128BitsDownCast<17> for Reg128Bits<116> {}
impl Reg128BitsDownCast<18> for Reg128Bits<116> {}
impl Reg128BitsDownCast<19> for Reg128Bits<116> {}
impl Reg128BitsDownCast<20> for Reg128Bits<116> {}
impl Reg128BitsDownCast<21> for Reg128Bits<116> {}
impl Reg128BitsDownCast<22> for Reg128Bits<116> {}
impl Reg128BitsDownCast<23> for Reg128Bits<116> {}
impl Reg128BitsDownCast<24> for Reg128Bits<116> {}
impl Reg128BitsDownCast<25> for Reg128Bits<116> {}
impl Reg128BitsDownCast<26> for Reg128Bits<116> {}
impl Reg128BitsDownCast<27> for Reg128Bits<116> {}
impl Reg128BitsDownCast<28> for Reg128Bits<116> {}
impl Reg128BitsDownCast<29> for Reg128Bits<116> {}
impl Reg128BitsDownCast<30> for Reg128Bits<116> {}
impl Reg128BitsDownCast<31> for Reg128Bits<116> {}
impl Reg128BitsDownCast<32> for Reg128Bits<116> {}
impl Reg128BitsDownCast<33> for Reg128Bits<116> {}
impl Reg128BitsDownCast<34> for Reg128Bits<116> {}
impl Reg128BitsDownCast<35> for Reg128Bits<116> {}
impl Reg128BitsDownCast<36> for Reg128Bits<116> {}
impl Reg128BitsDownCast<37> for Reg128Bits<116> {}
impl Reg128BitsDownCast<38> for Reg128Bits<116> {}
impl Reg128BitsDownCast<39> for Reg128Bits<116> {}
impl Reg128BitsDownCast<40> for Reg128Bits<116> {}
impl Reg128BitsDownCast<41> for Reg128Bits<116> {}
impl Reg128BitsDownCast<42> for Reg128Bits<116> {}
impl Reg128BitsDownCast<43> for Reg128Bits<116> {}
impl Reg128BitsDownCast<44> for Reg128Bits<116> {}
impl Reg128BitsDownCast<45> for Reg128Bits<116> {}
impl Reg128BitsDownCast<46> for Reg128Bits<116> {}
impl Reg128BitsDownCast<47> for Reg128Bits<116> {}
impl Reg128BitsDownCast<48> for Reg128Bits<116> {}
impl Reg128BitsDownCast<49> for Reg128Bits<116> {}
impl Reg128BitsDownCast<50> for Reg128Bits<116> {}
impl Reg128BitsDownCast<51> for Reg128Bits<116> {}
impl Reg128BitsDownCast<52> for Reg128Bits<116> {}
impl Reg128BitsDownCast<53> for Reg128Bits<116> {}
impl Reg128BitsDownCast<54> for Reg128Bits<116> {}
impl Reg128BitsDownCast<55> for Reg128Bits<116> {}
impl Reg128BitsDownCast<56> for Reg128Bits<116> {}
impl Reg128BitsDownCast<57> for Reg128Bits<116> {}
impl Reg128BitsDownCast<58> for Reg128Bits<116> {}
impl Reg128BitsDownCast<59> for Reg128Bits<116> {}
impl Reg128BitsDownCast<60> for Reg128Bits<116> {}
impl Reg128BitsDownCast<61> for Reg128Bits<116> {}
impl Reg128BitsDownCast<62> for Reg128Bits<116> {}
impl Reg128BitsDownCast<63> for Reg128Bits<116> {}
impl Reg128BitsDownCast<64> for Reg128Bits<116> {}
impl Reg128BitsDownCast<65> for Reg128Bits<116> {}
impl Reg128BitsDownCast<66> for Reg128Bits<116> {}
impl Reg128BitsDownCast<67> for Reg128Bits<116> {}
impl Reg128BitsDownCast<68> for Reg128Bits<116> {}
impl Reg128BitsDownCast<69> for Reg128Bits<116> {}
impl Reg128BitsDownCast<70> for Reg128Bits<116> {}
impl Reg128BitsDownCast<71> for Reg128Bits<116> {}
impl Reg128BitsDownCast<72> for Reg128Bits<116> {}
impl Reg128BitsDownCast<73> for Reg128Bits<116> {}
impl Reg128BitsDownCast<74> for Reg128Bits<116> {}
impl Reg128BitsDownCast<75> for Reg128Bits<116> {}
impl Reg128BitsDownCast<76> for Reg128Bits<116> {}
impl Reg128BitsDownCast<77> for Reg128Bits<116> {}
impl Reg128BitsDownCast<78> for Reg128Bits<116> {}
impl Reg128BitsDownCast<79> for Reg128Bits<116> {}
impl Reg128BitsDownCast<80> for Reg128Bits<116> {}
impl Reg128BitsDownCast<81> for Reg128Bits<116> {}
impl Reg128BitsDownCast<82> for Reg128Bits<116> {}
impl Reg128BitsDownCast<83> for Reg128Bits<116> {}
impl Reg128BitsDownCast<84> for Reg128Bits<116> {}
impl Reg128BitsDownCast<85> for Reg128Bits<116> {}
impl Reg128BitsDownCast<86> for Reg128Bits<116> {}
impl Reg128BitsDownCast<87> for Reg128Bits<116> {}
impl Reg128BitsDownCast<88> for Reg128Bits<116> {}
impl Reg128BitsDownCast<89> for Reg128Bits<116> {}
impl Reg128BitsDownCast<90> for Reg128Bits<116> {}
impl Reg128BitsDownCast<91> for Reg128Bits<116> {}
impl Reg128BitsDownCast<92> for Reg128Bits<116> {}
impl Reg128BitsDownCast<93> for Reg128Bits<116> {}
impl Reg128BitsDownCast<94> for Reg128Bits<116> {}
impl Reg128BitsDownCast<95> for Reg128Bits<116> {}
impl Reg128BitsDownCast<96> for Reg128Bits<116> {}
impl Reg128BitsDownCast<97> for Reg128Bits<116> {}
impl Reg128BitsDownCast<98> for Reg128Bits<116> {}
impl Reg128BitsDownCast<99> for Reg128Bits<116> {}
impl Reg128BitsDownCast<100> for Reg128Bits<116> {}
impl Reg128BitsDownCast<101> for Reg128Bits<116> {}
impl Reg128BitsDownCast<102> for Reg128Bits<116> {}
impl Reg128BitsDownCast<103> for Reg128Bits<116> {}
impl Reg128BitsDownCast<104> for Reg128Bits<116> {}
impl Reg128BitsDownCast<105> for Reg128Bits<116> {}
impl Reg128BitsDownCast<106> for Reg128Bits<116> {}
impl Reg128BitsDownCast<107> for Reg128Bits<116> {}
impl Reg128BitsDownCast<108> for Reg128Bits<116> {}
impl Reg128BitsDownCast<109> for Reg128Bits<116> {}
impl Reg128BitsDownCast<110> for Reg128Bits<116> {}
impl Reg128BitsDownCast<111> for Reg128Bits<116> {}
impl Reg128BitsDownCast<112> for Reg128Bits<116> {}
impl Reg128BitsDownCast<113> for Reg128Bits<116> {}
impl Reg128BitsDownCast<114> for Reg128Bits<116> {}
impl Reg128BitsDownCast<115> for Reg128Bits<116> {}
impl Reg128BitsDownCast<116> for Reg128Bits<116> {}
impl Reg128BitsDownCast<1> for Reg128Bits<117> {}
impl Reg128BitsConcat<1, 118> for Reg128Bits<117> {}
impl Reg128BitsDownCast<2> for Reg128Bits<117> {}
impl Reg128BitsConcat<2, 119> for Reg128Bits<117> {}
impl Reg128BitsDownCast<3> for Reg128Bits<117> {}
impl Reg128BitsConcat<3, 120> for Reg128Bits<117> {}
impl Reg128BitsDownCast<4> for Reg128Bits<117> {}
impl Reg128BitsConcat<4, 121> for Reg128Bits<117> {}
impl Reg128BitsDownCast<5> for Reg128Bits<117> {}
impl Reg128BitsConcat<5, 122> for Reg128Bits<117> {}
impl Reg128BitsDownCast<6> for Reg128Bits<117> {}
impl Reg128BitsConcat<6, 123> for Reg128Bits<117> {}
impl Reg128BitsDownCast<7> for Reg128Bits<117> {}
impl Reg128BitsConcat<7, 124> for Reg128Bits<117> {}
impl Reg128BitsDownCast<8> for Reg128Bits<117> {}
impl Reg128BitsConcat<8, 125> for Reg128Bits<117> {}
impl Reg128BitsDownCast<9> for Reg128Bits<117> {}
impl Reg128BitsConcat<9, 126> for Reg128Bits<117> {}
impl Reg128BitsDownCast<10> for Reg128Bits<117> {}
impl Reg128BitsConcat<10, 127> for Reg128Bits<117> {}
impl Reg128BitsDownCast<11> for Reg128Bits<117> {}
impl Reg128BitsConcat<11, 128> for Reg128Bits<117> {}
impl Reg128BitsDownCast<12> for Reg128Bits<117> {}
impl Reg128BitsDownCast<13> for Reg128Bits<117> {}
impl Reg128BitsDownCast<14> for Reg128Bits<117> {}
impl Reg128BitsDownCast<15> for Reg128Bits<117> {}
impl Reg128BitsDownCast<16> for Reg128Bits<117> {}
impl Reg128BitsDownCast<17> for Reg128Bits<117> {}
impl Reg128BitsDownCast<18> for Reg128Bits<117> {}
impl Reg128BitsDownCast<19> for Reg128Bits<117> {}
impl Reg128BitsDownCast<20> for Reg128Bits<117> {}
impl Reg128BitsDownCast<21> for Reg128Bits<117> {}
impl Reg128BitsDownCast<22> for Reg128Bits<117> {}
impl Reg128BitsDownCast<23> for Reg128Bits<117> {}
impl Reg128BitsDownCast<24> for Reg128Bits<117> {}
impl Reg128BitsDownCast<25> for Reg128Bits<117> {}
impl Reg128BitsDownCast<26> for Reg128Bits<117> {}
impl Reg128BitsDownCast<27> for Reg128Bits<117> {}
impl Reg128BitsDownCast<28> for Reg128Bits<117> {}
impl Reg128BitsDownCast<29> for Reg128Bits<117> {}
impl Reg128BitsDownCast<30> for Reg128Bits<117> {}
impl Reg128BitsDownCast<31> for Reg128Bits<117> {}
impl Reg128BitsDownCast<32> for Reg128Bits<117> {}
impl Reg128BitsDownCast<33> for Reg128Bits<117> {}
impl Reg128BitsDownCast<34> for Reg128Bits<117> {}
impl Reg128BitsDownCast<35> for Reg128Bits<117> {}
impl Reg128BitsDownCast<36> for Reg128Bits<117> {}
impl Reg128BitsDownCast<37> for Reg128Bits<117> {}
impl Reg128BitsDownCast<38> for Reg128Bits<117> {}
impl Reg128BitsDownCast<39> for Reg128Bits<117> {}
impl Reg128BitsDownCast<40> for Reg128Bits<117> {}
impl Reg128BitsDownCast<41> for Reg128Bits<117> {}
impl Reg128BitsDownCast<42> for Reg128Bits<117> {}
impl Reg128BitsDownCast<43> for Reg128Bits<117> {}
impl Reg128BitsDownCast<44> for Reg128Bits<117> {}
impl Reg128BitsDownCast<45> for Reg128Bits<117> {}
impl Reg128BitsDownCast<46> for Reg128Bits<117> {}
impl Reg128BitsDownCast<47> for Reg128Bits<117> {}
impl Reg128BitsDownCast<48> for Reg128Bits<117> {}
impl Reg128BitsDownCast<49> for Reg128Bits<117> {}
impl Reg128BitsDownCast<50> for Reg128Bits<117> {}
impl Reg128BitsDownCast<51> for Reg128Bits<117> {}
impl Reg128BitsDownCast<52> for Reg128Bits<117> {}
impl Reg128BitsDownCast<53> for Reg128Bits<117> {}
impl Reg128BitsDownCast<54> for Reg128Bits<117> {}
impl Reg128BitsDownCast<55> for Reg128Bits<117> {}
impl Reg128BitsDownCast<56> for Reg128Bits<117> {}
impl Reg128BitsDownCast<57> for Reg128Bits<117> {}
impl Reg128BitsDownCast<58> for Reg128Bits<117> {}
impl Reg128BitsDownCast<59> for Reg128Bits<117> {}
impl Reg128BitsDownCast<60> for Reg128Bits<117> {}
impl Reg128BitsDownCast<61> for Reg128Bits<117> {}
impl Reg128BitsDownCast<62> for Reg128Bits<117> {}
impl Reg128BitsDownCast<63> for Reg128Bits<117> {}
impl Reg128BitsDownCast<64> for Reg128Bits<117> {}
impl Reg128BitsDownCast<65> for Reg128Bits<117> {}
impl Reg128BitsDownCast<66> for Reg128Bits<117> {}
impl Reg128BitsDownCast<67> for Reg128Bits<117> {}
impl Reg128BitsDownCast<68> for Reg128Bits<117> {}
impl Reg128BitsDownCast<69> for Reg128Bits<117> {}
impl Reg128BitsDownCast<70> for Reg128Bits<117> {}
impl Reg128BitsDownCast<71> for Reg128Bits<117> {}
impl Reg128BitsDownCast<72> for Reg128Bits<117> {}
impl Reg128BitsDownCast<73> for Reg128Bits<117> {}
impl Reg128BitsDownCast<74> for Reg128Bits<117> {}
impl Reg128BitsDownCast<75> for Reg128Bits<117> {}
impl Reg128BitsDownCast<76> for Reg128Bits<117> {}
impl Reg128BitsDownCast<77> for Reg128Bits<117> {}
impl Reg128BitsDownCast<78> for Reg128Bits<117> {}
impl Reg128BitsDownCast<79> for Reg128Bits<117> {}
impl Reg128BitsDownCast<80> for Reg128Bits<117> {}
impl Reg128BitsDownCast<81> for Reg128Bits<117> {}
impl Reg128BitsDownCast<82> for Reg128Bits<117> {}
impl Reg128BitsDownCast<83> for Reg128Bits<117> {}
impl Reg128BitsDownCast<84> for Reg128Bits<117> {}
impl Reg128BitsDownCast<85> for Reg128Bits<117> {}
impl Reg128BitsDownCast<86> for Reg128Bits<117> {}
impl Reg128BitsDownCast<87> for Reg128Bits<117> {}
impl Reg128BitsDownCast<88> for Reg128Bits<117> {}
impl Reg128BitsDownCast<89> for Reg128Bits<117> {}
impl Reg128BitsDownCast<90> for Reg128Bits<117> {}
impl Reg128BitsDownCast<91> for Reg128Bits<117> {}
impl Reg128BitsDownCast<92> for Reg128Bits<117> {}
impl Reg128BitsDownCast<93> for Reg128Bits<117> {}
impl Reg128BitsDownCast<94> for Reg128Bits<117> {}
impl Reg128BitsDownCast<95> for Reg128Bits<117> {}
impl Reg128BitsDownCast<96> for Reg128Bits<117> {}
impl Reg128BitsDownCast<97> for Reg128Bits<117> {}
impl Reg128BitsDownCast<98> for Reg128Bits<117> {}
impl Reg128BitsDownCast<99> for Reg128Bits<117> {}
impl Reg128BitsDownCast<100> for Reg128Bits<117> {}
impl Reg128BitsDownCast<101> for Reg128Bits<117> {}
impl Reg128BitsDownCast<102> for Reg128Bits<117> {}
impl Reg128BitsDownCast<103> for Reg128Bits<117> {}
impl Reg128BitsDownCast<104> for Reg128Bits<117> {}
impl Reg128BitsDownCast<105> for Reg128Bits<117> {}
impl Reg128BitsDownCast<106> for Reg128Bits<117> {}
impl Reg128BitsDownCast<107> for Reg128Bits<117> {}
impl Reg128BitsDownCast<108> for Reg128Bits<117> {}
impl Reg128BitsDownCast<109> for Reg128Bits<117> {}
impl Reg128BitsDownCast<110> for Reg128Bits<117> {}
impl Reg128BitsDownCast<111> for Reg128Bits<117> {}
impl Reg128BitsDownCast<112> for Reg128Bits<117> {}
impl Reg128BitsDownCast<113> for Reg128Bits<117> {}
impl Reg128BitsDownCast<114> for Reg128Bits<117> {}
impl Reg128BitsDownCast<115> for Reg128Bits<117> {}
impl Reg128BitsDownCast<116> for Reg128Bits<117> {}
impl Reg128BitsDownCast<117> for Reg128Bits<117> {}
impl Reg128BitsDownCast<1> for Reg128Bits<118> {}
impl Reg128BitsConcat<1, 119> for Reg128Bits<118> {}
impl Reg128BitsDownCast<2> for Reg128Bits<118> {}
impl Reg128BitsConcat<2, 120> for Reg128Bits<118> {}
impl Reg128BitsDownCast<3> for Reg128Bits<118> {}
impl Reg128BitsConcat<3, 121> for Reg128Bits<118> {}
impl Reg128BitsDownCast<4> for Reg128Bits<118> {}
impl Reg128BitsConcat<4, 122> for Reg128Bits<118> {}
impl Reg128BitsDownCast<5> for Reg128Bits<118> {}
impl Reg128BitsConcat<5, 123> for Reg128Bits<118> {}
impl Reg128BitsDownCast<6> for Reg128Bits<118> {}
impl Reg128BitsConcat<6, 124> for Reg128Bits<118> {}
impl Reg128BitsDownCast<7> for Reg128Bits<118> {}
impl Reg128BitsConcat<7, 125> for Reg128Bits<118> {}
impl Reg128BitsDownCast<8> for Reg128Bits<118> {}
impl Reg128BitsConcat<8, 126> for Reg128Bits<118> {}
impl Reg128BitsDownCast<9> for Reg128Bits<118> {}
impl Reg128BitsConcat<9, 127> for Reg128Bits<118> {}
impl Reg128BitsDownCast<10> for Reg128Bits<118> {}
impl Reg128BitsConcat<10, 128> for Reg128Bits<118> {}
impl Reg128BitsDownCast<11> for Reg128Bits<118> {}
impl Reg128BitsDownCast<12> for Reg128Bits<118> {}
impl Reg128BitsDownCast<13> for Reg128Bits<118> {}
impl Reg128BitsDownCast<14> for Reg128Bits<118> {}
impl Reg128BitsDownCast<15> for Reg128Bits<118> {}
impl Reg128BitsDownCast<16> for Reg128Bits<118> {}
impl Reg128BitsDownCast<17> for Reg128Bits<118> {}
impl Reg128BitsDownCast<18> for Reg128Bits<118> {}
impl Reg128BitsDownCast<19> for Reg128Bits<118> {}
impl Reg128BitsDownCast<20> for Reg128Bits<118> {}
impl Reg128BitsDownCast<21> for Reg128Bits<118> {}
impl Reg128BitsDownCast<22> for Reg128Bits<118> {}
impl Reg128BitsDownCast<23> for Reg128Bits<118> {}
impl Reg128BitsDownCast<24> for Reg128Bits<118> {}
impl Reg128BitsDownCast<25> for Reg128Bits<118> {}
impl Reg128BitsDownCast<26> for Reg128Bits<118> {}
impl Reg128BitsDownCast<27> for Reg128Bits<118> {}
impl Reg128BitsDownCast<28> for Reg128Bits<118> {}
impl Reg128BitsDownCast<29> for Reg128Bits<118> {}
impl Reg128BitsDownCast<30> for Reg128Bits<118> {}
impl Reg128BitsDownCast<31> for Reg128Bits<118> {}
impl Reg128BitsDownCast<32> for Reg128Bits<118> {}
impl Reg128BitsDownCast<33> for Reg128Bits<118> {}
impl Reg128BitsDownCast<34> for Reg128Bits<118> {}
impl Reg128BitsDownCast<35> for Reg128Bits<118> {}
impl Reg128BitsDownCast<36> for Reg128Bits<118> {}
impl Reg128BitsDownCast<37> for Reg128Bits<118> {}
impl Reg128BitsDownCast<38> for Reg128Bits<118> {}
impl Reg128BitsDownCast<39> for Reg128Bits<118> {}
impl Reg128BitsDownCast<40> for Reg128Bits<118> {}
impl Reg128BitsDownCast<41> for Reg128Bits<118> {}
impl Reg128BitsDownCast<42> for Reg128Bits<118> {}
impl Reg128BitsDownCast<43> for Reg128Bits<118> {}
impl Reg128BitsDownCast<44> for Reg128Bits<118> {}
impl Reg128BitsDownCast<45> for Reg128Bits<118> {}
impl Reg128BitsDownCast<46> for Reg128Bits<118> {}
impl Reg128BitsDownCast<47> for Reg128Bits<118> {}
impl Reg128BitsDownCast<48> for Reg128Bits<118> {}
impl Reg128BitsDownCast<49> for Reg128Bits<118> {}
impl Reg128BitsDownCast<50> for Reg128Bits<118> {}
impl Reg128BitsDownCast<51> for Reg128Bits<118> {}
impl Reg128BitsDownCast<52> for Reg128Bits<118> {}
impl Reg128BitsDownCast<53> for Reg128Bits<118> {}
impl Reg128BitsDownCast<54> for Reg128Bits<118> {}
impl Reg128BitsDownCast<55> for Reg128Bits<118> {}
impl Reg128BitsDownCast<56> for Reg128Bits<118> {}
impl Reg128BitsDownCast<57> for Reg128Bits<118> {}
impl Reg128BitsDownCast<58> for Reg128Bits<118> {}
impl Reg128BitsDownCast<59> for Reg128Bits<118> {}
impl Reg128BitsDownCast<60> for Reg128Bits<118> {}
impl Reg128BitsDownCast<61> for Reg128Bits<118> {}
impl Reg128BitsDownCast<62> for Reg128Bits<118> {}
impl Reg128BitsDownCast<63> for Reg128Bits<118> {}
impl Reg128BitsDownCast<64> for Reg128Bits<118> {}
impl Reg128BitsDownCast<65> for Reg128Bits<118> {}
impl Reg128BitsDownCast<66> for Reg128Bits<118> {}
impl Reg128BitsDownCast<67> for Reg128Bits<118> {}
impl Reg128BitsDownCast<68> for Reg128Bits<118> {}
impl Reg128BitsDownCast<69> for Reg128Bits<118> {}
impl Reg128BitsDownCast<70> for Reg128Bits<118> {}
impl Reg128BitsDownCast<71> for Reg128Bits<118> {}
impl Reg128BitsDownCast<72> for Reg128Bits<118> {}
impl Reg128BitsDownCast<73> for Reg128Bits<118> {}
impl Reg128BitsDownCast<74> for Reg128Bits<118> {}
impl Reg128BitsDownCast<75> for Reg128Bits<118> {}
impl Reg128BitsDownCast<76> for Reg128Bits<118> {}
impl Reg128BitsDownCast<77> for Reg128Bits<118> {}
impl Reg128BitsDownCast<78> for Reg128Bits<118> {}
impl Reg128BitsDownCast<79> for Reg128Bits<118> {}
impl Reg128BitsDownCast<80> for Reg128Bits<118> {}
impl Reg128BitsDownCast<81> for Reg128Bits<118> {}
impl Reg128BitsDownCast<82> for Reg128Bits<118> {}
impl Reg128BitsDownCast<83> for Reg128Bits<118> {}
impl Reg128BitsDownCast<84> for Reg128Bits<118> {}
impl Reg128BitsDownCast<85> for Reg128Bits<118> {}
impl Reg128BitsDownCast<86> for Reg128Bits<118> {}
impl Reg128BitsDownCast<87> for Reg128Bits<118> {}
impl Reg128BitsDownCast<88> for Reg128Bits<118> {}
impl Reg128BitsDownCast<89> for Reg128Bits<118> {}
impl Reg128BitsDownCast<90> for Reg128Bits<118> {}
impl Reg128BitsDownCast<91> for Reg128Bits<118> {}
impl Reg128BitsDownCast<92> for Reg128Bits<118> {}
impl Reg128BitsDownCast<93> for Reg128Bits<118> {}
impl Reg128BitsDownCast<94> for Reg128Bits<118> {}
impl Reg128BitsDownCast<95> for Reg128Bits<118> {}
impl Reg128BitsDownCast<96> for Reg128Bits<118> {}
impl Reg128BitsDownCast<97> for Reg128Bits<118> {}
impl Reg128BitsDownCast<98> for Reg128Bits<118> {}
impl Reg128BitsDownCast<99> for Reg128Bits<118> {}
impl Reg128BitsDownCast<100> for Reg128Bits<118> {}
impl Reg128BitsDownCast<101> for Reg128Bits<118> {}
impl Reg128BitsDownCast<102> for Reg128Bits<118> {}
impl Reg128BitsDownCast<103> for Reg128Bits<118> {}
impl Reg128BitsDownCast<104> for Reg128Bits<118> {}
impl Reg128BitsDownCast<105> for Reg128Bits<118> {}
impl Reg128BitsDownCast<106> for Reg128Bits<118> {}
impl Reg128BitsDownCast<107> for Reg128Bits<118> {}
impl Reg128BitsDownCast<108> for Reg128Bits<118> {}
impl Reg128BitsDownCast<109> for Reg128Bits<118> {}
impl Reg128BitsDownCast<110> for Reg128Bits<118> {}
impl Reg128BitsDownCast<111> for Reg128Bits<118> {}
impl Reg128BitsDownCast<112> for Reg128Bits<118> {}
impl Reg128BitsDownCast<113> for Reg128Bits<118> {}
impl Reg128BitsDownCast<114> for Reg128Bits<118> {}
impl Reg128BitsDownCast<115> for Reg128Bits<118> {}
impl Reg128BitsDownCast<116> for Reg128Bits<118> {}
impl Reg128BitsDownCast<117> for Reg128Bits<118> {}
impl Reg128BitsDownCast<118> for Reg128Bits<118> {}
impl Reg128BitsDownCast<1> for Reg128Bits<119> {}
impl Reg128BitsConcat<1, 120> for Reg128Bits<119> {}
impl Reg128BitsDownCast<2> for Reg128Bits<119> {}
impl Reg128BitsConcat<2, 121> for Reg128Bits<119> {}
impl Reg128BitsDownCast<3> for Reg128Bits<119> {}
impl Reg128BitsConcat<3, 122> for Reg128Bits<119> {}
impl Reg128BitsDownCast<4> for Reg128Bits<119> {}
impl Reg128BitsConcat<4, 123> for Reg128Bits<119> {}
impl Reg128BitsDownCast<5> for Reg128Bits<119> {}
impl Reg128BitsConcat<5, 124> for Reg128Bits<119> {}
impl Reg128BitsDownCast<6> for Reg128Bits<119> {}
impl Reg128BitsConcat<6, 125> for Reg128Bits<119> {}
impl Reg128BitsDownCast<7> for Reg128Bits<119> {}
impl Reg128BitsConcat<7, 126> for Reg128Bits<119> {}
impl Reg128BitsDownCast<8> for Reg128Bits<119> {}
impl Reg128BitsConcat<8, 127> for Reg128Bits<119> {}
impl Reg128BitsDownCast<9> for Reg128Bits<119> {}
impl Reg128BitsConcat<9, 128> for Reg128Bits<119> {}
impl Reg128BitsDownCast<10> for Reg128Bits<119> {}
impl Reg128BitsDownCast<11> for Reg128Bits<119> {}
impl Reg128BitsDownCast<12> for Reg128Bits<119> {}
impl Reg128BitsDownCast<13> for Reg128Bits<119> {}
impl Reg128BitsDownCast<14> for Reg128Bits<119> {}
impl Reg128BitsDownCast<15> for Reg128Bits<119> {}
impl Reg128BitsDownCast<16> for Reg128Bits<119> {}
impl Reg128BitsDownCast<17> for Reg128Bits<119> {}
impl Reg128BitsDownCast<18> for Reg128Bits<119> {}
impl Reg128BitsDownCast<19> for Reg128Bits<119> {}
impl Reg128BitsDownCast<20> for Reg128Bits<119> {}
impl Reg128BitsDownCast<21> for Reg128Bits<119> {}
impl Reg128BitsDownCast<22> for Reg128Bits<119> {}
impl Reg128BitsDownCast<23> for Reg128Bits<119> {}
impl Reg128BitsDownCast<24> for Reg128Bits<119> {}
impl Reg128BitsDownCast<25> for Reg128Bits<119> {}
impl Reg128BitsDownCast<26> for Reg128Bits<119> {}
impl Reg128BitsDownCast<27> for Reg128Bits<119> {}
impl Reg128BitsDownCast<28> for Reg128Bits<119> {}
impl Reg128BitsDownCast<29> for Reg128Bits<119> {}
impl Reg128BitsDownCast<30> for Reg128Bits<119> {}
impl Reg128BitsDownCast<31> for Reg128Bits<119> {}
impl Reg128BitsDownCast<32> for Reg128Bits<119> {}
impl Reg128BitsDownCast<33> for Reg128Bits<119> {}
impl Reg128BitsDownCast<34> for Reg128Bits<119> {}
impl Reg128BitsDownCast<35> for Reg128Bits<119> {}
impl Reg128BitsDownCast<36> for Reg128Bits<119> {}
impl Reg128BitsDownCast<37> for Reg128Bits<119> {}
impl Reg128BitsDownCast<38> for Reg128Bits<119> {}
impl Reg128BitsDownCast<39> for Reg128Bits<119> {}
impl Reg128BitsDownCast<40> for Reg128Bits<119> {}
impl Reg128BitsDownCast<41> for Reg128Bits<119> {}
impl Reg128BitsDownCast<42> for Reg128Bits<119> {}
impl Reg128BitsDownCast<43> for Reg128Bits<119> {}
impl Reg128BitsDownCast<44> for Reg128Bits<119> {}
impl Reg128BitsDownCast<45> for Reg128Bits<119> {}
impl Reg128BitsDownCast<46> for Reg128Bits<119> {}
impl Reg128BitsDownCast<47> for Reg128Bits<119> {}
impl Reg128BitsDownCast<48> for Reg128Bits<119> {}
impl Reg128BitsDownCast<49> for Reg128Bits<119> {}
impl Reg128BitsDownCast<50> for Reg128Bits<119> {}
impl Reg128BitsDownCast<51> for Reg128Bits<119> {}
impl Reg128BitsDownCast<52> for Reg128Bits<119> {}
impl Reg128BitsDownCast<53> for Reg128Bits<119> {}
impl Reg128BitsDownCast<54> for Reg128Bits<119> {}
impl Reg128BitsDownCast<55> for Reg128Bits<119> {}
impl Reg128BitsDownCast<56> for Reg128Bits<119> {}
impl Reg128BitsDownCast<57> for Reg128Bits<119> {}
impl Reg128BitsDownCast<58> for Reg128Bits<119> {}
impl Reg128BitsDownCast<59> for Reg128Bits<119> {}
impl Reg128BitsDownCast<60> for Reg128Bits<119> {}
impl Reg128BitsDownCast<61> for Reg128Bits<119> {}
impl Reg128BitsDownCast<62> for Reg128Bits<119> {}
impl Reg128BitsDownCast<63> for Reg128Bits<119> {}
impl Reg128BitsDownCast<64> for Reg128Bits<119> {}
impl Reg128BitsDownCast<65> for Reg128Bits<119> {}
impl Reg128BitsDownCast<66> for Reg128Bits<119> {}
impl Reg128BitsDownCast<67> for Reg128Bits<119> {}
impl Reg128BitsDownCast<68> for Reg128Bits<119> {}
impl Reg128BitsDownCast<69> for Reg128Bits<119> {}
impl Reg128BitsDownCast<70> for Reg128Bits<119> {}
impl Reg128BitsDownCast<71> for Reg128Bits<119> {}
impl Reg128BitsDownCast<72> for Reg128Bits<119> {}
impl Reg128BitsDownCast<73> for Reg128Bits<119> {}
impl Reg128BitsDownCast<74> for Reg128Bits<119> {}
impl Reg128BitsDownCast<75> for Reg128Bits<119> {}
impl Reg128BitsDownCast<76> for Reg128Bits<119> {}
impl Reg128BitsDownCast<77> for Reg128Bits<119> {}
impl Reg128BitsDownCast<78> for Reg128Bits<119> {}
impl Reg128BitsDownCast<79> for Reg128Bits<119> {}
impl Reg128BitsDownCast<80> for Reg128Bits<119> {}
impl Reg128BitsDownCast<81> for Reg128Bits<119> {}
impl Reg128BitsDownCast<82> for Reg128Bits<119> {}
impl Reg128BitsDownCast<83> for Reg128Bits<119> {}
impl Reg128BitsDownCast<84> for Reg128Bits<119> {}
impl Reg128BitsDownCast<85> for Reg128Bits<119> {}
impl Reg128BitsDownCast<86> for Reg128Bits<119> {}
impl Reg128BitsDownCast<87> for Reg128Bits<119> {}
impl Reg128BitsDownCast<88> for Reg128Bits<119> {}
impl Reg128BitsDownCast<89> for Reg128Bits<119> {}
impl Reg128BitsDownCast<90> for Reg128Bits<119> {}
impl Reg128BitsDownCast<91> for Reg128Bits<119> {}
impl Reg128BitsDownCast<92> for Reg128Bits<119> {}
impl Reg128BitsDownCast<93> for Reg128Bits<119> {}
impl Reg128BitsDownCast<94> for Reg128Bits<119> {}
impl Reg128BitsDownCast<95> for Reg128Bits<119> {}
impl Reg128BitsDownCast<96> for Reg128Bits<119> {}
impl Reg128BitsDownCast<97> for Reg128Bits<119> {}
impl Reg128BitsDownCast<98> for Reg128Bits<119> {}
impl Reg128BitsDownCast<99> for Reg128Bits<119> {}
impl Reg128BitsDownCast<100> for Reg128Bits<119> {}
impl Reg128BitsDownCast<101> for Reg128Bits<119> {}
impl Reg128BitsDownCast<102> for Reg128Bits<119> {}
impl Reg128BitsDownCast<103> for Reg128Bits<119> {}
impl Reg128BitsDownCast<104> for Reg128Bits<119> {}
impl Reg128BitsDownCast<105> for Reg128Bits<119> {}
impl Reg128BitsDownCast<106> for Reg128Bits<119> {}
impl Reg128BitsDownCast<107> for Reg128Bits<119> {}
impl Reg128BitsDownCast<108> for Reg128Bits<119> {}
impl Reg128BitsDownCast<109> for Reg128Bits<119> {}
impl Reg128BitsDownCast<110> for Reg128Bits<119> {}
impl Reg128BitsDownCast<111> for Reg128Bits<119> {}
impl Reg128BitsDownCast<112> for Reg128Bits<119> {}
impl Reg128BitsDownCast<113> for Reg128Bits<119> {}
impl Reg128BitsDownCast<114> for Reg128Bits<119> {}
impl Reg128BitsDownCast<115> for Reg128Bits<119> {}
impl Reg128BitsDownCast<116> for Reg128Bits<119> {}
impl Reg128BitsDownCast<117> for Reg128Bits<119> {}
impl Reg128BitsDownCast<118> for Reg128Bits<119> {}
impl Reg128BitsDownCast<119> for Reg128Bits<119> {}
impl Reg128BitsDownCast<1> for Reg128Bits<120> {}
impl Reg128BitsConcat<1, 121> for Reg128Bits<120> {}
impl Reg128BitsDownCast<2> for Reg128Bits<120> {}
impl Reg128BitsConcat<2, 122> for Reg128Bits<120> {}
impl Reg128BitsDownCast<3> for Reg128Bits<120> {}
impl Reg128BitsConcat<3, 123> for Reg128Bits<120> {}
impl Reg128BitsDownCast<4> for Reg128Bits<120> {}
impl Reg128BitsConcat<4, 124> for Reg128Bits<120> {}
impl Reg128BitsDownCast<5> for Reg128Bits<120> {}
impl Reg128BitsConcat<5, 125> for Reg128Bits<120> {}
impl Reg128BitsDownCast<6> for Reg128Bits<120> {}
impl Reg128BitsConcat<6, 126> for Reg128Bits<120> {}
impl Reg128BitsDownCast<7> for Reg128Bits<120> {}
impl Reg128BitsConcat<7, 127> for Reg128Bits<120> {}
impl Reg128BitsDownCast<8> for Reg128Bits<120> {}
impl Reg128BitsConcat<8, 128> for Reg128Bits<120> {}
impl Reg128BitsDownCast<9> for Reg128Bits<120> {}
impl Reg128BitsDownCast<10> for Reg128Bits<120> {}
impl Reg128BitsDownCast<11> for Reg128Bits<120> {}
impl Reg128BitsDownCast<12> for Reg128Bits<120> {}
impl Reg128BitsDownCast<13> for Reg128Bits<120> {}
impl Reg128BitsDownCast<14> for Reg128Bits<120> {}
impl Reg128BitsDownCast<15> for Reg128Bits<120> {}
impl Reg128BitsDownCast<16> for Reg128Bits<120> {}
impl Reg128BitsDownCast<17> for Reg128Bits<120> {}
impl Reg128BitsDownCast<18> for Reg128Bits<120> {}
impl Reg128BitsDownCast<19> for Reg128Bits<120> {}
impl Reg128BitsDownCast<20> for Reg128Bits<120> {}
impl Reg128BitsDownCast<21> for Reg128Bits<120> {}
impl Reg128BitsDownCast<22> for Reg128Bits<120> {}
impl Reg128BitsDownCast<23> for Reg128Bits<120> {}
impl Reg128BitsDownCast<24> for Reg128Bits<120> {}
impl Reg128BitsDownCast<25> for Reg128Bits<120> {}
impl Reg128BitsDownCast<26> for Reg128Bits<120> {}
impl Reg128BitsDownCast<27> for Reg128Bits<120> {}
impl Reg128BitsDownCast<28> for Reg128Bits<120> {}
impl Reg128BitsDownCast<29> for Reg128Bits<120> {}
impl Reg128BitsDownCast<30> for Reg128Bits<120> {}
impl Reg128BitsDownCast<31> for Reg128Bits<120> {}
impl Reg128BitsDownCast<32> for Reg128Bits<120> {}
impl Reg128BitsDownCast<33> for Reg128Bits<120> {}
impl Reg128BitsDownCast<34> for Reg128Bits<120> {}
impl Reg128BitsDownCast<35> for Reg128Bits<120> {}
impl Reg128BitsDownCast<36> for Reg128Bits<120> {}
impl Reg128BitsDownCast<37> for Reg128Bits<120> {}
impl Reg128BitsDownCast<38> for Reg128Bits<120> {}
impl Reg128BitsDownCast<39> for Reg128Bits<120> {}
impl Reg128BitsDownCast<40> for Reg128Bits<120> {}
impl Reg128BitsDownCast<41> for Reg128Bits<120> {}
impl Reg128BitsDownCast<42> for Reg128Bits<120> {}
impl Reg128BitsDownCast<43> for Reg128Bits<120> {}
impl Reg128BitsDownCast<44> for Reg128Bits<120> {}
impl Reg128BitsDownCast<45> for Reg128Bits<120> {}
impl Reg128BitsDownCast<46> for Reg128Bits<120> {}
impl Reg128BitsDownCast<47> for Reg128Bits<120> {}
impl Reg128BitsDownCast<48> for Reg128Bits<120> {}
impl Reg128BitsDownCast<49> for Reg128Bits<120> {}
impl Reg128BitsDownCast<50> for Reg128Bits<120> {}
impl Reg128BitsDownCast<51> for Reg128Bits<120> {}
impl Reg128BitsDownCast<52> for Reg128Bits<120> {}
impl Reg128BitsDownCast<53> for Reg128Bits<120> {}
impl Reg128BitsDownCast<54> for Reg128Bits<120> {}
impl Reg128BitsDownCast<55> for Reg128Bits<120> {}
impl Reg128BitsDownCast<56> for Reg128Bits<120> {}
impl Reg128BitsDownCast<57> for Reg128Bits<120> {}
impl Reg128BitsDownCast<58> for Reg128Bits<120> {}
impl Reg128BitsDownCast<59> for Reg128Bits<120> {}
impl Reg128BitsDownCast<60> for Reg128Bits<120> {}
impl Reg128BitsDownCast<61> for Reg128Bits<120> {}
impl Reg128BitsDownCast<62> for Reg128Bits<120> {}
impl Reg128BitsDownCast<63> for Reg128Bits<120> {}
impl Reg128BitsDownCast<64> for Reg128Bits<120> {}
impl Reg128BitsDownCast<65> for Reg128Bits<120> {}
impl Reg128BitsDownCast<66> for Reg128Bits<120> {}
impl Reg128BitsDownCast<67> for Reg128Bits<120> {}
impl Reg128BitsDownCast<68> for Reg128Bits<120> {}
impl Reg128BitsDownCast<69> for Reg128Bits<120> {}
impl Reg128BitsDownCast<70> for Reg128Bits<120> {}
impl Reg128BitsDownCast<71> for Reg128Bits<120> {}
impl Reg128BitsDownCast<72> for Reg128Bits<120> {}
impl Reg128BitsDownCast<73> for Reg128Bits<120> {}
impl Reg128BitsDownCast<74> for Reg128Bits<120> {}
impl Reg128BitsDownCast<75> for Reg128Bits<120> {}
impl Reg128BitsDownCast<76> for Reg128Bits<120> {}
impl Reg128BitsDownCast<77> for Reg128Bits<120> {}
impl Reg128BitsDownCast<78> for Reg128Bits<120> {}
impl Reg128BitsDownCast<79> for Reg128Bits<120> {}
impl Reg128BitsDownCast<80> for Reg128Bits<120> {}
impl Reg128BitsDownCast<81> for Reg128Bits<120> {}
impl Reg128BitsDownCast<82> for Reg128Bits<120> {}
impl Reg128BitsDownCast<83> for Reg128Bits<120> {}
impl Reg128BitsDownCast<84> for Reg128Bits<120> {}
impl Reg128BitsDownCast<85> for Reg128Bits<120> {}
impl Reg128BitsDownCast<86> for Reg128Bits<120> {}
impl Reg128BitsDownCast<87> for Reg128Bits<120> {}
impl Reg128BitsDownCast<88> for Reg128Bits<120> {}
impl Reg128BitsDownCast<89> for Reg128Bits<120> {}
impl Reg128BitsDownCast<90> for Reg128Bits<120> {}
impl Reg128BitsDownCast<91> for Reg128Bits<120> {}
impl Reg128BitsDownCast<92> for Reg128Bits<120> {}
impl Reg128BitsDownCast<93> for Reg128Bits<120> {}
impl Reg128BitsDownCast<94> for Reg128Bits<120> {}
impl Reg128BitsDownCast<95> for Reg128Bits<120> {}
impl Reg128BitsDownCast<96> for Reg128Bits<120> {}
impl Reg128BitsDownCast<97> for Reg128Bits<120> {}
impl Reg128BitsDownCast<98> for Reg128Bits<120> {}
impl Reg128BitsDownCast<99> for Reg128Bits<120> {}
impl Reg128BitsDownCast<100> for Reg128Bits<120> {}
impl Reg128BitsDownCast<101> for Reg128Bits<120> {}
impl Reg128BitsDownCast<102> for Reg128Bits<120> {}
impl Reg128BitsDownCast<103> for Reg128Bits<120> {}
impl Reg128BitsDownCast<104> for Reg128Bits<120> {}
impl Reg128BitsDownCast<105> for Reg128Bits<120> {}
impl Reg128BitsDownCast<106> for Reg128Bits<120> {}
impl Reg128BitsDownCast<107> for Reg128Bits<120> {}
impl Reg128BitsDownCast<108> for Reg128Bits<120> {}
impl Reg128BitsDownCast<109> for Reg128Bits<120> {}
impl Reg128BitsDownCast<110> for Reg128Bits<120> {}
impl Reg128BitsDownCast<111> for Reg128Bits<120> {}
impl Reg128BitsDownCast<112> for Reg128Bits<120> {}
impl Reg128BitsDownCast<113> for Reg128Bits<120> {}
impl Reg128BitsDownCast<114> for Reg128Bits<120> {}
impl Reg128BitsDownCast<115> for Reg128Bits<120> {}
impl Reg128BitsDownCast<116> for Reg128Bits<120> {}
impl Reg128BitsDownCast<117> for Reg128Bits<120> {}
impl Reg128BitsDownCast<118> for Reg128Bits<120> {}
impl Reg128BitsDownCast<119> for Reg128Bits<120> {}
impl Reg128BitsDownCast<120> for Reg128Bits<120> {}
impl Reg128BitsDownCast<1> for Reg128Bits<121> {}
impl Reg128BitsConcat<1, 122> for Reg128Bits<121> {}
impl Reg128BitsDownCast<2> for Reg128Bits<121> {}
impl Reg128BitsConcat<2, 123> for Reg128Bits<121> {}
impl Reg128BitsDownCast<3> for Reg128Bits<121> {}
impl Reg128BitsConcat<3, 124> for Reg128Bits<121> {}
impl Reg128BitsDownCast<4> for Reg128Bits<121> {}
impl Reg128BitsConcat<4, 125> for Reg128Bits<121> {}
impl Reg128BitsDownCast<5> for Reg128Bits<121> {}
impl Reg128BitsConcat<5, 126> for Reg128Bits<121> {}
impl Reg128BitsDownCast<6> for Reg128Bits<121> {}
impl Reg128BitsConcat<6, 127> for Reg128Bits<121> {}
impl Reg128BitsDownCast<7> for Reg128Bits<121> {}
impl Reg128BitsConcat<7, 128> for Reg128Bits<121> {}
impl Reg128BitsDownCast<8> for Reg128Bits<121> {}
impl Reg128BitsDownCast<9> for Reg128Bits<121> {}
impl Reg128BitsDownCast<10> for Reg128Bits<121> {}
impl Reg128BitsDownCast<11> for Reg128Bits<121> {}
impl Reg128BitsDownCast<12> for Reg128Bits<121> {}
impl Reg128BitsDownCast<13> for Reg128Bits<121> {}
impl Reg128BitsDownCast<14> for Reg128Bits<121> {}
impl Reg128BitsDownCast<15> for Reg128Bits<121> {}
impl Reg128BitsDownCast<16> for Reg128Bits<121> {}
impl Reg128BitsDownCast<17> for Reg128Bits<121> {}
impl Reg128BitsDownCast<18> for Reg128Bits<121> {}
impl Reg128BitsDownCast<19> for Reg128Bits<121> {}
impl Reg128BitsDownCast<20> for Reg128Bits<121> {}
impl Reg128BitsDownCast<21> for Reg128Bits<121> {}
impl Reg128BitsDownCast<22> for Reg128Bits<121> {}
impl Reg128BitsDownCast<23> for Reg128Bits<121> {}
impl Reg128BitsDownCast<24> for Reg128Bits<121> {}
impl Reg128BitsDownCast<25> for Reg128Bits<121> {}
impl Reg128BitsDownCast<26> for Reg128Bits<121> {}
impl Reg128BitsDownCast<27> for Reg128Bits<121> {}
impl Reg128BitsDownCast<28> for Reg128Bits<121> {}
impl Reg128BitsDownCast<29> for Reg128Bits<121> {}
impl Reg128BitsDownCast<30> for Reg128Bits<121> {}
impl Reg128BitsDownCast<31> for Reg128Bits<121> {}
impl Reg128BitsDownCast<32> for Reg128Bits<121> {}
impl Reg128BitsDownCast<33> for Reg128Bits<121> {}
impl Reg128BitsDownCast<34> for Reg128Bits<121> {}
impl Reg128BitsDownCast<35> for Reg128Bits<121> {}
impl Reg128BitsDownCast<36> for Reg128Bits<121> {}
impl Reg128BitsDownCast<37> for Reg128Bits<121> {}
impl Reg128BitsDownCast<38> for Reg128Bits<121> {}
impl Reg128BitsDownCast<39> for Reg128Bits<121> {}
impl Reg128BitsDownCast<40> for Reg128Bits<121> {}
impl Reg128BitsDownCast<41> for Reg128Bits<121> {}
impl Reg128BitsDownCast<42> for Reg128Bits<121> {}
impl Reg128BitsDownCast<43> for Reg128Bits<121> {}
impl Reg128BitsDownCast<44> for Reg128Bits<121> {}
impl Reg128BitsDownCast<45> for Reg128Bits<121> {}
impl Reg128BitsDownCast<46> for Reg128Bits<121> {}
impl Reg128BitsDownCast<47> for Reg128Bits<121> {}
impl Reg128BitsDownCast<48> for Reg128Bits<121> {}
impl Reg128BitsDownCast<49> for Reg128Bits<121> {}
impl Reg128BitsDownCast<50> for Reg128Bits<121> {}
impl Reg128BitsDownCast<51> for Reg128Bits<121> {}
impl Reg128BitsDownCast<52> for Reg128Bits<121> {}
impl Reg128BitsDownCast<53> for Reg128Bits<121> {}
impl Reg128BitsDownCast<54> for Reg128Bits<121> {}
impl Reg128BitsDownCast<55> for Reg128Bits<121> {}
impl Reg128BitsDownCast<56> for Reg128Bits<121> {}
impl Reg128BitsDownCast<57> for Reg128Bits<121> {}
impl Reg128BitsDownCast<58> for Reg128Bits<121> {}
impl Reg128BitsDownCast<59> for Reg128Bits<121> {}
impl Reg128BitsDownCast<60> for Reg128Bits<121> {}
impl Reg128BitsDownCast<61> for Reg128Bits<121> {}
impl Reg128BitsDownCast<62> for Reg128Bits<121> {}
impl Reg128BitsDownCast<63> for Reg128Bits<121> {}
impl Reg128BitsDownCast<64> for Reg128Bits<121> {}
impl Reg128BitsDownCast<65> for Reg128Bits<121> {}
impl Reg128BitsDownCast<66> for Reg128Bits<121> {}
impl Reg128BitsDownCast<67> for Reg128Bits<121> {}
impl Reg128BitsDownCast<68> for Reg128Bits<121> {}
impl Reg128BitsDownCast<69> for Reg128Bits<121> {}
impl Reg128BitsDownCast<70> for Reg128Bits<121> {}
impl Reg128BitsDownCast<71> for Reg128Bits<121> {}
impl Reg128BitsDownCast<72> for Reg128Bits<121> {}
impl Reg128BitsDownCast<73> for Reg128Bits<121> {}
impl Reg128BitsDownCast<74> for Reg128Bits<121> {}
impl Reg128BitsDownCast<75> for Reg128Bits<121> {}
impl Reg128BitsDownCast<76> for Reg128Bits<121> {}
impl Reg128BitsDownCast<77> for Reg128Bits<121> {}
impl Reg128BitsDownCast<78> for Reg128Bits<121> {}
impl Reg128BitsDownCast<79> for Reg128Bits<121> {}
impl Reg128BitsDownCast<80> for Reg128Bits<121> {}
impl Reg128BitsDownCast<81> for Reg128Bits<121> {}
impl Reg128BitsDownCast<82> for Reg128Bits<121> {}
impl Reg128BitsDownCast<83> for Reg128Bits<121> {}
impl Reg128BitsDownCast<84> for Reg128Bits<121> {}
impl Reg128BitsDownCast<85> for Reg128Bits<121> {}
impl Reg128BitsDownCast<86> for Reg128Bits<121> {}
impl Reg128BitsDownCast<87> for Reg128Bits<121> {}
impl Reg128BitsDownCast<88> for Reg128Bits<121> {}
impl Reg128BitsDownCast<89> for Reg128Bits<121> {}
impl Reg128BitsDownCast<90> for Reg128Bits<121> {}
impl Reg128BitsDownCast<91> for Reg128Bits<121> {}
impl Reg128BitsDownCast<92> for Reg128Bits<121> {}
impl Reg128BitsDownCast<93> for Reg128Bits<121> {}
impl Reg128BitsDownCast<94> for Reg128Bits<121> {}
impl Reg128BitsDownCast<95> for Reg128Bits<121> {}
impl Reg128BitsDownCast<96> for Reg128Bits<121> {}
impl Reg128BitsDownCast<97> for Reg128Bits<121> {}
impl Reg128BitsDownCast<98> for Reg128Bits<121> {}
impl Reg128BitsDownCast<99> for Reg128Bits<121> {}
impl Reg128BitsDownCast<100> for Reg128Bits<121> {}
impl Reg128BitsDownCast<101> for Reg128Bits<121> {}
impl Reg128BitsDownCast<102> for Reg128Bits<121> {}
impl Reg128BitsDownCast<103> for Reg128Bits<121> {}
impl Reg128BitsDownCast<104> for Reg128Bits<121> {}
impl Reg128BitsDownCast<105> for Reg128Bits<121> {}
impl Reg128BitsDownCast<106> for Reg128Bits<121> {}
impl Reg128BitsDownCast<107> for Reg128Bits<121> {}
impl Reg128BitsDownCast<108> for Reg128Bits<121> {}
impl Reg128BitsDownCast<109> for Reg128Bits<121> {}
impl Reg128BitsDownCast<110> for Reg128Bits<121> {}
impl Reg128BitsDownCast<111> for Reg128Bits<121> {}
impl Reg128BitsDownCast<112> for Reg128Bits<121> {}
impl Reg128BitsDownCast<113> for Reg128Bits<121> {}
impl Reg128BitsDownCast<114> for Reg128Bits<121> {}
impl Reg128BitsDownCast<115> for Reg128Bits<121> {}
impl Reg128BitsDownCast<116> for Reg128Bits<121> {}
impl Reg128BitsDownCast<117> for Reg128Bits<121> {}
impl Reg128BitsDownCast<118> for Reg128Bits<121> {}
impl Reg128BitsDownCast<119> for Reg128Bits<121> {}
impl Reg128BitsDownCast<120> for Reg128Bits<121> {}
impl Reg128BitsDownCast<121> for Reg128Bits<121> {}
impl Reg128BitsDownCast<1> for Reg128Bits<122> {}
impl Reg128BitsConcat<1, 123> for Reg128Bits<122> {}
impl Reg128BitsDownCast<2> for Reg128Bits<122> {}
impl Reg128BitsConcat<2, 124> for Reg128Bits<122> {}
impl Reg128BitsDownCast<3> for Reg128Bits<122> {}
impl Reg128BitsConcat<3, 125> for Reg128Bits<122> {}
impl Reg128BitsDownCast<4> for Reg128Bits<122> {}
impl Reg128BitsConcat<4, 126> for Reg128Bits<122> {}
impl Reg128BitsDownCast<5> for Reg128Bits<122> {}
impl Reg128BitsConcat<5, 127> for Reg128Bits<122> {}
impl Reg128BitsDownCast<6> for Reg128Bits<122> {}
impl Reg128BitsConcat<6, 128> for Reg128Bits<122> {}
impl Reg128BitsDownCast<7> for Reg128Bits<122> {}
impl Reg128BitsDownCast<8> for Reg128Bits<122> {}
impl Reg128BitsDownCast<9> for Reg128Bits<122> {}
impl Reg128BitsDownCast<10> for Reg128Bits<122> {}
impl Reg128BitsDownCast<11> for Reg128Bits<122> {}
impl Reg128BitsDownCast<12> for Reg128Bits<122> {}
impl Reg128BitsDownCast<13> for Reg128Bits<122> {}
impl Reg128BitsDownCast<14> for Reg128Bits<122> {}
impl Reg128BitsDownCast<15> for Reg128Bits<122> {}
impl Reg128BitsDownCast<16> for Reg128Bits<122> {}
impl Reg128BitsDownCast<17> for Reg128Bits<122> {}
impl Reg128BitsDownCast<18> for Reg128Bits<122> {}
impl Reg128BitsDownCast<19> for Reg128Bits<122> {}
impl Reg128BitsDownCast<20> for Reg128Bits<122> {}
impl Reg128BitsDownCast<21> for Reg128Bits<122> {}
impl Reg128BitsDownCast<22> for Reg128Bits<122> {}
impl Reg128BitsDownCast<23> for Reg128Bits<122> {}
impl Reg128BitsDownCast<24> for Reg128Bits<122> {}
impl Reg128BitsDownCast<25> for Reg128Bits<122> {}
impl Reg128BitsDownCast<26> for Reg128Bits<122> {}
impl Reg128BitsDownCast<27> for Reg128Bits<122> {}
impl Reg128BitsDownCast<28> for Reg128Bits<122> {}
impl Reg128BitsDownCast<29> for Reg128Bits<122> {}
impl Reg128BitsDownCast<30> for Reg128Bits<122> {}
impl Reg128BitsDownCast<31> for Reg128Bits<122> {}
impl Reg128BitsDownCast<32> for Reg128Bits<122> {}
impl Reg128BitsDownCast<33> for Reg128Bits<122> {}
impl Reg128BitsDownCast<34> for Reg128Bits<122> {}
impl Reg128BitsDownCast<35> for Reg128Bits<122> {}
impl Reg128BitsDownCast<36> for Reg128Bits<122> {}
impl Reg128BitsDownCast<37> for Reg128Bits<122> {}
impl Reg128BitsDownCast<38> for Reg128Bits<122> {}
impl Reg128BitsDownCast<39> for Reg128Bits<122> {}
impl Reg128BitsDownCast<40> for Reg128Bits<122> {}
impl Reg128BitsDownCast<41> for Reg128Bits<122> {}
impl Reg128BitsDownCast<42> for Reg128Bits<122> {}
impl Reg128BitsDownCast<43> for Reg128Bits<122> {}
impl Reg128BitsDownCast<44> for Reg128Bits<122> {}
impl Reg128BitsDownCast<45> for Reg128Bits<122> {}
impl Reg128BitsDownCast<46> for Reg128Bits<122> {}
impl Reg128BitsDownCast<47> for Reg128Bits<122> {}
impl Reg128BitsDownCast<48> for Reg128Bits<122> {}
impl Reg128BitsDownCast<49> for Reg128Bits<122> {}
impl Reg128BitsDownCast<50> for Reg128Bits<122> {}
impl Reg128BitsDownCast<51> for Reg128Bits<122> {}
impl Reg128BitsDownCast<52> for Reg128Bits<122> {}
impl Reg128BitsDownCast<53> for Reg128Bits<122> {}
impl Reg128BitsDownCast<54> for Reg128Bits<122> {}
impl Reg128BitsDownCast<55> for Reg128Bits<122> {}
impl Reg128BitsDownCast<56> for Reg128Bits<122> {}
impl Reg128BitsDownCast<57> for Reg128Bits<122> {}
impl Reg128BitsDownCast<58> for Reg128Bits<122> {}
impl Reg128BitsDownCast<59> for Reg128Bits<122> {}
impl Reg128BitsDownCast<60> for Reg128Bits<122> {}
impl Reg128BitsDownCast<61> for Reg128Bits<122> {}
impl Reg128BitsDownCast<62> for Reg128Bits<122> {}
impl Reg128BitsDownCast<63> for Reg128Bits<122> {}
impl Reg128BitsDownCast<64> for Reg128Bits<122> {}
impl Reg128BitsDownCast<65> for Reg128Bits<122> {}
impl Reg128BitsDownCast<66> for Reg128Bits<122> {}
impl Reg128BitsDownCast<67> for Reg128Bits<122> {}
impl Reg128BitsDownCast<68> for Reg128Bits<122> {}
impl Reg128BitsDownCast<69> for Reg128Bits<122> {}
impl Reg128BitsDownCast<70> for Reg128Bits<122> {}
impl Reg128BitsDownCast<71> for Reg128Bits<122> {}
impl Reg128BitsDownCast<72> for Reg128Bits<122> {}
impl Reg128BitsDownCast<73> for Reg128Bits<122> {}
impl Reg128BitsDownCast<74> for Reg128Bits<122> {}
impl Reg128BitsDownCast<75> for Reg128Bits<122> {}
impl Reg128BitsDownCast<76> for Reg128Bits<122> {}
impl Reg128BitsDownCast<77> for Reg128Bits<122> {}
impl Reg128BitsDownCast<78> for Reg128Bits<122> {}
impl Reg128BitsDownCast<79> for Reg128Bits<122> {}
impl Reg128BitsDownCast<80> for Reg128Bits<122> {}
impl Reg128BitsDownCast<81> for Reg128Bits<122> {}
impl Reg128BitsDownCast<82> for Reg128Bits<122> {}
impl Reg128BitsDownCast<83> for Reg128Bits<122> {}
impl Reg128BitsDownCast<84> for Reg128Bits<122> {}
impl Reg128BitsDownCast<85> for Reg128Bits<122> {}
impl Reg128BitsDownCast<86> for Reg128Bits<122> {}
impl Reg128BitsDownCast<87> for Reg128Bits<122> {}
impl Reg128BitsDownCast<88> for Reg128Bits<122> {}
impl Reg128BitsDownCast<89> for Reg128Bits<122> {}
impl Reg128BitsDownCast<90> for Reg128Bits<122> {}
impl Reg128BitsDownCast<91> for Reg128Bits<122> {}
impl Reg128BitsDownCast<92> for Reg128Bits<122> {}
impl Reg128BitsDownCast<93> for Reg128Bits<122> {}
impl Reg128BitsDownCast<94> for Reg128Bits<122> {}
impl Reg128BitsDownCast<95> for Reg128Bits<122> {}
impl Reg128BitsDownCast<96> for Reg128Bits<122> {}
impl Reg128BitsDownCast<97> for Reg128Bits<122> {}
impl Reg128BitsDownCast<98> for Reg128Bits<122> {}
impl Reg128BitsDownCast<99> for Reg128Bits<122> {}
impl Reg128BitsDownCast<100> for Reg128Bits<122> {}
impl Reg128BitsDownCast<101> for Reg128Bits<122> {}
impl Reg128BitsDownCast<102> for Reg128Bits<122> {}
impl Reg128BitsDownCast<103> for Reg128Bits<122> {}
impl Reg128BitsDownCast<104> for Reg128Bits<122> {}
impl Reg128BitsDownCast<105> for Reg128Bits<122> {}
impl Reg128BitsDownCast<106> for Reg128Bits<122> {}
impl Reg128BitsDownCast<107> for Reg128Bits<122> {}
impl Reg128BitsDownCast<108> for Reg128Bits<122> {}
impl Reg128BitsDownCast<109> for Reg128Bits<122> {}
impl Reg128BitsDownCast<110> for Reg128Bits<122> {}
impl Reg128BitsDownCast<111> for Reg128Bits<122> {}
impl Reg128BitsDownCast<112> for Reg128Bits<122> {}
impl Reg128BitsDownCast<113> for Reg128Bits<122> {}
impl Reg128BitsDownCast<114> for Reg128Bits<122> {}
impl Reg128BitsDownCast<115> for Reg128Bits<122> {}
impl Reg128BitsDownCast<116> for Reg128Bits<122> {}
impl Reg128BitsDownCast<117> for Reg128Bits<122> {}
impl Reg128BitsDownCast<118> for Reg128Bits<122> {}
impl Reg128BitsDownCast<119> for Reg128Bits<122> {}
impl Reg128BitsDownCast<120> for Reg128Bits<122> {}
impl Reg128BitsDownCast<121> for Reg128Bits<122> {}
impl Reg128BitsDownCast<122> for Reg128Bits<122> {}
impl Reg128BitsDownCast<1> for Reg128Bits<123> {}
impl Reg128BitsConcat<1, 124> for Reg128Bits<123> {}
impl Reg128BitsDownCast<2> for Reg128Bits<123> {}
impl Reg128BitsConcat<2, 125> for Reg128Bits<123> {}
impl Reg128BitsDownCast<3> for Reg128Bits<123> {}
impl Reg128BitsConcat<3, 126> for Reg128Bits<123> {}
impl Reg128BitsDownCast<4> for Reg128Bits<123> {}
impl Reg128BitsConcat<4, 127> for Reg128Bits<123> {}
impl Reg128BitsDownCast<5> for Reg128Bits<123> {}
impl Reg128BitsConcat<5, 128> for Reg128Bits<123> {}
impl Reg128BitsDownCast<6> for Reg128Bits<123> {}
impl Reg128BitsDownCast<7> for Reg128Bits<123> {}
impl Reg128BitsDownCast<8> for Reg128Bits<123> {}
impl Reg128BitsDownCast<9> for Reg128Bits<123> {}
impl Reg128BitsDownCast<10> for Reg128Bits<123> {}
impl Reg128BitsDownCast<11> for Reg128Bits<123> {}
impl Reg128BitsDownCast<12> for Reg128Bits<123> {}
impl Reg128BitsDownCast<13> for Reg128Bits<123> {}
impl Reg128BitsDownCast<14> for Reg128Bits<123> {}
impl Reg128BitsDownCast<15> for Reg128Bits<123> {}
impl Reg128BitsDownCast<16> for Reg128Bits<123> {}
impl Reg128BitsDownCast<17> for Reg128Bits<123> {}
impl Reg128BitsDownCast<18> for Reg128Bits<123> {}
impl Reg128BitsDownCast<19> for Reg128Bits<123> {}
impl Reg128BitsDownCast<20> for Reg128Bits<123> {}
impl Reg128BitsDownCast<21> for Reg128Bits<123> {}
impl Reg128BitsDownCast<22> for Reg128Bits<123> {}
impl Reg128BitsDownCast<23> for Reg128Bits<123> {}
impl Reg128BitsDownCast<24> for Reg128Bits<123> {}
impl Reg128BitsDownCast<25> for Reg128Bits<123> {}
impl Reg128BitsDownCast<26> for Reg128Bits<123> {}
impl Reg128BitsDownCast<27> for Reg128Bits<123> {}
impl Reg128BitsDownCast<28> for Reg128Bits<123> {}
impl Reg128BitsDownCast<29> for Reg128Bits<123> {}
impl Reg128BitsDownCast<30> for Reg128Bits<123> {}
impl Reg128BitsDownCast<31> for Reg128Bits<123> {}
impl Reg128BitsDownCast<32> for Reg128Bits<123> {}
impl Reg128BitsDownCast<33> for Reg128Bits<123> {}
impl Reg128BitsDownCast<34> for Reg128Bits<123> {}
impl Reg128BitsDownCast<35> for Reg128Bits<123> {}
impl Reg128BitsDownCast<36> for Reg128Bits<123> {}
impl Reg128BitsDownCast<37> for Reg128Bits<123> {}
impl Reg128BitsDownCast<38> for Reg128Bits<123> {}
impl Reg128BitsDownCast<39> for Reg128Bits<123> {}
impl Reg128BitsDownCast<40> for Reg128Bits<123> {}
impl Reg128BitsDownCast<41> for Reg128Bits<123> {}
impl Reg128BitsDownCast<42> for Reg128Bits<123> {}
impl Reg128BitsDownCast<43> for Reg128Bits<123> {}
impl Reg128BitsDownCast<44> for Reg128Bits<123> {}
impl Reg128BitsDownCast<45> for Reg128Bits<123> {}
impl Reg128BitsDownCast<46> for Reg128Bits<123> {}
impl Reg128BitsDownCast<47> for Reg128Bits<123> {}
impl Reg128BitsDownCast<48> for Reg128Bits<123> {}
impl Reg128BitsDownCast<49> for Reg128Bits<123> {}
impl Reg128BitsDownCast<50> for Reg128Bits<123> {}
impl Reg128BitsDownCast<51> for Reg128Bits<123> {}
impl Reg128BitsDownCast<52> for Reg128Bits<123> {}
impl Reg128BitsDownCast<53> for Reg128Bits<123> {}
impl Reg128BitsDownCast<54> for Reg128Bits<123> {}
impl Reg128BitsDownCast<55> for Reg128Bits<123> {}
impl Reg128BitsDownCast<56> for Reg128Bits<123> {}
impl Reg128BitsDownCast<57> for Reg128Bits<123> {}
impl Reg128BitsDownCast<58> for Reg128Bits<123> {}
impl Reg128BitsDownCast<59> for Reg128Bits<123> {}
impl Reg128BitsDownCast<60> for Reg128Bits<123> {}
impl Reg128BitsDownCast<61> for Reg128Bits<123> {}
impl Reg128BitsDownCast<62> for Reg128Bits<123> {}
impl Reg128BitsDownCast<63> for Reg128Bits<123> {}
impl Reg128BitsDownCast<64> for Reg128Bits<123> {}
impl Reg128BitsDownCast<65> for Reg128Bits<123> {}
impl Reg128BitsDownCast<66> for Reg128Bits<123> {}
impl Reg128BitsDownCast<67> for Reg128Bits<123> {}
impl Reg128BitsDownCast<68> for Reg128Bits<123> {}
impl Reg128BitsDownCast<69> for Reg128Bits<123> {}
impl Reg128BitsDownCast<70> for Reg128Bits<123> {}
impl Reg128BitsDownCast<71> for Reg128Bits<123> {}
impl Reg128BitsDownCast<72> for Reg128Bits<123> {}
impl Reg128BitsDownCast<73> for Reg128Bits<123> {}
impl Reg128BitsDownCast<74> for Reg128Bits<123> {}
impl Reg128BitsDownCast<75> for Reg128Bits<123> {}
impl Reg128BitsDownCast<76> for Reg128Bits<123> {}
impl Reg128BitsDownCast<77> for Reg128Bits<123> {}
impl Reg128BitsDownCast<78> for Reg128Bits<123> {}
impl Reg128BitsDownCast<79> for Reg128Bits<123> {}
impl Reg128BitsDownCast<80> for Reg128Bits<123> {}
impl Reg128BitsDownCast<81> for Reg128Bits<123> {}
impl Reg128BitsDownCast<82> for Reg128Bits<123> {}
impl Reg128BitsDownCast<83> for Reg128Bits<123> {}
impl Reg128BitsDownCast<84> for Reg128Bits<123> {}
impl Reg128BitsDownCast<85> for Reg128Bits<123> {}
impl Reg128BitsDownCast<86> for Reg128Bits<123> {}
impl Reg128BitsDownCast<87> for Reg128Bits<123> {}
impl Reg128BitsDownCast<88> for Reg128Bits<123> {}
impl Reg128BitsDownCast<89> for Reg128Bits<123> {}
impl Reg128BitsDownCast<90> for Reg128Bits<123> {}
impl Reg128BitsDownCast<91> for Reg128Bits<123> {}
impl Reg128BitsDownCast<92> for Reg128Bits<123> {}
impl Reg128BitsDownCast<93> for Reg128Bits<123> {}
impl Reg128BitsDownCast<94> for Reg128Bits<123> {}
impl Reg128BitsDownCast<95> for Reg128Bits<123> {}
impl Reg128BitsDownCast<96> for Reg128Bits<123> {}
impl Reg128BitsDownCast<97> for Reg128Bits<123> {}
impl Reg128BitsDownCast<98> for Reg128Bits<123> {}
impl Reg128BitsDownCast<99> for Reg128Bits<123> {}
impl Reg128BitsDownCast<100> for Reg128Bits<123> {}
impl Reg128BitsDownCast<101> for Reg128Bits<123> {}
impl Reg128BitsDownCast<102> for Reg128Bits<123> {}
impl Reg128BitsDownCast<103> for Reg128Bits<123> {}
impl Reg128BitsDownCast<104> for Reg128Bits<123> {}
impl Reg128BitsDownCast<105> for Reg128Bits<123> {}
impl Reg128BitsDownCast<106> for Reg128Bits<123> {}
impl Reg128BitsDownCast<107> for Reg128Bits<123> {}
impl Reg128BitsDownCast<108> for Reg128Bits<123> {}
impl Reg128BitsDownCast<109> for Reg128Bits<123> {}
impl Reg128BitsDownCast<110> for Reg128Bits<123> {}
impl Reg128BitsDownCast<111> for Reg128Bits<123> {}
impl Reg128BitsDownCast<112> for Reg128Bits<123> {}
impl Reg128BitsDownCast<113> for Reg128Bits<123> {}
impl Reg128BitsDownCast<114> for Reg128Bits<123> {}
impl Reg128BitsDownCast<115> for Reg128Bits<123> {}
impl Reg128BitsDownCast<116> for Reg128Bits<123> {}
impl Reg128BitsDownCast<117> for Reg128Bits<123> {}
impl Reg128BitsDownCast<118> for Reg128Bits<123> {}
impl Reg128BitsDownCast<119> for Reg128Bits<123> {}
impl Reg128BitsDownCast<120> for Reg128Bits<123> {}
impl Reg128BitsDownCast<121> for Reg128Bits<123> {}
impl Reg128BitsDownCast<122> for Reg128Bits<123> {}
impl Reg128BitsDownCast<123> for Reg128Bits<123> {}
impl Reg128BitsDownCast<1> for Reg128Bits<124> {}
impl Reg128BitsConcat<1, 125> for Reg128Bits<124> {}
impl Reg128BitsDownCast<2> for Reg128Bits<124> {}
impl Reg128BitsConcat<2, 126> for Reg128Bits<124> {}
impl Reg128BitsDownCast<3> for Reg128Bits<124> {}
impl Reg128BitsConcat<3, 127> for Reg128Bits<124> {}
impl Reg128BitsDownCast<4> for Reg128Bits<124> {}
impl Reg128BitsConcat<4, 128> for Reg128Bits<124> {}
impl Reg128BitsDownCast<5> for Reg128Bits<124> {}
impl Reg128BitsDownCast<6> for Reg128Bits<124> {}
impl Reg128BitsDownCast<7> for Reg128Bits<124> {}
impl Reg128BitsDownCast<8> for Reg128Bits<124> {}
impl Reg128BitsDownCast<9> for Reg128Bits<124> {}
impl Reg128BitsDownCast<10> for Reg128Bits<124> {}
impl Reg128BitsDownCast<11> for Reg128Bits<124> {}
impl Reg128BitsDownCast<12> for Reg128Bits<124> {}
impl Reg128BitsDownCast<13> for Reg128Bits<124> {}
impl Reg128BitsDownCast<14> for Reg128Bits<124> {}
impl Reg128BitsDownCast<15> for Reg128Bits<124> {}
impl Reg128BitsDownCast<16> for Reg128Bits<124> {}
impl Reg128BitsDownCast<17> for Reg128Bits<124> {}
impl Reg128BitsDownCast<18> for Reg128Bits<124> {}
impl Reg128BitsDownCast<19> for Reg128Bits<124> {}
impl Reg128BitsDownCast<20> for Reg128Bits<124> {}
impl Reg128BitsDownCast<21> for Reg128Bits<124> {}
impl Reg128BitsDownCast<22> for Reg128Bits<124> {}
impl Reg128BitsDownCast<23> for Reg128Bits<124> {}
impl Reg128BitsDownCast<24> for Reg128Bits<124> {}
impl Reg128BitsDownCast<25> for Reg128Bits<124> {}
impl Reg128BitsDownCast<26> for Reg128Bits<124> {}
impl Reg128BitsDownCast<27> for Reg128Bits<124> {}
impl Reg128BitsDownCast<28> for Reg128Bits<124> {}
impl Reg128BitsDownCast<29> for Reg128Bits<124> {}
impl Reg128BitsDownCast<30> for Reg128Bits<124> {}
impl Reg128BitsDownCast<31> for Reg128Bits<124> {}
impl Reg128BitsDownCast<32> for Reg128Bits<124> {}
impl Reg128BitsDownCast<33> for Reg128Bits<124> {}
impl Reg128BitsDownCast<34> for Reg128Bits<124> {}
impl Reg128BitsDownCast<35> for Reg128Bits<124> {}
impl Reg128BitsDownCast<36> for Reg128Bits<124> {}
impl Reg128BitsDownCast<37> for Reg128Bits<124> {}
impl Reg128BitsDownCast<38> for Reg128Bits<124> {}
impl Reg128BitsDownCast<39> for Reg128Bits<124> {}
impl Reg128BitsDownCast<40> for Reg128Bits<124> {}
impl Reg128BitsDownCast<41> for Reg128Bits<124> {}
impl Reg128BitsDownCast<42> for Reg128Bits<124> {}
impl Reg128BitsDownCast<43> for Reg128Bits<124> {}
impl Reg128BitsDownCast<44> for Reg128Bits<124> {}
impl Reg128BitsDownCast<45> for Reg128Bits<124> {}
impl Reg128BitsDownCast<46> for Reg128Bits<124> {}
impl Reg128BitsDownCast<47> for Reg128Bits<124> {}
impl Reg128BitsDownCast<48> for Reg128Bits<124> {}
impl Reg128BitsDownCast<49> for Reg128Bits<124> {}
impl Reg128BitsDownCast<50> for Reg128Bits<124> {}
impl Reg128BitsDownCast<51> for Reg128Bits<124> {}
impl Reg128BitsDownCast<52> for Reg128Bits<124> {}
impl Reg128BitsDownCast<53> for Reg128Bits<124> {}
impl Reg128BitsDownCast<54> for Reg128Bits<124> {}
impl Reg128BitsDownCast<55> for Reg128Bits<124> {}
impl Reg128BitsDownCast<56> for Reg128Bits<124> {}
impl Reg128BitsDownCast<57> for Reg128Bits<124> {}
impl Reg128BitsDownCast<58> for Reg128Bits<124> {}
impl Reg128BitsDownCast<59> for Reg128Bits<124> {}
impl Reg128BitsDownCast<60> for Reg128Bits<124> {}
impl Reg128BitsDownCast<61> for Reg128Bits<124> {}
impl Reg128BitsDownCast<62> for Reg128Bits<124> {}
impl Reg128BitsDownCast<63> for Reg128Bits<124> {}
impl Reg128BitsDownCast<64> for Reg128Bits<124> {}
impl Reg128BitsDownCast<65> for Reg128Bits<124> {}
impl Reg128BitsDownCast<66> for Reg128Bits<124> {}
impl Reg128BitsDownCast<67> for Reg128Bits<124> {}
impl Reg128BitsDownCast<68> for Reg128Bits<124> {}
impl Reg128BitsDownCast<69> for Reg128Bits<124> {}
impl Reg128BitsDownCast<70> for Reg128Bits<124> {}
impl Reg128BitsDownCast<71> for Reg128Bits<124> {}
impl Reg128BitsDownCast<72> for Reg128Bits<124> {}
impl Reg128BitsDownCast<73> for Reg128Bits<124> {}
impl Reg128BitsDownCast<74> for Reg128Bits<124> {}
impl Reg128BitsDownCast<75> for Reg128Bits<124> {}
impl Reg128BitsDownCast<76> for Reg128Bits<124> {}
impl Reg128BitsDownCast<77> for Reg128Bits<124> {}
impl Reg128BitsDownCast<78> for Reg128Bits<124> {}
impl Reg128BitsDownCast<79> for Reg128Bits<124> {}
impl Reg128BitsDownCast<80> for Reg128Bits<124> {}
impl Reg128BitsDownCast<81> for Reg128Bits<124> {}
impl Reg128BitsDownCast<82> for Reg128Bits<124> {}
impl Reg128BitsDownCast<83> for Reg128Bits<124> {}
impl Reg128BitsDownCast<84> for Reg128Bits<124> {}
impl Reg128BitsDownCast<85> for Reg128Bits<124> {}
impl Reg128BitsDownCast<86> for Reg128Bits<124> {}
impl Reg128BitsDownCast<87> for Reg128Bits<124> {}
impl Reg128BitsDownCast<88> for Reg128Bits<124> {}
impl Reg128BitsDownCast<89> for Reg128Bits<124> {}
impl Reg128BitsDownCast<90> for Reg128Bits<124> {}
impl Reg128BitsDownCast<91> for Reg128Bits<124> {}
impl Reg128BitsDownCast<92> for Reg128Bits<124> {}
impl Reg128BitsDownCast<93> for Reg128Bits<124> {}
impl Reg128BitsDownCast<94> for Reg128Bits<124> {}
impl Reg128BitsDownCast<95> for Reg128Bits<124> {}
impl Reg128BitsDownCast<96> for Reg128Bits<124> {}
impl Reg128BitsDownCast<97> for Reg128Bits<124> {}
impl Reg128BitsDownCast<98> for Reg128Bits<124> {}
impl Reg128BitsDownCast<99> for Reg128Bits<124> {}
impl Reg128BitsDownCast<100> for Reg128Bits<124> {}
impl Reg128BitsDownCast<101> for Reg128Bits<124> {}
impl Reg128BitsDownCast<102> for Reg128Bits<124> {}
impl Reg128BitsDownCast<103> for Reg128Bits<124> {}
impl Reg128BitsDownCast<104> for Reg128Bits<124> {}
impl Reg128BitsDownCast<105> for Reg128Bits<124> {}
impl Reg128BitsDownCast<106> for Reg128Bits<124> {}
impl Reg128BitsDownCast<107> for Reg128Bits<124> {}
impl Reg128BitsDownCast<108> for Reg128Bits<124> {}
impl Reg128BitsDownCast<109> for Reg128Bits<124> {}
impl Reg128BitsDownCast<110> for Reg128Bits<124> {}
impl Reg128BitsDownCast<111> for Reg128Bits<124> {}
impl Reg128BitsDownCast<112> for Reg128Bits<124> {}
impl Reg128BitsDownCast<113> for Reg128Bits<124> {}
impl Reg128BitsDownCast<114> for Reg128Bits<124> {}
impl Reg128BitsDownCast<115> for Reg128Bits<124> {}
impl Reg128BitsDownCast<116> for Reg128Bits<124> {}
impl Reg128BitsDownCast<117> for Reg128Bits<124> {}
impl Reg128BitsDownCast<118> for Reg128Bits<124> {}
impl Reg128BitsDownCast<119> for Reg128Bits<124> {}
impl Reg128BitsDownCast<120> for Reg128Bits<124> {}
impl Reg128BitsDownCast<121> for Reg128Bits<124> {}
impl Reg128BitsDownCast<122> for Reg128Bits<124> {}
impl Reg128BitsDownCast<123> for Reg128Bits<124> {}
impl Reg128BitsDownCast<124> for Reg128Bits<124> {}
impl Reg128BitsDownCast<1> for Reg128Bits<125> {}
impl Reg128BitsConcat<1, 126> for Reg128Bits<125> {}
impl Reg128BitsDownCast<2> for Reg128Bits<125> {}
impl Reg128BitsConcat<2, 127> for Reg128Bits<125> {}
impl Reg128BitsDownCast<3> for Reg128Bits<125> {}
impl Reg128BitsConcat<3, 128> for Reg128Bits<125> {}
impl Reg128BitsDownCast<4> for Reg128Bits<125> {}
impl Reg128BitsDownCast<5> for Reg128Bits<125> {}
impl Reg128BitsDownCast<6> for Reg128Bits<125> {}
impl Reg128BitsDownCast<7> for Reg128Bits<125> {}
impl Reg128BitsDownCast<8> for Reg128Bits<125> {}
impl Reg128BitsDownCast<9> for Reg128Bits<125> {}
impl Reg128BitsDownCast<10> for Reg128Bits<125> {}
impl Reg128BitsDownCast<11> for Reg128Bits<125> {}
impl Reg128BitsDownCast<12> for Reg128Bits<125> {}
impl Reg128BitsDownCast<13> for Reg128Bits<125> {}
impl Reg128BitsDownCast<14> for Reg128Bits<125> {}
impl Reg128BitsDownCast<15> for Reg128Bits<125> {}
impl Reg128BitsDownCast<16> for Reg128Bits<125> {}
impl Reg128BitsDownCast<17> for Reg128Bits<125> {}
impl Reg128BitsDownCast<18> for Reg128Bits<125> {}
impl Reg128BitsDownCast<19> for Reg128Bits<125> {}
impl Reg128BitsDownCast<20> for Reg128Bits<125> {}
impl Reg128BitsDownCast<21> for Reg128Bits<125> {}
impl Reg128BitsDownCast<22> for Reg128Bits<125> {}
impl Reg128BitsDownCast<23> for Reg128Bits<125> {}
impl Reg128BitsDownCast<24> for Reg128Bits<125> {}
impl Reg128BitsDownCast<25> for Reg128Bits<125> {}
impl Reg128BitsDownCast<26> for Reg128Bits<125> {}
impl Reg128BitsDownCast<27> for Reg128Bits<125> {}
impl Reg128BitsDownCast<28> for Reg128Bits<125> {}
impl Reg128BitsDownCast<29> for Reg128Bits<125> {}
impl Reg128BitsDownCast<30> for Reg128Bits<125> {}
impl Reg128BitsDownCast<31> for Reg128Bits<125> {}
impl Reg128BitsDownCast<32> for Reg128Bits<125> {}
impl Reg128BitsDownCast<33> for Reg128Bits<125> {}
impl Reg128BitsDownCast<34> for Reg128Bits<125> {}
impl Reg128BitsDownCast<35> for Reg128Bits<125> {}
impl Reg128BitsDownCast<36> for Reg128Bits<125> {}
impl Reg128BitsDownCast<37> for Reg128Bits<125> {}
impl Reg128BitsDownCast<38> for Reg128Bits<125> {}
impl Reg128BitsDownCast<39> for Reg128Bits<125> {}
impl Reg128BitsDownCast<40> for Reg128Bits<125> {}
impl Reg128BitsDownCast<41> for Reg128Bits<125> {}
impl Reg128BitsDownCast<42> for Reg128Bits<125> {}
impl Reg128BitsDownCast<43> for Reg128Bits<125> {}
impl Reg128BitsDownCast<44> for Reg128Bits<125> {}
impl Reg128BitsDownCast<45> for Reg128Bits<125> {}
impl Reg128BitsDownCast<46> for Reg128Bits<125> {}
impl Reg128BitsDownCast<47> for Reg128Bits<125> {}
impl Reg128BitsDownCast<48> for Reg128Bits<125> {}
impl Reg128BitsDownCast<49> for Reg128Bits<125> {}
impl Reg128BitsDownCast<50> for Reg128Bits<125> {}
impl Reg128BitsDownCast<51> for Reg128Bits<125> {}
impl Reg128BitsDownCast<52> for Reg128Bits<125> {}
impl Reg128BitsDownCast<53> for Reg128Bits<125> {}
impl Reg128BitsDownCast<54> for Reg128Bits<125> {}
impl Reg128BitsDownCast<55> for Reg128Bits<125> {}
impl Reg128BitsDownCast<56> for Reg128Bits<125> {}
impl Reg128BitsDownCast<57> for Reg128Bits<125> {}
impl Reg128BitsDownCast<58> for Reg128Bits<125> {}
impl Reg128BitsDownCast<59> for Reg128Bits<125> {}
impl Reg128BitsDownCast<60> for Reg128Bits<125> {}
impl Reg128BitsDownCast<61> for Reg128Bits<125> {}
impl Reg128BitsDownCast<62> for Reg128Bits<125> {}
impl Reg128BitsDownCast<63> for Reg128Bits<125> {}
impl Reg128BitsDownCast<64> for Reg128Bits<125> {}
impl Reg128BitsDownCast<65> for Reg128Bits<125> {}
impl Reg128BitsDownCast<66> for Reg128Bits<125> {}
impl Reg128BitsDownCast<67> for Reg128Bits<125> {}
impl Reg128BitsDownCast<68> for Reg128Bits<125> {}
impl Reg128BitsDownCast<69> for Reg128Bits<125> {}
impl Reg128BitsDownCast<70> for Reg128Bits<125> {}
impl Reg128BitsDownCast<71> for Reg128Bits<125> {}
impl Reg128BitsDownCast<72> for Reg128Bits<125> {}
impl Reg128BitsDownCast<73> for Reg128Bits<125> {}
impl Reg128BitsDownCast<74> for Reg128Bits<125> {}
impl Reg128BitsDownCast<75> for Reg128Bits<125> {}
impl Reg128BitsDownCast<76> for Reg128Bits<125> {}
impl Reg128BitsDownCast<77> for Reg128Bits<125> {}
impl Reg128BitsDownCast<78> for Reg128Bits<125> {}
impl Reg128BitsDownCast<79> for Reg128Bits<125> {}
impl Reg128BitsDownCast<80> for Reg128Bits<125> {}
impl Reg128BitsDownCast<81> for Reg128Bits<125> {}
impl Reg128BitsDownCast<82> for Reg128Bits<125> {}
impl Reg128BitsDownCast<83> for Reg128Bits<125> {}
impl Reg128BitsDownCast<84> for Reg128Bits<125> {}
impl Reg128BitsDownCast<85> for Reg128Bits<125> {}
impl Reg128BitsDownCast<86> for Reg128Bits<125> {}
impl Reg128BitsDownCast<87> for Reg128Bits<125> {}
impl Reg128BitsDownCast<88> for Reg128Bits<125> {}
impl Reg128BitsDownCast<89> for Reg128Bits<125> {}
impl Reg128BitsDownCast<90> for Reg128Bits<125> {}
impl Reg128BitsDownCast<91> for Reg128Bits<125> {}
impl Reg128BitsDownCast<92> for Reg128Bits<125> {}
impl Reg128BitsDownCast<93> for Reg128Bits<125> {}
impl Reg128BitsDownCast<94> for Reg128Bits<125> {}
impl Reg128BitsDownCast<95> for Reg128Bits<125> {}
impl Reg128BitsDownCast<96> for Reg128Bits<125> {}
impl Reg128BitsDownCast<97> for Reg128Bits<125> {}
impl Reg128BitsDownCast<98> for Reg128Bits<125> {}
impl Reg128BitsDownCast<99> for Reg128Bits<125> {}
impl Reg128BitsDownCast<100> for Reg128Bits<125> {}
impl Reg128BitsDownCast<101> for Reg128Bits<125> {}
impl Reg128BitsDownCast<102> for Reg128Bits<125> {}
impl Reg128BitsDownCast<103> for Reg128Bits<125> {}
impl Reg128BitsDownCast<104> for Reg128Bits<125> {}
impl Reg128BitsDownCast<105> for Reg128Bits<125> {}
impl Reg128BitsDownCast<106> for Reg128Bits<125> {}
impl Reg128BitsDownCast<107> for Reg128Bits<125> {}
impl Reg128BitsDownCast<108> for Reg128Bits<125> {}
impl Reg128BitsDownCast<109> for Reg128Bits<125> {}
impl Reg128BitsDownCast<110> for Reg128Bits<125> {}
impl Reg128BitsDownCast<111> for Reg128Bits<125> {}
impl Reg128BitsDownCast<112> for Reg128Bits<125> {}
impl Reg128BitsDownCast<113> for Reg128Bits<125> {}
impl Reg128BitsDownCast<114> for Reg128Bits<125> {}
impl Reg128BitsDownCast<115> for Reg128Bits<125> {}
impl Reg128BitsDownCast<116> for Reg128Bits<125> {}
impl Reg128BitsDownCast<117> for Reg128Bits<125> {}
impl Reg128BitsDownCast<118> for Reg128Bits<125> {}
impl Reg128BitsDownCast<119> for Reg128Bits<125> {}
impl Reg128BitsDownCast<120> for Reg128Bits<125> {}
impl Reg128BitsDownCast<121> for Reg128Bits<125> {}
impl Reg128BitsDownCast<122> for Reg128Bits<125> {}
impl Reg128BitsDownCast<123> for Reg128Bits<125> {}
impl Reg128BitsDownCast<124> for Reg128Bits<125> {}
impl Reg128BitsDownCast<125> for Reg128Bits<125> {}
impl Reg128BitsDownCast<1> for Reg128Bits<126> {}
impl Reg128BitsConcat<1, 127> for Reg128Bits<126> {}
impl Reg128BitsDownCast<2> for Reg128Bits<126> {}
impl Reg128BitsConcat<2, 128> for Reg128Bits<126> {}
impl Reg128BitsDownCast<3> for Reg128Bits<126> {}
impl Reg128BitsDownCast<4> for Reg128Bits<126> {}
impl Reg128BitsDownCast<5> for Reg128Bits<126> {}
impl Reg128BitsDownCast<6> for Reg128Bits<126> {}
impl Reg128BitsDownCast<7> for Reg128Bits<126> {}
impl Reg128BitsDownCast<8> for Reg128Bits<126> {}
impl Reg128BitsDownCast<9> for Reg128Bits<126> {}
impl Reg128BitsDownCast<10> for Reg128Bits<126> {}
impl Reg128BitsDownCast<11> for Reg128Bits<126> {}
impl Reg128BitsDownCast<12> for Reg128Bits<126> {}
impl Reg128BitsDownCast<13> for Reg128Bits<126> {}
impl Reg128BitsDownCast<14> for Reg128Bits<126> {}
impl Reg128BitsDownCast<15> for Reg128Bits<126> {}
impl Reg128BitsDownCast<16> for Reg128Bits<126> {}
impl Reg128BitsDownCast<17> for Reg128Bits<126> {}
impl Reg128BitsDownCast<18> for Reg128Bits<126> {}
impl Reg128BitsDownCast<19> for Reg128Bits<126> {}
impl Reg128BitsDownCast<20> for Reg128Bits<126> {}
impl Reg128BitsDownCast<21> for Reg128Bits<126> {}
impl Reg128BitsDownCast<22> for Reg128Bits<126> {}
impl Reg128BitsDownCast<23> for Reg128Bits<126> {}
impl Reg128BitsDownCast<24> for Reg128Bits<126> {}
impl Reg128BitsDownCast<25> for Reg128Bits<126> {}
impl Reg128BitsDownCast<26> for Reg128Bits<126> {}
impl Reg128BitsDownCast<27> for Reg128Bits<126> {}
impl Reg128BitsDownCast<28> for Reg128Bits<126> {}
impl Reg128BitsDownCast<29> for Reg128Bits<126> {}
impl Reg128BitsDownCast<30> for Reg128Bits<126> {}
impl Reg128BitsDownCast<31> for Reg128Bits<126> {}
impl Reg128BitsDownCast<32> for Reg128Bits<126> {}
impl Reg128BitsDownCast<33> for Reg128Bits<126> {}
impl Reg128BitsDownCast<34> for Reg128Bits<126> {}
impl Reg128BitsDownCast<35> for Reg128Bits<126> {}
impl Reg128BitsDownCast<36> for Reg128Bits<126> {}
impl Reg128BitsDownCast<37> for Reg128Bits<126> {}
impl Reg128BitsDownCast<38> for Reg128Bits<126> {}
impl Reg128BitsDownCast<39> for Reg128Bits<126> {}
impl Reg128BitsDownCast<40> for Reg128Bits<126> {}
impl Reg128BitsDownCast<41> for Reg128Bits<126> {}
impl Reg128BitsDownCast<42> for Reg128Bits<126> {}
impl Reg128BitsDownCast<43> for Reg128Bits<126> {}
impl Reg128BitsDownCast<44> for Reg128Bits<126> {}
impl Reg128BitsDownCast<45> for Reg128Bits<126> {}
impl Reg128BitsDownCast<46> for Reg128Bits<126> {}
impl Reg128BitsDownCast<47> for Reg128Bits<126> {}
impl Reg128BitsDownCast<48> for Reg128Bits<126> {}
impl Reg128BitsDownCast<49> for Reg128Bits<126> {}
impl Reg128BitsDownCast<50> for Reg128Bits<126> {}
impl Reg128BitsDownCast<51> for Reg128Bits<126> {}
impl Reg128BitsDownCast<52> for Reg128Bits<126> {}
impl Reg128BitsDownCast<53> for Reg128Bits<126> {}
impl Reg128BitsDownCast<54> for Reg128Bits<126> {}
impl Reg128BitsDownCast<55> for Reg128Bits<126> {}
impl Reg128BitsDownCast<56> for Reg128Bits<126> {}
impl Reg128BitsDownCast<57> for Reg128Bits<126> {}
impl Reg128BitsDownCast<58> for Reg128Bits<126> {}
impl Reg128BitsDownCast<59> for Reg128Bits<126> {}
impl Reg128BitsDownCast<60> for Reg128Bits<126> {}
impl Reg128BitsDownCast<61> for Reg128Bits<126> {}
impl Reg128BitsDownCast<62> for Reg128Bits<126> {}
impl Reg128BitsDownCast<63> for Reg128Bits<126> {}
impl Reg128BitsDownCast<64> for Reg128Bits<126> {}
impl Reg128BitsDownCast<65> for Reg128Bits<126> {}
impl Reg128BitsDownCast<66> for Reg128Bits<126> {}
impl Reg128BitsDownCast<67> for Reg128Bits<126> {}
impl Reg128BitsDownCast<68> for Reg128Bits<126> {}
impl Reg128BitsDownCast<69> for Reg128Bits<126> {}
impl Reg128BitsDownCast<70> for Reg128Bits<126> {}
impl Reg128BitsDownCast<71> for Reg128Bits<126> {}
impl Reg128BitsDownCast<72> for Reg128Bits<126> {}
impl Reg128BitsDownCast<73> for Reg128Bits<126> {}
impl Reg128BitsDownCast<74> for Reg128Bits<126> {}
impl Reg128BitsDownCast<75> for Reg128Bits<126> {}
impl Reg128BitsDownCast<76> for Reg128Bits<126> {}
impl Reg128BitsDownCast<77> for Reg128Bits<126> {}
impl Reg128BitsDownCast<78> for Reg128Bits<126> {}
impl Reg128BitsDownCast<79> for Reg128Bits<126> {}
impl Reg128BitsDownCast<80> for Reg128Bits<126> {}
impl Reg128BitsDownCast<81> for Reg128Bits<126> {}
impl Reg128BitsDownCast<82> for Reg128Bits<126> {}
impl Reg128BitsDownCast<83> for Reg128Bits<126> {}
impl Reg128BitsDownCast<84> for Reg128Bits<126> {}
impl Reg128BitsDownCast<85> for Reg128Bits<126> {}
impl Reg128BitsDownCast<86> for Reg128Bits<126> {}
impl Reg128BitsDownCast<87> for Reg128Bits<126> {}
impl Reg128BitsDownCast<88> for Reg128Bits<126> {}
impl Reg128BitsDownCast<89> for Reg128Bits<126> {}
impl Reg128BitsDownCast<90> for Reg128Bits<126> {}
impl Reg128BitsDownCast<91> for Reg128Bits<126> {}
impl Reg128BitsDownCast<92> for Reg128Bits<126> {}
impl Reg128BitsDownCast<93> for Reg128Bits<126> {}
impl Reg128BitsDownCast<94> for Reg128Bits<126> {}
impl Reg128BitsDownCast<95> for Reg128Bits<126> {}
impl Reg128BitsDownCast<96> for Reg128Bits<126> {}
impl Reg128BitsDownCast<97> for Reg128Bits<126> {}
impl Reg128BitsDownCast<98> for Reg128Bits<126> {}
impl Reg128BitsDownCast<99> for Reg128Bits<126> {}
impl Reg128BitsDownCast<100> for Reg128Bits<126> {}
impl Reg128BitsDownCast<101> for Reg128Bits<126> {}
impl Reg128BitsDownCast<102> for Reg128Bits<126> {}
impl Reg128BitsDownCast<103> for Reg128Bits<126> {}
impl Reg128BitsDownCast<104> for Reg128Bits<126> {}
impl Reg128BitsDownCast<105> for Reg128Bits<126> {}
impl Reg128BitsDownCast<106> for Reg128Bits<126> {}
impl Reg128BitsDownCast<107> for Reg128Bits<126> {}
impl Reg128BitsDownCast<108> for Reg128Bits<126> {}
impl Reg128BitsDownCast<109> for Reg128Bits<126> {}
impl Reg128BitsDownCast<110> for Reg128Bits<126> {}
impl Reg128BitsDownCast<111> for Reg128Bits<126> {}
impl Reg128BitsDownCast<112> for Reg128Bits<126> {}
impl Reg128BitsDownCast<113> for Reg128Bits<126> {}
impl Reg128BitsDownCast<114> for Reg128Bits<126> {}
impl Reg128BitsDownCast<115> for Reg128Bits<126> {}
impl Reg128BitsDownCast<116> for Reg128Bits<126> {}
impl Reg128BitsDownCast<117> for Reg128Bits<126> {}
impl Reg128BitsDownCast<118> for Reg128Bits<126> {}
impl Reg128BitsDownCast<119> for Reg128Bits<126> {}
impl Reg128BitsDownCast<120> for Reg128Bits<126> {}
impl Reg128BitsDownCast<121> for Reg128Bits<126> {}
impl Reg128BitsDownCast<122> for Reg128Bits<126> {}
impl Reg128BitsDownCast<123> for Reg128Bits<126> {}
impl Reg128BitsDownCast<124> for Reg128Bits<126> {}
impl Reg128BitsDownCast<125> for Reg128Bits<126> {}
impl Reg128BitsDownCast<126> for Reg128Bits<126> {}
impl Reg128BitsDownCast<1> for Reg128Bits<127> {}
impl Reg128BitsConcat<1, 128> for Reg128Bits<127> {}
impl Reg128BitsDownCast<2> for Reg128Bits<127> {}
impl Reg128BitsDownCast<3> for Reg128Bits<127> {}
impl Reg128BitsDownCast<4> for Reg128Bits<127> {}
impl Reg128BitsDownCast<5> for Reg128Bits<127> {}
impl Reg128BitsDownCast<6> for Reg128Bits<127> {}
impl Reg128BitsDownCast<7> for Reg128Bits<127> {}
impl Reg128BitsDownCast<8> for Reg128Bits<127> {}
impl Reg128BitsDownCast<9> for Reg128Bits<127> {}
impl Reg128BitsDownCast<10> for Reg128Bits<127> {}
impl Reg128BitsDownCast<11> for Reg128Bits<127> {}
impl Reg128BitsDownCast<12> for Reg128Bits<127> {}
impl Reg128BitsDownCast<13> for Reg128Bits<127> {}
impl Reg128BitsDownCast<14> for Reg128Bits<127> {}
impl Reg128BitsDownCast<15> for Reg128Bits<127> {}
impl Reg128BitsDownCast<16> for Reg128Bits<127> {}
impl Reg128BitsDownCast<17> for Reg128Bits<127> {}
impl Reg128BitsDownCast<18> for Reg128Bits<127> {}
impl Reg128BitsDownCast<19> for Reg128Bits<127> {}
impl Reg128BitsDownCast<20> for Reg128Bits<127> {}
impl Reg128BitsDownCast<21> for Reg128Bits<127> {}
impl Reg128BitsDownCast<22> for Reg128Bits<127> {}
impl Reg128BitsDownCast<23> for Reg128Bits<127> {}
impl Reg128BitsDownCast<24> for Reg128Bits<127> {}
impl Reg128BitsDownCast<25> for Reg128Bits<127> {}
impl Reg128BitsDownCast<26> for Reg128Bits<127> {}
impl Reg128BitsDownCast<27> for Reg128Bits<127> {}
impl Reg128BitsDownCast<28> for Reg128Bits<127> {}
impl Reg128BitsDownCast<29> for Reg128Bits<127> {}
impl Reg128BitsDownCast<30> for Reg128Bits<127> {}
impl Reg128BitsDownCast<31> for Reg128Bits<127> {}
impl Reg128BitsDownCast<32> for Reg128Bits<127> {}
impl Reg128BitsDownCast<33> for Reg128Bits<127> {}
impl Reg128BitsDownCast<34> for Reg128Bits<127> {}
impl Reg128BitsDownCast<35> for Reg128Bits<127> {}
impl Reg128BitsDownCast<36> for Reg128Bits<127> {}
impl Reg128BitsDownCast<37> for Reg128Bits<127> {}
impl Reg128BitsDownCast<38> for Reg128Bits<127> {}
impl Reg128BitsDownCast<39> for Reg128Bits<127> {}
impl Reg128BitsDownCast<40> for Reg128Bits<127> {}
impl Reg128BitsDownCast<41> for Reg128Bits<127> {}
impl Reg128BitsDownCast<42> for Reg128Bits<127> {}
impl Reg128BitsDownCast<43> for Reg128Bits<127> {}
impl Reg128BitsDownCast<44> for Reg128Bits<127> {}
impl Reg128BitsDownCast<45> for Reg128Bits<127> {}
impl Reg128BitsDownCast<46> for Reg128Bits<127> {}
impl Reg128BitsDownCast<47> for Reg128Bits<127> {}
impl Reg128BitsDownCast<48> for Reg128Bits<127> {}
impl Reg128BitsDownCast<49> for Reg128Bits<127> {}
impl Reg128BitsDownCast<50> for Reg128Bits<127> {}
impl Reg128BitsDownCast<51> for Reg128Bits<127> {}
impl Reg128BitsDownCast<52> for Reg128Bits<127> {}
impl Reg128BitsDownCast<53> for Reg128Bits<127> {}
impl Reg128BitsDownCast<54> for Reg128Bits<127> {}
impl Reg128BitsDownCast<55> for Reg128Bits<127> {}
impl Reg128BitsDownCast<56> for Reg128Bits<127> {}
impl Reg128BitsDownCast<57> for Reg128Bits<127> {}
impl Reg128BitsDownCast<58> for Reg128Bits<127> {}
impl Reg128BitsDownCast<59> for Reg128Bits<127> {}
impl Reg128BitsDownCast<60> for Reg128Bits<127> {}
impl Reg128BitsDownCast<61> for Reg128Bits<127> {}
impl Reg128BitsDownCast<62> for Reg128Bits<127> {}
impl Reg128BitsDownCast<63> for Reg128Bits<127> {}
impl Reg128BitsDownCast<64> for Reg128Bits<127> {}
impl Reg128BitsDownCast<65> for Reg128Bits<127> {}
impl Reg128BitsDownCast<66> for Reg128Bits<127> {}
impl Reg128BitsDownCast<67> for Reg128Bits<127> {}
impl Reg128BitsDownCast<68> for Reg128Bits<127> {}
impl Reg128BitsDownCast<69> for Reg128Bits<127> {}
impl Reg128BitsDownCast<70> for Reg128Bits<127> {}
impl Reg128BitsDownCast<71> for Reg128Bits<127> {}
impl Reg128BitsDownCast<72> for Reg128Bits<127> {}
impl Reg128BitsDownCast<73> for Reg128Bits<127> {}
impl Reg128BitsDownCast<74> for Reg128Bits<127> {}
impl Reg128BitsDownCast<75> for Reg128Bits<127> {}
impl Reg128BitsDownCast<76> for Reg128Bits<127> {}
impl Reg128BitsDownCast<77> for Reg128Bits<127> {}
impl Reg128BitsDownCast<78> for Reg128Bits<127> {}
impl Reg128BitsDownCast<79> for Reg128Bits<127> {}
impl Reg128BitsDownCast<80> for Reg128Bits<127> {}
impl Reg128BitsDownCast<81> for Reg128Bits<127> {}
impl Reg128BitsDownCast<82> for Reg128Bits<127> {}
impl Reg128BitsDownCast<83> for Reg128Bits<127> {}
impl Reg128BitsDownCast<84> for Reg128Bits<127> {}
impl Reg128BitsDownCast<85> for Reg128Bits<127> {}
impl Reg128BitsDownCast<86> for Reg128Bits<127> {}
impl Reg128BitsDownCast<87> for Reg128Bits<127> {}
impl Reg128BitsDownCast<88> for Reg128Bits<127> {}
impl Reg128BitsDownCast<89> for Reg128Bits<127> {}
impl Reg128BitsDownCast<90> for Reg128Bits<127> {}
impl Reg128BitsDownCast<91> for Reg128Bits<127> {}
impl Reg128BitsDownCast<92> for Reg128Bits<127> {}
impl Reg128BitsDownCast<93> for Reg128Bits<127> {}
impl Reg128BitsDownCast<94> for Reg128Bits<127> {}
impl Reg128BitsDownCast<95> for Reg128Bits<127> {}
impl Reg128BitsDownCast<96> for Reg128Bits<127> {}
impl Reg128BitsDownCast<97> for Reg128Bits<127> {}
impl Reg128BitsDownCast<98> for Reg128Bits<127> {}
impl Reg128BitsDownCast<99> for Reg128Bits<127> {}
impl Reg128BitsDownCast<100> for Reg128Bits<127> {}
impl Reg128BitsDownCast<101> for Reg128Bits<127> {}
impl Reg128BitsDownCast<102> for Reg128Bits<127> {}
impl Reg128BitsDownCast<103> for Reg128Bits<127> {}
impl Reg128BitsDownCast<104> for Reg128Bits<127> {}
impl Reg128BitsDownCast<105> for Reg128Bits<127> {}
impl Reg128BitsDownCast<106> for Reg128Bits<127> {}
impl Reg128BitsDownCast<107> for Reg128Bits<127> {}
impl Reg128BitsDownCast<108> for Reg128Bits<127> {}
impl Reg128BitsDownCast<109> for Reg128Bits<127> {}
impl Reg128BitsDownCast<110> for Reg128Bits<127> {}
impl Reg128BitsDownCast<111> for Reg128Bits<127> {}
impl Reg128BitsDownCast<112> for Reg128Bits<127> {}
impl Reg128BitsDownCast<113> for Reg128Bits<127> {}
impl Reg128BitsDownCast<114> for Reg128Bits<127> {}
impl Reg128BitsDownCast<115> for Reg128Bits<127> {}
impl Reg128BitsDownCast<116> for Reg128Bits<127> {}
impl Reg128BitsDownCast<117> for Reg128Bits<127> {}
impl Reg128BitsDownCast<118> for Reg128Bits<127> {}
impl Reg128BitsDownCast<119> for Reg128Bits<127> {}
impl Reg128BitsDownCast<120> for Reg128Bits<127> {}
impl Reg128BitsDownCast<121> for Reg128Bits<127> {}
impl Reg128BitsDownCast<122> for Reg128Bits<127> {}
impl Reg128BitsDownCast<123> for Reg128Bits<127> {}
impl Reg128BitsDownCast<124> for Reg128Bits<127> {}
impl Reg128BitsDownCast<125> for Reg128Bits<127> {}
impl Reg128BitsDownCast<126> for Reg128Bits<127> {}
impl Reg128BitsDownCast<127> for Reg128Bits<127> {}
impl Reg128BitsDownCast<1> for Reg128Bits<128> {}
impl Reg128BitsDownCast<2> for Reg128Bits<128> {}
impl Reg128BitsDownCast<3> for Reg128Bits<128> {}
impl Reg128BitsDownCast<4> for Reg128Bits<128> {}
impl Reg128BitsDownCast<5> for Reg128Bits<128> {}
impl Reg128BitsDownCast<6> for Reg128Bits<128> {}
impl Reg128BitsDownCast<7> for Reg128Bits<128> {}
impl Reg128BitsDownCast<8> for Reg128Bits<128> {}
impl Reg128BitsDownCast<9> for Reg128Bits<128> {}
impl Reg128BitsDownCast<10> for Reg128Bits<128> {}
impl Reg128BitsDownCast<11> for Reg128Bits<128> {}
impl Reg128BitsDownCast<12> for Reg128Bits<128> {}
impl Reg128BitsDownCast<13> for Reg128Bits<128> {}
impl Reg128BitsDownCast<14> for Reg128Bits<128> {}
impl Reg128BitsDownCast<15> for Reg128Bits<128> {}
impl Reg128BitsDownCast<16> for Reg128Bits<128> {}
impl Reg128BitsDownCast<17> for Reg128Bits<128> {}
impl Reg128BitsDownCast<18> for Reg128Bits<128> {}
impl Reg128BitsDownCast<19> for Reg128Bits<128> {}
impl Reg128BitsDownCast<20> for Reg128Bits<128> {}
impl Reg128BitsDownCast<21> for Reg128Bits<128> {}
impl Reg128BitsDownCast<22> for Reg128Bits<128> {}
impl Reg128BitsDownCast<23> for Reg128Bits<128> {}
impl Reg128BitsDownCast<24> for Reg128Bits<128> {}
impl Reg128BitsDownCast<25> for Reg128Bits<128> {}
impl Reg128BitsDownCast<26> for Reg128Bits<128> {}
impl Reg128BitsDownCast<27> for Reg128Bits<128> {}
impl Reg128BitsDownCast<28> for Reg128Bits<128> {}
impl Reg128BitsDownCast<29> for Reg128Bits<128> {}
impl Reg128BitsDownCast<30> for Reg128Bits<128> {}
impl Reg128BitsDownCast<31> for Reg128Bits<128> {}
impl Reg128BitsDownCast<32> for Reg128Bits<128> {}
impl Reg128BitsDownCast<33> for Reg128Bits<128> {}
impl Reg128BitsDownCast<34> for Reg128Bits<128> {}
impl Reg128BitsDownCast<35> for Reg128Bits<128> {}
impl Reg128BitsDownCast<36> for Reg128Bits<128> {}
impl Reg128BitsDownCast<37> for Reg128Bits<128> {}
impl Reg128BitsDownCast<38> for Reg128Bits<128> {}
impl Reg128BitsDownCast<39> for Reg128Bits<128> {}
impl Reg128BitsDownCast<40> for Reg128Bits<128> {}
impl Reg128BitsDownCast<41> for Reg128Bits<128> {}
impl Reg128BitsDownCast<42> for Reg128Bits<128> {}
impl Reg128BitsDownCast<43> for Reg128Bits<128> {}
impl Reg128BitsDownCast<44> for Reg128Bits<128> {}
impl Reg128BitsDownCast<45> for Reg128Bits<128> {}
impl Reg128BitsDownCast<46> for Reg128Bits<128> {}
impl Reg128BitsDownCast<47> for Reg128Bits<128> {}
impl Reg128BitsDownCast<48> for Reg128Bits<128> {}
impl Reg128BitsDownCast<49> for Reg128Bits<128> {}
impl Reg128BitsDownCast<50> for Reg128Bits<128> {}
impl Reg128BitsDownCast<51> for Reg128Bits<128> {}
impl Reg128BitsDownCast<52> for Reg128Bits<128> {}
impl Reg128BitsDownCast<53> for Reg128Bits<128> {}
impl Reg128BitsDownCast<54> for Reg128Bits<128> {}
impl Reg128BitsDownCast<55> for Reg128Bits<128> {}
impl Reg128BitsDownCast<56> for Reg128Bits<128> {}
impl Reg128BitsDownCast<57> for Reg128Bits<128> {}
impl Reg128BitsDownCast<58> for Reg128Bits<128> {}
impl Reg128BitsDownCast<59> for Reg128Bits<128> {}
impl Reg128BitsDownCast<60> for Reg128Bits<128> {}
impl Reg128BitsDownCast<61> for Reg128Bits<128> {}
impl Reg128BitsDownCast<62> for Reg128Bits<128> {}
impl Reg128BitsDownCast<63> for Reg128Bits<128> {}
impl Reg128BitsDownCast<64> for Reg128Bits<128> {}
impl Reg128BitsDownCast<65> for Reg128Bits<128> {}
impl Reg128BitsDownCast<66> for Reg128Bits<128> {}
impl Reg128BitsDownCast<67> for Reg128Bits<128> {}
impl Reg128BitsDownCast<68> for Reg128Bits<128> {}
impl Reg128BitsDownCast<69> for Reg128Bits<128> {}
impl Reg128BitsDownCast<70> for Reg128Bits<128> {}
impl Reg128BitsDownCast<71> for Reg128Bits<128> {}
impl Reg128BitsDownCast<72> for Reg128Bits<128> {}
impl Reg128BitsDownCast<73> for Reg128Bits<128> {}
impl Reg128BitsDownCast<74> for Reg128Bits<128> {}
impl Reg128BitsDownCast<75> for Reg128Bits<128> {}
impl Reg128BitsDownCast<76> for Reg128Bits<128> {}
impl Reg128BitsDownCast<77> for Reg128Bits<128> {}
impl Reg128BitsDownCast<78> for Reg128Bits<128> {}
impl Reg128BitsDownCast<79> for Reg128Bits<128> {}
impl Reg128BitsDownCast<80> for Reg128Bits<128> {}
impl Reg128BitsDownCast<81> for Reg128Bits<128> {}
impl Reg128BitsDownCast<82> for Reg128Bits<128> {}
impl Reg128BitsDownCast<83> for Reg128Bits<128> {}
impl Reg128BitsDownCast<84> for Reg128Bits<128> {}
impl Reg128BitsDownCast<85> for Reg128Bits<128> {}
impl Reg128BitsDownCast<86> for Reg128Bits<128> {}
impl Reg128BitsDownCast<87> for Reg128Bits<128> {}
impl Reg128BitsDownCast<88> for Reg128Bits<128> {}
impl Reg128BitsDownCast<89> for Reg128Bits<128> {}
impl Reg128BitsDownCast<90> for Reg128Bits<128> {}
impl Reg128BitsDownCast<91> for Reg128Bits<128> {}
impl Reg128BitsDownCast<92> for Reg128Bits<128> {}
impl Reg128BitsDownCast<93> for Reg128Bits<128> {}
impl Reg128BitsDownCast<94> for Reg128Bits<128> {}
impl Reg128BitsDownCast<95> for Reg128Bits<128> {}
impl Reg128BitsDownCast<96> for Reg128Bits<128> {}
impl Reg128BitsDownCast<97> for Reg128Bits<128> {}
impl Reg128BitsDownCast<98> for Reg128Bits<128> {}
impl Reg128BitsDownCast<99> for Reg128Bits<128> {}
impl Reg128BitsDownCast<100> for Reg128Bits<128> {}
impl Reg128BitsDownCast<101> for Reg128Bits<128> {}
impl Reg128BitsDownCast<102> for Reg128Bits<128> {}
impl Reg128BitsDownCast<103> for Reg128Bits<128> {}
impl Reg128BitsDownCast<104> for Reg128Bits<128> {}
impl Reg128BitsDownCast<105> for Reg128Bits<128> {}
impl Reg128BitsDownCast<106> for Reg128Bits<128> {}
impl Reg128BitsDownCast<107> for Reg128Bits<128> {}
impl Reg128BitsDownCast<108> for Reg128Bits<128> {}
impl Reg128BitsDownCast<109> for Reg128Bits<128> {}
impl Reg128BitsDownCast<110> for Reg128Bits<128> {}
impl Reg128BitsDownCast<111> for Reg128Bits<128> {}
impl Reg128BitsDownCast<112> for Reg128Bits<128> {}
impl Reg128BitsDownCast<113> for Reg128Bits<128> {}
impl Reg128BitsDownCast<114> for Reg128Bits<128> {}
impl Reg128BitsDownCast<115> for Reg128Bits<128> {}
impl Reg128BitsDownCast<116> for Reg128Bits<128> {}
impl Reg128BitsDownCast<117> for Reg128Bits<128> {}
impl Reg128BitsDownCast<118> for Reg128Bits<128> {}
impl Reg128BitsDownCast<119> for Reg128Bits<128> {}
impl Reg128BitsDownCast<120> for Reg128Bits<128> {}
impl Reg128BitsDownCast<121> for Reg128Bits<128> {}
impl Reg128BitsDownCast<122> for Reg128Bits<128> {}
impl Reg128BitsDownCast<123> for Reg128Bits<128> {}
impl Reg128BitsDownCast<124> for Reg128Bits<128> {}
impl Reg128BitsDownCast<125> for Reg128Bits<128> {}
impl Reg128BitsDownCast<126> for Reg128Bits<128> {}
impl Reg128BitsDownCast<127> for Reg128Bits<128> {}
impl Reg128BitsDownCast<128> for Reg128Bits<128> {}
