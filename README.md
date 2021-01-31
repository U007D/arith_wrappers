# arith_wrappers

Rust's `std` offers the [`Wrapping`](std::num::Wrapping) type for "intentionally wrapping" arithmetic,
but curiously does not provide analagous implementations for intentionally saturating, checked, 
overflowing or panicking arithmetic.  This crate addresses these missing wrappers as well as 
re-exports `std`'s [`Wrapping`](std::num::Wrapping) type for completeness and consistency. 

Without these wrappers, performing correct arithmetic is verbose, error-prone and pedantic, leading
to ignoring the problems altogether or hard-to-maintain implementations.

This crate is at a very early stage of implementation and should be considered a PoC.

## License
Licensed under either:
* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
at your option.

## Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be dual licensed as above, without any additional terms or conditions.
