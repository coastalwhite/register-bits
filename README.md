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


```rust
use register_bits::prelude::*;

let value = Reg32Bits::new(0x4321_1234);
let first_12bits: Reg32Bits<12> = value.take_low();

assert_eq!(first_12bits, 0x234);
```


## No-Std

The `no-std` ensures that the environment does not utilize the standard library.


## Development

The `reg8.rs`, `reg16.rs`, `reg32.rs` and `reg64.rs` are automatically generated from the `reg_reference.rs` file. This is done with the `generate_impl_rs.py` script.


 [__cargo_doc2readme_dependencies_info]: ggGkYW0AYXSEG_Fi7gAv9SUoG-NvRGkL0p9pG7gKrlMPMhgOGwwClx6a7y-DYXKEGx0AlPZ-eQXvGzvHtyI2SJw4G3dritKM6W61G-WWNEVsADi-YWSBg21yZWdpc3Rlci1iaXRzZTAuMS4wbXJlZ2lzdGVyX2JpdHM
 [__link0]: https://docs.rs/register-bits/0.1.0/register_bits/?search=register_bits::reg8
 [__link1]: https://docs.rs/register-bits/0.1.0/register_bits/?search=register_bits::reg16
 [__link2]: https://docs.rs/register-bits/0.1.0/register_bits/?search=register_bits::reg32
 [__link3]: https://docs.rs/register-bits/0.1.0/register_bits/?search=register_bits::reg64
