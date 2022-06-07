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
