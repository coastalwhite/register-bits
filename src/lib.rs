//! A crate to perform register-bit manipulation which is verified at compile time
//!
//! This crate provides many inlined procedures for performing bit manipulations including taking a
//! selection of bits, concatinating bits and forming new bitstring. All these manipulations are
//! checked at compile time to ensure reliability at runtime at the cost of compilation duration.
//!
//! There are 4 different register types.
//! - [A 8 bit register][reg8]
//! - [A 16 bit register][reg16]
//! - [A 32 bit register][reg32]
//! - [A 64 bit register][reg64]
//!
//! To utilize most of the functionality a set of traits need to used. To utilize all the useful
//! traites it is recommended to use the prelude.
//!
//! ```
//! use register_bits::prelude::*;
//!
//! let value = Reg32Bits::new(0x4321_1234);
//! let first_12bits = value.take_low::<12>::();
//!
//! assert_eq!(first_12bits, 0x234);
//! ```
#![no_std]

pub mod prelude;

#[cfg(feature = "8bit")]
pub mod reg8;
#[cfg(feature = "16bit")]
pub mod reg16;
#[cfg(feature = "32bit")]
pub mod reg32;
#[cfg(feature = "64bit")]
pub mod reg64;

#[cfg(test)]
mod reg_reference;
