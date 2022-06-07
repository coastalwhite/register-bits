#[cfg(feature = "8bit")]
pub use crate::reg8::{Reg8Bits, Reg8BitsDownCast, Reg8BitsUpCast, Reg8BitsConcat};
#[cfg(feature = "16bit")]
pub use crate::reg16::{Reg16Bits, Reg16BitsDownCast, Reg16BitsUpCast, Reg16BitsConcat};
#[cfg(feature = "32bit")]
pub use crate::reg32::{Reg32Bits, Reg32BitsDownCast, Reg32BitsUpCast, Reg32BitsConcat};
#[cfg(feature = "64bit")]
pub use crate::reg64::{Reg64Bits, Reg64BitsDownCast, Reg64BitsUpCast, Reg64BitsConcat};