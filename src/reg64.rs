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
type BaseType = u64; // [REF_REPLACE]
const NUM_BITS: usize = BaseType::BITS as usize;

#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
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
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
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
    
    1 << (NUM_BITS - 1)
}

// Function to ease matching of Bits to a specific sequence of bits
impl<const N: usize> Reg64Bits<N> {
    /// N 0's in the base type
    const BASE_ZEROS: BaseType = 0;
    /// N 1's in the base type
    const BASE_ONES: BaseType = truncate(!0, N);

    // This is actually used within the UpCast trait implementation
    #[allow(unused)]
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

impl<const N: usize> core::ops::Add for Reg64Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.0);

        Self(((lhs + rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize> core::ops::Sub for Reg64Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.0);

        Self(((lhs - rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize> core::ops::Div for Reg64Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs / rhs)
    }
}

impl<const N: usize> core::ops::Rem for Reg64Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

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

impl<const N: usize, T> core::ops::Shr<T> for Reg64Bits<N>
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
pub trait Reg64BitsDownCast<const T: usize>: Copy + Into<BaseType> {
    #[inline(always)]
    fn take_low(self) -> Reg64Bits<T> {
        let value: BaseType = self.into();
        Reg64Bits(Reg64Bits::<T>::BASE_ONES & value)
    }
    #[inline(always)]
    fn take_high(self) -> Reg64Bits<T> {
        let value: BaseType = self.into();
        Reg64Bits(value >> (NUM_BITS - T))
    }
}

pub trait Reg64BitsUpCast<const T: usize>: Copy + Into<BaseType> {
    #[inline(always)]
    fn zero_extend(self) -> Reg64Bits<T> {
        let value = self.into();
        Reg64Bits(value)
    }

    fn sign_extend(self) -> Reg64Bits<T> {
        // NOTE: We are assuming here that no Bits<0> structure can exist
        let value = self.into();

        let top_bit = value & Reg64Bits::<T>::TOP_BIT_MASK; // Capture only the top bit
        let top_bits = if top_bit == 0 { // Create a set of NUM_BITS-N bits of with the given sign
            0
        } else {
            !Reg64Bits::<T>::BASE_ONES // !001111 = 110000
        };

        Reg64Bits(top_bits & value)
    }
}

pub trait Reg64BitsConcat<const R: usize, const O: usize>: Copy + Into<BaseType> {
    fn concat(self, rhs: Reg64Bits<R>) -> Reg64Bits<O> {
        let lhs = self.into();
        let rhs: BaseType = rhs.into();

        Reg64Bits((lhs << R) | rhs)
    }
}

impl<const F: usize, const T: usize> Reg64BitsUpCast<T> for Reg64Bits<F>
where
    Reg64Bits<T>: Reg64BitsDownCast<F>,
{
}



impl Reg64BitsDownCast<1> for Reg64Bits<1> {}
impl Reg64BitsConcat<1, 2> for Reg64Bits<1> {}
impl Reg64BitsDownCast<1> for Reg64Bits<2> {}
impl Reg64BitsConcat<1, 3> for Reg64Bits<2> {}
impl Reg64BitsDownCast<2> for Reg64Bits<2> {}
impl Reg64BitsConcat<2, 4> for Reg64Bits<2> {}
impl Reg64BitsDownCast<1> for Reg64Bits<3> {}
impl Reg64BitsConcat<1, 4> for Reg64Bits<3> {}
impl Reg64BitsDownCast<2> for Reg64Bits<3> {}
impl Reg64BitsConcat<2, 5> for Reg64Bits<3> {}
impl Reg64BitsDownCast<3> for Reg64Bits<3> {}
impl Reg64BitsConcat<3, 6> for Reg64Bits<3> {}
impl Reg64BitsDownCast<1> for Reg64Bits<4> {}
impl Reg64BitsConcat<1, 5> for Reg64Bits<4> {}
impl Reg64BitsDownCast<2> for Reg64Bits<4> {}
impl Reg64BitsConcat<2, 6> for Reg64Bits<4> {}
impl Reg64BitsDownCast<3> for Reg64Bits<4> {}
impl Reg64BitsConcat<3, 7> for Reg64Bits<4> {}
impl Reg64BitsDownCast<4> for Reg64Bits<4> {}
impl Reg64BitsConcat<4, 8> for Reg64Bits<4> {}
impl Reg64BitsDownCast<1> for Reg64Bits<5> {}
impl Reg64BitsConcat<1, 6> for Reg64Bits<5> {}
impl Reg64BitsDownCast<2> for Reg64Bits<5> {}
impl Reg64BitsConcat<2, 7> for Reg64Bits<5> {}
impl Reg64BitsDownCast<3> for Reg64Bits<5> {}
impl Reg64BitsConcat<3, 8> for Reg64Bits<5> {}
impl Reg64BitsDownCast<4> for Reg64Bits<5> {}
impl Reg64BitsConcat<4, 9> for Reg64Bits<5> {}
impl Reg64BitsDownCast<5> for Reg64Bits<5> {}
impl Reg64BitsConcat<5, 10> for Reg64Bits<5> {}
impl Reg64BitsDownCast<1> for Reg64Bits<6> {}
impl Reg64BitsConcat<1, 7> for Reg64Bits<6> {}
impl Reg64BitsDownCast<2> for Reg64Bits<6> {}
impl Reg64BitsConcat<2, 8> for Reg64Bits<6> {}
impl Reg64BitsDownCast<3> for Reg64Bits<6> {}
impl Reg64BitsConcat<3, 9> for Reg64Bits<6> {}
impl Reg64BitsDownCast<4> for Reg64Bits<6> {}
impl Reg64BitsConcat<4, 10> for Reg64Bits<6> {}
impl Reg64BitsDownCast<5> for Reg64Bits<6> {}
impl Reg64BitsConcat<5, 11> for Reg64Bits<6> {}
impl Reg64BitsDownCast<6> for Reg64Bits<6> {}
impl Reg64BitsConcat<6, 12> for Reg64Bits<6> {}
impl Reg64BitsDownCast<1> for Reg64Bits<7> {}
impl Reg64BitsConcat<1, 8> for Reg64Bits<7> {}
impl Reg64BitsDownCast<2> for Reg64Bits<7> {}
impl Reg64BitsConcat<2, 9> for Reg64Bits<7> {}
impl Reg64BitsDownCast<3> for Reg64Bits<7> {}
impl Reg64BitsConcat<3, 10> for Reg64Bits<7> {}
impl Reg64BitsDownCast<4> for Reg64Bits<7> {}
impl Reg64BitsConcat<4, 11> for Reg64Bits<7> {}
impl Reg64BitsDownCast<5> for Reg64Bits<7> {}
impl Reg64BitsConcat<5, 12> for Reg64Bits<7> {}
impl Reg64BitsDownCast<6> for Reg64Bits<7> {}
impl Reg64BitsConcat<6, 13> for Reg64Bits<7> {}
impl Reg64BitsDownCast<7> for Reg64Bits<7> {}
impl Reg64BitsConcat<7, 14> for Reg64Bits<7> {}
impl Reg64BitsDownCast<1> for Reg64Bits<8> {}
impl Reg64BitsConcat<1, 9> for Reg64Bits<8> {}
impl Reg64BitsDownCast<2> for Reg64Bits<8> {}
impl Reg64BitsConcat<2, 10> for Reg64Bits<8> {}
impl Reg64BitsDownCast<3> for Reg64Bits<8> {}
impl Reg64BitsConcat<3, 11> for Reg64Bits<8> {}
impl Reg64BitsDownCast<4> for Reg64Bits<8> {}
impl Reg64BitsConcat<4, 12> for Reg64Bits<8> {}
impl Reg64BitsDownCast<5> for Reg64Bits<8> {}
impl Reg64BitsConcat<5, 13> for Reg64Bits<8> {}
impl Reg64BitsDownCast<6> for Reg64Bits<8> {}
impl Reg64BitsConcat<6, 14> for Reg64Bits<8> {}
impl Reg64BitsDownCast<7> for Reg64Bits<8> {}
impl Reg64BitsConcat<7, 15> for Reg64Bits<8> {}
impl Reg64BitsDownCast<8> for Reg64Bits<8> {}
impl Reg64BitsConcat<8, 16> for Reg64Bits<8> {}
impl Reg64BitsDownCast<1> for Reg64Bits<9> {}
impl Reg64BitsConcat<1, 10> for Reg64Bits<9> {}
impl Reg64BitsDownCast<2> for Reg64Bits<9> {}
impl Reg64BitsConcat<2, 11> for Reg64Bits<9> {}
impl Reg64BitsDownCast<3> for Reg64Bits<9> {}
impl Reg64BitsConcat<3, 12> for Reg64Bits<9> {}
impl Reg64BitsDownCast<4> for Reg64Bits<9> {}
impl Reg64BitsConcat<4, 13> for Reg64Bits<9> {}
impl Reg64BitsDownCast<5> for Reg64Bits<9> {}
impl Reg64BitsConcat<5, 14> for Reg64Bits<9> {}
impl Reg64BitsDownCast<6> for Reg64Bits<9> {}
impl Reg64BitsConcat<6, 15> for Reg64Bits<9> {}
impl Reg64BitsDownCast<7> for Reg64Bits<9> {}
impl Reg64BitsConcat<7, 16> for Reg64Bits<9> {}
impl Reg64BitsDownCast<8> for Reg64Bits<9> {}
impl Reg64BitsConcat<8, 17> for Reg64Bits<9> {}
impl Reg64BitsDownCast<9> for Reg64Bits<9> {}
impl Reg64BitsConcat<9, 18> for Reg64Bits<9> {}
impl Reg64BitsDownCast<1> for Reg64Bits<10> {}
impl Reg64BitsConcat<1, 11> for Reg64Bits<10> {}
impl Reg64BitsDownCast<2> for Reg64Bits<10> {}
impl Reg64BitsConcat<2, 12> for Reg64Bits<10> {}
impl Reg64BitsDownCast<3> for Reg64Bits<10> {}
impl Reg64BitsConcat<3, 13> for Reg64Bits<10> {}
impl Reg64BitsDownCast<4> for Reg64Bits<10> {}
impl Reg64BitsConcat<4, 14> for Reg64Bits<10> {}
impl Reg64BitsDownCast<5> for Reg64Bits<10> {}
impl Reg64BitsConcat<5, 15> for Reg64Bits<10> {}
impl Reg64BitsDownCast<6> for Reg64Bits<10> {}
impl Reg64BitsConcat<6, 16> for Reg64Bits<10> {}
impl Reg64BitsDownCast<7> for Reg64Bits<10> {}
impl Reg64BitsConcat<7, 17> for Reg64Bits<10> {}
impl Reg64BitsDownCast<8> for Reg64Bits<10> {}
impl Reg64BitsConcat<8, 18> for Reg64Bits<10> {}
impl Reg64BitsDownCast<9> for Reg64Bits<10> {}
impl Reg64BitsConcat<9, 19> for Reg64Bits<10> {}
impl Reg64BitsDownCast<10> for Reg64Bits<10> {}
impl Reg64BitsConcat<10, 20> for Reg64Bits<10> {}
impl Reg64BitsDownCast<1> for Reg64Bits<11> {}
impl Reg64BitsConcat<1, 12> for Reg64Bits<11> {}
impl Reg64BitsDownCast<2> for Reg64Bits<11> {}
impl Reg64BitsConcat<2, 13> for Reg64Bits<11> {}
impl Reg64BitsDownCast<3> for Reg64Bits<11> {}
impl Reg64BitsConcat<3, 14> for Reg64Bits<11> {}
impl Reg64BitsDownCast<4> for Reg64Bits<11> {}
impl Reg64BitsConcat<4, 15> for Reg64Bits<11> {}
impl Reg64BitsDownCast<5> for Reg64Bits<11> {}
impl Reg64BitsConcat<5, 16> for Reg64Bits<11> {}
impl Reg64BitsDownCast<6> for Reg64Bits<11> {}
impl Reg64BitsConcat<6, 17> for Reg64Bits<11> {}
impl Reg64BitsDownCast<7> for Reg64Bits<11> {}
impl Reg64BitsConcat<7, 18> for Reg64Bits<11> {}
impl Reg64BitsDownCast<8> for Reg64Bits<11> {}
impl Reg64BitsConcat<8, 19> for Reg64Bits<11> {}
impl Reg64BitsDownCast<9> for Reg64Bits<11> {}
impl Reg64BitsConcat<9, 20> for Reg64Bits<11> {}
impl Reg64BitsDownCast<10> for Reg64Bits<11> {}
impl Reg64BitsConcat<10, 21> for Reg64Bits<11> {}
impl Reg64BitsDownCast<11> for Reg64Bits<11> {}
impl Reg64BitsConcat<11, 22> for Reg64Bits<11> {}
impl Reg64BitsDownCast<1> for Reg64Bits<12> {}
impl Reg64BitsConcat<1, 13> for Reg64Bits<12> {}
impl Reg64BitsDownCast<2> for Reg64Bits<12> {}
impl Reg64BitsConcat<2, 14> for Reg64Bits<12> {}
impl Reg64BitsDownCast<3> for Reg64Bits<12> {}
impl Reg64BitsConcat<3, 15> for Reg64Bits<12> {}
impl Reg64BitsDownCast<4> for Reg64Bits<12> {}
impl Reg64BitsConcat<4, 16> for Reg64Bits<12> {}
impl Reg64BitsDownCast<5> for Reg64Bits<12> {}
impl Reg64BitsConcat<5, 17> for Reg64Bits<12> {}
impl Reg64BitsDownCast<6> for Reg64Bits<12> {}
impl Reg64BitsConcat<6, 18> for Reg64Bits<12> {}
impl Reg64BitsDownCast<7> for Reg64Bits<12> {}
impl Reg64BitsConcat<7, 19> for Reg64Bits<12> {}
impl Reg64BitsDownCast<8> for Reg64Bits<12> {}
impl Reg64BitsConcat<8, 20> for Reg64Bits<12> {}
impl Reg64BitsDownCast<9> for Reg64Bits<12> {}
impl Reg64BitsConcat<9, 21> for Reg64Bits<12> {}
impl Reg64BitsDownCast<10> for Reg64Bits<12> {}
impl Reg64BitsConcat<10, 22> for Reg64Bits<12> {}
impl Reg64BitsDownCast<11> for Reg64Bits<12> {}
impl Reg64BitsConcat<11, 23> for Reg64Bits<12> {}
impl Reg64BitsDownCast<12> for Reg64Bits<12> {}
impl Reg64BitsConcat<12, 24> for Reg64Bits<12> {}
impl Reg64BitsDownCast<1> for Reg64Bits<13> {}
impl Reg64BitsConcat<1, 14> for Reg64Bits<13> {}
impl Reg64BitsDownCast<2> for Reg64Bits<13> {}
impl Reg64BitsConcat<2, 15> for Reg64Bits<13> {}
impl Reg64BitsDownCast<3> for Reg64Bits<13> {}
impl Reg64BitsConcat<3, 16> for Reg64Bits<13> {}
impl Reg64BitsDownCast<4> for Reg64Bits<13> {}
impl Reg64BitsConcat<4, 17> for Reg64Bits<13> {}
impl Reg64BitsDownCast<5> for Reg64Bits<13> {}
impl Reg64BitsConcat<5, 18> for Reg64Bits<13> {}
impl Reg64BitsDownCast<6> for Reg64Bits<13> {}
impl Reg64BitsConcat<6, 19> for Reg64Bits<13> {}
impl Reg64BitsDownCast<7> for Reg64Bits<13> {}
impl Reg64BitsConcat<7, 20> for Reg64Bits<13> {}
impl Reg64BitsDownCast<8> for Reg64Bits<13> {}
impl Reg64BitsConcat<8, 21> for Reg64Bits<13> {}
impl Reg64BitsDownCast<9> for Reg64Bits<13> {}
impl Reg64BitsConcat<9, 22> for Reg64Bits<13> {}
impl Reg64BitsDownCast<10> for Reg64Bits<13> {}
impl Reg64BitsConcat<10, 23> for Reg64Bits<13> {}
impl Reg64BitsDownCast<11> for Reg64Bits<13> {}
impl Reg64BitsConcat<11, 24> for Reg64Bits<13> {}
impl Reg64BitsDownCast<12> for Reg64Bits<13> {}
impl Reg64BitsConcat<12, 25> for Reg64Bits<13> {}
impl Reg64BitsDownCast<13> for Reg64Bits<13> {}
impl Reg64BitsConcat<13, 26> for Reg64Bits<13> {}
impl Reg64BitsDownCast<1> for Reg64Bits<14> {}
impl Reg64BitsConcat<1, 15> for Reg64Bits<14> {}
impl Reg64BitsDownCast<2> for Reg64Bits<14> {}
impl Reg64BitsConcat<2, 16> for Reg64Bits<14> {}
impl Reg64BitsDownCast<3> for Reg64Bits<14> {}
impl Reg64BitsConcat<3, 17> for Reg64Bits<14> {}
impl Reg64BitsDownCast<4> for Reg64Bits<14> {}
impl Reg64BitsConcat<4, 18> for Reg64Bits<14> {}
impl Reg64BitsDownCast<5> for Reg64Bits<14> {}
impl Reg64BitsConcat<5, 19> for Reg64Bits<14> {}
impl Reg64BitsDownCast<6> for Reg64Bits<14> {}
impl Reg64BitsConcat<6, 20> for Reg64Bits<14> {}
impl Reg64BitsDownCast<7> for Reg64Bits<14> {}
impl Reg64BitsConcat<7, 21> for Reg64Bits<14> {}
impl Reg64BitsDownCast<8> for Reg64Bits<14> {}
impl Reg64BitsConcat<8, 22> for Reg64Bits<14> {}
impl Reg64BitsDownCast<9> for Reg64Bits<14> {}
impl Reg64BitsConcat<9, 23> for Reg64Bits<14> {}
impl Reg64BitsDownCast<10> for Reg64Bits<14> {}
impl Reg64BitsConcat<10, 24> for Reg64Bits<14> {}
impl Reg64BitsDownCast<11> for Reg64Bits<14> {}
impl Reg64BitsConcat<11, 25> for Reg64Bits<14> {}
impl Reg64BitsDownCast<12> for Reg64Bits<14> {}
impl Reg64BitsConcat<12, 26> for Reg64Bits<14> {}
impl Reg64BitsDownCast<13> for Reg64Bits<14> {}
impl Reg64BitsConcat<13, 27> for Reg64Bits<14> {}
impl Reg64BitsDownCast<14> for Reg64Bits<14> {}
impl Reg64BitsConcat<14, 28> for Reg64Bits<14> {}
impl Reg64BitsDownCast<1> for Reg64Bits<15> {}
impl Reg64BitsConcat<1, 16> for Reg64Bits<15> {}
impl Reg64BitsDownCast<2> for Reg64Bits<15> {}
impl Reg64BitsConcat<2, 17> for Reg64Bits<15> {}
impl Reg64BitsDownCast<3> for Reg64Bits<15> {}
impl Reg64BitsConcat<3, 18> for Reg64Bits<15> {}
impl Reg64BitsDownCast<4> for Reg64Bits<15> {}
impl Reg64BitsConcat<4, 19> for Reg64Bits<15> {}
impl Reg64BitsDownCast<5> for Reg64Bits<15> {}
impl Reg64BitsConcat<5, 20> for Reg64Bits<15> {}
impl Reg64BitsDownCast<6> for Reg64Bits<15> {}
impl Reg64BitsConcat<6, 21> for Reg64Bits<15> {}
impl Reg64BitsDownCast<7> for Reg64Bits<15> {}
impl Reg64BitsConcat<7, 22> for Reg64Bits<15> {}
impl Reg64BitsDownCast<8> for Reg64Bits<15> {}
impl Reg64BitsConcat<8, 23> for Reg64Bits<15> {}
impl Reg64BitsDownCast<9> for Reg64Bits<15> {}
impl Reg64BitsConcat<9, 24> for Reg64Bits<15> {}
impl Reg64BitsDownCast<10> for Reg64Bits<15> {}
impl Reg64BitsConcat<10, 25> for Reg64Bits<15> {}
impl Reg64BitsDownCast<11> for Reg64Bits<15> {}
impl Reg64BitsConcat<11, 26> for Reg64Bits<15> {}
impl Reg64BitsDownCast<12> for Reg64Bits<15> {}
impl Reg64BitsConcat<12, 27> for Reg64Bits<15> {}
impl Reg64BitsDownCast<13> for Reg64Bits<15> {}
impl Reg64BitsConcat<13, 28> for Reg64Bits<15> {}
impl Reg64BitsDownCast<14> for Reg64Bits<15> {}
impl Reg64BitsConcat<14, 29> for Reg64Bits<15> {}
impl Reg64BitsDownCast<15> for Reg64Bits<15> {}
impl Reg64BitsConcat<15, 30> for Reg64Bits<15> {}
impl Reg64BitsDownCast<1> for Reg64Bits<16> {}
impl Reg64BitsConcat<1, 17> for Reg64Bits<16> {}
impl Reg64BitsDownCast<2> for Reg64Bits<16> {}
impl Reg64BitsConcat<2, 18> for Reg64Bits<16> {}
impl Reg64BitsDownCast<3> for Reg64Bits<16> {}
impl Reg64BitsConcat<3, 19> for Reg64Bits<16> {}
impl Reg64BitsDownCast<4> for Reg64Bits<16> {}
impl Reg64BitsConcat<4, 20> for Reg64Bits<16> {}
impl Reg64BitsDownCast<5> for Reg64Bits<16> {}
impl Reg64BitsConcat<5, 21> for Reg64Bits<16> {}
impl Reg64BitsDownCast<6> for Reg64Bits<16> {}
impl Reg64BitsConcat<6, 22> for Reg64Bits<16> {}
impl Reg64BitsDownCast<7> for Reg64Bits<16> {}
impl Reg64BitsConcat<7, 23> for Reg64Bits<16> {}
impl Reg64BitsDownCast<8> for Reg64Bits<16> {}
impl Reg64BitsConcat<8, 24> for Reg64Bits<16> {}
impl Reg64BitsDownCast<9> for Reg64Bits<16> {}
impl Reg64BitsConcat<9, 25> for Reg64Bits<16> {}
impl Reg64BitsDownCast<10> for Reg64Bits<16> {}
impl Reg64BitsConcat<10, 26> for Reg64Bits<16> {}
impl Reg64BitsDownCast<11> for Reg64Bits<16> {}
impl Reg64BitsConcat<11, 27> for Reg64Bits<16> {}
impl Reg64BitsDownCast<12> for Reg64Bits<16> {}
impl Reg64BitsConcat<12, 28> for Reg64Bits<16> {}
impl Reg64BitsDownCast<13> for Reg64Bits<16> {}
impl Reg64BitsConcat<13, 29> for Reg64Bits<16> {}
impl Reg64BitsDownCast<14> for Reg64Bits<16> {}
impl Reg64BitsConcat<14, 30> for Reg64Bits<16> {}
impl Reg64BitsDownCast<15> for Reg64Bits<16> {}
impl Reg64BitsConcat<15, 31> for Reg64Bits<16> {}
impl Reg64BitsDownCast<16> for Reg64Bits<16> {}
impl Reg64BitsConcat<16, 32> for Reg64Bits<16> {}
impl Reg64BitsDownCast<1> for Reg64Bits<17> {}
impl Reg64BitsConcat<1, 18> for Reg64Bits<17> {}
impl Reg64BitsDownCast<2> for Reg64Bits<17> {}
impl Reg64BitsConcat<2, 19> for Reg64Bits<17> {}
impl Reg64BitsDownCast<3> for Reg64Bits<17> {}
impl Reg64BitsConcat<3, 20> for Reg64Bits<17> {}
impl Reg64BitsDownCast<4> for Reg64Bits<17> {}
impl Reg64BitsConcat<4, 21> for Reg64Bits<17> {}
impl Reg64BitsDownCast<5> for Reg64Bits<17> {}
impl Reg64BitsConcat<5, 22> for Reg64Bits<17> {}
impl Reg64BitsDownCast<6> for Reg64Bits<17> {}
impl Reg64BitsConcat<6, 23> for Reg64Bits<17> {}
impl Reg64BitsDownCast<7> for Reg64Bits<17> {}
impl Reg64BitsConcat<7, 24> for Reg64Bits<17> {}
impl Reg64BitsDownCast<8> for Reg64Bits<17> {}
impl Reg64BitsConcat<8, 25> for Reg64Bits<17> {}
impl Reg64BitsDownCast<9> for Reg64Bits<17> {}
impl Reg64BitsConcat<9, 26> for Reg64Bits<17> {}
impl Reg64BitsDownCast<10> for Reg64Bits<17> {}
impl Reg64BitsConcat<10, 27> for Reg64Bits<17> {}
impl Reg64BitsDownCast<11> for Reg64Bits<17> {}
impl Reg64BitsConcat<11, 28> for Reg64Bits<17> {}
impl Reg64BitsDownCast<12> for Reg64Bits<17> {}
impl Reg64BitsConcat<12, 29> for Reg64Bits<17> {}
impl Reg64BitsDownCast<13> for Reg64Bits<17> {}
impl Reg64BitsConcat<13, 30> for Reg64Bits<17> {}
impl Reg64BitsDownCast<14> for Reg64Bits<17> {}
impl Reg64BitsConcat<14, 31> for Reg64Bits<17> {}
impl Reg64BitsDownCast<15> for Reg64Bits<17> {}
impl Reg64BitsConcat<15, 32> for Reg64Bits<17> {}
impl Reg64BitsDownCast<16> for Reg64Bits<17> {}
impl Reg64BitsConcat<16, 33> for Reg64Bits<17> {}
impl Reg64BitsDownCast<17> for Reg64Bits<17> {}
impl Reg64BitsConcat<17, 34> for Reg64Bits<17> {}
impl Reg64BitsDownCast<1> for Reg64Bits<18> {}
impl Reg64BitsConcat<1, 19> for Reg64Bits<18> {}
impl Reg64BitsDownCast<2> for Reg64Bits<18> {}
impl Reg64BitsConcat<2, 20> for Reg64Bits<18> {}
impl Reg64BitsDownCast<3> for Reg64Bits<18> {}
impl Reg64BitsConcat<3, 21> for Reg64Bits<18> {}
impl Reg64BitsDownCast<4> for Reg64Bits<18> {}
impl Reg64BitsConcat<4, 22> for Reg64Bits<18> {}
impl Reg64BitsDownCast<5> for Reg64Bits<18> {}
impl Reg64BitsConcat<5, 23> for Reg64Bits<18> {}
impl Reg64BitsDownCast<6> for Reg64Bits<18> {}
impl Reg64BitsConcat<6, 24> for Reg64Bits<18> {}
impl Reg64BitsDownCast<7> for Reg64Bits<18> {}
impl Reg64BitsConcat<7, 25> for Reg64Bits<18> {}
impl Reg64BitsDownCast<8> for Reg64Bits<18> {}
impl Reg64BitsConcat<8, 26> for Reg64Bits<18> {}
impl Reg64BitsDownCast<9> for Reg64Bits<18> {}
impl Reg64BitsConcat<9, 27> for Reg64Bits<18> {}
impl Reg64BitsDownCast<10> for Reg64Bits<18> {}
impl Reg64BitsConcat<10, 28> for Reg64Bits<18> {}
impl Reg64BitsDownCast<11> for Reg64Bits<18> {}
impl Reg64BitsConcat<11, 29> for Reg64Bits<18> {}
impl Reg64BitsDownCast<12> for Reg64Bits<18> {}
impl Reg64BitsConcat<12, 30> for Reg64Bits<18> {}
impl Reg64BitsDownCast<13> for Reg64Bits<18> {}
impl Reg64BitsConcat<13, 31> for Reg64Bits<18> {}
impl Reg64BitsDownCast<14> for Reg64Bits<18> {}
impl Reg64BitsConcat<14, 32> for Reg64Bits<18> {}
impl Reg64BitsDownCast<15> for Reg64Bits<18> {}
impl Reg64BitsConcat<15, 33> for Reg64Bits<18> {}
impl Reg64BitsDownCast<16> for Reg64Bits<18> {}
impl Reg64BitsConcat<16, 34> for Reg64Bits<18> {}
impl Reg64BitsDownCast<17> for Reg64Bits<18> {}
impl Reg64BitsConcat<17, 35> for Reg64Bits<18> {}
impl Reg64BitsDownCast<18> for Reg64Bits<18> {}
impl Reg64BitsConcat<18, 36> for Reg64Bits<18> {}
impl Reg64BitsDownCast<1> for Reg64Bits<19> {}
impl Reg64BitsConcat<1, 20> for Reg64Bits<19> {}
impl Reg64BitsDownCast<2> for Reg64Bits<19> {}
impl Reg64BitsConcat<2, 21> for Reg64Bits<19> {}
impl Reg64BitsDownCast<3> for Reg64Bits<19> {}
impl Reg64BitsConcat<3, 22> for Reg64Bits<19> {}
impl Reg64BitsDownCast<4> for Reg64Bits<19> {}
impl Reg64BitsConcat<4, 23> for Reg64Bits<19> {}
impl Reg64BitsDownCast<5> for Reg64Bits<19> {}
impl Reg64BitsConcat<5, 24> for Reg64Bits<19> {}
impl Reg64BitsDownCast<6> for Reg64Bits<19> {}
impl Reg64BitsConcat<6, 25> for Reg64Bits<19> {}
impl Reg64BitsDownCast<7> for Reg64Bits<19> {}
impl Reg64BitsConcat<7, 26> for Reg64Bits<19> {}
impl Reg64BitsDownCast<8> for Reg64Bits<19> {}
impl Reg64BitsConcat<8, 27> for Reg64Bits<19> {}
impl Reg64BitsDownCast<9> for Reg64Bits<19> {}
impl Reg64BitsConcat<9, 28> for Reg64Bits<19> {}
impl Reg64BitsDownCast<10> for Reg64Bits<19> {}
impl Reg64BitsConcat<10, 29> for Reg64Bits<19> {}
impl Reg64BitsDownCast<11> for Reg64Bits<19> {}
impl Reg64BitsConcat<11, 30> for Reg64Bits<19> {}
impl Reg64BitsDownCast<12> for Reg64Bits<19> {}
impl Reg64BitsConcat<12, 31> for Reg64Bits<19> {}
impl Reg64BitsDownCast<13> for Reg64Bits<19> {}
impl Reg64BitsConcat<13, 32> for Reg64Bits<19> {}
impl Reg64BitsDownCast<14> for Reg64Bits<19> {}
impl Reg64BitsConcat<14, 33> for Reg64Bits<19> {}
impl Reg64BitsDownCast<15> for Reg64Bits<19> {}
impl Reg64BitsConcat<15, 34> for Reg64Bits<19> {}
impl Reg64BitsDownCast<16> for Reg64Bits<19> {}
impl Reg64BitsConcat<16, 35> for Reg64Bits<19> {}
impl Reg64BitsDownCast<17> for Reg64Bits<19> {}
impl Reg64BitsConcat<17, 36> for Reg64Bits<19> {}
impl Reg64BitsDownCast<18> for Reg64Bits<19> {}
impl Reg64BitsConcat<18, 37> for Reg64Bits<19> {}
impl Reg64BitsDownCast<19> for Reg64Bits<19> {}
impl Reg64BitsConcat<19, 38> for Reg64Bits<19> {}
impl Reg64BitsDownCast<1> for Reg64Bits<20> {}
impl Reg64BitsConcat<1, 21> for Reg64Bits<20> {}
impl Reg64BitsDownCast<2> for Reg64Bits<20> {}
impl Reg64BitsConcat<2, 22> for Reg64Bits<20> {}
impl Reg64BitsDownCast<3> for Reg64Bits<20> {}
impl Reg64BitsConcat<3, 23> for Reg64Bits<20> {}
impl Reg64BitsDownCast<4> for Reg64Bits<20> {}
impl Reg64BitsConcat<4, 24> for Reg64Bits<20> {}
impl Reg64BitsDownCast<5> for Reg64Bits<20> {}
impl Reg64BitsConcat<5, 25> for Reg64Bits<20> {}
impl Reg64BitsDownCast<6> for Reg64Bits<20> {}
impl Reg64BitsConcat<6, 26> for Reg64Bits<20> {}
impl Reg64BitsDownCast<7> for Reg64Bits<20> {}
impl Reg64BitsConcat<7, 27> for Reg64Bits<20> {}
impl Reg64BitsDownCast<8> for Reg64Bits<20> {}
impl Reg64BitsConcat<8, 28> for Reg64Bits<20> {}
impl Reg64BitsDownCast<9> for Reg64Bits<20> {}
impl Reg64BitsConcat<9, 29> for Reg64Bits<20> {}
impl Reg64BitsDownCast<10> for Reg64Bits<20> {}
impl Reg64BitsConcat<10, 30> for Reg64Bits<20> {}
impl Reg64BitsDownCast<11> for Reg64Bits<20> {}
impl Reg64BitsConcat<11, 31> for Reg64Bits<20> {}
impl Reg64BitsDownCast<12> for Reg64Bits<20> {}
impl Reg64BitsConcat<12, 32> for Reg64Bits<20> {}
impl Reg64BitsDownCast<13> for Reg64Bits<20> {}
impl Reg64BitsConcat<13, 33> for Reg64Bits<20> {}
impl Reg64BitsDownCast<14> for Reg64Bits<20> {}
impl Reg64BitsConcat<14, 34> for Reg64Bits<20> {}
impl Reg64BitsDownCast<15> for Reg64Bits<20> {}
impl Reg64BitsConcat<15, 35> for Reg64Bits<20> {}
impl Reg64BitsDownCast<16> for Reg64Bits<20> {}
impl Reg64BitsConcat<16, 36> for Reg64Bits<20> {}
impl Reg64BitsDownCast<17> for Reg64Bits<20> {}
impl Reg64BitsConcat<17, 37> for Reg64Bits<20> {}
impl Reg64BitsDownCast<18> for Reg64Bits<20> {}
impl Reg64BitsConcat<18, 38> for Reg64Bits<20> {}
impl Reg64BitsDownCast<19> for Reg64Bits<20> {}
impl Reg64BitsConcat<19, 39> for Reg64Bits<20> {}
impl Reg64BitsDownCast<20> for Reg64Bits<20> {}
impl Reg64BitsConcat<20, 40> for Reg64Bits<20> {}
impl Reg64BitsDownCast<1> for Reg64Bits<21> {}
impl Reg64BitsConcat<1, 22> for Reg64Bits<21> {}
impl Reg64BitsDownCast<2> for Reg64Bits<21> {}
impl Reg64BitsConcat<2, 23> for Reg64Bits<21> {}
impl Reg64BitsDownCast<3> for Reg64Bits<21> {}
impl Reg64BitsConcat<3, 24> for Reg64Bits<21> {}
impl Reg64BitsDownCast<4> for Reg64Bits<21> {}
impl Reg64BitsConcat<4, 25> for Reg64Bits<21> {}
impl Reg64BitsDownCast<5> for Reg64Bits<21> {}
impl Reg64BitsConcat<5, 26> for Reg64Bits<21> {}
impl Reg64BitsDownCast<6> for Reg64Bits<21> {}
impl Reg64BitsConcat<6, 27> for Reg64Bits<21> {}
impl Reg64BitsDownCast<7> for Reg64Bits<21> {}
impl Reg64BitsConcat<7, 28> for Reg64Bits<21> {}
impl Reg64BitsDownCast<8> for Reg64Bits<21> {}
impl Reg64BitsConcat<8, 29> for Reg64Bits<21> {}
impl Reg64BitsDownCast<9> for Reg64Bits<21> {}
impl Reg64BitsConcat<9, 30> for Reg64Bits<21> {}
impl Reg64BitsDownCast<10> for Reg64Bits<21> {}
impl Reg64BitsConcat<10, 31> for Reg64Bits<21> {}
impl Reg64BitsDownCast<11> for Reg64Bits<21> {}
impl Reg64BitsConcat<11, 32> for Reg64Bits<21> {}
impl Reg64BitsDownCast<12> for Reg64Bits<21> {}
impl Reg64BitsConcat<12, 33> for Reg64Bits<21> {}
impl Reg64BitsDownCast<13> for Reg64Bits<21> {}
impl Reg64BitsConcat<13, 34> for Reg64Bits<21> {}
impl Reg64BitsDownCast<14> for Reg64Bits<21> {}
impl Reg64BitsConcat<14, 35> for Reg64Bits<21> {}
impl Reg64BitsDownCast<15> for Reg64Bits<21> {}
impl Reg64BitsConcat<15, 36> for Reg64Bits<21> {}
impl Reg64BitsDownCast<16> for Reg64Bits<21> {}
impl Reg64BitsConcat<16, 37> for Reg64Bits<21> {}
impl Reg64BitsDownCast<17> for Reg64Bits<21> {}
impl Reg64BitsConcat<17, 38> for Reg64Bits<21> {}
impl Reg64BitsDownCast<18> for Reg64Bits<21> {}
impl Reg64BitsConcat<18, 39> for Reg64Bits<21> {}
impl Reg64BitsDownCast<19> for Reg64Bits<21> {}
impl Reg64BitsConcat<19, 40> for Reg64Bits<21> {}
impl Reg64BitsDownCast<20> for Reg64Bits<21> {}
impl Reg64BitsConcat<20, 41> for Reg64Bits<21> {}
impl Reg64BitsDownCast<21> for Reg64Bits<21> {}
impl Reg64BitsConcat<21, 42> for Reg64Bits<21> {}
impl Reg64BitsDownCast<1> for Reg64Bits<22> {}
impl Reg64BitsConcat<1, 23> for Reg64Bits<22> {}
impl Reg64BitsDownCast<2> for Reg64Bits<22> {}
impl Reg64BitsConcat<2, 24> for Reg64Bits<22> {}
impl Reg64BitsDownCast<3> for Reg64Bits<22> {}
impl Reg64BitsConcat<3, 25> for Reg64Bits<22> {}
impl Reg64BitsDownCast<4> for Reg64Bits<22> {}
impl Reg64BitsConcat<4, 26> for Reg64Bits<22> {}
impl Reg64BitsDownCast<5> for Reg64Bits<22> {}
impl Reg64BitsConcat<5, 27> for Reg64Bits<22> {}
impl Reg64BitsDownCast<6> for Reg64Bits<22> {}
impl Reg64BitsConcat<6, 28> for Reg64Bits<22> {}
impl Reg64BitsDownCast<7> for Reg64Bits<22> {}
impl Reg64BitsConcat<7, 29> for Reg64Bits<22> {}
impl Reg64BitsDownCast<8> for Reg64Bits<22> {}
impl Reg64BitsConcat<8, 30> for Reg64Bits<22> {}
impl Reg64BitsDownCast<9> for Reg64Bits<22> {}
impl Reg64BitsConcat<9, 31> for Reg64Bits<22> {}
impl Reg64BitsDownCast<10> for Reg64Bits<22> {}
impl Reg64BitsConcat<10, 32> for Reg64Bits<22> {}
impl Reg64BitsDownCast<11> for Reg64Bits<22> {}
impl Reg64BitsConcat<11, 33> for Reg64Bits<22> {}
impl Reg64BitsDownCast<12> for Reg64Bits<22> {}
impl Reg64BitsConcat<12, 34> for Reg64Bits<22> {}
impl Reg64BitsDownCast<13> for Reg64Bits<22> {}
impl Reg64BitsConcat<13, 35> for Reg64Bits<22> {}
impl Reg64BitsDownCast<14> for Reg64Bits<22> {}
impl Reg64BitsConcat<14, 36> for Reg64Bits<22> {}
impl Reg64BitsDownCast<15> for Reg64Bits<22> {}
impl Reg64BitsConcat<15, 37> for Reg64Bits<22> {}
impl Reg64BitsDownCast<16> for Reg64Bits<22> {}
impl Reg64BitsConcat<16, 38> for Reg64Bits<22> {}
impl Reg64BitsDownCast<17> for Reg64Bits<22> {}
impl Reg64BitsConcat<17, 39> for Reg64Bits<22> {}
impl Reg64BitsDownCast<18> for Reg64Bits<22> {}
impl Reg64BitsConcat<18, 40> for Reg64Bits<22> {}
impl Reg64BitsDownCast<19> for Reg64Bits<22> {}
impl Reg64BitsConcat<19, 41> for Reg64Bits<22> {}
impl Reg64BitsDownCast<20> for Reg64Bits<22> {}
impl Reg64BitsConcat<20, 42> for Reg64Bits<22> {}
impl Reg64BitsDownCast<21> for Reg64Bits<22> {}
impl Reg64BitsConcat<21, 43> for Reg64Bits<22> {}
impl Reg64BitsDownCast<22> for Reg64Bits<22> {}
impl Reg64BitsConcat<22, 44> for Reg64Bits<22> {}
impl Reg64BitsDownCast<1> for Reg64Bits<23> {}
impl Reg64BitsConcat<1, 24> for Reg64Bits<23> {}
impl Reg64BitsDownCast<2> for Reg64Bits<23> {}
impl Reg64BitsConcat<2, 25> for Reg64Bits<23> {}
impl Reg64BitsDownCast<3> for Reg64Bits<23> {}
impl Reg64BitsConcat<3, 26> for Reg64Bits<23> {}
impl Reg64BitsDownCast<4> for Reg64Bits<23> {}
impl Reg64BitsConcat<4, 27> for Reg64Bits<23> {}
impl Reg64BitsDownCast<5> for Reg64Bits<23> {}
impl Reg64BitsConcat<5, 28> for Reg64Bits<23> {}
impl Reg64BitsDownCast<6> for Reg64Bits<23> {}
impl Reg64BitsConcat<6, 29> for Reg64Bits<23> {}
impl Reg64BitsDownCast<7> for Reg64Bits<23> {}
impl Reg64BitsConcat<7, 30> for Reg64Bits<23> {}
impl Reg64BitsDownCast<8> for Reg64Bits<23> {}
impl Reg64BitsConcat<8, 31> for Reg64Bits<23> {}
impl Reg64BitsDownCast<9> for Reg64Bits<23> {}
impl Reg64BitsConcat<9, 32> for Reg64Bits<23> {}
impl Reg64BitsDownCast<10> for Reg64Bits<23> {}
impl Reg64BitsConcat<10, 33> for Reg64Bits<23> {}
impl Reg64BitsDownCast<11> for Reg64Bits<23> {}
impl Reg64BitsConcat<11, 34> for Reg64Bits<23> {}
impl Reg64BitsDownCast<12> for Reg64Bits<23> {}
impl Reg64BitsConcat<12, 35> for Reg64Bits<23> {}
impl Reg64BitsDownCast<13> for Reg64Bits<23> {}
impl Reg64BitsConcat<13, 36> for Reg64Bits<23> {}
impl Reg64BitsDownCast<14> for Reg64Bits<23> {}
impl Reg64BitsConcat<14, 37> for Reg64Bits<23> {}
impl Reg64BitsDownCast<15> for Reg64Bits<23> {}
impl Reg64BitsConcat<15, 38> for Reg64Bits<23> {}
impl Reg64BitsDownCast<16> for Reg64Bits<23> {}
impl Reg64BitsConcat<16, 39> for Reg64Bits<23> {}
impl Reg64BitsDownCast<17> for Reg64Bits<23> {}
impl Reg64BitsConcat<17, 40> for Reg64Bits<23> {}
impl Reg64BitsDownCast<18> for Reg64Bits<23> {}
impl Reg64BitsConcat<18, 41> for Reg64Bits<23> {}
impl Reg64BitsDownCast<19> for Reg64Bits<23> {}
impl Reg64BitsConcat<19, 42> for Reg64Bits<23> {}
impl Reg64BitsDownCast<20> for Reg64Bits<23> {}
impl Reg64BitsConcat<20, 43> for Reg64Bits<23> {}
impl Reg64BitsDownCast<21> for Reg64Bits<23> {}
impl Reg64BitsConcat<21, 44> for Reg64Bits<23> {}
impl Reg64BitsDownCast<22> for Reg64Bits<23> {}
impl Reg64BitsConcat<22, 45> for Reg64Bits<23> {}
impl Reg64BitsDownCast<23> for Reg64Bits<23> {}
impl Reg64BitsConcat<23, 46> for Reg64Bits<23> {}
impl Reg64BitsDownCast<1> for Reg64Bits<24> {}
impl Reg64BitsConcat<1, 25> for Reg64Bits<24> {}
impl Reg64BitsDownCast<2> for Reg64Bits<24> {}
impl Reg64BitsConcat<2, 26> for Reg64Bits<24> {}
impl Reg64BitsDownCast<3> for Reg64Bits<24> {}
impl Reg64BitsConcat<3, 27> for Reg64Bits<24> {}
impl Reg64BitsDownCast<4> for Reg64Bits<24> {}
impl Reg64BitsConcat<4, 28> for Reg64Bits<24> {}
impl Reg64BitsDownCast<5> for Reg64Bits<24> {}
impl Reg64BitsConcat<5, 29> for Reg64Bits<24> {}
impl Reg64BitsDownCast<6> for Reg64Bits<24> {}
impl Reg64BitsConcat<6, 30> for Reg64Bits<24> {}
impl Reg64BitsDownCast<7> for Reg64Bits<24> {}
impl Reg64BitsConcat<7, 31> for Reg64Bits<24> {}
impl Reg64BitsDownCast<8> for Reg64Bits<24> {}
impl Reg64BitsConcat<8, 32> for Reg64Bits<24> {}
impl Reg64BitsDownCast<9> for Reg64Bits<24> {}
impl Reg64BitsConcat<9, 33> for Reg64Bits<24> {}
impl Reg64BitsDownCast<10> for Reg64Bits<24> {}
impl Reg64BitsConcat<10, 34> for Reg64Bits<24> {}
impl Reg64BitsDownCast<11> for Reg64Bits<24> {}
impl Reg64BitsConcat<11, 35> for Reg64Bits<24> {}
impl Reg64BitsDownCast<12> for Reg64Bits<24> {}
impl Reg64BitsConcat<12, 36> for Reg64Bits<24> {}
impl Reg64BitsDownCast<13> for Reg64Bits<24> {}
impl Reg64BitsConcat<13, 37> for Reg64Bits<24> {}
impl Reg64BitsDownCast<14> for Reg64Bits<24> {}
impl Reg64BitsConcat<14, 38> for Reg64Bits<24> {}
impl Reg64BitsDownCast<15> for Reg64Bits<24> {}
impl Reg64BitsConcat<15, 39> for Reg64Bits<24> {}
impl Reg64BitsDownCast<16> for Reg64Bits<24> {}
impl Reg64BitsConcat<16, 40> for Reg64Bits<24> {}
impl Reg64BitsDownCast<17> for Reg64Bits<24> {}
impl Reg64BitsConcat<17, 41> for Reg64Bits<24> {}
impl Reg64BitsDownCast<18> for Reg64Bits<24> {}
impl Reg64BitsConcat<18, 42> for Reg64Bits<24> {}
impl Reg64BitsDownCast<19> for Reg64Bits<24> {}
impl Reg64BitsConcat<19, 43> for Reg64Bits<24> {}
impl Reg64BitsDownCast<20> for Reg64Bits<24> {}
impl Reg64BitsConcat<20, 44> for Reg64Bits<24> {}
impl Reg64BitsDownCast<21> for Reg64Bits<24> {}
impl Reg64BitsConcat<21, 45> for Reg64Bits<24> {}
impl Reg64BitsDownCast<22> for Reg64Bits<24> {}
impl Reg64BitsConcat<22, 46> for Reg64Bits<24> {}
impl Reg64BitsDownCast<23> for Reg64Bits<24> {}
impl Reg64BitsConcat<23, 47> for Reg64Bits<24> {}
impl Reg64BitsDownCast<24> for Reg64Bits<24> {}
impl Reg64BitsConcat<24, 48> for Reg64Bits<24> {}
impl Reg64BitsDownCast<1> for Reg64Bits<25> {}
impl Reg64BitsConcat<1, 26> for Reg64Bits<25> {}
impl Reg64BitsDownCast<2> for Reg64Bits<25> {}
impl Reg64BitsConcat<2, 27> for Reg64Bits<25> {}
impl Reg64BitsDownCast<3> for Reg64Bits<25> {}
impl Reg64BitsConcat<3, 28> for Reg64Bits<25> {}
impl Reg64BitsDownCast<4> for Reg64Bits<25> {}
impl Reg64BitsConcat<4, 29> for Reg64Bits<25> {}
impl Reg64BitsDownCast<5> for Reg64Bits<25> {}
impl Reg64BitsConcat<5, 30> for Reg64Bits<25> {}
impl Reg64BitsDownCast<6> for Reg64Bits<25> {}
impl Reg64BitsConcat<6, 31> for Reg64Bits<25> {}
impl Reg64BitsDownCast<7> for Reg64Bits<25> {}
impl Reg64BitsConcat<7, 32> for Reg64Bits<25> {}
impl Reg64BitsDownCast<8> for Reg64Bits<25> {}
impl Reg64BitsConcat<8, 33> for Reg64Bits<25> {}
impl Reg64BitsDownCast<9> for Reg64Bits<25> {}
impl Reg64BitsConcat<9, 34> for Reg64Bits<25> {}
impl Reg64BitsDownCast<10> for Reg64Bits<25> {}
impl Reg64BitsConcat<10, 35> for Reg64Bits<25> {}
impl Reg64BitsDownCast<11> for Reg64Bits<25> {}
impl Reg64BitsConcat<11, 36> for Reg64Bits<25> {}
impl Reg64BitsDownCast<12> for Reg64Bits<25> {}
impl Reg64BitsConcat<12, 37> for Reg64Bits<25> {}
impl Reg64BitsDownCast<13> for Reg64Bits<25> {}
impl Reg64BitsConcat<13, 38> for Reg64Bits<25> {}
impl Reg64BitsDownCast<14> for Reg64Bits<25> {}
impl Reg64BitsConcat<14, 39> for Reg64Bits<25> {}
impl Reg64BitsDownCast<15> for Reg64Bits<25> {}
impl Reg64BitsConcat<15, 40> for Reg64Bits<25> {}
impl Reg64BitsDownCast<16> for Reg64Bits<25> {}
impl Reg64BitsConcat<16, 41> for Reg64Bits<25> {}
impl Reg64BitsDownCast<17> for Reg64Bits<25> {}
impl Reg64BitsConcat<17, 42> for Reg64Bits<25> {}
impl Reg64BitsDownCast<18> for Reg64Bits<25> {}
impl Reg64BitsConcat<18, 43> for Reg64Bits<25> {}
impl Reg64BitsDownCast<19> for Reg64Bits<25> {}
impl Reg64BitsConcat<19, 44> for Reg64Bits<25> {}
impl Reg64BitsDownCast<20> for Reg64Bits<25> {}
impl Reg64BitsConcat<20, 45> for Reg64Bits<25> {}
impl Reg64BitsDownCast<21> for Reg64Bits<25> {}
impl Reg64BitsConcat<21, 46> for Reg64Bits<25> {}
impl Reg64BitsDownCast<22> for Reg64Bits<25> {}
impl Reg64BitsConcat<22, 47> for Reg64Bits<25> {}
impl Reg64BitsDownCast<23> for Reg64Bits<25> {}
impl Reg64BitsConcat<23, 48> for Reg64Bits<25> {}
impl Reg64BitsDownCast<24> for Reg64Bits<25> {}
impl Reg64BitsConcat<24, 49> for Reg64Bits<25> {}
impl Reg64BitsDownCast<25> for Reg64Bits<25> {}
impl Reg64BitsConcat<25, 50> for Reg64Bits<25> {}
impl Reg64BitsDownCast<1> for Reg64Bits<26> {}
impl Reg64BitsConcat<1, 27> for Reg64Bits<26> {}
impl Reg64BitsDownCast<2> for Reg64Bits<26> {}
impl Reg64BitsConcat<2, 28> for Reg64Bits<26> {}
impl Reg64BitsDownCast<3> for Reg64Bits<26> {}
impl Reg64BitsConcat<3, 29> for Reg64Bits<26> {}
impl Reg64BitsDownCast<4> for Reg64Bits<26> {}
impl Reg64BitsConcat<4, 30> for Reg64Bits<26> {}
impl Reg64BitsDownCast<5> for Reg64Bits<26> {}
impl Reg64BitsConcat<5, 31> for Reg64Bits<26> {}
impl Reg64BitsDownCast<6> for Reg64Bits<26> {}
impl Reg64BitsConcat<6, 32> for Reg64Bits<26> {}
impl Reg64BitsDownCast<7> for Reg64Bits<26> {}
impl Reg64BitsConcat<7, 33> for Reg64Bits<26> {}
impl Reg64BitsDownCast<8> for Reg64Bits<26> {}
impl Reg64BitsConcat<8, 34> for Reg64Bits<26> {}
impl Reg64BitsDownCast<9> for Reg64Bits<26> {}
impl Reg64BitsConcat<9, 35> for Reg64Bits<26> {}
impl Reg64BitsDownCast<10> for Reg64Bits<26> {}
impl Reg64BitsConcat<10, 36> for Reg64Bits<26> {}
impl Reg64BitsDownCast<11> for Reg64Bits<26> {}
impl Reg64BitsConcat<11, 37> for Reg64Bits<26> {}
impl Reg64BitsDownCast<12> for Reg64Bits<26> {}
impl Reg64BitsConcat<12, 38> for Reg64Bits<26> {}
impl Reg64BitsDownCast<13> for Reg64Bits<26> {}
impl Reg64BitsConcat<13, 39> for Reg64Bits<26> {}
impl Reg64BitsDownCast<14> for Reg64Bits<26> {}
impl Reg64BitsConcat<14, 40> for Reg64Bits<26> {}
impl Reg64BitsDownCast<15> for Reg64Bits<26> {}
impl Reg64BitsConcat<15, 41> for Reg64Bits<26> {}
impl Reg64BitsDownCast<16> for Reg64Bits<26> {}
impl Reg64BitsConcat<16, 42> for Reg64Bits<26> {}
impl Reg64BitsDownCast<17> for Reg64Bits<26> {}
impl Reg64BitsConcat<17, 43> for Reg64Bits<26> {}
impl Reg64BitsDownCast<18> for Reg64Bits<26> {}
impl Reg64BitsConcat<18, 44> for Reg64Bits<26> {}
impl Reg64BitsDownCast<19> for Reg64Bits<26> {}
impl Reg64BitsConcat<19, 45> for Reg64Bits<26> {}
impl Reg64BitsDownCast<20> for Reg64Bits<26> {}
impl Reg64BitsConcat<20, 46> for Reg64Bits<26> {}
impl Reg64BitsDownCast<21> for Reg64Bits<26> {}
impl Reg64BitsConcat<21, 47> for Reg64Bits<26> {}
impl Reg64BitsDownCast<22> for Reg64Bits<26> {}
impl Reg64BitsConcat<22, 48> for Reg64Bits<26> {}
impl Reg64BitsDownCast<23> for Reg64Bits<26> {}
impl Reg64BitsConcat<23, 49> for Reg64Bits<26> {}
impl Reg64BitsDownCast<24> for Reg64Bits<26> {}
impl Reg64BitsConcat<24, 50> for Reg64Bits<26> {}
impl Reg64BitsDownCast<25> for Reg64Bits<26> {}
impl Reg64BitsConcat<25, 51> for Reg64Bits<26> {}
impl Reg64BitsDownCast<26> for Reg64Bits<26> {}
impl Reg64BitsConcat<26, 52> for Reg64Bits<26> {}
impl Reg64BitsDownCast<1> for Reg64Bits<27> {}
impl Reg64BitsConcat<1, 28> for Reg64Bits<27> {}
impl Reg64BitsDownCast<2> for Reg64Bits<27> {}
impl Reg64BitsConcat<2, 29> for Reg64Bits<27> {}
impl Reg64BitsDownCast<3> for Reg64Bits<27> {}
impl Reg64BitsConcat<3, 30> for Reg64Bits<27> {}
impl Reg64BitsDownCast<4> for Reg64Bits<27> {}
impl Reg64BitsConcat<4, 31> for Reg64Bits<27> {}
impl Reg64BitsDownCast<5> for Reg64Bits<27> {}
impl Reg64BitsConcat<5, 32> for Reg64Bits<27> {}
impl Reg64BitsDownCast<6> for Reg64Bits<27> {}
impl Reg64BitsConcat<6, 33> for Reg64Bits<27> {}
impl Reg64BitsDownCast<7> for Reg64Bits<27> {}
impl Reg64BitsConcat<7, 34> for Reg64Bits<27> {}
impl Reg64BitsDownCast<8> for Reg64Bits<27> {}
impl Reg64BitsConcat<8, 35> for Reg64Bits<27> {}
impl Reg64BitsDownCast<9> for Reg64Bits<27> {}
impl Reg64BitsConcat<9, 36> for Reg64Bits<27> {}
impl Reg64BitsDownCast<10> for Reg64Bits<27> {}
impl Reg64BitsConcat<10, 37> for Reg64Bits<27> {}
impl Reg64BitsDownCast<11> for Reg64Bits<27> {}
impl Reg64BitsConcat<11, 38> for Reg64Bits<27> {}
impl Reg64BitsDownCast<12> for Reg64Bits<27> {}
impl Reg64BitsConcat<12, 39> for Reg64Bits<27> {}
impl Reg64BitsDownCast<13> for Reg64Bits<27> {}
impl Reg64BitsConcat<13, 40> for Reg64Bits<27> {}
impl Reg64BitsDownCast<14> for Reg64Bits<27> {}
impl Reg64BitsConcat<14, 41> for Reg64Bits<27> {}
impl Reg64BitsDownCast<15> for Reg64Bits<27> {}
impl Reg64BitsConcat<15, 42> for Reg64Bits<27> {}
impl Reg64BitsDownCast<16> for Reg64Bits<27> {}
impl Reg64BitsConcat<16, 43> for Reg64Bits<27> {}
impl Reg64BitsDownCast<17> for Reg64Bits<27> {}
impl Reg64BitsConcat<17, 44> for Reg64Bits<27> {}
impl Reg64BitsDownCast<18> for Reg64Bits<27> {}
impl Reg64BitsConcat<18, 45> for Reg64Bits<27> {}
impl Reg64BitsDownCast<19> for Reg64Bits<27> {}
impl Reg64BitsConcat<19, 46> for Reg64Bits<27> {}
impl Reg64BitsDownCast<20> for Reg64Bits<27> {}
impl Reg64BitsConcat<20, 47> for Reg64Bits<27> {}
impl Reg64BitsDownCast<21> for Reg64Bits<27> {}
impl Reg64BitsConcat<21, 48> for Reg64Bits<27> {}
impl Reg64BitsDownCast<22> for Reg64Bits<27> {}
impl Reg64BitsConcat<22, 49> for Reg64Bits<27> {}
impl Reg64BitsDownCast<23> for Reg64Bits<27> {}
impl Reg64BitsConcat<23, 50> for Reg64Bits<27> {}
impl Reg64BitsDownCast<24> for Reg64Bits<27> {}
impl Reg64BitsConcat<24, 51> for Reg64Bits<27> {}
impl Reg64BitsDownCast<25> for Reg64Bits<27> {}
impl Reg64BitsConcat<25, 52> for Reg64Bits<27> {}
impl Reg64BitsDownCast<26> for Reg64Bits<27> {}
impl Reg64BitsConcat<26, 53> for Reg64Bits<27> {}
impl Reg64BitsDownCast<27> for Reg64Bits<27> {}
impl Reg64BitsConcat<27, 54> for Reg64Bits<27> {}
impl Reg64BitsDownCast<1> for Reg64Bits<28> {}
impl Reg64BitsConcat<1, 29> for Reg64Bits<28> {}
impl Reg64BitsDownCast<2> for Reg64Bits<28> {}
impl Reg64BitsConcat<2, 30> for Reg64Bits<28> {}
impl Reg64BitsDownCast<3> for Reg64Bits<28> {}
impl Reg64BitsConcat<3, 31> for Reg64Bits<28> {}
impl Reg64BitsDownCast<4> for Reg64Bits<28> {}
impl Reg64BitsConcat<4, 32> for Reg64Bits<28> {}
impl Reg64BitsDownCast<5> for Reg64Bits<28> {}
impl Reg64BitsConcat<5, 33> for Reg64Bits<28> {}
impl Reg64BitsDownCast<6> for Reg64Bits<28> {}
impl Reg64BitsConcat<6, 34> for Reg64Bits<28> {}
impl Reg64BitsDownCast<7> for Reg64Bits<28> {}
impl Reg64BitsConcat<7, 35> for Reg64Bits<28> {}
impl Reg64BitsDownCast<8> for Reg64Bits<28> {}
impl Reg64BitsConcat<8, 36> for Reg64Bits<28> {}
impl Reg64BitsDownCast<9> for Reg64Bits<28> {}
impl Reg64BitsConcat<9, 37> for Reg64Bits<28> {}
impl Reg64BitsDownCast<10> for Reg64Bits<28> {}
impl Reg64BitsConcat<10, 38> for Reg64Bits<28> {}
impl Reg64BitsDownCast<11> for Reg64Bits<28> {}
impl Reg64BitsConcat<11, 39> for Reg64Bits<28> {}
impl Reg64BitsDownCast<12> for Reg64Bits<28> {}
impl Reg64BitsConcat<12, 40> for Reg64Bits<28> {}
impl Reg64BitsDownCast<13> for Reg64Bits<28> {}
impl Reg64BitsConcat<13, 41> for Reg64Bits<28> {}
impl Reg64BitsDownCast<14> for Reg64Bits<28> {}
impl Reg64BitsConcat<14, 42> for Reg64Bits<28> {}
impl Reg64BitsDownCast<15> for Reg64Bits<28> {}
impl Reg64BitsConcat<15, 43> for Reg64Bits<28> {}
impl Reg64BitsDownCast<16> for Reg64Bits<28> {}
impl Reg64BitsConcat<16, 44> for Reg64Bits<28> {}
impl Reg64BitsDownCast<17> for Reg64Bits<28> {}
impl Reg64BitsConcat<17, 45> for Reg64Bits<28> {}
impl Reg64BitsDownCast<18> for Reg64Bits<28> {}
impl Reg64BitsConcat<18, 46> for Reg64Bits<28> {}
impl Reg64BitsDownCast<19> for Reg64Bits<28> {}
impl Reg64BitsConcat<19, 47> for Reg64Bits<28> {}
impl Reg64BitsDownCast<20> for Reg64Bits<28> {}
impl Reg64BitsConcat<20, 48> for Reg64Bits<28> {}
impl Reg64BitsDownCast<21> for Reg64Bits<28> {}
impl Reg64BitsConcat<21, 49> for Reg64Bits<28> {}
impl Reg64BitsDownCast<22> for Reg64Bits<28> {}
impl Reg64BitsConcat<22, 50> for Reg64Bits<28> {}
impl Reg64BitsDownCast<23> for Reg64Bits<28> {}
impl Reg64BitsConcat<23, 51> for Reg64Bits<28> {}
impl Reg64BitsDownCast<24> for Reg64Bits<28> {}
impl Reg64BitsConcat<24, 52> for Reg64Bits<28> {}
impl Reg64BitsDownCast<25> for Reg64Bits<28> {}
impl Reg64BitsConcat<25, 53> for Reg64Bits<28> {}
impl Reg64BitsDownCast<26> for Reg64Bits<28> {}
impl Reg64BitsConcat<26, 54> for Reg64Bits<28> {}
impl Reg64BitsDownCast<27> for Reg64Bits<28> {}
impl Reg64BitsConcat<27, 55> for Reg64Bits<28> {}
impl Reg64BitsDownCast<28> for Reg64Bits<28> {}
impl Reg64BitsConcat<28, 56> for Reg64Bits<28> {}
impl Reg64BitsDownCast<1> for Reg64Bits<29> {}
impl Reg64BitsConcat<1, 30> for Reg64Bits<29> {}
impl Reg64BitsDownCast<2> for Reg64Bits<29> {}
impl Reg64BitsConcat<2, 31> for Reg64Bits<29> {}
impl Reg64BitsDownCast<3> for Reg64Bits<29> {}
impl Reg64BitsConcat<3, 32> for Reg64Bits<29> {}
impl Reg64BitsDownCast<4> for Reg64Bits<29> {}
impl Reg64BitsConcat<4, 33> for Reg64Bits<29> {}
impl Reg64BitsDownCast<5> for Reg64Bits<29> {}
impl Reg64BitsConcat<5, 34> for Reg64Bits<29> {}
impl Reg64BitsDownCast<6> for Reg64Bits<29> {}
impl Reg64BitsConcat<6, 35> for Reg64Bits<29> {}
impl Reg64BitsDownCast<7> for Reg64Bits<29> {}
impl Reg64BitsConcat<7, 36> for Reg64Bits<29> {}
impl Reg64BitsDownCast<8> for Reg64Bits<29> {}
impl Reg64BitsConcat<8, 37> for Reg64Bits<29> {}
impl Reg64BitsDownCast<9> for Reg64Bits<29> {}
impl Reg64BitsConcat<9, 38> for Reg64Bits<29> {}
impl Reg64BitsDownCast<10> for Reg64Bits<29> {}
impl Reg64BitsConcat<10, 39> for Reg64Bits<29> {}
impl Reg64BitsDownCast<11> for Reg64Bits<29> {}
impl Reg64BitsConcat<11, 40> for Reg64Bits<29> {}
impl Reg64BitsDownCast<12> for Reg64Bits<29> {}
impl Reg64BitsConcat<12, 41> for Reg64Bits<29> {}
impl Reg64BitsDownCast<13> for Reg64Bits<29> {}
impl Reg64BitsConcat<13, 42> for Reg64Bits<29> {}
impl Reg64BitsDownCast<14> for Reg64Bits<29> {}
impl Reg64BitsConcat<14, 43> for Reg64Bits<29> {}
impl Reg64BitsDownCast<15> for Reg64Bits<29> {}
impl Reg64BitsConcat<15, 44> for Reg64Bits<29> {}
impl Reg64BitsDownCast<16> for Reg64Bits<29> {}
impl Reg64BitsConcat<16, 45> for Reg64Bits<29> {}
impl Reg64BitsDownCast<17> for Reg64Bits<29> {}
impl Reg64BitsConcat<17, 46> for Reg64Bits<29> {}
impl Reg64BitsDownCast<18> for Reg64Bits<29> {}
impl Reg64BitsConcat<18, 47> for Reg64Bits<29> {}
impl Reg64BitsDownCast<19> for Reg64Bits<29> {}
impl Reg64BitsConcat<19, 48> for Reg64Bits<29> {}
impl Reg64BitsDownCast<20> for Reg64Bits<29> {}
impl Reg64BitsConcat<20, 49> for Reg64Bits<29> {}
impl Reg64BitsDownCast<21> for Reg64Bits<29> {}
impl Reg64BitsConcat<21, 50> for Reg64Bits<29> {}
impl Reg64BitsDownCast<22> for Reg64Bits<29> {}
impl Reg64BitsConcat<22, 51> for Reg64Bits<29> {}
impl Reg64BitsDownCast<23> for Reg64Bits<29> {}
impl Reg64BitsConcat<23, 52> for Reg64Bits<29> {}
impl Reg64BitsDownCast<24> for Reg64Bits<29> {}
impl Reg64BitsConcat<24, 53> for Reg64Bits<29> {}
impl Reg64BitsDownCast<25> for Reg64Bits<29> {}
impl Reg64BitsConcat<25, 54> for Reg64Bits<29> {}
impl Reg64BitsDownCast<26> for Reg64Bits<29> {}
impl Reg64BitsConcat<26, 55> for Reg64Bits<29> {}
impl Reg64BitsDownCast<27> for Reg64Bits<29> {}
impl Reg64BitsConcat<27, 56> for Reg64Bits<29> {}
impl Reg64BitsDownCast<28> for Reg64Bits<29> {}
impl Reg64BitsConcat<28, 57> for Reg64Bits<29> {}
impl Reg64BitsDownCast<29> for Reg64Bits<29> {}
impl Reg64BitsConcat<29, 58> for Reg64Bits<29> {}
impl Reg64BitsDownCast<1> for Reg64Bits<30> {}
impl Reg64BitsConcat<1, 31> for Reg64Bits<30> {}
impl Reg64BitsDownCast<2> for Reg64Bits<30> {}
impl Reg64BitsConcat<2, 32> for Reg64Bits<30> {}
impl Reg64BitsDownCast<3> for Reg64Bits<30> {}
impl Reg64BitsConcat<3, 33> for Reg64Bits<30> {}
impl Reg64BitsDownCast<4> for Reg64Bits<30> {}
impl Reg64BitsConcat<4, 34> for Reg64Bits<30> {}
impl Reg64BitsDownCast<5> for Reg64Bits<30> {}
impl Reg64BitsConcat<5, 35> for Reg64Bits<30> {}
impl Reg64BitsDownCast<6> for Reg64Bits<30> {}
impl Reg64BitsConcat<6, 36> for Reg64Bits<30> {}
impl Reg64BitsDownCast<7> for Reg64Bits<30> {}
impl Reg64BitsConcat<7, 37> for Reg64Bits<30> {}
impl Reg64BitsDownCast<8> for Reg64Bits<30> {}
impl Reg64BitsConcat<8, 38> for Reg64Bits<30> {}
impl Reg64BitsDownCast<9> for Reg64Bits<30> {}
impl Reg64BitsConcat<9, 39> for Reg64Bits<30> {}
impl Reg64BitsDownCast<10> for Reg64Bits<30> {}
impl Reg64BitsConcat<10, 40> for Reg64Bits<30> {}
impl Reg64BitsDownCast<11> for Reg64Bits<30> {}
impl Reg64BitsConcat<11, 41> for Reg64Bits<30> {}
impl Reg64BitsDownCast<12> for Reg64Bits<30> {}
impl Reg64BitsConcat<12, 42> for Reg64Bits<30> {}
impl Reg64BitsDownCast<13> for Reg64Bits<30> {}
impl Reg64BitsConcat<13, 43> for Reg64Bits<30> {}
impl Reg64BitsDownCast<14> for Reg64Bits<30> {}
impl Reg64BitsConcat<14, 44> for Reg64Bits<30> {}
impl Reg64BitsDownCast<15> for Reg64Bits<30> {}
impl Reg64BitsConcat<15, 45> for Reg64Bits<30> {}
impl Reg64BitsDownCast<16> for Reg64Bits<30> {}
impl Reg64BitsConcat<16, 46> for Reg64Bits<30> {}
impl Reg64BitsDownCast<17> for Reg64Bits<30> {}
impl Reg64BitsConcat<17, 47> for Reg64Bits<30> {}
impl Reg64BitsDownCast<18> for Reg64Bits<30> {}
impl Reg64BitsConcat<18, 48> for Reg64Bits<30> {}
impl Reg64BitsDownCast<19> for Reg64Bits<30> {}
impl Reg64BitsConcat<19, 49> for Reg64Bits<30> {}
impl Reg64BitsDownCast<20> for Reg64Bits<30> {}
impl Reg64BitsConcat<20, 50> for Reg64Bits<30> {}
impl Reg64BitsDownCast<21> for Reg64Bits<30> {}
impl Reg64BitsConcat<21, 51> for Reg64Bits<30> {}
impl Reg64BitsDownCast<22> for Reg64Bits<30> {}
impl Reg64BitsConcat<22, 52> for Reg64Bits<30> {}
impl Reg64BitsDownCast<23> for Reg64Bits<30> {}
impl Reg64BitsConcat<23, 53> for Reg64Bits<30> {}
impl Reg64BitsDownCast<24> for Reg64Bits<30> {}
impl Reg64BitsConcat<24, 54> for Reg64Bits<30> {}
impl Reg64BitsDownCast<25> for Reg64Bits<30> {}
impl Reg64BitsConcat<25, 55> for Reg64Bits<30> {}
impl Reg64BitsDownCast<26> for Reg64Bits<30> {}
impl Reg64BitsConcat<26, 56> for Reg64Bits<30> {}
impl Reg64BitsDownCast<27> for Reg64Bits<30> {}
impl Reg64BitsConcat<27, 57> for Reg64Bits<30> {}
impl Reg64BitsDownCast<28> for Reg64Bits<30> {}
impl Reg64BitsConcat<28, 58> for Reg64Bits<30> {}
impl Reg64BitsDownCast<29> for Reg64Bits<30> {}
impl Reg64BitsConcat<29, 59> for Reg64Bits<30> {}
impl Reg64BitsDownCast<30> for Reg64Bits<30> {}
impl Reg64BitsConcat<30, 60> for Reg64Bits<30> {}
impl Reg64BitsDownCast<1> for Reg64Bits<31> {}
impl Reg64BitsConcat<1, 32> for Reg64Bits<31> {}
impl Reg64BitsDownCast<2> for Reg64Bits<31> {}
impl Reg64BitsConcat<2, 33> for Reg64Bits<31> {}
impl Reg64BitsDownCast<3> for Reg64Bits<31> {}
impl Reg64BitsConcat<3, 34> for Reg64Bits<31> {}
impl Reg64BitsDownCast<4> for Reg64Bits<31> {}
impl Reg64BitsConcat<4, 35> for Reg64Bits<31> {}
impl Reg64BitsDownCast<5> for Reg64Bits<31> {}
impl Reg64BitsConcat<5, 36> for Reg64Bits<31> {}
impl Reg64BitsDownCast<6> for Reg64Bits<31> {}
impl Reg64BitsConcat<6, 37> for Reg64Bits<31> {}
impl Reg64BitsDownCast<7> for Reg64Bits<31> {}
impl Reg64BitsConcat<7, 38> for Reg64Bits<31> {}
impl Reg64BitsDownCast<8> for Reg64Bits<31> {}
impl Reg64BitsConcat<8, 39> for Reg64Bits<31> {}
impl Reg64BitsDownCast<9> for Reg64Bits<31> {}
impl Reg64BitsConcat<9, 40> for Reg64Bits<31> {}
impl Reg64BitsDownCast<10> for Reg64Bits<31> {}
impl Reg64BitsConcat<10, 41> for Reg64Bits<31> {}
impl Reg64BitsDownCast<11> for Reg64Bits<31> {}
impl Reg64BitsConcat<11, 42> for Reg64Bits<31> {}
impl Reg64BitsDownCast<12> for Reg64Bits<31> {}
impl Reg64BitsConcat<12, 43> for Reg64Bits<31> {}
impl Reg64BitsDownCast<13> for Reg64Bits<31> {}
impl Reg64BitsConcat<13, 44> for Reg64Bits<31> {}
impl Reg64BitsDownCast<14> for Reg64Bits<31> {}
impl Reg64BitsConcat<14, 45> for Reg64Bits<31> {}
impl Reg64BitsDownCast<15> for Reg64Bits<31> {}
impl Reg64BitsConcat<15, 46> for Reg64Bits<31> {}
impl Reg64BitsDownCast<16> for Reg64Bits<31> {}
impl Reg64BitsConcat<16, 47> for Reg64Bits<31> {}
impl Reg64BitsDownCast<17> for Reg64Bits<31> {}
impl Reg64BitsConcat<17, 48> for Reg64Bits<31> {}
impl Reg64BitsDownCast<18> for Reg64Bits<31> {}
impl Reg64BitsConcat<18, 49> for Reg64Bits<31> {}
impl Reg64BitsDownCast<19> for Reg64Bits<31> {}
impl Reg64BitsConcat<19, 50> for Reg64Bits<31> {}
impl Reg64BitsDownCast<20> for Reg64Bits<31> {}
impl Reg64BitsConcat<20, 51> for Reg64Bits<31> {}
impl Reg64BitsDownCast<21> for Reg64Bits<31> {}
impl Reg64BitsConcat<21, 52> for Reg64Bits<31> {}
impl Reg64BitsDownCast<22> for Reg64Bits<31> {}
impl Reg64BitsConcat<22, 53> for Reg64Bits<31> {}
impl Reg64BitsDownCast<23> for Reg64Bits<31> {}
impl Reg64BitsConcat<23, 54> for Reg64Bits<31> {}
impl Reg64BitsDownCast<24> for Reg64Bits<31> {}
impl Reg64BitsConcat<24, 55> for Reg64Bits<31> {}
impl Reg64BitsDownCast<25> for Reg64Bits<31> {}
impl Reg64BitsConcat<25, 56> for Reg64Bits<31> {}
impl Reg64BitsDownCast<26> for Reg64Bits<31> {}
impl Reg64BitsConcat<26, 57> for Reg64Bits<31> {}
impl Reg64BitsDownCast<27> for Reg64Bits<31> {}
impl Reg64BitsConcat<27, 58> for Reg64Bits<31> {}
impl Reg64BitsDownCast<28> for Reg64Bits<31> {}
impl Reg64BitsConcat<28, 59> for Reg64Bits<31> {}
impl Reg64BitsDownCast<29> for Reg64Bits<31> {}
impl Reg64BitsConcat<29, 60> for Reg64Bits<31> {}
impl Reg64BitsDownCast<30> for Reg64Bits<31> {}
impl Reg64BitsConcat<30, 61> for Reg64Bits<31> {}
impl Reg64BitsDownCast<31> for Reg64Bits<31> {}
impl Reg64BitsConcat<31, 62> for Reg64Bits<31> {}
impl Reg64BitsDownCast<1> for Reg64Bits<32> {}
impl Reg64BitsConcat<1, 33> for Reg64Bits<32> {}
impl Reg64BitsDownCast<2> for Reg64Bits<32> {}
impl Reg64BitsConcat<2, 34> for Reg64Bits<32> {}
impl Reg64BitsDownCast<3> for Reg64Bits<32> {}
impl Reg64BitsConcat<3, 35> for Reg64Bits<32> {}
impl Reg64BitsDownCast<4> for Reg64Bits<32> {}
impl Reg64BitsConcat<4, 36> for Reg64Bits<32> {}
impl Reg64BitsDownCast<5> for Reg64Bits<32> {}
impl Reg64BitsConcat<5, 37> for Reg64Bits<32> {}
impl Reg64BitsDownCast<6> for Reg64Bits<32> {}
impl Reg64BitsConcat<6, 38> for Reg64Bits<32> {}
impl Reg64BitsDownCast<7> for Reg64Bits<32> {}
impl Reg64BitsConcat<7, 39> for Reg64Bits<32> {}
impl Reg64BitsDownCast<8> for Reg64Bits<32> {}
impl Reg64BitsConcat<8, 40> for Reg64Bits<32> {}
impl Reg64BitsDownCast<9> for Reg64Bits<32> {}
impl Reg64BitsConcat<9, 41> for Reg64Bits<32> {}
impl Reg64BitsDownCast<10> for Reg64Bits<32> {}
impl Reg64BitsConcat<10, 42> for Reg64Bits<32> {}
impl Reg64BitsDownCast<11> for Reg64Bits<32> {}
impl Reg64BitsConcat<11, 43> for Reg64Bits<32> {}
impl Reg64BitsDownCast<12> for Reg64Bits<32> {}
impl Reg64BitsConcat<12, 44> for Reg64Bits<32> {}
impl Reg64BitsDownCast<13> for Reg64Bits<32> {}
impl Reg64BitsConcat<13, 45> for Reg64Bits<32> {}
impl Reg64BitsDownCast<14> for Reg64Bits<32> {}
impl Reg64BitsConcat<14, 46> for Reg64Bits<32> {}
impl Reg64BitsDownCast<15> for Reg64Bits<32> {}
impl Reg64BitsConcat<15, 47> for Reg64Bits<32> {}
impl Reg64BitsDownCast<16> for Reg64Bits<32> {}
impl Reg64BitsConcat<16, 48> for Reg64Bits<32> {}
impl Reg64BitsDownCast<17> for Reg64Bits<32> {}
impl Reg64BitsConcat<17, 49> for Reg64Bits<32> {}
impl Reg64BitsDownCast<18> for Reg64Bits<32> {}
impl Reg64BitsConcat<18, 50> for Reg64Bits<32> {}
impl Reg64BitsDownCast<19> for Reg64Bits<32> {}
impl Reg64BitsConcat<19, 51> for Reg64Bits<32> {}
impl Reg64BitsDownCast<20> for Reg64Bits<32> {}
impl Reg64BitsConcat<20, 52> for Reg64Bits<32> {}
impl Reg64BitsDownCast<21> for Reg64Bits<32> {}
impl Reg64BitsConcat<21, 53> for Reg64Bits<32> {}
impl Reg64BitsDownCast<22> for Reg64Bits<32> {}
impl Reg64BitsConcat<22, 54> for Reg64Bits<32> {}
impl Reg64BitsDownCast<23> for Reg64Bits<32> {}
impl Reg64BitsConcat<23, 55> for Reg64Bits<32> {}
impl Reg64BitsDownCast<24> for Reg64Bits<32> {}
impl Reg64BitsConcat<24, 56> for Reg64Bits<32> {}
impl Reg64BitsDownCast<25> for Reg64Bits<32> {}
impl Reg64BitsConcat<25, 57> for Reg64Bits<32> {}
impl Reg64BitsDownCast<26> for Reg64Bits<32> {}
impl Reg64BitsConcat<26, 58> for Reg64Bits<32> {}
impl Reg64BitsDownCast<27> for Reg64Bits<32> {}
impl Reg64BitsConcat<27, 59> for Reg64Bits<32> {}
impl Reg64BitsDownCast<28> for Reg64Bits<32> {}
impl Reg64BitsConcat<28, 60> for Reg64Bits<32> {}
impl Reg64BitsDownCast<29> for Reg64Bits<32> {}
impl Reg64BitsConcat<29, 61> for Reg64Bits<32> {}
impl Reg64BitsDownCast<30> for Reg64Bits<32> {}
impl Reg64BitsConcat<30, 62> for Reg64Bits<32> {}
impl Reg64BitsDownCast<31> for Reg64Bits<32> {}
impl Reg64BitsConcat<31, 63> for Reg64Bits<32> {}
impl Reg64BitsDownCast<32> for Reg64Bits<32> {}
impl Reg64BitsConcat<32, 64> for Reg64Bits<32> {}
impl Reg64BitsDownCast<1> for Reg64Bits<33> {}
impl Reg64BitsConcat<1, 34> for Reg64Bits<33> {}
impl Reg64BitsDownCast<2> for Reg64Bits<33> {}
impl Reg64BitsConcat<2, 35> for Reg64Bits<33> {}
impl Reg64BitsDownCast<3> for Reg64Bits<33> {}
impl Reg64BitsConcat<3, 36> for Reg64Bits<33> {}
impl Reg64BitsDownCast<4> for Reg64Bits<33> {}
impl Reg64BitsConcat<4, 37> for Reg64Bits<33> {}
impl Reg64BitsDownCast<5> for Reg64Bits<33> {}
impl Reg64BitsConcat<5, 38> for Reg64Bits<33> {}
impl Reg64BitsDownCast<6> for Reg64Bits<33> {}
impl Reg64BitsConcat<6, 39> for Reg64Bits<33> {}
impl Reg64BitsDownCast<7> for Reg64Bits<33> {}
impl Reg64BitsConcat<7, 40> for Reg64Bits<33> {}
impl Reg64BitsDownCast<8> for Reg64Bits<33> {}
impl Reg64BitsConcat<8, 41> for Reg64Bits<33> {}
impl Reg64BitsDownCast<9> for Reg64Bits<33> {}
impl Reg64BitsConcat<9, 42> for Reg64Bits<33> {}
impl Reg64BitsDownCast<10> for Reg64Bits<33> {}
impl Reg64BitsConcat<10, 43> for Reg64Bits<33> {}
impl Reg64BitsDownCast<11> for Reg64Bits<33> {}
impl Reg64BitsConcat<11, 44> for Reg64Bits<33> {}
impl Reg64BitsDownCast<12> for Reg64Bits<33> {}
impl Reg64BitsConcat<12, 45> for Reg64Bits<33> {}
impl Reg64BitsDownCast<13> for Reg64Bits<33> {}
impl Reg64BitsConcat<13, 46> for Reg64Bits<33> {}
impl Reg64BitsDownCast<14> for Reg64Bits<33> {}
impl Reg64BitsConcat<14, 47> for Reg64Bits<33> {}
impl Reg64BitsDownCast<15> for Reg64Bits<33> {}
impl Reg64BitsConcat<15, 48> for Reg64Bits<33> {}
impl Reg64BitsDownCast<16> for Reg64Bits<33> {}
impl Reg64BitsConcat<16, 49> for Reg64Bits<33> {}
impl Reg64BitsDownCast<17> for Reg64Bits<33> {}
impl Reg64BitsConcat<17, 50> for Reg64Bits<33> {}
impl Reg64BitsDownCast<18> for Reg64Bits<33> {}
impl Reg64BitsConcat<18, 51> for Reg64Bits<33> {}
impl Reg64BitsDownCast<19> for Reg64Bits<33> {}
impl Reg64BitsConcat<19, 52> for Reg64Bits<33> {}
impl Reg64BitsDownCast<20> for Reg64Bits<33> {}
impl Reg64BitsConcat<20, 53> for Reg64Bits<33> {}
impl Reg64BitsDownCast<21> for Reg64Bits<33> {}
impl Reg64BitsConcat<21, 54> for Reg64Bits<33> {}
impl Reg64BitsDownCast<22> for Reg64Bits<33> {}
impl Reg64BitsConcat<22, 55> for Reg64Bits<33> {}
impl Reg64BitsDownCast<23> for Reg64Bits<33> {}
impl Reg64BitsConcat<23, 56> for Reg64Bits<33> {}
impl Reg64BitsDownCast<24> for Reg64Bits<33> {}
impl Reg64BitsConcat<24, 57> for Reg64Bits<33> {}
impl Reg64BitsDownCast<25> for Reg64Bits<33> {}
impl Reg64BitsConcat<25, 58> for Reg64Bits<33> {}
impl Reg64BitsDownCast<26> for Reg64Bits<33> {}
impl Reg64BitsConcat<26, 59> for Reg64Bits<33> {}
impl Reg64BitsDownCast<27> for Reg64Bits<33> {}
impl Reg64BitsConcat<27, 60> for Reg64Bits<33> {}
impl Reg64BitsDownCast<28> for Reg64Bits<33> {}
impl Reg64BitsConcat<28, 61> for Reg64Bits<33> {}
impl Reg64BitsDownCast<29> for Reg64Bits<33> {}
impl Reg64BitsConcat<29, 62> for Reg64Bits<33> {}
impl Reg64BitsDownCast<30> for Reg64Bits<33> {}
impl Reg64BitsConcat<30, 63> for Reg64Bits<33> {}
impl Reg64BitsDownCast<31> for Reg64Bits<33> {}
impl Reg64BitsConcat<31, 64> for Reg64Bits<33> {}
impl Reg64BitsDownCast<32> for Reg64Bits<33> {}
impl Reg64BitsDownCast<33> for Reg64Bits<33> {}
impl Reg64BitsDownCast<1> for Reg64Bits<34> {}
impl Reg64BitsConcat<1, 35> for Reg64Bits<34> {}
impl Reg64BitsDownCast<2> for Reg64Bits<34> {}
impl Reg64BitsConcat<2, 36> for Reg64Bits<34> {}
impl Reg64BitsDownCast<3> for Reg64Bits<34> {}
impl Reg64BitsConcat<3, 37> for Reg64Bits<34> {}
impl Reg64BitsDownCast<4> for Reg64Bits<34> {}
impl Reg64BitsConcat<4, 38> for Reg64Bits<34> {}
impl Reg64BitsDownCast<5> for Reg64Bits<34> {}
impl Reg64BitsConcat<5, 39> for Reg64Bits<34> {}
impl Reg64BitsDownCast<6> for Reg64Bits<34> {}
impl Reg64BitsConcat<6, 40> for Reg64Bits<34> {}
impl Reg64BitsDownCast<7> for Reg64Bits<34> {}
impl Reg64BitsConcat<7, 41> for Reg64Bits<34> {}
impl Reg64BitsDownCast<8> for Reg64Bits<34> {}
impl Reg64BitsConcat<8, 42> for Reg64Bits<34> {}
impl Reg64BitsDownCast<9> for Reg64Bits<34> {}
impl Reg64BitsConcat<9, 43> for Reg64Bits<34> {}
impl Reg64BitsDownCast<10> for Reg64Bits<34> {}
impl Reg64BitsConcat<10, 44> for Reg64Bits<34> {}
impl Reg64BitsDownCast<11> for Reg64Bits<34> {}
impl Reg64BitsConcat<11, 45> for Reg64Bits<34> {}
impl Reg64BitsDownCast<12> for Reg64Bits<34> {}
impl Reg64BitsConcat<12, 46> for Reg64Bits<34> {}
impl Reg64BitsDownCast<13> for Reg64Bits<34> {}
impl Reg64BitsConcat<13, 47> for Reg64Bits<34> {}
impl Reg64BitsDownCast<14> for Reg64Bits<34> {}
impl Reg64BitsConcat<14, 48> for Reg64Bits<34> {}
impl Reg64BitsDownCast<15> for Reg64Bits<34> {}
impl Reg64BitsConcat<15, 49> for Reg64Bits<34> {}
impl Reg64BitsDownCast<16> for Reg64Bits<34> {}
impl Reg64BitsConcat<16, 50> for Reg64Bits<34> {}
impl Reg64BitsDownCast<17> for Reg64Bits<34> {}
impl Reg64BitsConcat<17, 51> for Reg64Bits<34> {}
impl Reg64BitsDownCast<18> for Reg64Bits<34> {}
impl Reg64BitsConcat<18, 52> for Reg64Bits<34> {}
impl Reg64BitsDownCast<19> for Reg64Bits<34> {}
impl Reg64BitsConcat<19, 53> for Reg64Bits<34> {}
impl Reg64BitsDownCast<20> for Reg64Bits<34> {}
impl Reg64BitsConcat<20, 54> for Reg64Bits<34> {}
impl Reg64BitsDownCast<21> for Reg64Bits<34> {}
impl Reg64BitsConcat<21, 55> for Reg64Bits<34> {}
impl Reg64BitsDownCast<22> for Reg64Bits<34> {}
impl Reg64BitsConcat<22, 56> for Reg64Bits<34> {}
impl Reg64BitsDownCast<23> for Reg64Bits<34> {}
impl Reg64BitsConcat<23, 57> for Reg64Bits<34> {}
impl Reg64BitsDownCast<24> for Reg64Bits<34> {}
impl Reg64BitsConcat<24, 58> for Reg64Bits<34> {}
impl Reg64BitsDownCast<25> for Reg64Bits<34> {}
impl Reg64BitsConcat<25, 59> for Reg64Bits<34> {}
impl Reg64BitsDownCast<26> for Reg64Bits<34> {}
impl Reg64BitsConcat<26, 60> for Reg64Bits<34> {}
impl Reg64BitsDownCast<27> for Reg64Bits<34> {}
impl Reg64BitsConcat<27, 61> for Reg64Bits<34> {}
impl Reg64BitsDownCast<28> for Reg64Bits<34> {}
impl Reg64BitsConcat<28, 62> for Reg64Bits<34> {}
impl Reg64BitsDownCast<29> for Reg64Bits<34> {}
impl Reg64BitsConcat<29, 63> for Reg64Bits<34> {}
impl Reg64BitsDownCast<30> for Reg64Bits<34> {}
impl Reg64BitsConcat<30, 64> for Reg64Bits<34> {}
impl Reg64BitsDownCast<31> for Reg64Bits<34> {}
impl Reg64BitsDownCast<32> for Reg64Bits<34> {}
impl Reg64BitsDownCast<33> for Reg64Bits<34> {}
impl Reg64BitsDownCast<34> for Reg64Bits<34> {}
impl Reg64BitsDownCast<1> for Reg64Bits<35> {}
impl Reg64BitsConcat<1, 36> for Reg64Bits<35> {}
impl Reg64BitsDownCast<2> for Reg64Bits<35> {}
impl Reg64BitsConcat<2, 37> for Reg64Bits<35> {}
impl Reg64BitsDownCast<3> for Reg64Bits<35> {}
impl Reg64BitsConcat<3, 38> for Reg64Bits<35> {}
impl Reg64BitsDownCast<4> for Reg64Bits<35> {}
impl Reg64BitsConcat<4, 39> for Reg64Bits<35> {}
impl Reg64BitsDownCast<5> for Reg64Bits<35> {}
impl Reg64BitsConcat<5, 40> for Reg64Bits<35> {}
impl Reg64BitsDownCast<6> for Reg64Bits<35> {}
impl Reg64BitsConcat<6, 41> for Reg64Bits<35> {}
impl Reg64BitsDownCast<7> for Reg64Bits<35> {}
impl Reg64BitsConcat<7, 42> for Reg64Bits<35> {}
impl Reg64BitsDownCast<8> for Reg64Bits<35> {}
impl Reg64BitsConcat<8, 43> for Reg64Bits<35> {}
impl Reg64BitsDownCast<9> for Reg64Bits<35> {}
impl Reg64BitsConcat<9, 44> for Reg64Bits<35> {}
impl Reg64BitsDownCast<10> for Reg64Bits<35> {}
impl Reg64BitsConcat<10, 45> for Reg64Bits<35> {}
impl Reg64BitsDownCast<11> for Reg64Bits<35> {}
impl Reg64BitsConcat<11, 46> for Reg64Bits<35> {}
impl Reg64BitsDownCast<12> for Reg64Bits<35> {}
impl Reg64BitsConcat<12, 47> for Reg64Bits<35> {}
impl Reg64BitsDownCast<13> for Reg64Bits<35> {}
impl Reg64BitsConcat<13, 48> for Reg64Bits<35> {}
impl Reg64BitsDownCast<14> for Reg64Bits<35> {}
impl Reg64BitsConcat<14, 49> for Reg64Bits<35> {}
impl Reg64BitsDownCast<15> for Reg64Bits<35> {}
impl Reg64BitsConcat<15, 50> for Reg64Bits<35> {}
impl Reg64BitsDownCast<16> for Reg64Bits<35> {}
impl Reg64BitsConcat<16, 51> for Reg64Bits<35> {}
impl Reg64BitsDownCast<17> for Reg64Bits<35> {}
impl Reg64BitsConcat<17, 52> for Reg64Bits<35> {}
impl Reg64BitsDownCast<18> for Reg64Bits<35> {}
impl Reg64BitsConcat<18, 53> for Reg64Bits<35> {}
impl Reg64BitsDownCast<19> for Reg64Bits<35> {}
impl Reg64BitsConcat<19, 54> for Reg64Bits<35> {}
impl Reg64BitsDownCast<20> for Reg64Bits<35> {}
impl Reg64BitsConcat<20, 55> for Reg64Bits<35> {}
impl Reg64BitsDownCast<21> for Reg64Bits<35> {}
impl Reg64BitsConcat<21, 56> for Reg64Bits<35> {}
impl Reg64BitsDownCast<22> for Reg64Bits<35> {}
impl Reg64BitsConcat<22, 57> for Reg64Bits<35> {}
impl Reg64BitsDownCast<23> for Reg64Bits<35> {}
impl Reg64BitsConcat<23, 58> for Reg64Bits<35> {}
impl Reg64BitsDownCast<24> for Reg64Bits<35> {}
impl Reg64BitsConcat<24, 59> for Reg64Bits<35> {}
impl Reg64BitsDownCast<25> for Reg64Bits<35> {}
impl Reg64BitsConcat<25, 60> for Reg64Bits<35> {}
impl Reg64BitsDownCast<26> for Reg64Bits<35> {}
impl Reg64BitsConcat<26, 61> for Reg64Bits<35> {}
impl Reg64BitsDownCast<27> for Reg64Bits<35> {}
impl Reg64BitsConcat<27, 62> for Reg64Bits<35> {}
impl Reg64BitsDownCast<28> for Reg64Bits<35> {}
impl Reg64BitsConcat<28, 63> for Reg64Bits<35> {}
impl Reg64BitsDownCast<29> for Reg64Bits<35> {}
impl Reg64BitsConcat<29, 64> for Reg64Bits<35> {}
impl Reg64BitsDownCast<30> for Reg64Bits<35> {}
impl Reg64BitsDownCast<31> for Reg64Bits<35> {}
impl Reg64BitsDownCast<32> for Reg64Bits<35> {}
impl Reg64BitsDownCast<33> for Reg64Bits<35> {}
impl Reg64BitsDownCast<34> for Reg64Bits<35> {}
impl Reg64BitsDownCast<35> for Reg64Bits<35> {}
impl Reg64BitsDownCast<1> for Reg64Bits<36> {}
impl Reg64BitsConcat<1, 37> for Reg64Bits<36> {}
impl Reg64BitsDownCast<2> for Reg64Bits<36> {}
impl Reg64BitsConcat<2, 38> for Reg64Bits<36> {}
impl Reg64BitsDownCast<3> for Reg64Bits<36> {}
impl Reg64BitsConcat<3, 39> for Reg64Bits<36> {}
impl Reg64BitsDownCast<4> for Reg64Bits<36> {}
impl Reg64BitsConcat<4, 40> for Reg64Bits<36> {}
impl Reg64BitsDownCast<5> for Reg64Bits<36> {}
impl Reg64BitsConcat<5, 41> for Reg64Bits<36> {}
impl Reg64BitsDownCast<6> for Reg64Bits<36> {}
impl Reg64BitsConcat<6, 42> for Reg64Bits<36> {}
impl Reg64BitsDownCast<7> for Reg64Bits<36> {}
impl Reg64BitsConcat<7, 43> for Reg64Bits<36> {}
impl Reg64BitsDownCast<8> for Reg64Bits<36> {}
impl Reg64BitsConcat<8, 44> for Reg64Bits<36> {}
impl Reg64BitsDownCast<9> for Reg64Bits<36> {}
impl Reg64BitsConcat<9, 45> for Reg64Bits<36> {}
impl Reg64BitsDownCast<10> for Reg64Bits<36> {}
impl Reg64BitsConcat<10, 46> for Reg64Bits<36> {}
impl Reg64BitsDownCast<11> for Reg64Bits<36> {}
impl Reg64BitsConcat<11, 47> for Reg64Bits<36> {}
impl Reg64BitsDownCast<12> for Reg64Bits<36> {}
impl Reg64BitsConcat<12, 48> for Reg64Bits<36> {}
impl Reg64BitsDownCast<13> for Reg64Bits<36> {}
impl Reg64BitsConcat<13, 49> for Reg64Bits<36> {}
impl Reg64BitsDownCast<14> for Reg64Bits<36> {}
impl Reg64BitsConcat<14, 50> for Reg64Bits<36> {}
impl Reg64BitsDownCast<15> for Reg64Bits<36> {}
impl Reg64BitsConcat<15, 51> for Reg64Bits<36> {}
impl Reg64BitsDownCast<16> for Reg64Bits<36> {}
impl Reg64BitsConcat<16, 52> for Reg64Bits<36> {}
impl Reg64BitsDownCast<17> for Reg64Bits<36> {}
impl Reg64BitsConcat<17, 53> for Reg64Bits<36> {}
impl Reg64BitsDownCast<18> for Reg64Bits<36> {}
impl Reg64BitsConcat<18, 54> for Reg64Bits<36> {}
impl Reg64BitsDownCast<19> for Reg64Bits<36> {}
impl Reg64BitsConcat<19, 55> for Reg64Bits<36> {}
impl Reg64BitsDownCast<20> for Reg64Bits<36> {}
impl Reg64BitsConcat<20, 56> for Reg64Bits<36> {}
impl Reg64BitsDownCast<21> for Reg64Bits<36> {}
impl Reg64BitsConcat<21, 57> for Reg64Bits<36> {}
impl Reg64BitsDownCast<22> for Reg64Bits<36> {}
impl Reg64BitsConcat<22, 58> for Reg64Bits<36> {}
impl Reg64BitsDownCast<23> for Reg64Bits<36> {}
impl Reg64BitsConcat<23, 59> for Reg64Bits<36> {}
impl Reg64BitsDownCast<24> for Reg64Bits<36> {}
impl Reg64BitsConcat<24, 60> for Reg64Bits<36> {}
impl Reg64BitsDownCast<25> for Reg64Bits<36> {}
impl Reg64BitsConcat<25, 61> for Reg64Bits<36> {}
impl Reg64BitsDownCast<26> for Reg64Bits<36> {}
impl Reg64BitsConcat<26, 62> for Reg64Bits<36> {}
impl Reg64BitsDownCast<27> for Reg64Bits<36> {}
impl Reg64BitsConcat<27, 63> for Reg64Bits<36> {}
impl Reg64BitsDownCast<28> for Reg64Bits<36> {}
impl Reg64BitsConcat<28, 64> for Reg64Bits<36> {}
impl Reg64BitsDownCast<29> for Reg64Bits<36> {}
impl Reg64BitsDownCast<30> for Reg64Bits<36> {}
impl Reg64BitsDownCast<31> for Reg64Bits<36> {}
impl Reg64BitsDownCast<32> for Reg64Bits<36> {}
impl Reg64BitsDownCast<33> for Reg64Bits<36> {}
impl Reg64BitsDownCast<34> for Reg64Bits<36> {}
impl Reg64BitsDownCast<35> for Reg64Bits<36> {}
impl Reg64BitsDownCast<36> for Reg64Bits<36> {}
impl Reg64BitsDownCast<1> for Reg64Bits<37> {}
impl Reg64BitsConcat<1, 38> for Reg64Bits<37> {}
impl Reg64BitsDownCast<2> for Reg64Bits<37> {}
impl Reg64BitsConcat<2, 39> for Reg64Bits<37> {}
impl Reg64BitsDownCast<3> for Reg64Bits<37> {}
impl Reg64BitsConcat<3, 40> for Reg64Bits<37> {}
impl Reg64BitsDownCast<4> for Reg64Bits<37> {}
impl Reg64BitsConcat<4, 41> for Reg64Bits<37> {}
impl Reg64BitsDownCast<5> for Reg64Bits<37> {}
impl Reg64BitsConcat<5, 42> for Reg64Bits<37> {}
impl Reg64BitsDownCast<6> for Reg64Bits<37> {}
impl Reg64BitsConcat<6, 43> for Reg64Bits<37> {}
impl Reg64BitsDownCast<7> for Reg64Bits<37> {}
impl Reg64BitsConcat<7, 44> for Reg64Bits<37> {}
impl Reg64BitsDownCast<8> for Reg64Bits<37> {}
impl Reg64BitsConcat<8, 45> for Reg64Bits<37> {}
impl Reg64BitsDownCast<9> for Reg64Bits<37> {}
impl Reg64BitsConcat<9, 46> for Reg64Bits<37> {}
impl Reg64BitsDownCast<10> for Reg64Bits<37> {}
impl Reg64BitsConcat<10, 47> for Reg64Bits<37> {}
impl Reg64BitsDownCast<11> for Reg64Bits<37> {}
impl Reg64BitsConcat<11, 48> for Reg64Bits<37> {}
impl Reg64BitsDownCast<12> for Reg64Bits<37> {}
impl Reg64BitsConcat<12, 49> for Reg64Bits<37> {}
impl Reg64BitsDownCast<13> for Reg64Bits<37> {}
impl Reg64BitsConcat<13, 50> for Reg64Bits<37> {}
impl Reg64BitsDownCast<14> for Reg64Bits<37> {}
impl Reg64BitsConcat<14, 51> for Reg64Bits<37> {}
impl Reg64BitsDownCast<15> for Reg64Bits<37> {}
impl Reg64BitsConcat<15, 52> for Reg64Bits<37> {}
impl Reg64BitsDownCast<16> for Reg64Bits<37> {}
impl Reg64BitsConcat<16, 53> for Reg64Bits<37> {}
impl Reg64BitsDownCast<17> for Reg64Bits<37> {}
impl Reg64BitsConcat<17, 54> for Reg64Bits<37> {}
impl Reg64BitsDownCast<18> for Reg64Bits<37> {}
impl Reg64BitsConcat<18, 55> for Reg64Bits<37> {}
impl Reg64BitsDownCast<19> for Reg64Bits<37> {}
impl Reg64BitsConcat<19, 56> for Reg64Bits<37> {}
impl Reg64BitsDownCast<20> for Reg64Bits<37> {}
impl Reg64BitsConcat<20, 57> for Reg64Bits<37> {}
impl Reg64BitsDownCast<21> for Reg64Bits<37> {}
impl Reg64BitsConcat<21, 58> for Reg64Bits<37> {}
impl Reg64BitsDownCast<22> for Reg64Bits<37> {}
impl Reg64BitsConcat<22, 59> for Reg64Bits<37> {}
impl Reg64BitsDownCast<23> for Reg64Bits<37> {}
impl Reg64BitsConcat<23, 60> for Reg64Bits<37> {}
impl Reg64BitsDownCast<24> for Reg64Bits<37> {}
impl Reg64BitsConcat<24, 61> for Reg64Bits<37> {}
impl Reg64BitsDownCast<25> for Reg64Bits<37> {}
impl Reg64BitsConcat<25, 62> for Reg64Bits<37> {}
impl Reg64BitsDownCast<26> for Reg64Bits<37> {}
impl Reg64BitsConcat<26, 63> for Reg64Bits<37> {}
impl Reg64BitsDownCast<27> for Reg64Bits<37> {}
impl Reg64BitsConcat<27, 64> for Reg64Bits<37> {}
impl Reg64BitsDownCast<28> for Reg64Bits<37> {}
impl Reg64BitsDownCast<29> for Reg64Bits<37> {}
impl Reg64BitsDownCast<30> for Reg64Bits<37> {}
impl Reg64BitsDownCast<31> for Reg64Bits<37> {}
impl Reg64BitsDownCast<32> for Reg64Bits<37> {}
impl Reg64BitsDownCast<33> for Reg64Bits<37> {}
impl Reg64BitsDownCast<34> for Reg64Bits<37> {}
impl Reg64BitsDownCast<35> for Reg64Bits<37> {}
impl Reg64BitsDownCast<36> for Reg64Bits<37> {}
impl Reg64BitsDownCast<37> for Reg64Bits<37> {}
impl Reg64BitsDownCast<1> for Reg64Bits<38> {}
impl Reg64BitsConcat<1, 39> for Reg64Bits<38> {}
impl Reg64BitsDownCast<2> for Reg64Bits<38> {}
impl Reg64BitsConcat<2, 40> for Reg64Bits<38> {}
impl Reg64BitsDownCast<3> for Reg64Bits<38> {}
impl Reg64BitsConcat<3, 41> for Reg64Bits<38> {}
impl Reg64BitsDownCast<4> for Reg64Bits<38> {}
impl Reg64BitsConcat<4, 42> for Reg64Bits<38> {}
impl Reg64BitsDownCast<5> for Reg64Bits<38> {}
impl Reg64BitsConcat<5, 43> for Reg64Bits<38> {}
impl Reg64BitsDownCast<6> for Reg64Bits<38> {}
impl Reg64BitsConcat<6, 44> for Reg64Bits<38> {}
impl Reg64BitsDownCast<7> for Reg64Bits<38> {}
impl Reg64BitsConcat<7, 45> for Reg64Bits<38> {}
impl Reg64BitsDownCast<8> for Reg64Bits<38> {}
impl Reg64BitsConcat<8, 46> for Reg64Bits<38> {}
impl Reg64BitsDownCast<9> for Reg64Bits<38> {}
impl Reg64BitsConcat<9, 47> for Reg64Bits<38> {}
impl Reg64BitsDownCast<10> for Reg64Bits<38> {}
impl Reg64BitsConcat<10, 48> for Reg64Bits<38> {}
impl Reg64BitsDownCast<11> for Reg64Bits<38> {}
impl Reg64BitsConcat<11, 49> for Reg64Bits<38> {}
impl Reg64BitsDownCast<12> for Reg64Bits<38> {}
impl Reg64BitsConcat<12, 50> for Reg64Bits<38> {}
impl Reg64BitsDownCast<13> for Reg64Bits<38> {}
impl Reg64BitsConcat<13, 51> for Reg64Bits<38> {}
impl Reg64BitsDownCast<14> for Reg64Bits<38> {}
impl Reg64BitsConcat<14, 52> for Reg64Bits<38> {}
impl Reg64BitsDownCast<15> for Reg64Bits<38> {}
impl Reg64BitsConcat<15, 53> for Reg64Bits<38> {}
impl Reg64BitsDownCast<16> for Reg64Bits<38> {}
impl Reg64BitsConcat<16, 54> for Reg64Bits<38> {}
impl Reg64BitsDownCast<17> for Reg64Bits<38> {}
impl Reg64BitsConcat<17, 55> for Reg64Bits<38> {}
impl Reg64BitsDownCast<18> for Reg64Bits<38> {}
impl Reg64BitsConcat<18, 56> for Reg64Bits<38> {}
impl Reg64BitsDownCast<19> for Reg64Bits<38> {}
impl Reg64BitsConcat<19, 57> for Reg64Bits<38> {}
impl Reg64BitsDownCast<20> for Reg64Bits<38> {}
impl Reg64BitsConcat<20, 58> for Reg64Bits<38> {}
impl Reg64BitsDownCast<21> for Reg64Bits<38> {}
impl Reg64BitsConcat<21, 59> for Reg64Bits<38> {}
impl Reg64BitsDownCast<22> for Reg64Bits<38> {}
impl Reg64BitsConcat<22, 60> for Reg64Bits<38> {}
impl Reg64BitsDownCast<23> for Reg64Bits<38> {}
impl Reg64BitsConcat<23, 61> for Reg64Bits<38> {}
impl Reg64BitsDownCast<24> for Reg64Bits<38> {}
impl Reg64BitsConcat<24, 62> for Reg64Bits<38> {}
impl Reg64BitsDownCast<25> for Reg64Bits<38> {}
impl Reg64BitsConcat<25, 63> for Reg64Bits<38> {}
impl Reg64BitsDownCast<26> for Reg64Bits<38> {}
impl Reg64BitsConcat<26, 64> for Reg64Bits<38> {}
impl Reg64BitsDownCast<27> for Reg64Bits<38> {}
impl Reg64BitsDownCast<28> for Reg64Bits<38> {}
impl Reg64BitsDownCast<29> for Reg64Bits<38> {}
impl Reg64BitsDownCast<30> for Reg64Bits<38> {}
impl Reg64BitsDownCast<31> for Reg64Bits<38> {}
impl Reg64BitsDownCast<32> for Reg64Bits<38> {}
impl Reg64BitsDownCast<33> for Reg64Bits<38> {}
impl Reg64BitsDownCast<34> for Reg64Bits<38> {}
impl Reg64BitsDownCast<35> for Reg64Bits<38> {}
impl Reg64BitsDownCast<36> for Reg64Bits<38> {}
impl Reg64BitsDownCast<37> for Reg64Bits<38> {}
impl Reg64BitsDownCast<38> for Reg64Bits<38> {}
impl Reg64BitsDownCast<1> for Reg64Bits<39> {}
impl Reg64BitsConcat<1, 40> for Reg64Bits<39> {}
impl Reg64BitsDownCast<2> for Reg64Bits<39> {}
impl Reg64BitsConcat<2, 41> for Reg64Bits<39> {}
impl Reg64BitsDownCast<3> for Reg64Bits<39> {}
impl Reg64BitsConcat<3, 42> for Reg64Bits<39> {}
impl Reg64BitsDownCast<4> for Reg64Bits<39> {}
impl Reg64BitsConcat<4, 43> for Reg64Bits<39> {}
impl Reg64BitsDownCast<5> for Reg64Bits<39> {}
impl Reg64BitsConcat<5, 44> for Reg64Bits<39> {}
impl Reg64BitsDownCast<6> for Reg64Bits<39> {}
impl Reg64BitsConcat<6, 45> for Reg64Bits<39> {}
impl Reg64BitsDownCast<7> for Reg64Bits<39> {}
impl Reg64BitsConcat<7, 46> for Reg64Bits<39> {}
impl Reg64BitsDownCast<8> for Reg64Bits<39> {}
impl Reg64BitsConcat<8, 47> for Reg64Bits<39> {}
impl Reg64BitsDownCast<9> for Reg64Bits<39> {}
impl Reg64BitsConcat<9, 48> for Reg64Bits<39> {}
impl Reg64BitsDownCast<10> for Reg64Bits<39> {}
impl Reg64BitsConcat<10, 49> for Reg64Bits<39> {}
impl Reg64BitsDownCast<11> for Reg64Bits<39> {}
impl Reg64BitsConcat<11, 50> for Reg64Bits<39> {}
impl Reg64BitsDownCast<12> for Reg64Bits<39> {}
impl Reg64BitsConcat<12, 51> for Reg64Bits<39> {}
impl Reg64BitsDownCast<13> for Reg64Bits<39> {}
impl Reg64BitsConcat<13, 52> for Reg64Bits<39> {}
impl Reg64BitsDownCast<14> for Reg64Bits<39> {}
impl Reg64BitsConcat<14, 53> for Reg64Bits<39> {}
impl Reg64BitsDownCast<15> for Reg64Bits<39> {}
impl Reg64BitsConcat<15, 54> for Reg64Bits<39> {}
impl Reg64BitsDownCast<16> for Reg64Bits<39> {}
impl Reg64BitsConcat<16, 55> for Reg64Bits<39> {}
impl Reg64BitsDownCast<17> for Reg64Bits<39> {}
impl Reg64BitsConcat<17, 56> for Reg64Bits<39> {}
impl Reg64BitsDownCast<18> for Reg64Bits<39> {}
impl Reg64BitsConcat<18, 57> for Reg64Bits<39> {}
impl Reg64BitsDownCast<19> for Reg64Bits<39> {}
impl Reg64BitsConcat<19, 58> for Reg64Bits<39> {}
impl Reg64BitsDownCast<20> for Reg64Bits<39> {}
impl Reg64BitsConcat<20, 59> for Reg64Bits<39> {}
impl Reg64BitsDownCast<21> for Reg64Bits<39> {}
impl Reg64BitsConcat<21, 60> for Reg64Bits<39> {}
impl Reg64BitsDownCast<22> for Reg64Bits<39> {}
impl Reg64BitsConcat<22, 61> for Reg64Bits<39> {}
impl Reg64BitsDownCast<23> for Reg64Bits<39> {}
impl Reg64BitsConcat<23, 62> for Reg64Bits<39> {}
impl Reg64BitsDownCast<24> for Reg64Bits<39> {}
impl Reg64BitsConcat<24, 63> for Reg64Bits<39> {}
impl Reg64BitsDownCast<25> for Reg64Bits<39> {}
impl Reg64BitsConcat<25, 64> for Reg64Bits<39> {}
impl Reg64BitsDownCast<26> for Reg64Bits<39> {}
impl Reg64BitsDownCast<27> for Reg64Bits<39> {}
impl Reg64BitsDownCast<28> for Reg64Bits<39> {}
impl Reg64BitsDownCast<29> for Reg64Bits<39> {}
impl Reg64BitsDownCast<30> for Reg64Bits<39> {}
impl Reg64BitsDownCast<31> for Reg64Bits<39> {}
impl Reg64BitsDownCast<32> for Reg64Bits<39> {}
impl Reg64BitsDownCast<33> for Reg64Bits<39> {}
impl Reg64BitsDownCast<34> for Reg64Bits<39> {}
impl Reg64BitsDownCast<35> for Reg64Bits<39> {}
impl Reg64BitsDownCast<36> for Reg64Bits<39> {}
impl Reg64BitsDownCast<37> for Reg64Bits<39> {}
impl Reg64BitsDownCast<38> for Reg64Bits<39> {}
impl Reg64BitsDownCast<39> for Reg64Bits<39> {}
impl Reg64BitsDownCast<1> for Reg64Bits<40> {}
impl Reg64BitsConcat<1, 41> for Reg64Bits<40> {}
impl Reg64BitsDownCast<2> for Reg64Bits<40> {}
impl Reg64BitsConcat<2, 42> for Reg64Bits<40> {}
impl Reg64BitsDownCast<3> for Reg64Bits<40> {}
impl Reg64BitsConcat<3, 43> for Reg64Bits<40> {}
impl Reg64BitsDownCast<4> for Reg64Bits<40> {}
impl Reg64BitsConcat<4, 44> for Reg64Bits<40> {}
impl Reg64BitsDownCast<5> for Reg64Bits<40> {}
impl Reg64BitsConcat<5, 45> for Reg64Bits<40> {}
impl Reg64BitsDownCast<6> for Reg64Bits<40> {}
impl Reg64BitsConcat<6, 46> for Reg64Bits<40> {}
impl Reg64BitsDownCast<7> for Reg64Bits<40> {}
impl Reg64BitsConcat<7, 47> for Reg64Bits<40> {}
impl Reg64BitsDownCast<8> for Reg64Bits<40> {}
impl Reg64BitsConcat<8, 48> for Reg64Bits<40> {}
impl Reg64BitsDownCast<9> for Reg64Bits<40> {}
impl Reg64BitsConcat<9, 49> for Reg64Bits<40> {}
impl Reg64BitsDownCast<10> for Reg64Bits<40> {}
impl Reg64BitsConcat<10, 50> for Reg64Bits<40> {}
impl Reg64BitsDownCast<11> for Reg64Bits<40> {}
impl Reg64BitsConcat<11, 51> for Reg64Bits<40> {}
impl Reg64BitsDownCast<12> for Reg64Bits<40> {}
impl Reg64BitsConcat<12, 52> for Reg64Bits<40> {}
impl Reg64BitsDownCast<13> for Reg64Bits<40> {}
impl Reg64BitsConcat<13, 53> for Reg64Bits<40> {}
impl Reg64BitsDownCast<14> for Reg64Bits<40> {}
impl Reg64BitsConcat<14, 54> for Reg64Bits<40> {}
impl Reg64BitsDownCast<15> for Reg64Bits<40> {}
impl Reg64BitsConcat<15, 55> for Reg64Bits<40> {}
impl Reg64BitsDownCast<16> for Reg64Bits<40> {}
impl Reg64BitsConcat<16, 56> for Reg64Bits<40> {}
impl Reg64BitsDownCast<17> for Reg64Bits<40> {}
impl Reg64BitsConcat<17, 57> for Reg64Bits<40> {}
impl Reg64BitsDownCast<18> for Reg64Bits<40> {}
impl Reg64BitsConcat<18, 58> for Reg64Bits<40> {}
impl Reg64BitsDownCast<19> for Reg64Bits<40> {}
impl Reg64BitsConcat<19, 59> for Reg64Bits<40> {}
impl Reg64BitsDownCast<20> for Reg64Bits<40> {}
impl Reg64BitsConcat<20, 60> for Reg64Bits<40> {}
impl Reg64BitsDownCast<21> for Reg64Bits<40> {}
impl Reg64BitsConcat<21, 61> for Reg64Bits<40> {}
impl Reg64BitsDownCast<22> for Reg64Bits<40> {}
impl Reg64BitsConcat<22, 62> for Reg64Bits<40> {}
impl Reg64BitsDownCast<23> for Reg64Bits<40> {}
impl Reg64BitsConcat<23, 63> for Reg64Bits<40> {}
impl Reg64BitsDownCast<24> for Reg64Bits<40> {}
impl Reg64BitsConcat<24, 64> for Reg64Bits<40> {}
impl Reg64BitsDownCast<25> for Reg64Bits<40> {}
impl Reg64BitsDownCast<26> for Reg64Bits<40> {}
impl Reg64BitsDownCast<27> for Reg64Bits<40> {}
impl Reg64BitsDownCast<28> for Reg64Bits<40> {}
impl Reg64BitsDownCast<29> for Reg64Bits<40> {}
impl Reg64BitsDownCast<30> for Reg64Bits<40> {}
impl Reg64BitsDownCast<31> for Reg64Bits<40> {}
impl Reg64BitsDownCast<32> for Reg64Bits<40> {}
impl Reg64BitsDownCast<33> for Reg64Bits<40> {}
impl Reg64BitsDownCast<34> for Reg64Bits<40> {}
impl Reg64BitsDownCast<35> for Reg64Bits<40> {}
impl Reg64BitsDownCast<36> for Reg64Bits<40> {}
impl Reg64BitsDownCast<37> for Reg64Bits<40> {}
impl Reg64BitsDownCast<38> for Reg64Bits<40> {}
impl Reg64BitsDownCast<39> for Reg64Bits<40> {}
impl Reg64BitsDownCast<40> for Reg64Bits<40> {}
impl Reg64BitsDownCast<1> for Reg64Bits<41> {}
impl Reg64BitsConcat<1, 42> for Reg64Bits<41> {}
impl Reg64BitsDownCast<2> for Reg64Bits<41> {}
impl Reg64BitsConcat<2, 43> for Reg64Bits<41> {}
impl Reg64BitsDownCast<3> for Reg64Bits<41> {}
impl Reg64BitsConcat<3, 44> for Reg64Bits<41> {}
impl Reg64BitsDownCast<4> for Reg64Bits<41> {}
impl Reg64BitsConcat<4, 45> for Reg64Bits<41> {}
impl Reg64BitsDownCast<5> for Reg64Bits<41> {}
impl Reg64BitsConcat<5, 46> for Reg64Bits<41> {}
impl Reg64BitsDownCast<6> for Reg64Bits<41> {}
impl Reg64BitsConcat<6, 47> for Reg64Bits<41> {}
impl Reg64BitsDownCast<7> for Reg64Bits<41> {}
impl Reg64BitsConcat<7, 48> for Reg64Bits<41> {}
impl Reg64BitsDownCast<8> for Reg64Bits<41> {}
impl Reg64BitsConcat<8, 49> for Reg64Bits<41> {}
impl Reg64BitsDownCast<9> for Reg64Bits<41> {}
impl Reg64BitsConcat<9, 50> for Reg64Bits<41> {}
impl Reg64BitsDownCast<10> for Reg64Bits<41> {}
impl Reg64BitsConcat<10, 51> for Reg64Bits<41> {}
impl Reg64BitsDownCast<11> for Reg64Bits<41> {}
impl Reg64BitsConcat<11, 52> for Reg64Bits<41> {}
impl Reg64BitsDownCast<12> for Reg64Bits<41> {}
impl Reg64BitsConcat<12, 53> for Reg64Bits<41> {}
impl Reg64BitsDownCast<13> for Reg64Bits<41> {}
impl Reg64BitsConcat<13, 54> for Reg64Bits<41> {}
impl Reg64BitsDownCast<14> for Reg64Bits<41> {}
impl Reg64BitsConcat<14, 55> for Reg64Bits<41> {}
impl Reg64BitsDownCast<15> for Reg64Bits<41> {}
impl Reg64BitsConcat<15, 56> for Reg64Bits<41> {}
impl Reg64BitsDownCast<16> for Reg64Bits<41> {}
impl Reg64BitsConcat<16, 57> for Reg64Bits<41> {}
impl Reg64BitsDownCast<17> for Reg64Bits<41> {}
impl Reg64BitsConcat<17, 58> for Reg64Bits<41> {}
impl Reg64BitsDownCast<18> for Reg64Bits<41> {}
impl Reg64BitsConcat<18, 59> for Reg64Bits<41> {}
impl Reg64BitsDownCast<19> for Reg64Bits<41> {}
impl Reg64BitsConcat<19, 60> for Reg64Bits<41> {}
impl Reg64BitsDownCast<20> for Reg64Bits<41> {}
impl Reg64BitsConcat<20, 61> for Reg64Bits<41> {}
impl Reg64BitsDownCast<21> for Reg64Bits<41> {}
impl Reg64BitsConcat<21, 62> for Reg64Bits<41> {}
impl Reg64BitsDownCast<22> for Reg64Bits<41> {}
impl Reg64BitsConcat<22, 63> for Reg64Bits<41> {}
impl Reg64BitsDownCast<23> for Reg64Bits<41> {}
impl Reg64BitsConcat<23, 64> for Reg64Bits<41> {}
impl Reg64BitsDownCast<24> for Reg64Bits<41> {}
impl Reg64BitsDownCast<25> for Reg64Bits<41> {}
impl Reg64BitsDownCast<26> for Reg64Bits<41> {}
impl Reg64BitsDownCast<27> for Reg64Bits<41> {}
impl Reg64BitsDownCast<28> for Reg64Bits<41> {}
impl Reg64BitsDownCast<29> for Reg64Bits<41> {}
impl Reg64BitsDownCast<30> for Reg64Bits<41> {}
impl Reg64BitsDownCast<31> for Reg64Bits<41> {}
impl Reg64BitsDownCast<32> for Reg64Bits<41> {}
impl Reg64BitsDownCast<33> for Reg64Bits<41> {}
impl Reg64BitsDownCast<34> for Reg64Bits<41> {}
impl Reg64BitsDownCast<35> for Reg64Bits<41> {}
impl Reg64BitsDownCast<36> for Reg64Bits<41> {}
impl Reg64BitsDownCast<37> for Reg64Bits<41> {}
impl Reg64BitsDownCast<38> for Reg64Bits<41> {}
impl Reg64BitsDownCast<39> for Reg64Bits<41> {}
impl Reg64BitsDownCast<40> for Reg64Bits<41> {}
impl Reg64BitsDownCast<41> for Reg64Bits<41> {}
impl Reg64BitsDownCast<1> for Reg64Bits<42> {}
impl Reg64BitsConcat<1, 43> for Reg64Bits<42> {}
impl Reg64BitsDownCast<2> for Reg64Bits<42> {}
impl Reg64BitsConcat<2, 44> for Reg64Bits<42> {}
impl Reg64BitsDownCast<3> for Reg64Bits<42> {}
impl Reg64BitsConcat<3, 45> for Reg64Bits<42> {}
impl Reg64BitsDownCast<4> for Reg64Bits<42> {}
impl Reg64BitsConcat<4, 46> for Reg64Bits<42> {}
impl Reg64BitsDownCast<5> for Reg64Bits<42> {}
impl Reg64BitsConcat<5, 47> for Reg64Bits<42> {}
impl Reg64BitsDownCast<6> for Reg64Bits<42> {}
impl Reg64BitsConcat<6, 48> for Reg64Bits<42> {}
impl Reg64BitsDownCast<7> for Reg64Bits<42> {}
impl Reg64BitsConcat<7, 49> for Reg64Bits<42> {}
impl Reg64BitsDownCast<8> for Reg64Bits<42> {}
impl Reg64BitsConcat<8, 50> for Reg64Bits<42> {}
impl Reg64BitsDownCast<9> for Reg64Bits<42> {}
impl Reg64BitsConcat<9, 51> for Reg64Bits<42> {}
impl Reg64BitsDownCast<10> for Reg64Bits<42> {}
impl Reg64BitsConcat<10, 52> for Reg64Bits<42> {}
impl Reg64BitsDownCast<11> for Reg64Bits<42> {}
impl Reg64BitsConcat<11, 53> for Reg64Bits<42> {}
impl Reg64BitsDownCast<12> for Reg64Bits<42> {}
impl Reg64BitsConcat<12, 54> for Reg64Bits<42> {}
impl Reg64BitsDownCast<13> for Reg64Bits<42> {}
impl Reg64BitsConcat<13, 55> for Reg64Bits<42> {}
impl Reg64BitsDownCast<14> for Reg64Bits<42> {}
impl Reg64BitsConcat<14, 56> for Reg64Bits<42> {}
impl Reg64BitsDownCast<15> for Reg64Bits<42> {}
impl Reg64BitsConcat<15, 57> for Reg64Bits<42> {}
impl Reg64BitsDownCast<16> for Reg64Bits<42> {}
impl Reg64BitsConcat<16, 58> for Reg64Bits<42> {}
impl Reg64BitsDownCast<17> for Reg64Bits<42> {}
impl Reg64BitsConcat<17, 59> for Reg64Bits<42> {}
impl Reg64BitsDownCast<18> for Reg64Bits<42> {}
impl Reg64BitsConcat<18, 60> for Reg64Bits<42> {}
impl Reg64BitsDownCast<19> for Reg64Bits<42> {}
impl Reg64BitsConcat<19, 61> for Reg64Bits<42> {}
impl Reg64BitsDownCast<20> for Reg64Bits<42> {}
impl Reg64BitsConcat<20, 62> for Reg64Bits<42> {}
impl Reg64BitsDownCast<21> for Reg64Bits<42> {}
impl Reg64BitsConcat<21, 63> for Reg64Bits<42> {}
impl Reg64BitsDownCast<22> for Reg64Bits<42> {}
impl Reg64BitsConcat<22, 64> for Reg64Bits<42> {}
impl Reg64BitsDownCast<23> for Reg64Bits<42> {}
impl Reg64BitsDownCast<24> for Reg64Bits<42> {}
impl Reg64BitsDownCast<25> for Reg64Bits<42> {}
impl Reg64BitsDownCast<26> for Reg64Bits<42> {}
impl Reg64BitsDownCast<27> for Reg64Bits<42> {}
impl Reg64BitsDownCast<28> for Reg64Bits<42> {}
impl Reg64BitsDownCast<29> for Reg64Bits<42> {}
impl Reg64BitsDownCast<30> for Reg64Bits<42> {}
impl Reg64BitsDownCast<31> for Reg64Bits<42> {}
impl Reg64BitsDownCast<32> for Reg64Bits<42> {}
impl Reg64BitsDownCast<33> for Reg64Bits<42> {}
impl Reg64BitsDownCast<34> for Reg64Bits<42> {}
impl Reg64BitsDownCast<35> for Reg64Bits<42> {}
impl Reg64BitsDownCast<36> for Reg64Bits<42> {}
impl Reg64BitsDownCast<37> for Reg64Bits<42> {}
impl Reg64BitsDownCast<38> for Reg64Bits<42> {}
impl Reg64BitsDownCast<39> for Reg64Bits<42> {}
impl Reg64BitsDownCast<40> for Reg64Bits<42> {}
impl Reg64BitsDownCast<41> for Reg64Bits<42> {}
impl Reg64BitsDownCast<42> for Reg64Bits<42> {}
impl Reg64BitsDownCast<1> for Reg64Bits<43> {}
impl Reg64BitsConcat<1, 44> for Reg64Bits<43> {}
impl Reg64BitsDownCast<2> for Reg64Bits<43> {}
impl Reg64BitsConcat<2, 45> for Reg64Bits<43> {}
impl Reg64BitsDownCast<3> for Reg64Bits<43> {}
impl Reg64BitsConcat<3, 46> for Reg64Bits<43> {}
impl Reg64BitsDownCast<4> for Reg64Bits<43> {}
impl Reg64BitsConcat<4, 47> for Reg64Bits<43> {}
impl Reg64BitsDownCast<5> for Reg64Bits<43> {}
impl Reg64BitsConcat<5, 48> for Reg64Bits<43> {}
impl Reg64BitsDownCast<6> for Reg64Bits<43> {}
impl Reg64BitsConcat<6, 49> for Reg64Bits<43> {}
impl Reg64BitsDownCast<7> for Reg64Bits<43> {}
impl Reg64BitsConcat<7, 50> for Reg64Bits<43> {}
impl Reg64BitsDownCast<8> for Reg64Bits<43> {}
impl Reg64BitsConcat<8, 51> for Reg64Bits<43> {}
impl Reg64BitsDownCast<9> for Reg64Bits<43> {}
impl Reg64BitsConcat<9, 52> for Reg64Bits<43> {}
impl Reg64BitsDownCast<10> for Reg64Bits<43> {}
impl Reg64BitsConcat<10, 53> for Reg64Bits<43> {}
impl Reg64BitsDownCast<11> for Reg64Bits<43> {}
impl Reg64BitsConcat<11, 54> for Reg64Bits<43> {}
impl Reg64BitsDownCast<12> for Reg64Bits<43> {}
impl Reg64BitsConcat<12, 55> for Reg64Bits<43> {}
impl Reg64BitsDownCast<13> for Reg64Bits<43> {}
impl Reg64BitsConcat<13, 56> for Reg64Bits<43> {}
impl Reg64BitsDownCast<14> for Reg64Bits<43> {}
impl Reg64BitsConcat<14, 57> for Reg64Bits<43> {}
impl Reg64BitsDownCast<15> for Reg64Bits<43> {}
impl Reg64BitsConcat<15, 58> for Reg64Bits<43> {}
impl Reg64BitsDownCast<16> for Reg64Bits<43> {}
impl Reg64BitsConcat<16, 59> for Reg64Bits<43> {}
impl Reg64BitsDownCast<17> for Reg64Bits<43> {}
impl Reg64BitsConcat<17, 60> for Reg64Bits<43> {}
impl Reg64BitsDownCast<18> for Reg64Bits<43> {}
impl Reg64BitsConcat<18, 61> for Reg64Bits<43> {}
impl Reg64BitsDownCast<19> for Reg64Bits<43> {}
impl Reg64BitsConcat<19, 62> for Reg64Bits<43> {}
impl Reg64BitsDownCast<20> for Reg64Bits<43> {}
impl Reg64BitsConcat<20, 63> for Reg64Bits<43> {}
impl Reg64BitsDownCast<21> for Reg64Bits<43> {}
impl Reg64BitsConcat<21, 64> for Reg64Bits<43> {}
impl Reg64BitsDownCast<22> for Reg64Bits<43> {}
impl Reg64BitsDownCast<23> for Reg64Bits<43> {}
impl Reg64BitsDownCast<24> for Reg64Bits<43> {}
impl Reg64BitsDownCast<25> for Reg64Bits<43> {}
impl Reg64BitsDownCast<26> for Reg64Bits<43> {}
impl Reg64BitsDownCast<27> for Reg64Bits<43> {}
impl Reg64BitsDownCast<28> for Reg64Bits<43> {}
impl Reg64BitsDownCast<29> for Reg64Bits<43> {}
impl Reg64BitsDownCast<30> for Reg64Bits<43> {}
impl Reg64BitsDownCast<31> for Reg64Bits<43> {}
impl Reg64BitsDownCast<32> for Reg64Bits<43> {}
impl Reg64BitsDownCast<33> for Reg64Bits<43> {}
impl Reg64BitsDownCast<34> for Reg64Bits<43> {}
impl Reg64BitsDownCast<35> for Reg64Bits<43> {}
impl Reg64BitsDownCast<36> for Reg64Bits<43> {}
impl Reg64BitsDownCast<37> for Reg64Bits<43> {}
impl Reg64BitsDownCast<38> for Reg64Bits<43> {}
impl Reg64BitsDownCast<39> for Reg64Bits<43> {}
impl Reg64BitsDownCast<40> for Reg64Bits<43> {}
impl Reg64BitsDownCast<41> for Reg64Bits<43> {}
impl Reg64BitsDownCast<42> for Reg64Bits<43> {}
impl Reg64BitsDownCast<43> for Reg64Bits<43> {}
impl Reg64BitsDownCast<1> for Reg64Bits<44> {}
impl Reg64BitsConcat<1, 45> for Reg64Bits<44> {}
impl Reg64BitsDownCast<2> for Reg64Bits<44> {}
impl Reg64BitsConcat<2, 46> for Reg64Bits<44> {}
impl Reg64BitsDownCast<3> for Reg64Bits<44> {}
impl Reg64BitsConcat<3, 47> for Reg64Bits<44> {}
impl Reg64BitsDownCast<4> for Reg64Bits<44> {}
impl Reg64BitsConcat<4, 48> for Reg64Bits<44> {}
impl Reg64BitsDownCast<5> for Reg64Bits<44> {}
impl Reg64BitsConcat<5, 49> for Reg64Bits<44> {}
impl Reg64BitsDownCast<6> for Reg64Bits<44> {}
impl Reg64BitsConcat<6, 50> for Reg64Bits<44> {}
impl Reg64BitsDownCast<7> for Reg64Bits<44> {}
impl Reg64BitsConcat<7, 51> for Reg64Bits<44> {}
impl Reg64BitsDownCast<8> for Reg64Bits<44> {}
impl Reg64BitsConcat<8, 52> for Reg64Bits<44> {}
impl Reg64BitsDownCast<9> for Reg64Bits<44> {}
impl Reg64BitsConcat<9, 53> for Reg64Bits<44> {}
impl Reg64BitsDownCast<10> for Reg64Bits<44> {}
impl Reg64BitsConcat<10, 54> for Reg64Bits<44> {}
impl Reg64BitsDownCast<11> for Reg64Bits<44> {}
impl Reg64BitsConcat<11, 55> for Reg64Bits<44> {}
impl Reg64BitsDownCast<12> for Reg64Bits<44> {}
impl Reg64BitsConcat<12, 56> for Reg64Bits<44> {}
impl Reg64BitsDownCast<13> for Reg64Bits<44> {}
impl Reg64BitsConcat<13, 57> for Reg64Bits<44> {}
impl Reg64BitsDownCast<14> for Reg64Bits<44> {}
impl Reg64BitsConcat<14, 58> for Reg64Bits<44> {}
impl Reg64BitsDownCast<15> for Reg64Bits<44> {}
impl Reg64BitsConcat<15, 59> for Reg64Bits<44> {}
impl Reg64BitsDownCast<16> for Reg64Bits<44> {}
impl Reg64BitsConcat<16, 60> for Reg64Bits<44> {}
impl Reg64BitsDownCast<17> for Reg64Bits<44> {}
impl Reg64BitsConcat<17, 61> for Reg64Bits<44> {}
impl Reg64BitsDownCast<18> for Reg64Bits<44> {}
impl Reg64BitsConcat<18, 62> for Reg64Bits<44> {}
impl Reg64BitsDownCast<19> for Reg64Bits<44> {}
impl Reg64BitsConcat<19, 63> for Reg64Bits<44> {}
impl Reg64BitsDownCast<20> for Reg64Bits<44> {}
impl Reg64BitsConcat<20, 64> for Reg64Bits<44> {}
impl Reg64BitsDownCast<21> for Reg64Bits<44> {}
impl Reg64BitsDownCast<22> for Reg64Bits<44> {}
impl Reg64BitsDownCast<23> for Reg64Bits<44> {}
impl Reg64BitsDownCast<24> for Reg64Bits<44> {}
impl Reg64BitsDownCast<25> for Reg64Bits<44> {}
impl Reg64BitsDownCast<26> for Reg64Bits<44> {}
impl Reg64BitsDownCast<27> for Reg64Bits<44> {}
impl Reg64BitsDownCast<28> for Reg64Bits<44> {}
impl Reg64BitsDownCast<29> for Reg64Bits<44> {}
impl Reg64BitsDownCast<30> for Reg64Bits<44> {}
impl Reg64BitsDownCast<31> for Reg64Bits<44> {}
impl Reg64BitsDownCast<32> for Reg64Bits<44> {}
impl Reg64BitsDownCast<33> for Reg64Bits<44> {}
impl Reg64BitsDownCast<34> for Reg64Bits<44> {}
impl Reg64BitsDownCast<35> for Reg64Bits<44> {}
impl Reg64BitsDownCast<36> for Reg64Bits<44> {}
impl Reg64BitsDownCast<37> for Reg64Bits<44> {}
impl Reg64BitsDownCast<38> for Reg64Bits<44> {}
impl Reg64BitsDownCast<39> for Reg64Bits<44> {}
impl Reg64BitsDownCast<40> for Reg64Bits<44> {}
impl Reg64BitsDownCast<41> for Reg64Bits<44> {}
impl Reg64BitsDownCast<42> for Reg64Bits<44> {}
impl Reg64BitsDownCast<43> for Reg64Bits<44> {}
impl Reg64BitsDownCast<44> for Reg64Bits<44> {}
impl Reg64BitsDownCast<1> for Reg64Bits<45> {}
impl Reg64BitsConcat<1, 46> for Reg64Bits<45> {}
impl Reg64BitsDownCast<2> for Reg64Bits<45> {}
impl Reg64BitsConcat<2, 47> for Reg64Bits<45> {}
impl Reg64BitsDownCast<3> for Reg64Bits<45> {}
impl Reg64BitsConcat<3, 48> for Reg64Bits<45> {}
impl Reg64BitsDownCast<4> for Reg64Bits<45> {}
impl Reg64BitsConcat<4, 49> for Reg64Bits<45> {}
impl Reg64BitsDownCast<5> for Reg64Bits<45> {}
impl Reg64BitsConcat<5, 50> for Reg64Bits<45> {}
impl Reg64BitsDownCast<6> for Reg64Bits<45> {}
impl Reg64BitsConcat<6, 51> for Reg64Bits<45> {}
impl Reg64BitsDownCast<7> for Reg64Bits<45> {}
impl Reg64BitsConcat<7, 52> for Reg64Bits<45> {}
impl Reg64BitsDownCast<8> for Reg64Bits<45> {}
impl Reg64BitsConcat<8, 53> for Reg64Bits<45> {}
impl Reg64BitsDownCast<9> for Reg64Bits<45> {}
impl Reg64BitsConcat<9, 54> for Reg64Bits<45> {}
impl Reg64BitsDownCast<10> for Reg64Bits<45> {}
impl Reg64BitsConcat<10, 55> for Reg64Bits<45> {}
impl Reg64BitsDownCast<11> for Reg64Bits<45> {}
impl Reg64BitsConcat<11, 56> for Reg64Bits<45> {}
impl Reg64BitsDownCast<12> for Reg64Bits<45> {}
impl Reg64BitsConcat<12, 57> for Reg64Bits<45> {}
impl Reg64BitsDownCast<13> for Reg64Bits<45> {}
impl Reg64BitsConcat<13, 58> for Reg64Bits<45> {}
impl Reg64BitsDownCast<14> for Reg64Bits<45> {}
impl Reg64BitsConcat<14, 59> for Reg64Bits<45> {}
impl Reg64BitsDownCast<15> for Reg64Bits<45> {}
impl Reg64BitsConcat<15, 60> for Reg64Bits<45> {}
impl Reg64BitsDownCast<16> for Reg64Bits<45> {}
impl Reg64BitsConcat<16, 61> for Reg64Bits<45> {}
impl Reg64BitsDownCast<17> for Reg64Bits<45> {}
impl Reg64BitsConcat<17, 62> for Reg64Bits<45> {}
impl Reg64BitsDownCast<18> for Reg64Bits<45> {}
impl Reg64BitsConcat<18, 63> for Reg64Bits<45> {}
impl Reg64BitsDownCast<19> for Reg64Bits<45> {}
impl Reg64BitsConcat<19, 64> for Reg64Bits<45> {}
impl Reg64BitsDownCast<20> for Reg64Bits<45> {}
impl Reg64BitsDownCast<21> for Reg64Bits<45> {}
impl Reg64BitsDownCast<22> for Reg64Bits<45> {}
impl Reg64BitsDownCast<23> for Reg64Bits<45> {}
impl Reg64BitsDownCast<24> for Reg64Bits<45> {}
impl Reg64BitsDownCast<25> for Reg64Bits<45> {}
impl Reg64BitsDownCast<26> for Reg64Bits<45> {}
impl Reg64BitsDownCast<27> for Reg64Bits<45> {}
impl Reg64BitsDownCast<28> for Reg64Bits<45> {}
impl Reg64BitsDownCast<29> for Reg64Bits<45> {}
impl Reg64BitsDownCast<30> for Reg64Bits<45> {}
impl Reg64BitsDownCast<31> for Reg64Bits<45> {}
impl Reg64BitsDownCast<32> for Reg64Bits<45> {}
impl Reg64BitsDownCast<33> for Reg64Bits<45> {}
impl Reg64BitsDownCast<34> for Reg64Bits<45> {}
impl Reg64BitsDownCast<35> for Reg64Bits<45> {}
impl Reg64BitsDownCast<36> for Reg64Bits<45> {}
impl Reg64BitsDownCast<37> for Reg64Bits<45> {}
impl Reg64BitsDownCast<38> for Reg64Bits<45> {}
impl Reg64BitsDownCast<39> for Reg64Bits<45> {}
impl Reg64BitsDownCast<40> for Reg64Bits<45> {}
impl Reg64BitsDownCast<41> for Reg64Bits<45> {}
impl Reg64BitsDownCast<42> for Reg64Bits<45> {}
impl Reg64BitsDownCast<43> for Reg64Bits<45> {}
impl Reg64BitsDownCast<44> for Reg64Bits<45> {}
impl Reg64BitsDownCast<45> for Reg64Bits<45> {}
impl Reg64BitsDownCast<1> for Reg64Bits<46> {}
impl Reg64BitsConcat<1, 47> for Reg64Bits<46> {}
impl Reg64BitsDownCast<2> for Reg64Bits<46> {}
impl Reg64BitsConcat<2, 48> for Reg64Bits<46> {}
impl Reg64BitsDownCast<3> for Reg64Bits<46> {}
impl Reg64BitsConcat<3, 49> for Reg64Bits<46> {}
impl Reg64BitsDownCast<4> for Reg64Bits<46> {}
impl Reg64BitsConcat<4, 50> for Reg64Bits<46> {}
impl Reg64BitsDownCast<5> for Reg64Bits<46> {}
impl Reg64BitsConcat<5, 51> for Reg64Bits<46> {}
impl Reg64BitsDownCast<6> for Reg64Bits<46> {}
impl Reg64BitsConcat<6, 52> for Reg64Bits<46> {}
impl Reg64BitsDownCast<7> for Reg64Bits<46> {}
impl Reg64BitsConcat<7, 53> for Reg64Bits<46> {}
impl Reg64BitsDownCast<8> for Reg64Bits<46> {}
impl Reg64BitsConcat<8, 54> for Reg64Bits<46> {}
impl Reg64BitsDownCast<9> for Reg64Bits<46> {}
impl Reg64BitsConcat<9, 55> for Reg64Bits<46> {}
impl Reg64BitsDownCast<10> for Reg64Bits<46> {}
impl Reg64BitsConcat<10, 56> for Reg64Bits<46> {}
impl Reg64BitsDownCast<11> for Reg64Bits<46> {}
impl Reg64BitsConcat<11, 57> for Reg64Bits<46> {}
impl Reg64BitsDownCast<12> for Reg64Bits<46> {}
impl Reg64BitsConcat<12, 58> for Reg64Bits<46> {}
impl Reg64BitsDownCast<13> for Reg64Bits<46> {}
impl Reg64BitsConcat<13, 59> for Reg64Bits<46> {}
impl Reg64BitsDownCast<14> for Reg64Bits<46> {}
impl Reg64BitsConcat<14, 60> for Reg64Bits<46> {}
impl Reg64BitsDownCast<15> for Reg64Bits<46> {}
impl Reg64BitsConcat<15, 61> for Reg64Bits<46> {}
impl Reg64BitsDownCast<16> for Reg64Bits<46> {}
impl Reg64BitsConcat<16, 62> for Reg64Bits<46> {}
impl Reg64BitsDownCast<17> for Reg64Bits<46> {}
impl Reg64BitsConcat<17, 63> for Reg64Bits<46> {}
impl Reg64BitsDownCast<18> for Reg64Bits<46> {}
impl Reg64BitsConcat<18, 64> for Reg64Bits<46> {}
impl Reg64BitsDownCast<19> for Reg64Bits<46> {}
impl Reg64BitsDownCast<20> for Reg64Bits<46> {}
impl Reg64BitsDownCast<21> for Reg64Bits<46> {}
impl Reg64BitsDownCast<22> for Reg64Bits<46> {}
impl Reg64BitsDownCast<23> for Reg64Bits<46> {}
impl Reg64BitsDownCast<24> for Reg64Bits<46> {}
impl Reg64BitsDownCast<25> for Reg64Bits<46> {}
impl Reg64BitsDownCast<26> for Reg64Bits<46> {}
impl Reg64BitsDownCast<27> for Reg64Bits<46> {}
impl Reg64BitsDownCast<28> for Reg64Bits<46> {}
impl Reg64BitsDownCast<29> for Reg64Bits<46> {}
impl Reg64BitsDownCast<30> for Reg64Bits<46> {}
impl Reg64BitsDownCast<31> for Reg64Bits<46> {}
impl Reg64BitsDownCast<32> for Reg64Bits<46> {}
impl Reg64BitsDownCast<33> for Reg64Bits<46> {}
impl Reg64BitsDownCast<34> for Reg64Bits<46> {}
impl Reg64BitsDownCast<35> for Reg64Bits<46> {}
impl Reg64BitsDownCast<36> for Reg64Bits<46> {}
impl Reg64BitsDownCast<37> for Reg64Bits<46> {}
impl Reg64BitsDownCast<38> for Reg64Bits<46> {}
impl Reg64BitsDownCast<39> for Reg64Bits<46> {}
impl Reg64BitsDownCast<40> for Reg64Bits<46> {}
impl Reg64BitsDownCast<41> for Reg64Bits<46> {}
impl Reg64BitsDownCast<42> for Reg64Bits<46> {}
impl Reg64BitsDownCast<43> for Reg64Bits<46> {}
impl Reg64BitsDownCast<44> for Reg64Bits<46> {}
impl Reg64BitsDownCast<45> for Reg64Bits<46> {}
impl Reg64BitsDownCast<46> for Reg64Bits<46> {}
impl Reg64BitsDownCast<1> for Reg64Bits<47> {}
impl Reg64BitsConcat<1, 48> for Reg64Bits<47> {}
impl Reg64BitsDownCast<2> for Reg64Bits<47> {}
impl Reg64BitsConcat<2, 49> for Reg64Bits<47> {}
impl Reg64BitsDownCast<3> for Reg64Bits<47> {}
impl Reg64BitsConcat<3, 50> for Reg64Bits<47> {}
impl Reg64BitsDownCast<4> for Reg64Bits<47> {}
impl Reg64BitsConcat<4, 51> for Reg64Bits<47> {}
impl Reg64BitsDownCast<5> for Reg64Bits<47> {}
impl Reg64BitsConcat<5, 52> for Reg64Bits<47> {}
impl Reg64BitsDownCast<6> for Reg64Bits<47> {}
impl Reg64BitsConcat<6, 53> for Reg64Bits<47> {}
impl Reg64BitsDownCast<7> for Reg64Bits<47> {}
impl Reg64BitsConcat<7, 54> for Reg64Bits<47> {}
impl Reg64BitsDownCast<8> for Reg64Bits<47> {}
impl Reg64BitsConcat<8, 55> for Reg64Bits<47> {}
impl Reg64BitsDownCast<9> for Reg64Bits<47> {}
impl Reg64BitsConcat<9, 56> for Reg64Bits<47> {}
impl Reg64BitsDownCast<10> for Reg64Bits<47> {}
impl Reg64BitsConcat<10, 57> for Reg64Bits<47> {}
impl Reg64BitsDownCast<11> for Reg64Bits<47> {}
impl Reg64BitsConcat<11, 58> for Reg64Bits<47> {}
impl Reg64BitsDownCast<12> for Reg64Bits<47> {}
impl Reg64BitsConcat<12, 59> for Reg64Bits<47> {}
impl Reg64BitsDownCast<13> for Reg64Bits<47> {}
impl Reg64BitsConcat<13, 60> for Reg64Bits<47> {}
impl Reg64BitsDownCast<14> for Reg64Bits<47> {}
impl Reg64BitsConcat<14, 61> for Reg64Bits<47> {}
impl Reg64BitsDownCast<15> for Reg64Bits<47> {}
impl Reg64BitsConcat<15, 62> for Reg64Bits<47> {}
impl Reg64BitsDownCast<16> for Reg64Bits<47> {}
impl Reg64BitsConcat<16, 63> for Reg64Bits<47> {}
impl Reg64BitsDownCast<17> for Reg64Bits<47> {}
impl Reg64BitsConcat<17, 64> for Reg64Bits<47> {}
impl Reg64BitsDownCast<18> for Reg64Bits<47> {}
impl Reg64BitsDownCast<19> for Reg64Bits<47> {}
impl Reg64BitsDownCast<20> for Reg64Bits<47> {}
impl Reg64BitsDownCast<21> for Reg64Bits<47> {}
impl Reg64BitsDownCast<22> for Reg64Bits<47> {}
impl Reg64BitsDownCast<23> for Reg64Bits<47> {}
impl Reg64BitsDownCast<24> for Reg64Bits<47> {}
impl Reg64BitsDownCast<25> for Reg64Bits<47> {}
impl Reg64BitsDownCast<26> for Reg64Bits<47> {}
impl Reg64BitsDownCast<27> for Reg64Bits<47> {}
impl Reg64BitsDownCast<28> for Reg64Bits<47> {}
impl Reg64BitsDownCast<29> for Reg64Bits<47> {}
impl Reg64BitsDownCast<30> for Reg64Bits<47> {}
impl Reg64BitsDownCast<31> for Reg64Bits<47> {}
impl Reg64BitsDownCast<32> for Reg64Bits<47> {}
impl Reg64BitsDownCast<33> for Reg64Bits<47> {}
impl Reg64BitsDownCast<34> for Reg64Bits<47> {}
impl Reg64BitsDownCast<35> for Reg64Bits<47> {}
impl Reg64BitsDownCast<36> for Reg64Bits<47> {}
impl Reg64BitsDownCast<37> for Reg64Bits<47> {}
impl Reg64BitsDownCast<38> for Reg64Bits<47> {}
impl Reg64BitsDownCast<39> for Reg64Bits<47> {}
impl Reg64BitsDownCast<40> for Reg64Bits<47> {}
impl Reg64BitsDownCast<41> for Reg64Bits<47> {}
impl Reg64BitsDownCast<42> for Reg64Bits<47> {}
impl Reg64BitsDownCast<43> for Reg64Bits<47> {}
impl Reg64BitsDownCast<44> for Reg64Bits<47> {}
impl Reg64BitsDownCast<45> for Reg64Bits<47> {}
impl Reg64BitsDownCast<46> for Reg64Bits<47> {}
impl Reg64BitsDownCast<47> for Reg64Bits<47> {}
impl Reg64BitsDownCast<1> for Reg64Bits<48> {}
impl Reg64BitsConcat<1, 49> for Reg64Bits<48> {}
impl Reg64BitsDownCast<2> for Reg64Bits<48> {}
impl Reg64BitsConcat<2, 50> for Reg64Bits<48> {}
impl Reg64BitsDownCast<3> for Reg64Bits<48> {}
impl Reg64BitsConcat<3, 51> for Reg64Bits<48> {}
impl Reg64BitsDownCast<4> for Reg64Bits<48> {}
impl Reg64BitsConcat<4, 52> for Reg64Bits<48> {}
impl Reg64BitsDownCast<5> for Reg64Bits<48> {}
impl Reg64BitsConcat<5, 53> for Reg64Bits<48> {}
impl Reg64BitsDownCast<6> for Reg64Bits<48> {}
impl Reg64BitsConcat<6, 54> for Reg64Bits<48> {}
impl Reg64BitsDownCast<7> for Reg64Bits<48> {}
impl Reg64BitsConcat<7, 55> for Reg64Bits<48> {}
impl Reg64BitsDownCast<8> for Reg64Bits<48> {}
impl Reg64BitsConcat<8, 56> for Reg64Bits<48> {}
impl Reg64BitsDownCast<9> for Reg64Bits<48> {}
impl Reg64BitsConcat<9, 57> for Reg64Bits<48> {}
impl Reg64BitsDownCast<10> for Reg64Bits<48> {}
impl Reg64BitsConcat<10, 58> for Reg64Bits<48> {}
impl Reg64BitsDownCast<11> for Reg64Bits<48> {}
impl Reg64BitsConcat<11, 59> for Reg64Bits<48> {}
impl Reg64BitsDownCast<12> for Reg64Bits<48> {}
impl Reg64BitsConcat<12, 60> for Reg64Bits<48> {}
impl Reg64BitsDownCast<13> for Reg64Bits<48> {}
impl Reg64BitsConcat<13, 61> for Reg64Bits<48> {}
impl Reg64BitsDownCast<14> for Reg64Bits<48> {}
impl Reg64BitsConcat<14, 62> for Reg64Bits<48> {}
impl Reg64BitsDownCast<15> for Reg64Bits<48> {}
impl Reg64BitsConcat<15, 63> for Reg64Bits<48> {}
impl Reg64BitsDownCast<16> for Reg64Bits<48> {}
impl Reg64BitsConcat<16, 64> for Reg64Bits<48> {}
impl Reg64BitsDownCast<17> for Reg64Bits<48> {}
impl Reg64BitsDownCast<18> for Reg64Bits<48> {}
impl Reg64BitsDownCast<19> for Reg64Bits<48> {}
impl Reg64BitsDownCast<20> for Reg64Bits<48> {}
impl Reg64BitsDownCast<21> for Reg64Bits<48> {}
impl Reg64BitsDownCast<22> for Reg64Bits<48> {}
impl Reg64BitsDownCast<23> for Reg64Bits<48> {}
impl Reg64BitsDownCast<24> for Reg64Bits<48> {}
impl Reg64BitsDownCast<25> for Reg64Bits<48> {}
impl Reg64BitsDownCast<26> for Reg64Bits<48> {}
impl Reg64BitsDownCast<27> for Reg64Bits<48> {}
impl Reg64BitsDownCast<28> for Reg64Bits<48> {}
impl Reg64BitsDownCast<29> for Reg64Bits<48> {}
impl Reg64BitsDownCast<30> for Reg64Bits<48> {}
impl Reg64BitsDownCast<31> for Reg64Bits<48> {}
impl Reg64BitsDownCast<32> for Reg64Bits<48> {}
impl Reg64BitsDownCast<33> for Reg64Bits<48> {}
impl Reg64BitsDownCast<34> for Reg64Bits<48> {}
impl Reg64BitsDownCast<35> for Reg64Bits<48> {}
impl Reg64BitsDownCast<36> for Reg64Bits<48> {}
impl Reg64BitsDownCast<37> for Reg64Bits<48> {}
impl Reg64BitsDownCast<38> for Reg64Bits<48> {}
impl Reg64BitsDownCast<39> for Reg64Bits<48> {}
impl Reg64BitsDownCast<40> for Reg64Bits<48> {}
impl Reg64BitsDownCast<41> for Reg64Bits<48> {}
impl Reg64BitsDownCast<42> for Reg64Bits<48> {}
impl Reg64BitsDownCast<43> for Reg64Bits<48> {}
impl Reg64BitsDownCast<44> for Reg64Bits<48> {}
impl Reg64BitsDownCast<45> for Reg64Bits<48> {}
impl Reg64BitsDownCast<46> for Reg64Bits<48> {}
impl Reg64BitsDownCast<47> for Reg64Bits<48> {}
impl Reg64BitsDownCast<48> for Reg64Bits<48> {}
impl Reg64BitsDownCast<1> for Reg64Bits<49> {}
impl Reg64BitsConcat<1, 50> for Reg64Bits<49> {}
impl Reg64BitsDownCast<2> for Reg64Bits<49> {}
impl Reg64BitsConcat<2, 51> for Reg64Bits<49> {}
impl Reg64BitsDownCast<3> for Reg64Bits<49> {}
impl Reg64BitsConcat<3, 52> for Reg64Bits<49> {}
impl Reg64BitsDownCast<4> for Reg64Bits<49> {}
impl Reg64BitsConcat<4, 53> for Reg64Bits<49> {}
impl Reg64BitsDownCast<5> for Reg64Bits<49> {}
impl Reg64BitsConcat<5, 54> for Reg64Bits<49> {}
impl Reg64BitsDownCast<6> for Reg64Bits<49> {}
impl Reg64BitsConcat<6, 55> for Reg64Bits<49> {}
impl Reg64BitsDownCast<7> for Reg64Bits<49> {}
impl Reg64BitsConcat<7, 56> for Reg64Bits<49> {}
impl Reg64BitsDownCast<8> for Reg64Bits<49> {}
impl Reg64BitsConcat<8, 57> for Reg64Bits<49> {}
impl Reg64BitsDownCast<9> for Reg64Bits<49> {}
impl Reg64BitsConcat<9, 58> for Reg64Bits<49> {}
impl Reg64BitsDownCast<10> for Reg64Bits<49> {}
impl Reg64BitsConcat<10, 59> for Reg64Bits<49> {}
impl Reg64BitsDownCast<11> for Reg64Bits<49> {}
impl Reg64BitsConcat<11, 60> for Reg64Bits<49> {}
impl Reg64BitsDownCast<12> for Reg64Bits<49> {}
impl Reg64BitsConcat<12, 61> for Reg64Bits<49> {}
impl Reg64BitsDownCast<13> for Reg64Bits<49> {}
impl Reg64BitsConcat<13, 62> for Reg64Bits<49> {}
impl Reg64BitsDownCast<14> for Reg64Bits<49> {}
impl Reg64BitsConcat<14, 63> for Reg64Bits<49> {}
impl Reg64BitsDownCast<15> for Reg64Bits<49> {}
impl Reg64BitsConcat<15, 64> for Reg64Bits<49> {}
impl Reg64BitsDownCast<16> for Reg64Bits<49> {}
impl Reg64BitsDownCast<17> for Reg64Bits<49> {}
impl Reg64BitsDownCast<18> for Reg64Bits<49> {}
impl Reg64BitsDownCast<19> for Reg64Bits<49> {}
impl Reg64BitsDownCast<20> for Reg64Bits<49> {}
impl Reg64BitsDownCast<21> for Reg64Bits<49> {}
impl Reg64BitsDownCast<22> for Reg64Bits<49> {}
impl Reg64BitsDownCast<23> for Reg64Bits<49> {}
impl Reg64BitsDownCast<24> for Reg64Bits<49> {}
impl Reg64BitsDownCast<25> for Reg64Bits<49> {}
impl Reg64BitsDownCast<26> for Reg64Bits<49> {}
impl Reg64BitsDownCast<27> for Reg64Bits<49> {}
impl Reg64BitsDownCast<28> for Reg64Bits<49> {}
impl Reg64BitsDownCast<29> for Reg64Bits<49> {}
impl Reg64BitsDownCast<30> for Reg64Bits<49> {}
impl Reg64BitsDownCast<31> for Reg64Bits<49> {}
impl Reg64BitsDownCast<32> for Reg64Bits<49> {}
impl Reg64BitsDownCast<33> for Reg64Bits<49> {}
impl Reg64BitsDownCast<34> for Reg64Bits<49> {}
impl Reg64BitsDownCast<35> for Reg64Bits<49> {}
impl Reg64BitsDownCast<36> for Reg64Bits<49> {}
impl Reg64BitsDownCast<37> for Reg64Bits<49> {}
impl Reg64BitsDownCast<38> for Reg64Bits<49> {}
impl Reg64BitsDownCast<39> for Reg64Bits<49> {}
impl Reg64BitsDownCast<40> for Reg64Bits<49> {}
impl Reg64BitsDownCast<41> for Reg64Bits<49> {}
impl Reg64BitsDownCast<42> for Reg64Bits<49> {}
impl Reg64BitsDownCast<43> for Reg64Bits<49> {}
impl Reg64BitsDownCast<44> for Reg64Bits<49> {}
impl Reg64BitsDownCast<45> for Reg64Bits<49> {}
impl Reg64BitsDownCast<46> for Reg64Bits<49> {}
impl Reg64BitsDownCast<47> for Reg64Bits<49> {}
impl Reg64BitsDownCast<48> for Reg64Bits<49> {}
impl Reg64BitsDownCast<49> for Reg64Bits<49> {}
impl Reg64BitsDownCast<1> for Reg64Bits<50> {}
impl Reg64BitsConcat<1, 51> for Reg64Bits<50> {}
impl Reg64BitsDownCast<2> for Reg64Bits<50> {}
impl Reg64BitsConcat<2, 52> for Reg64Bits<50> {}
impl Reg64BitsDownCast<3> for Reg64Bits<50> {}
impl Reg64BitsConcat<3, 53> for Reg64Bits<50> {}
impl Reg64BitsDownCast<4> for Reg64Bits<50> {}
impl Reg64BitsConcat<4, 54> for Reg64Bits<50> {}
impl Reg64BitsDownCast<5> for Reg64Bits<50> {}
impl Reg64BitsConcat<5, 55> for Reg64Bits<50> {}
impl Reg64BitsDownCast<6> for Reg64Bits<50> {}
impl Reg64BitsConcat<6, 56> for Reg64Bits<50> {}
impl Reg64BitsDownCast<7> for Reg64Bits<50> {}
impl Reg64BitsConcat<7, 57> for Reg64Bits<50> {}
impl Reg64BitsDownCast<8> for Reg64Bits<50> {}
impl Reg64BitsConcat<8, 58> for Reg64Bits<50> {}
impl Reg64BitsDownCast<9> for Reg64Bits<50> {}
impl Reg64BitsConcat<9, 59> for Reg64Bits<50> {}
impl Reg64BitsDownCast<10> for Reg64Bits<50> {}
impl Reg64BitsConcat<10, 60> for Reg64Bits<50> {}
impl Reg64BitsDownCast<11> for Reg64Bits<50> {}
impl Reg64BitsConcat<11, 61> for Reg64Bits<50> {}
impl Reg64BitsDownCast<12> for Reg64Bits<50> {}
impl Reg64BitsConcat<12, 62> for Reg64Bits<50> {}
impl Reg64BitsDownCast<13> for Reg64Bits<50> {}
impl Reg64BitsConcat<13, 63> for Reg64Bits<50> {}
impl Reg64BitsDownCast<14> for Reg64Bits<50> {}
impl Reg64BitsConcat<14, 64> for Reg64Bits<50> {}
impl Reg64BitsDownCast<15> for Reg64Bits<50> {}
impl Reg64BitsDownCast<16> for Reg64Bits<50> {}
impl Reg64BitsDownCast<17> for Reg64Bits<50> {}
impl Reg64BitsDownCast<18> for Reg64Bits<50> {}
impl Reg64BitsDownCast<19> for Reg64Bits<50> {}
impl Reg64BitsDownCast<20> for Reg64Bits<50> {}
impl Reg64BitsDownCast<21> for Reg64Bits<50> {}
impl Reg64BitsDownCast<22> for Reg64Bits<50> {}
impl Reg64BitsDownCast<23> for Reg64Bits<50> {}
impl Reg64BitsDownCast<24> for Reg64Bits<50> {}
impl Reg64BitsDownCast<25> for Reg64Bits<50> {}
impl Reg64BitsDownCast<26> for Reg64Bits<50> {}
impl Reg64BitsDownCast<27> for Reg64Bits<50> {}
impl Reg64BitsDownCast<28> for Reg64Bits<50> {}
impl Reg64BitsDownCast<29> for Reg64Bits<50> {}
impl Reg64BitsDownCast<30> for Reg64Bits<50> {}
impl Reg64BitsDownCast<31> for Reg64Bits<50> {}
impl Reg64BitsDownCast<32> for Reg64Bits<50> {}
impl Reg64BitsDownCast<33> for Reg64Bits<50> {}
impl Reg64BitsDownCast<34> for Reg64Bits<50> {}
impl Reg64BitsDownCast<35> for Reg64Bits<50> {}
impl Reg64BitsDownCast<36> for Reg64Bits<50> {}
impl Reg64BitsDownCast<37> for Reg64Bits<50> {}
impl Reg64BitsDownCast<38> for Reg64Bits<50> {}
impl Reg64BitsDownCast<39> for Reg64Bits<50> {}
impl Reg64BitsDownCast<40> for Reg64Bits<50> {}
impl Reg64BitsDownCast<41> for Reg64Bits<50> {}
impl Reg64BitsDownCast<42> for Reg64Bits<50> {}
impl Reg64BitsDownCast<43> for Reg64Bits<50> {}
impl Reg64BitsDownCast<44> for Reg64Bits<50> {}
impl Reg64BitsDownCast<45> for Reg64Bits<50> {}
impl Reg64BitsDownCast<46> for Reg64Bits<50> {}
impl Reg64BitsDownCast<47> for Reg64Bits<50> {}
impl Reg64BitsDownCast<48> for Reg64Bits<50> {}
impl Reg64BitsDownCast<49> for Reg64Bits<50> {}
impl Reg64BitsDownCast<50> for Reg64Bits<50> {}
impl Reg64BitsDownCast<1> for Reg64Bits<51> {}
impl Reg64BitsConcat<1, 52> for Reg64Bits<51> {}
impl Reg64BitsDownCast<2> for Reg64Bits<51> {}
impl Reg64BitsConcat<2, 53> for Reg64Bits<51> {}
impl Reg64BitsDownCast<3> for Reg64Bits<51> {}
impl Reg64BitsConcat<3, 54> for Reg64Bits<51> {}
impl Reg64BitsDownCast<4> for Reg64Bits<51> {}
impl Reg64BitsConcat<4, 55> for Reg64Bits<51> {}
impl Reg64BitsDownCast<5> for Reg64Bits<51> {}
impl Reg64BitsConcat<5, 56> for Reg64Bits<51> {}
impl Reg64BitsDownCast<6> for Reg64Bits<51> {}
impl Reg64BitsConcat<6, 57> for Reg64Bits<51> {}
impl Reg64BitsDownCast<7> for Reg64Bits<51> {}
impl Reg64BitsConcat<7, 58> for Reg64Bits<51> {}
impl Reg64BitsDownCast<8> for Reg64Bits<51> {}
impl Reg64BitsConcat<8, 59> for Reg64Bits<51> {}
impl Reg64BitsDownCast<9> for Reg64Bits<51> {}
impl Reg64BitsConcat<9, 60> for Reg64Bits<51> {}
impl Reg64BitsDownCast<10> for Reg64Bits<51> {}
impl Reg64BitsConcat<10, 61> for Reg64Bits<51> {}
impl Reg64BitsDownCast<11> for Reg64Bits<51> {}
impl Reg64BitsConcat<11, 62> for Reg64Bits<51> {}
impl Reg64BitsDownCast<12> for Reg64Bits<51> {}
impl Reg64BitsConcat<12, 63> for Reg64Bits<51> {}
impl Reg64BitsDownCast<13> for Reg64Bits<51> {}
impl Reg64BitsConcat<13, 64> for Reg64Bits<51> {}
impl Reg64BitsDownCast<14> for Reg64Bits<51> {}
impl Reg64BitsDownCast<15> for Reg64Bits<51> {}
impl Reg64BitsDownCast<16> for Reg64Bits<51> {}
impl Reg64BitsDownCast<17> for Reg64Bits<51> {}
impl Reg64BitsDownCast<18> for Reg64Bits<51> {}
impl Reg64BitsDownCast<19> for Reg64Bits<51> {}
impl Reg64BitsDownCast<20> for Reg64Bits<51> {}
impl Reg64BitsDownCast<21> for Reg64Bits<51> {}
impl Reg64BitsDownCast<22> for Reg64Bits<51> {}
impl Reg64BitsDownCast<23> for Reg64Bits<51> {}
impl Reg64BitsDownCast<24> for Reg64Bits<51> {}
impl Reg64BitsDownCast<25> for Reg64Bits<51> {}
impl Reg64BitsDownCast<26> for Reg64Bits<51> {}
impl Reg64BitsDownCast<27> for Reg64Bits<51> {}
impl Reg64BitsDownCast<28> for Reg64Bits<51> {}
impl Reg64BitsDownCast<29> for Reg64Bits<51> {}
impl Reg64BitsDownCast<30> for Reg64Bits<51> {}
impl Reg64BitsDownCast<31> for Reg64Bits<51> {}
impl Reg64BitsDownCast<32> for Reg64Bits<51> {}
impl Reg64BitsDownCast<33> for Reg64Bits<51> {}
impl Reg64BitsDownCast<34> for Reg64Bits<51> {}
impl Reg64BitsDownCast<35> for Reg64Bits<51> {}
impl Reg64BitsDownCast<36> for Reg64Bits<51> {}
impl Reg64BitsDownCast<37> for Reg64Bits<51> {}
impl Reg64BitsDownCast<38> for Reg64Bits<51> {}
impl Reg64BitsDownCast<39> for Reg64Bits<51> {}
impl Reg64BitsDownCast<40> for Reg64Bits<51> {}
impl Reg64BitsDownCast<41> for Reg64Bits<51> {}
impl Reg64BitsDownCast<42> for Reg64Bits<51> {}
impl Reg64BitsDownCast<43> for Reg64Bits<51> {}
impl Reg64BitsDownCast<44> for Reg64Bits<51> {}
impl Reg64BitsDownCast<45> for Reg64Bits<51> {}
impl Reg64BitsDownCast<46> for Reg64Bits<51> {}
impl Reg64BitsDownCast<47> for Reg64Bits<51> {}
impl Reg64BitsDownCast<48> for Reg64Bits<51> {}
impl Reg64BitsDownCast<49> for Reg64Bits<51> {}
impl Reg64BitsDownCast<50> for Reg64Bits<51> {}
impl Reg64BitsDownCast<51> for Reg64Bits<51> {}
impl Reg64BitsDownCast<1> for Reg64Bits<52> {}
impl Reg64BitsConcat<1, 53> for Reg64Bits<52> {}
impl Reg64BitsDownCast<2> for Reg64Bits<52> {}
impl Reg64BitsConcat<2, 54> for Reg64Bits<52> {}
impl Reg64BitsDownCast<3> for Reg64Bits<52> {}
impl Reg64BitsConcat<3, 55> for Reg64Bits<52> {}
impl Reg64BitsDownCast<4> for Reg64Bits<52> {}
impl Reg64BitsConcat<4, 56> for Reg64Bits<52> {}
impl Reg64BitsDownCast<5> for Reg64Bits<52> {}
impl Reg64BitsConcat<5, 57> for Reg64Bits<52> {}
impl Reg64BitsDownCast<6> for Reg64Bits<52> {}
impl Reg64BitsConcat<6, 58> for Reg64Bits<52> {}
impl Reg64BitsDownCast<7> for Reg64Bits<52> {}
impl Reg64BitsConcat<7, 59> for Reg64Bits<52> {}
impl Reg64BitsDownCast<8> for Reg64Bits<52> {}
impl Reg64BitsConcat<8, 60> for Reg64Bits<52> {}
impl Reg64BitsDownCast<9> for Reg64Bits<52> {}
impl Reg64BitsConcat<9, 61> for Reg64Bits<52> {}
impl Reg64BitsDownCast<10> for Reg64Bits<52> {}
impl Reg64BitsConcat<10, 62> for Reg64Bits<52> {}
impl Reg64BitsDownCast<11> for Reg64Bits<52> {}
impl Reg64BitsConcat<11, 63> for Reg64Bits<52> {}
impl Reg64BitsDownCast<12> for Reg64Bits<52> {}
impl Reg64BitsConcat<12, 64> for Reg64Bits<52> {}
impl Reg64BitsDownCast<13> for Reg64Bits<52> {}
impl Reg64BitsDownCast<14> for Reg64Bits<52> {}
impl Reg64BitsDownCast<15> for Reg64Bits<52> {}
impl Reg64BitsDownCast<16> for Reg64Bits<52> {}
impl Reg64BitsDownCast<17> for Reg64Bits<52> {}
impl Reg64BitsDownCast<18> for Reg64Bits<52> {}
impl Reg64BitsDownCast<19> for Reg64Bits<52> {}
impl Reg64BitsDownCast<20> for Reg64Bits<52> {}
impl Reg64BitsDownCast<21> for Reg64Bits<52> {}
impl Reg64BitsDownCast<22> for Reg64Bits<52> {}
impl Reg64BitsDownCast<23> for Reg64Bits<52> {}
impl Reg64BitsDownCast<24> for Reg64Bits<52> {}
impl Reg64BitsDownCast<25> for Reg64Bits<52> {}
impl Reg64BitsDownCast<26> for Reg64Bits<52> {}
impl Reg64BitsDownCast<27> for Reg64Bits<52> {}
impl Reg64BitsDownCast<28> for Reg64Bits<52> {}
impl Reg64BitsDownCast<29> for Reg64Bits<52> {}
impl Reg64BitsDownCast<30> for Reg64Bits<52> {}
impl Reg64BitsDownCast<31> for Reg64Bits<52> {}
impl Reg64BitsDownCast<32> for Reg64Bits<52> {}
impl Reg64BitsDownCast<33> for Reg64Bits<52> {}
impl Reg64BitsDownCast<34> for Reg64Bits<52> {}
impl Reg64BitsDownCast<35> for Reg64Bits<52> {}
impl Reg64BitsDownCast<36> for Reg64Bits<52> {}
impl Reg64BitsDownCast<37> for Reg64Bits<52> {}
impl Reg64BitsDownCast<38> for Reg64Bits<52> {}
impl Reg64BitsDownCast<39> for Reg64Bits<52> {}
impl Reg64BitsDownCast<40> for Reg64Bits<52> {}
impl Reg64BitsDownCast<41> for Reg64Bits<52> {}
impl Reg64BitsDownCast<42> for Reg64Bits<52> {}
impl Reg64BitsDownCast<43> for Reg64Bits<52> {}
impl Reg64BitsDownCast<44> for Reg64Bits<52> {}
impl Reg64BitsDownCast<45> for Reg64Bits<52> {}
impl Reg64BitsDownCast<46> for Reg64Bits<52> {}
impl Reg64BitsDownCast<47> for Reg64Bits<52> {}
impl Reg64BitsDownCast<48> for Reg64Bits<52> {}
impl Reg64BitsDownCast<49> for Reg64Bits<52> {}
impl Reg64BitsDownCast<50> for Reg64Bits<52> {}
impl Reg64BitsDownCast<51> for Reg64Bits<52> {}
impl Reg64BitsDownCast<52> for Reg64Bits<52> {}
impl Reg64BitsDownCast<1> for Reg64Bits<53> {}
impl Reg64BitsConcat<1, 54> for Reg64Bits<53> {}
impl Reg64BitsDownCast<2> for Reg64Bits<53> {}
impl Reg64BitsConcat<2, 55> for Reg64Bits<53> {}
impl Reg64BitsDownCast<3> for Reg64Bits<53> {}
impl Reg64BitsConcat<3, 56> for Reg64Bits<53> {}
impl Reg64BitsDownCast<4> for Reg64Bits<53> {}
impl Reg64BitsConcat<4, 57> for Reg64Bits<53> {}
impl Reg64BitsDownCast<5> for Reg64Bits<53> {}
impl Reg64BitsConcat<5, 58> for Reg64Bits<53> {}
impl Reg64BitsDownCast<6> for Reg64Bits<53> {}
impl Reg64BitsConcat<6, 59> for Reg64Bits<53> {}
impl Reg64BitsDownCast<7> for Reg64Bits<53> {}
impl Reg64BitsConcat<7, 60> for Reg64Bits<53> {}
impl Reg64BitsDownCast<8> for Reg64Bits<53> {}
impl Reg64BitsConcat<8, 61> for Reg64Bits<53> {}
impl Reg64BitsDownCast<9> for Reg64Bits<53> {}
impl Reg64BitsConcat<9, 62> for Reg64Bits<53> {}
impl Reg64BitsDownCast<10> for Reg64Bits<53> {}
impl Reg64BitsConcat<10, 63> for Reg64Bits<53> {}
impl Reg64BitsDownCast<11> for Reg64Bits<53> {}
impl Reg64BitsConcat<11, 64> for Reg64Bits<53> {}
impl Reg64BitsDownCast<12> for Reg64Bits<53> {}
impl Reg64BitsDownCast<13> for Reg64Bits<53> {}
impl Reg64BitsDownCast<14> for Reg64Bits<53> {}
impl Reg64BitsDownCast<15> for Reg64Bits<53> {}
impl Reg64BitsDownCast<16> for Reg64Bits<53> {}
impl Reg64BitsDownCast<17> for Reg64Bits<53> {}
impl Reg64BitsDownCast<18> for Reg64Bits<53> {}
impl Reg64BitsDownCast<19> for Reg64Bits<53> {}
impl Reg64BitsDownCast<20> for Reg64Bits<53> {}
impl Reg64BitsDownCast<21> for Reg64Bits<53> {}
impl Reg64BitsDownCast<22> for Reg64Bits<53> {}
impl Reg64BitsDownCast<23> for Reg64Bits<53> {}
impl Reg64BitsDownCast<24> for Reg64Bits<53> {}
impl Reg64BitsDownCast<25> for Reg64Bits<53> {}
impl Reg64BitsDownCast<26> for Reg64Bits<53> {}
impl Reg64BitsDownCast<27> for Reg64Bits<53> {}
impl Reg64BitsDownCast<28> for Reg64Bits<53> {}
impl Reg64BitsDownCast<29> for Reg64Bits<53> {}
impl Reg64BitsDownCast<30> for Reg64Bits<53> {}
impl Reg64BitsDownCast<31> for Reg64Bits<53> {}
impl Reg64BitsDownCast<32> for Reg64Bits<53> {}
impl Reg64BitsDownCast<33> for Reg64Bits<53> {}
impl Reg64BitsDownCast<34> for Reg64Bits<53> {}
impl Reg64BitsDownCast<35> for Reg64Bits<53> {}
impl Reg64BitsDownCast<36> for Reg64Bits<53> {}
impl Reg64BitsDownCast<37> for Reg64Bits<53> {}
impl Reg64BitsDownCast<38> for Reg64Bits<53> {}
impl Reg64BitsDownCast<39> for Reg64Bits<53> {}
impl Reg64BitsDownCast<40> for Reg64Bits<53> {}
impl Reg64BitsDownCast<41> for Reg64Bits<53> {}
impl Reg64BitsDownCast<42> for Reg64Bits<53> {}
impl Reg64BitsDownCast<43> for Reg64Bits<53> {}
impl Reg64BitsDownCast<44> for Reg64Bits<53> {}
impl Reg64BitsDownCast<45> for Reg64Bits<53> {}
impl Reg64BitsDownCast<46> for Reg64Bits<53> {}
impl Reg64BitsDownCast<47> for Reg64Bits<53> {}
impl Reg64BitsDownCast<48> for Reg64Bits<53> {}
impl Reg64BitsDownCast<49> for Reg64Bits<53> {}
impl Reg64BitsDownCast<50> for Reg64Bits<53> {}
impl Reg64BitsDownCast<51> for Reg64Bits<53> {}
impl Reg64BitsDownCast<52> for Reg64Bits<53> {}
impl Reg64BitsDownCast<53> for Reg64Bits<53> {}
impl Reg64BitsDownCast<1> for Reg64Bits<54> {}
impl Reg64BitsConcat<1, 55> for Reg64Bits<54> {}
impl Reg64BitsDownCast<2> for Reg64Bits<54> {}
impl Reg64BitsConcat<2, 56> for Reg64Bits<54> {}
impl Reg64BitsDownCast<3> for Reg64Bits<54> {}
impl Reg64BitsConcat<3, 57> for Reg64Bits<54> {}
impl Reg64BitsDownCast<4> for Reg64Bits<54> {}
impl Reg64BitsConcat<4, 58> for Reg64Bits<54> {}
impl Reg64BitsDownCast<5> for Reg64Bits<54> {}
impl Reg64BitsConcat<5, 59> for Reg64Bits<54> {}
impl Reg64BitsDownCast<6> for Reg64Bits<54> {}
impl Reg64BitsConcat<6, 60> for Reg64Bits<54> {}
impl Reg64BitsDownCast<7> for Reg64Bits<54> {}
impl Reg64BitsConcat<7, 61> for Reg64Bits<54> {}
impl Reg64BitsDownCast<8> for Reg64Bits<54> {}
impl Reg64BitsConcat<8, 62> for Reg64Bits<54> {}
impl Reg64BitsDownCast<9> for Reg64Bits<54> {}
impl Reg64BitsConcat<9, 63> for Reg64Bits<54> {}
impl Reg64BitsDownCast<10> for Reg64Bits<54> {}
impl Reg64BitsConcat<10, 64> for Reg64Bits<54> {}
impl Reg64BitsDownCast<11> for Reg64Bits<54> {}
impl Reg64BitsDownCast<12> for Reg64Bits<54> {}
impl Reg64BitsDownCast<13> for Reg64Bits<54> {}
impl Reg64BitsDownCast<14> for Reg64Bits<54> {}
impl Reg64BitsDownCast<15> for Reg64Bits<54> {}
impl Reg64BitsDownCast<16> for Reg64Bits<54> {}
impl Reg64BitsDownCast<17> for Reg64Bits<54> {}
impl Reg64BitsDownCast<18> for Reg64Bits<54> {}
impl Reg64BitsDownCast<19> for Reg64Bits<54> {}
impl Reg64BitsDownCast<20> for Reg64Bits<54> {}
impl Reg64BitsDownCast<21> for Reg64Bits<54> {}
impl Reg64BitsDownCast<22> for Reg64Bits<54> {}
impl Reg64BitsDownCast<23> for Reg64Bits<54> {}
impl Reg64BitsDownCast<24> for Reg64Bits<54> {}
impl Reg64BitsDownCast<25> for Reg64Bits<54> {}
impl Reg64BitsDownCast<26> for Reg64Bits<54> {}
impl Reg64BitsDownCast<27> for Reg64Bits<54> {}
impl Reg64BitsDownCast<28> for Reg64Bits<54> {}
impl Reg64BitsDownCast<29> for Reg64Bits<54> {}
impl Reg64BitsDownCast<30> for Reg64Bits<54> {}
impl Reg64BitsDownCast<31> for Reg64Bits<54> {}
impl Reg64BitsDownCast<32> for Reg64Bits<54> {}
impl Reg64BitsDownCast<33> for Reg64Bits<54> {}
impl Reg64BitsDownCast<34> for Reg64Bits<54> {}
impl Reg64BitsDownCast<35> for Reg64Bits<54> {}
impl Reg64BitsDownCast<36> for Reg64Bits<54> {}
impl Reg64BitsDownCast<37> for Reg64Bits<54> {}
impl Reg64BitsDownCast<38> for Reg64Bits<54> {}
impl Reg64BitsDownCast<39> for Reg64Bits<54> {}
impl Reg64BitsDownCast<40> for Reg64Bits<54> {}
impl Reg64BitsDownCast<41> for Reg64Bits<54> {}
impl Reg64BitsDownCast<42> for Reg64Bits<54> {}
impl Reg64BitsDownCast<43> for Reg64Bits<54> {}
impl Reg64BitsDownCast<44> for Reg64Bits<54> {}
impl Reg64BitsDownCast<45> for Reg64Bits<54> {}
impl Reg64BitsDownCast<46> for Reg64Bits<54> {}
impl Reg64BitsDownCast<47> for Reg64Bits<54> {}
impl Reg64BitsDownCast<48> for Reg64Bits<54> {}
impl Reg64BitsDownCast<49> for Reg64Bits<54> {}
impl Reg64BitsDownCast<50> for Reg64Bits<54> {}
impl Reg64BitsDownCast<51> for Reg64Bits<54> {}
impl Reg64BitsDownCast<52> for Reg64Bits<54> {}
impl Reg64BitsDownCast<53> for Reg64Bits<54> {}
impl Reg64BitsDownCast<54> for Reg64Bits<54> {}
impl Reg64BitsDownCast<1> for Reg64Bits<55> {}
impl Reg64BitsConcat<1, 56> for Reg64Bits<55> {}
impl Reg64BitsDownCast<2> for Reg64Bits<55> {}
impl Reg64BitsConcat<2, 57> for Reg64Bits<55> {}
impl Reg64BitsDownCast<3> for Reg64Bits<55> {}
impl Reg64BitsConcat<3, 58> for Reg64Bits<55> {}
impl Reg64BitsDownCast<4> for Reg64Bits<55> {}
impl Reg64BitsConcat<4, 59> for Reg64Bits<55> {}
impl Reg64BitsDownCast<5> for Reg64Bits<55> {}
impl Reg64BitsConcat<5, 60> for Reg64Bits<55> {}
impl Reg64BitsDownCast<6> for Reg64Bits<55> {}
impl Reg64BitsConcat<6, 61> for Reg64Bits<55> {}
impl Reg64BitsDownCast<7> for Reg64Bits<55> {}
impl Reg64BitsConcat<7, 62> for Reg64Bits<55> {}
impl Reg64BitsDownCast<8> for Reg64Bits<55> {}
impl Reg64BitsConcat<8, 63> for Reg64Bits<55> {}
impl Reg64BitsDownCast<9> for Reg64Bits<55> {}
impl Reg64BitsConcat<9, 64> for Reg64Bits<55> {}
impl Reg64BitsDownCast<10> for Reg64Bits<55> {}
impl Reg64BitsDownCast<11> for Reg64Bits<55> {}
impl Reg64BitsDownCast<12> for Reg64Bits<55> {}
impl Reg64BitsDownCast<13> for Reg64Bits<55> {}
impl Reg64BitsDownCast<14> for Reg64Bits<55> {}
impl Reg64BitsDownCast<15> for Reg64Bits<55> {}
impl Reg64BitsDownCast<16> for Reg64Bits<55> {}
impl Reg64BitsDownCast<17> for Reg64Bits<55> {}
impl Reg64BitsDownCast<18> for Reg64Bits<55> {}
impl Reg64BitsDownCast<19> for Reg64Bits<55> {}
impl Reg64BitsDownCast<20> for Reg64Bits<55> {}
impl Reg64BitsDownCast<21> for Reg64Bits<55> {}
impl Reg64BitsDownCast<22> for Reg64Bits<55> {}
impl Reg64BitsDownCast<23> for Reg64Bits<55> {}
impl Reg64BitsDownCast<24> for Reg64Bits<55> {}
impl Reg64BitsDownCast<25> for Reg64Bits<55> {}
impl Reg64BitsDownCast<26> for Reg64Bits<55> {}
impl Reg64BitsDownCast<27> for Reg64Bits<55> {}
impl Reg64BitsDownCast<28> for Reg64Bits<55> {}
impl Reg64BitsDownCast<29> for Reg64Bits<55> {}
impl Reg64BitsDownCast<30> for Reg64Bits<55> {}
impl Reg64BitsDownCast<31> for Reg64Bits<55> {}
impl Reg64BitsDownCast<32> for Reg64Bits<55> {}
impl Reg64BitsDownCast<33> for Reg64Bits<55> {}
impl Reg64BitsDownCast<34> for Reg64Bits<55> {}
impl Reg64BitsDownCast<35> for Reg64Bits<55> {}
impl Reg64BitsDownCast<36> for Reg64Bits<55> {}
impl Reg64BitsDownCast<37> for Reg64Bits<55> {}
impl Reg64BitsDownCast<38> for Reg64Bits<55> {}
impl Reg64BitsDownCast<39> for Reg64Bits<55> {}
impl Reg64BitsDownCast<40> for Reg64Bits<55> {}
impl Reg64BitsDownCast<41> for Reg64Bits<55> {}
impl Reg64BitsDownCast<42> for Reg64Bits<55> {}
impl Reg64BitsDownCast<43> for Reg64Bits<55> {}
impl Reg64BitsDownCast<44> for Reg64Bits<55> {}
impl Reg64BitsDownCast<45> for Reg64Bits<55> {}
impl Reg64BitsDownCast<46> for Reg64Bits<55> {}
impl Reg64BitsDownCast<47> for Reg64Bits<55> {}
impl Reg64BitsDownCast<48> for Reg64Bits<55> {}
impl Reg64BitsDownCast<49> for Reg64Bits<55> {}
impl Reg64BitsDownCast<50> for Reg64Bits<55> {}
impl Reg64BitsDownCast<51> for Reg64Bits<55> {}
impl Reg64BitsDownCast<52> for Reg64Bits<55> {}
impl Reg64BitsDownCast<53> for Reg64Bits<55> {}
impl Reg64BitsDownCast<54> for Reg64Bits<55> {}
impl Reg64BitsDownCast<55> for Reg64Bits<55> {}
impl Reg64BitsDownCast<1> for Reg64Bits<56> {}
impl Reg64BitsConcat<1, 57> for Reg64Bits<56> {}
impl Reg64BitsDownCast<2> for Reg64Bits<56> {}
impl Reg64BitsConcat<2, 58> for Reg64Bits<56> {}
impl Reg64BitsDownCast<3> for Reg64Bits<56> {}
impl Reg64BitsConcat<3, 59> for Reg64Bits<56> {}
impl Reg64BitsDownCast<4> for Reg64Bits<56> {}
impl Reg64BitsConcat<4, 60> for Reg64Bits<56> {}
impl Reg64BitsDownCast<5> for Reg64Bits<56> {}
impl Reg64BitsConcat<5, 61> for Reg64Bits<56> {}
impl Reg64BitsDownCast<6> for Reg64Bits<56> {}
impl Reg64BitsConcat<6, 62> for Reg64Bits<56> {}
impl Reg64BitsDownCast<7> for Reg64Bits<56> {}
impl Reg64BitsConcat<7, 63> for Reg64Bits<56> {}
impl Reg64BitsDownCast<8> for Reg64Bits<56> {}
impl Reg64BitsConcat<8, 64> for Reg64Bits<56> {}
impl Reg64BitsDownCast<9> for Reg64Bits<56> {}
impl Reg64BitsDownCast<10> for Reg64Bits<56> {}
impl Reg64BitsDownCast<11> for Reg64Bits<56> {}
impl Reg64BitsDownCast<12> for Reg64Bits<56> {}
impl Reg64BitsDownCast<13> for Reg64Bits<56> {}
impl Reg64BitsDownCast<14> for Reg64Bits<56> {}
impl Reg64BitsDownCast<15> for Reg64Bits<56> {}
impl Reg64BitsDownCast<16> for Reg64Bits<56> {}
impl Reg64BitsDownCast<17> for Reg64Bits<56> {}
impl Reg64BitsDownCast<18> for Reg64Bits<56> {}
impl Reg64BitsDownCast<19> for Reg64Bits<56> {}
impl Reg64BitsDownCast<20> for Reg64Bits<56> {}
impl Reg64BitsDownCast<21> for Reg64Bits<56> {}
impl Reg64BitsDownCast<22> for Reg64Bits<56> {}
impl Reg64BitsDownCast<23> for Reg64Bits<56> {}
impl Reg64BitsDownCast<24> for Reg64Bits<56> {}
impl Reg64BitsDownCast<25> for Reg64Bits<56> {}
impl Reg64BitsDownCast<26> for Reg64Bits<56> {}
impl Reg64BitsDownCast<27> for Reg64Bits<56> {}
impl Reg64BitsDownCast<28> for Reg64Bits<56> {}
impl Reg64BitsDownCast<29> for Reg64Bits<56> {}
impl Reg64BitsDownCast<30> for Reg64Bits<56> {}
impl Reg64BitsDownCast<31> for Reg64Bits<56> {}
impl Reg64BitsDownCast<32> for Reg64Bits<56> {}
impl Reg64BitsDownCast<33> for Reg64Bits<56> {}
impl Reg64BitsDownCast<34> for Reg64Bits<56> {}
impl Reg64BitsDownCast<35> for Reg64Bits<56> {}
impl Reg64BitsDownCast<36> for Reg64Bits<56> {}
impl Reg64BitsDownCast<37> for Reg64Bits<56> {}
impl Reg64BitsDownCast<38> for Reg64Bits<56> {}
impl Reg64BitsDownCast<39> for Reg64Bits<56> {}
impl Reg64BitsDownCast<40> for Reg64Bits<56> {}
impl Reg64BitsDownCast<41> for Reg64Bits<56> {}
impl Reg64BitsDownCast<42> for Reg64Bits<56> {}
impl Reg64BitsDownCast<43> for Reg64Bits<56> {}
impl Reg64BitsDownCast<44> for Reg64Bits<56> {}
impl Reg64BitsDownCast<45> for Reg64Bits<56> {}
impl Reg64BitsDownCast<46> for Reg64Bits<56> {}
impl Reg64BitsDownCast<47> for Reg64Bits<56> {}
impl Reg64BitsDownCast<48> for Reg64Bits<56> {}
impl Reg64BitsDownCast<49> for Reg64Bits<56> {}
impl Reg64BitsDownCast<50> for Reg64Bits<56> {}
impl Reg64BitsDownCast<51> for Reg64Bits<56> {}
impl Reg64BitsDownCast<52> for Reg64Bits<56> {}
impl Reg64BitsDownCast<53> for Reg64Bits<56> {}
impl Reg64BitsDownCast<54> for Reg64Bits<56> {}
impl Reg64BitsDownCast<55> for Reg64Bits<56> {}
impl Reg64BitsDownCast<56> for Reg64Bits<56> {}
impl Reg64BitsDownCast<1> for Reg64Bits<57> {}
impl Reg64BitsConcat<1, 58> for Reg64Bits<57> {}
impl Reg64BitsDownCast<2> for Reg64Bits<57> {}
impl Reg64BitsConcat<2, 59> for Reg64Bits<57> {}
impl Reg64BitsDownCast<3> for Reg64Bits<57> {}
impl Reg64BitsConcat<3, 60> for Reg64Bits<57> {}
impl Reg64BitsDownCast<4> for Reg64Bits<57> {}
impl Reg64BitsConcat<4, 61> for Reg64Bits<57> {}
impl Reg64BitsDownCast<5> for Reg64Bits<57> {}
impl Reg64BitsConcat<5, 62> for Reg64Bits<57> {}
impl Reg64BitsDownCast<6> for Reg64Bits<57> {}
impl Reg64BitsConcat<6, 63> for Reg64Bits<57> {}
impl Reg64BitsDownCast<7> for Reg64Bits<57> {}
impl Reg64BitsConcat<7, 64> for Reg64Bits<57> {}
impl Reg64BitsDownCast<8> for Reg64Bits<57> {}
impl Reg64BitsDownCast<9> for Reg64Bits<57> {}
impl Reg64BitsDownCast<10> for Reg64Bits<57> {}
impl Reg64BitsDownCast<11> for Reg64Bits<57> {}
impl Reg64BitsDownCast<12> for Reg64Bits<57> {}
impl Reg64BitsDownCast<13> for Reg64Bits<57> {}
impl Reg64BitsDownCast<14> for Reg64Bits<57> {}
impl Reg64BitsDownCast<15> for Reg64Bits<57> {}
impl Reg64BitsDownCast<16> for Reg64Bits<57> {}
impl Reg64BitsDownCast<17> for Reg64Bits<57> {}
impl Reg64BitsDownCast<18> for Reg64Bits<57> {}
impl Reg64BitsDownCast<19> for Reg64Bits<57> {}
impl Reg64BitsDownCast<20> for Reg64Bits<57> {}
impl Reg64BitsDownCast<21> for Reg64Bits<57> {}
impl Reg64BitsDownCast<22> for Reg64Bits<57> {}
impl Reg64BitsDownCast<23> for Reg64Bits<57> {}
impl Reg64BitsDownCast<24> for Reg64Bits<57> {}
impl Reg64BitsDownCast<25> for Reg64Bits<57> {}
impl Reg64BitsDownCast<26> for Reg64Bits<57> {}
impl Reg64BitsDownCast<27> for Reg64Bits<57> {}
impl Reg64BitsDownCast<28> for Reg64Bits<57> {}
impl Reg64BitsDownCast<29> for Reg64Bits<57> {}
impl Reg64BitsDownCast<30> for Reg64Bits<57> {}
impl Reg64BitsDownCast<31> for Reg64Bits<57> {}
impl Reg64BitsDownCast<32> for Reg64Bits<57> {}
impl Reg64BitsDownCast<33> for Reg64Bits<57> {}
impl Reg64BitsDownCast<34> for Reg64Bits<57> {}
impl Reg64BitsDownCast<35> for Reg64Bits<57> {}
impl Reg64BitsDownCast<36> for Reg64Bits<57> {}
impl Reg64BitsDownCast<37> for Reg64Bits<57> {}
impl Reg64BitsDownCast<38> for Reg64Bits<57> {}
impl Reg64BitsDownCast<39> for Reg64Bits<57> {}
impl Reg64BitsDownCast<40> for Reg64Bits<57> {}
impl Reg64BitsDownCast<41> for Reg64Bits<57> {}
impl Reg64BitsDownCast<42> for Reg64Bits<57> {}
impl Reg64BitsDownCast<43> for Reg64Bits<57> {}
impl Reg64BitsDownCast<44> for Reg64Bits<57> {}
impl Reg64BitsDownCast<45> for Reg64Bits<57> {}
impl Reg64BitsDownCast<46> for Reg64Bits<57> {}
impl Reg64BitsDownCast<47> for Reg64Bits<57> {}
impl Reg64BitsDownCast<48> for Reg64Bits<57> {}
impl Reg64BitsDownCast<49> for Reg64Bits<57> {}
impl Reg64BitsDownCast<50> for Reg64Bits<57> {}
impl Reg64BitsDownCast<51> for Reg64Bits<57> {}
impl Reg64BitsDownCast<52> for Reg64Bits<57> {}
impl Reg64BitsDownCast<53> for Reg64Bits<57> {}
impl Reg64BitsDownCast<54> for Reg64Bits<57> {}
impl Reg64BitsDownCast<55> for Reg64Bits<57> {}
impl Reg64BitsDownCast<56> for Reg64Bits<57> {}
impl Reg64BitsDownCast<57> for Reg64Bits<57> {}
impl Reg64BitsDownCast<1> for Reg64Bits<58> {}
impl Reg64BitsConcat<1, 59> for Reg64Bits<58> {}
impl Reg64BitsDownCast<2> for Reg64Bits<58> {}
impl Reg64BitsConcat<2, 60> for Reg64Bits<58> {}
impl Reg64BitsDownCast<3> for Reg64Bits<58> {}
impl Reg64BitsConcat<3, 61> for Reg64Bits<58> {}
impl Reg64BitsDownCast<4> for Reg64Bits<58> {}
impl Reg64BitsConcat<4, 62> for Reg64Bits<58> {}
impl Reg64BitsDownCast<5> for Reg64Bits<58> {}
impl Reg64BitsConcat<5, 63> for Reg64Bits<58> {}
impl Reg64BitsDownCast<6> for Reg64Bits<58> {}
impl Reg64BitsConcat<6, 64> for Reg64Bits<58> {}
impl Reg64BitsDownCast<7> for Reg64Bits<58> {}
impl Reg64BitsDownCast<8> for Reg64Bits<58> {}
impl Reg64BitsDownCast<9> for Reg64Bits<58> {}
impl Reg64BitsDownCast<10> for Reg64Bits<58> {}
impl Reg64BitsDownCast<11> for Reg64Bits<58> {}
impl Reg64BitsDownCast<12> for Reg64Bits<58> {}
impl Reg64BitsDownCast<13> for Reg64Bits<58> {}
impl Reg64BitsDownCast<14> for Reg64Bits<58> {}
impl Reg64BitsDownCast<15> for Reg64Bits<58> {}
impl Reg64BitsDownCast<16> for Reg64Bits<58> {}
impl Reg64BitsDownCast<17> for Reg64Bits<58> {}
impl Reg64BitsDownCast<18> for Reg64Bits<58> {}
impl Reg64BitsDownCast<19> for Reg64Bits<58> {}
impl Reg64BitsDownCast<20> for Reg64Bits<58> {}
impl Reg64BitsDownCast<21> for Reg64Bits<58> {}
impl Reg64BitsDownCast<22> for Reg64Bits<58> {}
impl Reg64BitsDownCast<23> for Reg64Bits<58> {}
impl Reg64BitsDownCast<24> for Reg64Bits<58> {}
impl Reg64BitsDownCast<25> for Reg64Bits<58> {}
impl Reg64BitsDownCast<26> for Reg64Bits<58> {}
impl Reg64BitsDownCast<27> for Reg64Bits<58> {}
impl Reg64BitsDownCast<28> for Reg64Bits<58> {}
impl Reg64BitsDownCast<29> for Reg64Bits<58> {}
impl Reg64BitsDownCast<30> for Reg64Bits<58> {}
impl Reg64BitsDownCast<31> for Reg64Bits<58> {}
impl Reg64BitsDownCast<32> for Reg64Bits<58> {}
impl Reg64BitsDownCast<33> for Reg64Bits<58> {}
impl Reg64BitsDownCast<34> for Reg64Bits<58> {}
impl Reg64BitsDownCast<35> for Reg64Bits<58> {}
impl Reg64BitsDownCast<36> for Reg64Bits<58> {}
impl Reg64BitsDownCast<37> for Reg64Bits<58> {}
impl Reg64BitsDownCast<38> for Reg64Bits<58> {}
impl Reg64BitsDownCast<39> for Reg64Bits<58> {}
impl Reg64BitsDownCast<40> for Reg64Bits<58> {}
impl Reg64BitsDownCast<41> for Reg64Bits<58> {}
impl Reg64BitsDownCast<42> for Reg64Bits<58> {}
impl Reg64BitsDownCast<43> for Reg64Bits<58> {}
impl Reg64BitsDownCast<44> for Reg64Bits<58> {}
impl Reg64BitsDownCast<45> for Reg64Bits<58> {}
impl Reg64BitsDownCast<46> for Reg64Bits<58> {}
impl Reg64BitsDownCast<47> for Reg64Bits<58> {}
impl Reg64BitsDownCast<48> for Reg64Bits<58> {}
impl Reg64BitsDownCast<49> for Reg64Bits<58> {}
impl Reg64BitsDownCast<50> for Reg64Bits<58> {}
impl Reg64BitsDownCast<51> for Reg64Bits<58> {}
impl Reg64BitsDownCast<52> for Reg64Bits<58> {}
impl Reg64BitsDownCast<53> for Reg64Bits<58> {}
impl Reg64BitsDownCast<54> for Reg64Bits<58> {}
impl Reg64BitsDownCast<55> for Reg64Bits<58> {}
impl Reg64BitsDownCast<56> for Reg64Bits<58> {}
impl Reg64BitsDownCast<57> for Reg64Bits<58> {}
impl Reg64BitsDownCast<58> for Reg64Bits<58> {}
impl Reg64BitsDownCast<1> for Reg64Bits<59> {}
impl Reg64BitsConcat<1, 60> for Reg64Bits<59> {}
impl Reg64BitsDownCast<2> for Reg64Bits<59> {}
impl Reg64BitsConcat<2, 61> for Reg64Bits<59> {}
impl Reg64BitsDownCast<3> for Reg64Bits<59> {}
impl Reg64BitsConcat<3, 62> for Reg64Bits<59> {}
impl Reg64BitsDownCast<4> for Reg64Bits<59> {}
impl Reg64BitsConcat<4, 63> for Reg64Bits<59> {}
impl Reg64BitsDownCast<5> for Reg64Bits<59> {}
impl Reg64BitsConcat<5, 64> for Reg64Bits<59> {}
impl Reg64BitsDownCast<6> for Reg64Bits<59> {}
impl Reg64BitsDownCast<7> for Reg64Bits<59> {}
impl Reg64BitsDownCast<8> for Reg64Bits<59> {}
impl Reg64BitsDownCast<9> for Reg64Bits<59> {}
impl Reg64BitsDownCast<10> for Reg64Bits<59> {}
impl Reg64BitsDownCast<11> for Reg64Bits<59> {}
impl Reg64BitsDownCast<12> for Reg64Bits<59> {}
impl Reg64BitsDownCast<13> for Reg64Bits<59> {}
impl Reg64BitsDownCast<14> for Reg64Bits<59> {}
impl Reg64BitsDownCast<15> for Reg64Bits<59> {}
impl Reg64BitsDownCast<16> for Reg64Bits<59> {}
impl Reg64BitsDownCast<17> for Reg64Bits<59> {}
impl Reg64BitsDownCast<18> for Reg64Bits<59> {}
impl Reg64BitsDownCast<19> for Reg64Bits<59> {}
impl Reg64BitsDownCast<20> for Reg64Bits<59> {}
impl Reg64BitsDownCast<21> for Reg64Bits<59> {}
impl Reg64BitsDownCast<22> for Reg64Bits<59> {}
impl Reg64BitsDownCast<23> for Reg64Bits<59> {}
impl Reg64BitsDownCast<24> for Reg64Bits<59> {}
impl Reg64BitsDownCast<25> for Reg64Bits<59> {}
impl Reg64BitsDownCast<26> for Reg64Bits<59> {}
impl Reg64BitsDownCast<27> for Reg64Bits<59> {}
impl Reg64BitsDownCast<28> for Reg64Bits<59> {}
impl Reg64BitsDownCast<29> for Reg64Bits<59> {}
impl Reg64BitsDownCast<30> for Reg64Bits<59> {}
impl Reg64BitsDownCast<31> for Reg64Bits<59> {}
impl Reg64BitsDownCast<32> for Reg64Bits<59> {}
impl Reg64BitsDownCast<33> for Reg64Bits<59> {}
impl Reg64BitsDownCast<34> for Reg64Bits<59> {}
impl Reg64BitsDownCast<35> for Reg64Bits<59> {}
impl Reg64BitsDownCast<36> for Reg64Bits<59> {}
impl Reg64BitsDownCast<37> for Reg64Bits<59> {}
impl Reg64BitsDownCast<38> for Reg64Bits<59> {}
impl Reg64BitsDownCast<39> for Reg64Bits<59> {}
impl Reg64BitsDownCast<40> for Reg64Bits<59> {}
impl Reg64BitsDownCast<41> for Reg64Bits<59> {}
impl Reg64BitsDownCast<42> for Reg64Bits<59> {}
impl Reg64BitsDownCast<43> for Reg64Bits<59> {}
impl Reg64BitsDownCast<44> for Reg64Bits<59> {}
impl Reg64BitsDownCast<45> for Reg64Bits<59> {}
impl Reg64BitsDownCast<46> for Reg64Bits<59> {}
impl Reg64BitsDownCast<47> for Reg64Bits<59> {}
impl Reg64BitsDownCast<48> for Reg64Bits<59> {}
impl Reg64BitsDownCast<49> for Reg64Bits<59> {}
impl Reg64BitsDownCast<50> for Reg64Bits<59> {}
impl Reg64BitsDownCast<51> for Reg64Bits<59> {}
impl Reg64BitsDownCast<52> for Reg64Bits<59> {}
impl Reg64BitsDownCast<53> for Reg64Bits<59> {}
impl Reg64BitsDownCast<54> for Reg64Bits<59> {}
impl Reg64BitsDownCast<55> for Reg64Bits<59> {}
impl Reg64BitsDownCast<56> for Reg64Bits<59> {}
impl Reg64BitsDownCast<57> for Reg64Bits<59> {}
impl Reg64BitsDownCast<58> for Reg64Bits<59> {}
impl Reg64BitsDownCast<59> for Reg64Bits<59> {}
impl Reg64BitsDownCast<1> for Reg64Bits<60> {}
impl Reg64BitsConcat<1, 61> for Reg64Bits<60> {}
impl Reg64BitsDownCast<2> for Reg64Bits<60> {}
impl Reg64BitsConcat<2, 62> for Reg64Bits<60> {}
impl Reg64BitsDownCast<3> for Reg64Bits<60> {}
impl Reg64BitsConcat<3, 63> for Reg64Bits<60> {}
impl Reg64BitsDownCast<4> for Reg64Bits<60> {}
impl Reg64BitsConcat<4, 64> for Reg64Bits<60> {}
impl Reg64BitsDownCast<5> for Reg64Bits<60> {}
impl Reg64BitsDownCast<6> for Reg64Bits<60> {}
impl Reg64BitsDownCast<7> for Reg64Bits<60> {}
impl Reg64BitsDownCast<8> for Reg64Bits<60> {}
impl Reg64BitsDownCast<9> for Reg64Bits<60> {}
impl Reg64BitsDownCast<10> for Reg64Bits<60> {}
impl Reg64BitsDownCast<11> for Reg64Bits<60> {}
impl Reg64BitsDownCast<12> for Reg64Bits<60> {}
impl Reg64BitsDownCast<13> for Reg64Bits<60> {}
impl Reg64BitsDownCast<14> for Reg64Bits<60> {}
impl Reg64BitsDownCast<15> for Reg64Bits<60> {}
impl Reg64BitsDownCast<16> for Reg64Bits<60> {}
impl Reg64BitsDownCast<17> for Reg64Bits<60> {}
impl Reg64BitsDownCast<18> for Reg64Bits<60> {}
impl Reg64BitsDownCast<19> for Reg64Bits<60> {}
impl Reg64BitsDownCast<20> for Reg64Bits<60> {}
impl Reg64BitsDownCast<21> for Reg64Bits<60> {}
impl Reg64BitsDownCast<22> for Reg64Bits<60> {}
impl Reg64BitsDownCast<23> for Reg64Bits<60> {}
impl Reg64BitsDownCast<24> for Reg64Bits<60> {}
impl Reg64BitsDownCast<25> for Reg64Bits<60> {}
impl Reg64BitsDownCast<26> for Reg64Bits<60> {}
impl Reg64BitsDownCast<27> for Reg64Bits<60> {}
impl Reg64BitsDownCast<28> for Reg64Bits<60> {}
impl Reg64BitsDownCast<29> for Reg64Bits<60> {}
impl Reg64BitsDownCast<30> for Reg64Bits<60> {}
impl Reg64BitsDownCast<31> for Reg64Bits<60> {}
impl Reg64BitsDownCast<32> for Reg64Bits<60> {}
impl Reg64BitsDownCast<33> for Reg64Bits<60> {}
impl Reg64BitsDownCast<34> for Reg64Bits<60> {}
impl Reg64BitsDownCast<35> for Reg64Bits<60> {}
impl Reg64BitsDownCast<36> for Reg64Bits<60> {}
impl Reg64BitsDownCast<37> for Reg64Bits<60> {}
impl Reg64BitsDownCast<38> for Reg64Bits<60> {}
impl Reg64BitsDownCast<39> for Reg64Bits<60> {}
impl Reg64BitsDownCast<40> for Reg64Bits<60> {}
impl Reg64BitsDownCast<41> for Reg64Bits<60> {}
impl Reg64BitsDownCast<42> for Reg64Bits<60> {}
impl Reg64BitsDownCast<43> for Reg64Bits<60> {}
impl Reg64BitsDownCast<44> for Reg64Bits<60> {}
impl Reg64BitsDownCast<45> for Reg64Bits<60> {}
impl Reg64BitsDownCast<46> for Reg64Bits<60> {}
impl Reg64BitsDownCast<47> for Reg64Bits<60> {}
impl Reg64BitsDownCast<48> for Reg64Bits<60> {}
impl Reg64BitsDownCast<49> for Reg64Bits<60> {}
impl Reg64BitsDownCast<50> for Reg64Bits<60> {}
impl Reg64BitsDownCast<51> for Reg64Bits<60> {}
impl Reg64BitsDownCast<52> for Reg64Bits<60> {}
impl Reg64BitsDownCast<53> for Reg64Bits<60> {}
impl Reg64BitsDownCast<54> for Reg64Bits<60> {}
impl Reg64BitsDownCast<55> for Reg64Bits<60> {}
impl Reg64BitsDownCast<56> for Reg64Bits<60> {}
impl Reg64BitsDownCast<57> for Reg64Bits<60> {}
impl Reg64BitsDownCast<58> for Reg64Bits<60> {}
impl Reg64BitsDownCast<59> for Reg64Bits<60> {}
impl Reg64BitsDownCast<60> for Reg64Bits<60> {}
impl Reg64BitsDownCast<1> for Reg64Bits<61> {}
impl Reg64BitsConcat<1, 62> for Reg64Bits<61> {}
impl Reg64BitsDownCast<2> for Reg64Bits<61> {}
impl Reg64BitsConcat<2, 63> for Reg64Bits<61> {}
impl Reg64BitsDownCast<3> for Reg64Bits<61> {}
impl Reg64BitsConcat<3, 64> for Reg64Bits<61> {}
impl Reg64BitsDownCast<4> for Reg64Bits<61> {}
impl Reg64BitsDownCast<5> for Reg64Bits<61> {}
impl Reg64BitsDownCast<6> for Reg64Bits<61> {}
impl Reg64BitsDownCast<7> for Reg64Bits<61> {}
impl Reg64BitsDownCast<8> for Reg64Bits<61> {}
impl Reg64BitsDownCast<9> for Reg64Bits<61> {}
impl Reg64BitsDownCast<10> for Reg64Bits<61> {}
impl Reg64BitsDownCast<11> for Reg64Bits<61> {}
impl Reg64BitsDownCast<12> for Reg64Bits<61> {}
impl Reg64BitsDownCast<13> for Reg64Bits<61> {}
impl Reg64BitsDownCast<14> for Reg64Bits<61> {}
impl Reg64BitsDownCast<15> for Reg64Bits<61> {}
impl Reg64BitsDownCast<16> for Reg64Bits<61> {}
impl Reg64BitsDownCast<17> for Reg64Bits<61> {}
impl Reg64BitsDownCast<18> for Reg64Bits<61> {}
impl Reg64BitsDownCast<19> for Reg64Bits<61> {}
impl Reg64BitsDownCast<20> for Reg64Bits<61> {}
impl Reg64BitsDownCast<21> for Reg64Bits<61> {}
impl Reg64BitsDownCast<22> for Reg64Bits<61> {}
impl Reg64BitsDownCast<23> for Reg64Bits<61> {}
impl Reg64BitsDownCast<24> for Reg64Bits<61> {}
impl Reg64BitsDownCast<25> for Reg64Bits<61> {}
impl Reg64BitsDownCast<26> for Reg64Bits<61> {}
impl Reg64BitsDownCast<27> for Reg64Bits<61> {}
impl Reg64BitsDownCast<28> for Reg64Bits<61> {}
impl Reg64BitsDownCast<29> for Reg64Bits<61> {}
impl Reg64BitsDownCast<30> for Reg64Bits<61> {}
impl Reg64BitsDownCast<31> for Reg64Bits<61> {}
impl Reg64BitsDownCast<32> for Reg64Bits<61> {}
impl Reg64BitsDownCast<33> for Reg64Bits<61> {}
impl Reg64BitsDownCast<34> for Reg64Bits<61> {}
impl Reg64BitsDownCast<35> for Reg64Bits<61> {}
impl Reg64BitsDownCast<36> for Reg64Bits<61> {}
impl Reg64BitsDownCast<37> for Reg64Bits<61> {}
impl Reg64BitsDownCast<38> for Reg64Bits<61> {}
impl Reg64BitsDownCast<39> for Reg64Bits<61> {}
impl Reg64BitsDownCast<40> for Reg64Bits<61> {}
impl Reg64BitsDownCast<41> for Reg64Bits<61> {}
impl Reg64BitsDownCast<42> for Reg64Bits<61> {}
impl Reg64BitsDownCast<43> for Reg64Bits<61> {}
impl Reg64BitsDownCast<44> for Reg64Bits<61> {}
impl Reg64BitsDownCast<45> for Reg64Bits<61> {}
impl Reg64BitsDownCast<46> for Reg64Bits<61> {}
impl Reg64BitsDownCast<47> for Reg64Bits<61> {}
impl Reg64BitsDownCast<48> for Reg64Bits<61> {}
impl Reg64BitsDownCast<49> for Reg64Bits<61> {}
impl Reg64BitsDownCast<50> for Reg64Bits<61> {}
impl Reg64BitsDownCast<51> for Reg64Bits<61> {}
impl Reg64BitsDownCast<52> for Reg64Bits<61> {}
impl Reg64BitsDownCast<53> for Reg64Bits<61> {}
impl Reg64BitsDownCast<54> for Reg64Bits<61> {}
impl Reg64BitsDownCast<55> for Reg64Bits<61> {}
impl Reg64BitsDownCast<56> for Reg64Bits<61> {}
impl Reg64BitsDownCast<57> for Reg64Bits<61> {}
impl Reg64BitsDownCast<58> for Reg64Bits<61> {}
impl Reg64BitsDownCast<59> for Reg64Bits<61> {}
impl Reg64BitsDownCast<60> for Reg64Bits<61> {}
impl Reg64BitsDownCast<61> for Reg64Bits<61> {}
impl Reg64BitsDownCast<1> for Reg64Bits<62> {}
impl Reg64BitsConcat<1, 63> for Reg64Bits<62> {}
impl Reg64BitsDownCast<2> for Reg64Bits<62> {}
impl Reg64BitsConcat<2, 64> for Reg64Bits<62> {}
impl Reg64BitsDownCast<3> for Reg64Bits<62> {}
impl Reg64BitsDownCast<4> for Reg64Bits<62> {}
impl Reg64BitsDownCast<5> for Reg64Bits<62> {}
impl Reg64BitsDownCast<6> for Reg64Bits<62> {}
impl Reg64BitsDownCast<7> for Reg64Bits<62> {}
impl Reg64BitsDownCast<8> for Reg64Bits<62> {}
impl Reg64BitsDownCast<9> for Reg64Bits<62> {}
impl Reg64BitsDownCast<10> for Reg64Bits<62> {}
impl Reg64BitsDownCast<11> for Reg64Bits<62> {}
impl Reg64BitsDownCast<12> for Reg64Bits<62> {}
impl Reg64BitsDownCast<13> for Reg64Bits<62> {}
impl Reg64BitsDownCast<14> for Reg64Bits<62> {}
impl Reg64BitsDownCast<15> for Reg64Bits<62> {}
impl Reg64BitsDownCast<16> for Reg64Bits<62> {}
impl Reg64BitsDownCast<17> for Reg64Bits<62> {}
impl Reg64BitsDownCast<18> for Reg64Bits<62> {}
impl Reg64BitsDownCast<19> for Reg64Bits<62> {}
impl Reg64BitsDownCast<20> for Reg64Bits<62> {}
impl Reg64BitsDownCast<21> for Reg64Bits<62> {}
impl Reg64BitsDownCast<22> for Reg64Bits<62> {}
impl Reg64BitsDownCast<23> for Reg64Bits<62> {}
impl Reg64BitsDownCast<24> for Reg64Bits<62> {}
impl Reg64BitsDownCast<25> for Reg64Bits<62> {}
impl Reg64BitsDownCast<26> for Reg64Bits<62> {}
impl Reg64BitsDownCast<27> for Reg64Bits<62> {}
impl Reg64BitsDownCast<28> for Reg64Bits<62> {}
impl Reg64BitsDownCast<29> for Reg64Bits<62> {}
impl Reg64BitsDownCast<30> for Reg64Bits<62> {}
impl Reg64BitsDownCast<31> for Reg64Bits<62> {}
impl Reg64BitsDownCast<32> for Reg64Bits<62> {}
impl Reg64BitsDownCast<33> for Reg64Bits<62> {}
impl Reg64BitsDownCast<34> for Reg64Bits<62> {}
impl Reg64BitsDownCast<35> for Reg64Bits<62> {}
impl Reg64BitsDownCast<36> for Reg64Bits<62> {}
impl Reg64BitsDownCast<37> for Reg64Bits<62> {}
impl Reg64BitsDownCast<38> for Reg64Bits<62> {}
impl Reg64BitsDownCast<39> for Reg64Bits<62> {}
impl Reg64BitsDownCast<40> for Reg64Bits<62> {}
impl Reg64BitsDownCast<41> for Reg64Bits<62> {}
impl Reg64BitsDownCast<42> for Reg64Bits<62> {}
impl Reg64BitsDownCast<43> for Reg64Bits<62> {}
impl Reg64BitsDownCast<44> for Reg64Bits<62> {}
impl Reg64BitsDownCast<45> for Reg64Bits<62> {}
impl Reg64BitsDownCast<46> for Reg64Bits<62> {}
impl Reg64BitsDownCast<47> for Reg64Bits<62> {}
impl Reg64BitsDownCast<48> for Reg64Bits<62> {}
impl Reg64BitsDownCast<49> for Reg64Bits<62> {}
impl Reg64BitsDownCast<50> for Reg64Bits<62> {}
impl Reg64BitsDownCast<51> for Reg64Bits<62> {}
impl Reg64BitsDownCast<52> for Reg64Bits<62> {}
impl Reg64BitsDownCast<53> for Reg64Bits<62> {}
impl Reg64BitsDownCast<54> for Reg64Bits<62> {}
impl Reg64BitsDownCast<55> for Reg64Bits<62> {}
impl Reg64BitsDownCast<56> for Reg64Bits<62> {}
impl Reg64BitsDownCast<57> for Reg64Bits<62> {}
impl Reg64BitsDownCast<58> for Reg64Bits<62> {}
impl Reg64BitsDownCast<59> for Reg64Bits<62> {}
impl Reg64BitsDownCast<60> for Reg64Bits<62> {}
impl Reg64BitsDownCast<61> for Reg64Bits<62> {}
impl Reg64BitsDownCast<62> for Reg64Bits<62> {}
impl Reg64BitsDownCast<1> for Reg64Bits<63> {}
impl Reg64BitsConcat<1, 64> for Reg64Bits<63> {}
impl Reg64BitsDownCast<2> for Reg64Bits<63> {}
impl Reg64BitsDownCast<3> for Reg64Bits<63> {}
impl Reg64BitsDownCast<4> for Reg64Bits<63> {}
impl Reg64BitsDownCast<5> for Reg64Bits<63> {}
impl Reg64BitsDownCast<6> for Reg64Bits<63> {}
impl Reg64BitsDownCast<7> for Reg64Bits<63> {}
impl Reg64BitsDownCast<8> for Reg64Bits<63> {}
impl Reg64BitsDownCast<9> for Reg64Bits<63> {}
impl Reg64BitsDownCast<10> for Reg64Bits<63> {}
impl Reg64BitsDownCast<11> for Reg64Bits<63> {}
impl Reg64BitsDownCast<12> for Reg64Bits<63> {}
impl Reg64BitsDownCast<13> for Reg64Bits<63> {}
impl Reg64BitsDownCast<14> for Reg64Bits<63> {}
impl Reg64BitsDownCast<15> for Reg64Bits<63> {}
impl Reg64BitsDownCast<16> for Reg64Bits<63> {}
impl Reg64BitsDownCast<17> for Reg64Bits<63> {}
impl Reg64BitsDownCast<18> for Reg64Bits<63> {}
impl Reg64BitsDownCast<19> for Reg64Bits<63> {}
impl Reg64BitsDownCast<20> for Reg64Bits<63> {}
impl Reg64BitsDownCast<21> for Reg64Bits<63> {}
impl Reg64BitsDownCast<22> for Reg64Bits<63> {}
impl Reg64BitsDownCast<23> for Reg64Bits<63> {}
impl Reg64BitsDownCast<24> for Reg64Bits<63> {}
impl Reg64BitsDownCast<25> for Reg64Bits<63> {}
impl Reg64BitsDownCast<26> for Reg64Bits<63> {}
impl Reg64BitsDownCast<27> for Reg64Bits<63> {}
impl Reg64BitsDownCast<28> for Reg64Bits<63> {}
impl Reg64BitsDownCast<29> for Reg64Bits<63> {}
impl Reg64BitsDownCast<30> for Reg64Bits<63> {}
impl Reg64BitsDownCast<31> for Reg64Bits<63> {}
impl Reg64BitsDownCast<32> for Reg64Bits<63> {}
impl Reg64BitsDownCast<33> for Reg64Bits<63> {}
impl Reg64BitsDownCast<34> for Reg64Bits<63> {}
impl Reg64BitsDownCast<35> for Reg64Bits<63> {}
impl Reg64BitsDownCast<36> for Reg64Bits<63> {}
impl Reg64BitsDownCast<37> for Reg64Bits<63> {}
impl Reg64BitsDownCast<38> for Reg64Bits<63> {}
impl Reg64BitsDownCast<39> for Reg64Bits<63> {}
impl Reg64BitsDownCast<40> for Reg64Bits<63> {}
impl Reg64BitsDownCast<41> for Reg64Bits<63> {}
impl Reg64BitsDownCast<42> for Reg64Bits<63> {}
impl Reg64BitsDownCast<43> for Reg64Bits<63> {}
impl Reg64BitsDownCast<44> for Reg64Bits<63> {}
impl Reg64BitsDownCast<45> for Reg64Bits<63> {}
impl Reg64BitsDownCast<46> for Reg64Bits<63> {}
impl Reg64BitsDownCast<47> for Reg64Bits<63> {}
impl Reg64BitsDownCast<48> for Reg64Bits<63> {}
impl Reg64BitsDownCast<49> for Reg64Bits<63> {}
impl Reg64BitsDownCast<50> for Reg64Bits<63> {}
impl Reg64BitsDownCast<51> for Reg64Bits<63> {}
impl Reg64BitsDownCast<52> for Reg64Bits<63> {}
impl Reg64BitsDownCast<53> for Reg64Bits<63> {}
impl Reg64BitsDownCast<54> for Reg64Bits<63> {}
impl Reg64BitsDownCast<55> for Reg64Bits<63> {}
impl Reg64BitsDownCast<56> for Reg64Bits<63> {}
impl Reg64BitsDownCast<57> for Reg64Bits<63> {}
impl Reg64BitsDownCast<58> for Reg64Bits<63> {}
impl Reg64BitsDownCast<59> for Reg64Bits<63> {}
impl Reg64BitsDownCast<60> for Reg64Bits<63> {}
impl Reg64BitsDownCast<61> for Reg64Bits<63> {}
impl Reg64BitsDownCast<62> for Reg64Bits<63> {}
impl Reg64BitsDownCast<63> for Reg64Bits<63> {}
impl Reg64BitsDownCast<1> for Reg64Bits<64> {}
impl Reg64BitsDownCast<2> for Reg64Bits<64> {}
impl Reg64BitsDownCast<3> for Reg64Bits<64> {}
impl Reg64BitsDownCast<4> for Reg64Bits<64> {}
impl Reg64BitsDownCast<5> for Reg64Bits<64> {}
impl Reg64BitsDownCast<6> for Reg64Bits<64> {}
impl Reg64BitsDownCast<7> for Reg64Bits<64> {}
impl Reg64BitsDownCast<8> for Reg64Bits<64> {}
impl Reg64BitsDownCast<9> for Reg64Bits<64> {}
impl Reg64BitsDownCast<10> for Reg64Bits<64> {}
impl Reg64BitsDownCast<11> for Reg64Bits<64> {}
impl Reg64BitsDownCast<12> for Reg64Bits<64> {}
impl Reg64BitsDownCast<13> for Reg64Bits<64> {}
impl Reg64BitsDownCast<14> for Reg64Bits<64> {}
impl Reg64BitsDownCast<15> for Reg64Bits<64> {}
impl Reg64BitsDownCast<16> for Reg64Bits<64> {}
impl Reg64BitsDownCast<17> for Reg64Bits<64> {}
impl Reg64BitsDownCast<18> for Reg64Bits<64> {}
impl Reg64BitsDownCast<19> for Reg64Bits<64> {}
impl Reg64BitsDownCast<20> for Reg64Bits<64> {}
impl Reg64BitsDownCast<21> for Reg64Bits<64> {}
impl Reg64BitsDownCast<22> for Reg64Bits<64> {}
impl Reg64BitsDownCast<23> for Reg64Bits<64> {}
impl Reg64BitsDownCast<24> for Reg64Bits<64> {}
impl Reg64BitsDownCast<25> for Reg64Bits<64> {}
impl Reg64BitsDownCast<26> for Reg64Bits<64> {}
impl Reg64BitsDownCast<27> for Reg64Bits<64> {}
impl Reg64BitsDownCast<28> for Reg64Bits<64> {}
impl Reg64BitsDownCast<29> for Reg64Bits<64> {}
impl Reg64BitsDownCast<30> for Reg64Bits<64> {}
impl Reg64BitsDownCast<31> for Reg64Bits<64> {}
impl Reg64BitsDownCast<32> for Reg64Bits<64> {}
impl Reg64BitsDownCast<33> for Reg64Bits<64> {}
impl Reg64BitsDownCast<34> for Reg64Bits<64> {}
impl Reg64BitsDownCast<35> for Reg64Bits<64> {}
impl Reg64BitsDownCast<36> for Reg64Bits<64> {}
impl Reg64BitsDownCast<37> for Reg64Bits<64> {}
impl Reg64BitsDownCast<38> for Reg64Bits<64> {}
impl Reg64BitsDownCast<39> for Reg64Bits<64> {}
impl Reg64BitsDownCast<40> for Reg64Bits<64> {}
impl Reg64BitsDownCast<41> for Reg64Bits<64> {}
impl Reg64BitsDownCast<42> for Reg64Bits<64> {}
impl Reg64BitsDownCast<43> for Reg64Bits<64> {}
impl Reg64BitsDownCast<44> for Reg64Bits<64> {}
impl Reg64BitsDownCast<45> for Reg64Bits<64> {}
impl Reg64BitsDownCast<46> for Reg64Bits<64> {}
impl Reg64BitsDownCast<47> for Reg64Bits<64> {}
impl Reg64BitsDownCast<48> for Reg64Bits<64> {}
impl Reg64BitsDownCast<49> for Reg64Bits<64> {}
impl Reg64BitsDownCast<50> for Reg64Bits<64> {}
impl Reg64BitsDownCast<51> for Reg64Bits<64> {}
impl Reg64BitsDownCast<52> for Reg64Bits<64> {}
impl Reg64BitsDownCast<53> for Reg64Bits<64> {}
impl Reg64BitsDownCast<54> for Reg64Bits<64> {}
impl Reg64BitsDownCast<55> for Reg64Bits<64> {}
impl Reg64BitsDownCast<56> for Reg64Bits<64> {}
impl Reg64BitsDownCast<57> for Reg64Bits<64> {}
impl Reg64BitsDownCast<58> for Reg64Bits<64> {}
impl Reg64BitsDownCast<59> for Reg64Bits<64> {}
impl Reg64BitsDownCast<60> for Reg64Bits<64> {}
impl Reg64BitsDownCast<61> for Reg64Bits<64> {}
impl Reg64BitsDownCast<62> for Reg64Bits<64> {}
impl Reg64BitsDownCast<63> for Reg64Bits<64> {}
impl Reg64BitsDownCast<64> for Reg64Bits<64> {}
