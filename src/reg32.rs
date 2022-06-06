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
type BaseType = u32; // [REF_REPLACE]
const NUM_BITS: usize = BaseType::BITS as usize;

#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[repr(transparent)]
pub struct Reg32Bits<const N: usize>(BaseType);

impl<const N: usize> core::ops::Deref for Reg32Bits<N> {
    type Target = BaseType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Eq for Reg32Bits<N> {}
impl<const N: usize> Ord for Reg32Bits<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
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
    
    1 << (NUM_BITS - 1)
}

// Function to ease matching of Bits to a specific sequence of bits
impl<const N: usize> Reg32Bits<N> {
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

impl<const N: usize> core::ops::Add for Reg32Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.0);

        Self(((lhs + rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize> core::ops::Sub for Reg32Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = Wrapping::<BaseType>(self.0);
        let rhs = Wrapping::<BaseType>(rhs.0);

        Self(((lhs - rhs) & Wrapping(Self::BASE_ONES)).0)
    }
}

impl<const N: usize> core::ops::Div for Reg32Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

        Self(lhs / rhs)
    }
}

impl<const N: usize> core::ops::Rem for Reg32Bits<N> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: Self) -> Self::Output {
        let lhs = self.0;
        let rhs = rhs.0;

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

impl<const N: usize, T> core::ops::Shr<T> for Reg32Bits<N>
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
pub trait Reg32BitsDownCast<const T: usize>: Copy + Into<BaseType> {
    #[inline(always)]
    fn take_low(self) -> Reg32Bits<T> {
        let value: BaseType = self.into();
        Reg32Bits(Reg32Bits::<T>::BASE_ONES & value)
    }
    #[inline(always)]
    fn take_high(self) -> Reg32Bits<T> {
        let value: BaseType = self.into();
        Reg32Bits(value >> (NUM_BITS - T))
    }
}

pub trait Reg32BitsUpCast<const T: usize>: Copy + Into<BaseType> {
    #[inline(always)]
    fn zero_extend(self) -> Reg32Bits<T> {
        let value = self.into();
        Reg32Bits(value)
    }

