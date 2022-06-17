# Register Bits 🦀 ![License: MIT](https://img.shields.io/badge/license-MIT-blue) [![register-bits on crates.io](https://img.shields.io/crates/v/register-bits)](https://crates.io/crates/register-bits) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/coastalwhite/register-bits)

A crate to perform register-bit manipulation which is verified at compile time

This crate provides many inlined procedures for performing bit manipulations including taking a selection of bits, concatinating bits and forming new bitstring. All these manipulations are checked at compile time to ensure reliability at runtime at the cost of compilation duration.

There are 4 different register types.

 - [A 8 bit register][__link0] which can be enabled with the `8bit` feature
 - [A 16 bit register][__link1] which can be enabled with the `16bit` feature
 - [A 32 bit register][__link2] which can be enabled with the `32bit` feature (included by default)
 - [A 64 bit register][__link3] which can be enabled with the `64bit` feature

All the register variants can be included using the `all-regs` feature.


## Usage

To utilize most of the functionality a set of traits need to used. To utilize all the useful traites it is recommended to use the prelude.


### Basic Usage


```rust
use register_bits::prelude::*;

// Forms a Reg32Bits<32>
let value = Reg32Bits::new(0x1234_5678);

// Take substrings from value
let low_12bits: Reg32Bits<12> = value.take_low(); // 0x678
let high_12bits: Reg32Bits<12> = value.take_high(); // 0x123

// Type of Reg32Bits<24> is automatically inferred
let concatination = high_12bits.concat(low_12bits); // 0x123_678

assert_eq!(high_12bits.concat(low_12bits), 0x123_678); 
```


### Casting


```rust
use register_bits::prelude::*;

// Forms a Reg32Bits<32>
let value = Reg32Bits::new(0x1234_5678);

let low_12bits: Reg32Bits<12> = value.take_low(); // 0x678

// We can fetch the inner value
let uint_value = u32::from(low_12bits);
assert_eq!(uint_value, 0x678);

// Most of the operations you can do from within the struct however
assert_eq!(low_12bits, 0x678);
assert_eq!(low_12bits + 1, 0x679);
assert_eq!(low_12bits - 1, 0x677);
assert_eq!(low_12bits % 2, 0);
assert_eq!(low_12bits >> 2, 0x6);

// You can also add bits
let bigger: Reg32Bits<16> = low_12bits.zero_extend(); // 0x0678
let sign_extended: Reg32Bits<16> = low_12bits.sign_extend(); // 0x0678
let padded: Reg32Bits<16> = low_12bits.zero_pad(); // 0x6780
```


### Individual bit manipulations


```rust
use register_bits::prelude::*;

// Forms a Reg32Bits<32>
let value = Reg32Bits::new(0b1011_1000);

let low_byte: Reg32Bits<8> = value.take_low(); // 0b1011_1000

// We can get the value of individual bits
let bits = low_byte.bits();

// This is perfect for pattern matching
assert_eq!(bits, [1, 0, 1, 1, 1, 0, 0, 0]);

// We can also get a bit from a runtime variable
assert_eq!(low_byte.get(3).unwrap(), 1u8);
```


## No-Std

The `no-std` ensures that the environment does not utilize the standard library.


## Development

The `reg8.rs`, `reg16.rs`, `reg32.rs` and `reg64.rs` are automatically generated from the `reg_reference.rs` file. This is done with the `generate_impl_rs.py` script.


 [__cargo_doc2readme_dependencies_info]: ggGkYW0AYXSEG_Fi7gAv9SUoG-NvRGkL0p9pG7gKrlMPMhgOGwwClx6a7y-DYXKEG40CcuyI-6AJG7O0lMCLFTbpG_-YIaqOB7S0G1lJNphkaO1yYWSBg21yZWdpc3Rlci1iaXRzZTAuMS4wbXJlZ2lzdGVyX2JpdHM
 [__link0]: https://docs.rs/register-bits/0.1.0/register_bits/?search=register_bits::reg8
 [__link1]: https://docs.rs/register-bits/0.1.0/register_bits/?search=register_bits::reg16
 [__link2]: https://docs.rs/register-bits/0.1.0/register_bits/?search=register_bits::reg32
 [__link3]: https://docs.rs/register-bits/0.1.0/register_bits/?search=register_bits::reg64
