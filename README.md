# Register Bits 🦀

A rust crate to verify the size of the commonly used binary operations on
register values. Currently this is only a proof of concept.

## Generating the implementations

The `generate_impl_rs.py` script generates the code in the `src/impls_*.rs`
files. To regenerate the code in those files. Run `./generate_impl_rs >
src/impls_32.rs`. At the moment this only generates the code for 32 bit max
register values.