    fn sign_extend(self) -> Reg32Bits<T> {
        // NOTE: We are assuming here that no Bits<0> structure can exist
        let value = self.into();

        let top_bit = value & Reg32Bits::<T>::TOP_BIT_MASK; // Capture only the top bit
        let top_bits = if top_bit == 0 { // Create a set of NUM_BITS-N bits of with the given sign
            0
        } else {
            !Reg32Bits::<T>::BASE_ONES // !001111 = 110000
        };

        Reg32Bits(top_bits & value)
    }
}

pub trait Reg32BitsConcat<const R: usize, const O: usize>: Copy + Into<BaseType> {
    fn concat(self, rhs: Reg32Bits<R>) -> Reg32Bits<O> {
        let lhs = self.into();
        let rhs: BaseType = rhs.into();

        Reg32Bits((lhs << R) | rhs)
    }
}

impl<const F: usize, const T: usize> Reg32BitsUpCast<T> for Reg32Bits<F>
where
    Reg32Bits<T>: Reg32BitsDownCast<F>,
{
}



impl Reg32BitsDownCast<1> for Reg32Bits<1> {}
impl Reg32BitsConcat<1, 2> for Reg32Bits<1> {}
impl Reg32BitsDownCast<1> for Reg32Bits<2> {}
impl Reg32BitsConcat<1, 3> for Reg32Bits<2> {}
impl Reg32BitsDownCast<2> for Reg32Bits<2> {}
impl Reg32BitsConcat<2, 4> for Reg32Bits<2> {}
impl Reg32BitsDownCast<1> for Reg32Bits<3> {}
impl Reg32BitsConcat<1, 4> for Reg32Bits<3> {}
impl Reg32BitsDownCast<2> for Reg32Bits<3> {}
impl Reg32BitsConcat<2, 5> for Reg32Bits<3> {}
impl Reg32BitsDownCast<3> for Reg32Bits<3> {}
impl Reg32BitsConcat<3, 6> for Reg32Bits<3> {}
impl Reg32BitsDownCast<1> for Reg32Bits<4> {}
impl Reg32BitsConcat<1, 5> for Reg32Bits<4> {}
impl Reg32BitsDownCast<2> for Reg32Bits<4> {}
impl Reg32BitsConcat<2, 6> for Reg32Bits<4> {}
impl Reg32BitsDownCast<3> for Reg32Bits<4> {}
impl Reg32BitsConcat<3, 7> for Reg32Bits<4> {}
impl Reg32BitsDownCast<4> for Reg32Bits<4> {}
impl Reg32BitsConcat<4, 8> for Reg32Bits<4> {}
impl Reg32BitsDownCast<1> for Reg32Bits<5> {}
impl Reg32BitsConcat<1, 6> for Reg32Bits<5> {}
impl Reg32BitsDownCast<2> for Reg32Bits<5> {}
impl Reg32BitsConcat<2, 7> for Reg32Bits<5> {}
impl Reg32BitsDownCast<3> for Reg32Bits<5> {}
impl Reg32BitsConcat<3, 8> for Reg32Bits<5> {}
impl Reg32BitsDownCast<4> for Reg32Bits<5> {}
impl Reg32BitsConcat<4, 9> for Reg32Bits<5> {}
impl Reg32BitsDownCast<5> for Reg32Bits<5> {}
impl Reg32BitsConcat<5, 10> for Reg32Bits<5> {}
impl Reg32BitsDownCast<1> for Reg32Bits<6> {}
impl Reg32BitsConcat<1, 7> for Reg32Bits<6> {}
impl Reg32BitsDownCast<2> for Reg32Bits<6> {}
impl Reg32BitsConcat<2, 8> for Reg32Bits<6> {}
impl Reg32BitsDownCast<3> for Reg32Bits<6> {}
impl Reg32BitsConcat<3, 9> for Reg32Bits<6> {}
impl Reg32BitsDownCast<4> for Reg32Bits<6> {}
impl Reg32BitsConcat<4, 10> for Reg32Bits<6> {}
impl Reg32BitsDownCast<5> for Reg32Bits<6> {}
impl Reg32BitsConcat<5, 11> for Reg32Bits<6> {}
impl Reg32BitsDownCast<6> for Reg32Bits<6> {}
impl Reg32BitsConcat<6, 12> for Reg32Bits<6> {}
impl Reg32BitsDownCast<1> for Reg32Bits<7> {}
impl Reg32BitsConcat<1, 8> for Reg32Bits<7> {}
impl Reg32BitsDownCast<2> for Reg32Bits<7> {}
impl Reg32BitsConcat<2, 9> for Reg32Bits<7> {}
impl Reg32BitsDownCast<3> for Reg32Bits<7> {}
impl Reg32BitsConcat<3, 10> for Reg32Bits<7> {}
impl Reg32BitsDownCast<4> for Reg32Bits<7> {}
impl Reg32BitsConcat<4, 11> for Reg32Bits<7> {}
impl Reg32BitsDownCast<5> for Reg32Bits<7> {}
impl Reg32BitsConcat<5, 12> for Reg32Bits<7> {}
impl Reg32BitsDownCast<6> for Reg32Bits<7> {}
impl Reg32BitsConcat<6, 13> for Reg32Bits<7> {}
impl Reg32BitsDownCast<7> for Reg32Bits<7> {}
impl Reg32BitsConcat<7, 14> for Reg32Bits<7> {}
impl Reg32BitsDownCast<1> for Reg32Bits<8> {}
impl Reg32BitsConcat<1, 9> for Reg32Bits<8> {}
impl Reg32BitsDownCast<2> for Reg32Bits<8> {}
impl Reg32BitsConcat<2, 10> for Reg32Bits<8> {}
impl Reg32BitsDownCast<3> for Reg32Bits<8> {}
impl Reg32BitsConcat<3, 11> for Reg32Bits<8> {}
impl Reg32BitsDownCast<4> for Reg32Bits<8> {}
impl Reg32BitsConcat<4, 12> for Reg32Bits<8> {}
impl Reg32BitsDownCast<5> for Reg32Bits<8> {}
impl Reg32BitsConcat<5, 13> for Reg32Bits<8> {}
impl Reg32BitsDownCast<6> for Reg32Bits<8> {}
impl Reg32BitsConcat<6, 14> for Reg32Bits<8> {}
impl Reg32BitsDownCast<7> for Reg32Bits<8> {}
impl Reg32BitsConcat<7, 15> for Reg32Bits<8> {}
impl Reg32BitsDownCast<8> for Reg32Bits<8> {}
impl Reg32BitsConcat<8, 16> for Reg32Bits<8> {}
impl Reg32BitsDownCast<1> for Reg32Bits<9> {}
impl Reg32BitsConcat<1, 10> for Reg32Bits<9> {}
impl Reg32BitsDownCast<2> for Reg32Bits<9> {}
impl Reg32BitsConcat<2, 11> for Reg32Bits<9> {}
impl Reg32BitsDownCast<3> for Reg32Bits<9> {}
impl Reg32BitsConcat<3, 12> for Reg32Bits<9> {}
impl Reg32BitsDownCast<4> for Reg32Bits<9> {}
impl Reg32BitsConcat<4, 13> for Reg32Bits<9> {}
impl Reg32BitsDownCast<5> for Reg32Bits<9> {}
impl Reg32BitsConcat<5, 14> for Reg32Bits<9> {}
impl Reg32BitsDownCast<6> for Reg32Bits<9> {}
impl Reg32BitsConcat<6, 15> for Reg32Bits<9> {}
impl Reg32BitsDownCast<7> for Reg32Bits<9> {}
impl Reg32BitsConcat<7, 16> for Reg32Bits<9> {}
impl Reg32BitsDownCast<8> for Reg32Bits<9> {}
impl Reg32BitsConcat<8, 17> for Reg32Bits<9> {}
impl Reg32BitsDownCast<9> for Reg32Bits<9> {}
impl Reg32BitsConcat<9, 18> for Reg32Bits<9> {}
impl Reg32BitsDownCast<1> for Reg32Bits<10> {}
impl Reg32BitsConcat<1, 11> for Reg32Bits<10> {}
impl Reg32BitsDownCast<2> for Reg32Bits<10> {}
impl Reg32BitsConcat<2, 12> for Reg32Bits<10> {}
impl Reg32BitsDownCast<3> for Reg32Bits<10> {}
impl Reg32BitsConcat<3, 13> for Reg32Bits<10> {}
impl Reg32BitsDownCast<4> for Reg32Bits<10> {}
impl Reg32BitsConcat<4, 14> for Reg32Bits<10> {}
impl Reg32BitsDownCast<5> for Reg32Bits<10> {}
impl Reg32BitsConcat<5, 15> for Reg32Bits<10> {}
impl Reg32BitsDownCast<6> for Reg32Bits<10> {}
impl Reg32BitsConcat<6, 16> for Reg32Bits<10> {}
impl Reg32BitsDownCast<7> for Reg32Bits<10> {}
impl Reg32BitsConcat<7, 17> for Reg32Bits<10> {}
impl Reg32BitsDownCast<8> for Reg32Bits<10> {}
impl Reg32BitsConcat<8, 18> for Reg32Bits<10> {}
impl Reg32BitsDownCast<9> for Reg32Bits<10> {}
impl Reg32BitsConcat<9, 19> for Reg32Bits<10> {}
impl Reg32BitsDownCast<10> for Reg32Bits<10> {}
impl Reg32BitsConcat<10, 20> for Reg32Bits<10> {}
impl Reg32BitsDownCast<1> for Reg32Bits<11> {}
impl Reg32BitsConcat<1, 12> for Reg32Bits<11> {}
impl Reg32BitsDownCast<2> for Reg32Bits<11> {}
impl Reg32BitsConcat<2, 13> for Reg32Bits<11> {}
impl Reg32BitsDownCast<3> for Reg32Bits<11> {}
impl Reg32BitsConcat<3, 14> for Reg32Bits<11> {}
impl Reg32BitsDownCast<4> for Reg32Bits<11> {}
impl Reg32BitsConcat<4, 15> for Reg32Bits<11> {}
impl Reg32BitsDownCast<5> for Reg32Bits<11> {}
impl Reg32BitsConcat<5, 16> for Reg32Bits<11> {}
impl Reg32BitsDownCast<6> for Reg32Bits<11> {}
impl Reg32BitsConcat<6, 17> for Reg32Bits<11> {}
impl Reg32BitsDownCast<7> for Reg32Bits<11> {}
impl Reg32BitsConcat<7, 18> for Reg32Bits<11> {}
impl Reg32BitsDownCast<8> for Reg32Bits<11> {}
impl Reg32BitsConcat<8, 19> for Reg32Bits<11> {}
impl Reg32BitsDownCast<9> for Reg32Bits<11> {}
impl Reg32BitsConcat<9, 20> for Reg32Bits<11> {}
impl Reg32BitsDownCast<10> for Reg32Bits<11> {}
impl Reg32BitsConcat<10, 21> for Reg32Bits<11> {}
impl Reg32BitsDownCast<11> for Reg32Bits<11> {}
impl Reg32BitsConcat<11, 22> for Reg32Bits<11> {}
impl Reg32BitsDownCast<1> for Reg32Bits<12> {}
impl Reg32BitsConcat<1, 13> for Reg32Bits<12> {}
impl Reg32BitsDownCast<2> for Reg32Bits<12> {}
impl Reg32BitsConcat<2, 14> for Reg32Bits<12> {}
impl Reg32BitsDownCast<3> for Reg32Bits<12> {}
impl Reg32BitsConcat<3, 15> for Reg32Bits<12> {}
impl Reg32BitsDownCast<4> for Reg32Bits<12> {}
impl Reg32BitsConcat<4, 16> for Reg32Bits<12> {}
impl Reg32BitsDownCast<5> for Reg32Bits<12> {}
impl Reg32BitsConcat<5, 17> for Reg32Bits<12> {}
impl Reg32BitsDownCast<6> for Reg32Bits<12> {}
impl Reg32BitsConcat<6, 18> for Reg32Bits<12> {}
impl Reg32BitsDownCast<7> for Reg32Bits<12> {}
impl Reg32BitsConcat<7, 19> for Reg32Bits<12> {}
impl Reg32BitsDownCast<8> for Reg32Bits<12> {}
impl Reg32BitsConcat<8, 20> for Reg32Bits<12> {}
impl Reg32BitsDownCast<9> for Reg32Bits<12> {}
impl Reg32BitsConcat<9, 21> for Reg32Bits<12> {}
impl Reg32BitsDownCast<10> for Reg32Bits<12> {}
impl Reg32BitsConcat<10, 22> for Reg32Bits<12> {}
impl Reg32BitsDownCast<11> for Reg32Bits<12> {}
impl Reg32BitsConcat<11, 23> for Reg32Bits<12> {}
impl Reg32BitsDownCast<12> for Reg32Bits<12> {}
impl Reg32BitsConcat<12, 24> for Reg32Bits<12> {}
impl Reg32BitsDownCast<1> for Reg32Bits<13> {}
impl Reg32BitsConcat<1, 14> for Reg32Bits<13> {}
impl Reg32BitsDownCast<2> for Reg32Bits<13> {}
impl Reg32BitsConcat<2, 15> for Reg32Bits<13> {}
impl Reg32BitsDownCast<3> for Reg32Bits<13> {}
impl Reg32BitsConcat<3, 16> for Reg32Bits<13> {}
impl Reg32BitsDownCast<4> for Reg32Bits<13> {}
impl Reg32BitsConcat<4, 17> for Reg32Bits<13> {}
impl Reg32BitsDownCast<5> for Reg32Bits<13> {}
impl Reg32BitsConcat<5, 18> for Reg32Bits<13> {}
impl Reg32BitsDownCast<6> for Reg32Bits<13> {}
impl Reg32BitsConcat<6, 19> for Reg32Bits<13> {}
impl Reg32BitsDownCast<7> for Reg32Bits<13> {}
impl Reg32BitsConcat<7, 20> for Reg32Bits<13> {}
impl Reg32BitsDownCast<8> for Reg32Bits<13> {}
impl Reg32BitsConcat<8, 21> for Reg32Bits<13> {}
impl Reg32BitsDownCast<9> for Reg32Bits<13> {}
impl Reg32BitsConcat<9, 22> for Reg32Bits<13> {}
impl Reg32BitsDownCast<10> for Reg32Bits<13> {}
impl Reg32BitsConcat<10, 23> for Reg32Bits<13> {}
impl Reg32BitsDownCast<11> for Reg32Bits<13> {}
impl Reg32BitsConcat<11, 24> for Reg32Bits<13> {}
impl Reg32BitsDownCast<12> for Reg32Bits<13> {}
impl Reg32BitsConcat<12, 25> for Reg32Bits<13> {}
impl Reg32BitsDownCast<13> for Reg32Bits<13> {}
impl Reg32BitsConcat<13, 26> for Reg32Bits<13> {}
impl Reg32BitsDownCast<1> for Reg32Bits<14> {}
impl Reg32BitsConcat<1, 15> for Reg32Bits<14> {}
impl Reg32BitsDownCast<2> for Reg32Bits<14> {}
impl Reg32BitsConcat<2, 16> for Reg32Bits<14> {}
impl Reg32BitsDownCast<3> for Reg32Bits<14> {}
impl Reg32BitsConcat<3, 17> for Reg32Bits<14> {}
impl Reg32BitsDownCast<4> for Reg32Bits<14> {}
impl Reg32BitsConcat<4, 18> for Reg32Bits<14> {}
impl Reg32BitsDownCast<5> for Reg32Bits<14> {}
impl Reg32BitsConcat<5, 19> for Reg32Bits<14> {}
impl Reg32BitsDownCast<6> for Reg32Bits<14> {}
impl Reg32BitsConcat<6, 20> for Reg32Bits<14> {}
impl Reg32BitsDownCast<7> for Reg32Bits<14> {}
impl Reg32BitsConcat<7, 21> for Reg32Bits<14> {}
impl Reg32BitsDownCast<8> for Reg32Bits<14> {}
impl Reg32BitsConcat<8, 22> for Reg32Bits<14> {}
impl Reg32BitsDownCast<9> for Reg32Bits<14> {}
impl Reg32BitsConcat<9, 23> for Reg32Bits<14> {}
impl Reg32BitsDownCast<10> for Reg32Bits<14> {}
impl Reg32BitsConcat<10, 24> for Reg32Bits<14> {}
impl Reg32BitsDownCast<11> for Reg32Bits<14> {}
impl Reg32BitsConcat<11, 25> for Reg32Bits<14> {}
impl Reg32BitsDownCast<12> for Reg32Bits<14> {}
impl Reg32BitsConcat<12, 26> for Reg32Bits<14> {}
impl Reg32BitsDownCast<13> for Reg32Bits<14> {}
impl Reg32BitsConcat<13, 27> for Reg32Bits<14> {}
impl Reg32BitsDownCast<14> for Reg32Bits<14> {}
impl Reg32BitsConcat<14, 28> for Reg32Bits<14> {}
impl Reg32BitsDownCast<1> for Reg32Bits<15> {}
impl Reg32BitsConcat<1, 16> for Reg32Bits<15> {}
impl Reg32BitsDownCast<2> for Reg32Bits<15> {}
impl Reg32BitsConcat<2, 17> for Reg32Bits<15> {}
impl Reg32BitsDownCast<3> for Reg32Bits<15> {}
impl Reg32BitsConcat<3, 18> for Reg32Bits<15> {}
impl Reg32BitsDownCast<4> for Reg32Bits<15> {}
impl Reg32BitsConcat<4, 19> for Reg32Bits<15> {}
impl Reg32BitsDownCast<5> for Reg32Bits<15> {}
impl Reg32BitsConcat<5, 20> for Reg32Bits<15> {}
impl Reg32BitsDownCast<6> for Reg32Bits<15> {}
impl Reg32BitsConcat<6, 21> for Reg32Bits<15> {}
impl Reg32BitsDownCast<7> for Reg32Bits<15> {}
impl Reg32BitsConcat<7, 22> for Reg32Bits<15> {}
impl Reg32BitsDownCast<8> for Reg32Bits<15> {}
impl Reg32BitsConcat<8, 23> for Reg32Bits<15> {}
impl Reg32BitsDownCast<9> for Reg32Bits<15> {}
impl Reg32BitsConcat<9, 24> for Reg32Bits<15> {}
impl Reg32BitsDownCast<10> for Reg32Bits<15> {}
impl Reg32BitsConcat<10, 25> for Reg32Bits<15> {}
impl Reg32BitsDownCast<11> for Reg32Bits<15> {}
impl Reg32BitsConcat<11, 26> for Reg32Bits<15> {}
impl Reg32BitsDownCast<12> for Reg32Bits<15> {}
impl Reg32BitsConcat<12, 27> for Reg32Bits<15> {}
impl Reg32BitsDownCast<13> for Reg32Bits<15> {}
impl Reg32BitsConcat<13, 28> for Reg32Bits<15> {}
impl Reg32BitsDownCast<14> for Reg32Bits<15> {}
impl Reg32BitsConcat<14, 29> for Reg32Bits<15> {}
impl Reg32BitsDownCast<15> for Reg32Bits<15> {}
impl Reg32BitsConcat<15, 30> for Reg32Bits<15> {}
impl Reg32BitsDownCast<1> for Reg32Bits<16> {}
impl Reg32BitsConcat<1, 17> for Reg32Bits<16> {}
impl Reg32BitsDownCast<2> for Reg32Bits<16> {}
impl Reg32BitsConcat<2, 18> for Reg32Bits<16> {}
impl Reg32BitsDownCast<3> for Reg32Bits<16> {}
impl Reg32BitsConcat<3, 19> for Reg32Bits<16> {}
impl Reg32BitsDownCast<4> for Reg32Bits<16> {}
impl Reg32BitsConcat<4, 20> for Reg32Bits<16> {}
impl Reg32BitsDownCast<5> for Reg32Bits<16> {}
impl Reg32BitsConcat<5, 21> for Reg32Bits<16> {}
impl Reg32BitsDownCast<6> for Reg32Bits<16> {}
impl Reg32BitsConcat<6, 22> for Reg32Bits<16> {}
impl Reg32BitsDownCast<7> for Reg32Bits<16> {}
impl Reg32BitsConcat<7, 23> for Reg32Bits<16> {}
impl Reg32BitsDownCast<8> for Reg32Bits<16> {}
impl Reg32BitsConcat<8, 24> for Reg32Bits<16> {}
impl Reg32BitsDownCast<9> for Reg32Bits<16> {}
impl Reg32BitsConcat<9, 25> for Reg32Bits<16> {}
impl Reg32BitsDownCast<10> for Reg32Bits<16> {}
impl Reg32BitsConcat<10, 26> for Reg32Bits<16> {}
impl Reg32BitsDownCast<11> for Reg32Bits<16> {}
impl Reg32BitsConcat<11, 27> for Reg32Bits<16> {}
impl Reg32BitsDownCast<12> for Reg32Bits<16> {}
impl Reg32BitsConcat<12, 28> for Reg32Bits<16> {}
impl Reg32BitsDownCast<13> for Reg32Bits<16> {}
impl Reg32BitsConcat<13, 29> for Reg32Bits<16> {}
impl Reg32BitsDownCast<14> for Reg32Bits<16> {}
impl Reg32BitsConcat<14, 30> for Reg32Bits<16> {}
impl Reg32BitsDownCast<15> for Reg32Bits<16> {}
impl Reg32BitsConcat<15, 31> for Reg32Bits<16> {}
impl Reg32BitsDownCast<16> for Reg32Bits<16> {}
impl Reg32BitsConcat<16, 32> for Reg32Bits<16> {}
impl Reg32BitsDownCast<1> for Reg32Bits<17> {}
impl Reg32BitsConcat<1, 18> for Reg32Bits<17> {}
impl Reg32BitsDownCast<2> for Reg32Bits<17> {}
impl Reg32BitsConcat<2, 19> for Reg32Bits<17> {}
impl Reg32BitsDownCast<3> for Reg32Bits<17> {}
impl Reg32BitsConcat<3, 20> for Reg32Bits<17> {}
impl Reg32BitsDownCast<4> for Reg32Bits<17> {}
impl Reg32BitsConcat<4, 21> for Reg32Bits<17> {}
impl Reg32BitsDownCast<5> for Reg32Bits<17> {}
impl Reg32BitsConcat<5, 22> for Reg32Bits<17> {}
impl Reg32BitsDownCast<6> for Reg32Bits<17> {}
impl Reg32BitsConcat<6, 23> for Reg32Bits<17> {}
impl Reg32BitsDownCast<7> for Reg32Bits<17> {}
impl Reg32BitsConcat<7, 24> for Reg32Bits<17> {}
impl Reg32BitsDownCast<8> for Reg32Bits<17> {}
impl Reg32BitsConcat<8, 25> for Reg32Bits<17> {}
impl Reg32BitsDownCast<9> for Reg32Bits<17> {}
impl Reg32BitsConcat<9, 26> for Reg32Bits<17> {}
impl Reg32BitsDownCast<10> for Reg32Bits<17> {}
impl Reg32BitsConcat<10, 27> for Reg32Bits<17> {}
impl Reg32BitsDownCast<11> for Reg32Bits<17> {}
impl Reg32BitsConcat<11, 28> for Reg32Bits<17> {}
impl Reg32BitsDownCast<12> for Reg32Bits<17> {}
impl Reg32BitsConcat<12, 29> for Reg32Bits<17> {}
impl Reg32BitsDownCast<13> for Reg32Bits<17> {}
impl Reg32BitsConcat<13, 30> for Reg32Bits<17> {}
impl Reg32BitsDownCast<14> for Reg32Bits<17> {}
impl Reg32BitsConcat<14, 31> for Reg32Bits<17> {}
impl Reg32BitsDownCast<15> for Reg32Bits<17> {}
impl Reg32BitsConcat<15, 32> for Reg32Bits<17> {}
impl Reg32BitsDownCast<16> for Reg32Bits<17> {}
impl Reg32BitsDownCast<17> for Reg32Bits<17> {}
impl Reg32BitsDownCast<1> for Reg32Bits<18> {}
impl Reg32BitsConcat<1, 19> for Reg32Bits<18> {}
impl Reg32BitsDownCast<2> for Reg32Bits<18> {}
impl Reg32BitsConcat<2, 20> for Reg32Bits<18> {}
impl Reg32BitsDownCast<3> for Reg32Bits<18> {}
impl Reg32BitsConcat<3, 21> for Reg32Bits<18> {}
impl Reg32BitsDownCast<4> for Reg32Bits<18> {}
impl Reg32BitsConcat<4, 22> for Reg32Bits<18> {}
impl Reg32BitsDownCast<5> for Reg32Bits<18> {}
impl Reg32BitsConcat<5, 23> for Reg32Bits<18> {}
impl Reg32BitsDownCast<6> for Reg32Bits<18> {}
impl Reg32BitsConcat<6, 24> for Reg32Bits<18> {}
impl Reg32BitsDownCast<7> for Reg32Bits<18> {}
impl Reg32BitsConcat<7, 25> for Reg32Bits<18> {}
impl Reg32BitsDownCast<8> for Reg32Bits<18> {}
impl Reg32BitsConcat<8, 26> for Reg32Bits<18> {}
impl Reg32BitsDownCast<9> for Reg32Bits<18> {}
impl Reg32BitsConcat<9, 27> for Reg32Bits<18> {}
impl Reg32BitsDownCast<10> for Reg32Bits<18> {}
impl Reg32BitsConcat<10, 28> for Reg32Bits<18> {}
impl Reg32BitsDownCast<11> for Reg32Bits<18> {}
impl Reg32BitsConcat<11, 29> for Reg32Bits<18> {}
impl Reg32BitsDownCast<12> for Reg32Bits<18> {}
impl Reg32BitsConcat<12, 30> for Reg32Bits<18> {}
impl Reg32BitsDownCast<13> for Reg32Bits<18> {}
impl Reg32BitsConcat<13, 31> for Reg32Bits<18> {}
impl Reg32BitsDownCast<14> for Reg32Bits<18> {}
impl Reg32BitsConcat<14, 32> for Reg32Bits<18> {}
impl Reg32BitsDownCast<15> for Reg32Bits<18> {}
impl Reg32BitsDownCast<16> for Reg32Bits<18> {}
impl Reg32BitsDownCast<17> for Reg32Bits<18> {}
impl Reg32BitsDownCast<18> for Reg32Bits<18> {}
impl Reg32BitsDownCast<1> for Reg32Bits<19> {}
impl Reg32BitsConcat<1, 20> for Reg32Bits<19> {}
impl Reg32BitsDownCast<2> for Reg32Bits<19> {}
impl Reg32BitsConcat<2, 21> for Reg32Bits<19> {}
impl Reg32BitsDownCast<3> for Reg32Bits<19> {}
impl Reg32BitsConcat<3, 22> for Reg32Bits<19> {}
impl Reg32BitsDownCast<4> for Reg32Bits<19> {}
impl Reg32BitsConcat<4, 23> for Reg32Bits<19> {}
impl Reg32BitsDownCast<5> for Reg32Bits<19> {}
impl Reg32BitsConcat<5, 24> for Reg32Bits<19> {}
impl Reg32BitsDownCast<6> for Reg32Bits<19> {}
impl Reg32BitsConcat<6, 25> for Reg32Bits<19> {}
impl Reg32BitsDownCast<7> for Reg32Bits<19> {}
impl Reg32BitsConcat<7, 26> for Reg32Bits<19> {}
impl Reg32BitsDownCast<8> for Reg32Bits<19> {}
impl Reg32BitsConcat<8, 27> for Reg32Bits<19> {}
impl Reg32BitsDownCast<9> for Reg32Bits<19> {}
impl Reg32BitsConcat<9, 28> for Reg32Bits<19> {}
impl Reg32BitsDownCast<10> for Reg32Bits<19> {}
impl Reg32BitsConcat<10, 29> for Reg32Bits<19> {}
impl Reg32BitsDownCast<11> for Reg32Bits<19> {}
impl Reg32BitsConcat<11, 30> for Reg32Bits<19> {}
impl Reg32BitsDownCast<12> for Reg32Bits<19> {}
impl Reg32BitsConcat<12, 31> for Reg32Bits<19> {}
impl Reg32BitsDownCast<13> for Reg32Bits<19> {}
impl Reg32BitsConcat<13, 32> for Reg32Bits<19> {}
impl Reg32BitsDownCast<14> for Reg32Bits<19> {}
impl Reg32BitsDownCast<15> for Reg32Bits<19> {}
impl Reg32BitsDownCast<16> for Reg32Bits<19> {}
impl Reg32BitsDownCast<17> for Reg32Bits<19> {}
impl Reg32BitsDownCast<18> for Reg32Bits<19> {}
impl Reg32BitsDownCast<19> for Reg32Bits<19> {}
impl Reg32BitsDownCast<1> for Reg32Bits<20> {}
impl Reg32BitsConcat<1, 21> for Reg32Bits<20> {}
impl Reg32BitsDownCast<2> for Reg32Bits<20> {}
impl Reg32BitsConcat<2, 22> for Reg32Bits<20> {}
impl Reg32BitsDownCast<3> for Reg32Bits<20> {}
impl Reg32BitsConcat<3, 23> for Reg32Bits<20> {}
impl Reg32BitsDownCast<4> for Reg32Bits<20> {}
impl Reg32BitsConcat<4, 24> for Reg32Bits<20> {}
impl Reg32BitsDownCast<5> for Reg32Bits<20> {}
impl Reg32BitsConcat<5, 25> for Reg32Bits<20> {}
impl Reg32BitsDownCast<6> for Reg32Bits<20> {}
impl Reg32BitsConcat<6, 26> for Reg32Bits<20> {}
impl Reg32BitsDownCast<7> for Reg32Bits<20> {}
impl Reg32BitsConcat<7, 27> for Reg32Bits<20> {}
impl Reg32BitsDownCast<8> for Reg32Bits<20> {}
impl Reg32BitsConcat<8, 28> for Reg32Bits<20> {}
impl Reg32BitsDownCast<9> for Reg32Bits<20> {}
impl Reg32BitsConcat<9, 29> for Reg32Bits<20> {}
impl Reg32BitsDownCast<10> for Reg32Bits<20> {}
impl Reg32BitsConcat<10, 30> for Reg32Bits<20> {}
impl Reg32BitsDownCast<11> for Reg32Bits<20> {}
impl Reg32BitsConcat<11, 31> for Reg32Bits<20> {}
impl Reg32BitsDownCast<12> for Reg32Bits<20> {}
impl Reg32BitsConcat<12, 32> for Reg32Bits<20> {}
impl Reg32BitsDownCast<13> for Reg32Bits<20> {}
impl Reg32BitsDownCast<14> for Reg32Bits<20> {}
impl Reg32BitsDownCast<15> for Reg32Bits<20> {}
impl Reg32BitsDownCast<16> for Reg32Bits<20> {}
impl Reg32BitsDownCast<17> for Reg32Bits<20> {}
impl Reg32BitsDownCast<18> for Reg32Bits<20> {}
impl Reg32BitsDownCast<19> for Reg32Bits<20> {}
impl Reg32BitsDownCast<20> for Reg32Bits<20> {}
impl Reg32BitsDownCast<1> for Reg32Bits<21> {}
impl Reg32BitsConcat<1, 22> for Reg32Bits<21> {}
impl Reg32BitsDownCast<2> for Reg32Bits<21> {}
impl Reg32BitsConcat<2, 23> for Reg32Bits<21> {}
impl Reg32BitsDownCast<3> for Reg32Bits<21> {}
impl Reg32BitsConcat<3, 24> for Reg32Bits<21> {}
impl Reg32BitsDownCast<4> for Reg32Bits<21> {}
impl Reg32BitsConcat<4, 25> for Reg32Bits<21> {}
impl Reg32BitsDownCast<5> for Reg32Bits<21> {}
impl Reg32BitsConcat<5, 26> for Reg32Bits<21> {}
impl Reg32BitsDownCast<6> for Reg32Bits<21> {}
impl Reg32BitsConcat<6, 27> for Reg32Bits<21> {}
impl Reg32BitsDownCast<7> for Reg32Bits<21> {}
impl Reg32BitsConcat<7, 28> for Reg32Bits<21> {}
impl Reg32BitsDownCast<8> for Reg32Bits<21> {}
impl Reg32BitsConcat<8, 29> for Reg32Bits<21> {}
impl Reg32BitsDownCast<9> for Reg32Bits<21> {}
impl Reg32BitsConcat<9, 30> for Reg32Bits<21> {}
impl Reg32BitsDownCast<10> for Reg32Bits<21> {}
impl Reg32BitsConcat<10, 31> for Reg32Bits<21> {}
impl Reg32BitsDownCast<11> for Reg32Bits<21> {}
impl Reg32BitsConcat<11, 32> for Reg32Bits<21> {}
impl Reg32BitsDownCast<12> for Reg32Bits<21> {}
impl Reg32BitsDownCast<13> for Reg32Bits<21> {}
impl Reg32BitsDownCast<14> for Reg32Bits<21> {}
impl Reg32BitsDownCast<15> for Reg32Bits<21> {}
impl Reg32BitsDownCast<16> for Reg32Bits<21> {}
impl Reg32BitsDownCast<17> for Reg32Bits<21> {}
impl Reg32BitsDownCast<18> for Reg32Bits<21> {}
impl Reg32BitsDownCast<19> for Reg32Bits<21> {}
impl Reg32BitsDownCast<20> for Reg32Bits<21> {}
impl Reg32BitsDownCast<21> for Reg32Bits<21> {}
impl Reg32BitsDownCast<1> for Reg32Bits<22> {}
impl Reg32BitsConcat<1, 23> for Reg32Bits<22> {}
impl Reg32BitsDownCast<2> for Reg32Bits<22> {}
impl Reg32BitsConcat<2, 24> for Reg32Bits<22> {}
impl Reg32BitsDownCast<3> for Reg32Bits<22> {}
impl Reg32BitsConcat<3, 25> for Reg32Bits<22> {}
impl Reg32BitsDownCast<4> for Reg32Bits<22> {}
impl Reg32BitsConcat<4, 26> for Reg32Bits<22> {}
impl Reg32BitsDownCast<5> for Reg32Bits<22> {}
impl Reg32BitsConcat<5, 27> for Reg32Bits<22> {}
impl Reg32BitsDownCast<6> for Reg32Bits<22> {}
impl Reg32BitsConcat<6, 28> for Reg32Bits<22> {}
impl Reg32BitsDownCast<7> for Reg32Bits<22> {}
impl Reg32BitsConcat<7, 29> for Reg32Bits<22> {}
impl Reg32BitsDownCast<8> for Reg32Bits<22> {}
impl Reg32BitsConcat<8, 30> for Reg32Bits<22> {}
impl Reg32BitsDownCast<9> for Reg32Bits<22> {}
impl Reg32BitsConcat<9, 31> for Reg32Bits<22> {}
impl Reg32BitsDownCast<10> for Reg32Bits<22> {}
impl Reg32BitsConcat<10, 32> for Reg32Bits<22> {}
impl Reg32BitsDownCast<11> for Reg32Bits<22> {}
impl Reg32BitsDownCast<12> for Reg32Bits<22> {}
impl Reg32BitsDownCast<13> for Reg32Bits<22> {}
impl Reg32BitsDownCast<14> for Reg32Bits<22> {}
impl Reg32BitsDownCast<15> for Reg32Bits<22> {}
impl Reg32BitsDownCast<16> for Reg32Bits<22> {}
impl Reg32BitsDownCast<17> for Reg32Bits<22> {}
impl Reg32BitsDownCast<18> for Reg32Bits<22> {}
impl Reg32BitsDownCast<19> for Reg32Bits<22> {}
impl Reg32BitsDownCast<20> for Reg32Bits<22> {}
impl Reg32BitsDownCast<21> for Reg32Bits<22> {}
impl Reg32BitsDownCast<22> for Reg32Bits<22> {}
impl Reg32BitsDownCast<1> for Reg32Bits<23> {}
impl Reg32BitsConcat<1, 24> for Reg32Bits<23> {}
impl Reg32BitsDownCast<2> for Reg32Bits<23> {}
impl Reg32BitsConcat<2, 25> for Reg32Bits<23> {}
impl Reg32BitsDownCast<3> for Reg32Bits<23> {}
impl Reg32BitsConcat<3, 26> for Reg32Bits<23> {}
impl Reg32BitsDownCast<4> for Reg32Bits<23> {}
impl Reg32BitsConcat<4, 27> for Reg32Bits<23> {}
impl Reg32BitsDownCast<5> for Reg32Bits<23> {}
impl Reg32BitsConcat<5, 28> for Reg32Bits<23> {}
impl Reg32BitsDownCast<6> for Reg32Bits<23> {}
impl Reg32BitsConcat<6, 29> for Reg32Bits<23> {}
impl Reg32BitsDownCast<7> for Reg32Bits<23> {}
impl Reg32BitsConcat<7, 30> for Reg32Bits<23> {}
impl Reg32BitsDownCast<8> for Reg32Bits<23> {}
impl Reg32BitsConcat<8, 31> for Reg32Bits<23> {}
impl Reg32BitsDownCast<9> for Reg32Bits<23> {}
impl Reg32BitsConcat<9, 32> for Reg32Bits<23> {}
impl Reg32BitsDownCast<10> for Reg32Bits<23> {}
impl Reg32BitsDownCast<11> for Reg32Bits<23> {}
impl Reg32BitsDownCast<12> for Reg32Bits<23> {}
impl Reg32BitsDownCast<13> for Reg32Bits<23> {}
impl Reg32BitsDownCast<14> for Reg32Bits<23> {}
impl Reg32BitsDownCast<15> for Reg32Bits<23> {}
impl Reg32BitsDownCast<16> for Reg32Bits<23> {}
impl Reg32BitsDownCast<17> for Reg32Bits<23> {}
impl Reg32BitsDownCast<18> for Reg32Bits<23> {}
impl Reg32BitsDownCast<19> for Reg32Bits<23> {}
impl Reg32BitsDownCast<20> for Reg32Bits<23> {}
impl Reg32BitsDownCast<21> for Reg32Bits<23> {}
impl Reg32BitsDownCast<22> for Reg32Bits<23> {}
impl Reg32BitsDownCast<23> for Reg32Bits<23> {}
impl Reg32BitsDownCast<1> for Reg32Bits<24> {}
impl Reg32BitsConcat<1, 25> for Reg32Bits<24> {}
impl Reg32BitsDownCast<2> for Reg32Bits<24> {}
impl Reg32BitsConcat<2, 26> for Reg32Bits<24> {}
impl Reg32BitsDownCast<3> for Reg32Bits<24> {}
impl Reg32BitsConcat<3, 27> for Reg32Bits<24> {}
impl Reg32BitsDownCast<4> for Reg32Bits<24> {}
impl Reg32BitsConcat<4, 28> for Reg32Bits<24> {}
impl Reg32BitsDownCast<5> for Reg32Bits<24> {}
impl Reg32BitsConcat<5, 29> for Reg32Bits<24> {}
impl Reg32BitsDownCast<6> for Reg32Bits<24> {}
impl Reg32BitsConcat<6, 30> for Reg32Bits<24> {}
impl Reg32BitsDownCast<7> for Reg32Bits<24> {}
impl Reg32BitsConcat<7, 31> for Reg32Bits<24> {}
impl Reg32BitsDownCast<8> for Reg32Bits<24> {}
impl Reg32BitsConcat<8, 32> for Reg32Bits<24> {}
impl Reg32BitsDownCast<9> for Reg32Bits<24> {}
impl Reg32BitsDownCast<10> for Reg32Bits<24> {}
impl Reg32BitsDownCast<11> for Reg32Bits<24> {}
impl Reg32BitsDownCast<12> for Reg32Bits<24> {}
impl Reg32BitsDownCast<13> for Reg32Bits<24> {}
impl Reg32BitsDownCast<14> for Reg32Bits<24> {}
impl Reg32BitsDownCast<15> for Reg32Bits<24> {}
impl Reg32BitsDownCast<16> for Reg32Bits<24> {}
impl Reg32BitsDownCast<17> for Reg32Bits<24> {}
impl Reg32BitsDownCast<18> for Reg32Bits<24> {}
impl Reg32BitsDownCast<19> for Reg32Bits<24> {}
impl Reg32BitsDownCast<20> for Reg32Bits<24> {}
impl Reg32BitsDownCast<21> for Reg32Bits<24> {}
impl Reg32BitsDownCast<22> for Reg32Bits<24> {}
impl Reg32BitsDownCast<23> for Reg32Bits<24> {}
impl Reg32BitsDownCast<24> for Reg32Bits<24> {}
impl Reg32BitsDownCast<1> for Reg32Bits<25> {}
impl Reg32BitsConcat<1, 26> for Reg32Bits<25> {}
impl Reg32BitsDownCast<2> for Reg32Bits<25> {}
impl Reg32BitsConcat<2, 27> for Reg32Bits<25> {}
impl Reg32BitsDownCast<3> for Reg32Bits<25> {}
impl Reg32BitsConcat<3, 28> for Reg32Bits<25> {}
impl Reg32BitsDownCast<4> for Reg32Bits<25> {}
impl Reg32BitsConcat<4, 29> for Reg32Bits<25> {}
impl Reg32BitsDownCast<5> for Reg32Bits<25> {}
impl Reg32BitsConcat<5, 30> for Reg32Bits<25> {}
impl Reg32BitsDownCast<6> for Reg32Bits<25> {}
impl Reg32BitsConcat<6, 31> for Reg32Bits<25> {}
impl Reg32BitsDownCast<7> for Reg32Bits<25> {}
impl Reg32BitsConcat<7, 32> for Reg32Bits<25> {}
impl Reg32BitsDownCast<8> for Reg32Bits<25> {}
impl Reg32BitsDownCast<9> for Reg32Bits<25> {}
impl Reg32BitsDownCast<10> for Reg32Bits<25> {}
impl Reg32BitsDownCast<11> for Reg32Bits<25> {}
impl Reg32BitsDownCast<12> for Reg32Bits<25> {}
impl Reg32BitsDownCast<13> for Reg32Bits<25> {}
impl Reg32BitsDownCast<14> for Reg32Bits<25> {}
impl Reg32BitsDownCast<15> for Reg32Bits<25> {}
impl Reg32BitsDownCast<16> for Reg32Bits<25> {}
impl Reg32BitsDownCast<17> for Reg32Bits<25> {}
impl Reg32BitsDownCast<18> for Reg32Bits<25> {}
impl Reg32BitsDownCast<19> for Reg32Bits<25> {}
impl Reg32BitsDownCast<20> for Reg32Bits<25> {}
impl Reg32BitsDownCast<21> for Reg32Bits<25> {}
impl Reg32BitsDownCast<22> for Reg32Bits<25> {}
impl Reg32BitsDownCast<23> for Reg32Bits<25> {}
impl Reg32BitsDownCast<24> for Reg32Bits<25> {}
impl Reg32BitsDownCast<25> for Reg32Bits<25> {}
impl Reg32BitsDownCast<1> for Reg32Bits<26> {}
impl Reg32BitsConcat<1, 27> for Reg32Bits<26> {}
impl Reg32BitsDownCast<2> for Reg32Bits<26> {}
impl Reg32BitsConcat<2, 28> for Reg32Bits<26> {}
impl Reg32BitsDownCast<3> for Reg32Bits<26> {}
impl Reg32BitsConcat<3, 29> for Reg32Bits<26> {}
impl Reg32BitsDownCast<4> for Reg32Bits<26> {}
impl Reg32BitsConcat<4, 30> for Reg32Bits<26> {}
impl Reg32BitsDownCast<5> for Reg32Bits<26> {}
impl Reg32BitsConcat<5, 31> for Reg32Bits<26> {}
impl Reg32BitsDownCast<6> for Reg32Bits<26> {}
impl Reg32BitsConcat<6, 32> for Reg32Bits<26> {}
impl Reg32BitsDownCast<7> for Reg32Bits<26> {}
impl Reg32BitsDownCast<8> for Reg32Bits<26> {}
impl Reg32BitsDownCast<9> for Reg32Bits<26> {}
impl Reg32BitsDownCast<10> for Reg32Bits<26> {}
impl Reg32BitsDownCast<11> for Reg32Bits<26> {}
impl Reg32BitsDownCast<12> for Reg32Bits<26> {}
impl Reg32BitsDownCast<13> for Reg32Bits<26> {}
impl Reg32BitsDownCast<14> for Reg32Bits<26> {}
impl Reg32BitsDownCast<15> for Reg32Bits<26> {}
impl Reg32BitsDownCast<16> for Reg32Bits<26> {}
impl Reg32BitsDownCast<17> for Reg32Bits<26> {}
impl Reg32BitsDownCast<18> for Reg32Bits<26> {}
impl Reg32BitsDownCast<19> for Reg32Bits<26> {}
impl Reg32BitsDownCast<20> for Reg32Bits<26> {}
impl Reg32BitsDownCast<21> for Reg32Bits<26> {}
impl Reg32BitsDownCast<22> for Reg32Bits<26> {}
impl Reg32BitsDownCast<23> for Reg32Bits<26> {}
impl Reg32BitsDownCast<24> for Reg32Bits<26> {}
impl Reg32BitsDownCast<25> for Reg32Bits<26> {}
impl Reg32BitsDownCast<26> for Reg32Bits<26> {}
impl Reg32BitsDownCast<1> for Reg32Bits<27> {}
impl Reg32BitsConcat<1, 28> for Reg32Bits<27> {}
impl Reg32BitsDownCast<2> for Reg32Bits<27> {}
impl Reg32BitsConcat<2, 29> for Reg32Bits<27> {}
impl Reg32BitsDownCast<3> for Reg32Bits<27> {}
impl Reg32BitsConcat<3, 30> for Reg32Bits<27> {}
impl Reg32BitsDownCast<4> for Reg32Bits<27> {}
impl Reg32BitsConcat<4, 31> for Reg32Bits<27> {}
impl Reg32BitsDownCast<5> for Reg32Bits<27> {}
impl Reg32BitsConcat<5, 32> for Reg32Bits<27> {}
impl Reg32BitsDownCast<6> for Reg32Bits<27> {}
impl Reg32BitsDownCast<7> for Reg32Bits<27> {}
impl Reg32BitsDownCast<8> for Reg32Bits<27> {}
impl Reg32BitsDownCast<9> for Reg32Bits<27> {}
impl Reg32BitsDownCast<10> for Reg32Bits<27> {}
impl Reg32BitsDownCast<11> for Reg32Bits<27> {}
impl Reg32BitsDownCast<12> for Reg32Bits<27> {}
impl Reg32BitsDownCast<13> for Reg32Bits<27> {}
impl Reg32BitsDownCast<14> for Reg32Bits<27> {}
impl Reg32BitsDownCast<15> for Reg32Bits<27> {}
impl Reg32BitsDownCast<16> for Reg32Bits<27> {}
impl Reg32BitsDownCast<17> for Reg32Bits<27> {}
impl Reg32BitsDownCast<18> for Reg32Bits<27> {}
impl Reg32BitsDownCast<19> for Reg32Bits<27> {}
impl Reg32BitsDownCast<20> for Reg32Bits<27> {}
impl Reg32BitsDownCast<21> for Reg32Bits<27> {}
impl Reg32BitsDownCast<22> for Reg32Bits<27> {}
impl Reg32BitsDownCast<23> for Reg32Bits<27> {}
impl Reg32BitsDownCast<24> for Reg32Bits<27> {}
impl Reg32BitsDownCast<25> for Reg32Bits<27> {}
impl Reg32BitsDownCast<26> for Reg32Bits<27> {}
impl Reg32BitsDownCast<27> for Reg32Bits<27> {}
impl Reg32BitsDownCast<1> for Reg32Bits<28> {}
impl Reg32BitsConcat<1, 29> for Reg32Bits<28> {}
impl Reg32BitsDownCast<2> for Reg32Bits<28> {}
impl Reg32BitsConcat<2, 30> for Reg32Bits<28> {}
impl Reg32BitsDownCast<3> for Reg32Bits<28> {}
impl Reg32BitsConcat<3, 31> for Reg32Bits<28> {}
impl Reg32BitsDownCast<4> for Reg32Bits<28> {}
impl Reg32BitsConcat<4, 32> for Reg32Bits<28> {}
impl Reg32BitsDownCast<5> for Reg32Bits<28> {}
impl Reg32BitsDownCast<6> for Reg32Bits<28> {}
impl Reg32BitsDownCast<7> for Reg32Bits<28> {}
impl Reg32BitsDownCast<8> for Reg32Bits<28> {}
impl Reg32BitsDownCast<9> for Reg32Bits<28> {}
impl Reg32BitsDownCast<10> for Reg32Bits<28> {}
impl Reg32BitsDownCast<11> for Reg32Bits<28> {}
impl Reg32BitsDownCast<12> for Reg32Bits<28> {}
impl Reg32BitsDownCast<13> for Reg32Bits<28> {}
impl Reg32BitsDownCast<14> for Reg32Bits<28> {}
impl Reg32BitsDownCast<15> for Reg32Bits<28> {}
impl Reg32BitsDownCast<16> for Reg32Bits<28> {}
impl Reg32BitsDownCast<17> for Reg32Bits<28> {}
impl Reg32BitsDownCast<18> for Reg32Bits<28> {}
impl Reg32BitsDownCast<19> for Reg32Bits<28> {}
impl Reg32BitsDownCast<20> for Reg32Bits<28> {}
impl Reg32BitsDownCast<21> for Reg32Bits<28> {}
impl Reg32BitsDownCast<22> for Reg32Bits<28> {}
impl Reg32BitsDownCast<23> for Reg32Bits<28> {}
impl Reg32BitsDownCast<24> for Reg32Bits<28> {}
impl Reg32BitsDownCast<25> for Reg32Bits<28> {}
impl Reg32BitsDownCast<26> for Reg32Bits<28> {}
impl Reg32BitsDownCast<27> for Reg32Bits<28> {}
impl Reg32BitsDownCast<28> for Reg32Bits<28> {}
impl Reg32BitsDownCast<1> for Reg32Bits<29> {}
impl Reg32BitsConcat<1, 30> for Reg32Bits<29> {}
impl Reg32BitsDownCast<2> for Reg32Bits<29> {}
impl Reg32BitsConcat<2, 31> for Reg32Bits<29> {}
impl Reg32BitsDownCast<3> for Reg32Bits<29> {}
impl Reg32BitsConcat<3, 32> for Reg32Bits<29> {}
impl Reg32BitsDownCast<4> for Reg32Bits<29> {}
impl Reg32BitsDownCast<5> for Reg32Bits<29> {}
impl Reg32BitsDownCast<6> for Reg32Bits<29> {}
impl Reg32BitsDownCast<7> for Reg32Bits<29> {}
impl Reg32BitsDownCast<8> for Reg32Bits<29> {}
impl Reg32BitsDownCast<9> for Reg32Bits<29> {}
impl Reg32BitsDownCast<10> for Reg32Bits<29> {}
impl Reg32BitsDownCast<11> for Reg32Bits<29> {}
impl Reg32BitsDownCast<12> for Reg32Bits<29> {}
impl Reg32BitsDownCast<13> for Reg32Bits<29> {}
impl Reg32BitsDownCast<14> for Reg32Bits<29> {}
impl Reg32BitsDownCast<15> for Reg32Bits<29> {}
impl Reg32BitsDownCast<16> for Reg32Bits<29> {}
impl Reg32BitsDownCast<17> for Reg32Bits<29> {}
impl Reg32BitsDownCast<18> for Reg32Bits<29> {}
impl Reg32BitsDownCast<19> for Reg32Bits<29> {}
impl Reg32BitsDownCast<20> for Reg32Bits<29> {}
impl Reg32BitsDownCast<21> for Reg32Bits<29> {}
impl Reg32BitsDownCast<22> for Reg32Bits<29> {}
impl Reg32BitsDownCast<23> for Reg32Bits<29> {}
impl Reg32BitsDownCast<24> for Reg32Bits<29> {}
impl Reg32BitsDownCast<25> for Reg32Bits<29> {}
impl Reg32BitsDownCast<26> for Reg32Bits<29> {}
impl Reg32BitsDownCast<27> for Reg32Bits<29> {}
impl Reg32BitsDownCast<28> for Reg32Bits<29> {}
impl Reg32BitsDownCast<29> for Reg32Bits<29> {}
impl Reg32BitsDownCast<1> for Reg32Bits<30> {}
impl Reg32BitsConcat<1, 31> for Reg32Bits<30> {}
impl Reg32BitsDownCast<2> for Reg32Bits<30> {}
impl Reg32BitsConcat<2, 32> for Reg32Bits<30> {}
impl Reg32BitsDownCast<3> for Reg32Bits<30> {}
impl Reg32BitsDownCast<4> for Reg32Bits<30> {}
impl Reg32BitsDownCast<5> for Reg32Bits<30> {}
impl Reg32BitsDownCast<6> for Reg32Bits<30> {}
impl Reg32BitsDownCast<7> for Reg32Bits<30> {}
impl Reg32BitsDownCast<8> for Reg32Bits<30> {}
impl Reg32BitsDownCast<9> for Reg32Bits<30> {}
impl Reg32BitsDownCast<10> for Reg32Bits<30> {}
impl Reg32BitsDownCast<11> for Reg32Bits<30> {}
impl Reg32BitsDownCast<12> for Reg32Bits<30> {}
impl Reg32BitsDownCast<13> for Reg32Bits<30> {}
impl Reg32BitsDownCast<14> for Reg32Bits<30> {}
impl Reg32BitsDownCast<15> for Reg32Bits<30> {}
impl Reg32BitsDownCast<16> for Reg32Bits<30> {}
impl Reg32BitsDownCast<17> for Reg32Bits<30> {}
impl Reg32BitsDownCast<18> for Reg32Bits<30> {}
impl Reg32BitsDownCast<19> for Reg32Bits<30> {}
impl Reg32BitsDownCast<20> for Reg32Bits<30> {}
impl Reg32BitsDownCast<21> for Reg32Bits<30> {}
impl Reg32BitsDownCast<22> for Reg32Bits<30> {}
impl Reg32BitsDownCast<23> for Reg32Bits<30> {}
impl Reg32BitsDownCast<24> for Reg32Bits<30> {}
impl Reg32BitsDownCast<25> for Reg32Bits<30> {}
impl Reg32BitsDownCast<26> for Reg32Bits<30> {}
impl Reg32BitsDownCast<27> for Reg32Bits<30> {}
impl Reg32BitsDownCast<28> for Reg32Bits<30> {}
impl Reg32BitsDownCast<29> for Reg32Bits<30> {}
impl Reg32BitsDownCast<30> for Reg32Bits<30> {}
impl Reg32BitsDownCast<1> for Reg32Bits<31> {}
impl Reg32BitsConcat<1, 32> for Reg32Bits<31> {}
impl Reg32BitsDownCast<2> for Reg32Bits<31> {}
impl Reg32BitsDownCast<3> for Reg32Bits<31> {}
impl Reg32BitsDownCast<4> for Reg32Bits<31> {}
impl Reg32BitsDownCast<5> for Reg32Bits<31> {}
impl Reg32BitsDownCast<6> for Reg32Bits<31> {}
impl Reg32BitsDownCast<7> for Reg32Bits<31> {}
impl Reg32BitsDownCast<8> for Reg32Bits<31> {}
impl Reg32BitsDownCast<9> for Reg32Bits<31> {}
impl Reg32BitsDownCast<10> for Reg32Bits<31> {}
impl Reg32BitsDownCast<11> for Reg32Bits<31> {}
impl Reg32BitsDownCast<12> for Reg32Bits<31> {}
impl Reg32BitsDownCast<13> for Reg32Bits<31> {}
impl Reg32BitsDownCast<14> for Reg32Bits<31> {}
impl Reg32BitsDownCast<15> for Reg32Bits<31> {}
impl Reg32BitsDownCast<16> for Reg32Bits<31> {}
impl Reg32BitsDownCast<17> for Reg32Bits<31> {}
impl Reg32BitsDownCast<18> for Reg32Bits<31> {}
impl Reg32BitsDownCast<19> for Reg32Bits<31> {}
impl Reg32BitsDownCast<20> for Reg32Bits<31> {}
impl Reg32BitsDownCast<21> for Reg32Bits<31> {}
impl Reg32BitsDownCast<22> for Reg32Bits<31> {}
impl Reg32BitsDownCast<23> for Reg32Bits<31> {}
impl Reg32BitsDownCast<24> for Reg32Bits<31> {}
impl Reg32BitsDownCast<25> for Reg32Bits<31> {}
impl Reg32BitsDownCast<26> for Reg32Bits<31> {}
impl Reg32BitsDownCast<27> for Reg32Bits<31> {}
impl Reg32BitsDownCast<28> for Reg32Bits<31> {}
impl Reg32BitsDownCast<29> for Reg32Bits<31> {}
impl Reg32BitsDownCast<30> for Reg32Bits<31> {}
impl Reg32BitsDownCast<31> for Reg32Bits<31> {}
impl Reg32BitsDownCast<1> for Reg32Bits<32> {}
impl Reg32BitsDownCast<2> for Reg32Bits<32> {}
impl Reg32BitsDownCast<3> for Reg32Bits<32> {}
impl Reg32BitsDownCast<4> for Reg32Bits<32> {}
impl Reg32BitsDownCast<5> for Reg32Bits<32> {}
impl Reg32BitsDownCast<6> for Reg32Bits<32> {}
impl Reg32BitsDownCast<7> for Reg32Bits<32> {}
impl Reg32BitsDownCast<8> for Reg32Bits<32> {}
impl Reg32BitsDownCast<9> for Reg32Bits<32> {}
impl Reg32BitsDownCast<10> for Reg32Bits<32> {}
impl Reg32BitsDownCast<11> for Reg32Bits<32> {}
impl Reg32BitsDownCast<12> for Reg32Bits<32> {}
impl Reg32BitsDownCast<13> for Reg32Bits<32> {}
impl Reg32BitsDownCast<14> for Reg32Bits<32> {}
impl Reg32BitsDownCast<15> for Reg32Bits<32> {}
impl Reg32BitsDownCast<16> for Reg32Bits<32> {}
impl Reg32BitsDownCast<17> for Reg32Bits<32> {}
impl Reg32BitsDownCast<18> for Reg32Bits<32> {}
impl Reg32BitsDownCast<19> for Reg32Bits<32> {}
impl Reg32BitsDownCast<20> for Reg32Bits<32> {}
impl Reg32BitsDownCast<21> for Reg32Bits<32> {}
impl Reg32BitsDownCast<22> for Reg32Bits<32> {}
impl Reg32BitsDownCast<23> for Reg32Bits<32> {}
impl Reg32BitsDownCast<24> for Reg32Bits<32> {}
impl Reg32BitsDownCast<25> for Reg32Bits<32> {}
impl Reg32BitsDownCast<26> for Reg32Bits<32> {}
impl Reg32BitsDownCast<27> for Reg32Bits<32> {}
impl Reg32BitsDownCast<28> for Reg32Bits<32> {}
impl Reg32BitsDownCast<29> for Reg32Bits<32> {}
impl Reg32BitsDownCast<30> for Reg32Bits<32> {}
impl Reg32BitsDownCast<31> for Reg32Bits<32> {}
impl Reg32BitsDownCast<32> for Reg32Bits<32> {}
